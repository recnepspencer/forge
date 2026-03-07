# Forge build targets
#
# Two-agent isolation: UI builds land in target-ui/, kernel builds in target/.
# Both can run simultaneously without Cargo lock contention.
#
# Usage:
#   make ui          — build forge-ui binary (isolated target dir)
#   make ui-test     — run forge-ui crate tests (isolated target dir)
#   make kernel      — build all kernel crates
#   make kernel-test — run all kernel tests
#   make test        — run everything
#   make trace-view  — open trace viewer GUI

UI_CRATES := forge-ui forge-ui-types forge-ui-theme forge-ui-components forge-ui-adapters forge-ui-state
UI_TARGET  := $(CURDIR)/target-ui

FORGE_LOG        ?= compact
FORGE_TRACE_DIR  ?=

# ── UI targets (isolated target dir) ─────────────────────────────────────────

.PHONY: ui
ui:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo build -p forge-ui $(ARGS)

.PHONY: ui-release
ui-release:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo build -p forge-ui --release $(ARGS)

.PHONY: ui-run
ui-run:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo run -p forge-ui $(ARGS)

.PHONY: ui-test
ui-test:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo test $(addprefix -p ,$(UI_CRATES)) $(ARGS)

.PHONY: ui-check
ui-check:
	CARGO_TARGET_DIR=$(UI_TARGET) cargo check $(addprefix -p ,$(UI_CRATES)) $(ARGS)

# ── Kernel targets (default target dir) ──────────────────────────────────────

KERNEL_EXCLUDES := $(addprefix --exclude ,$(UI_CRATES))

.PHONY: kernel
kernel:
	cargo build $(KERNEL_EXCLUDES) $(ARGS)

.PHONY: kernel-test
kernel-test:
	FORGE_LOG=$(FORGE_LOG) \
	FORGE_TRACE_DIR=$(FORGE_TRACE_DIR) \
	cargo test $(KERNEL_EXCLUDES) $(ARGS)

.PHONY: kernel-check
kernel-check:
	cargo check $(KERNEL_EXCLUDES) $(ARGS)

# ── Combined ─────────────────────────────────────────────────────────────────

.PHONY: test
test: kernel-test ui-test

.PHONY: check
check: kernel-check ui-check determinism-guards determinism-golden signal-runtime-guards

# ── Trace tooling ─────────────────────────────────────────────────────────────

.PHONY: trace-view
trace-view:
	cargo run -p forge-view --bin forge-trace-viewer $(DIR)

.PHONY: trace-issues
trace-issues:
	cargo run -p forge-view --bin forge-trace-cli -- issues $(DIR)

.PHONY: trace-list
trace-list:
	cargo run -p forge-view --bin forge-trace-cli -- list $(DIR)

.PHONY: determinism-guards
determinism-guards:
	python3 scripts/ci/check_determinism_guards.py

.PHONY: determinism-golden
determinism-golden:
	bash scripts/ci/check_determinism_golden.sh

.PHONY: signal-runtime-guards
signal-runtime-guards:
	bash scripts/ci/check_signal_runtime_guards.sh

# ── Helpers ───────────────────────────────────────────────────────────────────

.PHONY: clean-ui
clean-ui:
	rm -rf $(UI_TARGET)

.PHONY: clean
clean:
	cargo clean
	rm -rf $(UI_TARGET)
