---
description: Task list for backend-admin-plane-merge-fix-2026-03-29 phase 1.
---

# Tasks: backend-admin-plane-merge-fix-2026-03-29 Phase 1

## Input
- Canonical sources:
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-admin-plane-2026-03-27
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-admin-plane-merge-fix-2026-03-29

## Phase 1: Regression Freeze
Goal: Freeze the concrete regressions introduced by the merge and the contract that repairs must target.

Definition of Done: The scope docs identify the real regressions, the touched modules, the parent contract, and the verification line.

Tasks:
- [ ] T001 [Docs] Capture the confirmed merge regressions
  - DoD: The scope docs name the concrete regressions in `backend/src/admin/*`, `backend/src/lib.rs`, `backend/src/config.rs`, `backend/tests/contract_semantics.rs`, `backend/tests/admin_plane.rs`, `backend/web/src/*`, and `deploy/*`.
- [ ] T002 [QA] Freeze the required verification line
  - DoD: The docs list the backend/web/doc commands that must pass again before closeout.
- [ ] T003 [Security] Freeze the parent-scope compatibility boundary
  - DoD: The docs explicitly state that the repair target is `docs/backend-admin-plane-2026-03-27/`, not the drifted merged route/DTO behavior.
