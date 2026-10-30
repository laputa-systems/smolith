.PHONY: test lint publish publish-mycrates

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets

publish:
	./scripts/bump_minor_release.sh
	$(MAKE) publish-mycrates

# Publish every crate below to our private registry as well.
# `cargo publish --registry` resolves rewritten path dependencies against
# crates.io, so a crate is only listed here once its path-only deps exist
# there; everything in this workspace currently qualifies.
WORK_DIR ?= .work/registry-pub
PUBLISH_PORT ?= 8080
STATIC_PORT ?= 8081
REGISTRY ?= mycrates
REG_ENV ?= MYCRATES
PUBLIC_URL ?= https://crates.17166969.xyz
BUCKET ?= stuff-mycrates
CRATES ?= laputa-async-broadcast laputa-async-channel laputa-async-dup laputa-async-executor laputa-async-fs laputa-async-io laputa-async-lock laputa-async-net laputa-async-process laputa-async-signal laputa-async-task laputa-atomic-waker laputa-blocking laputa-concurrent-queue laputa-easy-parallel laputa-event-listener laputa-event-listener-strategy laputa-fastrand laputa-fastrand-contrib laputa-futures-lite laputa-parking laputa-piper laputa-polling laputa-smol laputa-smol-macros laputa-waker-fn
publish-mycrates:
	STUFF_DIR=../stuff \
	CRATES="$(CRATES)" \
	WORK_DIR="$(WORK_DIR)" PUBLISH_PORT="$(PUBLISH_PORT)" STATIC_PORT="$(STATIC_PORT)" \
	REGISTRY="$(REGISTRY)" REG_ENV="$(REG_ENV)" PUBLIC_URL="$(PUBLIC_URL)" BUCKET="$(BUCKET)" \
	../stuff/deploy/publish-mycrates.sh
