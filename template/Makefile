# Same target names as biorcrux. Builds go through the heavy queue; without
# `actqueue` they fail, naming it.
HEAVY := $(shell command -v actqueue >/dev/null 2>&1 && echo "actqueue run --quick --" || echo "")

.PHONY: need-actqueue format-check format lint test ci-local web desktop

need-actqueue:
	@command -v actqueue >/dev/null 2>&1 || { echo "error: 'actqueue' not found; builds must run through 'actqueue run'" >&2; exit 1; }

format-check:
	@cargo fmt --all --check

format:
	@cargo fmt --all

lint: need-actqueue
	@$(HEAVY) cargo clippy --all-targets --all-features -- -D warnings

test: need-actqueue
	@$(HEAVY) cargo test --all-features

web: need-actqueue
	@$(HEAVY) dx serve --platform web

desktop: need-actqueue
	@$(HEAVY) dx serve --platform desktop --no-default-features --features desktop

ci-local: format-check lint test
