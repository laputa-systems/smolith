.PHONY: test lint publish

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets

publish:
	./scripts/bump_minor_release.sh
