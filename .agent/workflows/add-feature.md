---
description: Formalized workflow for adding a new feature or milestone to the Forge geometry kernel
---

---
description: Advanced workflow for adding a feature to the Forge kernel (v2)
---

# Add Feature Workflow (v2)

Use this workflow to implement any milestone from `DEVELOPMENT_BLUEPRINT.MD`.

## Step 1: Component Analysis

1.  **Intent Check**: Define the serializable JSON schema in `intent.rs`. This must be token-efficient (< 200 tokens).
2.  **SDF Check**: Can this feature be represented as a distance function for 60fps previews?
3.  **Solver Check**: Identify which `forge-geom` solvers are needed. If they don't exist, create them using `PolicyResult`.
4.  **Attribute Check**: What semantic metadata (material, color, tolerance) does this feature need to tag on its output faces?

## Step 2: Create the Bento Box (Expanded)

Create `crates/forge-kernel/src/features/<feature_name>/`:
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
    - *Example*: Tag the inner cylinder of a hole as `"type": "bore"`.

## Step 5: Verification Checklist

- [ ] **D1 (Determinism)**: Run test 100x. Is the topology hash bit-identical?
- [ ] **D2 (Policy)**: Is there a hardcoded `1e-8` in the code? If yes, move it to `ToleranceConfig`.
- [ ] **D3 (Firewall)**: Does `forge-geom` know about `FaceId`? If yes, refactor to use a value-based trait.
- [ ] **Performance**: Does a clean rebuild take < 50ms?