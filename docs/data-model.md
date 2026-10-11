# Grow Map — database specification

## Conventions
MongoDB with explicit JSON Schema validators and versioned migrations. UUID strings are public IDs; UTC BSON dates are timestamps. Revisions/sequences are signed BSON int64 serialized as decimal strings by GraphQL Revision scalar (not JavaScript numbers). New revisions equal the new roadmap sequence, so restoring a graph never reuses an entity revision. UUIDs are not access controls. Every repository lookup is scoped by authorized roadmap and active generation. No arbitrary collection names, query operators, or sort expressions from clients.

## Collections and indexes
| Collection | Fields / invariants | Required indexes |
|---|---|---|
| users | _id, walletAddress, displayName, createdAt, updatedAt; case-sensitive canonical base58 32-byte Solana public key | unique walletAddress |
| auth_challenges | _id, walletAddress, nonceHash, exactMessage, domain, issuedAt, expiresAt, consumedAt | TTL expiresAt |
| sessions | _id, userId, familyId, refreshHash, previousRefreshHashes, expiresAt, revokedAt | unique refreshHash; userId; TTL expiresAt |
| roadmaps | _id, ownerId, title, description, visibility, activeGenerationId, revision, sequence, aclEpoch, minReplaySequence, nodeCount, edgeCount, reservedBytes, activeBytes, deletedAt, createdAt, updatedAt | ownerId/updatedAt/_id; visibility/updatedAt/_id |
| memberships | _id, roadmapId, userId, role, revision, createdAt | unique roadmapId/userId; userId/createdAt/_id |
| nodes | _id internal, nodeId public, roadmapId, generationId, name, description, tags[], links[], status, position{x,y}, revision, createdById, updatedById, createdAt, updatedAt | unique roadmapId/generationId/nodeId; roadmapId/generationId/status |
| edges | _id internal, edgeId public, roadmapId, generationId, sourceNodeId, targetNodeId, direction, label, revision | unique roadmapId/generationId/edgeId; unique roadmapId/generationId/sourceNodeId/targetNodeId/direction; roadmapId/generationId/targetNodeId |
| attachments | _id, roadmapId, nodeId, objectKey, filename, contentType, sizeBytes, checksum, state, deletedAt, createdAt | roadmapId/nodeId; unique objectKey; state/createdAt |
| uploads | _id, actorId, roadmapId, generationId, nodeId, objectKey, reservedBytes, expiresAt, state | unique objectKey; state/expiresAt |
| events | _id, roadmapId, sequence, type, actorId, operationId, payload, occurredAt, expiresAt | unique roadmapId/sequence; TTL expiresAt |
| operation_receipts | _id, actorId, operationId, requestHash, roadmapId, response, createdAt | unique actorId/operationId; roadmapId/createdAt |
| share_links | _id, roadmapId, tokenHash, expiresAt, revokedAt, createdById | unique tokenHash; roadmapId |
| invites | _id, roadmapId, tokenHash, role, targetWalletAddress?, expiresAt, consumedById?, revokedAt | unique tokenHash; roadmapId |
| versions | _id, roadmapId, name, sourceRevision, sourceGenerationId, snapshotId, counts, createdById, state, createdAt | roadmapId/createdAt/_id |
| snapshot_chunks | _id, snapshotId, chunkNumber, checksum, nodes[], edges[], attachmentIds[] | unique snapshotId/chunkNumber |
| jobs | _id, roadmapId, actorId, kind, state, expectedRevision, input, result, leaseUntil, attempts, createdAt, updatedAt | state/leaseUntil; roadmapId/createdAt |

Keep snapshot chunks below 4 MiB. Split version documents/chunks rather than growing arrays indefinitely. Embedded links and tags have strict cardinality bounds; nodes and events are separate documents. Attachment IDs in graph snapshots are reference metadata; never embed file bytes.

## Essential invariants
- Exactly one owner identity in roadmaps. Owner has an ADMIN membership. Transactions prevent owner removal and enforce membership cap.
- All live endpoints exist in the same active generation. Directed edges preserve order. Bidirectional endpoints are lexicographically canonicalized before insertion. Duplicate triples rejected. Opposite directed edges allowed.
- Delete node atomically removes incident edges, updates counts and detaches active attachment references. One NODE_DELETED event lists deleted edge and attachment IDs.
- All mutations advance global roadmap sequence once, including membership changes; entity revision changes only when that entity changes.
- events, graph/membership changes, and successful receipt commit together. No committed edit without replay evidence.
- receipt lifetime: retained for roadmap lifetime, including deleted roadmap tombstone retention; operation IDs may not be reused. Delete/retention jobs must preserve receipts at least 90 days after roadmap deletion. Clients never intentionally retry older than 30 days. Quotas bound receipt/event growth; archival policy is required at scale.
- Deleted roadmap is inaccessible immediately. IDs never reused. Cleanup and retention policy delete unreachable objects without deleting version-pinned files.
- TTL cleanup is eventual: authorization always compares expiration explicitly. Never treat record existence as proof of validity.
- Sensitive tokens stored as hashes; refresh token hashes use a keyed HMAC with rotation plan. IDs/tokens use cryptographically secure randomness.

## Atomic authorization and concurrency
The roadmap coordination write fences ACL modifications and graph writes. All mutation transactions read current ACL and conditionally write roadmap revision/aclEpoch. A racing removal forces transaction retry and a fresh permission check. Invite acceptance consumes token and creates membership atomically. Ownership transfer updates owner and both memberships atomically. Cannot remove a member and acknowledge their later edit from a stale permission cache.

Optimistic lock filter includes roadmapId, generationId, entity ID and expectedRevision. On zero matches return conflict (or not found without leaking inaccessible existence). Do not replace whole node documents from stale clients; patch explicitly provided fields only.

## Read paths
- My roadmaps: indexed memberships for user, stable cursor by createdAt/_id, batched roadmap fetch; filter deleted roadmap.
- Canvas: active generation nodes/edges; request-scoped batch attachment metadata; one snapshot read.
- Event replay: roadmapId and sequence > cursor ascending; bounded pages; check minReplaySequence and actual gaps.
- Versions: roadmapId, createdAt/_id cursor; members only.
Pagination default 20, max 100. Cursors are versioned opaque base64url JSON containing fixed sort tuple, scope and filter fingerprint; validate scope. UUID IDs do not require secret cursors.

## Retention and integrity
Event replay retention: 30 days initial default. Worker advances minReplaySequence conservatively before removing old events; missing sequences always force resync. Versions are retained until roadmap deletion in v1, subject to storage quota; no implicit version GC. Snapshot garbage collection requires no referencing version/job and expired leases. Jobs failing after partial staging clean the inactive generation, never active data. Periodic integrity audit checks counts, dangling edges, owner membership, version checksums, missing attachment objects and event gaps; repair actions must be audited.

## Migrations and operations
Versioned idempotent migration command creates validators/indexes and records checksum/version. Deploy migrations as a separate job; never race index creation on every API startup. Test upgrades from a previous fixture and rollback compatibility. Encrypt transport and managed storage; least-privileged DB/object credentials. Database point-in-time backups plus object retention must cover the same recovery window. Initial RPO target 15 minutes / RTO target 2 hours, subject to selected provider and demonstrated restore drill.

## Control-plane details
User profile idempotency uses the same actor/operation receipt key with nullable roadmapId. Upload reservation and job acceptance receipts store their grants/job IDs without a graph sequence; graph-visible completion obtains the next roadmap sequence. Worker events use the original operationId and initiating actorId. Shared/invite grant URLs in receipts contain sensitive bearer material: encrypt their stored response fields with a managed encryption key, scrub logs, and return receipts only to the authenticated authorized initiating actor. TTL/index cleanup of challenges/sessions does not replace explicit expiration checks.
