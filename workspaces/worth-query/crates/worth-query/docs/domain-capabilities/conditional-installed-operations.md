# Conditional Installed Operations

## What This Feature Is

Conditional installed operations let a domain author eligibility, trigger,
comparison, maintenance, and output meaning as part of a portable Query
operation. Query installs that meaning into the runtime’s actual Bridge-owned
Signal graph and preserves Signal’s decision through execution and workflow
progression.

Use this for incremental geometry, derived values, guarded workflow stages,
threshold-sensitive recomputation, time-driven evaluation, and explicit
on-demand work.

## Why You Use It

- Author conditional nodes without exposing Signal node IDs or aspect slots in
  domain packages.
- Preserve exact Relational field, endpoint, structural, and lifecycle change
  meaning through invalidation.
- Keep condition, trigger, comparator, and artifact-reuse semantics in
  canonical operation identity.
- Ensure ineligible and suppressed work performs no compute.
- Ensure reverted-clean work records cost but produces no new Query
  consequence.

## Stable Entry Points

Portable authoring:

- `domain::WorthQueryPortableConditionalNodeDeclaration::declare(...)`
- `domain::WorthQuerySemanticTruthDependency::new(...)`
- `domain::WorthQueryConditionalEvaluationCondition`
- `domain::WorthQueryConditionalTrigger`
- `domain::WorthQueryComparatorRequirement`
- `domain::WorthQueryOutputEquivalenceRequirement`
- `domain::WorthQueryArtifactReuseEquivalence`
- `domain::WorthQueryMaintenancePosture`
- `domain::WorthQueryConditionalNodeOutput`

Runtime installation:

- `runtime::WorthQueryRuntimeBuilder::conditional_signal_graph(...)`
- `runtime::WorthQueryRuntimeBuilder::conditional_node(...)`
- `domain::WorthQueryConditionalDependencyInstallation`
- `domain::WorthQueryConditionalNodeComputeProvider`
- `worth_runtime_bridge::facade::BridgeConditionalProviderSet`

Runtime evidence:

- `domain::WorthQueryConditionalProvenance`
- `domain::WorthQueryConditionalOutcomeClass`
- `domain::WorthQueryConditionalExecutionIndexRebuildReport`

## Core Mental Model

A conditional node crosses four ownership boundaries:

```text
Query portable declaration
  -> Relational authoritative aspect change
  -> Runtime Bridge installed correspondence and lowering
  -> Signal condition decision and evaluation
  -> Query consequence or typed deferral
```

Each owner contributes a different fact:

- Query says what semantic truth the operation depends on and what the node is
  intended to do.
- Relational says exactly what authoritative truth changed.
- Runtime Bridge proves where that semantic dependency lives in this Signal
  graph and lowers portable condition meaning into the installed node.
- Signal decides eligibility, computation, reuse, and whether output changed
  meaningfully.
- Query admits the returned evidence into the bound operation or workflow. It
  does not restamp the decision.

## How It Executes

1. Query validates and canonicalizes the portable node inside the operation.
2. Runtime construction admits exact graph participation, correspondence,
   Signal targets, lowering, and volatile providers.
3. Binding retains the installed conditional set with the operation’s basis
   and graph authority.
4. Execution preflights Query authority, gathers semantic observations through
   Runtime Bridge, and asks Signal for the decision.
5. Query accepts only evidence bound to the exact operation, runtime, graph,
   node, snapshot, attempt, and capability.
6. Changed work continues; unchanged or deferred work cannot mint later
   consequences.

## Small Example

A semantic dependency uses stable aspect meaning:

```rust
let dependency = domain::WorthQuerySemanticTruthDependency::new(
    domain::WorthQueryConditionalGraphReadRole::new("model")?,
    aspect_contract,
    projection_mask,
    aspect_binding,
    domain::WorthQuerySemanticLocality::SourceRecord,
    [worth_relational::facade::schema::RelationalAspectChangeKind::FieldSet],
)?;
```

The dependency retains:

- Foundational aspect key, identity, revision, shape, and absence law
- exact field projection mask
- Relational entity, relation, endpoint, structural, or lifecycle binding
- source-record, partition, or logical-graph locality
- relevant authoritative change kinds
- the installed operation graph-read role

The dependency does not contain a Signal node, partition token, numeric aspect
slot, callback, or runtime identity.

The operation’s graph-read contract must already authorize the dependency’s
semantic projection. Conditional declarations can narrow graph authority; they
cannot create or widen it.

## Real Example

The builder requires every executable semantic dimension:

```rust
let node = domain::WorthQueryPortableConditionalNodeDeclaration::declare(
    "rebuild-face-mesh",
    domain::WorthQueryConditionalNodeRole::Computed,
)
.dependencies([dependency.clone()])
.outputs([domain::WorthQueryConditionalNodeOutput::OperationOutput {
    projection_role: domain::WorthQueryOperationProjectionRole::new("mesh")?,
}])
.required_context([domain::WorthQueryConditionalNodeContext::Snapshot])
.evaluation(
    domain::WorthQueryConditionalEvaluationCondition::aspect_filtered([
        dependency,
    ])?,
    domain::WorthQueryConditionalTrigger::DependencyChange,
)
.comparison(
    domain::WorthQueryComparatorRequirement::FoundationalContractEquivalence,
    domain::WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue,
)
.artifact_policy(
    domain::WorthQueryArtifactReuseEquivalence::DependencyAndOutputEquivalent,
    domain::WorthQueryMaintenancePosture::LazyUntilObserved,
    domain::WorthQueryArtifactPosture::ReusableWhenEquivalent,
)
.output_relationship(
    domain::WorthQueryOutputRelationship::ContributesToOperationOutput,
)
.finish()?;
```

The declaration is canonicalized. Dependency and output order do not create a
new identity. Omitting a semantic dimension is an authoring error; the builder
does not silently choose `Always`, exact comparison, eager evaluation, or an
artifact policy.

## Condition Families

### Aspect-filtered

Use `aspect_filtered(...)` when eligible work depends on a declared semantic
aspect slice changing.

### Delta threshold

Use `delta_threshold(...)` with `WorthQueryDeltaThreshold::new::<Unit>(...)`.
The unit is a marker implementing `WorthQueryQuantityUnit`, and its portable
identity and value family participate in canonical meaning.

```rust
struct Millimeters;

impl domain::WorthQueryQuantityUnit for Millimeters {
    const PORTABLE_IDENTITY: &'static str = "geometry.units.millimeters";
    const VALUE_FAMILY: domain::WorthQueryQuantityValueFamily =
        domain::WorthQueryQuantityValueFamily::Float64;
}
```

Do not compare thresholds in Query or Bridge callbacks. Runtime Bridge lowers
the typed threshold; Signal resolves the decision.

### Temporal

Use `WorthQueryConditionalEvaluationCondition::temporal(...)` with a matching
`WorthQueryConditionalTrigger::Temporal(...)`. Temporal wake authority is
registered during runtime construction.

### On-demand

Define a marker implementing `WorthQueryOnDemandTriggerFamily`, then use the
same typed family through `WorthQueryConditionalTrigger::on_demand::<Family>()`.
The runtime provider answers whether that exact trigger was requested.

### Domain-specific

Define a marker implementing `WorthQueryDomainConditionFamily` and supply
portable typed parameters:

```rust
struct TopologyReady;

impl domain::WorthQueryDomainConditionFamily for TopologyReady {
    const PORTABLE_IDENTITY: &'static str = "geometry.conditions.topology-ready";
}

let condition = domain::WorthQueryConditionalEvaluationCondition::domain_specific::<
    TopologyReady,
>([
    domain::WorthQueryPortableConditionParameter::u64("minimum-shells", 1)?,
])?;
```

The family marker is portable identity. A string-dispatch callback is not.

## Register Runtime Correspondence And Providers

Portable declarations must be paired with exact runtime installations:

```rust
let builder = runtime::WorthQueryRuntime::builder()
    .domain_package(package)?
    .runtime_bridge(bridge)
    .conditional_signal_graph(signal_graph)
    .conditional_node(
        GeometryDomain,
        RebuildFaceMesh,
        GeometryFamily,
        ModelGraph,
        domain::WorthQueryConditionalNodeLocation::operation(
            "rebuild-face-mesh",
        )?,
        dependency_installations,
        bridge_providers,
        RebuildFaceMeshCompute,
    );
```

`conditional_node(...)` binds the exact domain, operation, family, graph,
declaration location, dependency targets, provider set, and compute provider.

Runtime construction rejects:

- missing or extra conditional registrations
- a different marker tuple with the same labels
- a foreign graph or Signal node
- dependency contract, mask, binding, locality, or change-kind drift
- unsupported or ambiguous correspondence
- mixed Signal graphs in one target set
- missing condition, trigger, wake, or comparator provider
- provider identity that does not match the portable declaration
- Signal aspect capacity exhaustion

The registration is volatile. It does not participate in portable package
serialization; its admitted identity is retained by the installed runtime.

## Semantic Correspondence

Runtime Bridge maps one Query semantic dependency to one or more installed
Signal aspect targets. Successful correspondence is either:

- `Exact`
- `DeclaredWidening`

Widening is never implicit. A field-level change cannot silently become a
whole-aspect invalidation. Ambiguous, unsupported, stale, rebind-required, and
failed candidates do not mint an installed correspondence witness.

Many-to-one target allocation is admitted only when precision and ownership
remain explicit. Equal numeric Signal aspects in different graphs or nodes are
unrelated.

## Execute And Observe The Decision

Execution uses the same bound operation journey:

```rust
let bound = workspace
    .operating_world(observation_basis)
    .family(GeometryFamily)
    .bind(&installed_domain, RebuildFaceMesh)?;

let executed = bound.execute(input, &mut workspace).unwrap();

for decision in executed.conditional_provenance() {
    inspect(decision.class(), decision.signal_identity());
}
```

The example unwraps the successful execution only for brevity. Preserve every
`TransitionOutcome` category in production code.

Query performs authority preflight before contacting Runtime Bridge or Signal.
The execution attempt identity includes the exact bound capability, snapshot,
and attempt number.

The outcome classes preserve operational meaning:

- eligible and changed: Query may continue to graph work, executor work, and
  declared consequences
- dependency-unchanged, suppressed, or deferred: compute contact count is zero
  and Query returns typed deferral where required
- computed reverted-clean: compute cost is retained, semantic-change count is
  zero, and no new publication or effect is invented
- failure: owner-specific failure counters and evidence survive Query re-entry

Query consequences consume Signal-minted evidence. Query does not call the
condition provider again and does not infer change from output presence.

## Workflow Stages

Conditional nodes may be attached to an operation or to a named installed
workflow stage through `WorthQueryConditionalNodeLocation`.

A stage decision retains the exact workflow run, stage, predecessor frontier,
basis, snapshot, graph authority, lowering, and Signal decision. Unchanged,
deferred, and reverted-clean stages cannot be advanced by manufacturing a
stage receipt.

## Inspection And Debugging

Inspect:

- portable node identity and canonical operation identity
- dependency contract revision, mask, binding, locality, and change kinds
- installed correspondence precision and target count
- lowering identity and graph instance
- Signal decision class and counters
- Query `conditional_provenance()`
- `conditional_compute_contacts`
- `conditional_semantic_changes`
- graph-provider and executor contacts
- `workspace.rebuild_conditional_execution_index()`
- `bridge.rebuild_correspondence_allocation_index()`

Rebuild reports must show exact parity. Index identity is diagnostic evidence,
not operational authority.

## How It Relates To Other Features

- [Runtime-Installed Domains And Operations](./runtime-installed-domains.md)
  owns the package, provider, binding, execution, and publication journey.
- [Aspects And Authority Lanes](../modeling/aspects-and-authority-lanes.md)
  defines the stable truth meaning carried by dependencies.
- Runtime Bridge owns installed semantic correspondence and condition
  lowering; Signal owns the runtime decision.
- Query effect conditions are a separate downstream-effect contract. They do
  not substitute for node evaluation conditions.

## Anti-Patterns

- Authoring a raw Signal `Aspect`, node ID, or mask in a domain package.
- Treating a mapping stable name as semantic identity.
- Implementing condition selection with string matching.
- Re-evaluating eligibility in Query or in a domain executor.
- Inferring “changed” because compute returned an output.
- Widening field or endpoint changes without a declared precision posture.
- Creating a second Signal graph for conditional Query work.
- Registering a provider without a matching portable declaration.
- Treating conditional nodes as permission to widen graph reads or effects.

## Current Limits

- The current public path installs conditional nodes before runtime
  publication.
- One Query runtime owns one conditional Signal graph through its selected
  Runtime Bridge.
- Portable declarations describe temporal and on-demand meaning, but the host
  must supply the exact runtime wake or trigger provider.
- Later replay, sharing, invalidation-delta, and patch authorities must carry
  conditional provenance when implemented. Their vocabulary does not make
  those paths available now.

## Related Docs

- [Runtime-Installed Domains And Operations](./runtime-installed-domains.md)
- [Aspects And Authority Lanes](../modeling/aspects-and-authority-lanes.md)
- [Downstream Runtime Integration](../foundations/downstream-runtime-integration.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
- [Projection Consumption](../capabilities/projection-consumption.md)
