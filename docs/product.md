# Grow Map — product specification

Status: implementation baseline v1. Decisions below are requirements; numerical limits are initial configurable defaults to validate in staging.

## Objective
Help students and working professionals organize knowledge, resources, and work as a visual roadmap. A roadmap is a collaborative graph of notes with progress states. The first client is a React Native Android app intended for Solana mobile distribution. Backend: Rust. Wallet authentication: Solana. Payments and on-chain storage are outside v1.

## User journeys
1. Sign in by proving ownership of a wallet; create a private roadmap.
2. Add notes, links, and attachments; arrange nodes; connect nodes.
3. Set a node to Not done, Cooking (In progress), Needs review, Complete, or Cancelled.
4. Invite collaborators; edit together; reconnect without losing committed changes.
5. Save a named version; inspect a historical graph; restore its graph content.
6. Share a read-only link or publish a public roadmap.

## MVP scope
- Wallet sessions, profile display name, multiple roadmaps.
- Graph pan, pinch zoom, node drag, selection, accessible list alternative.
- Node name, plain-text description, tags, HTTP(S) links, attachment references, position, shared status.
- Directed and bidirectional pairwise edges; optional label. Cycles allowed because this is also a knowledge map. No self-loops. A directed A→B and B→A may coexist; identical edge triples may not.
- Roadmap-scoped admin/member/viewer permissions; separate owner identity.
- Private, unlisted, public visibility; revocable read-only share links.
- Atomic edits, optimistic UI, conflict feedback, ordered event replay.
- Named immutable versions; restoration preserving membership and sharing configuration.
- Search within a loaded graph; cursor-paginated roadmap and history lists.

## Exclusions
CRDT rich-text co-editing, offline automatic merging, hyperedges, automatic prerequisite completion, personal progress on someone else's roadmap, wallet payments, marketplace, AI generation, enterprise organizations, and arbitrary large graphs. Queueing offline edits is not promised: preserve local draft edits and require reconciliation on reconnect.

## Roles
| Action | Owner | Admin | Member | Viewer / public reader |
|---|---|---|---|---|
| Read | yes | yes | yes | yes when authorized |
| Edit graph | yes | yes | yes | no |
| Save named version | yes | yes | yes | no |
| Manage members / invites / sharing | yes | yes, restricted | no | no |
| Change roadmap metadata | yes | yes | no | no |
| Restore version | yes | yes | no | no |
| Delete roadmap / transfer ownership | yes | no | no | no |

Admins can manage only member/viewer accounts and issue only member/viewer invites. Owner alone grants/revokes admin. Owner is effectively admin but may not be removed or leave. Ownership transfer requires an existing active member and demotes the prior owner to admin. Private access requires membership; unlisted access requires membership or a share token; public permits anonymous graph reads. Member lists, history, and download grants never become public automatically.

## UX acceptance
A confirmed save remains after relaunch. Saving/pending/conflicted/error states are visible. Never show an unacknowledged edit as durably saved. Conflicts preserve the local draft and offer reload/reapply. Public reads hide wallet identities and actor details. Mobile cache is user-scoped and cleared on logout. Attachment links expire. Canvas gestures remain local; only final positions are persisted. Large graph limit violations produce actionable errors.

## Initial limits
1,000 active nodes, 3,000 active edges, 50 active members per roadmap; 20 attachments per node; 10 MiB per attachment; 100 MiB active uploaded bytes per roadmap; title/name 200 characters; description 10,000 UTF-8 characters; 20 tags of 40 characters; 20 links per node. Counts and quotas are checked transactionally. Retained versions consume storage too; a separate retained-storage quota is required before launch.

## Release gate
All critical acceptance tests pass, staging mobile smoke tests pass on physical Android, wallet flow works with supported wallets, measured performance meets docs/testing.md, backup restoration is demonstrated, and privacy/store requirements are checked against current official distribution documentation before submission.
