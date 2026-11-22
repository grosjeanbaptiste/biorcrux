# Same target names as biorcrux. Builds go through the heavy queue; without
# `dotfiles` they fail, naming it.
HEAVY := $(shell command -v actqueue >/dev/null 2>&1 && echo "actqueue run --quick --" || echo "")

.PHONY: need-dotfiles format-check format lint test ci-local web desktop

need-dotfiles:
	@command -v actqueue >/dev/null 2>&1 || { echo "error: 'actqueue' not found; builds must run through 'actqueue run'" >&2; exit 1; }

format-check:
	@cargo fmt --all --check

format:
	@cargo fmt --all

lint: need-dotfiles
	@$(HEAVY) cargo clippy --all-targets --all-features -- -D warnings

test: need-dotfiles
	@$(HEAVY) cargo test --all-features

web: need-dotfiles
	@$(HEAVY) dx serve --platform web

desktop: need-dotfiles
	@$(HEAVY) dx serve --platform desktop --no-default-features --features desktop

ci-local: format-check lint test
