---
description: Single-contract scope for refreshing retained architecture snapshots to the 2026-03-25 runtime boundary.
---

# snapshot-refresh-2026-03-25 Contract

## Context

The repository intentionally keeps two top-level architecture/design snapshots under `docs/`:

- `context-engine-split-*`
- `remote-memory-backend-*`

An audit on `2026-03-25` found that the retained snapshot dates and parts of the remote-memory-backend wording no longer matched the active repository boundary after the public reflection-surface removal.

## Findings

- `docs/context-engine-split-2026-03-17/` still mostly matched the current repo layout, but it contained stale backend file-layout examples in the future handoff note.
- `docs/remote-memory-backend-2026-03-18/` still described reflection recall / injection and `memoryReflection` as an active public contract even though the current runtime rejects `memoryReflection` config and exposes only behavioral-named recall routes.
- Top-level doc indexes still pointed to the old `2026-03-17` / `2026-03-18` snapshot suffixes.

## Goals / Non-goals

Goals:

- Refresh the retained snapshot folders to `2026-03-25`.
- Align snapshot wording with the current canonical runtime boundary in `docs/runtime-architecture.md`.
- Preserve these folders as historical design snapshots rather than phased execution scopes.

Non-goals:

- No backend or plugin code changes.
- No archive rewrites for older closed scopes except where top-level snapshot references must be updated.
- No attempt to rename storage-internal `reflection` schema fields.

## Target files / modules

- `docs/context-engine-split-2026-03-17/*` -> `docs/context-engine-split-2026-03-25/*`
- `docs/remote-memory-backend-2026-03-18/*` -> `docs/remote-memory-backend-2026-03-25/*`
- `docs/README.md`
- `docs/archive-index.md`

## Constraints

- Keep `docs/runtime-architecture.md` as the canonical runtime/source-of-truth document.
- Do not present the refreshed snapshots as active phased execution scopes.
- Keep wording consistent with current code in `index.ts`, `openclaw.plugin.json`, `src/context/*`, and `backend/src/*`.

## Verification plan

- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/doc_placeholder_scan.sh docs/archive/snapshot-refresh-2026-03-25`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/post_refactor_text_scan.sh docs/archive/snapshot-refresh-2026-03-25 README.md`
- `rg -n "context-engine-split-2026-03-17|remote-memory-backend-2026-03-18" docs README.md README_CN.md --glob '!docs/archive/**'`
- `git diff --check`

## Rollback

- Move the refreshed snapshot folders back to their prior names.
- Restore the previous snapshot wording in the touched docs and indexes.

## Open questions

- None. The active runtime/source-of-truth boundary is already established in `docs/runtime-architecture.md`.

## Implementation / Evidence

- Renamed `docs/context-engine-split-2026-03-17/` to `docs/context-engine-split-2026-03-25/`.
- Renamed `docs/remote-memory-backend-2026-03-18/` to `docs/remote-memory-backend-2026-03-25/`.
- Reduced `docs/context-engine-split-2026-03-25/` to the retained snapshot minimum:
  - `README.md`
  - `context-engine-split-contracts.md`
  - `context-engine-split-2026-03-25-technical-documentation.md`
- Updated top-level index entries in `docs/README.md` and `docs/archive-index.md`.
- Updated remote-memory-backend snapshot wording so it no longer describes reflection-named public recall routes or `memoryReflection` as supported active contract surfaces.
- Verification:
  - `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/doc_placeholder_scan.sh docs/archive/snapshot-refresh-2026-03-25` -> `[OK]`
  - `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/post_refactor_text_scan.sh docs/archive/snapshot-refresh-2026-03-25 README.md` -> `[OK]`
  - `rg -n "context-engine-split-2026-03-17|remote-memory-backend-2026-03-18" docs/README.md docs/archive-index.md docs/context-engine-split-2026-03-25 docs/remote-memory-backend-2026-03-25` -> no matches
  - `git diff --check` -> clean
