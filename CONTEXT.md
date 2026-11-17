# biorcrux — public state as of 2026-11-16

The detailed working state names private projects and lives in `CONTEXT.private.md`
(ignored by git). This file holds only what is safe to publish.

## History rewrite (2026-11-01)

The history was rewritten to remove references to other projects from commit messages and
tags, and force-pushed (`main` and all 16 tags `v0.0.0`..`v0.9.0`). Commit hashes changed.
Anyone with a clone or a `Cargo.lock` pinning an old hash must re-clone or run
`cargo update -p biorcrux`. Verified on a fresh clone of the remote: no private name in any
reachable commit, message or tag.

## Current state

- Version 0.0.6 (tag `v0.0.6`). Workspace Dioxus dependencies follow the Dioxus `main` branch (owner, 2026-11-16; currently 0.8.0-alpha.3, rev in `Cargo.lock`); apps and the template use the same git source. `persist` and `time` (dioxus-sdk-* 0.7, which needs Dioxus ^0.7 even on its own `main`) pull a second Dioxus 0.7.10 and are unusable until the SDK follows; not tested. Licence: PolyForm
  Noncommercial 1.0.0 from v0.0.6 on (earlier tags: proprietary notice); see `LICENSE`
  and `CONTRIBUTING.md`.
- A library, not a framework: blocks are pieces the app calls; nothing imposes structure,
  state, route, language or look. User-facing text is always a prop (`Dialog.close_label`,
  `AppShell.more_label`).
- Product-specific values and defaults belong in separate data assets; reusable behavior
  consumes props or validated data instead of embedding product wording or settings.
  CSS and standalone HTML are data assets. Protocol, file-format, and algorithm constants
  stay with implementation and document their source. Audit in progress: `AppShell` now
  requires `nav_label`, removing its implicit English `Primary` label; source callers and
  template were updated. This is a breaking change, not yet versioned or released.
  Initial inventory found remaining embedded values in Rust for chart coordinates, the
  viewer library version, responsive breakpoints (duplicated in CSS), icon search paths and
  platform sizes, and the IndexedDB store name. Theme colors and widget styles already live
  in CSS assets; SQL schema is currently an inline string. These have not yet been migrated
  to package data files.
- `biorcrux` is the facade apps import. Features: `design`, `widgets`, `charts`, `locale`,
  `locale-json`, `viewer3d`, `viewer3d-ui` (default); opt-in `store`, `store-sqlite`, `sync`,
  `icons` (lucide-dioxus), `persist` (dioxus-sdk-storage), `time` (dioxus-sdk-time).
- `biorcrux-design`: tokens, themes, form factors, default app icon and iOS icon set.
- `biorcrux-widgets`: `AppShell` (bottom bar / rail / sidebar by CSS; `NavItem::secondary`
  moves items to a drawer on the phone bar), fields, table, dialogs, tabs, breadcrumb,
  empty state, avatar, status indicator and more. SSR tests; the drawer's open/close click
  is not tested.
- `biorcrux-charts`, `biorcrux-locale`, `biorcrux-store` (memory, SQLite, IndexedDB),
  `biorcrux-sync` (hybrid logical clock, last-write-wins; HTTP transport not built),
  `biorcrux-viewer3d`.
- `biorcrux-ios` (not in the facade, not released): `check(dioxus_toml, cargo_lock)` fails an
  app's own tests when `Dioxus.toml` lacks the single-scene `UIApplicationSceneManifest`
  (iOS 27 kills the app at launch otherwise) or `Cargo.lock` has tao < 0.37. The app
  template declares the manifest. Verified 2026-11-16: `cargo test --workspace`, clippy
  `-D warnings` and `cargo fmt --check` pass; not tested on a device.

## Rename (2026-11-17)

Renamed from `bioxus` to `biorcrux` (owner request): GitHub repository, folder, every package
(`biorcrux`, `biorcrux-*`, folders under `packages/`), Rust paths (`biorcrux::`,
`biorcrux_widgets::`), template, scripts and docs. Done by `actautoma rename-family`, the new
family pass of `actautoma rename-repo` (branch `feat/rename-family` of actautoma, not yet merged).
Verified: `cargo fmt --check`, clippy `-D warnings` and `cargo test --workspace` pass; no
`bioxus` left in any tracked file. Not released: apps pinning an old tag keep the name
`bioxus` until they move to a tag of the new name; the first tag under the new name is not cut.

## Decisions

- Biorcrux is intended to provide a complete multiplatform application library: it owns the coherent facade and integration for shared foundations such as databases, UI components, locales, sync, and design. Maximum modularity is the primary architecture constraint: isolate capabilities into small packages, make facade features opt-in, and keep platform/backend implementations behind ports so apps can compose only what they need. Reuse maintained compatible crates where useful; impose no framework or backend.
- Every possibility stays open: each block is a feature and a separate package; behaviour
  sits behind ports.
- Reuse maintained Dioxus crates before writing a block; only what none covers is written here.
- The repository is public so people can read and contribute; commercial use needs written
  permission.
- No reference to private projects in tracked files or commit messages.

## Gates

`make ci-local` (format, clippy `-D warnings`, wasm check, tests).
