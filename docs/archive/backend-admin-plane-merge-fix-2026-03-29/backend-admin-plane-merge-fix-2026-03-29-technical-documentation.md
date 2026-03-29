---
description: Technical execution notes for converging the merged admin-plane implementation back to the intended contract.
---

# backend-admin-plane-merge-fix-2026-03-29 Technical Documentation

## Canonical Architecture

- This is a repair scope for the already-merged bundled admin plane.
- The parent architecture does not change:
  - Rust backend remains the sole authority.
  - `/v1/*` remains the runtime data plane.
  - `/admin` and `/admin/api/*` remain the bundled operator surface.
- The purpose of this scope is convergence, not expansion.

## Key Constraints and Non-Goals

- Do not add new admin pages or new operator subsystems in this repair scope.
- Do not revise the parent `backend-admin-plane-2026-03-27` contract downward to bless the merged drift.
- Do not keep fake-success paths in settings or write APIs.

## Module Boundaries and Data Flow

- `backend/src/config.rs`
  - must keep the config schema coherent across runtime code and all active backend tests.
- `backend/src/lib.rs`
  - must expose one canonical admin route surface matching the active contract.
- `backend/src/admin/routes.rs` and `backend/src/state.rs`
  - must keep admin mutation semantics truthful, auditable, and idempotent where the contract requires it.
- `backend/web/src/*`
  - must speak the repaired backend DTO and route contract directly.
- `deploy/Dockerfile` and deploy config
  - must agree with backend asset-path expectations for `/admin`.

## Interfaces and Contracts

### Settings

- The merged tree previously presented editable settings in the UI while the backend write handler was a no-op.
- The repaired implementation now converges on one truthful behavior:
  - validate TOML before replacement
  - persist the config file atomically
  - refresh the backend's in-memory config view
  - emit audit records
  - return `restart_required=true` instead of implying hot-apply
- The repair scope does not downgrade Settings to read-only.

### Recall Simulation

- Recall simulation must use a single `mode` field across frontend and backend.
- The admin simulation path must remain side-effect-free as defined by the parent scope.

### Governance

- Governance review/promote mutations are admin write operations and therefore must use the admin-plane idempotency model.
- Audit emission remains required.

### Static Admin Assets

- The backend default `server.admin_assets_path`, test harnesses, deploy example config, and Docker image copy destination must point to one coherent built-assets location.
- The runtime container must not rely on an undocumented path override to make `/admin` work.

## Security and Reliability

- Truthful failure beats fake success.
- Contract convergence beats temporary dual-surface drift unless a short-lived shim is strictly needed to recover the merged tree safely.
- The runtime verification line remains part of reliability; the merge is not repaired while `contract_semantics` is broken.

## Test Strategy

- Restore the old baseline before claiming admin-plane quality:
  - `contract_semantics`
  - `admin_plane`
  - `clippy -D warnings`
- Keep SPA build in the gate because the merged issue set includes frontend/backend contract mismatches that Rust-only tests do not cover.
