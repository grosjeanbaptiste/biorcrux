# biorcrux

Project-level instructions for Claude Code. Loaded on top of the user-wide
`~/.claude/CLAUDE.md` in any session opened inside this repo.

Keep this file focused on what is *specific* to this project. Cross-machine
rules belong in the user-wide file; conventions that only apply here belong
here.

## Start of every session

Read `CONTEXT.md` first: it holds the verified current state, deadlines and
pending items, so the repository does not have to be re-read to rebuild
them. Update it (and its date) after any act that changes that state.

## What this is

Biorcrux is a modular library for building complete multiplatform Dioxus
applications. It owns a coherent API and integration for the shared foundations
apps need, including UI components, localization, persistence and databases,
sync, design, and charts. Apps consume it by git tag.

## Model

One workspace, role-named packages
under `packages/`, path dependencies between them, root-level gates run by
`make` (same target names). Apps stay in their own repos and consume packages
by git tag.

| Package | Role |
|---|---|
| `biorcrux` | Facade apps import: `biorcrux::{design,widgets,charts,locale,store,sync}` |
| `biorcrux-charts` | Bar charts, pure geometry → `biorcrux::charts` |
| `biorcrux-design` | Design system: tokens, constants, shared CSS |
| `biorcrux-widgets` | Responsive `AppShell` (bottom bar / rail / sidebar by CSS), `Badge` → `biorcrux::widgets` |
| `biorcrux-store` | Optional DB behind a `Store` port → `biorcrux::store` |
| `biorcrux-sync` | HLC + last-write-wins sync over a `Transport` port → `biorcrux::sync` |
| `biorcrux-locale` | i18n, open locale tags → `biorcrux::locale` |

## Conventions

- Keep every possibility open (owner, 2026-10-28): each block is a feature and a separate package; no router, renderer, DB or backend is imposed; behaviour sits behind ports; layout is CSS-driven with an explicit override prop.
- Cover the shared foundations of a complete multiplatform app (owner, 2026-11-11): Biorcrux provides and integrates capabilities such as databases, UI components, and locales behind a coherent facade. Keep capabilities modular and selectable; apps can compose them without being forced into a router, renderer, database, backend, or application structure. Reuse maintained compatible crates where they provide the needed capability.
- Modularity is the primary architecture constraint: isolate independently useful capabilities in small packages, expose them through opt-in facade features, keep platform and backend implementations behind ports/adapters, and avoid making one capability pull unrelated capabilities into an app.
- A library, not a framework (owner, 2026-10-31): biorcrux must allow building any app. Blocks are pieces the app calls; no imposed structure, state, route, language or look; no user-facing text baked in (labels are props).
- No product values baked into behavior: app wording and choices come from props or app-owned catalogs; reusable presets and tunable defaults live in package data assets, separately from Rust behavior. CSS and standalone HTML are data assets. Keep only values required by a protocol, file format, or algorithm in implementation code; document their source.
- Every app always has an app icon (owner, 2026-10-28): the repo's own, else the BIORCRUX icon (`biorcrux::design::icon`). Regenerate the default icons (512 PNG + 13-file iOS set, RGB no alpha) with `scripts/render-icon.sh`; build a set from any master with `scripts/make-ios-set.sh`.
- Follow the Dioxus `main` branch until the owner says otherwise (owner, 2026-11-16); declare Dioxus dependencies in the workspace so every crate resolves the same source. Features whose crates need a released Dioxus (`icons`, `persist`, `time`) do not compile meanwhile; `make` gates skip them.
- Crates stay small; no px units; each public widget has a test.

## Commands worth knowing

| Want | Run |
|---|---|
| All gates | `make ci-local` |
| Test | `actqueue run --quick -- cargo test --workspace` |
| Lint | `actqueue run --quick -- cargo clippy --workspace --all-targets -- -D warnings` |

## When in doubt

Open the README, or grep the repo. The source is the documentation.
