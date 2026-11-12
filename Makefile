.PHONY: test lint publish publish-plan publish-dry-run publish-mycrates

test:
	cargo test --workspace
	cargo test -p h12tiny --all-features
	cargo test -p laputa-hyper-futures-lite --features full --lib --tests

lint:
	cargo clippy --workspace --all-targets
	cargo clippy -p h12tiny --all-targets --all-features
	cargo clippy -p laputa-hyper-futures-lite --all-targets --features full

publish-plan:
	python3 tools/inventory.py plan

publish-dry-run:
	PUBLISH_DRY_RUN=1 ./scripts/bump_minor_release.sh

publish:
	./scripts/bump_minor_release.sh

REGISTRY ?= mycrates
STUFF ?= stuff
publish-mycrates:
	REGISTRY="$(REGISTRY)" STUFF="$(STUFF)" PUBLISH_PRIVATE_ONLY=1 ./scripts/bump_minor_release.sh
