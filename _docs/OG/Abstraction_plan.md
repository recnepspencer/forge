# Three-Tier Feature Pipeline Abstraction Spec

Status: Draft for review

Last updated: 2026-02-25

## Purpose

This spec defines a three-tier declarative pipeline infrastructure for the Forge kernel. It replaces ad-hoc per-feature wiring of policy checks, invariant validation, audit emission, and lineage recording with **compiler-enforced contracts** and **auto-injecting execution contexts**.

Every current and future kernel feature (boolean, fillet, chamfer, shell, extrude, loft, sweep, NURBS trim, pattern) will be built as a consumer of this infrastructure.

## Existing Infrastructure (What We Compose With)

This pipeline does NOT reinvent existing components. It composes with:

| Existing Component            | Location                  | What It Does                                                                                                                 | Pipeline Composes How                                                              |
| ----------------------------- | ------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `OperationFinalizer`          | `core/finalization.rs`    | Single-use boundary: drains decisions + metadata from `ModelingContext`, collects into `CollectedFinalization`, emits traces | Pipeline calls `OperationFinalizer::collect_success` / `collect_error` at boundary |
| `SubOperationMetadata`        | `core/context.rs`         | Accumulated warnings, metrics, lineage delta, error budget from sub-ops                                                      | Pipeline uses `absorb_sub_result` and `take_sub_metadata` for rollup               |
| `PolicyRegistrySnapshot`      | `core/context.rs`         | 4-layer policy resolver (default → session → model/feature → operation scopes)                                               | Pipeline calls existing `resolve_policy_query` for ambiguous results               |
| `KernelDraft`                 | `core/kernel_draft.rs`    | Transactional topo + geom patch + commit                                                                                     | Feature-level pipeline creates/commits `KernelDraft`                               |
| `OperationSpace`              | `core/operation_space.rs` | Coordinate transform lifecycle (analyze, transform, restore)                                                                 | Feature-level pipeline manages `OperationSpace` around execution                   |
| `DecisionLog` checkpoint/diff | `forge-core tracing/`     | `CheckpointLog`, `DecisionDelta`, decision diffing                                                                           | Step-level audit uses existing checkpoint mechanics                                |

**New infrastructure focuses exclusively on what doesn't exist yet:** declarative contracts, typed inputs, step sequencing, and auto-injection.

### How `OperationResult<T>` Integrates (Not Sidelines)

`OperationResult<T>` is the universal envelope — it already carries decision logs, warnings, metrics, lineage deltas, topology hashes, and error budget. The pipeline uses it as the **canonical metadata transport** at the feature boundary, not as a thing to work around.

**Today's problem:** `FeatureOutput` manually carries `Arc<DecisionLog>`, `Arc<ReplayLog>`, `Arc<Vec<LineageEvent>>` as bespoke fields. Meanwhile `OperationResult` carries the same metadata (plus metrics, warnings, hashes, error budget) in a unified envelope. This creates two parallel metadata paths that must be kept in sync.

**Pipeline unification:** `FeaturePipeline::execute` returns `OperationResult<FeatureOutput>`. The envelope carries all audit metadata; `FeatureOutput` carries only the domain result (topology + geometry). The `Arc`-wrapped decision/replay/lineage fields on `FeatureOutput` become redundant and are removed — that metadata lives in the envelope where it belongs.

```
Before:  Feature::evaluate → Result<FeatureOutput, KernelError>
                              (FeatureOutput manually packs decision_log, replay_log, lineage_events)

After:   FeaturePipeline::execute → OperationResult<Result<FeatureOutput, KernelError>>
                                     (envelope carries decisions, metrics, warnings, lineage, hashes)
                                     (FeatureOutput carries only topology + geometry)
```

This means:

- `FeatureTree::evaluate_feature` stores `OperationResult<FeatureOutput>` per node instead of bare `FeatureOutput`
- Sub-operation absorption (`absorb_sub_result`, `absorb_metadata`) works naturally — envelope-to-envelope
- `OperationFinalizer` at the feature boundary drains `ModelingContext` into the envelope exactly once
- `into_result()` handles trace persistence on both success and error paths (already exists)

### Other Existing Components

| Component                      | Location                       | Pipeline Relationship                                                                                      |
| ------------------------------ | ------------------------------ | ---------------------------------------------------------------------------------------------------------- |
| `PrimitiveSpec` / intent layer | `features/intent.rs`           | Orthogonal — operates before the pipeline. Intent resolves _what_ to build; pipeline orchestrates _how_.   |
| `BooleanEngine` traits         | `operations/boolean/traits.rs` | Internal to boolean. Pipeline wraps around `execute_boolean`, not inside the engine dispatch.              |
| `check_tolerance!` macro       | `core/macros.rs`               | Per-decision recording inside step implementations. Pipeline reads the decisions after; macro stays as-is. |

---

## Architecture: Three Tiers

```
┌─────────────────────────────────────────────────────────┐
│  Tier 0: Command Dispatch                               │
│  Bridges forge-schema Commands → FeatureTree            │
│  • Maps Command variants to NativeFeature construction  │
│  • Resolves EntityRef targets to FeatureOutput          │
│  • Inserts feature nodes into the signal graph          │
├─────────────────────────────────────────────────────────┤
│  Tier 1: FeaturePipeline                                │
│  Wraps top-level features                               │
│  • Compiler-enforced FeatureContract (sealed supertrait) │
│  • Typed input parsing (associated type DTOs)           │
│  • Post-invariant validation                            │
│  • Composes with KernelDraft + OperationFinalizer       │
├─────────────────────────────────────────────────────────┤
│  Tier 2: OperationPipeline                              │
│  Sequences sub-operations within a feature              │
│  • StepContract-driven auto-injection                   │
│  • Step-level audit rollup via DecisionLog checkpoints  │
│  • PipelineBuilder for typed intermediate state         │
├─────────────────────────────────────────────────────────┤
│  Tier 3: Step Library                                   │
│  Reusable atomic sub-operations                         │
│  • resolve_persistent_selection                         │
│  • classify_surface_pair                                │
│  • certify_boundary, validate_manifold, etc.            │
└─────────────────────────────────────────────────────────┘
```

---

## Tier 0: Command Dispatch

`forge-schema` defines a `Command` enum (`AddBlock`, `AddHole`, `BooleanUnion`, etc.) with `EntityRef` targeting. Currently **nothing bridges Commands to the feature system**. Tier 0 is that bridge.

```rust
pub struct CommandDispatcher<'a> {
    tree: &'a mut FeatureTree,
}

impl<'a> CommandDispatcher<'a> {
    /// Execute a schema command: resolve targets, construct the feature, insert into tree.
    pub fn dispatch(&mut self, cmd: &Command) -> Result<NodeId, KernelError> {
        match cmd {
            Command::AddBlock { origin, dimensions } => {
                let feature = NativeFeature::MakeCube(MakeCubeFeature::new("block", *origin, dimensions[0]));
                self.tree.register_feature(feature)
            }
            Command::BooleanSubtract { target, tool } => {
                let target_id = self.resolve_entity_ref(target)?;
                let tool_id = self.resolve_entity_ref(tool)?;
                let feature = NativeFeature::Boolean(BooleanFeature::new("subtract", BooleanOp::Subtract, target_id, tool_id));
                self.tree.register_feature(feature)
            }
            // ... exhaustive match on Command variants
        }
    }

    fn resolve_entity_ref(&self, entity: &EntityRef) -> Result<NodeId, KernelError> {
        match entity {
            EntityRef::ByFeature(name) => self.tree.get_node_by_name(name)
                .ok_or(KernelError::InvalidInput { message: format!("Feature '{}' not found", name), context: None }),
            EntityRef::ByIndex(idx) => /* resolve by insertion order */,
        }
    }
}
```

This is ~100 LOC. It grows one match arm per `Command` variant, which the compiler enforces exhaustively.

### Tier 0 File Manifest

| File                   | Purpose                                   |
| ---------------------- | ----------------------------------------- |
| `features/dispatch.rs` | `CommandDispatcher`, `resolve_entity_ref` |

### Tier 0 Acceptance Tests

- `dispatch_add_block_creates_make_cube_feature`
- `dispatch_boolean_subtract_resolves_entity_refs`
- `dispatch_unknown_entity_ref_returns_error`

---

## Tier 1: Feature Contracts and Pipeline

### 1.1 FeatureContract Trait (Sealed Supertrait)

```rust
pub trait FeatureContract {
    fn feature_kind(&self) -> &str;
    fn required_policies(&self) -> &[PolicyKind];
    fn entity_origins(&self) -> &[EntityOriginKind];
    fn post_invariants(&self) -> &[InvariantKind];
    fn audit_level(&self) -> AuditLevel;
}
```

> [!NOTE]
> `EntityOriginKind` is a **new enum** introduced by this spec. It does not exist in the codebase today. It classifies what kind of Euler/topological operations a feature uses, for lineage tracking and audit purposes. Defined in `features/contract.rs` alongside the other contract types:
>
> ```rust
> #[derive(Debug, Clone, Copy, PartialEq, Eq)]
> pub enum EntityOriginKind {
>     EulerOperator,      // Creates entities via MEV/MEF/MVF etc.
>     SplitOperator,      // Splits existing entities (boolean split phase)
>     MergeOperator,      // Merges entities (region merge, join_faces)
>     CopyOperator,       // Copies entities from one solid to another
> }
> ```

### 1.2 `FeatureOutput` Unification

Today `FeatureOutput` carries bespoke `Arc<DecisionLog>`, `Arc<ReplayLog>`, `Arc<Vec<LineageEvent>>` fields. These duplicate what `OperationResult` already provides. The pipeline slims `FeatureOutput` to domain-only data:

```rust
/// After pipeline unification — domain result only.
pub struct FeatureOutput {
    pub topology: TopologyState,
    pub geometry: GeometryState,
}
```

All audit metadata (decisions, replay, lineage, metrics, warnings, hashes, error budget) lives in `OperationResult<FeatureOutput>`, which the pipeline constructs and the `FeatureTree` stores per-node. This eliminates the two-metadata-path problem and means every feature automatically gets the full `OperationResult` envelope without manual `Arc` wrapping.

### 1.3 Sealed Feature Trait

```rust
pub trait Feature: FeatureContract + std::fmt::Debug + Any {
    type Inputs: FeatureInputs;
    fn parse_inputs(&self, raw: &HashMap<NodeId, FeatureOutput>) -> Result<Self::Inputs, KernelError>;
    fn execute_typed(&self, inputs: &Self::Inputs, ctx: &mut ModelingContext) -> Result<FeatureOutput, KernelError>;
    fn dependencies(&self) -> Vec<NodeId>;
    fn name(&self) -> &str;
}
```

> [!NOTE]
> **Why `&mut ModelingContext`, not a separate `ScratchpadContext`:** Transactional isolation is provided by `KernelDraft` at the feature level, not by a separate context type. `FeaturePipeline::execute` creates a `KernelDraft` before calling `execute_typed` and commits or rolls back on success/failure. `ModelingContext` provides policy resolution, decision logging, and sub-operation metadata absorption — all of which are needed inside feature execution and already exist. A separate `ScratchpadContext` would duplicate this surface area for no isolation benefit beyond what `KernelDraft` already provides. See Existing Infrastructure table.
>
> **Migration from current `Feature::evaluate`:** The current trait has `fn evaluate(&self, inputs: &HashMap<NodeId, FeatureOutput>) -> Result<FeatureOutput, KernelError>` with no context parameter. `execute_typed` replaces `evaluate` — the old method is removed, not kept as a default. `FeatureTree::evaluate_feature` must be updated to receive `&mut ModelingContext` and pass it through the `forge_signal::evaluate` closure. See §1.9 for the migration path.

### 1.4 Typed Input DTOs with Validation

`FeatureInputs` carries semantic validation, not just marking:

```rust
pub trait FeatureInputs {
    /// Semantic validation of parsed inputs.
    /// Called by pipeline AFTER parse_inputs, BEFORE execute_typed.
    fn validate(&self) -> Result<(), KernelError>;
}

pub struct BooleanInputs {
    pub target: FeatureOutput,
    pub tool: FeatureOutput,
}

impl FeatureInputs for BooleanInputs {
    fn validate(&self) -> Result<(), KernelError> {
        if self.target.topology.arena().face_count() == 0 {
            return Err(KernelError::InvalidInput { message: "target has no faces".into(), context: None });
        }
        Ok(())
    }
}
```

### 1.5 `declare_feature!` Macro

```rust
declare_feature!(MakeCubeFeature,
    kind: "make_cube",
    policies: [],
    origins: [EntityOriginKind::EulerOperator],
    invariants: [InvariantKind::ManifoldEdges],
    audit: AuditLevel::Summary,
);
```

### 1.6 FeaturePipeline Executor

Composes with `OperationFinalizer` for finalization boundary. The finalizer's real API is `OperationFinalizer::new(&mut ModelingContext)` and `collect_success/collect_error(&mut self, &mut OperationResult<T>, TraceAdjunctSet, TopologyHashBoundary)` — it drains context decisions and sub-op metadata into the envelope exactly once.

```rust
impl FeaturePipeline {
    pub fn execute<F: Feature>(
        feature: &F,
        raw_inputs: &HashMap<NodeId, FeatureOutput>,
        ctx: &mut ModelingContext,
    ) -> Result<FeatureOutput, KernelError> {
        // 1. Pre-validate policies (fail-fast)
        for policy in feature.required_policies() {
            ctx.validate_policy_configured(policy)?;
        }

        // 2. Parse + validate typed inputs
        let inputs = feature.parse_inputs(raw_inputs)?;
        inputs.validate()?;

        // 3. Snapshot topology hash before execution
        let hash_before = /* compute_arena_topology_hash on input state */;

        // 4. Execute business logic
        let result = feature.execute_typed(&inputs, ctx);

        // 5. Finalize — drain decisions + metadata from ctx into envelope
        //    OperationFinalizer::new takes &mut ModelingContext (borrows ctx)
        //    collect_success/error takes &mut OperationResult<T>, TraceAdjunctSet, TopologyHashBoundary
        let hash_after = result.as_ref().ok().map(|o| /* hash output topology */);
        let hashes = TopologyHashBoundary { before: Some(hash_before), after: hash_after };

        let mut envelope = OperationResult::new(result);
        let mut finalizer = OperationFinalizer::new(ctx);
        let finalization = match envelope.get_value() {
            Ok(_) => finalizer.collect_success(&mut envelope, TraceAdjunctSet::new(), hashes),
            Err(_) => finalizer.collect_error(&mut envelope, TraceAdjunctSet::new(), hashes),
        }?;
        // finalizer drops here — ctx borrow released

        // 6. Post-validate invariants (only on success)
        if let Ok(output) = envelope.get_value() {
            for inv in feature.post_invariants() {
                validate_invariant(&output.topology, inv)?;
            }
        }

        // 7. Audit (explicit match, not >= comparison)
        if let Ok(output) = envelope.get_value() {
            match feature.audit_level() {
                AuditLevel::Full => emit_feature_audit(feature, output, &finalization, ctx)?,
                AuditLevel::Summary => emit_feature_summary(feature, output, &finalization, ctx)?,
                AuditLevel::None => {},
            }
        }

        envelope.into_value()
    }
}
```

### 1.7 NativeFeature Exhaustive Dispatch

Adding a feature to Forge triggers a compiler-enforced chain:

1. Add `NativeFeature::Fillet(FilletFeature)` → compiler forces all `match` arms
2. Implement `Feature for FilletFeature` → compiler forces `FeatureContract` (supertrait)
3. `FeatureContract` → compiler forces `required_policies`, `post_invariants`, `audit_level`, `entity_origins`
4. `Feature::Inputs` → compiler forces typed input struct + `parse_inputs` adapter + `validate`

### 1.8 Acceptance Tests (Tier 1)

- `feature_without_contract_does_not_compile` (compile-fail test — this is the single most important test for the sealed supertrait guarantee; use `trybuild` crate)
- `pipeline_rejects_feature_with_missing_policy_configuration`
- `pipeline_validates_inputs_before_execution`
- `pipeline_validates_post_invariants_after_execution`
- `pipeline_emits_audit_at_full_level`
- `pipeline_skips_audit_at_none_level`
- `typed_inputs_reject_missing_dependency`

> [!NOTE]
> The compile-fail test uses `trybuild`. It verifies that `impl Feature for Foo` without `impl FeatureContract for Foo` produces a compiler error. This is the critical enforcement point — without it, the sealed supertrait guarantee is aspirational, not proven.

### 1.9 FeatureTree Migration Path

The current `FeatureTree::evaluate_feature` calls `feature.evaluate(&inputs)` inside a `forge_signal::evaluate` closure. The new `Feature` trait replaces `evaluate` with `execute_typed`, which requires `&mut ModelingContext`.

**Migration steps:**

1. `FeatureTree` receives `&mut ModelingContext` as a parameter to `evaluate_feature`.
2. The `forge_signal::evaluate` closure captures `&mut ctx` and passes it to `FeaturePipeline::execute(feature, &inputs, ctx)`.
3. `FeaturePipeline::execute` handles contract validation, input parsing, execution, post-invariants, finalization, and audit — the closure becomes a thin dispatch.
4. `FeatureTree` stores `OperationResult<FeatureOutput>` per node (replacing bare `FeatureOutput`). The envelope carries all audit metadata; `FeatureOutput` is domain-only (topology + geometry). Trace summary extraction for `NodeEntry` reads from the envelope's decision log instead of a separate `Arc<DecisionLog>` field.

**Boolean-specific note:** `execute_boolean` today takes `BooleanInput` and returns `OperationResult<Result<BooleanResult, KernelError>>` with no `ModelingContext` parameter. The `BooleanFeature::execute_typed` implementation will:

1. Call `execute_boolean(input)` — returns `OperationResult<Result<BooleanResult, KernelError>>`
2. Call `ctx.absorb_sub_result(&mut envelope)` — drains decisions, metrics, warnings, lineage into context
3. Extract `BooleanResult` and build `FeatureOutput { topology, geometry }` (no manual `Arc` wrapping)

The boolean internal pipeline (EMBER → split → classify → assemble → postprocess) is unchanged — the feature pipeline wraps _around_ it, not inside it.

### 1.10 `validate_policy_configured` Specification

This method does **not** exist on `ModelingContext` today and must be added. It is a thin pre-check, distinct from `resolve_policy_query` (which requires a full `PolicyQuery` with location/margin/overridable):

```rust
impl ModelingContext {
    /// Verify that a policy kind has a configured resolution strategy
    /// (default, session override, model override, or operation override).
    /// Returns Ok(()) if any scope has a configuration for this kind.
    /// Returns Err(PipelineError::PolicyNotConfigured) if no scope covers it.
    ///
    /// This is a fail-fast pre-check — it does NOT resolve the policy,
    /// it only verifies that resolution won't hit ForcedSafeFallback
    /// due to total absence of configuration.
    pub fn validate_policy_configured(&self, kind: &PolicyKind) -> Result<(), KernelError> {
        // Check if any layer in the PolicyRegistrySnapshot has a rule for `kind`.
        // Implementation: iterate policy_registry_snapshot().rules_for(kind)
        // and return Ok if non-empty.
    }
}
```

---

## Tier 2: Operation Pipeline and Step Contracts

### 2.1 StepContract Trait

```rust
pub trait StepContract {
    fn step_name(&self) -> &str;
    fn policy_queries(&self) -> &[PolicyKind];
    fn precision_sensitive(&self) -> bool;
}
```

### 2.2 `declare_step!` Macro

```rust
declare_step!(ClassifySurfacePair,
    name: "classify_surface_pair",
    policies: [PolicyKind::CoincidentGeometry, PolicyKind::NearTangency],
    precision_sensitive: true,
);
```

### 2.3 OperationPipeline with PipelineBuilder

For operations with 5+ steps, local variable threading through closures gets unwieldy. The `PipelineBuilder` provides typed intermediate state:

```rust
pub struct OperationPipeline<'a> {
    ctx: &'a mut ModelingContext,
    steps_executed: Vec<StepAuditEntry>,
}

impl<'a> OperationPipeline<'a> {
    pub fn new(ctx: &'a mut ModelingContext) -> Self {
        Self { ctx, steps_executed: Vec::new() }
    }

    /// Run a step with auto-injected context.
    /// Reads StepContract to: validate policies, set precision, collect audit.
    pub fn run_step<S, R, F>(&mut self, step: &S, execute: F) -> Result<R, KernelError>
    where
        S: StepContract,
        F: FnOnce(&mut ModelingContext) -> Result<R, KernelError>,
    {
        // Auto-injection: validate policies
        for policy in step.policy_queries() {
            self.ctx.validate_policy_configured(policy)?;
        }

        // Checkpoint for step-scoped audit
        let checkpoint = self.ctx.get_decision_count();

        // Execute
        let result = execute(self.ctx)?;

        // Collect step audit
        let decisions_count = self.ctx.get_decision_count() - checkpoint;
        self.steps_executed.push(StepAuditEntry {
            name: step.step_name().to_string(),
            decision_count: decisions_count,
            precision_sensitive: step.precision_sensitive(),
        });

        Ok(result)
    }

    /// Finalize using existing OperationFinalizer.
    pub fn finalize(self) -> OperationAuditRecord {
        OperationAuditRecord { steps: self.steps_executed }
    }
}
```

#### PipelineBuilder (typed intermediate state for 5+ steps)

For complex features like fillet with 6+ steps:

```rust
pub struct PipelineBuilder<'a, State> {
    pipeline: OperationPipeline<'a>,
    state: State,
}

impl<'a, State> PipelineBuilder<'a, State> {
    pub fn start(ctx: &'a mut ModelingContext, initial: State) -> Self {
        Self { pipeline: OperationPipeline::new(ctx), state: initial }
    }

    /// Run a step that transforms the intermediate state.
    pub fn then<S, NextState, F>(
        mut self, step: &S, transform: F,
    ) -> Result<PipelineBuilder<'a, NextState>, KernelError>
    where
        S: StepContract,
        F: FnOnce(State, &mut ModelingContext) -> Result<NextState, KernelError>,
    {
        let next = self.pipeline.run_step(step, |ctx| {
            transform(self.state, ctx)
        })?;
        Ok(PipelineBuilder { pipeline: self.pipeline, state: next })
    }

    /// Finalize and return the final state + audit record.
    pub fn finish(self) -> (State, OperationAuditRecord) {
        let audit = self.pipeline.finalize();
        (self.state, audit)
    }
}
```

Usage for fillet:

```rust
let (result, audit) = PipelineBuilder::start(ctx, selection)
    .then(&ResolveSelection, |sel, ctx| resolve_edge_chain(&sel, ctx))?
    .then(&ClassifyEdgeConvexity, |edges, ctx| classify_convexity(&edges, ctx))?
    .then(&ConstructSurface, |conv, ctx| construct_fillet_surface(&conv, radius, ctx))?
    .then(&ApplyEulerOps, |blend, ctx| apply_fillet_topology(&blend, ctx))?
    .then(&ValidateManifold, |topo, ctx| validate_manifold(&topo).map(|_| topo))?
    .then(&DetectSlivers, |topo, ctx| detect_slivers(&topo, ctx).map(|_| topo))?
    .finish();
```

> [!NOTE]
> **Known constraint:** `PipelineBuilder::then` consumes `self.state` by move, so there is no implicit "look-back" to a previous step's result. Each step receives only the output of the immediately preceding step. For operations where step N needs results from step N-2, the state type must be a composite struct that carries forward all needed values (e.g., `struct FilletClassifiedState { edges: Vec<Edge>, convexity: Vec<Convexity> }`). For linear pipelines like fillet this is natural; for operations with complex data dependencies, consider using `OperationPipeline::run_step` directly with explicit local variables instead of `PipelineBuilder`.

### 2.4 Acceptance Tests (Tier 2)

- `run_step_rejects_missing_policy_before_execution`
- `run_step_collects_step_scoped_decision_counts`
- `pipeline_builder_threads_typed_state_through_steps`
- `multi_step_pipeline_sequences_audit_entries_in_order`
- `pipeline_overhead_per_step_under_1_microsecond` (benchmark)

---

## Tier 3: Reusable Step Library

### 3.1 Step Catalog

| #   | Step                           | StepContract                                                  | Used By                                           |
| --- | ------------------------------ | ------------------------------------------------------------- | ------------------------------------------------- |
| 1   | `resolve_persistent_selection` | policies: [], precision: false                                | Boolean, Fillet, Chamfer, Shell, Extrude, Pattern |
| 2   | `classify_surface_pair`        | policies: [CoincidentGeometry, NearTangency], precision: true | Boolean, Fillet, Chamfer                          |
| 3   | `classify_edge_convexity`      | policies: [NearTangency], precision: true                     | Fillet, Chamfer                                   |
| 4   | `certify_boundary`             | policies: [CoincidentGeometry], precision: true               | Boolean, Fillet, Chamfer, Shell                   |
| 5   | `construct_surface`            | policies: [], precision: true                                 | Fillet, Chamfer, Shell, Extrude                   |
| 6   | `apply_euler_ops`              | policies: [], precision: false                                | All (D6 transactional)                            |
| 7   | `validate_manifold`            | policies: [], precision: false                                | All                                               |
| 8   | `detect_slivers`               | policies: [SliverFace], precision: true                       | Boolean, Fillet, Chamfer                          |

> [!NOTE]
> **Relationship to boolean's internal phases:** Boolean's existing pipeline (EMBER quantize → split → classify → assemble → postprocess) is NOT replaced or wrapped by these steps. Boolean's internal phases are engine-specific and orchestrated by `BooleanEngine` traits (`Splitter`, `Classifier`, `Assembler`, `PostProcessor`). The step catalog above captures _cross-feature_ reusable operations — things that fillet, chamfer, and boolean all do. For example, `certify_boundary` wraps the existing `forge-geom::boundary_cert` module, and `detect_slivers` wraps `forge-kernel::analysis::sliver::analyze_slivers`. Boolean's `execute_typed` implementation calls `execute_boolean` internally and uses the step library only for pre/post operations (selection resolution, manifold validation, sliver detection).

### 3.2 Queryability

```bash
grep -r "PolicyKind::CoincidentGeometry" --include="*.rs" crates/forge-kernel/src/features/
grep -r "precision_sensitive: true" --include="*.rs" crates/forge-kernel/src/operations/
grep -r "InvariantKind::ManifoldEdges" --include="*.rs" crates/forge-kernel/src/features/
```

---

## Cross-Cutting: Validation Integration

The spec declares `InvariantKind` and calls `validate_invariant(&output.topology, inv)` — but no such function exists yet. `forge-topo` has `validate_topology()` (structural) and `validate_geometric_invariants()` (geometric), and `forge-kernel` has `ValidationCheckpoint` with configurable modes (debug/release/all/disabled). The pipeline wires these together.

### `validate_invariant` Implementation

Maps `InvariantKind` variants to existing validators:

```rust
pub fn validate_invariant(
    topology: &TopologyState,
    geometry: &GeometryState,
    kind: &InvariantKind,
) -> Result<(), KernelError> {
    match kind {
        InvariantKind::ManifoldEdges => {
            // Delegates to forge_topo::integrity::structural::validate_topology()
            // Checks: every edge has exactly 2 uses, Euler formula holds
            validate_topology(topology.arena())?;
            Ok(())
        }
        InvariantKind::G1Continuity => {
            // Delegates to forge_topo::integrity::geometric::validate_geometric_invariants()
            // Checks: face normals agree at shared edges within tangency tolerance
            validate_geometric_invariants(topology.arena(), geometry)?;
            Ok(())
        }
        InvariantKind::NoSelfIntersection => {
            // Future: spatial self-intersection test via BVH
            Ok(()) // Stub until curved geometry demands it
        }
        InvariantKind::NoSliverFaces => {
            // Delegates to forge_kernel::analysis::sliver::analyze_slivers()
            let report = analyze_slivers(topology.arena(), geometry)?;
            if !report.slivers.is_empty() {
                return Err(KernelError::DiagnosticFailure { /* sliver details */ });
            }
            Ok(())
        }
    }
}
```

### `ValidationCheckpoint` Integration

`ModelingContext` already carries a `ValidationConfig` with checkpoint modes. The pipeline checks this after execution:

```rust
// Inside FeaturePipeline::execute, after step 4 (execute_typed):
if ctx.validation_config().is_checkpoint_active(ValidationCheckpoint::PostFeature) {
    for inv in feature.post_invariants() {
        validate_invariant(&output.topology, &output.geometry, inv)?;
    }
}
```

This respects the existing config — debug builds validate everything, release builds skip expensive checks, disabled mode skips all. The pipeline doesn't invent a new config system.

## Cross-Cutting: Optional Analysis Hooks

`forge-kernel::analysis` has causal chain reconstruction, counterfactual replay, and proof validation — all producing data the pipeline can carry in `OperationResult` trace adjuncts. These are opt-in post-finalization hooks, not core pipeline stages.

```rust
// Optional, enabled by ModelingContext config or feature contract flag:
if ctx.analysis_config().causal_chain_enabled {
    let chain = query_causal_chain(&output.topology, envelope.get_decision_log(), &replay);
    envelope.add_trace_adjunct("causal_chain", &chain);
}

if ctx.analysis_config().counterfactual_enabled {
    let report = replay_all_near_boundary(envelope.get_decision_log());
    envelope.add_trace_adjunct("counterfactual", &report);
}
```

These are expensive (causal chain is O(faces × lineage depth), counterfactual replays the entire operation per near-boundary decision). They're for debugging and proof, not production. The pipeline carries the results in the envelope when enabled; consumers (forge-view trace viewer, proof validation suites) read them.

---

## Pipeline Error Taxonomy

The pipeline introduces its own typed error cases (wrapping into `KernelError`):

```rust
pub enum PipelineError {
    /// A required policy is not configured in the ModelingContext.
    PolicyNotConfigured { kind: PolicyKind, feature: String },

    /// A post-execution invariant was violated.
    InvariantViolation { kind: InvariantKind, detail: String },

    /// Input parsing failed (missing dependency, wrong type).
    InputParseFailure { expected: String, actual: String },

    /// Input semantic validation failed.
    InputValidationFailure { message: String },

    /// A step within the operation failed.
    StepExecutionFailed { step: String, source: Box<KernelError> },
}
```

These wrap via `impl From<PipelineError> for KernelError` with `ErrorContext` pointing to the pipeline stage.

---

## Enums

All three enums below are **new types** introduced by this spec. None exist in the codebase today.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditLevel { None, Summary, Full }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantKind { ManifoldEdges, G1Continuity, NoSelfIntersection, NoSliverFaces }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityOriginKind { EulerOperator, SplitOperator, MergeOperator, CopyOperator }
```

> [!NOTE]
> `AuditLevel` does NOT derive `PartialOrd`. All dispatch uses explicit `match` to prevent silent ordering bugs when adding new levels.

---

## Deterministic Parallelism (Future Path)

The current `OperationPipeline` executes steps sequentially. This is correct for determinism. However, deterministic parallelism IS achievable for future features like multi-edge fillet:

**Strategy:** Fork-then-merge with deterministic key sorting.

```rust
// Future: parallel edge classification for fillet
let per_edge_contexts: Vec<_> = edge_chains
    .par_iter()                              // rayon parallel
    .map(|chain| {
        let mut local_ctx = ctx.fork_local();  // thread-local context
        let result = classify_edge_chain(chain, &mut local_ctx);
        (chain.ordering_key(), result, local_ctx)
    })
    .collect();

// Merge in deterministic order (sorted by OrderingKey)
per_edge_contexts.sort_by_key(|(key, _, _)| *key);
for (_, result, local_ctx) in per_edge_contexts {
    ctx.merge_local(local_ctx);  // decisions merge in key order
    results.push(result?);
}
```

**Key insight:** Parallelism is in _computation_, determinism is in _merge order_. The `OrderingKey` (which already exists in `forge-topo::ordering`) provides the stable sort key. Decision logs merge in `OrderingKey` order regardless of which thread finished first.

This requires:

- `ModelingContext::fork_local()` — creates a thread-local context with shared policy registry, independent decision log
- `ModelingContext::merge_local()` — drains decisions from local into parent in caller-determined order

This is ~200 LOC of infrastructure and can be built when the first parallel feature demands it. The `&mut self` on `run_step` does NOT prevent this — the parallelism happens _inside_ a step, not between steps.

---

## `classify_surface_pair` Migration Plan

> [!WARNING]
> This is the highest blast-radius change. Concrete plan:

1. **Single atomic commit.** No feature flag.
2. Change `classify_surface_pair` in `forge-geom/src/surface/eval.rs` from `-> SurfaceRelation` to `-> PolicyResult<SurfaceRelation>`
3. Update all callers (actual call sites as of 2026-02-25):
   - `forge-kernel/src/geometry_state/adversarial_tests.rs` (~10 call sites, test-only)
   - `forge-kernel/src/operations/boolean/postprocess/curved_merge/` (references `SurfaceRelation` conceptually; may need updates if it calls `classify_surface_pair` directly after curved merge is implemented)
   - Any future callers added before this migration lands
4. Each non-test caller resolves the `PolicyResult` via `ctx.resolve_policy_query()`; test callers use `into_result_strict()` or `into_result_accepting()`
5. All test updates in same commit
6. Run `cargo test --workspace` before merge

---

## Directory Layout: Adapter-by-Default

Pipeline infrastructure lives in dedicated `pipeline/` sub-directories. Feature and operation implementations are **consumers** that adapt to the pipeline — even when a feature's pipeline usage is a pass-through (e.g., MakeCube has no policies to validate). This eliminates "should I abstract this?" decisions: every feature goes through the pipeline, and the pipeline degrades to a no-op for simple cases.

```
features/
├── mod.rs              ← table of contents
├── intent.rs           ← PrimitiveSpec (dual SDF/B-Rep)
├── traits.rs           ← Feature trait, FeatureOutput
├── tree.rs             ← FeatureTree, NativeFeature enum
├── wrappers.rs         ← MakeCubeFeature, BooleanFeature (consumers)
└── pipeline/           ← canonical feature pipeline infrastructure
    ├── mod.rs           ← pipeline table of contents
    ├── dispatch.rs      ← Tier 0: CommandDispatcher
    ├── contract.rs      ← Tier 1: FeatureContract, AuditLevel, InvariantKind
    ├── executor.rs      ← Tier 1: FeaturePipeline::execute
    ├── macros.rs        ← Tier 1: declare_feature!
    └── tests.rs         ← tests for all pipeline tiers

operations/
├── pipeline/            ← step-level infrastructure (Tier 2)
│   ├── mod.rs
│   ├── step_contract.rs
│   ├── builder.rs
│   └── tests.rs
├── boolean/             ← consumer of operations/pipeline/
├── fillet/              ← future consumer
```

---

## File Manifest

### New Files

| File                                   | Tier | Purpose                                                   |
| -------------------------------------- | ---- | --------------------------------------------------------- |
| `features/pipeline/mod.rs`             | —    | Pipeline sub-directory table of contents                  |
| `features/pipeline/dispatch.rs`        | 0    | CommandDispatcher: Command → NativeFeature → FeatureTree  |
| `features/pipeline/contract.rs`        | 1    | FeatureContract, AuditLevel, InvariantKind, FeatureInputs |
| `features/pipeline/macros.rs`          | 1    | declare_feature! macro                                    |
| `features/pipeline/executor.rs`        | 1    | FeaturePipeline::execute, validate_invariant              |
| `features/pipeline/tests.rs`           | 0–1  | Acceptance tests for all pipeline tiers                   |
| `operations/pipeline/mod.rs`           | 2    | Operation pipeline table of contents                      |
| `operations/pipeline/step_contract.rs` | 2    | StepContract, declare_step!, step types                   |
| `operations/pipeline/builder.rs`       | 2    | OperationPipeline, PipelineBuilder, StepAuditEntry        |
| `operations/pipeline/tests.rs`         | 2    | Operation pipeline tests                                  |

### Modified Files

| File                   | Tier | Changes                                                                                                                                      |
| ---------------------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `features/traits.rs`   | 1    | Feature: FeatureContract supertrait, type Inputs; slim FeatureOutput to topology+geometry only (remove Arc'd decision/replay/lineage fields) |
| `features/wrappers.rs` | 1    | declare_feature! for MakeCube + Boolean, typed inputs; wrappers build slim FeatureOutput                                                     |
| `features/tree.rs`     | 1    | Route evaluate_feature through FeaturePipeline; store `OperationResult<FeatureOutput>` per node; accept `&mut ModelingContext`               |
| `core/context.rs`      | —    | Add `validate_policy_configured` (see §1.10 for specification)                                                                               |
| `envelope/schema.rs`   | —    | Add `OperationMetrics::accumulate`, `LineageDelta::accumulate` (dedup manual field addition)                                                 |

---

## Performance Budget

Per-step overhead is bounded by:

- **Policy validation:** One `&[PolicyKind]` slice iteration + hash lookup per policy (~50ns)
- **Checkpoint:** One `self.ctx.get_decision_count()` call (~1ns)
- **Audit collection:** One `Vec::push(StepAuditEntry)` (~10ns)
- **Total:** <100ns per step

Verification: `pipeline_overhead_per_step_under_1_microsecond` benchmark test in Phase 0b.

For context: a single `orient3d` predicate takes ~30ns. The pipeline overhead is less than 4 predicate calls. Negligible even for NURBS operations with 50+ steps.

---

## Verification Plan

```bash
cargo test -p forge-kernel feature_contract
cargo test -p forge-kernel step_contract
cargo test -p forge-kernel operation_pipeline
cargo test -p forge-kernel feature_pipeline
cargo test --workspace
```

## Definition of Done

1. All three tiers have concrete code + tests
2. MakeCube and Boolean features run through the full pipeline
3. Adding a new `NativeFeature` variant without `FeatureContract` fails to compile
4. `OperationPipeline::run_step` auto-injects context based on `StepContract`
5. `PipelineBuilder` threads typed state through 6+ steps cleanly
6. `PipelineError` has typed variants for all pipeline failure modes
7. Region merge specs (Phases 1-4) implemented as pipeline consumers
8. `cargo test --workspace` passes with zero regressions

---

## Scalability: Does This Reach NURBS and Fillets?

This section traces the path from today's planar boolean through the full operation roadmap to confirm the pipeline doesn't paint us into a corner.

### Feature Roadmap Through the Pipeline

| Feature        | Tier 1 (FeatureContract)                                                                               | Tier 2 (OperationPipeline / PipelineBuilder)                                                                         | Tier 3 (Step Library)                                                                                                                            | Notes                                                                                           |
| -------------- | ------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------- |
| **MakeCube**   | `policies: [], invariants: [ManifoldEdges]`                                                            | No sub-steps (single Euler op chain)                                                                                 | `apply_euler_ops`, `validate_manifold`                                                                                                           | Trivial — validates the pipeline with zero policy overhead                                      |
| **Boolean**    | `policies: [CoincidentGeometry, NearTangency, SliverFace], invariants: [ManifoldEdges, NoSliverFaces]` | Wraps `execute_boolean` as a single step; boolean's internal phases stay as-is                                       | `resolve_persistent_selection`, `validate_manifold`, `detect_slivers` (pre/post only)                                                            | Boolean's EMBER/standard pipeline is opaque to Tier 2 — no change to internal architecture      |
| **Fillet**     | `policies: [NearTangency, SliverFace], invariants: [ManifoldEdges, G1Continuity]`                      | 6-step `PipelineBuilder`: resolve → classify convexity → construct surface → apply Euler → validate → detect slivers | All of: `resolve_persistent_selection`, `classify_edge_convexity`, `construct_surface`, `apply_euler_ops`, `validate_manifold`, `detect_slivers` | First real consumer of `PipelineBuilder`'s typed state threading                                |
| **Chamfer**    | Same policies as fillet, minus G1Continuity                                                            | Nearly identical pipeline to fillet (flat surface instead of blend)                                                  | Same steps as fillet                                                                                                                             | Validates that step library enables code sharing without inheritance                            |
| **Shell**      | `policies: [CoincidentGeometry], invariants: [ManifoldEdges]`                                          | 4-step: resolve faces → offset surfaces → rebuild topology → validate                                                | `resolve_persistent_selection`, `construct_surface`, `apply_euler_ops`, `validate_manifold`, `certify_boundary`                                  | Offset surface construction reuses `construct_surface` step                                     |
| **Extrude**    | `policies: [], invariants: [ManifoldEdges]`                                                            | 3-step: resolve sketch → construct swept surface → apply Euler                                                       | `resolve_persistent_selection`, `construct_surface`, `apply_euler_ops`, `validate_manifold`                                                      | Simplest multi-step pipeline after MakeCube                                                     |
| **Loft/Sweep** | `policies: [NearTangency], invariants: [ManifoldEdges, G1Continuity]`                                  | Multi-step with intermediate surface fitting                                                                         | `construct_surface`, `apply_euler_ops`, `validate_manifold`                                                                                      | `construct_surface` step handles NURBS surface fitting via `SurfaceKind` dispatch               |
| **NURBS Trim** | `policies: [CoincidentGeometry, NearTangency], invariants: [ManifoldEdges]`                            | Multi-step: project trim curve → split face → validate                                                               | `classify_surface_pair`, `certify_boundary`, `apply_euler_ops`, `validate_manifold`                                                              | Trim is essentially a 2D boolean on a parametric surface — `certify_boundary` is critical       |
| **Pattern**    | Inherits from source feature                                                                           | Iterates a transform matrix, applies source feature N times                                                          | `resolve_persistent_selection`, then delegates to source feature's pipeline                                                                      | Pattern is a meta-feature — it calls `FeaturePipeline::execute` on the source feature in a loop |

### What The Pipeline Doesn't Constrain

- **Internal operation architecture**: Boolean keeps EMBER → split → classify → assemble → postprocess. Fillet will have its own internal phase structure. The pipeline wraps _around_ operations, not inside them.
- **Surface/curve representation**: `construct_surface` dispatches on `SurfaceKind` (planar, NURBS, analytic). The step contract doesn't know or care about surface type — it just declares precision sensitivity.
- **Precision escalation path**: Steps that set `precision_sensitive: true` get the existing `PrecisionMode` escalation (float → compensated → interval → rational). NURBS evaluation adds parametric domain precision, but that's inside the step implementation, not the contract.
- **OperationSpace coordinate transforms**: The pipeline manages `OperationSpace::analyze/transform/restore` around execution. This works identically for planar and curved geometry — `OperationSpace` already handles scale analysis via `LocalCoordinateSpace`.

### Where We'll Need To Extend (Not Redesign)

1. **`InvariantKind` enum**: Will grow as features demand new post-conditions (e.g., `ClosedVolume` for shell, `WatertightBoundary` for trim). Each new variant = one match arm in `validate_invariant`. No structural change.

2. **`PolicyKind` enum**: May need `ParametricTolerance` or `SurfaceContinuity` for NURBS features. Adding a variant to `PolicyKind` in `forge-core` forces all `match` arms to update — compiler-enforced, no silent breakage.

3. **Step library growth**: New steps like `fit_nurbs_surface`, `project_trim_curve`, `validate_g2_continuity` will be added as features demand them. Each is a `declare_step!` invocation + implementation. The library grows additively.

4. **`PipelineBuilder` composite states**: Complex features may need composite state structs to carry forward multiple intermediate results. This is a known constraint (see §2.3 note), not a design limitation — it's just explicit about data dependencies.

---

## Pre-Implementation Codebase Prep

Small changes to existing infrastructure that should land before or alongside the pipeline:

### 1. Add `PolicyRegistrySnapshot::has_any_rule_for` (~10 LOC)

Currently `PolicyRegistrySnapshot` is a bag of public `BTreeMap` fields with no methods. The pipeline needs a clean way to check if any scope has a configuration for a given `PolicyKind`:

```rust
impl PolicyRegistrySnapshot {
    pub fn has_any_rule_for(&self, kind: &PolicyKind) -> bool {
        self.defaults.contains_key(kind)
            || self.session_overrides.contains_key(kind)
            || self.active_model_scope.as_ref()
                .and_then(|s| self.model_overrides.get(s))
                .is_some_and(|m| m.contains_key(kind))
            || self.active_feature_scope.as_ref()
                .and_then(|s| self.feature_overrides.get(s))
                .is_some_and(|m| m.contains_key(kind))
            || self.active_operation_scope.as_ref()
                .and_then(|s| self.operation_overrides.get(s))
                .is_some_and(|m| m.contains_key(kind))
    }
}
```

Then `validate_policy_configured` on `ModelingContext` is a one-liner calling this.

### 2. Add `OperationMetrics::accumulate` and `LineageDelta::accumulate` (~20 LOC each)

`OperationFinalizer::collect` currently has 15 lines of manual field-by-field addition for metrics and 14 lines for lineage. As more features use `absorb_sub_result`, this pattern repeats. Add:

```rust
impl OperationMetrics {
    pub fn accumulate(&mut self, other: &OperationMetrics) {
        self.duration += other.duration;
        self.entities_created += other.entities_created;
        self.entities_deleted += other.entities_deleted;
        self.entities_modified += other.entities_modified;
        self.exact_predicate_calls += other.exact_predicate_calls;
        self.policy_decisions_made += other.policy_decisions_made;
    }
}
```

Same pattern for `LineageDelta`. This deduplicates the manual accumulation in both `finalization.rs` and `context.rs::absorb_sub_result`, and prevents future field-addition bugs (add a field to the struct, forget to add it to the accumulation).

### 3. Slim `FeatureOutput` to domain-only (~20 LOC change, but touches many files)

Remove `Arc<DecisionLog>`, `Arc<ReplayLog>`, `Arc<Vec<LineageEvent>>` from `FeatureOutput`. These move into the `OperationResult` envelope that wraps it. All code that constructs a `FeatureOutput` today (wrappers.rs, test helpers) drops the `Arc` wrapping. All code that reads those fields (tree.rs trace summary, test assertions) reads from the envelope instead.

This is the single most impactful prep change — it eliminates the dual metadata path before the pipeline lands, so the pipeline doesn't have to reconcile two sources of truth.

### 4. No changes needed to `execute_boolean`

Boolean stays self-contained. `BooleanFeature::execute_typed` calls `execute_boolean(input)`, gets back `OperationResult`, calls `ctx.absorb_sub_result(&mut envelope)` to drain metadata, then extracts the `BooleanResult` and builds the slim `FeatureOutput { topology, geometry }`. The boolean pipeline's internal `ModelingContext::default()` creation is fine — its decisions get captured in the `OperationResult` envelope which the feature-level pipeline absorbs.
