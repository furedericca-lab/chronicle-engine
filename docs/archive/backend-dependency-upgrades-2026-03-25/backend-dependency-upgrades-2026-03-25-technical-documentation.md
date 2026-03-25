---
description: Canonical technical architecture for backend-dependency-upgrades-2026-03-25.
---

# backend-dependency-upgrades-2026-03-25 Technical Documentation

## Canonical Architecture
- The backend remains a single Rust service:
  - router/auth in `backend/src/lib.rs`
  - DTOs in `backend/src/models.rs`
  - storage/retrieval/distill execution in `backend/src/state.rs`

## Key Constraints and Non-Goals
- Preserve the behavioral-facing backend contract.
- Preserve `contract_semantics` as the backend contract test target.
- Do not rename the backend crate or runtime binary in this scope.

## Module Boundaries and Data Flow
- `clap` affects CLI/config entrypoints.
- `http`, `axum`, and `reqwest` affect request/response types, handler signatures, and provider clients.
- `rusqlite` affects distill job state and artifact persistence.
- `toml` affects backend config parsing.
- `lancedb` plus `arrow-*` affect storage/index/query paths and seeded test fixtures.
- Dependency upgrades in this scope must prefer a single compatible ecosystem over forcing globally newest crates that would create duplicate transport or Arrow type graphs.

## Interfaces and Contracts
- Phase 1 is intentionally allowed to be a no-op if the lockfile already resolves to the latest compatible versions.
- Later phases may change internal helper signatures, but not the backend’s public route contract.
- Phase 2 confirmed that `lancedb` 0.27.1 still preserves the backend route contract, but storage writes now need `Box<dyn RecordBatchReader + Send>` instead of passing `RecordBatchIterator` directly to `Table::add`.
- Phase 3 confirmed that `axum` 0.8 requires `{capture}` route syntax, so backend route templates must avoid the old `/:capture` form.
- Phase 4 confirmed that `lancedb 0.27.1` still anchors the storage stack to Arrow `57.x`; the backend therefore closes on `arrow-array` / `arrow-schema` `57.3.0` and `reqwest 0.12.28` as the latest single-compatible transport/storage line.

## Security and Reliability
- Upgrades must not weaken admin/runtime token separation.
- Upgrades must not degrade deterministic distill behavior or behavioral-guidance enforcement.
- Storage compatibility risk is concentrated in `lancedb` and `arrow-*`, so they are isolated into later phases.
- Single-version transport and Arrow graphs are preferred over parallel major/minor lines because duplicated HTTP or Arrow types materially raise integration and runtime risk.

## Test Strategy
- Use `contract_semantics` as the authoritative backend regression suite.
- Keep `npm test` as the plugin-side compatibility gate.
- Keep `clippy -D warnings` as the release-line hygiene gate.
