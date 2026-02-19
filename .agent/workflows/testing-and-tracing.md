---
description: Comprehensive guide for writing, running, and inspecting Forge kernel tests with tracing
---

# Testing and Tracing Workflow

This workflow covers the end-to-end process of validating kernel logic: from writing a new test case to running it with trace persistence and drilling down into the decision logs.

## 1. How to Write a New Test

To ensure your test is observable, use the `execute_boolean_logged` helper or manual `ModelingContext` management.

### Pattern A: Standard Boolean Test (Recommended)

Use `forge_kernel::operations::boolean::test_helpers::execute_boolean_logged` for standard boolean operations. This automatically handles `ModelingContext` creation, execution, and trace persistence.

```rust
// forge-kernel/src/operations/boolean/tests/tests.rs (or similar)

#[test]
fn my_new_feature_test() {
    // 1. Setup geometry
    let (target, target_geom) = make_cube(10.0);
    let (tool, tool_geom) = make_cube(10.0);
    // ... apply transformations ...

    // 2. Execute with logging
    // This wrapper ensures ModelingContext is active and traces are saved if FORGE_TRACE_DIR is set.
    let result = execute_boolean_logged(
        &target, &target_geom,
        &tool, &tool_geom,
        BooleanOp::Union
    ).expect("Boolean operation failed");

    // 3. Verify topology
    assert_eq!(result.topology().arena().face_count(), 6);
}
```

### Pattern B: Low-Level / Custom Test

If you are testing internal logic (e.g., `split_face_by_plane` directly), you must manage the context yourself.

```rust
#[test]
fn test_internal_split_logic() {
    let mut ctx = ModelingContext::default();
    ctx.enable_auto_persist(); // CRITICAL: Enables trace saving on drop

    let result = ctx.scope("my_custom_scope", |ctx| {
        // Pass ctx down to your internal function
        internal_function(&mut data, ctx)
    });

    // Trace is saved when `ctx` is dropped at end of scope
}
```

## 2. How to Run Tests

### Standard Run (Traces Auto-Persisted)
In debug builds, traces auto-persist to `{workspace}/traces/trace.json`. All tests in a single `cargo test` invocation accumulate into one file. Re-running overwrites the previous batch.

// turbo
```bash
cargo test -p forge-kernel
```

### Run Specific Test

// turbo
```bash
cargo test -p forge-kernel --lib <FULL_TEST_PATH> -- --exact --nocapture 2>&1 | tail -10
```

### With Console Logging
See high-level decisions in stderr without opening a viewer.

```bash
FORGE_LOG=compact cargo test -p forge-kernel -- --nocapture
```

## 3. Drill Down into Traces (CLI)

All traces live in a single `traces/trace.json`. Each test gets its own entry, identified by its test name (with `::` replaced by `_`).

### Step 1: List all test traces

// turbo
```bash
cargo run -p forge-view --bin forge-trace-cli -- list /Users/spenstar/Documents/programming/Forge/traces
```

Output shows each test's decision count, span count, status (✅/❌), and hash.

### Step 2: Show span tree for one trace

// turbo
```bash
cargo run -p forge-view --bin forge-trace-cli -- show <TRACE_ID> /Users/spenstar/Documents/programming/Forge/traces
```

### Step 3: Drill into decisions for a specific span

// turbo
```bash
cargo run -p forge-view --bin forge-trace-cli -- decisions <TRACE_ID> <SPAN_ID> /Users/spenstar/Documents/programming/Forge/traces
```

### Step 4: Check for issues (non-deterministic or error traces)

// turbo
```bash
cargo run -p forge-view --bin forge-trace-cli -- issues /Users/spenstar/Documents/programming/Forge/traces
```

## 4. Visual Inspection (GUI)

Launch the native viewer for a full interactive experience.

```bash
cargo run -p forge-view --bin forge-trace-viewer /Users/spenstar/Documents/programming/Forge/traces
```

## 5. Debugging Discipline — First Principles

When a test fails, **DO NOT** read the entire pipeline and reason about it for hours. Follow Elon's framework:

### Step 1: Question Requirements
Before debugging, ask: "Is this test actually correct? Is the expected behavior clearly defined?" Remove any dumb requirements or assumptions.

### Step 2: Write a Minimal Reproduction
**The single most powerful debugging move.** Strip the failing scenario down to the smallest possible test that reproduces the error. If a 10-step chain fails at step 1, write a single-step test with the exact geometry that fails.

```rust
#[test]
fn minimal_stitch_failure() {
    // ISOLATE: exact geometry from step 1 of the chain
    let (target, target_geom) = build_specific_geometry();
    let (tool, tool_geom) = build_cube([x, y, z], half);
    let result = execute_boolean_logged(BooleanInput::new(
        target, target_geom, tool, tool_geom, BooleanOp::Subtraction,
    ));
    // Assert the specific failure mode
}
```

### Step 3: Hypothesize BEFORE Coding
Write a bulleted list of hypotheses. Test each one with a targeted test, not by reading more code.

**Good:** "Hypothesis: vertices at position (-3.5, 0.5, 5.0) are not unified. Test: add a diagnostic print in resolve_vertex for this position."

**Bad:** "Let me read all 600 lines of split.rs, copy.rs, stitch.rs, classify.rs, and merge.rs to understand the full data flow."

### Step 4: One Variable at a Time
Change ONE thing. Run the test. Observe. Repeat. Never change split.rs AND copy.rs AND stitch.rs in the same commit.

### Step 5: Automate and Iterate
If you wrote a hypothesis, turn it into a test assertion. Keep building a suite of minimal tests that pin down the behavior.

### Anti-Patterns (NEVER DO)
- ❌ Reading 5+ files at once to "understand the full picture"
- ❌ Reasoning about what "should" happen for 20+ tool calls without running code
- ❌ Changing multiple files simultaneously to fix a single bug
- ❌ Writing a fix without a failing test that proves the fix works
- ❌ Assuming downstream code is broken when the bug is upstream

### Context Diet for AI Agents
When debugging boolean pipeline failures:
1. **Assume stitch.rs and copy.rs are correct** unless you have a failing test proving otherwise
2. **Focus on ONE phase** at a time (split → classify → select → assemble)
3. **Never read more than 2 files** in the same debugging session
4. **Write a test, not an analysis** — tests give ground truth, analysis gives opinions
5. **Use traces** (FORGE_TRACE_DIR) instead of reading code to understand what happened

### Minimal Test Case Patterns

**Pattern: Isolate a chain step failure**
```rust
#[test]
fn isolate_chain_step_n() {
    // 1. Run steps 0..N-1 to get the intermediate result
    let (topo, geom) = run_chain_up_to_step(n - 1);
    // 2. Build the exact tool for step N
    let (tool, tool_geom) = build_tool_for_step(n);
    // 3. Run just step N
    let result = execute_boolean_logged(BooleanInput::new(
        topo, geom, tool, tool_geom, op,
    ));
    assert!(result.is_ok(), "Step {n} failed: {:?}", result.err());
}
```

**Pattern: Verify vertex unification**
```rust
#[test]
fn vertices_unified_at_boundary() {
    // After a boolean, check that no two vertices share the same position
    let result = execute_boolean_logged(input).unwrap().into_value();
    let arena = result.topology().arena();
    let geom = result.geometry();
    for (v1, _) in arena.iter_vertices() {
        for (v2, _) in arena.iter_vertices() {
            if v1 == v2 { continue; }
            let p1 = geom.get_vertex_position(v1).unwrap();
            let p2 = geom.get_vertex_position(v2).unwrap();
            let d = dist_sq(p1, p2);
            assert!(d > 1e-12, "Duplicate vertices {v1} and {v2} at {:?}", p1);
        }
    }
}
```
