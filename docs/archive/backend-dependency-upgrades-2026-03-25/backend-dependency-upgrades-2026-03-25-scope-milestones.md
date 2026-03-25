---
description: Scope boundaries and milestones for backend-dependency-upgrades-2026-03-25.
---

# backend-dependency-upgrades-2026-03-25 Scope and Milestones

## In Scope
- Audit current backend Rust dependencies against crates.io stable releases.
- Record that the low-risk semver-compatible set already resolves to the latest compatible versions in `backend/Cargo.lock`:
  - `anyhow`, `futures`, `parking_lot`, `regex`, `serde`, `serde_json`, `tokio`, `uuid`, `tempfile`, `tower`
- Plan and execute higher-risk dependency upgrades in bounded batches:
  - `clap`, `http`, `lancedb`
  - `axum`, `reqwest`, `rusqlite`, `toml`
  - `arrow-array`, `arrow-schema`

## Out of Scope
- TypeScript/npm dependency upgrades.
- Backend crate/binary renaming.
- Storage schema migrations unrelated to dependency compatibility.
- Runtime contract expansion unrelated to dependency fallout.

## Milestones
- Milestone 1: complete the dependency audit and record the no-op result for the safe set.
- Milestone 2: upgrade `clap`, `http`, and `lancedb`.
  - Completed on 2026-03-25 by aligning `clap`/`http` to the already-resolved lockfile versions and lifting `lancedb` to `0.27.1` with one boxed-reader compatibility fix.
- Milestone 3: upgrade `axum`, `reqwest`, `rusqlite`, and `toml`.
  - Completed on 2026-03-25 by moving to `axum 0.8.8`, `rusqlite 0.39.0`, and `toml 1.1.0`, while keeping `reqwest` on the single-compatible `0.12.28` line shared with `lancedb 0.27.1`.
- Milestone 4: upgrade `arrow-array` and `arrow-schema`, then close with full verification.
  - Completed on 2026-03-25 as a single-compatibility closeout on `arrow-array` / `arrow-schema` `57.3.0`; `58.x` was explicitly deferred because `lancedb 0.27.1` still anchors the Arrow ecosystem to `57.x`.

## Dependencies
- `backend/Cargo.toml`
- `backend/Cargo.lock`
- `backend/src/lib.rs`
- `backend/src/models.rs`
- `backend/src/state.rs`
- `backend/tests/contract_semantics.rs`
- `deploy/Dockerfile`

## Exit Criteria
- All planned upgrade groups either landed successfully or were closed on the latest single-compatible versions with documented ecosystem blockers.
- Backend contract behavior remains stable and test-backed.
- Deployment docs and Docker build remain compatible with the resulting dependency set.
