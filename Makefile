# Gate targets: format-check, lint, test, ci-local.
# Every build/test goes through the heavy queue; without `actqueue` it fails, naming it.
HEAVY := $(shell command -v actqueue >/dev/null 2>&1 && echo "actqueue run --quick --" || echo "")

# Every feature except `icons`, `persist` and `time`: lucide-dioxus and dioxus-sdk-* are built
# against Dioxus 0.7 and do not compile with Dioxus `main` (owner, 2026-11-16). Add them back
# (or use --all-features) once they follow.
FEATURES := biorcrux/design,biorcrux/widgets,biorcrux/charts,biorcrux/locale,biorcrux/locale-json,biorcrux/store-sqlite,biorcrux/sync,biorcrux/viewer3d-ui,biorcrux-store/indexeddb

.PHONY: need-actqueue format-check format lint test wasm-check wasm-test ci-local

need-actqueue:
	@command -v actqueue >/dev/null 2>&1 || { echo "error: 'actqueue' not found; builds must run through 'actqueue run'" >&2; exit 1; }

format-check: ## Check formatting without rewriting anything
	@cargo fmt --all --check

format: ## Format every package
	@cargo fmt --all

lint: need-actqueue ## clippy, warnings are errors
	@$(HEAVY) cargo clippy --workspace --all-targets --features $(FEATURES) -- -D warnings

test: need-actqueue ## Test every package
	@$(HEAVY) cargo test --workspace --features $(FEATURES)

wasm-check: need-actqueue ## Lint the browser adapter and compile its tests for wasm32
	@$(HEAVY) cargo clippy -p biorcrux-store --features indexeddb --target wasm32-unknown-unknown --all-targets -- -D warnings

wasm-test: need-actqueue ## Run the IndexedDB contract in headless Chrome (needs network for chromedriver)
	@wasm-pack test --headless --chrome packages/biorcrux-store --features indexeddb

ci-local: format-check lint wasm-check test ## The CI gates, on this machine
