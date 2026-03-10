# Forge Harness Workflow Certification Design

## Goal

Redesign `forge-harness` from a thin fixture and mutation shell into the shared workflow certification framework for `forge-signal`, `forge-relational`, `forge-bridge`, and future hard-runtime crates.

This design is compatibility-first and additive:

- domain worlds stay local to each crate
- `forge-harness` owns workflow execution, checkpoint orchestration, failure injection, invariant scheduling, artifact capture, differential comparison, and failure reproduction
- `forge-signal` and `forge-relational` are the joint design inputs for this revision
- `forge-bridge` is a target consumer, but not the first proving ground
- the first adapter surface is one `WorkflowCertificationAdapter`, not several speculative sub-traits

## Non-Negotiable Rules

- workflow execution is an explicit state machine
- overlap comparison is capability-aware and profile-aware
- failure bundles are schema-versioned from day one
- known failing workflows are first-class harness concepts
- the redesign is optimized for hostile and adversarial workflow certification, not fixture demos

## Scope Boundary

`forge-harness` owns:

- workflow plans, steps, checkpoints, and state transitions
- failure injection orchestration
- invariant scheduling and aggregation
- artifact taxonomy and capture boundaries
- differential comparison policy
- failure bundle schema and reproduction payloads
- regression target modeling

Runtime crates own:

- crate-local worlds and sessions
- the semantics of each workflow step
- the concrete artifact contents behind harness artifact classes
- crate-local invariants and their implementation
- honest declarations of what can and cannot be compared

## Compatibility Strategy

The existing harness surface remains valid in this revision:

- `ScenarioFixture`
- `MutationBatch`
- `ExecutionRequest`
- current capture and comparison shell

The new workflow-certification layer sits beside that surface. Existing users keep working while `forge-signal` and `forge-relational` migrate serious workflows into the new runner.

## Core Workflow Model

The harness adds these first-class concepts:

- `WorkflowPlan`
- `WorkflowStep`
- `WorkflowCheckpoint`
- `WorkflowSession`
- `WorkflowState`
- `FailureInjectionPoint`
- `ArtifactBundle`
- `InvariantCheck`
- `InvariantReport`
- `DifferentialComparison`
- `DifferentialOutcome`
- `FailureBundle`
- `FailureBundleVersion`
- `RegressionTarget`

### Workflow State Machine

Required states:

- `Initialized`
- `StepApplied`
- `Checkpointed`
- `Inspected`
- `Failed`
- `Completed`

Allowed transitions:

- `Initialized -> StepApplied`
- `Initialized -> Failed`
- `StepApplied -> Checkpointed`
- `StepApplied -> Inspected`
- `StepApplied -> Failed`
- `Checkpointed -> Inspected`
- `Checkpointed -> Failed`
- `Inspected -> StepApplied`
- `Inspected -> Completed`
- `Inspected -> Failed`

Forbidden examples:

- `Completed -> StepApplied`
- `Failed -> Completed`
- `Initialized -> Completed`
- `Checkpointed -> StepApplied`

The runner, not the adapter, enforces these transitions. This matters because invalid-transition bugs are certification failures even when the final output appears correct.

## WorkflowCertificationAdapter Contract

Start with one additive adapter:

- `WorkflowCertificationAdapter`

It owns these responsibilities:

- initialize a crate-local workflow world or session
- execute one workflow step
- create a harness-requested checkpoint
- capture supported artifacts at requested boundaries
- run crate-local invariants at requested boundaries
- declare comparison and capture capabilities honestly
- serialize reproduction data for failures

This stays unified until one serious `forge-signal` migration and one serious `forge-relational` migration prove stable trait seams. Splitting earlier would freeze the wrong fault lines.

### Why One Adapter First

The current uncertainty is not whether the harness needs world setup, artifact capture, or invariants. It is where the stable boundary between them will land after hostile workflow migrations.

Prematurely freezing separate world, artifact, and invariant traits would create migration theater:

- more trait plumbing
- weaker evolution room
- early false boundaries that both runtimes will need to work around

One additive adapter keeps the core explicit without pretending the stable seams are already known.

## Checkpoint Semantics

Checkpoint orchestration is harness-owned.

The harness decides:

- when checkpoints are requested
- which workflow boundary they correspond to
- which captured artifacts are associated with the checkpoint
- which invariants run after checkpoint creation

The adapter declares which checkpoint semantics it truthfully supports, such as:

- snapshot restore
- durable recovery
- branch-head snapshot
- replay anchor
- adapter-defined semantics with an explicit label

The harness must not assume checkpoint equivalence across runtimes. A `forge-signal` branch snapshot and a `forge-relational` durable recovery checkpoint are both checkpoints, but they are not the same guarantee.

## Artifact Taxonomy

Artifacts are explicit and typed. The runner knows the class and surface even when the payload is crate-local.

Required artifact classes:

- truth artifacts
- observability artifacts
- forensic artifacts
- performance artifacts

Required artifact surfaces:

- snapshot-visible truth
- branch/head state
- replay/recovery truth state
- diagnostics
- patch/change surfaces
- step trace
- checkpoint trace
- failure injection trace
- reproduction metadata
- complexity counters
- budget outcomes

The adapter may add crate-local content inside those surfaces, but the harness taxonomy must stay explicit and durable.

## Invariant Scheduling Model

Invariant scheduling is harness-owned and boundary-based.

The harness specifies:

- which invariant checks exist in the plan
- at which workflow boundaries they run
- whether a failure is required or advisory

The adapter executes the crate-local invariant logic and returns `InvariantReport`s.

Default certification boundaries:

- after `StepApplied`
- after `Checkpointed`
- after `Inspected`
- on `Failed`
- on `Completed`

Not every workflow uses every boundary, but the scheduling model is always explicit.

## Capability Declaration Rules

Capability declarations are machine-readable and fail closed.

Required declaration dimensions:

- supported artifact surfaces
- supported checkpoint semantics
- supported replay and recovery comparison surfaces
- supported differential matrices
- unsupported or partial comparisons with explicit reason
- profile-conditional guarantees
- optional budget artifact support

The harness must never silently compare more than the adapter promises.

If an adapter supports replay comparison only for:

- `profile=forensic`
- `policy=strict`
- `executor=serial`

then the harness compares only that overlap under exactly that active profile combination.

## Overlap Comparison Semantics

Differential comparison is defined as:

- compare only the overlap guaranteed by both participating capabilities
- compare only under the active runtime profile, policy, and executor combination
- emit explicit skipped surfaces and reasons
- fail closed when the requested comparison exceeds the guaranteed overlap

Examples:

- `forge-signal` serial vs staged-parallel comparison may guarantee branch-head truth, replay summaries, and lineage slices, but not scheduler-local diagnostics ordering
- `forge-relational` truth vs recovery comparison may guarantee recovered snapshot-visible truth and replay-derived audit envelopes, but not identical local storage counters

This overlap rule is the difference between truthful certification and overclaiming.

## Failure Bundle Schema

Failure bundles are standardized and schema-versioned immediately.

`FailureBundleVersion::V1` includes:

- bundle schema version
- crate name
- domain name
- workflow name
- scenario/world name
- seed
- runtime profile, policy, and executor
- workflow step trace
- checkpoint trace
- failure injection point
- invariant failures
- artifact diffs over guaranteed overlap surfaces
- reproduction metadata

Failure bundles are harness-owned records. Adapters provide reproduction payloads, but the bundle envelope remains stable across runtimes.

## Regression Target Model

Regression workflows are first-class harness concepts:

- `KnownFailing`
- `ExpectedFailure`
- `Quarantined`

Each regression target may include:

- issue key
- human summary
- reproduction hint
- prior known failure metadata

This replaces ad hoc ignored tests with a harness-visible regression contract.

Concrete example:

- the ignored `forge-relational` replay drift workflow should become a harness `RegressionTarget::KnownFailing`
- the harness should keep running it in a regression lane and preserve its failure bundle instead of hiding it behind `#[ignore]`

## Execution Budget Hooks

Budget assertions are optional, capability-driven certification features.

The harness must be able to express expectations such as:

- no full-state clone
- no full visibility scan
- bounded recovery replay after checkpoint
- equivalent logical workflow under multiple execution variants

If an adapter cannot emit complexity counters or budget outcomes, it must say so explicitly. Budget certification cannot be inferred from silence.

## Concrete `forge-signal` Example

Primary hostile proving workflow:

- file: `crates/forge-signal/src/tests/domains/fintech/workflows.rs`
- workflow: hostile branch, replay, restore, correction, and audit

Harness interpretation:

1. initialize the crate-local fintech world
2. apply a calm-regime seed step
3. checkpoint the main branch snapshot
4. switch to analysis branch and apply a hostile shock step
5. checkpoint the analysis branch snapshot
6. inject a failed correction transaction
7. inspect replay frames and rollback evidence
8. restore analysis snapshot and certify branch-local replay overlap
9. open correction branch, apply new regime, and inspect lineage overlap
10. restore main snapshot and compare branch/head truth over the guaranteed overlap

Required artifact focus for this migration:

- branch/head state
- snapshot-visible truth
- replay summaries
- step trace
- checkpoint trace
- failure injection trace
- reproduction metadata

Required invariant focus:

- branch-local replay stays branch-local
- rollback evidence is preserved
- restored analysis matches pre-failure analysis truth
- correction lineage contains expected replacement and refresh events

Differential example:

- compare serial versus parallel hostile certification only over surfaces `forge-signal` explicitly guarantees under the selected executor profile

## Concrete `forge-relational` Example

Primary proving workflows:

- file: `crates/forge-relational/src/tests/domains/fintech/workflows.rs`
- file: `crates/forge-relational/src/tests/runtime_observability.rs`

Harness interpretation:

1. initialize the crate-local fintech world via `setup_world()`
2. open an analysis branch
3. apply market shock and trade correction steps
4. certify branch-head truth and snapshot-visible truth
5. request a durable checkpoint
6. apply a post-checkpoint correction
7. build a recovery plan and recover a fresh runtime
8. compare recovered truth only over the guaranteed recovery overlap
9. preserve replay drift workflows as explicit regression targets

Required artifact focus for this migration:

- snapshot-visible truth
- branch/head state
- replay/recovery truth state
- diagnostics
- patch/change surfaces
- checkpoint trace
- reproduction metadata

Required invariant focus:

- recovered portfolio probe remains queryable
- branch head after recovery matches latest committed branch head over guaranteed surfaces
- replay or recovery drift is surfaced as a regression artifact, not silently ignored

Concrete regression target:

- the ignored replay drift workflow in `crates/forge-relational/src/tests/domains/fintech/workflows.rs` becomes a first-class known-failing certification target

## Rollout Plan

### Phase 1

Create and freeze this design document with:

- workflow state machine
- `WorkflowCertificationAdapter` contract
- checkpoint semantics
- artifact taxonomy
- overlap comparison semantics
- invariant scheduling model
- failure bundle schema and versioning
- regression target model
- capability declaration rules

### Phase 2

Add the new workflow-certification layer additively in `forge-harness`.

Requirements:

- no breakage of current users
- old fixture and mutation surfaces remain available
- new layer is explicit, not hidden behind compatibility wrappers

### Phase 3

Prove the model with one serious `forge-signal` migration:

- one serial adversarial fintech workflow
- one serial-vs-parallel hostile differential workflow
- branch, snapshot, replay, and lineage overlap certification through the new runner

Use that migration to harden:

- step execution model
- checkpoint model
- failure injection
- artifact bundle shape
- invariant scheduling
- failure bundle output

### Phase 4

Prove the same runner concepts with one serious `forge-relational` migration:

- fintech world via `setup_world()`
- branch, correction, audit workflow
- checkpoint and recovery workflow
- durable append or recovery failure injection

Success criterion:

- the same harness runner concepts work for both crates
- crate-local worlds remain local
- no domain hacks leak into `forge-harness`

### Phase 5

Split the adapter only if the migrations reveal stable and useful fault lines.

Possible later seams:

- world or session execution
- artifact capture and comparison
- invariant scheduling

Do not split for conceptual tidiness alone.

### Phase 6

Add explicit multi-runtime workflow certification after `forge-signal` and `forge-relational` prove the model.

This phase handles:

- cross-runtime workflows
- producer/consumer drift
- replay/correction across boundaries
- overlap comparison across multiple systems

### Phase 7

Standardize CI certification lanes once at least signal and relational have migrated:

- required PR lanes
- scheduled churn and stress lanes
- capability-aware comparison lanes
- failure bundle retention and publishing
- regression-target lanes

## Success Criteria

This redesign is successful when:

- `forge-harness` owns workflow certification semantics across runtimes
- `forge-signal` and `forge-relational` both use the same runner concepts for serious hostile workflows
- overlap comparison is truthful and fail-closed
- regression targets stop living as ignored tests
- failure bundles become durable cross-runtime debugging assets
- domain worlds stay crate-local with no domain leakage into harness core
