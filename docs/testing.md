# Grow Map — verification and release gates

## Principles
Tests assert observable behavior and data invariants. Use a real disposable MongoDB replica set and local S3-compatible service for integration tests; mocks alone cannot prove transaction/locking correctness. No real wallet secrets or production data in fixtures. CI containers wait for replica-set initialization and indexes before tests. No coverage percentage substitutes for critical-case evidence.

## Required suites
| Suite | Cases |
|---|---|
| Domain unit/property | Validation bounds, null vs omitted patch, edge canonicalization, role matrix, pagination cursor scope, finite coordinates, canonical request hash |
| Auth integration | Correct signature; wrong wallet/domain/message; expired/reused nonce; two verifications race; invalid algorithm/audience; refresh reuse revokes family; logout invalidates HTTP/socket |
| API contract | OpenAPI validation; runtime SDL equality; all mobile operations validate; examples conform; exact HTTP statuses; Revision rejects overflow/non-decimal; endpoint auth matrix |
| Database integration | Migration idempotency; unique indexes; owner invariant; counters; rollback on forced failure; no orphan edges; generation isolation |
| Authorization | Every operation × owner/admin/member/viewer/outsider/anonymous; wrong roadmap IDs; private existence masking; expired share/invite; admin cannot grant admin; version reads/downloads restricted |
| Concurrency | Same expectedRevision => one winner; independent nodes succeed after retry; duplicate operation exactly one receipt/event; different input same ID rejected; uncertain commit recovery; remove-member race fences writes |
| Reconnect | Snapshot/subscription boundary with racing write; replay ordering; overlap duplicates; retention gap; broker crash/restart; slow subscriber; ACL revoked mid-stream; token expiry |
| Attachments | Quota reservation race; actual size mismatch; type spoofing/scanner reject; deleted node during upload; expired upload; another user finalizes upload; READY-only download; version-pinned object retained |
| Versions/jobs | Exact snapshot revision; restore under concurrent edit fails; swap atomic; crash while staging; retried job one effect; GRAPH_RESET reload; no stale ABA writes; checksum mismatch |
| Mobile components | Loading/pending/error/conflict, local draft preserved, status labels, unauthorized controls hidden, reconnect, account cache reset |
| Android E2E | Wallet login on device; create/edit graph; relaunch; two-user sharing; viewer denied; revoke share; version restore; failed network |

## Critical invariant harness
Generate random create/update/delete/restore operations and assert: counts equal actual active graph; every edge references active nodes; owner remains admin; sequences monotonic; receipts/events match successful writes. Inject errors between transaction statements and before/after commit. Event delivery may repeat but committed state must not apply twice. Use deterministic seeds and keep failure traces.

## Test commands to implement
- pnpm lint / pnpm typecheck / pnpm test: mobile and workspace checks.
- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace: unit tests.
- pnpm test:integration: real services, API/auth/concurrency suites.
- pnpm contracts:check: OpenAPI + SDL + generated types + operation validation.
- pnpm test:e2e: Android workflow in staging/test environment.
- pnpm test:load: reproducible workload, report artifact.
Root Turbo task inputs include contracts, lockfiles, relevant configuration and codegen inputs. Integration/E2E/load tasks and dev tasks are not cached. Never reuse cached environment-dependent evidence.

## Performance acceptance (targets, not measured claims)
Same-region load generator and production-like staging, warm and cold results separately. Dataset 10,000 roadmaps with representative 100-node graphs plus 1,000-node maximum graphs. Start at 100 connected clients, 50 reads/s, 20 committed writes/s across roadmaps; hotspot run 10 writes/s on one roadmap. Ramp to saturation and report break point. 10-minute warmup + 30-minute steady run. Include varied field selections and replay bursts.
- Typical graph read (100 nodes) p95 <=150 ms server HTTP latency; max graph p95 <=500 ms.
- Committed small edit acknowledgement p95 <=200 ms; delivery to subscribed clients p95 <=300 ms from commit.
- Unexpected request failure <0.5%; zero lost committed edits or unauthorized successful operations.
- Monitor p99, memory growth, CPU, DB pool waits, transaction retries and queue/replay lag. No unbounded memory/queues.
- Android physical midrange device: typical canvas gestures target 60 fps, maximum graph remains usable >=30 fps; report frame time and memory. Measure graph open and edit round trip on Wi-Fi and realistic throttled mobile network separately; server targets do not imply end-to-end targets.
Budgets must be reevaluated from measurements; a miss requires profiling and an explicit decision, not fabricated performance numbers.

## CI and release
Every PR: formatting/lint/typecheck, unit and contracts, migrations and critical integration/concurrency suites. Nightly and release: full Android build, E2E, soak/reconnect/load/security dependencies scan. Release evidence lists command, commit, environment, result, and skipped checks. Dependency/version changes receive relevant contract checks. Secrets scan and vulnerable dependency triage required.

## Recovery/security drills
Kill API during writes; kill worker after lease acquisition; interrupt object upload; simulate DB transient outage; revoke role during edit; restore DB and version-referenced objects into fresh staging. Confirm RPO/RTO and rollback with old mobile build. Test trusted-proxy/rate-limit configuration, malicious GraphQL depth/aliases, upload MIME mismatch, token leakage in logs, and authorization bypass attempts. If platform/device checks cannot run, mark them UNVERIFIED and do not declare production-ready.
