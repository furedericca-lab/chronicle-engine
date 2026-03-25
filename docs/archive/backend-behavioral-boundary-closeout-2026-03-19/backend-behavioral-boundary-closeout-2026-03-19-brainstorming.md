---
description: Brainstorming and decision framing for backend-behavioral-boundary-closeout-2026-03-19.
---

# backend-behavioral-boundary-closeout-2026-03-19 Brainstorming

## Problem

The repo now presents governance + behavioral-guidance naming on the active plugin/runtime surface, but the backend wire/storage layer still exposes `reflection` naming in several places:

- HTTP routes: `/v1/recall/reflection`, `/v1/debug/recall/reflection`
- DTO fields: `reflectionCount`
- persistence fields: `reflection_kind`
- category/storage semantics: `category=reflection`
- internal temporary/session naming such as `temp:memory-reflection`

This is no longer a correctness blocker, but it leaves the architecture in a half-renamed state. The remaining debt has two costs:

1. operator/developer confusion because active docs and code use two semantic vocabularies for one feature lane;
2. long-term migration cost because future refactors must keep translating behavioral terminology onto a frozen reflection contract.

## Scope

Open a dedicated phased scope to decide and, if accepted, implement the remaining backend-facing rename and the related internal legacy-semantic cleanup.

Primary targets:

- backend route/DTO/storage naming still using `reflection`
- client/tool adapters that call those routes or map those DTOs
- active docs that currently document the compatibility boundary as intentional
- small internal legacy semantic names (for example `temp:memory-reflection`) that should be cleaned up alongside the contract change

## Constraints

- The repo is already on `main`; any change must preserve runtime correctness and testability.
- This scope is larger-risk than the previous closeout because it can touch API paths, persisted fields, and compatibility behavior.
- Existing rows/tables may already contain `category=reflection` and `reflection_kind`; data compatibility must be explicitly handled rather than assumed away.
- Active docs must not claim the rename is complete until code/tests/compatibility rules are aligned.

## Options

### Option A — Keep the backend compatibility boundary indefinitely

Keep `/v1/recall/reflection`, `reflection_kind`, `reflectionCount`, and `category=reflection` forever, but continue presenting behavioral wording only on the outer plugin/runtime surface.

Pros:
- smallest risk
- no migration/data-path work
- preserves current tests and DTOs

Cons:
- permanent semantic split
- internal naming debt remains
- future refactors must keep translating between vocabularies

### Option B — Rename only runtime-internal leftovers, keep backend contract frozen

Clean `temp:memory-reflection`, helper names, and any remaining plugin-side legacy strings, but do not rename backend routes/DTO/storage.

Pros:
- low risk
- removes some local confusion

Cons:
- leaves the highest-value debt intact
- active docs still need to explain the backend reflection boundary
- only partially solves the stated problem

### Option C — Full backend behavioral-boundary closeout

Rename the backend-facing recall/debug route names, stats field names, trace kinds where appropriate, and storage-facing semantic names where safe; define an explicit compatibility/migration policy for existing persisted data and any transitional acceptance/rejection behavior.

Pros:
- resolves the last major semantic split
- makes active runtime + backend vocabulary consistent
- reduces long-term translation overhead

Cons:
- highest implementation and compatibility risk
- may require additive compatibility window or data migration logic
- needs broader test and doc updates

## Decision

Choose **Option C** as the target scope, but structure it as a phased closeout so we can freeze contracts before code edits.

Reasoning:
- The remaining debt is now concentrated and easy to name.
- The previous closeout already removed public shims and aliases, so the biggest remaining mismatch is the backend contract itself.
- This is exactly the kind of change that benefits from an auditable docs-first scope with explicit compatibility rules.

## Risks

1. **Data compatibility risk**
   - persisted rows and table schemas may still use `reflection_kind` / `category=reflection`
2. **Client compatibility risk**
   - backend-client and tests currently call `/v1/recall/reflection`
3. **Trace/debug drift**
   - route rename, trace kind rename, and tool output normalization must stay aligned
4. **Archive/doc drift**
   - active docs must stop documenting the old compatibility boundary only after the code lands

## Open Questions

1. Should this scope support a temporary dual-route acceptance window (`/v1/recall/reflection` + `/v1/recall/behavioral`) or make a clean breaking rename?
2. Should persisted `category=reflection` / `reflection_kind` be migrated physically, aliased logically, or frozen as storage internals even if the HTTP contract changes?
3. How much historical compatibility is required for old clients, tests, and stored data?
