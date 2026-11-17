# vendor

Upstream crates carried here until upstream ships the fix. Excluded from the workspace.

## dioxus-desktop

`dioxus-desktop` 0.7.9 from crates.io, one change: `[dependencies.tao]` goes from `0.34.0`
to `0.37.1`. tao 0.37 adopts the UIKit scene lifecycle, which iOS 27 requires (an app without
it is killed at launch: SIGTRAP in `EvaluateRuntimeIssueForNoSceneLifecycleAdoption`).
Built against `wry 0.53.5`, the combination reported on DioxusLabs/dioxus#5853.

An app uses it with `[patch.crates-io] dioxus-desktop = { path = "…/biorcrux/vendor/dioxus-desktop" }`
and the scene manifest of `template/Dioxus.toml.tmpl`; `biorcrux-ios::check` verifies both.

Remove it once a released `dioxus-desktop` depends on tao >= 0.37 on iOS (DioxusLabs/dioxus#5893).
