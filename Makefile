# WORTH build targets
#
# Two-agent isolation: UI builds land in target-ui/, kernel builds in target/.
# Both can run simultaneously without Cargo lock contention.
#
# Usage:
#   make ui          â€” build worth-ui binary (isolated target dir)
#   make ui-test     â€” run worth-ui crate tests (isolated target dir)
#   make kernel      â€” build all kernel crates
#   make kernel-test â€” run all kernel tests
#   make test        â€” run everything
#   make trace-view  â€” open trace viewer GUI

UI_CRATES := worth-ui worth-ui-types worth-ui-theme worth-ui-components worth-ui-adapters worth-ui-state
UI_TARGET  := $(CURDIR)/target-ui

WORTH_LOG        ?= compact
WORTH_TRACE_DIR  ?=

# â”€â”€ UI targets (isolated target dir) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

.PHONY: ui
ui:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo build -p worth-ui $(ARGS)

.PHONY: ui-release
ui-release:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo build -p worth-ui --release $(ARGS)

.PHONY: ui-run
ui-run:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo run -p worth-ui $(ARGS)

.PHONY: ui-test
ui-test:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo test $(addprefix -p ,$(UI_CRATES)) $(ARGS)

.PHONY: ui-check
ui-check:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo check $(addprefix -p ,$(UI_CRATES)) $(ARGS)

# â”€â”€ Kernel targets (default target dir) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

KERNEL_EXCLUDES := $(addprefix --exclude ,$(UI_CRATES))

.PHONY: kernel
kernel:
	cargo build $(KERNEL_EXCLUDES) $(ARGS)

.PHONY: kernel-test
kernel-test:
	WORTH_LOG=$(WORTH_LOG) \
	WORTH_TRACE_DIR=$(WORTH_TRACE_DIR) \
	cargo test $(KERNEL_EXCLUDES) $(ARGS)

.PHONY: worth-fast
worth-fast: query-fast spatial-fast

.PHONY: query-fast
query-fast:
	cargo check -p worth-query --tests --message-format short
	cargo test -p worth-query --tests --no-run --message-format short
	cargo test -p worth-query --lib -- --format terse

.PHONY: spatial-fast
spatial-fast:
	cargo check -p worth-spatial --tests --message-format short
	cargo test -p worth-spatial --tests --no-run --message-format short
	cargo test -p worth-spatial --lib -- --format terse
	cargo test -p worth-spatial --test ui -- --format terse

.PHONY: query-closeout
query-closeout:
	cargo test -p worth-query --tests -- --format terse

.PHONY: spatial-public-api-closeout
spatial-public-api-closeout:
	cargo test -p worth-spatial --test public_api_contract -- --format terse

.PHONY: spatial-closeout
spatial-closeout:
	cargo test -p worth-spatial --tests -- --format terse

.PHONY: kernel-check
kernel-check:
	cargo check $(KERNEL_EXCLUDES) $(ARGS)

# â”€â”€ Combined â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

.PHONY: test
test: kernel-test ui-test

.PHONY: check
check: kernel-check ui-check determinism-guards determinism-golden signal-runtime-guards line-caps

# â”€â”€ Trace tooling â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

.PHONY: trace-view
trace-view:
	cargo run -p worth-view --bin worth-trace-viewer $(DIR)

.PHONY: trace-issues
trace-issues:
	cargo run -p worth-view --bin worth-trace-cli -- issues $(DIR)

.PHONY: trace-list
trace-list:
	cargo run -p worth-view --bin worth-trace-cli -- list $(DIR)

.PHONY: determinism-guards
determinism-guards:
	python3 scripts/ci/check_determinism_guards.py

.PHONY: determinism-golden
determinism-golden:
	bash scripts/ci/check_determinism_golden.sh

.PHONY: signal-runtime-guards
signal-runtime-guards:
	bash scripts/ci/check_signal_runtime_guards.sh

.PHONY: line-caps
line-caps:
	bash scripts/ci/check_workspace_rust_line_caps.sh

# â”€â”€ Helpers â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

.PHONY: clean-ui
clean-ui:
	rm -rf $(UI_TARGET)

.PHONY: clean
clean:
	cargo clean
	rm -rf $(UI_TARGET)
