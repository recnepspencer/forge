# Milestone 3.9: Execution-Plan Lowering, Equivalence, and Frame-Cost Surfaces

**Status:** Planned. Milestone 3.8.1 is the completed prerequisite. Milestone
3.10 must not begin until every closure gate in this specification is green.

## Purpose

Milestone 3.9 makes a lowered execution plan the only operational input to UI
frame execution. Canonical declaration, graph, capability, Query-binding,
host-support, and committed-allocation truth may participate while a candidate
plan is lowered. They must not be rediscovered by an executor during an
ordinary frame.

This is a consolidation and authority cutover, not a greenfield planner. The
workspace already contains committed-allocation lowering input, typed runtime
handles, plan topology, lane-specific plans, plan equivalence, reload and frame
counters, Foundational bridges, and atomic replacement infrastructure. Those
surfaces currently form overlapping partial paths. This milestone must leave
one phase chain, one active plan owner, and no operational predecessor lane.

## Why this milestone belongs here

Milestone 3.8 committed allocation truth and Milestone 3.8.1 closed the active
application, replacement, Query, host-session, framework-turn, and inspection
authority boundaries. Milestone 3.9 can therefore lower from exact committed
truth without inventing a second application authority. Milestone 3.10 will
add mounted receipts and a stricter host contract; it needs a coherent active
plan to consume first. Deferring this cutover would force mounted receipts to
depend on caller-assembled or digest-only plan state.

## Architectural inputs reconciled

This specification reconciles the current public facade and runtime APIs with
the active Worth UI orientation, DSL vision, AI-diagnostics architecture,
Milestones 1 through 3.8.1, and Worth Query's runtime mental model. The
load-bearing inherited contracts are:

- canonical declaration, graph, obligation, measurement, allocation, and
  committed-receipt authority remain distinct inputs; the execution plan is
  derived frame truth and cannot replace any of them;
- the DSL authors semantic lanes and lowers once; it never authors runtime
  handles, lane strategy, or host-specific execution contacts;
- Query crosses the UI boundary as installed, runtime-affine authority and one
  sealed consumed-projection outcome, not a tuple of copied status, basis,
  facts, labels, or digests;
- Foundational native aspect values remain native through Query consumption and
  UI lowering; plan construction may index them but may not round-trip them
  through JSON, strings, widened numerics, or UI-local mirrors;
- inspection and AI diagnostics observe the same receipts and indexes as human
  tools and may explain lowering, equivalence, activation, and frame cost, but
  may not mint or promote plan authority; and
- egui is an adapter substrate. Canonical execution-plan meaning is
  host-neutral even when the first production executor is egui.

## Governing adversarial constraint

> Under continuous frames and replacement churn, execution work must remain
> bounded by the active lowered plan and the admitted semantic delta.
> Equivalent replacements produce no swap, non-equivalent replacements are
> never suppressed, stale plans and handles open no authority, and executors
> never rediscover decisions that were available during lowering.

This constraint is more important than any proposed type or module name. If a
phase discovers that the cheapest implementation violates it, the
implementation must change rather than weakening the constraint.

## Current API disposition

The following inventory is the starting architecture, not an endorsement that
every current visibility or name survives.

| Current surface | Current role | Milestone disposition |
| --- | --- | --- |
| `UiCommittedAllocationLoweringInput` | Frozen 3.8-to-3.9 receipt/report/transaction handoff with freshness admission | Preserve as the allocation constituent of the sole sealed lowering authority. Do not let it become a standalone parallel authority. |
| `WorthUiExecutionPlanInput`, `WorthUiExecutionPlanInputPreparer`, and `WorthUiExecutionPlanInputWitness` | Reconstruct node, topology, reconciliation, Query-rebind, hook, and digest inputs from pending activation | Consolidate into the canonical lowerer or remove. No duplicated reconstruction, witness-only assembly, or predecessor input path may remain. |
| `WorthUiPlanLoweringBasis`, `WorthUiPlanLoweringContext`, node inputs, and lowering counters/denials | Describe provisional lowering inputs and their evidence | Retain only facts that belong in the canonical sealed input; replace raw digest identity with exact authority where identity matters. |
| `WorthUiInstalledQueryView`, `WorthUiQueryProjectionOutcome`, `WorthUiQueryAuthorityHandle`, Query prerequisite/measurement settlements, and native aspect contracts | Carry installed Query authority, consumed projection authority, and binding-owned native facts into Worth UI | Preserve the sealed binding-owned handoff. Plan rows may retain compact admitted references, but may not split the outcome into UI-owned basis/status/fact/digest fields or recreate Query lifecycle authority. |
| `WorthUiPlanningLaneInput` and `WorthUiAllocationPlanning` | Admit graph/measurement locality and produce upstream allocation candidates | Preserve as the 3.8 allocation-planning phase. Execution-plan lowering consumes the committed receipt it produces, never re-enters allocation planning or accepts planning input as execution truth. |
| `WorthUiRuntimeHandleAllocation` and typed component, command, token, child-range, view-binding, lane, and state-slot handles | Allocate `(plan_index, plan_generation)` handles from committed allocation | Preserve the typed families and sealed construction. Cut over to direct indexed resolution, exact active-plan authority, stable reuse where proven, and explicit exhaustion denial. |
| `WorthUiExecutionLaneInput`, public `allocate_runtime_handles*`, `admit_execution_lanes`, and `assemble_execution_plan_topology*` methods | Let callers advance individual allocation, lane-admission, handle, and topology steps | Internalize them behind the sealed candidate lowerer. Preserve explicit host-support admission as a named phase, but do not expose independently composable plan constituents to product consumers. |
| `WorthUiExecutionPlan`, topology, lane partitions, lookup index, child ranges, region structure, and render-resource refs | Represent a shared topology plan | Make this an internal constituent of one sealed candidate/active plan bundle. It must not be independently submitted to execution. |
| `WorthUiEguiBoundaryInput`, `WorthUiEguiBoundaryContact`, and `WorthUiEguiPlanBoundary` | Carry egui-named contact meaning inside runtime plan topology | Replace with a host-neutral lowered contact contract or confine it to the egui adapter. The canonical runtime plan may not make egui the owner of UI meaning. |
| `WorthUiOrdinaryLanePlan`, `WorthUiVirtualizedDataPlan`, `WorthUiCanvasSpatialPlan`, and `WorthUiHudPlan` | Store lane-ready rows, indexes, counters, support digests, and certifications | Preserve useful lane-ready representations as sealed constituents of the plan bundle. Remove public/caller assembly and arbitrary cross-plan composition. |
| `WorthUiRuntime::prepare_*_plan` and `WorthUiFrameworkTurnExecution::execute_*_frame(&plan, ...)` | Let callers prepare and pass lane plan objects into executors | Replace with active-session or active-runtime execution capabilities that borrow the one active plan generation. A caller must not inject a stale or foreign plan. |
| `WorthUiActiveExecutionPlan` | Record only a derived active-plan digest | Replace with ownership of the actual sealed active plan bundle plus exact generation/session authority. A digest remains evidence, never the plan or its authority. |
| `WorthUiExecutionPlanEquivalenceBasis`, digestor, counters, and `Reusable`/`RebuildRequired` | Compare whole-plan fingerprints that currently include generation-bearing allocation data | Separate executable semantic equivalence from activation freshness. Preserve fast fingerprints as indexes/checksums, add collision-safe proof, and report a decision rich enough to drive no-op, reuse, rebuild, or denial. |
| `WorthUiPlanSwapReceipt`, committed-allocation preflight/publication, last-valid state, frame scheduler, and invalidation ledgers | Publish a replacement atomically after validation | Extend the existing transaction to publish the complete plan bundle and plan decision with the application generation. Denial must preserve the complete predecessor. |
| `WorthUiReloadCounterBoundary`, `WorthUiSteadyFrameCounterBoundary`, lane-cost certification, diagnostics projection, and Foundational bridges | Expose provisional cost and claim surfaces | Reuse and consolidate them around the real lower/activate/execute lifecycle. Delete parallel or synthetic counter truth. Foundational remains a post-boundary claim projection. |
| `WorthUiExecutionPlanInspection`, provenance, lane/node inspection, `compare_execution_plans`, and AI inspection harness | Inspect caller-held plan objects and project diagnostics | Preserve structured explanation but bind it to sealed candidate or active-plan receipts. Inspection may compare evidence; it may not become another plan-construction or activation path. |
| `WorthUiActiveApplicationSession` and prepared/lowered/staged/cutover replacement types | Own the active application lifecycle and exact candidate session/generation binding | Extend this existing owner to hold or exclusively govern active-plan execution. Plan publication and application-generation publication must be one cutover. |
| `prepare_replacement` artifact-digest early `NoOp` | Suppress a replacement before allocation and plan lowering | Remove as an operational no-op decision. No-op is legal only after complete canonical plan equivalence and exact authority checks. |

## Target authority and phase chain

The exact final type names may change, but the ownership chain may not:

```text
prepared candidate application authority
  + exact candidate graph and capability/Query posture
  + admitted host/lane support
  + committed allocation lowering input
    -> sealed execution-plan lowering authority
    -> sealed candidate plan bundle
       { topology, typed handles, lane-ready plans, equivalence proof, counters }
    -> typed plan decision
       { initial activation | semantic no-op | bounded replacement | denial }
    -> atomic application + runtime + active-plan publication
    -> active-session execution capability
    -> lane execution receipts and optional diagnostic/Foundational projection
```

Only the sealed lowering authority may create a candidate plan bundle. Only
the activation transaction may promote one to active truth. Only an active
session capability may lend executable access to it. Inspection may observe
the active or candidate plan through typed receipts but may not promote it.

## Binding assumptions

These assumptions are explicit milestone law. Evidence that invalidates one
requires a visible specification amendment before implementation proceeds.

1. The active application session remains the one ordinary owner of a running
   application generation, host session, runtime, Query posture, graph
   authority, and active execution plan.
2. A plan is derived, reconstructable state. It may cache decisions and indexes
   but may not become the exclusive source of authored or graph meaning.
3. Launch may lower the complete initial plan. Replacement lowering must reuse
   admitted predecessor regions and be bounded by the affected semantic
   closure; an unrelated large graph may not turn a one-node change into a
   full-plan walk.
4. Ordinary frame execution receives no declaration artifact, graph snapshot,
   allocation planner, string registry, or candidate authority.
5. Executable semantic equivalence and activation authority are different
   questions. Incidental frame epochs or newly minted candidate generations do
   not by themselves make equivalent execution meaning different; stale or
   foreign authority still cannot activate or execute.
6. A digest is a lookup aid and diagnostic fact, never sufficient proof of
   identity, freshness, equivalence, or authorization.
7. Capability posture, Query-binding posture, host support, extension-hook
   admission, render-resource meaning, and lane policy participate in
   equivalence whenever changing them can change execution.
8. Diagnostic richness, counter materialization, and inspection formatting do
   not participate in operational equivalence unless they alter execution.
9. The four current execution regimes--ordinary, virtualized data,
   canvas/spatial, and realtime overlay--are the closed set for this milestone.
   Adding a regime requires an explicit plan constituent and cost contract, not
   a catch-all fallback.
10. Typed handles are sealed compact locators into one exact plan generation.
    They are not hashes, names, global registry keys, or transferable
    capabilities between otherwise identical sessions.
11. Query-free and headless applications use the same lifecycle with absent
    typed constituents. They do not require dummy Query or host adapters.
12. Foundational consumes certified boundary evidence. It does not define plan
    meaning, choose a strategy, or sit on the frame hot path.
13. Candidate denial, failed lowering, failed equivalence proof, and failed
    publication leave the predecessor application, plan, handles, counters,
    frame scheduler, and inspection authority coherent and usable.
14. Plan inspection and rich diagnostics are explicitly requested cold-path
    work. Ordinary execution emits compact receipts or references and does not
    materialize reports by default.
15. An application is not frame-executable until its initial committed
    allocation has lowered into and activated a real plan bundle. If host
    measurement makes that bootstrap asynchronous, the API exposes a typed
    non-executable planning posture; it may not synthesize an empty or
    digest-only "active plan."
16. Query projection crosses into Worth UI as one binding-owned sealed outcome.
    The plan may retain an admitted compact reference to that authority, but it
    may not decompose it into independently trusted UI-local basis, receipt,
    fact, support, source-label, or digest fields.
17. Query-installed domain handles and retained live resources are affine to
    the Query runtime and installation generation that minted them. Worth UI
    coordinates their admitted use and exact-once release; it does not become
    the activation, subscription, recovery, or disposal authority.
18. Native aspect values and struct values remain exact Foundational types at
    the Query-to-UI and UI-to-plan boundaries. Text/JSON conversion is allowed
    only at an explicitly declared authoring or presentation boundary, never as
    operational plan meaning.
19. Query-owned support visibility and runtime admission remain different
    questions. A visible installed capability is not plan-executable until the
    binding boundary supplies the required admitted authority and support
    evidence.

## Scale and cost model

The implementation must name its scale terms and keep them visible in counters
and adversarial tests:

- `P`: rows and edges in the active plan;
- `A`: rows and edges in the admitted affected closure for a replacement;
- `T`: rows intentionally touched by one frame target;
- `L`: active execution-lane count, bounded by the closed lane set;
- `D`: diagnostic or inspection rows explicitly requested for materialization.

Required envelopes:

- initial lowering is `O(P)` time and `O(P)` retained plan memory;
- replacement lowering and equivalence are `O(A)` plus the changed output,
  after existing impact and allocation locality have admitted `A`;
- a handle lookup is direct indexed `O(1)` work with exact generation/family
  validation--binary searching a copied row index is not the target design;
- a steady frame is `O(T)` and independent of unrelated plan, graph, registry,
  and declaration width;
- ordinary steady frames perform zero source parses, artifact validations,
  registry string lookups, broad graph/plan/registry scans, rich diagnostic
  materializations, and general-purpose heap allocations;
- candidate construction may allocate off the active frame path, but a small
  replacement may not rebuild every handle and lane vector merely for
  implementation convenience;
- diagnostic projection is `O(D)` and runs only when requested;
- denial work stops at the earliest boundary that has enough evidence and does
  not continue into later plan phases.

If exact structural equivalence would otherwise require a whole-plan scan, the
implementation must carry exact predecessor-region proof through lowering and
compare only changed regions. Merkle-style fingerprints may narrow comparison,
but a hash collision must not be able to authorize a no-op.

## Test-program and iteration-time constitution

Test topology is the first milestone priority because a correct architecture
that makes every development iteration recompile fixture workspaces is not a
scalable implementation process.

### Hard structural budgets

1. Keep exactly the existing three approved `trybuild` session owners. Add no
   compile-contract target or session.
2. Freeze executable compile-contract ceilings independently for all three
   sessions: 133 Worth UI cases, 62 certification cases, and 3 host-contract
   cases. A genuinely new compile-time invariant must replace or consolidate an
   existing representative in the same proof class and session. Any budget
   increase requires a reviewed spec/roadmap amendment and new cold/warm timing
   evidence; a phase implementation may not silently raise it.
3. Freeze physical compile-fixture inventory ceilings at 284 Worth UI cases, 62
   certification cases, and 3 host-contract cases. Inventory should shrink when
   a topology audit replaces a fixture; moving a case out of the executable CSV
   does not authorize leaving a new dormant fixture behind.
4. Keep the configured integration-target sets and per-package ceilings. New
   tests enter existing compiled targets as modules or replace weaker proof.
5. No ordinary test may launch nested Cargo, create a temporary crate/workspace,
   compile generated Rust, or invoke `rustc` directly.
6. No new per-phase, per-type, or milestone-numbered compile harness is allowed.
   Compile-fail proof is reserved for a public type-system impossibility that
   cannot be established by an existing compile representative or a mechanical
   source/topology audit.
7. No new `.stderr` snapshot or compile fixture may be added without a row in
   the reconciliation inventory that names its proof class and the case it
   replaces. Net executable case count remains non-increasing.
8. No test-only production constructor, enum variant, public escape hatch, or
   fake authority path may be added to provoke a denial. Test the real public
   or crate boundary, or inject data below the authority decision inside a
   test-only support module.
9. Production and test Rust files remain at or below the workspace 400-line
   cap. Test support obeys the same responsibility and naming laws as
   production code.

### Preferred proof order

For every invariant, select the earliest adequate and cheapest proof:

1. ordinary unit test for a local algorithm or denial;
2. ordinary compiled-once integration/scenario test for lifecycle behavior;
3. source/topology audit for visibility, dependency, or module-placement law;
4. an existing compile-contract equivalence class;
5. only then, a replacement compile fixture under the frozen budget.

One reusable scenario authority should create canonical applications, admitted
graphs, committed allocation, host support, and replacement deltas through
production APIs. Variants must be typed scenario operations, not cloned setup
forests. Shared support must not mint sealed internals, skip freshness checks,
or turn into a god fixture that hides which authority a test is exercising.

### Required timing evidence

Structural budgets are the mechanically enforced gate; wall-clock time is
recorded evidence rather than a flaky per-run assertion. At milestone open and
close, record the median of three isolated runs for:

- a targeted warm ordinary-test iteration;
- the warm fast proof lane;
- the cold and warm compile-contract lane; and
- the full Worth UI proof lane.

The closing implementation must not regress a comparable median by more than
10 percent without removing the regression before closure or recording a
reviewed budget amendment with a concrete reason. Compiler sessions, package
targets, executed compile cases, and nested-Cargo count remain hard gates even
when timing noise is present.

## Cross-phase engineering rules

- Each phase ships a usable vertical behavior or a mechanical prerequisite for
  the immediately following slice. No phase may leave a second operational
  plan path "temporarily" available to consumers.
- Lowerers and transactions read as named semantic steps. Avoid coordinator
  files that also validate, allocate, compare, publish, and format diagnostics.
- Public plan types are minimized. Consumers should ask the active session to
  perform a typed operation, not learn how to assemble runtime internals.
- Full desired candidate truth and a replacement delta remain distinct. A
  delta is never treated as complete truth; a complete candidate is never
  rebuilt from an unbounded scan when exact predecessor proof is available.
- Denials name the failed phase and carry compact evidence/counters. Booleans
  such as `is_valid`, `same_plan`, or `supported` do not cross authority
  boundaries without a typed receipt or classification.
- Every new counter has one production increment site, a named semantic unit,
  a declared ordinary/reconstructive boundary, and an adversarial test that
  would fail if it drifted.
- Query-backed proof should reuse Worth Query Consumer Kit scenarios and
  authority checks when they establish Query semantics. Worth UI adds only the
  UI-specific registration, graph, allocation, plan, and execution assertions;
  it does not copy Query's test runtime or proof machinery locally.
- Tests must be capable of falsifying authority, equivalence, locality, cost,
  and atomicity claims. Passing the happy path is not milestone evidence.

## Phase 1: Freeze the Fast Test Topology

**Vertical outcome:** Every later 3.9 slice can add hostile behavioral proof
without adding a compiler session, integration target, fixture workspace, or
duplicated application setup forest.

**Relevant subsystems and APIs**

- `scripts/ci/check_worth_ui_test_topology.py`
- `scripts/ci/worth_ui_test_topology_budget.json`
- `scripts/ci/run_worth_ui_test_lane.py`
- compile inventory/execution reconciliation CSVs
- `worth-ui-test-support` and the existing compiled-once certification scenario
  modules

**Deliverables**

- Extend the topology checker so every hard rule in the test-program
  constitution is mechanically enforced, including separate 133/62/3
  executable ceilings, 284/62/3 physical fixture ceilings, and the absence of
  generated compilation in ordinary tests.
- Record the opening structural and timing baseline using the prescribed lanes.
- Establish one narrow 3.9 scenario authority that creates application,
  committed-allocation, host-support, and replacement-delta inputs through
  production APIs. Split responsibilities rather than creating a universal
  fixture bag.
- Reconcile repeated current execution-plan/lane setup into typed scenario
  operations only where that reduces compilation or duplicated authority.

**Warnings**

- Do not rewrite working tests merely to make them look uniform.
- Do not move production algorithms into test support or let a scenario builder
  mint private authority.
- Do not gate CI directly on noisy wall-clock duration; gate topology and record
  comparable timing evidence.

**Test requirements**

- Prove every retained pre-cutover proof maps to an equal or stronger ordinary,
  topology, or compile-contract proof; the migration may not silently drop a
  denial class.
- Seed checker fixtures for an extra `trybuild` session, nested Cargo call,
  generated compile fixture, unexpected integration target, and compile-case
  budget increase; each must fail with a specific violation.
- Prove two scenario variants with equivalent semantic input produce equivalent
  setup authority without recompiling a second fixture crate.
- Prove a scenario operation cannot bypass freshness, session, or graph
  authority even when all visible digest/count fields match.

**Engineering decisions**

- The existing three `trybuild` owners, current target sets, and 133/62/3
  per-session executable cases are ceilings, not targets to fill.
- Runtime behavior is proven primarily in ordinary compiled-once tests.
- Source and topology laws use mechanical audits instead of compiler fixtures
  when the compiler is not the actual authority being tested.

**Open questions**

- Which existing narrow test-support module is the cleanest owner for the 3.9
  scenario entry point? Choose by responsibility and dependency direction, not
  by convenience.

## Phase 2: Seal One Canonical Lowering Authority

**Vertical outcome:** A candidate application generation and its exact graph,
capability/Query posture, host support, and committed allocation cross one
admission boundary and become the only legal input to execution-plan lowering.

**Relevant subsystems and APIs**

- `WorthUiActiveApplicationSession`, prepared/lowered/staged replacement types,
  and prepared application generation identity
- `WorthUiPendingActivation`
- `UiCommittedAllocationLoweringInput`
- `WorthUiExecutionPlanInput`, its preparer/witness, basis, context, node inputs,
  hooks, counters, and denials
- lane admission and host-session support
- binding-owned installed Query and consumed-projection authority

**Deliverables**

- Introduce one sealed lowering authority that binds exact candidate application
  generation, graph authority, capability snapshot/Query posture, host-session
  support, and committed allocation lineage before plan construction.
- Route launch and replacement through the same phase shape; their authority
  sources may differ, but plan construction may not.
- Fold every still-required fact from the provisional execution-plan input into
  this authority or a named derived constituent.
- Admit Query-bound constituents from the sealed binding-owned projection and
  installed-authority edge. Do not accept independently supplied Query basis,
  status, result-shape, source-label, fact-bag, or digest fields.
- Remove duplicated topology/Query/hook reconstruction and the parallel witness
  path. Delete or internalize predecessor APIs once their consumers move.
- Remove the artifact-digest early replacement `NoOp`; classification is deferred
  until the complete candidate plan is available.

**Warnings**

- Do not solve exact authority by cloning an entire graph or declaration tree
  into the lowerer.
- Do not encode candidate identity as another hand-folded digest.
- Do not permit a default host-support profile to participate invisibly; support
  that changes execution is explicit admitted input.

**Test requirements**

- Equivalent declaration/graph/allocation truth presented in different lawful
  input order lowers to equivalent canonical input facts.
- Equal artifact, capability, and allocation digests from a foreign graph or
  active session deny before node or handle construction.
- Missing Query posture for a Query-bound node, and unexpected Query posture for
  a Query-free node, produce typed denials without partial candidate state.
- A split or copied Query tuple with matching visible values but foreign
  installed-runtime authority denies before plan-row construction.
- A candidate formerly accepted by the early digest no-op path must continue to
  lowering when host support, Query posture, or allocation meaning differs.

**Engineering decisions**

- Exact sealed identities authorize; raw digests only accelerate comparison or
  support diagnostics.
- Committed allocation is a constituent of candidate plan authority, not a
  caller-provided sibling argument.
- One canonical ordered representation is established during admission so
  downstream phases do not sort or reconstruct it repeatedly.

**Open questions**

- Whether the sealed authority borrows cold canonical artifacts during lowering
  or owns compact admitted projections should be decided from measured lifetime
  and allocation cost. It must never expose those artifacts to execution.

## Phase 3: Activate and Execute the Minimal Ordinary Plan Slice

**Vertical outcome:** A root shell and one component lower from canonical truth
into a sealed plan bundle, activate atomically, and execute only through the
active application session.

**Relevant subsystems and APIs**

- `WorthUiExecutionPlan`, topology, lane partitions, and lookup index
- `WorthUiOrdinaryLanePlan` and ordinary plan builder/executor
- `WorthUiActiveExecutionPlan` and active runtime state
- `WorthUiFrameworkTurnExecution`
- committed-allocation activation preflight/publication and plan-swap receipt

**Deliverables**

- Define the sealed candidate/active plan bundle and make shared topology plus
  the ordinary root/component lane its first complete constituent.
- Replace digest-only active-plan state with ownership of the actual active
  bundle and exact application/session generation binding.
- Add an active-session execution capability that borrows the active plan.
  Remove caller-supplied ordinary plan execution from the product facade.
- Make launch publish a real initial plan rather than a digest synthesized from
  artifact and capability digests. If committed allocation is not yet
  available, retain a typed non-executable bootstrap posture until first plan
  activation.
- Emit compact ordinary execution receipts without per-frame general-purpose
  heap allocation.

**Warnings**

- The active plan bundle is derived runtime state, not a second application or
  graph authority.
- Do not leave public plan-preparation methods as an alternate product path.
- A root-shell target may intentionally touch its admitted ordinary subtree; it
  may not scan unrelated lanes or cold topology.

**Test requirements**

- Two lawful initial applications with equivalent execution meaning produce
  equivalent minimal ordinary bundles and equivalent frame receipts.
- A stale plan object, foreign session, forged visible digest, or predecessor
  component handle cannot execute through the active session.
- Attempt frame execution before initial committed allocation/plan activation;
  the typed bootstrap posture must deny without fabricating an empty plan.
- Inflate unrelated non-ordinary topology by two orders of magnitude and prove
  component-target work/counters do not grow.
- Prove an ordinary frame performs zero forbidden cold work and zero
  general-purpose heap allocations while still touching the intended component.

**Engineering decisions**

- The session lends execution; it does not export an owned active plan.
- Lane-ready rows are built during lowering/activation, never on first use in a
  frame.
- Frame receipts carry compact indexes/ranges or refs rather than allocating
  vectors of touched handles.

**Open questions**

- The exact borrow shape between framework-turn execution and the active plan
  should be chosen to preserve transactional source-turn semantics without
  widening plan visibility.

## Phase 4: Complete the Compact Handle Arena

**Vertical outcome:** Every current handle family resolves directly against one
exact active plan generation and cannot cross family, generation, or session
boundaries.

**Relevant subsystems and APIs**

- runtime handle allocation, basis, receipt, family widths, generation, and
  counters
- component, command, token, child-range, view-binding, lane, and state-slot
  handle types
- plan topology and lane indexes

**Deliverables**

- Back typed handles with dense plan-owned arenas or equivalent direct-indexed
  storage; remove binary-search lookup tables where the handle already carries
  an admitted plan index.
- Separate exact plan/session generation authority from semantic equivalence
  fingerprints.
- Preserve unchanged handles across a replacement only when carried
  predecessor-region proof establishes the same slot meaning; otherwise mint a
  new generation or slot.
- Add explicit capacity, index-conversion, and generation-exhaustion denial.
- Prevent any public constructor, raw integer conversion, or global registry
  lookup from minting authority.

**Warnings**

- Stable handle reuse must not become stable authority across changed meaning.
- A hash-shaped handle or digest-derived generation is not collision-safe
  authority.
- Do not allocate a second vector per family if one tagged arena plus compact
  family views is measurably simpler and cheaper; choose from evidence.

**Test requirements**

- The same plan index under a foreign generation/session and the same generation
  under a wrong family both deny before target touch.
- Reordering equivalent admitted source facts preserves semantic plan
  equivalence without making handles publicly interchangeable between sessions.
- Exercise `u32` index/capacity and generation boundaries without allocating an
  enormous fixture; exhaustion must deny before truncation or wraparound.
- Scale unrelated families and prove lookup counters for one handle remain
  constant and registry/string counters remain zero.

**Engineering decisions**

- Compact handles remain typed and non-mintable.
- Resolution validates generation and family before dereferencing the slot.
- Direct index access is the ordinary path; reconstructive scans are not an
  accepted fallback.

**Open questions**

- Choose arena partitioning after measuring memory layout and borrow complexity;
  the public handle contract must not expose that internal choice.

## Phase 5: Lower Commands, Tokens, Children, and State Through Ordinary Execution

**Vertical outcome:** The ordinary plan executes component shells, child ranges,
commands, token/style support, and state slots as one coherent lowered slice
without consulting authored identifiers during a frame.

**Relevant subsystems and APIs**

- ordinary lane plan/node/target/receipt/counters/certification
- command, token, child-range, component, and state-slot handles
- plan child ranges, region structure, lane partitions, and render refs
- durable state reconciliation receipts

**Deliverables**

- Lower all ordinary families into canonical lane-ready rows and compact child
  ranges with direct handle resolution.
- Carry admitted durable-state succession into state-slot plan meaning without
  letting the plan decide reconciliation.
- Remove repeated family index vectors or maps that duplicate arena authority.
- Make ordinary receipts distinguish intentional subtree breadth from forbidden
  full-plan scanning.

**Warnings**

- Token/style support must carry native admitted meaning, not recover names from
  a registry.
- Child ranges must be canonical and bounds-checked; overlapping or unsorted
  ranges cannot be repaired silently.
- A command or state transition may not execute through a component-shaped
  fallback.

**Test requirements**

- Equivalent ordinary meaning with reordered declarations produces equivalent
  lane behavior and receipts.
- Swap command/token family tags, overlap child ranges, or bind state succession
  to a foreign predecessor; each must deny before active publication.
- Change one leaf command in a large ordinary tree and prove replacement work is
  bounded by the admitted affected closure while unrelated handles remain
  stable.
- Execute component, command, and token targets repeatedly and prove zero source
  parsing, string lookup, broad scan, and per-frame allocation.

**Engineering decisions**

- Child traversal uses plan-owned ranges, not graph traversal.
- Durable state authority remains in reconciliation; the plan carries only the
  admitted slot/reference needed to execute.
- Family-specific behavior is explicit, not hidden behind a generic helper bag.

**Open questions**

- If root-shell traversal legitimately touches all ordinary rows, its receipt
  must name that target breadth distinctly from a broad-scan violation.

## Phase 6: Lower Query View Bindings Into the Virtualized Data Slice

**Vertical outcome:** A Query-bound view lowers into a cursor/range-aware data
lane and executes through an active view-binding handle; a Query-free
application follows the same plan lifecycle without synthetic Query state.

**Relevant subsystems and APIs**

- Worth Query binding identities, posture, required surfaces, preservation
  receipts, and live rebind evidence
- view-binding handles
- `WorthUiVirtualizedDataPlan`, visible ranges, Query patch posture, targets,
  receipts, counters, and certification
- active application Query status and inspection links

**Deliverables**

- Lower admitted Query binding meaning and required surfaces into plan-owned
  view-binding rows without storing raw Query registries or rediscovering
  bindings per frame.
- Consume one sealed `WorthUiQueryProjectionOutcome`/binding-owned authority
  envelope. Retain only compact references whose provenance and installation
  generation can still be checked; do not trust copied basis digests, support
  flags, projection bags, or source labels.
- Preserve exact Foundational native aspect and struct values through
  refinement and plan admission. Any derived plan scalar must have a named,
  lossless contract from the native value rather than a string/JSON/widened
  numeric conversion.
- Bind Query plan rows to exact candidate/active application generation and
  existing live-rebind evidence.
- Preserve cursor/range semantics and make visible-range execution proportional
  to the requested window.
- Represent Query absence explicitly and cheaply; remove dummy bindings or
  special lifecycle paths.

**Warnings**

- The UI plan consumes admitted Query meaning; it does not recreate Query basis
  or authority.
- Offset pagination is not a cursor substitute, and a full collection scan is
  not a valid fallback when range evidence is missing.
- Query status diagnostics must not be materialized during ordinary data-lane
  execution.
- Query owns installed-domain, live-resource, subscription, result-state,
  recovery, and disposal semantics. The plan records admitted references and
  coordinates lifecycle handoff; it must not implement a local resource
  manager.

**Test requirements**

- Equivalent Query bindings and visible ranges produce equivalent execution
  receipts across lawful candidate generations.
- Equal visible output with a changed Query binding identity, required surface,
  patch posture, or live-rebind generation is non-equivalent or denied as
  appropriate.
- Equal basis digests and native values from a foreign Query runtime or
  installation generation deny, while the same lawful binding expressed
  through equivalent native aspect values remains equivalent.
- A test proves native `AspectValue`/`StructAspectValue` meaning reaches the
  plan edge without JSON, text parsing, numeric widening, or a UI-local fact
  mirror.
- A stale view handle, empty/overflowing range, offset-pagination substitute,
  and full-collection fallback all deny before row execution.
- Grow the backing collection and unrelated graph independently; frame work
  remains bounded by the visible range and forbidden counters stay zero.

**Engineering decisions**

- Query execution identity participates in the operational equivalence basis;
  diagnostic presentation does not.
- Query binding equivalence consumes binding-owned exact authority/equivalence
  evidence; UI code never derives it from labels or digest equality.
- View binding resolution is direct through the active plan.
- Query-free parity is ordinary supported behavior, not a test-only exception.

**Open questions**

- Decide whether cursor-window metadata belongs directly in a lane row or in a
  plan-owned side arena based on measured update and cache behavior.

## Phase 7: Lower Canvas and Spatial Execution End to End

**Vertical outcome:** Canvas draw, hit-test, viewport, overlay, and tool-state
meaning are admitted during lowering and execute from one active spatial plan
without ordinary-widget fallback.

**Relevant subsystems and APIs**

- canvas/spatial plan, node, viewport, overlay, hit-test, tool-state, draw hooks,
  targets, receipts, counters, and certification
- extension-hook and lane admission
- render-resource refs and lane handles

**Deliverables**

- Lower admitted spatial hooks, viewport meaning, hit-test plan, overlay plan,
  tool-state access, and render-resource references into the sealed plan bundle.
- Bind every extension hook and resource ref to the exact host support and plan
  generation that admitted it.
- Execute spatial targets directly from the active plan without graph search,
  hook registry lookup, or ordinary layout reconstruction.
- Make spatial scale terms explicit: visible primitives, queried hit-test
  region, and touched overlay/tool rows.

**Warnings**

- A hook is an admitted extension point, not an alternate semantic authority.
- Missing spatial support must deny during lowering, not quietly reclassify the
  node into an ordinary lane.
- Renderer resources must not be validated by string or global registry on the
  frame path.

**Test requirements**

- Equivalent spatial declarations, hook admissions, and viewport facts produce
  equivalent plan behavior despite lawful source reordering.
- Foreign/stale hook admission, wrong resource generation, missing hit-test
  support, and ordinary-lane fallback each deny before candidate publication or
  frame work.
- Add a large unrelated ordinary/data plan and prove one spatial target's work
  and counters do not grow.
- Mutate one spatial region in a large canvas and prove only its admitted
  affected closure is rebuilt while untouched plan regions remain reusable.

**Engineering decisions**

- Spatial strategy is selected during lowering and represented explicitly in
  the plan.
- Hook admission and resource authority remain distinct even when one operation
  needs both.
- Frame receipts expose compact touched-region evidence, not materialized node
  lists.

**Open questions**

- The internal spatial index may be dense, tiled, or hierarchical; select it
  from scale evidence while preserving the same host-neutral plan contract.

## Phase 8: Lower Realtime Overlay Execution End to End

**Vertical outcome:** HUD and high-frequency overlay work executes from an
admitted active plan and renderer-surface capability under an explicit frame
policy, never through a hidden ordinary pass.

**Relevant subsystems and APIs**

- HUD plan/node/builder, realtime overlay lane, targets, receipts, counters, and
  certification
- high-frequency frame policy
- realtime overlay hooks
- renderer-surface admission and handles

**Deliverables**

- Lower HUD rows, overlay hooks, renderer-surface references, and frame policy
  into the active plan bundle.
- Bind renderer surfaces to exact platform/session and plan generations.
- Keep the high-frequency path free of ordinary layout, declaration/graph
  access, report materialization, and general-purpose allocation.
- Make budget exhaustion and unsupported policy explicit typed outcomes.

**Warnings**

- Realtime priority cannot suppress correctness counters or skip authority
  checks.
- A renderer-surface handle is not platform identity.
- A high-frequency policy must not become a generic callback that can perform
  unmeasured work.

**Test requirements**

- Equivalent HUD meaning, support, and policy produce equivalent plan and frame
  receipts.
- Stale/foreign renderer surfaces, wrong overlay hooks, zero/overflowing budget,
  and ordinary-widget fallback deny before draw work.
- A hostile custom hook that attempts hidden layout, broad scan, or counter
  suppression must fail certification.
- Scale unrelated plan lanes and HUD row count separately; execution work grows
  only with the explicitly targeted realtime rows.

**Engineering decisions**

- Realtime policy is immutable plan input for a generation unless a typed
  replacement changes it.
- Renderer and hook validation occurs off the high-frequency frame path.
- Realtime receipts remain compact enough to aggregate without allocation.

**Open questions**

- If host surface lifetime can end independently of application replacement,
  define the smallest typed invalidation transition rather than adding a frame
  lookup.

## Phase 9: Close the Cross-Lane Bundle and Host-Neutral Boundary

**Vertical outcome:** One candidate/active plan bundle contains all admitted
lanes, and the host adapter receives only host-neutral lowered contacts plus an
active execution capability.

**Relevant subsystems and APIs**

- all four lane plans and lane-meaning parity certification
- lane admission/support descriptors and extension hooks
- egui-named boundary input/contact/plan types
- runtime handoff and Worth UI product facades
- active-session/framework-turn execution surfaces

**Deliverables**

- Seal shared topology, handle arenas, all lane-ready plans, support basis,
  resource refs, equivalence basis, and counters into one internally coherent
  bundle.
- Validate cross-lane ownership and parity once during candidate construction;
  the frame path consumes the result.
- Replace egui-specific canonical-plan meaning with a host-neutral contact
  contract and lower it into egui only inside the adapter.
- Remove public/caller plan builders and plan-accepting frame methods after all
  consumers use active capabilities.
- Add mechanical visibility/dependency checks that prevent reintroduction of
  raw topology/lane-plan execution through product facades.

**Warnings**

- A generic `AnyLane` or default branch is not forward-compatible design; it
  hides missing cost and failure semantics.
- Host neutrality does not mean an untyped property bag.
- Do not duplicate common topology into every lane merely to simplify builders.

**Test requirements**

- Equivalent meaning routed through headless and egui hosts yields equivalent
  host-neutral plan constituents before adapter lowering.
- Conflicting lane ownership, missing support, duplicate plan index, foreign
  hook/resource admission, and egui-only meaning in the canonical plan each
  fail before publication.
- A node transition between lane regimes is classified as non-equivalent and
  cannot leave residue in the predecessor lane.
- Source/topology audits prove product consumers cannot import builders, mint
  bundles, or call an executor with an owned plan object; use no new compile
  fixture for these placement laws.

**Engineering decisions**

- The closed lane set is represented exhaustively.
- Host adapters translate contacts and observations; they never choose UI
  meaning or plan strategy.
- Cross-lane shared facts have one owner and are referenced, not copied into
  competing authorities.

**Open questions**

- Name the host-neutral contact vocabulary with Milestone 3.10 compatibility in
  mind, but do not pre-build mounted receipts in 3.9.

## Phase 10: Prove Canonical Executable Equivalence

**Vertical outcome:** The runtime can decide whether two complete candidate
plans have the same executable meaning without treating a digest or incidental
generation as proof.

**Relevant subsystems and APIs**

- execution-plan equivalence basis, digestor, counters, and reuse classification
- candidate/active plan bundle
- per-region predecessor proof and replacement locality
- lane support, Query, hooks, resources, frame policy, and durable-state slot
  meaning

**Deliverables**

- Define the complete executable-equivalence schema and document why each field
  participates or is excluded.
- Separate semantic fingerprints from exact activation/session/generation
  authority.
- Compare changed regions exactly using carried predecessor proof; use digests
  only for narrowing and diagnostics.
- For Query-backed rows, compare the binding-owned executable contract and
  installed/live authority posture. Do not compare reconstructed UI-local
  summaries of Query meaning.
- Replace the two-state reuse classification with a typed decision that can
  distinguish exact semantic no-op, bounded changed regions, required rebuild,
  and denial without conflating them.
- Make canonical ordering part of construction so equivalence never depends on
  map iteration or source order.

**Warnings**

- Including every generation-bearing handle in semantic equivalence makes
  lawful no-op replacements impossible; excluding changed support or resources
  makes unsafe no-ops possible.
- Digest equality followed by `Reusable` is not collision-safe.
- Diagnostic/counter formatting changes should not churn an operational plan.

**Test requirements**

- A table-driven ordinary test varies source order, incidental candidate
  generation, diagnostics policy, each execution-affecting lane/support/Query/
  hook/resource field, and proves the expected equivalence classification.
- Force equal test fingerprints for structurally different changed regions and
  prove exact comparison rejects the no-op without a production test-only
  authority bypass.
- Equivalent plans with freshly minted handle generations classify semantically
  equivalent while stale predecessor handles still cannot execute.
- Omit one equivalence field in a seeded test schema and prove the schema/field
  coverage audit detects drift.

**Engineering decisions**

- Equivalence is a typed proof over complete executable meaning.
- Authority validation precedes comparison; equivalence never authorizes a
  foreign candidate.
- Per-region proof is the scalability mechanism; whole-plan equality is not the
  ordinary replacement path.

**Open questions**

- Choose whether exact changed-region comparison uses canonical row slices,
  sealed structural receipts, or another representation after measuring it;
  collision safety and `O(A)` work are non-negotiable.

## Phase 11: Make Semantic No-Op a Complete Plan Decision

**Vertical outcome:** A fully lowered, exactly admitted candidate with equivalent
executable meaning produces a typed no-op receipt and leaves every active
authority and resource unchanged.

**Relevant subsystems and APIs**

- application replacement preparation and current early no-op surface
- execution-plan equivalence decision
- plan-swap receipt, frame scheduler, invalidation and allocation ledgers,
  durable state, active inspection, and application generation
- reload counters and diagnostics

**Deliverables**

- Classify no-op only after canonical candidate plan construction, exact
  candidate authority validation, and complete executable equivalence.
- Return a typed no-op receipt that identifies the admitted candidate, active
  generation, equivalence proof, and measured work without pretending a swap
  occurred.
- Preserve the active application generation, plan bundle, handle generation,
  ledgers, scheduler, state, host session, Query binding, and inspection
  authority on no-op.
- Ensure candidate-only allocations and diagnostics are discarded or released
  exactly once.

**Warnings**

- Equal source or artifact digest is not enough to skip lowering.
- No-op must not publish a fresh plan generation merely to report success.
- Candidate inspection remains candidate-scoped even when its plan is
  equivalent.

**Test requirements**

- Equivalent candidates across source order and incidental candidate generation
  produce a no-op with identical active observations and zero swap/publication
  counters.
- Equal artifact digests with changed Query posture, lane support, hook,
  resource, frame policy, or allocation meaning must not produce a no-op.
- Repeat thousands of equivalent replacements and prove no handle-generation,
  retained-memory, registry, scheduler, or inspection-authority drift.
- Interrupt no-op processing at every fallible pre-decision boundary and prove
  the active predecessor remains usable.

**Engineering decisions**

- No-op is a plan decision, not an admission shortcut.
- Candidate work is observable through reload counters even when activation work
  is zero.
- The no-op receipt is separate from `WorthUiPlanSwapReceipt` unless the latter
  is refactored into an exhaustive activation outcome.

**Open questions**

- Decide whether the public application replacement outcome should be one
  exhaustive enum or separate receipts; it must make no-op versus publication
  impossible to confuse.

## Phase 12: Apply Bounded Structural Replacement and Stable Reuse

**Vertical outcome:** A non-equivalent candidate replaces only its admitted
affected plan regions while producing one complete desired successor plan.

**Relevant subsystems and APIs**

- impact lookup and invalidation narrowing
- committed allocation replan and allocation truth delta
- candidate/active plan region representation and handle arena
- lane-ready plan constituents and equivalence decision
- durable-state and Query rebind succession

**Deliverables**

- Carry affected-closure proof into plan lowering so unchanged regions are
  structurally reused without rescanning or rebuilding them.
- Lower changed regions into a complete successor view; never publish a delta as
  if it were complete truth.
- Preserve handle slots and lane storage for exact unchanged meaning, retire
  removed regions, and mint fresh authority for changed/replaced meaning.
- Carry Query-owned live-resource succession/disposal receipts through the
  replacement transaction. Reused bindings retain their lawful Query-owned
  resource; replaced or removed bindings are released exactly once only after
  successor publication can no longer fail.
- Define insertion, deletion, move/reparent, lane transition, Query rebind,
  resource change, and support-policy change behavior.
- Keep retained memory `O(P)` across sustained churn; obsolete candidate and
  predecessor-only storage is reclaimed at an explicit lifecycle boundary.

**Warnings**

- Copying all vectors and calling the result incremental is not acceptable.
- Index stability may not override semantic correctness after reparenting or
  family changes.
- Overlapping affected regions must be canonicalized once, not processed twice.

**Test requirements**

- Change one leaf in plans of increasing unrelated size and prove plan-lowering,
  allocation, comparison, and lane-rebuild counters remain bounded by the same
  affected closure.
- Equivalent deltas expressed through different lawful invalidation order
  produce the same successor plan and reuse receipt.
- Adversarial insert/delete/reparent/family/lane/Query/resource changes preserve
  only exact unaffected handles and reject every stale affected handle.
- Run a long mixed replacement storm and prove retained plan memory, live arena
  slots, and region counts return to the active-plan envelope rather than
  growing with history.
- Rebind, remove, deny, and retry Query-backed regions under a replacement storm
  and prove no double activation, double disposal, leaked live resource, or
  UI-local recovery path.

**Engineering decisions**

- Existing graph/allocation locality proof defines the maximum legal replacement
  scope; the plan lowerer may narrow further but may not widen silently.
- Structural sharing is internal derived storage and does not share mutable
  authority between candidate and active plans.
- Stable reuse requires exact carried predecessor proof, not matching names or
  hashes.

**Open questions**

- Select copy-on-write regions, immutable arenas, or another representation from
  measured `A/P` workloads. The representation must support atomic successor
  publication and prompt predecessor reclamation.

## Phase 13: Publish Application and Plan Authority Atomically

**Vertical outcome:** A successful replacement publishes the successor
application generation, complete plan bundle, allocation/invalidation truth,
state, scheduler posture, Query/inspection authority, and host binding as one
frame-boundary transition.

**Relevant subsystems and APIs**

- `WorthUiActiveApplicationSession` cutover
- committed-allocation activation preflight, transaction, publication, and
  commit resources
- active runtime state, last-valid observation, frame scheduler, transient
  admission, durable resize, Query and planning inspection registries
- plan-swap and application-cutover receipts

**Deliverables**

- Extend the existing activation transaction so the actual plan bundle and
  application generation are prepared and committed together, rather than
  binding application generation after runtime publication.
- Validate every predecessor and candidate authority before acquiring the final
  commit boundary.
- Make commit infallible after the last validation or provide rollback that is
  itself mechanically complete.
- Return one receipt/envelope whose generation, plan, allocation, state,
  scheduler, Query, host, and inspection facts all describe the same successor.
- Publish Query binding succession with the plan cutover and schedule any
  predecessor Query-resource release from the successful commit receipt, never
  from candidate preparation or failed publication.
- Keep last-valid predecessor observation available for typed denial reporting
  without retaining competing active authority.

**Warnings**

- Sequential field assignment inside one method is not automatically atomic
  authority publication.
- Do not let inspection observe the successor before execution can, or vice
  versa.
- A frame-in-progress denial must not consume a candidate irreversibly unless
  the API explicitly returns a retryable pending handle.

**Test requirements**

- Inject failure at every fallible precommit stage and prove all active
  observations, handles, ledgers, state, Query bindings, and inspection remain
  the predecessor.
- At successful cutover, simultaneously inspect application generation, active
  plan, allocation, Query, host, and state facts and prove exact successor
  parity.
- Cross two identical-looking sessions/candidates with equal digests and counts;
  exact session/graph/generation authority must deny before mutation.
- Race a frame-boundary posture change against prepared activation in the
  deterministic scheduler harness and prove no torn or double commit.

**Engineering decisions**

- Publication owns the complete successor bundle and consumes its one-shot
  authority.
- Expected contention and stale-boundary conditions are typed denials, not
  unwinds.
- Plan swap and application cutover are views of one transaction, not nested
  independently authoritative commits.

**Open questions**

- If retry after a transient frame-boundary denial is supported, define a
  session-bound pending activation handle with explicit freshness and disposal.

## Phase 14: Bind Honest Reload and Steady-Frame Cost Surfaces

**Vertical outcome:** The actual launch/replacement/lower/compare/activate/frame
paths emit exact counters that certify their declared complexity and lower into
Foundational only at the measurement boundary.

**Relevant subsystems and APIs**

- reload counter boundary, builder, stop stages, receipt, and Foundational bridge
- steady-frame counter boundary, lane receipts, report planner, diagnostic
  policy, and Foundational bridge
- lane/frame cost certification and scale-variation proof
- measurement counter packets and complexity contracts

**Deliverables**

- Reconcile existing provisional counters with the final canonical lifecycle;
  remove synthetic, duplicate, or never-authoritative rows.
- Define exact counter schemas for initial lowering, semantic no-op, bounded
  replacement, denied replacement, and each steady execution lane.
- Carry active plan/generation identity and affected-scope evidence in compact
  receipts so counter packets cannot be attached to foreign work.
- Enforce forbidden steady-frame counters at the production boundary before
  Foundational lowering.
- Lower certified Worth UI evidence into shared Foundational claim vocabulary
  without importing Foundational into plan meaning or control flow.

**Warnings**

- Always-zero counters that no production path can increment are theater.
- Counter increments inside diagnostics projection do not measure the operation
  they claim to describe.
- Foundational readiness cannot certify a malformed or foreign Worth UI receipt.

**Test requirements**

- Equivalent executions produce equivalent certified cost meaning even when
  diagnostic richness differs.
- Attach a valid-looking counter packet to a foreign plan/generation, omit a
  required row, duplicate a lane, add an unattributed bucket, and overflow a
  counter; every case must deny before Foundational projection.
- Scale `P`, `A`, and `T` independently and prove the measured slopes match the
  declared envelopes for launch, replacement, and frame work.
- Seed each forbidden frame operation through a lower-level counter-schema test
  rather than a test-only production executor branch; certification must fail.

**Engineering decisions**

- One production event owns each increment.
- Counter schemas distinguish ordinary frame work from reconstructive reload
  work and stop-stage denial.
- Foundational claim bundles are derived evidence, not active runtime state.

**Open questions**

- Retain only counters that answer an architectural or operational question;
  remove ornamental row families during schema reconciliation.

## Phase 15: Finish the Scalable Developer and Inspection Surface

**Vertical outcome:** The common API reads as application intent, expensive
lowering/replacement is visibly phased and inspectable, and advanced diagnostics
do not expose a second execution authority.

**Relevant subsystems and APIs**

- Worth UI product `app` and `runtime` facades
- active application session and typed replacement outcomes
- plan inspection, diagnostics projection, AI harness, and Query inspection links
- runtime handoff exports and documentation/`AI_README.md`

**Deliverables**

- Curate the public surface around active-session operations and exhaustive typed
  outcomes; remove raw plan builders, internal digests, and assembly plumbing
  from ordinary consumers.
- Provide compact plan summary/cost/equivalence inspection before activation and
  typed active-plan observation after activation, without exposing owned
  executable plan data.
- Link Query-backed plan rows to Query-owned inspection/evidence handles instead
  of copying Query explanations into UI diagnostics.
- Make expensive replacement/lowering calls visibly expensive and leave cheap
  active observations as ordinary borrows.
- Keep common Query-free/headless usage free of optional-system ceremony while
  retaining explicit advanced Query/host/lane controls at their responsible
  boundary.
- Update feature documentation, examples, discovery docs, and `AI_README.md` to
  teach only the final phase chain and its cost/authority model.

**DX target (illustrative shape, not frozen spelling)**

```rust
let mut session = app.launch(host)?;

let prepared = session.prepare_replacement(submission)?;
let candidate = session.lower_replacement(prepared, committed_allocation)?;

candidate.summary();       // local, compact observation
candidate.cost_envelope(); // planned affected scope; no execution

match session.activate(candidate, frame_boundary)? {
    ReplacementOutcome::NoOp(receipt) => observe(receipt),
    ReplacementOutcome::Activated(receipt) => observe(receipt),
}

session.execute_frame(|frame| {
    frame.component(component_handle).execute()
})?;
```

The ordinary caller never constructs a plan, chooses internal lane strategy,
passes a plan to an executor, or compares digests.

**Warnings**

- A friendly one-call replacement helper may compose the phases, but it cannot
  erase typed no-op/denial outcomes or hide expensive work.
- Explanation is a structured projection of receipts, not log scraping.
- Do not preserve old public names as aliases if they keep the predecessor
  authority model usable.

**Test requirements**

- Common query-free, Query-bound, headless, and egui applications follow the
  same phase progression and produce parity at shared boundaries.
- Product-facade source audits reject raw plan construction, digest-based
  comparison, executor plan injection, and internal lowerer imports without new
  compile fixtures.
- Candidate inspection cannot activate or execute a plan; active inspection
  cannot observe candidate-only facts before cutover.
- Plan inspection cannot promote a Query receipt, basis digest, native aspect
  value, or consumed-projection reference into installed or executable Query
  authority.
- Rich explanation/materialization changes neither equivalence nor steady-frame
  counters and runs only on explicit request.

**Engineering decisions**

- Organized truth and proof-carrying progression define the DX; shorthand is
  secondary.
- Public errors/outcomes preserve failure topology rather than collapsing to a
  boolean or string.
- Advanced APIs expose the next responsible boundary, not unsealed internals.

**Open questions**

- Decide which one convenience method, if any, composes prepare/lower/activate
  for simple applications after the explicit surface is proven and measured.

## Phase 16: Hostile Certification, Legacy Removal, and Closure

**Vertical outcome:** Only the canonical plan lifecycle remains, its claims
survive adversarial end-to-end scenarios, and the milestone closes without test
or compile-time debt.

**Relevant subsystems and APIs**

- every API in the current-disposition table
- source/topology audits, boundary checker, agent-context checker, line-cap gate,
  Clippy, test lanes, and timing evidence
- reload storm, lane parity, identity/state/Query drift, and cost certification
- documentation and public exports

**Deliverables**

- Delete predecessor execution-plan input/witness paths, digest-only active plan,
  caller-built/caller-passed lane execution, egui meaning in canonical topology,
  raw digest no-op, redundant counters, obsolete tests, and compatibility aliases.
- Add anti-bypass source/topology rules at the narrowest authoritative boundary.
- Run the final adversarial matrix against real public lifecycle operations and
  production authority, reusing compiled-once scenario support.
- Record closing structural/timing evidence and reconcile every spec claim to a
  test, audit, counter receipt, or documented engineering decision.
- Leave strict Clippy, warnings, dead-code, Worth UI line caps, test topology,
  boundary-check, agent-context, and all Worth UI proof lanes green.

**Warnings**

- A deprecated alias is still an operational predecessor path.
- Do not add last-minute compile fixtures to prove closure; use the frozen proof
  topology.
- Passing isolated module tests is not end-to-end authority evidence.

**Test requirements**

- Run a mixed launch/frame/replacement storm containing semantic no-ops, bounded
  changes, denials at every phase, Query/no-Query transitions, host-support
  changes, lane transitions, stale handles, equal-digest foreign authority, and
  inspection requests; active truth must remain coherent after every step.
- The Query portion of that storm uses the Query Consumer Kit/public installed
  domain path and includes foreign installation generation, equal digest,
  native-aspect, live-resource succession, and exact-once disposal cases.
- Repeat the storm at multiple unrelated plan widths and prove replacement/frame
  work follows `A`/`T`, not `P`.
- Mechanical audits prove every removed API/path is absent and cannot be reached
  through another facade or alias.
- Re-run proof-parity reconciliation and demonstrate zero net new compiler
  sessions, compile targets, executable compile cases, physical compile fixtures,
  nested Cargo invocations, and generated fixture workspaces.

**Engineering decisions**

- Closure means one principled path, not one preferred path plus compatibility
  residue.
- Any remaining provisional API must be justified as part of the canonical
  lifecycle in the disposition table; otherwise remove it.
- Timing, counter, topology, and behavioral evidence are all required because no
  one proof form establishes the whole milestone.

**Open questions**

- None may remain about authority ownership, equivalence meaning, operational
  plan input, or test topology. A non-blocking representation choice may be
  recorded only if both choices already satisfy every binding invariant.

## Phase ordering rationale

The sequence is intentional:

1. Phase 1 protects iteration speed before the test program grows.
2. Phase 2 establishes the one legal lowering boundary.
3. Phases 3 through 9 grow the actual plan bundle as working vertical execution
   slices: minimal ordinary, handles, complete ordinary, Query/data, spatial,
   realtime, then cross-lane/host closure.
4. Phase 10 defines equivalence only after every execution-affecting constituent
   exists.
5. Phases 11 through 13 use that proof for no-op, bounded replacement, and
   atomic publication.
6. Phase 14 binds cost evidence to the final lifecycle rather than provisional
   functions.
7. Phase 15 curates DX and inspection after the honest authority surface is
   known.
8. Phase 16 removes all predecessor paths and certifies the whole system.

This avoids horizontal half-completion: each executable lane works through the
same active-plan owner before optimization and public-surface closure depend on
it.

## Milestone acceptance evidence

Milestone 3.9 is complete only when all of the following are true:

- one sealed authority lowers exact candidate application, graph,
  capability/Query, host-support, and committed-allocation truth into one
  candidate plan bundle;
- the active application session owns or exclusively governs the complete
  active plan, and callers cannot construct or submit plans to executors;
- launch and every successful non-no-op replacement publish a real plan bundle
  atomically with application/runtime authority;
- identical executable meaning produces collision-safe equivalence and a typed
  no-op even across incidental candidate generations;
- every execution-affecting change is classified non-equivalent, and stale or
  foreign authority cannot activate or execute even when digests/counts match;
- component, command, token, child-range, state-slot, view-binding, lane, and
  render-resource access uses sealed compact handles with direct indexed
  resolution and explicit exhaustion denial;
- ordinary, virtualized data, canvas/spatial, and realtime execution consume
  lane-ready active-plan constituents and satisfy their separate cost/failure
  contracts;
- initial lowering is `O(P)`, replacement work is `O(A)` plus changed output,
  target execution is `O(T)`, and retained plan memory is `O(P)` under long
  replacement storms;
- steady frames prove zero parsing, artifact validation, string registry lookup,
  broad scan, rich diagnostic materialization, and general-purpose heap
  allocation;
- reload and steady-frame counter receipts are exact, foreign-proof, and lower
  into Foundational only after Worth UI certification;
- host-neutral plan meaning reaches egui only through the adapter, with no
  egui-owned semantic decision in canonical runtime topology;
- Query-backed plan rows preserve one sealed installed/consumed authority edge,
  exact native aspect meaning, support-versus-admission posture, and Query-owned
  live-resource lifecycle without UI-local mirrors;
- plan and AI inspection link to Query-owned evidence without promoting
  receipts, digests, or observed native values into authority;
- test topology retains proof parity with no new compiler session, compile
  target, executable compile case, physical compile fixture, nested Cargo
  invocation, or generated fixture workspace, and closing timing evidence
  satisfies the iteration budget;
- documentation and `AI_README.md` teach only the canonical lifecycle; and
- strict Worth UI tests, Clippy, warnings/dead-code, line caps, boundary-check,
  agent-context, topology budgets, and hostile certification are green.

## Non-goals

- Milestone 3.9 does not define mounted-node or mounted-frame receipts; that is
  Milestone 3.10.
- It does not implement the full host observation/rebind planner scheduled after
  mounted receipts, though its active plan must be ready to consume those later
  transitions.
- It does not add an open-ended third-party lane plugin system.
- It does not persist or replay executable plans as a new source of truth.
- It does not optimize compiler performance outside the Worth UI test program.
- It does not preserve predecessor APIs for source compatibility when they keep
  a competing authority path alive.
