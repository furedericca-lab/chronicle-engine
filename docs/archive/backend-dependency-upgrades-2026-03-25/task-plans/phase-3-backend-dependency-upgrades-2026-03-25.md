---
description: Task list for backend-dependency-upgrades-2026-03-25 phase 3.
---

# Tasks: backend-dependency-upgrades-2026-03-25 Phase 3

## Input
- Canonical sources:
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/README.md
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-dependency-upgrades-2026-03-25/backend-dependency-upgrades-2026-03-25-scope-milestones.md
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-dependency-upgrades-2026-03-25/backend-dependency-upgrades-2026-03-25-technical-documentation.md
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-dependency-upgrades-2026-03-25/backend-dependency-upgrades-2026-03-25-contracts.md

## Canonical architecture / Key constraints
- Keep architecture aligned with backend-dependency-upgrades-2026-03-25 scope docs and contracts.
- Keep provider/runtime/channel boundaries unchanged unless explicitly in scope.
- Keep security and test gates in Definition of Done.

## Format
- [ID] [P?] [Component] Description
- [P] means parallelizable.
- Valid components: Backend, Frontend, Agentic, Docs, Config, QA, Security, Infra.
- Every task must have a clear DoD.

## Phase 3: Framework and Persistence Upgrades
Goal: Upgrade `axum`, `reqwest`, `rusqlite`, and `toml`.

Definition of Done: All phase tasks are implemented, tested, and evidenced with commands and outputs.

Tasks:
- [x] T041 [Backend] Upgrade `axum` and resolve handler/signature/test fallout.
  - DoD: request handlers and integration tests compile on the new framework version.
- [x] T042 [Backend] Upgrade `reqwest`, `rusqlite`, and `toml` and resolve provider/persistence/config fallout.
  - DoD: provider clients, SQLite job state, and config loading remain stable under current tests.
- [x] T043 [Security] Reconfirm auth and failure-boundary behavior after the framework/transport batch.
  - DoD: token boundaries and fail-open/fail-closed tests remain passing.

Checkpoint: Phase 3 artifacts are merged, verified, and recorded in 4phases-checklist.md before next phase starts.

## Execution Record
- Batch date: 2026-03-25
- Result:
  - `axum` upgraded from `0.7` to `0.8.8`.
  - `rusqlite` upgraded from `0.32` to `0.39.0`.
  - `toml` upgraded from `0.8` to `1.1.0`.
  - `reqwest` was evaluated against `0.13.2`, but the backend was kept on the single-compatible `0.12.28` line to avoid a dual-`reqwest` graph alongside `lancedb 0.27.1`.
- Fallout handled:
  - `axum` 0.8 rejects `/:param` route syntax. The distill status route was migrated to `/v1/distill/jobs/{job_id}`.
  - Auth and route-boundary tests were rerun after the framework change to confirm no regression in principal enforcement or fail-open/fail-closed behavior.
- Evidence commands:
  - `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`
  - `cargo test --manifest-path backend/Cargo.toml --test contract_semantics -- --nocapture`
  - `npm test`

## Dependencies & Execution Order
- Phase 1 blocks all others.
- Phase 3 depends on completion of phases 1-2.
- Tasks marked [P] within this phase may run concurrently only when they do not touch the same files.
