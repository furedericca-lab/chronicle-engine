---
description: Task list for backend-admin-plane-merge-fix-2026-03-29 phase 2.
---

# Tasks: backend-admin-plane-merge-fix-2026-03-29 Phase 2

## Input
- Canonical sources:
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-admin-plane-2026-03-27
  - /root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/backend-admin-plane-merge-fix-2026-03-29

## Phase 2: Backend and Contract Repair
Goal: Repair the merged backend implementation so tests compile again and admin routes/DTOs behave according to the frozen contract.

Definition of Done: The backend regains the old verification line and no longer exposes the confirmed merged drift.

Tasks:
- [ ] T021 [Backend] Restore the backend verification baseline
  - DoD: `backend/tests/contract_semantics.rs` and any other active test harnesses compile against the current config schema.
- [ ] T022 [Backend] Repair admin route and DTO drift
  - DoD: The backend route map and DTO field names converge on the `backend-admin-plane-2026-03-27` contract, including recall simulate and principal-scoped admin paths.
- [ ] T023 [Security] Add missing admin mutation idempotency
  - DoD: Governance review/promote writes require `Idempotency-Key` and route through admin-plane operation names with audit preservation.
- [ ] T024 [Backend] Make Settings truthful
  - DoD: The backend either performs validated atomic TOML writes or explicitly becomes a truthful read-only surface; no success-shaped no-op remains.
