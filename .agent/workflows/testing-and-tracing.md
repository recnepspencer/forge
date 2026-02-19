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

### Standard Run (No Traces)
Just runs the tests. Trace overhead is near-zero when disabled.

```bash
cargo test -p forge-kernel
```

### With Trace Persistence (For Debugging)
Set `FORGE_TRACE_DIR` to an absolute path. The kernel will write `trace_*.json` files there.

// turbo
```bash
mkdir -p /Users/spenstar/Documents/programming/Forge/traces
FORGE_TRACE_DIR=/Users/spenstar/Documents/programming/Forge/traces cargo test -p forge-kernel my_new_feature_test
```

### With Console Logging
See high-level decisions in stderr without opening a viewer.

```bash
FORGE_LOG=compact cargo test -p forge-kernel my_new_feature_test
```

## 3. Drill Down into Traces (CLI)

After running tests with `FORGE_TRACE_DIR`, use the CLI to inspect the potentially huge JSON files.

### Step 1: Check for Issues
Identify traces with non-deterministic decisions or errors.

// turbo
```bash
cargo run -p forge-view --bin forge-trace-cli -- issues /Users/spenstar/Documents/programming/Forge/traces
```

### Step 2: List Summaries
See all generated traces.

// turbo
```bash
cargo run -p forge-view --bin forge-trace-cli -- list /Users/spenstar/Documents/programming/Forge/traces
```

### Step 3: Show Trace Structure
See the span hierarchy (Split -> Classify -> Select -> Assemble).

```bash
cargo run -p forge-view --bin forge-trace-cli -- show <TRACE_ID> /Users/spenstar/Documents/programming/Forge/traces
```

### Step 4: Inspect Decisions
See the actual geometric decisions (e.g., "Split face #12 by plane #5").

```bash
cargo run -p forge-view --bin forge-trace-cli -- decisions <TRACE_ID> <SPAN_ID> /Users/spenstar/Documents/programming/Forge/traces
```

## 4. Visual Inspection (GUI)

Launch the native viewer for a full interactive experience.

```bash
cargo run -p forge-view --bin forge-trace-viewer /Users/spenstar/Documents/programming/Forge/traces
```
