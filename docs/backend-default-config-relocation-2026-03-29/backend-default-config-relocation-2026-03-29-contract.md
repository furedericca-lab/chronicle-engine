---
description: Single-contract scope for relocating the baked backend default config from deploy/ into backend/.
---

# backend-default-config-relocation-2026-03-29 Contract

## Context
- The backend runtime image now bakes a default config file into `/etc/chronicle-engine-backend/backend.toml`.
- Keeping the source file under `deploy/` works, but it mixes backend-owned default runtime config with deploy-only scaffolding.

## Findings
- `deploy/backend.toml.example` is currently both the baked image input and the deploy schema reference.
- `deploy/Dockerfile`, deploy docs, and active runtime docs reference that path.

## Goals / Non-goals
- Goals:
  - Move the baked default config source into `backend/`.
  - Rename it to reflect that it is the backend default config base, not a required host-mounted example file.
  - Update active build/deploy/runtime docs to the new path.
- Non-goals:
  - Changing the runtime container config destination.
  - Changing the environment override model.
  - Rewriting archive documents.

## Target files / modules
- `/root/.openclaw/workspace/plugins/openclaw-chronicle-engine/backend/backend.default.toml`
- `/root/.openclaw/workspace/plugins/openclaw-chronicle-engine/deploy/Dockerfile`
- `/root/.openclaw/workspace/plugins/openclaw-chronicle-engine/deploy/README.md`
- `/root/.openclaw/workspace/plugins/openclaw-chronicle-engine/README.md`
- `/root/.openclaw/workspace/plugins/openclaw-chronicle-engine/README_CN.md`
- `/root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/runtime-architecture.md`
- `/root/.openclaw/workspace/plugins/openclaw-chronicle-engine/docs/README.md`

## Constraints
- Keep `/etc/chronicle-engine-backend/backend.toml` as the runtime path inside the container.
- Keep verification gates and compose behavior unchanged.

## Verification plan
- `cargo test --manifest-path backend/Cargo.toml config::tests:: -- --nocapture`
- `cargo test --manifest-path backend/Cargo.toml --test admin_plane -- --nocapture`
- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`
- `docker compose -f deploy/docker-compose.yml config`
- `git diff --check`

## Rollback
- Restore the default config source path to `deploy/backend.toml.example` and revert the docs/Dockerfile references.

## Open questions
- None.
