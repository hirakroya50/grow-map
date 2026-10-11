# Grow Map — API contract and behavior

## Contract authority
contracts/openapi.json is canonical for HTTP endpoints, bodies, statuses, authentication and transport envelopes. contracts/schema.graphql is canonical for roadmap queries/mutations/subscriptions. Both MUST be implemented: OpenAPI cannot enumerate the complete typed GraphQL operation system. Do not invent REST CRUD endpoints parallel to GraphQL. CI validates OpenAPI, validates SDL, compares exported runtime SDL semantically, and validates mobile operation documents. Documentation provides semantics and limits; a contradiction blocks the affected implementation until explicitly resolved.

## HTTP endpoints
| Method/path | Purpose | Authentication |
|---|---|---|
| POST /v1/auth/challenges | Exact single-use wallet signing message | anonymous, rate limited |
| POST /v1/auth/sessions | Verify challenge/signature and issue tokens | anonymous, rate limited |
| POST /v1/auth/sessions/refresh | Rotate opaque refresh token | refresh token in JSON |
| DELETE /v1/auth/session | Revoke current session and token family | Bearer |
| POST /graphql | Queries and mutations | operation-dependent |
| GET /graphql/ws | Upgrade to graphql-transport-ws | connection_init bearer token |
| GET /health/live | Process live | anonymous |
| GET /health/ready | Required dependencies ready | anonymous, minimal details |
| GET /openapi.json | HTTP contract | anonymous |
| GET /docs | Swagger UI for HTTP API | local/staging; disabled in prod |

Use HTTPS/WSS in deployment. No GraphQL GET or batch request arrays in v1. Swagger documents HTTP/auth/envelope; GraphiQL in local/staging explores GraphQL, disabled in production.

## Authentication
Solana Ed25519 public key, base58 32-byte wallet address; signature wire format base64 encoding of 64 bytes. Backend generates SIWS-compatible exact message with configured application domain/URI, random nonce, issue and expiration times. Store exact UTF-8 message; verify signed bytes without client-provided overrides. Challenge expires after 5 minutes, consumed atomically with session creation after successful verification. Sign-in is a signature, not a payment. Challenge request default limit 10/min/IP plus 5/min/wallet; verification 10/min/IP; tune behind trusted proxy configuration.

Access token TTL 15 minutes; opaque refresh token TTL 30 days; JWT access tokens contain sub, sid, issuer, audience, expiration. Validate algorithms against allowlist. Every protected request verifies active session, not just JWT signature, so logout works. Rotating refresh tokens allow only one successful successor; detected reuse revokes the family and requires login. Simultaneous mobile refresh is prevented with a client single-flight lock. Store refresh token in device secure storage; never logs/URL/localStorage. WebSocket connection_init within 5 seconds contains {authorization: "Bearer ..."}; no query-string token. Tokens never accepted in GraphQL variables.

## GraphQL operation inventory
Queries: me; myRoadmaps; roadmap; roadmapGraph; sharedRoadmapGraph; roadmapMembers; roadmapVersions; versionGraph; job; attachmentDownloadUrl.
Mutations: updateProfile; createRoadmap; updateRoadmap; deleteRoadmap; createNode; updateNode; deleteNode; createEdge; updateEdge; deleteEdge; createShareLink; revokeShareLink; createInvite; revokeInvite; acceptInvite; setMemberRole; removeMember; leaveRoadmap; transferOwnership; createVersion; restoreVersion; requestAttachmentUpload; completeAttachmentUpload; removeAttachment.
Subscription: roadmapChanges.
Names/arguments/types are defined exactly in SDL. Name a single operation in every mobile request. Avoid aliases creating write batches. Multiple mutation root fields are rejected in v1: one operationId and one transaction per write request.

## Validation and updates
UUID operation IDs required for all mutations. Fields omitted mean unchanged; null clears nullable description/label, but null rejects nonnullable name/status/position/tags/links. Empty array clears lists. Position values finite and between -1,000,000 and 1,000,000. Links HTTP(S) only, max 2,048 characters, no automatic server-side fetch. No filesystem paths from filenames. Tags trimmed/deduplicated deterministically; names nonblank after trimming. Pagination 1–100. Reject client-controlled ownerId, actorId, object keys, timestamps and revisions. Graph input caps from product.md. Initial request body cap 256 KiB; depth 10, cost 5,000, aliases 20; define cost weights in implementation and tests.

expectedRevision optimistic locking applies to metadata/entity updates and deletes. New node/edge returns revision equal to roadmap sequence. createVersion/restoreVersion require expected roadmap revision. Membership updates/removal require expected membership revision; setMemberRole can never modify owner. Create ops and transfers atomically recheck caps and ownership.

## Responses and errors
Revision is a decimal string scalar for 64-bit safety. Successful writes return operationId and committed sequence. Job-based writes return durable job acceptance, not completion. job polling shows QUEUED/RUNNING/SUCCEEDED/FAILED and result IDs. Errors contain stable code, requestId and safe details; conflict may expose currentRevision only to authorized users.

HTTP malformed JSON/request shape or invalid GraphQL validation: 400 with GraphQL envelope for /graphql. Missing/invalid presented bearer credentials: 401. A valid GraphQL request that fails domain authorization/validation/conflict during execution returns 200 with errors; do not treat 200 alone as success. Infrastructure admission failures use 413/429/503 with HTTP error envelope. Negotiate application/graphql-response+json or application/json. /graphql has optional transport auth because public reads exist; authentication is enforced per root field. Requests mixing permitted and forbidden query fields follow normal partial-result behavior. Mask inaccessible private resources as NOT_FOUND; operation requiring login returns UNAUTHENTICATED.

Codes: UNAUTHENTICATED, FORBIDDEN, NOT_FOUND, VALIDATION_ERROR, CONFLICT, IDEMPOTENCY_MISMATCH, LIMIT_EXCEEDED, GRAPH_TOO_LARGE, RESYNC_REQUIRED, RATE_LIMITED, STORAGE_NOT_READY, INTERNAL_ERROR. Auth HTTP errors additionally CHALLENGE_EXPIRED, CHALLENGE_USED, INVALID_SIGNATURE, SESSION_REVOKED.

## Sharing/privacy
Share URL puts token in fragment; mobile extracts it and sends GraphQL variable. Link token is 32 random bytes encoded base64url, hashed at rest, optional expiry, explicitly revocable. sharedRoadmapGraph rejects deleted/private roadmaps and invalid/revoked links. PUBLIC normal roadmapGraph may be anonymous. Node DTO omits actor wallet identities; actor IDs are visible only in authenticated member event/history access. Anonymous viewAttachmentUrl uses shareToken in attachmentDownloadUrl when unlisted; grants only READY objects referenced by the current graph. Version attachment download requires versionId and membership. Signed grants expire in 60 seconds; revocation cannot retract already issued grants before expiry. History and members remain membership-only.

## Subscriptions
Transport protocol graphql-transport-ws; one authenticated member session, max 5 active roadmap subscriptions/connection. Replay afterSequence (not inclusive); decimal string. Typed event upserts include public graph entity DTO, deletes include affected IDs, ACL changes cause authorized member refresh, GRAPH_RESET causes full snapshot reload. Payload excludes auth secrets. Document payload shapes in SDL. No promise of exactly-once delivery. Backpressure/replay rules in architecture.md.

## Attachments
Supported types JPEG, PNG, WebP, PDF, text/plain. Request reserves bytes, returns opaque uploadId, signed URL, PUT method and required headers with 5-minute expiry. complete verifies storage metadata/checksum and returns PROCESSING attachment; scanner later emits ATTACHMENT_UPDATED to READY/REJECTED. Pending upload reservation lasts 15 minutes. Calls are idempotent and authorize again at finalization. No public bucket or API proxy of file bytes. READY download grant expires in 60 seconds.

## Compatibility
HTTP endpoints under /v1 for auth; GraphQL evolves additively. Do not remove/rename fields or enum values without a documented deprecation window and supported-client usage evidence. Mobile operation codegen in CI. Error codes stable; clients handle unknown codes safely. Store a schema version in snapshots; migrators upgrade old snapshots before restoring.
