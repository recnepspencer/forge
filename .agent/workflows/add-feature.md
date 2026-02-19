---
description: Formalized workflow for adding a new feature or milestone to the Forge geometry kernel
---

# Add Feature Workflow (v2)

Use this workflow to implement any milestone from `DEVELOPMENT_BLUEPRINT.MD`.

> **Before writing any code**, read `CRATE_MAP.md` to know which crate owns
> which abstractions. Most violations come from putting code in the wrong crate.

## Step 1: Component Analysis

1.  **Intent Check**: Define the serializable JSON schema in `intent.rs`. This must be token-efficient (< 200 tokens).
2.  **SDF Check**: Can this feature be represented as a distance function for 60fps previews?
3.  **Solver Check**: Identify which `forge-geom` solvers are needed. If they don't exist, create them using `PolicyResult`.
4.  **Attribute Check**: What semantic metadata (material, color, tolerance) does this feature need to tag on its output faces?

## Step 2: Create the Bento Box (Expanded)

Create `crates/forge-kernel/src/operations/<feature_name>/` (for modeling operations) or `crates/forge-kernel/src/features/<feature_name>/` (for feature tree entries):

- `intent.rs` — **Primary Source of Truth.** Serializable parameters and SDF logic.
- `schema.rs` — Internal Rust types and helper enums.
- `eval.rs` — The Bridge. Implements `GeometrySource` and handles `PolicyResult` escalation.
- `topo.rs` — Euler Operator structs.
- `tests.rs` — Linear verification tests.

## Step 3: Implement the Geometry Bridge (eval.rs)

Every feature evaluation must follow the **Escalation Pattern**:

1.  **Construct Provider**: Create a local adapter that wraps the `TopologyArena` to satisfy `forge-geom`'s `GeometrySource` trait.
2.  **Call Solver**: Pass the provider and `ToleranceConfig` to the `forge-geom` solver.
3.  **Handle Ambiguity**:
    ```rust
    match solver_result {
        PolicyResult::Success(val) => apply_op(&mut draft, Op { val })?,
        PolicyResult::Ambiguous(query) => {
            // Escalation to ModelingContext
            let decision = draft.ctx.resolve(query)?;
            apply_op(&mut draft, Op { val: decision.value })?;
        },
        PolicyResult::HardError(e) => return Err(e),
    }
    ```

## Step 4: Lineage & Attributes

1.  **Lineage**: Ensure the Euler operator hashes the parent lineage with the new `intent` parameters.
2.  **Attributes**: Register the feature's output entities in the `AttributeStore`.
    - _Example_: Tag the inner cylinder of a hole as `"type": "bore"`.

## Step 5: Architecture Compliance Checklist

Before considering a feature complete, verify **every** item:

### Layering & Dependencies

- [ ] No upward dependencies (e.g., forge-geom does NOT import forge-topo types)
- [ ] All shared types (`KernelError`, `PolicyResult`) come from `forge-core`
- [ ] `GeometrySource` trait comes from `forge-math::data_access`
- [ ] `forge-math` only uses `MathError`, never `KernelError`

### Geometry Firewall (D3)

- [ ] **Zero** raw f64 comparisons in `forge-topo` (no `dist < EPS`, no `denom.abs() < 1e-30`)
- [ ] All floating-point geometry lives in `forge-geom` functions
- [ ] Topology decisions driven by `CertifiedTriSign` or imported geometry results

### Tolerance & Policy (D2)

- [ ] **Zero** hardcoded `const EPS` or magic numbers in `forge-geom` or `forge-topo`
- [ ] All thresholds flow from `ToleranceConfig` (owned by `forge-kernel`)
- [ ] Ambiguous results return `PolicyResult::Ambiguous`, never silently rounded

### Safety (D6, Rule 5.1)

- [ ] **Zero** `unwrap()` / `expect()` / `panic!()` outside `#[cfg(test)]`
- [ ] All mutations go through `MutableDraft` — no direct arena mutation
- [ ] All fallible functions return `Result<T, KernelError>` (or `MathError` in forge-math)

### Conventions (CONVENTIONS.md)

- [ ] All struct fields are private with `get_*` / `set_*` accessors
- [ ] No raw `u32`/`usize` for IDs — use `FaceId`, `VertexId`, etc.
- [ ] No inline `//` comments — use `///` doc comments or better variable names
- [ ] Functions named as verbs, structs named as nouns
- [ ] `mod.rs` is purely a table of contents (zero business logic)
- [ ] No file exceeds ~400 lines

### Verification

- [ ] **D1 (Determinism)**: Run test 100x. Is the topology hash bit-identical?
- [ ] **D2 (Policy)**: Is there a hardcoded `1e-8` in the code? If yes, move it to `ToleranceConfig`.
- [ ] **D3 (Firewall)**: Does `forge-geom` know about `FaceId`? If yes, refactor to use a value-based trait.
- [ ] `cargo build --workspace` clean (no errors)
- [ ] `cargo test --workspace` passes (no regressions)
- [ ] `cargo clippy --workspace -- -D warnings` clean

### Trace Verification

- [ ] Run `/testing-and-tracing` workflow to inspect kernel decisions
- [ ] `forge-trace-cli issues` reports zero interesting decisions (all "deterministic")
- [ ] If non-deterministic decisions exist, drill down with `forge-trace-cli show` and `decisions` to verify they are expected
