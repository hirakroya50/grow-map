# Grow Map implementation specifications

This package is a contract and engineering specification, not an implemented or tested application.

Copy AGENTS.md, docs/, and contracts/ into the root of the grow-map repository. Keep both machine-readable contracts in Git. Authentication uses REST HTTP endpoints; roadmap functionality uses GraphQL. OpenAPI alone cannot specify the GraphQL field/type system.

## Start Codex with this prompt

Read AGENTS.md, all docs, contracts/openapi.json and contracts/schema.graphql. Treat both contracts as binding.

First identify contradictions that block scaffolding.

Scaffold the pnpm/Turborepo and Cargo monorepo, React Native TypeScript Android app, Axum/async-graphql Rust API, MongoDB replica-set development infrastructure and object storage.

Implement health endpoints, contract serving and a mobile public graph read using an explicitly seeded public roadmap in development only.

Build an SDL export/semantic contract comparison check.

Do not claim unsupported contract fields are implemented: report scaffold-only paths and do not deploy this skeleton to production.

Implement wallet authentication and private-roadmap authorization in the next slice before user data is enabled.

Add CI, environment examples and reproducible startup commands.

Run available checks and report exact results and skipped checks.

Do not generate all product features in one unreviewable change.

## Files

- AGENTS.md: Codex instructions.
- docs/product.md: scope, journeys, permission matrix, limits.
- docs/architecture.md: transaction, event, job and restore design.
- docs/data-model.md: collections, indexes and invariants.
- docs/api-design.md: endpoints and API semantics.
- docs/testing.md: tests, SLO targets and release gates.
- contracts/openapi.json: HTTP contract.
- contracts/schema.graphql: typed GraphQL contract.

Performance numbers are acceptance targets, not measurements.

Device/platform release requirements and dependency compatibility must be verified during implementation.

These contracts deliberately refine the earlier discussion:

- REST authentication instead of duplicate GraphQL authentication.
- Asynchronous version jobs.
- String-encoded revisions.
- Globally increasing entity revisions across restoration.
