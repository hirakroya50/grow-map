# Grow Map — system architecture

## Architecture decision
Use a modular Rust monolith: Axum HTTP, async-graphql schema, Tokio runtime, official async MongoDB driver, object storage adapter, and background worker. Pin compatible supported dependency versions when scaffolding and commit lockfiles. React Native TypeScript mobile client uses generated GraphQL operation types. pnpm + Turborepo orchestrate JS tasks and wrapper scripts invoking Cargo; Cargo remains the Rust build system. Avoid microservices until measured constraints justify them.

## Repository
- apps/mobile: screens, graph interaction, wallet adapter, secure session storage, operation documents.
- services/api: transport adapters, domain services, repositories, authorization, auth, worker.
- packages/graphql: generated client types and client operation documents.
- contracts/openapi.json: canonical HTTP contract.
- contracts/schema.graphql: canonical desired GraphQL contract.
- infra: replica-set MongoDB, local S3-compatible storage, deployment assets.
- docs and root AGENTS.md: implementation policy.

## Trust and ownership
Mobile input is untrusted. Rust owns validation, authorization, revisions, limits, event sequence, and timestamps. GraphQL resolvers and REST handlers call the same domain services. Database collections are inaccessible to clients. Object buckets are private. Only signed exact-object grants permit transfer. Wallets sign a challenge, not an on-chain transaction. Blockchain RPC is not on the normal roadmap read/write path.

## Data consistency
Use MongoDB replica sets, including a single-node development replica set. Production uses a managed multi-node deployment. No standalone MongoDB for transaction-dependent paths. All state-changing roadmap operations use a transaction with snapshot read concern and majority commit write concern.

Every writer conditionally updates the roadmap coordination document (revision/sequence, ACL epoch). This serializes writes per roadmap and fences concurrent permission changes: checking membership without this coordination write is insufficient. Keep transactions short; no external network calls inside them. Retry transient transaction conflicts with bounded jitter. Entity expectedRevision is separate from roadmap revision, so changes to different nodes can succeed after a transaction retry.

## Write algorithm
1. Authenticate, parse, validate field lengths and operation ID; compute canonical request hash including operation name and normalized input.
2. Transaction: read live roadmap and membership; authorize; look up (actorId, operationId) receipt. Recheck authorization before serving a receipt.
3. Same hash receipt returns original committed result; different hash is IDEMPOTENCY_MISMATCH. If no receipt, compare entity revision; apply changes and limits.
4. Increment roadmap revision and sequence once per mutation; update counts if needed; create durable event and operation receipt in the same transaction.
5. Majority commit, then acknowledge. If commit outcome is uncertain, resolve using receipt/driver transaction semantics; never blindly apply twice.
6. Worker delivers committed events at least once. Clients deduplicate by sequence and operation ID. No broadcast before commit.

Coordination is an intentional per-roadmap throughput bottleneck. Benchmark hotspots before introducing partitioned ordering.

## Graph read and reconnect
roadmapGraph reads roadmap, active nodes, edges, and sequence in one short read-only snapshot transaction. It returns all graph metadata except attachment bytes. Response size is capped at 8 MiB; GRAPH_TOO_LARGE instructs users to split roadmaps. Snapshot reads are bounded by graph limits; do not hold a transaction while sending the response.

Subscription authenticates over graphql-transport-ws connection_init, checks membership, and accepts afterSequence. Only authenticated members subscribe in v1. A replay-capable broker reads durable events in ascending order, registers live interest before catch-up, and deduplicates overlap. MongoDB change streams are wake-up hints, not the sole replay source. After broker restart/reconnect, poll from last delivered sequence. Gaps or expired retention return RESYNC_REQUIRED. Each event delivery rechecks ACL/session state; active connections are also revalidated at least every 5 seconds. Queues cap at 256 events; slow consumers close and reconnect. Access tokens expiring on a socket require reconnection. Public viewers refresh the read-only snapshot instead.

## Graph generations and versions
Each roadmap has activeGenerationId. Nodes/edges belong to a generation. Ordinary writes modify only the active generation. A saved version references immutable snapshot chunks created from a consistent snapshot (bounded, short transaction). Snapshot chunks and attachment references are retained. Restoring is a job: build a fresh inactive generation from snapshot chunks, verify counts/checksums, then atomically authorize, compare expected roadmap revision, switch activeGenerationId, increment sequence, and append GRAPH_RESET. Stale restore fails without replacing current edits. Replayed GRAPH_RESET requires a new snapshot. Clients cannot apply stale node IDs/revisions to a different generation. Internal generation identity prevents old-write ABA errors; expected entity revisions remain globally increasing from roadmap sequence across restores. Old generations are garbage-collected only after leases/retention safety checks.

## Attachments and jobs
Upload initiation reserves quota and creates a pending upload record; returns a short-lived signed exact-key PUT with required headers. Content length is verified after transfer, never trusted solely from request headers. Completion HEAD-checks storage outside a transaction, then transactionally rechecks node/generation/role/quota and finalizes attachment metadata plus event. File safety scanning is asynchronous; downloads are allowed only for READY files. Orphan/pending uploads expire and are deleted by a retryable worker. Attachment IDs referenced by versions retain their objects. SVG/HTML/executables are disallowed in v1; display safe file types without executing content.

Version restore, snapshot creation, file scanning, deletion cleanup, and generation cleanup run as durable jobs with lease expiry, retry budgets, and idempotent steps. API records the job transactionally; no detached in-memory job is sufficient. Delete roadmap marks deleted and revokes access first, then cleans storage asynchronously.

## Deployment and resilience
API and MongoDB in same region; start near Indian users and measure real mobile networks. Horizontally scale stateless API instances; shared persistent events/jobs/session state. No Redis required initially; add distributed ephemeral presence only when justified. Graceful shutdown stops admission, drains bounded requests, releases leases, and closes sockets with reconnect guidance. Health readiness checks DB/required configuration with deadlines; liveness never fails merely because DB is unavailable.

Structured logs: request ID, operation name, duration, status, roadmap opaque ID; no tokens, signatures, descriptions, private URLs, or wallet identities. Metrics: p50/p95/p99 latency, transaction retries, pool waits, authorization denials, queue depth, replay lag, restore jobs, object errors. Alerts on SLO burn and sustained worker lag. Backup database and version-referenced objects; test joint restoration. Expand/contract migrations and additive schema changes preserve older mobile clients.

## Performance priorities
Batch resolver reads with request-scoped loaders; no per-node queries. Connection pools and bounded concurrency. Payload selection, compressed responses, input/body limits, query depth/complexity and alias limits. Canvas virtualization/viewport culling; optimistic UI and local gestures. No real-time database write on every drag frame. Cache private data only with user/access-scoped keys; no shared CDN caching of private GraphQL results.

## References
- https://www.mongodb.com/docs/manual/core/transactions-production-consideration/
- https://async-graphql.github.io/async-graphql/en/dataloader.html
- https://spec.openapis.org/oas/v3.1.0.html
- https://github.com/enisdenjo/graphql-ws/blob/master/PROTOCOL.md
Implementers must verify current official wallet/platform docs and compatible library versions during setup.

## Control-plane receipts
Profile edits use a user-scoped receipt transaction. Upload reservations use a roadmap-coordinated quota transaction and receipt but do not emit a graph event until completion. Version/restore job acceptance stores a receipt and job transactionally without advancing graph sequence; the successful snapshot registration or generation switch advances sequence and emits the event. Failed jobs never report a committed graph change. All sequence/event invariants refer to graph/ACL-visible state mutations.

## Snapshot job execution
createVersion acceptance checks permission and expectedRevision but does not freeze edits. Worker opens a bounded snapshot transaction, rechecks authorization and exact expectedRevision, then reads active graph and writes immutable chunks/version registration/event/roadmap sequence together. If edits happened before execution, the job fails CONFLICT and the client retries deliberately. Named version sourceRevision is the revision before the version-registration event. Never capture a later revision under an earlier version label. Restore staging may run outside a transaction, but activation rechecks initiating actor permission and expectedRevision in the coordination transaction. Failed or revoked jobs cannot activate.

## Public DTO policy
Public graph fields expose opaque ownerId and content metadata but never User walletAddress, member lists, actor identities or events. Public/private resolvers must not offer nested profile expansion through these IDs. Do not make public-roadmap visibility a grant to every user-scoped query.
