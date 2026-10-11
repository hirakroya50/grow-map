# Grow Map — Codex implementation instructions

## Mission and source of truth

Build the React Native Android app and Rust API in this monorepo. Read docs/product.md, architecture.md, data-model.md, api-design.md, testing.md and both contracts before changing affected behavior. contracts/openapi.json is binding for HTTP; contracts/schema.graphql is binding for GraphQL. Neither replaces the other. Do not create undocumented endpoint variants. Resolve contradictory requirements before implementing the affected behavior; continue independent work.

## Work method

Implement one vertical slice at a time. Inspect existing code first. Describe acceptance criteria, implement, add meaningful tests, run relevant checks, and fix failures. Keep unrelated changes out. Do not fabricate checks, latency, security guarantees, or production readiness. Use clear domain modules, typed Rust errors and TypeScript strict mode. Separate resolver/handler transport code from domain services and repositories.

## Non-negotiable invariants

- Authorization is server-side and roadmap-scoped for every read/write/download/subscription.
- Private records must not leak through IDs, errors, aliases, batching, logs or caches.
- Transactions use current ACL and roadmap coordination fence. No blind last-write-wins.
- expectedRevision checks include active generation. Preserve null vs omitted fields explicitly.
- Each successful write has one durable sequence/event and idempotency receipt atomically.
- Bounded transaction retries, bounded queues, bounded query and file sizes.
- Acknowledge only committed state; job acceptance is not job completion.
- No external storage calls inside a database transaction. Durable jobs for async work.
- No secrets committed; no auth/private payload logging; do not auto-fetch arbitrary user links.
- Never store attachment bytes or an unbounded entire graph/history in one document.

## Contract workflow

Contract-first: implement the committed desired SDL and OpenAPI, generate mobile types, validate operation documents. A contract change requires updating associated docs/tests/examples in the same change. Export runtime SDL and compare semantically in CI. No parallel REST graph CRUD. WebSocket behavior follows documented protocol; OpenAPI upgrade documentation is not a subscription schema.

## Tooling and setup

pnpm + Turborepo JS workspace; Cargo Rust workspace with lockfile. Rust package may expose package.json scripts calling Cargo. Pin compatible supported dependencies after checking official docs; do not assume versions in this spec. Local MongoDB replica set and S3-compatible store. Commit environment examples without credentials. Local dev commands start services reliably and document Android emulator/device backend URLs (device localhost is not host localhost). Mobile wallet modules may require native builds; do not promise Expo Go support without verification.

## Completion report

State behavior changed, tests/checks actually executed, failures/skips and remaining risks. Run checks in docs/testing.md relevant to the change. Before claiming release readiness, meet all release gates. If CI/devices/services unavailable, record limitation and provide exact verification steps. Do not remove tests or weaken authorization/locking to make checks pass.

## Suggested slices

1. Scaffold workspaces/local infra/CI; GraphQL API and mobile connectivity.
2. Auth HTTP contract and session security tests.
3. Private roadmaps and server authorization.
4. Nodes/edges/canvas with revision/idempotency integration tests.
5. Roles/invites/read-only sharing.
6. Uploads/downloads and jobs.
7. Durable event replay and collaboration.
8. Versions/generation restore.
9. Performance, recovery and physical-device release checks.
