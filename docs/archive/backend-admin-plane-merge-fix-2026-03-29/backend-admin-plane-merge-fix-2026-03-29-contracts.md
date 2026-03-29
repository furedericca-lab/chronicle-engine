---
description: Contracts for repairing the regressions and contract drift introduced by the merged admin-plane implementation.
---

# backend-admin-plane-merge-fix-2026-03-29 Contracts

## Context

- The merged admin-plane PR landed the first bundled `/admin` and `/admin/api/*` surface, but the resulting tree is not at an executable closeout line.
- Verified regressions already exist in the merged state:
  - `backend/tests/contract_semantics.rs` no longer compiles because `ServerConfig` now requires `admin_assets_path`.
  - the Settings page edits JSON while the backend expects TOML and the backend write path is still a stub.
  - the Recall Lab frontend sends `mode_override` while the backend DTO requires `mode`.
  - governance review/promote mutations do not implement the admin-plane idempotency contract.
  - the implemented admin route family names drift from the frozen `backend-admin-plane-2026-03-27` contract.

## API Contracts

- `/v1/*` runtime behavior remains untouched in this scope except for restoring broken test/config compatibility and shared helper cleanup needed to keep the admin-plane merge from regressing the runtime verification line.
- `/admin/api/*` must converge on the `backend-admin-plane-2026-03-27` contract rather than defining a second naming surface ad hoc.
- Route naming must be normalized back to the documented admin-plane family:
  - `/admin/api/principals/{principalId}/distill/jobs`
  - `/admin/api/principals/{principalId}/distill/jobs/{jobId}`
  - `/admin/api/principals/{principalId}/session-transcripts`
  - `/admin/api/principals/{principalId}/session-transcripts/{transcriptId}`
  - `/admin/api/principals/{principalId}/governance/artifacts`
  - `/admin/api/principals/{principalId}/governance/artifacts/{artifactId}/review`
  - `/admin/api/principals/{principalId}/governance/artifacts/{artifactId}/promote`
  - `/admin/api/settings/runtime-config`
- `/admin` static asset serving must resolve to a real built asset directory in local dev, tests, and the runtime container image.

## Shared Types / Schemas

- `ServerConfig` additions must not break existing backend test targets; all active backend test harnesses must compile with the current config schema.
- The Settings editor and backend API must agree on a single editable source format for the MVP.
- This repair scope chooses TOML end to end because the backend config source of truth is TOML and the active admin-plane contract already froze `runtime-config` as TOML-backed.
- The Recall Lab request DTO must converge on a single field name:
  - `mode`
- Admin governance review/promote mutations are in-scope admin write operations and therefore must require `Idempotency-Key` just like the documented admin memory mutations.

## Event and Streaming Contracts

- No WebSocket or SSE work is in scope.
- Existing polling semantics stay unchanged.

## Error Model

- Settings write failures must fail explicitly; the API must not return success-shaped responses for a no-op write path.
- Frontend save flows must surface backend validation errors as backend validation errors, not as silent local success banners.

## Validation and Compatibility Rules

- This scope is complete only when the merged tree regains the previous backend verification line:
  - `cargo test --manifest-path backend/Cargo.toml --test contract_semantics -- --nocapture`
  - `cargo test --manifest-path backend/Cargo.toml --test admin_plane -- --nocapture`
  - `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`
  - `npm --prefix backend/web ci`
  - `npm --prefix backend/web run build`
  - `git diff --check`
- The scope must update the active admin-plane docs so the implementation and the documented route/config contracts stop drifting.
- This scope must not expand the admin-plane ambition beyond repair and convergence work:
  - no new product pages
  - no new admin subsystems
  - no new external services
