---
description: Comprehensive guide for writing, running, and inspecting Forge kernel tests with tracing
---

# Testing & Debugging Workflow

// turbo-all

## Running Tests

**Always release mode.** Basic command:

```bash
cargo test --release -p <crate> -- --nocapture 2>&1 | tail -40
```

### Verbosity Control

Tracing uses the standard `tracing` crate via `RUST_LOG`. Default is `info` (set in `.cargo/config.toml`).

```bash
# Default (info): display_interesting() compact summary
cargo test --release -p forge-kernel --lib my_test -- --nocapture

# Debug: every decision, line by line
RUST_LOG=debug cargo test --release -p forge-kernel --lib my_test -- --nocapture

# Silent (CI-friendly)
RUST_LOG=off cargo test --release -p forge-kernel -- 2>&1 | tail -10
```

### Heavy Output (large tests)

Redirect to file, then grep:

```bash
RUST_LOG=debug cargo test --release -p forge-kernel --lib coplanar_grid -- --nocapture > /tmp/forge_debug.txt 2>&1

# Grep by entity
grep "entity=Face#68" /tmp/forge_debug.txt
grep "entity=HalfEdge#" /tmp/forge_debug.txt

# Grep by decision type
grep "escalated\|near-boundary" /tmp/forge_debug.txt
grep "Classification" /tmp/forge_debug.txt
grep "Degeneracy" /tmp/forge_debug.txt

# Grep by margin (most marginal decisions)
grep "margin=0.00e0" /tmp/forge_debug.txt
```

## Tracing Architecture

All tracing flows through `forge_core::tracing`.

### Core Data Flow

```
DecisionSink (trait)  →  DecisionLog  →  log_result() / log_decision_log()
     ↑                       ↑                    ↓
 call sites            stores events       tracing::info!  (display_interesting)
                                           tracing::debug! (full log)
                                           tracing::error! (always, on failure)
```

### Key Types

| Type                 | Purpose                                                                                                                                                                                         |
| :------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `TracedDecision`     | Atomic unit: id, kind, tier, margin, entity_scope, span_id, topology_delta                                                                                                                      |
| `DecisionLog`        | Queryable collection of `TraceEvent`s. Provides `display_interesting()`, `by_margin_ascending()`, `ambiguous_only()`, `tier_at_least()`                                                         |
| `DecisionSink`       | Trait with typed recording methods: `record_tolerance_snap`, `record_near_boundary`, `record_classification`, `record_escalation`, `record_policy_applied`, `record_ambiguous`, `record_forced` |
| `NullSink`           | No-op sink for tests where tracing is irrelevant                                                                                                                                                |
| `DecisionSinkHandle` | Thread-safe handle for production use                                                                                                                                                           |

### Decision Tiers

- **Deterministic** — exact result, no ambiguity
- **Resolved** — resolved via policy or heuristic
- **NearBoundary** — within threshold margin
- **PolicyApplied** — policy override applied
- **Escalated** — precision escalation required

### Display Format

Each decision prints as:

```
[decision-{id}] [{tier}] {kind} margin={margin:.2e} span-{span_id} entity={entity} | {context}
```

### `display_interesting()` (Inverted Noise Rule)

At `info` level, only interesting decisions (Tier 2+ / NearBoundary) are shown in detail. Boring spans collapse into one-liners. This keeps test output readable while surfacing risky decisions.

## topo-only Tests (no tracing)

For `forge-topo` unit tests (validators, operators), there is no `DecisionSink` — these are pure structural checks that return `Result<(), KernelError>`. Use `--nocapture` to see panic messages:

```bash
cargo test --release -p forge-topo -- --nocapture 2>&1 | tail -40
```

For debugging specific topology errors, print the error directly in the test:

```rust
let res = validate_foo(arena);
assert!(res.is_err(), "Expected error, got {:?}", res);
```
