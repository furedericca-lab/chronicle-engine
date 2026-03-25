---
description: Task list for backend-release-line-closeout-2026-03-25 phase 3.
---

# Tasks: backend-release-line-closeout-2026-03-25 Phase 3

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

## Phase 3: Contract Test Restructure
Goal: Make the backend contract suite maintainable by grouping it into feature-scoped modules.

Definition of Done: All phase tasks are implemented, tested, and evidenced with commands and outputs.

Tasks:
- [x] T041 [Backend] Split `contract_semantics.rs` into feature-scoped modules under a dedicated `backend/tests/contract_semantics/` directory.
  - DoD: Tests are grouped by behavior and still compile as one `cargo test --test contract_semantics` integration-test target.
- [x] T042 [QA] Keep shared fixtures centralized and reusable across all feature modules.
  - DoD: Common request/mocking/storage helpers remain single-sourced in the root `backend/tests/contract_semantics.rs` entrypoint.
- [x] T043 [Security] Ensure the restructure does not drop rejection-path coverage for removed legacy surfaces.
  - DoD: Reflection-removal, auth, idempotency, and admin-token isolation tests remain present and passing.

Checkpoint: Phase 3 artifacts are merged, verified, and recorded in 4phases-checklist.md before next phase starts.

## Dependencies & Execution Order
- Phase 1 blocks all others.
- Phase 3 depends on completion of phases 1-2.
- Tasks marked [P] within this phase may run concurrently only when they do not touch the same files.
