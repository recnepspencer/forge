---
description: First-principles debugging workflow for kernel failures — minimize, hypothesize, test, iterate
---

⚠️ Mandatory Execution-First Rule
Why This Rule Exists

This rule exists because of a well-documented failure mode in LLM agents: analysis spirals.

Agent observability research (TrueFoundry, Datadog, Galileo) consistently finds that agents enter recursive reasoning loops that look productive — each step returns 200 OK — but the agent is stuck, burning context without converging.

The Reflexion framework (Shinn et al., NeurIPS 2023) demonstrated that agents improve through environmental feedback, not internal reasoning. Agents that reflect on test output outperform those reflecting on their own analysis by 20%+ across coding tasks.

The TDFlow system (2025) goes further:

Debugging is decomposed into sub-agents

Each sub-agent gets:

One failing test

One debugger tool

No architecture reading

Loop: run → observe → fix

No architectural speculation.
No multi-file reasoning.
No narrative.

Execution dominates cognition.

Kernel-Specific Reality

For geometry kernels specifically:

You cannot simulate:

AABB overlaps

Plane–face intersection signatures

Halfedge adjacency

Coplanar selection effects

Topological stitching

The kernel exists to compute these.

Your job is not to re-derive topology mentally.
Your job is to ask the kernel what happened.

If you are imagining topology without trace output, you are guessing.

Binding Rule

Before any reasoning:

Run the failing test.

Capture the trace.

Extract concrete facts from output.

Form ≤ 3 hypotheses.

Test one.

If you are not running code, you are violating protocol.

If you are reading more than two files without running code, you are drifting.

If you describe geometry verbally without trace evidence, you are speculating.

The Loop

The only valid loop:

Run → Observe → Hypothesize → Instrument → Run → Measure

# Debugging Workflow

When a kernel test fails, follow this protocol. **Do NOT read the whole pipeline at once.** Think like Musk: start from first principles, remove dumb assumptions, minimize, automate, iterate.

Overthinking Trigger

If you:

Explain adjacency relationships verbally

Predict stitching outcomes mentally

Discuss architectural causes without trace data

Stop immediately.

Reduce the test.
Instrument.
Run again.

## Phase 0: Stop and Think (Before ANY Code Reading)

1. **Read the error message and test output.** That's it. Don't open source files yet.
2. **Write down in plain English:** What failed? What was the expected behavior? What actually happened?
3. **Identify the phase:** Is the failure in split, classify, select, copy, or stitch? The error message tells you.

## Phase 1: Minimize the Reproduction

**This is the most important step. A minimal test case is worth 1000 lines of code reading.**

### Rule: If a chain of N steps fails at step K, isolate step K.

```rust
#[test]
fn minimal_repro() {
    // Build exactly the geometry from step K-1's result + step K's tool
    let (target, target_geom) = /* result of steps 0..K-1 */;
    let (tool, tool_geom) = build_cube([x, y, z], half);
    let result = execute_boolean_logged(BooleanInput::new(
        target, target_geom, tool, tool_geom, BooleanOp::Subtraction,
    ));
    assert!(result.is_ok());
}
```

### Rule: If the minimal repro still needs prior steps, simplify the geometry.

Replace the chain with a hand-built topology that has the same structural characteristics. For example, if step 1 fails because the target has a notch from step 0, build a cube-with-notch directly.

### Rule: If you can't simplify further, add assertion checkpoints.

```rust
// After split: how many faces on each solid?
// After classify: which faces classified as what?
// After select: which faces were kept/dropped?  
// After copy: how many vertices in the result? Any duplicates?
// After stitch: how many unpaired halfedges?
```

## Phase 2: Hypothesize Before Coding

**Write a numbered list of hypotheses. Do NOT write code to fix anything yet.**

### Format:
```
H1: Vertices at position X are not unified because [reason].
    TEST: Print resolve_vertex calls for that position.
    
H2: Face Y is classified incorrectly because its centroid is on a boundary.
    TEST: Print the centroid and classification of face Y.

H3: The reverse halfedge for edge A→B was never copied because face Z was dropped.
    TEST: Print all selected faces and check if the face containing B→A exists.
```

### Rules:
- Each hypothesis must name a **specific entity** (face ID, vertex ID, position)
- Each hypothesis must have a **concrete test** (not "read more code")
- Maximum 3 hypotheses at a time — test them, then form new ones
- **Never hypothesize about code you haven't run** — run it first

## Phase 3: Test One Variable at a Time

### The Protocol:
1. Pick the most likely hypothesis
2. Add ONE diagnostic print/assertion to verify it
3. Run the minimal test
4. Read the output — was the hypothesis correct?
5. If yes → you found the bug, write the fix
6. If no → remove the diagnostic, pick the next hypothesis

### Rules:
- **Change ONE file per iteration.** Never edit split.rs AND copy.rs simultaneously
- **Use traces** (`FORGE_TRACE_DIR`) instead of adding prints when possible

// turbo
```bash
mkdir -p /Users/spenstar/Documents/programming/Forge/traces
FORGE_TRACE_DIR=/Users/spenstar/Documents/programming/Forge/traces \
  cargo test -p forge-kernel -- my_minimal_repro 2>&1
```

// turbo  
```bash
cargo run -p forge-view --bin forge-trace-cli -- issues /Users/spenstar/Documents/programming/Forge/traces
```

- **Commit working state before each change.** If your fix breaks something else, you can revert instantly

## Phase 4: Fix and Verify

1. Write the fix in ONE file
2. Run the minimal repro — it should pass
3. Run the full test suite — no regressions

// turbo
```bash
cargo test -p forge-kernel 2>&1 | tail -5
```

4. If regressions appear, **revert the fix** and go back to Phase 2

## Anti-Patterns (NEVER DO)

| ❌ Don't | ✅ Do Instead |
|----------|---------------|
| Read 5+ source files to "understand the pipeline" | Read the error message + 1 file max |
| Reason about topology for 20+ tool calls | Write a test and run it |
| Change multiple files in one iteration | Change ONE file, run, observe |
| Write a fix without a failing test | Write the minimal repro FIRST |
| Assume downstream is broken | Assume stitch/copy are correct; prove otherwise |
| Chase "what should happen" through code | Run code and observe what DOES happen |
| Spend >5 minutes reading without running | If stuck reading, STOP and write a test |
| Re-read code you've already seen | Use traces and test output as ground truth |
| Write exact float comparisons (a == b) | Use ToleranceConfig or VERTEX_WELD_TOLERANCE_SQ |
| Assume 4-plane intersections yield identical 3-plane signatures | Assume numeric drift; fall back to spatial proximity |

## Context Diet for AI Agents

AI agents degrade when holding too many mental models. Follow these strict rules:

1. **Never read more than 2 pipeline files** in one debugging session
2. **Assume downstream code is correct** unless a test proves otherwise
3. **Focus on ONE phase** (split → classify → select → copy → stitch)
4. **Prefer running code over reading code** — tests give ground truth, analysis gives opinions
5. **If you've been analyzing for >10 tool calls without running anything, STOP** — write a test
6. **When forming hypotheses, always include the specific test you'll run** — no vague "let me check X"

## Quick Reference: Common Failure Patterns

| Error | Phase | Likely Cause |
|-------|-------|-------------|
| `MissingTwin` | stitch | Kept face borders a dropped face (selection mismatch) OR duplicate vertices (copy didn't unify) |
| `Euler χ ≠ 2` | post-assemble | Topology inconsistency from bad split or bad copy |
| `cut_points < 2` | split | Floating-point: vertices classified ON the plane aren't detected |
| `InvalidInput` | any | Geometry missing from GeometryStore (plane or position) |
| Chain step N fails | split/classify | Prior result's topology has internal faces that interact with new tool unexpectedly |