---
description: Task list for backend-dependency-upgrades-2026-03-25 phase 2.
---

# Tasks: backend-dependency-upgrades-2026-03-25 Phase 2

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

## Phase 2: Medium-Risk Runtime Upgrades
Goal: Upgrade `clap`, `http`, and `lancedb` with bounded fallout.

Definition of Done: All phase tasks are implemented, tested, and evidenced with commands and outputs.

Tasks:
- [x] T021 [Backend] Upgrade `clap` and resolve CLI/config entrypoint fallout.
  - DoD: backend CLI still builds and accepts the current `--config` usage.
- [x] T022 [Backend] Upgrade `http` and resolve request/response helper fallout.
  - DoD: router handlers and test request builders compile cleanly.
- [x] T023 [Backend] Upgrade `lancedb` and resolve storage/index/query fallout short of Arrow major upgrades.
  - DoD: storage paths and `contract_semantics` stay green without schema regressions.

Checkpoint: Phase 2 artifacts are merged, verified, and recorded in 4phases-checklist.md before next phase starts.

## Execution Record
- Batch date: 2026-03-25
- Result:
  - `clap` manifest constraint tightened to `4.6.0`, matching the already-resolved lockfile version.
  - `http` manifest constraint tightened to `1.4.0`, matching the already-resolved lockfile version.
  - `lancedb` upgraded from `0.26.2` to `0.27.1`, pulling the corresponding Lance/DataFusion patchline.
- Fallout handled:
  - `lancedb` 0.27.1 no longer accepts raw `RecordBatchIterator` in `Table::add`.
  - Backend insert path and legacy-schema test helper were updated to pass `Box<dyn RecordBatchReader + Send>` instead.
- Evidence commands:
  - `cargo update --manifest-path backend/Cargo.toml -p lancedb -p clap -p http`
  - `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`
  - `cargo test --manifest-path backend/Cargo.toml --test contract_semantics -- --nocapture`
  - `npm test`

## Dependencies & Execution Order
- Phase 1 blocks all others.
- Phase 2 depends on completion of phases 1-1.
- Tasks marked [P] within this phase may run concurrently only when they do not touch the same files.
