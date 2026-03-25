---
description: Task list for backend-release-line-closeout-2026-03-25 phase 2.
---

# Tasks: backend-release-line-closeout-2026-03-25 Phase 2

## Input
- Canonical sources:
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/README.md
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-release-line-closeout-2026-03-25/backend-release-line-closeout-2026-03-25-scope-milestones.md
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-release-line-closeout-2026-03-25/backend-release-line-closeout-2026-03-25-technical-documentation.md
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-release-line-closeout-2026-03-25/backend-release-line-closeout-2026-03-25-contracts.md

## Canonical architecture / Key constraints
- Keep architecture aligned with backend-release-line-closeout-2026-03-25 scope docs and contracts.
- Keep provider/runtime/channel boundaries unchanged unless explicitly in scope.
- Keep security and test gates in Definition of Done.

## Format
- [ID] [P?] [Component] Description
- [P] means parallelizable.
- Valid components: Backend, Frontend, Agentic, Docs, Config, QA, Security, Infra.
- Every task must have a clear DoD.

## Phase 2: Release-Line Cleanup
Goal: Eliminate the active backend engineering blockers that prevented release-line signoff.

Definition of Done: All phase tasks are implemented, tested, and evidenced with commands and outputs.

Tasks:
- [x] T021 [Backend] Remove current Clippy failures from active backend code.
  - DoD: `backend/` passes `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] T022 [QA] Replace panic-style trace invariant unwraps with structured internal errors.
  - DoD: Trace wrappers return `AppError` instead of using `expect(...)`.
- [x] T023 [Security] Preserve auth, route, and storage-boundary behavior while making the cleanup.
  - DoD: No route/config compatibility widens, and the contract suite still passes.

Checkpoint: Phase 2 artifacts are merged, verified, and recorded in 4phases-checklist.md before next phase starts.

## Dependencies & Execution Order
- Phase 1 blocks all others.
- Phase 2 depends on completion of phases 1-1.
- Tasks marked [P] within this phase may run concurrently only when they do not touch the same files.
