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
REGISTRY ?= mycrates
CRATES ?= laputa-async-broadcast laputa-async-channel laputa-async-dup laputa-async-executor laputa-async-fs laputa-async-io laputa-async-lock laputa-async-net laputa-async-process laputa-async-signal laputa-async-task laputa-atomic-waker laputa-blocking laputa-concurrent-queue laputa-easy-parallel laputa-event-listener laputa-event-listener-strategy laputa-fastrand laputa-fastrand-contrib laputa-futures-lite laputa-parking laputa-piper laputa-polling laputa-smol laputa-smol-macros laputa-waker-fn
STUFF ?= stuff
publish-mycrates:
	$(STUFF) publish cargo $(addprefix -p ,$(CRATES)) --registry $(REGISTRY)
