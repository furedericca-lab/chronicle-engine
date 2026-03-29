---
description: Task list for backend-admin-plane-merge-fix-2026-03-29 phase 3.
---

# Tasks: backend-admin-plane-merge-fix-2026-03-29 Phase 3

## Input
- Canonical sources:
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-admin-plane-2026-03-27
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-admin-plane-merge-fix-2026-03-29

## Phase 3: Web and Deploy Alignment
Goal: Align the SPA, static asset path, and active docs with the repaired backend contract.

Definition of Done: The admin web client, deploy surface, and active docs all speak the same repaired contract.

Tasks:
- [ ] T041 [Frontend] Repair the admin web client contract usage
  - DoD: Settings and Recall Lab speak the repaired backend DTO and route surface without field-name or format mismatches.
- [ ] T042 [Infra] Align admin asset path behavior
  - DoD: backend defaults, tests, deploy config, and Docker image copy destination agree on one working asset location for `/admin`.
- [ ] T043 [Docs] Refresh active docs and operator references
  - DoD: `backend-admin-plane-2026-03-27` and related active docs no longer describe routes/settings semantics that disagree with the repaired code.
