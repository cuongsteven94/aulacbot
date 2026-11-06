# AuLacBot — product notes

AuLacBot is a rebrand of CopilotKit OpenDuet (MIT License, © CopilotKit — see `LICENSE`, kept
unchanged). Built 2026-11-07 from upstream commit `f52288f`.

## Decisions (locked)

- **Brand:** AuLacBot. Tenant package: `openbot/examples/aulacbot` (a copy of
  `openbot/examples/openduet`), selected by `TENANT_PACKAGE_DIR=../examples/aulacbot` in the
  repo-root `.env`. Core code under `openbot/` — including the internal module name
  `server/src/openduet` — stays untouched so upstream fixes can still be merged.
- **Memory/threads backend:** CopilotKit Intelligence, kept as-is. Pay-as-you-go
  ($49/month per organization, first 10,000 interactions included) is far cheaper than
  replacing it (estimated 4–8 dev-weeks, plus permanent merge burden on `copilot.ts` /
  `index.ts`). Revisit only if the Intelligence bill becomes material at scale.
- **Model:** bring-your-own key. Default stays OpenAI `gpt-5.6-terra` in the tenant
  package's `model.yaml`; Anthropic or an OpenAI-compatible endpoint can replace it there.

## Language policy (UI)

Vietnamese is the UI language; technical terms stay in English.

- **Translate:** everyday UI wording — buttons, menus, guidance, notifications
  (e.g. Cài đặt, Kết nối, Cho phép một lần, Luôn cho phép, Từ chối, Cần bạn xử lý, Công việc).
- **Never translate:** Bot, Agent, Skill, Thread, Memory, Workflow, Prompt, Model, API Key,
  Token, MCP, AG-UI, CopilotKit Intelligence, product/provider names.
- **Bot behaviour (done, in the tenant package):** the AuLacBot Bot's system prompt tells it
  to converse in Vietnamese by default with technical terms kept in English, and its
  title / role description / channel copy are Vietnamese.

## UI localization — status and honest scope

The upstream app has **no i18n framework**: UI strings are hardcoded English across
`openbot/app/src` (~500 source files), and generated copy (morning brief, triage, reply
chips, first-day) is produced in `openbot/server/src/openduet/` (17 files). Localizing the
full UI is therefore a separate project, not a setting:

1. Phase 1 (done): brand + tenant-package surface + Bot speaks Vietnamese.
2. Phase 2: server-generated copy in `server/src/openduet/` → Vietnamese (kept as a small,
   clearly-marked patch set so upstream merges stay manageable).
3. Phase 3: introduce an i18n layer in the app and extract strings screen by screen,
   starting with Chat, Tasks, onboarding and Settings → Setup.

## Upstream sync

`upstream` = https://github.com/CopilotKit/OpenDuet.git. Product work lives on the
`aulacbot` branch; `main` tracks upstream only. To update: `git fetch upstream`, merge
`upstream/main` into a `sync/upstream-<date>` branch, test, then merge into `aulacbot`
and tag, e.g. `aulacbot-v0.1_on-f52288f`.

## Windows installer (.exe) — status

Upstream ships a Tauri desktop app (`openbot/desktop`) whose Windows bundle target is
NSIS: a small `.exe` installer. On first run it fetches the deployment, and it finds or
installs the container engine itself (on Windows: a per-user Podman MSI on WSL2, no
admin rights, no manual Docker step for the user).

- Done: installer identity rebranded — `tauri.conf.json` productName/title `AuLacBot`,
  identifier `app.aulacbot.desktop`, bundle descriptions, and the desktop shell's
  user-facing strings (commit `baf5825`). Rust internals (machine name, data dirs) are
  deliberately unchanged.
- Not done / blocked on a GitHub home for AuLacBot: the desktop app downloads its
  deployment from `CopilotKit/OpenBot` releases (hardcoded in
  `desktop/src-tauri/src/deployment_release.rs`) and runs that release's pinned
  container images. An AuLacBot installer therefore needs (1) a GitHub repo for
  AuLacBot, (2) release workflows that build/publish the AuLacBot container images
  and a release tarball (upstream's live in the OpenBot repo, not in this one),
  (3) repointing that one constant at AuLacBot's releases, and (4) a Windows build
  (GitHub Actions `windows-latest`, as upstream does, or a Windows PC with Rust+Bun).
  The .exe cannot be built on a Linux host.

## Still needed to run for real

- A CopilotKit Intelligence project + key (free hosted tier: 1,000 interactions/month).
- A model key (OpenAI by default).
- Google OAuth client if Gmail / Google Calendar connections are wanted.
  Enter these via the normal setup flow — never commit them to this repo.
