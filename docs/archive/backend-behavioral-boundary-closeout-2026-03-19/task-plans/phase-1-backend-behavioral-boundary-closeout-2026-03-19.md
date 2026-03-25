---
description: Task list for backend-behavioral-boundary-closeout-2026-03-19 phase 1.
---

# Tasks: backend-behavioral-boundary-closeout-2026-03-19 Phase 1

## Phase 1: Freeze compatibility and rename policy

Goal: decide whether this scope is breaking, dual-acceptance, or storage-internal-only before implementation starts.

Definition of Done: contracts/docs/checklist all agree on the chosen policy, touched modules are enumerated, and required verification commands are frozen.

Tasks:
- [x] T001 [Docs] Classify every active `reflection` surface as rename / retain / alias / reject.
  - DoD: routes, DTOs, trace kinds, storage fields, category values, and temp runtime keys are listed explicitly in scope docs.
- [x] T002 [Backend] Decide route/DTO compatibility behavior.
  - DoD: docs say whether `/v1/recall/reflection` remains accepted, becomes alias-only, or is removed.
- [x] T003 [Backend] Decide storage compatibility behavior.
  - DoD: docs say whether `category=reflection` and `reflection_kind` are migrated, aliased internally, or retained intentionally.
- [x] T004 [QA] Freeze the minimal verification matrix.
  - DoD: targeted Node tests, backend tests, doc scans, and diff hygiene commands are listed concretely.

Decision summary:
- Use bounded dual-acceptance for HTTP routes.
- Make behavioral-facing naming canonical on active client/tool/doc surfaces.
- Keep storage-facing `reflection` naming internal in this scope unless a minimal safe alias is required.
- Remove internal legacy runtime naming where it no longer serves compatibility.

Checkpoint: completed; code implementation may start.
