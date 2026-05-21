# Milestone 4 Engineering Spec: Topology-Certified Primitive Construction

> **Status:** Planned
>
> **Roadmap parent:** [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
>
> **Predecessors:**
> - [milestone-1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1.md)
> - [milestone-2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-2.md)
> - [milestone-3.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-3.md)
>
> **Predecessor closeouts:**
> - [milestone-1-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1-closeout.md)
> - [milestone-2-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-2-closeout.md)
> - [milestone-3-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-3-closeout.md)
>
> **Vision parent:** [VISION.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/VISION.md)
>
> **Test requirements:**
> - [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
> - [topo-test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/topo-test-requirements.md)
>
> **Primary architectural driver:** establish the first honest
> kernel-to-spatial-to-topology primitive construction pipeline so primitive
> bodies are born with explicit construction-time geometry meaning, stable
> authority boundaries, and replay-safe certification instead of forcing later
> binding, boolean, fillet, and NURBS milestones to retrofit meaning onto
> topology that was created geometry-blind

## Goal

Establish the first honest primitive and body construction substrate for Worth:

- kernel-authored
- geometry-backed
- spatially explicit at construction birth
- topology-authoritative
- replay-safe
- branch-safe
- hostile-proof-oriented from the start

Milestone 4 is where Worth stops depending on seeded topology fixtures as the
main source of bodies and starts proving that bodies can be constructed
generically through a real pipeline without collapsing topology truth, geometry
truth, and kernel orchestration into one crate.

## Why This Milestone Exists

Milestone 4 is not "port cube builders."

It is the milestone that decides whether primitive construction becomes:

- a trustworthy authority-respecting construction pipeline that later booleans,
  fillets, continuity, and NURBS work can inherit, or
- a temporary pile of primitive helpers that create topology first, invent
  geometry meaning later, and force Milestone 5+ to retrofit binding truth onto
  already-born bodies

Milestone 4 must therefore solve the hard problem first:

- `worth-topo` must remain topology-authoritative and geometry-free
- `worth-geom` must remain pure geometry and construction math
- `worth-kernel` must become the construction orchestrator instead of silently
  reusing old `forge-kernel` architecture as authority
- `worth-spatial` must exist early enough to own the construction-time
  topology/geometry birth seam, but narrowly enough that Milestone 4 does not
  collapse into the full rebinding and continuity scope reserved for Milestone 5

That is the real precedent this milestone sets for the geometry kernel.

If Milestone 4 gets this wrong, every later milestone will inherit one of two
lies:

- geometry-free topology birth that later needs retroactive binding truth, or
- topology construction that is already contaminated with spatial or kernel
  authority

Milestone 4 must instead prove one explicit story:

`worth-geom` produces construction carriers, `worth-kernel` authors admitted
construction intent, `worth-spatial` owns construction-time topology/geometry
birth truth, and `worth-topo` remains the only topology authority.

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects is solving the hard problem first for
  the product we are actually building. Milestone 4 must therefore create the
  construction boundary that cubes, booleans, fillets, and later NURBS can all
  inherit, rather than shipping a geometry-blind primitive layer and promising
  to add honest spatial meaning later.
- `arch_laws.md`
  The most important thing it protects here is authority separation and
  proof-bearing boundaries. Primitive construction must have distinct kernel,
  spatial, topology, and certification surfaces, with no shadow authority in
  helpers or convenience APIs.
- `composition_laws.md`
  The most important thing it protects is named semantic steps. Milestone 4
  must decompose into explicit construction intent, scaffold generation,
  spatial birth attachment, topology lowering, and certification steps rather
  than one large primitive builder that hides the authority chain inline.
- `domain_structure_laws.md`
  The most important thing it protects is crate topology that teaches the
  system. Milestone 4 must make `worth-kernel`, `worth-spatial`,
  `worth-topo`, and `worth-geom` earn distinct boundaries instead of hiding the
  construction seam in whichever crate is currently convenient.
- `perf_laws.md`
  The most important thing it protects is explicit breadth and locality
  accounting. Primitive construction must expose construction breadth, assembly
  breadth, binding breadth, and certification breadth directly, rather than
  burying whole-body scans inside cheap-looking APIs.
- `VISION.md`
  The most important thing it protects is the spec graph and traced truth
  thesis. Milestone 4 must make primitive construction a real specification
  workflow with replayable, inspectable, auditable outcomes rather than a pile
  of one-off body generators.
- `worth_roadmap.md`
  The most important thing it protects is sequencing. Milestone 4 belongs after
  topology editing and before broad binding, planar exactness, and booleans
  because later milestones need a real construction substrate, not seeded
  topology and not retroactive geometry meaning.
- `worth/test-requirements.md`
  The most important thing it protects is family closure. Milestone 4 must
  prove admitted primitive and body families generically over family ladders and
  arbitrary admitted counts, not one cube, one prism, and one tetrahedron.
- `worth/topo-test-requirements.md`
  The most important thing it protects is that topology must still certify the
  resulting bodies generically and adversarially. Primitive construction may be
  authored by `worth-kernel`, but topology legality, replay, and failure
  localization still have to close honestly in `worth-topo`.
- `milestone-3.md`
  The most important thing it protects is the geometry-free topology-edit
  substrate. Milestone 4 must consume that substrate rather than reintroducing
  shape-program semantics or geometry-bearing helpers into `worth-topo`.
- `milestone-3-closeout.md`
  The most important thing it protects is that topology editing is now trusted
  enough to serve as the construction substrate. Milestone 4 must build on that
  authority instead of bypassing it with direct body assembly internals.
- `milestone-2-closeout.md`
  The most important thing it protects is the derived-read and invalidation
  boundary. Milestone 4 must not collapse primitive construction diagnostics or
  inspection into hidden derived helper logic.
- `crates/forge-query/docs/foundations/workspace-overview.md`
  The most important thing it protects is that ordinary runtime-backed product
  code must start from `ForgeQueryWorkspace` rather than from lower-runtime
  plumbing. Milestone 4 must therefore specify a Query-authored front door for
  primitive construction, replay, preview, and inspection work.
- `crates/forge-query/docs/foundations/support-matrix-and-admission.md`
  The most important thing it protects is that method presence is not support.
  Milestone 4 must name which Query runtime families it depends on and require
  support-matrix or admission-gate proof rather than casual facade use.
- `crates/forge-query/docs/foundations/branches-and-previews.md`
  The most important thing it protects is that preview and branch work are
  authority-lane shifts over retained surfaces, not separate runtimes.
  Milestone 4 branch-local construction must reuse that lane model directly.
- `crates/forge-query/docs/modeling/aspects-and-authority-lanes.md`
  The most important thing it protects is explicit aspect contracts and
  auditable authority lanes. Milestone 4 must not invent a separate Worth-local
  lane story for construction, preview, delivery, or pending work.
- `crates/forge-query/docs/runtime-surfaces/live-views.md`
  The most important thing it protects is that live views are retained
  query-shaped installations, not one-shot queries. Milestone 4 diagnostics and
  certification reads must use retained Query surfaces honestly.
- `crates/forge-query/docs/runtime-surfaces/computed.md`
  The most important thing it protects is that derived runtime state remains
  rebuildable and explicit. Milestone 4 must keep derived construction
  diagnostics and rollups in Query-owned computed surfaces instead of smuggling
  them into authority paths.
- `crates/forge-query/docs/runtime-surfaces/reads-observe-materialize.md`
  The most important thing it protects is that reads, observation, and
  materialization are distinct retained-surface consumption paths. Milestone 4
  must not flatten them into one generic "get some rows" story.
- `crates/forge-query/docs/execution/writes-and-intents.md`
  The most important thing it protects is the distinction between ordinary
  direct writes, graph-shaped same-batch authoring, existing-truth mutation,
  and covered intent families. Milestone 4 must use the right mutation surface
  per construction step instead of reinventing authoring vocabulary.
- `crates/forge-query/docs/execution/intent-admission.md`
  The most important thing it protects is one shared public admission lattice
  for covered families. Milestone 4 must use intent admission only where the
  family truly belongs there and otherwise stay on direct mutation or read
  surfaces.
- `crates/forge-query/docs/execution/effects.md`
  The most important thing it protects is that effects are retained staging or
  delivery surfaces, not hidden truth mutation. Milestone 4 may use them for
  diagnostics or staged work, but not as a shadow construction authority.
- `crates/forge-query/docs/capabilities/projection-consumption.md`
  The most important thing it protects is typed fact extraction from Query
  artifacts. Milestone 4 diagnostics and certification must use projection
  consumption instead of payload or row archaeology when they need typed
  identities, memberships, or continuity facts.
- `crates/forge-query/docs/capabilities/existing-truth.md`
  The most important thing it protects is typed binding and verification for
  already authoritative targets. Milestone 4 must use these surfaces whenever
  construction edits reuse authoritative topology instead of rebuilding local
  target identity logic.
- `crates/forge-query/docs/capabilities/historical-diff-and-basis.md`
  The most important thing it protects is explicit basis binding and query-
  shaped diff semantics. Milestone 4 replay, branch-local parity, and
  historical construction comparisons must consume admitted basis posture rather
  than ad hoc branch or snapshot identifiers.
- `crates/forge-query/docs/capabilities/inspection.md`
  The most important thing it protects is one unified public explanation
  surface over retained Query artifacts. Milestone 4 must inspect through Query
  receipts and inspection artifacts instead of lower-runtime spelunking.
- `crates/forge-query/docs/authoring/graph-composition-authoring.md`
  The most important thing it protects is that same-batch graph authoring is a
  first-class Query surface with symbolic handles, lifecycle evidence, and
  denied-path diagnostics. Milestone 4 must use that surface for construction
  programs that are truly graph-shaped instead of caller-owned batch folklore.

## Adversarial Constraint

Milestone 4 must survive this hostile condition:

> Arbitrary admitted primitive-construction workflows, including high-face-count
> shells, shell-with-hole construction, wire-body construction, branch-local
> construction histories, replayed accepted and rejected construction requests,
> and family-parameter sweeps over the admitted primitive ladder, must either
> produce the same deterministic topology truth, construction-time spatial birth
> truth, and machine-checkable certification artifacts or fail with exact,
> localized diagnostics, all while `worth-topo` remains geometry-free and no
> later milestone is forced to retrofit geometry meaning onto already-committed
> topology.

Concretely, the design fails if Milestone 4:

- lets primitive construction bypass the canonical topology authority boundary
- lets `worth-kernel` become a shadow owner of topology or spatial truth
- lets `worth-topo` absorb geometry or construction semantics that belong in
  `worth-spatial` or `worth-kernel`
- creates topology first and treats geometry meaning as a later attachment
  convenience instead of explicit construction-time truth
- proves primitive construction with a few curated solids while generic family
  ladders remain unproven
- replays the same admitted construction request into different topology,
  spatial birth, naming, or diagnostic outcomes
- makes later booleans, fillets, or NURBS depend on hidden assumptions about
  how constructed faces, edges, loops, or shells acquired their geometry
  meaning

The hostile question for this milestone is:

`if later kernel work depends on the bodies born here, did Milestone 4 make the birth contract explicit enough that later work inherits truth instead of compensating for missing authority?`

## Product Decision Lock

- `forge-kernel` and `forge-spatial` are historical reference material only;
  they are not architectural authority for Worth
- Milestone 4 establishes fresh Worth crate roles rather than porting old
  kernel structure by inertia
- `worth-topo` remains geometry-free and topology-authoritative
- `worth-geom` remains pure geometry and construction math
- `worth-kernel` owns primitive construction workflows and orchestration
- `worth-spatial` owns the construction-time topology/geometry birth seam
- Milestone 4 spatial truth is intentionally narrow:
  - birth-time carrier attachment
  - construction-time geometry meaning
  - stable construction identity bridges needed by later milestones
- Milestone 4 does not yet close the whole Milestone 5 spatial binding world:
  - broad rebinding
  - historical continuity inspection over mature binding histories
  - full branch-local binding inspection families
  - full curved carrier closure
- primitive construction must lower through an explicit
  kernel -> spatial -> topology pipeline
- topology certification remains in `worth-topo`, even when the workloads are
  authored by `worth-kernel`
- spatial construction truth must be authoritative enough that later booleans,
  fillets, and curved work inherit a real substrate instead of retrofitting one

## Forge Query Utilization Contract

Milestone 4 must be explicit about how Worth uses the real public
`forge-query` runtime surface.

The crate docs in `crates/forge-query/docs` are the authority here, not vague
runtime folklore. Primitive construction must consume Query exactly as a
Query-authored runtime, not as an excuse to recreate local runtime patterns in
`worth-kernel`, `worth-spatial`, or `worth-topo`.

### Public Runtime Front Door

Ordinary runtime-backed Worth code must enter through
`ForgeQueryWorkspace`-shaped public surfaces:

- `runtime.workspace(...)`
- `workspace.insert(...)`
- `workspace.update(...)`
- `workspace.update_existing(...)`
- `workspace.delete(...)`
- `workspace.batch(...)`
- `workspace.compose_graph(...)`
- `workspace.live_view(...)`
- `workspace.computed(...)`
- `workspace.effect(...)`
- `workspace.read(...)`
- `workspace.observe(...)`
- `workspace.materialize(...)`
- `workspace.preview(...)` / `workspace.branch(...)`
- `workspace.inspect(...)`

`workspace.write(...)` remains available as the expert lower-level seam, but
Milestone 4 should treat it as exceptional infrastructure, not ordinary domain
DX.

### Query-First Escalation Rule

Milestone 4 must assume Query handles runtime semantics natively.

That means Worth is not allowed to compensate for a missing Query runtime
boundary by teaching a private substitute in `worth-kernel`, `worth-spatial`,
or `worth-topo`.

If the milestone discovers that Query does not yet provide a required runtime
surface, the required response is:

1. identify the exact missing Query boundary
2. name the missing public Query surface or proof artifact explicitly
3. harden Query first, or mark the capability blocked and out of scope
4. only then continue the Worth implementation

Milestone 4 must not close by relying on:

- Worth-local branch or preview semantics
- Worth-local graph authoring semantics where Query should own them
- Worth-local target binding, verification, or inspection folklore
- row or payload archaeology where Query should expose typed fact extraction
- hidden lower-runtime plumbing passed off as ordinary downstream runtime DX

The kernel hardens the runtime just as much as the runtime launches the kernel.
If Worth hits a real runtime boundary gap, the milestone should surface that as
Query work, not bury it under local workaround code.

### Mutation And Authoring Rules

Primitive and body construction must use the Query mutation surfaces
intentionally:

- use direct authoritative write surfaces when the construction step is already
  fully known
- use `workspace.compose_graph(...)` when one construction step needs
  same-batch symbolic handles, created-target follow-up mutation, mixed created
  and existing authoritative targets, or graph-shaped lifecycle evidence
- use `workspace.bind_existing_entity(...)`,
  `workspace.bind_existing_relation(...)`, `workspace.update_existing(...)`,
  `workspace.delete_existing(...)`, `workspace.assert_existing(...)`,
  `workspace.verify_existing(...)`, `workspace.update_existing_verified(...)`,
  `workspace.delete_existing_verified(...)`, and `workspace.probe_existing(...)`
  when construction must reuse or verify already authoritative topology truth
- use covered intent admission only when the family genuinely belongs on the
  admitted intent path; direct writes remain the ordinary path

Milestone 4 must not teach:

- caller-owned `workspace.batch(...)` string choreography as a substitute for
  graph composition
- direct lower-runtime mutation plumbing from Worth crates
- hand-built target-resolution or relation-rewrite semantics outside Query's
  existing-truth and graph-composition surfaces

### Read, Projection, And Inspection Rules

Milestone 4 construction diagnostics and certification must consume Query
artifacts through the public retained-handle model:

- `workspace.live_view(...)` declares retained query-shaped truth surfaces
- `workspace.computed(...)` declares retained derived runtime state
- `workspace.effect(...)` declares retained delivery or pending-intent staging
  surfaces
- `workspace.read(...)`, `workspace.observe(...)`, and
  `workspace.materialize(...)` consume retained surfaces; they do not create
  alternate authority paths
- projection fact extraction must go through Projection Consumption, not row
  archaeology
- retained explanations must go through `workspace.inspect(...)` or the covered
  inspection intent family, not through lower-runtime spelunking

When Worth needs typed facts from a read result, write receipt, or
query-context artifact, it must use Query projection consumption instead of
re-parsing payload bags or rebuilding meaning in caller code.

### Basis, Branch, And Preview Rules

Milestone 4 must treat branch, preview, and history posture as explicit Query
capabilities:

- preview and branch work are authority-lane shifts over the same retained
  runtime surfaces
- preview-local and branch-local construction experiments must enter through
  `workspace.preview(...)` / `workspace.branch(...)`
- current-head, branch-head, preview-derived, and historical comparisons must
  use admitted basis and diff surfaces instead of ad hoc branch or snapshot
  identifiers
- branch-local parity and replay parity must therefore preserve both Worth
  semantics and Query basis semantics

Milestone 4 must not teach "branch-aware construction" as custom branch
plumbing in Worth crates. Query already owns that runtime language.

### Support, Admission, And Anti-Bypass Rules

Method presence is not a support claim.

Milestone 4 code and certification must use:

- `workspace.public_support_matrix()`
- `workspace.public_api_contract()`
- `workspace.public_handle_contract()`
- `workspace.public_mutation_surface_report()`
- `workspace.admit_public_api_family(...)`

to prove which Query families Worth is building on and where support remains
deferred or unsupported.

Milestone 4 must fail closed rather than:

- assuming future temporal or async families already execute
- assuming any visible facade family is supported just because the method name
  exists
- inventing parallel Worth-local APIs for something Query has already declared
  as the future public facade
- widening Worth-local runtime semantics just because the current Query surface
  is inconvenient

If one of these checks fails, the milestone should emit an explicit Query
boundary gap report instead of "temporarily" hand-wiring the missing behavior
inside Worth.

### Query-Owned Semantics Worth Must Reuse

Milestone 4 must explicitly inherit these Query semantics rather than
redefining them:

- aspects as the auditable contract for what a surface reads, writes, or
  produces
- authority lanes as the auditable ownership locations for truth, branch-local
  truth, preview truth, derived runtime state, effect delivery state, and
  pending write intent
- graph-composition lifecycle evidence as the canonical story for same-batch
  symbolic authoring
- existing-truth binding evidence as the canonical story for mutation or probe
  against already authoritative topology truth
- basis admission and historical diff shaping as the canonical story for
  current, branch, preview, and historical execution
- inspection artifacts and projection-consumption receipts as the canonical
  explanation surfaces

### Required Workflow-To-Query Mapping

Milestone 4 should not leave Query usage at the level of "use the runtime
somewhere in here." Each construction workflow class must map to one explicit
Query surface family.

At minimum, the spec assumes this default mapping:

- primitive construction declaration and retained runtime setup
  - `runtime.workspace(...)`
  - `workspace.live_view(...)`
  - `workspace.computed(...)`
  - `workspace.effect(...)` only when the milestone truly needs retained
    delivery or pending-intent staging
- ordinary authoritative primitive birth steps
  - `workspace.insert(...)`
  - `workspace.update(...)`
  - `workspace.delete(...)`
  - `workspace.batch(...)` only when ordering is sufficient and no symbolic
    graph semantics are required
- graph-shaped construction steps
  - `workspace.compose_graph(...)`
  - `workspace.compose_graph_with_invariant_pack(...)` when the runtime-valid
    graph still needs domain-owned rejection
- reuse of already authoritative topology truth
  - `workspace.bind_existing_entity(...)`
  - `workspace.bind_existing_relation(...)`
  - `workspace.update_existing(...)`
  - `workspace.delete_existing(...)`
  - `workspace.assert_existing(...)`
  - `workspace.verify_existing(...)`
  - `workspace.update_existing_verified(...)`
  - `workspace.delete_existing_verified(...)`
  - `workspace.probe_existing(...)`
- runtime-backed readback and certification consumption
  - `workspace.read(...)`
  - `workspace.observe(...)`
  - `workspace.materialize(...)`
  - Projection Consumption
  - `workspace.inspect(...)`
- isolated experimentation and parity proof
  - `workspace.preview(...)`
  - `workspace.branch(...)`
  - admitted basis and diff surfaces from historical/basis Query capability

If a workflow step cannot name its Query surface family this concretely, the
spec should treat that as unfinished design, not as implementation freedom.

### Mechanical Enforcement Requirements

Milestone 4 must turn the most important architectural rules into compile-time
or mechanical enforcement, not just prose.

Required enforcement directions:

- `worth-kernel`, `worth-spatial`, and `worth-topo` must expose narrow facades;
  lower-runtime or crate-internal helper modules should remain `pub(crate)` or
  stricter wherever possible
- construction phase progression must use proof-bearing wrapper types so later
  phases cannot accept earlier artifacts accidentally
- Query anti-bypass must be testable through compile-fail or structural audit
  lanes, not left as a naming convention
- if a new required runtime subsystem is introduced, construction sites and
  propagation sites should fail to compile until the new subsystem is threaded
  through the milestone-owned pipeline
- direct lower-runtime imports from Worth public workflow code should be
  mechanically audited and rejected

Expected enforcement lanes include:

- compile-fail proof for out-of-order construction phase usage
- compile-fail or visibility proof for bypassing crate facades
- structural audit proof that public Worth workflow code does not import
  lower-runtime Query plumbing directly
- closeout proof that every required Query family is consumed through its
  documented public surface

Document-only rules are insufficient for:

- phase ordering
- authority separation
- Query anti-bypass
- canonical artifact ownership

## Target End-State Layer DX

Milestone 4 should not stop at "the boundaries exist." It should define what
the finished code looks like when those boundaries are being used correctly.

The end-state DX must be explicit enough that:

- a junior engineer can write the common path without inventing local runtime
  folklore
- a senior engineer can find the advanced path and inspect the real lowering
  and proof chain
- an operator can understand where Query receipts, construction digests, birth
  truth, and topology certification come from
- later milestones can widen the same layer surfaces instead of replacing them

The milestone should therefore describe the final code shape at:

- the `worth-kernel` authoring surface
- the `worth-kernel -> worth-spatial` boundary
- the `worth-spatial` admitted birth surface
- the `worth-spatial -> worth-topo` lowering boundary
- the `worth-topo` authority surface
- the `worth stack -> forge-query` runtime boundary
- the simulation / replay surface
- the inspection / certification surface

### Kernel Call-Site DX

The common path should read like primitive intent, not like topology surgery or
runtime plumbing.

Representative common-path shape:

```rust
let mut workspace = runtime.workspace("worth.kernel").unwrap();

let result = kernel
    .construction()
    .execute(
        PrimitiveConstruction::orthotope(
            OrthotopeSpec::solid(
                Length3::mm(40.0, 20.0, 10.0),
                CoordinateFrame::world(),
            )
            .named("housing.main")
            .with_birth_policy(ConstructionBirthPolicy::planar_faces()),
        ),
        ConstructionExecution::authoritative()
            .actor(actor)
            .artifact_policy(ArtifactPolicy::Audit),
        &mut workspace,
    )?;

let body = result.body();
let receipt = result.receipt();
let birth = result.birth_truth();
let certification = result.certification();
```

This common path should make obvious:

- what primitive family is being requested
- what semantic parameters define it
- what execution context is being used
- that Query is the runtime front door
- that one canonical result envelope is returned

The common path should not require the caller to:

- manually assemble topology operators
- manually assemble Query graph writes
- manually bind branch or preview posture
- manually braid together receipts from multiple subsystems

### Kernel Advanced-Path DX

The advanced path should expose the real lowering and proof chain without
forcing the common path to speak infrastructure.

Representative advanced-path shape:

```rust
let intent = kernel.construction().intent(
    PrimitiveConstruction::orthotope(
        OrthotopeSpec::solid(
            Length3::mm(40.0, 20.0, 10.0),
            CoordinateFrame::world(),
        )
        .named("housing.main")
        .with_birth_policy(ConstructionBirthPolicy::planar_faces()),
    ),
    ConstructionExecution::authoritative()
        .actor(actor)
        .artifact_policy(ArtifactPolicy::Audit),
)?;

let planned = intent.plan()?;
let lowered = planned.lower()?;

lowered.query_surface_report();
lowered.construction_scaffold();
lowered.spatial_birth_plan();
lowered.topology_lowering_plan();
lowered.concurrency();
lowered.cost();

let executed = lowered.execute(&mut workspace)?;
let diagnostics = executed.materialize_diagnostics(DiagnosticDetail::Forensic)?;
```

This is where the milestone should make the DX distinction from
`dx_laws.md` explicit:

- the common path reads like intent
- the advanced path exposes the next lower truthful layer
- the advanced path is where cost, scope, policy, locality, and Query-family
  usage become inspectable

### Kernel -> Spatial Boundary DX

This boundary should feel like semantic lowering, not helper passing.

Kernel should hand spatial:

- primitive family
- normalized primitive parameters
- geometric construction scaffold
- construction execution context
- explicit construction birth policy

Spatial should return:

- admitted construction birth plan
- typed impossible-birth rejection
- birth completeness evidence
- birth digest / identity artifact

Representative shape:

```rust
let birth_plan = spatial
    .bindings()
    .plan_construction_birth(
        scaffold,
        ConstructionBirthContext::new()
            .family(PrimitiveFamily::Orthotope)
            .policy(ConstructionBirthPolicy::planar_faces()),
    )?;
```

Kernel must not:

- perform carrier attachment semantics itself
- lower directly into topology mutation details
- invent construction-born binding identity locally

### Spatial Internal DX

The common spatial surface should read as admitted construction birth truth,
not as generic geometry glue.

Representative shape:

```rust
let admitted_birth = birth_plan.admit()?;
let topology_plan = admitted_birth.lower_to_topology()?;
```

Spatial should own:

- construction-time face/edge/vertex geometric meaning
- completeness and impossibility checks
- birth identity sufficient for replay and parity proof

Spatial should not own:

- primitive-family authoring ergonomics
- topology legality execution
- Query branch, basis, or receipt semantics

### Spatial -> Topo Boundary DX

This boundary should be the lowering boundary from admitted birth truth into
topology authority.

Spatial should pass:

- admitted topology-lowering plan
- explicit topology-to-birth mapping
- proof that the birth contract is complete enough to lower

Topo should return:

- executed topology artifact
- topology legality result
- naming / identity / certification evidence
- localized topology rejection when invalid

Representative shape:

```rust
let executed = topo
    .construction_authority()
    .execute_lowered_construction(topology_plan, &mut workspace)?;
```

Spatial must not mutate topology truth directly. Topo must not infer geometry
meaning from partial hints.

### Topo Authority DX

The topo surface in this milestone should look like authority execution, not
like shape generation.

Topo should expose:

- execution of admitted topology construction plans
- topology legality and locality artifacts
- naming / replay / certification artifacts

Topo should not expose:

- primitive-family authoring convenience
- geometric carrier semantics
- Worth-local runtime compensation APIs

### Worth -> Query Runtime DX

Worth should consume Query as a native runtime substrate, not as a transport
pipe or fallback utility.

The end-state code should make it obvious when we expect:

- direct write surfaces
- graph-shaped same-batch authoring
- existing-truth reuse and verification
- retained live/computed/effect readback
- preview / branch experimentation
- inspection
- projection consumption
- basis and diff parity

Representative shapes:

```rust
let receipt = workspace.compose_graph(|graph| {
    // same-batch symbolic construction lowering
    Ok(())
})?;
```

```rust
let probe = workspace.probe_existing(binding, ["identity.id", "kind.value"])?;
```

```rust
let facts = receipt.consume_projection_facts(...)?;
```

The finished Worth product code should not have to reconstruct these meanings
from:

- caller-owned batch choreography
- local target-binding helpers
- raw payload parsing
- local branch or snapshot identifiers

### Simulation And Replay DX

Simulation and replay should be first-class common surfaces, not late forensic
helpers.

Representative shapes:

```rust
let simulation = kernel
    .construction()
    .simulate(
        intent.clone(),
        SimulationContext::new()
            .actor(actor)
            .clock(fixed_clock)
            .artifact_policy(ArtifactPolicy::Audit),
        &mut workspace,
    )?;
```

```rust
let replay = kernel
    .construction()
    .replay(result.decision_id(), &mut workspace)?;
```

Simulation and replay should preserve:

- the same primitive intent identity
- the same Query basis / branch semantics
- the same construction birth truth semantics
- the same topology and certification parity story

### Inspection And Certification DX

The caller should get one canonical construction result envelope, with richer
diagnostics materialized on demand.

Representative shape:

```rust
let result = kernel.construction().execute(...)?;

result.result_digest();
result.birth_truth_digest();
result.query_receipt_digest();
result.certification_digest();

let forensic = result.materialize_diagnostics(DiagnosticDetail::Forensic)?;
```

This should make clear:

- domain truth is separate from diagnostics
- diagnostics are policy-controlled and structured
- the final result is one canonical artifact family, not a caller-assembled
  braid of subsystem internals

### End-State DX Rules

Milestone 4 should explicitly reject finished code shapes that look like:

- caller-owned `workspace.batch(...)` choreography where
  `workspace.compose_graph(...)` is the honest boundary
- row or payload archaeology where Projection Consumption should be used
- Worth-local branch / preview / basis folklore instead of Query basis
  semantics
- several unrelated receipts that the caller must manually correlate to
  understand one construction outcome

The milestone should describe the common path, advanced path, simulation path,
and inspection path in enough detail that implementation drift is obvious.

## Worth Crate Documentation Model

Milestone 4 should also define how Worth crate documentation is organized so
future engineers and AI agents can learn the runtime and construction model
from docs first instead of re-reading the implementation tree.

This is not an optional afterthought. The docs are part of the architecture.
They must preserve the public meanings, boundary rules, feature surfaces, and
anti-bypass posture the code implements.

### Documentation Rules

Worth docs should be treated as an executable architecture map. They are not a
marketing layer and not a loose notebook.

For every Worth crate that becomes part of the milestone surface:

- crate docs live inside the crate under `docs/`
- the crate docs are organized into three explicit layers:
  - crate-map docs
  - feature docs
  - boundary docs
- docs are organized into folders by category rather than as one flat dump
- each shipped public feature gets exactly one owning feature doc
- each major cross-crate or crate-to-Query handoff gets an owning boundary doc
- examples live inside the owning feature doc rather than in parallel example
  files
- `docs/README.md` must explain:
  - what this crate owns
  - what this crate does not own
  - what style of docs this crate writes
  - the category map
  - the reading order
  - when a reader should jump to a neighboring crate or to `forge-query`
- docs must teach the common path, advanced path, anti-patterns, and ownership
  story for the owning feature
- docs must explicitly name Query usage when that feature depends on Query

The intended workflow for future agents should be:

1. read crate `docs/README.md`
2. read the relevant feature and boundary docs for the surface they are
   touching
3. only then read implementation modules if the docs do not answer the design
   question

### One-Doc-Per-Feature Rule

The docs model should mirror the `forge-query` crate style:

- one doc per public feature surface
- one folder per category
- examples folded into the feature doc
- feature docs own:
  - what the feature is
  - why you use it
  - stable entry points
  - common path
  - advanced path
  - Query integration, if any
  - inspection / debugging
  - anti-patterns
  - current limits
  - related docs

This keeps future readers out of grep-driven archaeology and makes feature
ownership obvious.

### Boundary Doc Rule

Feature docs are not enough on their own because Worth is a stacked system.
The public story also depends on the handoff contracts between layers.

Milestone 4 should therefore require explicit boundary docs for the major
handoffs it introduces, including:

- `worth-kernel -> worth-spatial`
- `worth-spatial -> worth-topo`
- `Worth -> forge-query`

Each boundary doc should teach:

- what the upstream layer is allowed to provide
- what the downstream layer must return
- what must be impossible to represent or call from this boundary
- what artifact, receipt, or digest binds the handoff
- what Query surface is allowed here, if the boundary uses Query
- what counts as an anti-pattern or bypass

Boundary docs exist so future engineers and AI agents do not have to infer the
handoff contract from code shape or scattered comments.

### Shared Worth Category Pattern

Every Worth crate does not need the same categories populated at the same time,
but the category system should be stable and predictable across crates.

Preferred shared categories when they are earned:

- `foundations/`
- `features/`
- `boundaries/`
- `runtime-surfaces/`
- `certification/`

Crate-specific domain categories are encouraged when they preserve clearer
meaning than generic buckets.

### worth-kernel Docs Target

`worth-kernel` should eventually teach at least:

- `docs/README.md`
  - this README must explicitly say that `worth-kernel` docs are workflow-first
    and ergonomic-first at the public call site, while still linking downward
    to the stricter boundary and proof surfaces
- `docs/foundations/`
  - kernel-overview
  - execution-context-and-artifact-policy
- `docs/features/`
  - primitive-construction
  - shell-with-hole-construction
  - wire-body-construction
  - construction-simulation
  - construction-replay
- `docs/boundaries/`
  - kernel-to-spatial
  - worth-to-query
- `docs/runtime-surfaces/`
  - construction-results-and-diagnostics
- `docs/certification/`
  - primitive-construction-closeout

Milestone 4 only needs to populate the construction slice it actually ships,
but it should establish this organization model now.

### worth-spatial Docs Target

`worth-spatial` should eventually teach at least:

- `docs/README.md`
  - this README must explicitly say that `worth-spatial` docs are not
    ergonomic-first; they teach semantic construction meaning, binding truth,
    impossibility, and handoff contracts
- `docs/foundations/`
  - spatial-overview
- `docs/bindings/`
  - construction-time-birth-bindings
  - birth-completeness-and-impossibility
- `docs/boundaries/`
  - spatial-to-topo
- `docs/runtime-surfaces/`
  - birth-truth-artifacts
- `docs/certification/`
  - spatial-birth-closeout

Milestone 4 should document the construction-time admitted subset and defer the
broader rebinding world to later milestones explicitly.

### worth-topo Docs Target

`worth-topo` already has docs and should keep growing in the same
feature-oriented style. Milestone 4 should add docs only for the new
construction-authority surfaces it actually widens, for example:

- `docs/README.md`
  - this README must explicitly say that `worth-topo` docs are
    authority-oriented rather than shape-authoring-oriented
- topology construction authority execution
- topology construction certification
- any new construction-related projection or inspection surface

Milestone 4 must not force future agents to discover construction authority by
reading operator modules directly.

### worth-geom Docs Target

If Milestone 4 widens `worth-geom` enough to expose new public scaffold or
carrier surfaces, those should also be documented in the crate:

- `docs/README.md`
  - this README must explicitly say that `worth-geom` docs are pure
    geometry/scaffold docs and do not own topology, runtime, or construction
    authority semantics
- `docs/foundations/`
  - geometry-scaffold-overview
- `docs/features/`
  - construction-scaffold-generation
  - planar-carrier-contracts
- `docs/boundaries/`
  - geom-to-spatial-scaffold-contract

`worth-geom` should document only the public carrier and scaffold surfaces the
milestone actually teaches, not speculative later geometry families.

## Admitted Surface

- topology-certified primitive construction workflows authored by `worth-kernel`
- construction-time topology/geometry birth truth owned by `worth-spatial`
- generic admitted primitive and body families such as:
  - simplex-like solids
  - orthotope / box-like solids
  - prisms
  - pyramids
  - wire bodies
  - shell-with-hole bodies within the admitted planar construction class
- direct topology certification of constructed results
- direct construction-time spatial birth certification of constructed results
- branch-local and replayed histories over admitted primitive construction
  workflows

## Excluded Surface

- full boolean programs
- full rebinding closure and broad topology replacement semantics
- broad continuity inspection over mature binding histories
- full exact planar hostility beyond the construction-safe class
- broad curved or freeform construction classes
- full NURBS carrier and trim closure
- any construction path that requires `worth-topo` to interpret geometry
  directly

## Workflow Surface

Milestone 4 is not done because:

- one tetrahedron can be built
- one cube can be built
- one prism can be built
- one showcase shell-with-hole body can be built

It is only done when primitive construction operates generically over admitted
workflow classes such as:

- arbitrary admitted shell-building workflows over arbitrary admitted face
  counts
- arbitrary admitted body-construction workflows over the admitted primitive
  family ladder
- arbitrary admitted wire-body workflows
- arbitrary admitted shell-with-hole construction workflows inside the admitted
  class
- arbitrary admitted replayed and branch-local primitive construction histories

This milestone must close workflow classes, not primitive demos.

## Operator Closure

Milestone 4 must close the first admitted construction operator families across
the kernel/spatial/topology chain.

At minimum, the admitted operator families are:

- kernel-side primitive construction intent families
- spatial construction birth attachment families
- topology entity creation and membership composition operators required by the
  admitted primitive ladder
- topology-certified shell / wire / hole assembly operators required by the
  admitted workflow surface

The ambitious-but-real operator subset for Milestone 4 should be the operators
that admitted primitive construction actually needs, not the whole future
operator universe from [_docs/topo/operators-list.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/topo/operators-list.md).

Milestone 4 should therefore explicitly admit the following topology operator
families or their Worth-topology contract equivalents:

- entity and container lifecycle
  - `CreateBody`
  - `CreateLump`
  - `CreateShell`
  - `CreateFace`
  - `CreateLoop`
  - `CreateEdge`
  - `CreateVertex`
- ownership and containment composition
  - `InsertLump`
  - `InsertShell`
  - `AttachFace`
  - `AttachLoop`
  - `AttachEdge`
  - `AttachVertex`
  - `BindShellToRegion`
- loop and boundary wiring
  - `InsertEdgeIntoLoop`
  - `InsertVertexIntoEdge`
  - `SpliceLoopAtVertex`
  - `ReplaceLoopEdgeChain`
  - `SetLoopContainment`
  - `RecomputeLoopContainment`
- laminar / wire / shell composition
  - `CreateWireBody`
  - `AddWireEdge`
  - `CreateBoundaryLoop`
  - `MarkEdgeLaminar`
- shell-with-hole and solid-region composition
  - `CreateRegion`
  - `CreateOutsideRegion`
  - `BindOutsideRegion`
  - `AddRegionBoundaryFace`

Milestone 4 should **not** widen yet into:

- full radial-cycle or vertex-disk surgery
- boolean imprint or intersection surgery
- healing, sewing, or remove-and-heal families
- broad parametric trim-network editing
- transform, pattern, or import/export families

Those belong later even though the permanent skeleton should already have places
for them.

For every admitted family, Milestone 4 must certify:

- legal admitted cases
- hostile admitted cases
- explicit out-of-class exits
- replay parity
- exact rejection localization when blocked

Milestone 4 does not admit one giant untyped "build primitive" surface.
Every construction workflow must declare:

- kernel construction intent
- spatial birth contract
- topology lowering class
- expected admitted primitive family

or else clean-fail as unsupported.

## Spatial Construction Closure

Milestone 4 is the first milestone where `worth-spatial` must become real, but
its job is deliberately narrow.

This milestone does **not** ask `worth-spatial` to close full rebinding truth.
It asks `worth-spatial` to close **construction-time birth truth**.

That means Milestone 4 must define one explicit spatial construction seam that
answers:

- what geometric carriers or supports were used to author this body?
- which topology entities were born from which construction carriers?
- which construction-time geometry facts are authoritative enough to persist as
  birth truth for later milestones?
- which later spatial meanings remain intentionally deferred?

At minimum, the spatial birth seam must make explicit:

- face support birth for admitted planar primitive families
- vertex geometry birth for admitted construction classes
- edge or boundary carrier birth where later milestones will depend on identity
  continuity
- shell / loop / body association to the construction scaffold when that
  association matters for replay or diagnostics

And it must not silently pretend to have closed:

- general rebinding
- broad continuity reasoning
- full curved carrier truth
- general replacement semantics

Milestone 4 should therefore produce construction-born spatial artifacts that
are:

- authoritative for the admitted primitive birth lane
- narrow enough not to steal Milestone 5's job
- explicit enough that later booleans, fillets, and NURBS do not need to infer
  geometry meaning from topology archaeology

## Validator Closure

Milestone 4 must preserve and explicitly exercise:

- Milestone 1 topology truth and naming validator closure
- Milestone 2 derived-read and invalidation boundary closure as inspection and
  proof support, not construction authority
- Milestone 3 topology edit and replay closure as the substrate primitive
  construction consumes
- topology legality independent from geometry optimism
- spatial birth legality independent from topology legality

No admitted primitive workflow may bypass the validator ladder simply because
the primitive shape is "well-known."

Construction must prove at minimum:

- topology legality
- shell / wire / hole legality for the admitted workflow
- spatial birth contract completeness for the admitted workflow
- naming and replay closure where the primitive workflow is expected to be
  deterministic

The ambitious-but-real validator subset for Milestone 4 should likewise be the
subset that primitive construction truly relies on from
[_docs/topo/validators.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/topo/validators.md).

Milestone 4 should therefore explicitly require validator-family closure for:

- reference integrity and ownership
  - `ValidateNoDanglingHandles`
  - `ValidateOwnership`
  - `ValidateNoOrphans`
  - `ValidateBidirectionalLinks`
  - `ValidateAcyclicContainmentGraph`
- half-edge and loop wiring
  - `ValidateTwinSymmetry`
  - `ValidateNextPrevSymmetry`
  - `ValidateLoopClosure`
  - `ValidateLoopHasMinimumCardinality`
  - `ValidateNoDuplicateCoedgesInLoop`
  - `ValidateEdgeEndpointsMatchLoopVertices`
- face sanity
  - `ValidateFaceHasAtLeastOneLoop`
  - `ValidateNoFaceWithBrokenBoundary`
  - `ValidateFaceAdjacencyConsistency`
- shell / body / region closure
  - `ValidateShellWatertightness`
  - `ValidateBoundaryEdgesAreLaminarOnly`
  - `ValidateConsistentShellOrientation`
  - `ValidateInnerShellContainment`
  - `ValidateOutsideRegionConnectivity`
  - `ValidateNoRegionLeaks`
- determinism and identity guards
  - `ValidateCanonicalOrderingStable`
  - `ValidateHashStability`
  - `ValidateTieBreakerCoverage`

Milestone 4 should not claim closure yet on the broader validator families that
belong to later milestones, including:

- full radial-cycle and vertex-disk hostility
- broad parametric trim and p-curve closure
- tangent, coincidence, and predicate-divergence validation
- cache, BVH, and spatial acceleration validation
- importer soup-recovery validation

Those later validators should already have permanent homes in the long-term
crate skeleton, but they should not be overclaimed in Milestone 4.

## Replay Closure

Milestone 4 must prove:

- admitted accepted primitive construction histories replay identically
- admitted rejected primitive construction histories replay as the same typed
  rejection
- branch-local primitive construction preserves the same topology and birth
  truth semantics for the same local basis
- topology truth and spatial birth truth remain parity-linked under replay for
  admitted workflows

Primitive construction that only works once is not construction authority.

## Diagnostics Closure

Milestone 4 must emit diagnostics that identify:

- exact blocking boundary
- exact rejection class
- exact primitive family and workflow class
- exact topology scope affected
- exact spatial birth scope affected
- exact distinction between:
  - kernel construction intent failure
  - spatial birth-contract failure
  - topology legality failure
  - out-of-class primitive rejection

If a primitive fails and the runtime cannot say whether the failure was kernel
intent, spatial birth truth, or topology legality, the milestone is not
certified honestly.

## Determinism Closure

Milestone 4 must make explicit:

- canonical primitive parameter ordering rules where ordering should not change
  meaning
- canonical construction lowering rules where one admitted primitive class maps
  to one admitted topology/spatial birth workflow
- stable rejection classification for the same illegal or out-of-class request
- stable topology truth, spatial birth truth, and certification artifacts for
  the same admitted construction history

## Complexity / Proof Closure

Milestone 4 must name and prove:

- construction breadth contracts
- shell / wire / hole assembly breadth contracts
- spatial birth attachment breadth contracts
- topology certification breadth contracts
- replay and branch parity breadth contracts

Primitive construction must not hide broad scans behind ergonomic builders.
Whole-body or whole-history work, when unavoidable, must be surfaced explicitly
as declared breadth rather than optimistic locality.

## Allowed Debt

- full rebinding closure may remain deferred to Milestone 5
- broader continuity inspection over mature spatial histories may remain
  deferred to Milestone 5
- exact planar hostility beyond the admitted construction class may remain
  deferred to Milestone 6
- curved, analytic, and freeform carrier closure may remain deferred
- broad boolean pipelines may remain deferred

What may not remain implicit debt:

- kernel / spatial / topology authority separation for primitive birth
- topology-certified primitive workflow closure over the admitted family ladder
- construction-born geometry meaning for admitted primitives
- replay-safe and branch-safe construction semantics
- direct proof that later milestones can inherit construction truth instead of
  reconstructing it

## Phases

These phases are linear. They are not interchangeable, parallel buffet items,
or categories to make partial progress across simultaneously.

The implementation rule is:

- finish `Phase 1` before starting `Phase 2`
- finish `Phase 2` before starting `Phase 3`
- finish `Phase 3` before starting `Phase 4`
- finish `Phase 4` before starting `Phase 5`
- finish `Phase 5` before starting `Phase 5.5`
- finish `Phase 5.5` before starting `Phase 5.5.1`
- finish `Phase 5.5.1` before starting `Phase 5.5.2`
- finish `Phase 5.5.2` before starting `Phase 5.5.3`
- finish `Phase 5.5.3` before starting `Phase 5.5.4`
- finish `Phase 5.5.4` before starting `Phase 5.5.5`
- finish `Phase 5.5.5` before starting `Phase 5.5.6`
- finish `Phase 5.5.6` before starting `Phase 5.6`
- finish `Phase 5.6` before starting `Phase 6`
- finish `Phase 6` before starting `Phase 7`

Each phase must leave the system in a coherent, enforceable state for the next
phase. If a prerequisite is not met, the next phase is blocked by design.

Each phase below therefore includes:

- `Why this phase comes now`
- `Do this in this phase`
- `Do not start the next phase until`

### Phase 1: Freeze Construction Authority And Crate Boundaries

Define one honest primitive construction authority chain.

This phase exists to stop Milestone 4 from degenerating into "whatever code can
make a cube."

#### Why this phase comes now

Nothing else in the milestone is trustworthy until the authority split is
frozen. If this phase is skipped or only half-done, every later phase can hide
wrong responsibilities behind "temporary" helpers.

It must establish:

- `worth-kernel` as the construction orchestrator
  - `worth-spatial` as the owner of construction-time topology/geometry birth
    truth
  - `worth-topo` as the only topology authority
  - `worth-geom` as pure geometry and construction math
  - one explicit kernel -> spatial -> topology construction pipeline
  - one explicit Query runtime front door through `ForgeQueryWorkspace`
    instead of Worth-local runtime invention

This phase must make the following things impossible:

- topology construction that bypasses the canonical topology authority surface
- kernel helpers that directly own topology or spatial truth
- spatial birth logic living ambiguously in kernel or topology helpers
- geometry semantics leaking into `worth-topo`

#### Do this in this phase

- create the fresh `worth-kernel` and `worth-spatial` crate boundaries
- freeze the public facade responsibility for `worth-kernel`,
  `worth-spatial`, and `worth-topo`
- decide where primitive authoring starts, where birth truth starts, and where
  topology execution starts
- define the public runtime entry surface through `ForgeQueryWorkspace`
- define the compile-time anti-bypass posture for internal modules and facades

#### Do not start the next phase until

- an engineer can point to one authoring facade in `worth-kernel`
- an engineer can point to one admitted birth facade in `worth-spatial`
- an engineer can point to one topology authority facade in `worth-topo`
- there is no unresolved ambiguity about which crate owns construction intent,
  construction-time geometry meaning, or topology legality
- the Query runtime front door for Milestone 4 is named explicitly
- any required-later Query runtime family that is not yet admitted is recorded
  as an explicit gap artifact rather than being quietly treated as already
  supported
- the crate boundaries for primitive birth are explicit, named, and
  mechanically teachable

### Phase 2: Freeze Construction Scaffold And Lowering Contracts

Define the explicit construction scaffold that later phases will use.

#### Why this phase comes now

After authority is frozen, the next hard problem is the shape of the pipeline
itself. This phase defines the typed artifacts that move between layers.
Without that, later family workflows would be demos glued to implicit lowering.

This phase must introduce:

  - one kernel-owned primitive construction intent family
  - one geometry-owned construction carrier/scaffold family
  - one spatial birth contract family that maps scaffold meaning onto topology
    birth
  - one topology lowering family that consumes admitted spatial birth contracts
  - one explicit lowering path into Query public mutation surfaces, with
    `workspace.compose_graph(...)` as the required same-batch graph authoring
    surface whenever symbolic handles, mixed created/existing targets, or
    graph-lifecycle receipts are needed
  - one explicit "Query gap before Worth workaround" rule for every runtime
    boundary the scaffold depends on
  - one phase-typed construction progression such as:
    - raw primitive parameters
    - admitted primitive intent
    - geometric construction scaffold
    - admitted spatial birth plan
    - admitted topology lowering plan
    - executed construction artifact
    - certified construction artifact

At minimum, the scaffold must carry enough meaning to express:

- face-support birth
- vertex-position birth
- loop / shell / body assembly hints required by the admitted primitive ladder
- exact rejection when the geometry scaffold cannot lower honestly into the
  admitted topology class

This phase is the hard-problem-first phase for Milestone 4.
Without it, later phases would merely prove that a handful of shape generators
happen to work.

If this phase discovers that the scaffold needs runtime behavior Query does not
yet expose honestly, the phase must stop and widen Query rather than teaching a
private Worth substitute.

#### Do this in this phase

- define the primitive intent object shape
- define the geometric construction scaffold shape
- define the spatial birth-plan shape
- define the topology lowering-plan shape
- define the canonical construction artifact chain and its proof-bearing phase
  types
- define the default Query mutation/read/inspection surface used by each step
- define the explicit gap-escalation path when Query does not yet expose a
  required runtime boundary

#### Do not start the next phase until

- the common path and advanced path can both name the same phase chain
- out-of-order progression can be made unrepresentable in code
- one admitted scaffold can lower into one admitted spatial birth plan without
  ad hoc helper interpretation
- one admitted spatial birth plan can lower into one admitted topology plan
  without topology guessing geometry meaning
- primitive construction no longer means "generate data and improvise the
  rest"

### Phase 3: Freeze Admitted Primitive Family Construction Workflows

Close primitive construction over generic admitted family ladders instead of
showcase bodies.

#### Why this phase comes now

Only after the artifact chain is frozen does it make sense to populate real
workflow families. Otherwise every family would invent its own private
construction path and the milestone would close on examples instead of a
reusable substrate.

At minimum, the admitted workflow surface must cover:

- simplex-like solids
- orthotope / box-like solids
- prisms
- pyramids
- wire bodies
- shell-with-hole workflows inside the admitted class

These workflows must operate generically over:

- arbitrary admitted face counts
- arbitrary admitted loop cardinalities
- arbitrary admitted shell counts for the primitive family ladder
- arbitrary admitted branch-local and replayed construction histories

This phase must also freeze:

- typed out-of-class primitive rejection
- family-attributed construction breadth reporting
- primitive-family coverage reporting

#### Do this in this phase

- implement the admitted primitive family ladder on top of the phase-typed
  construction chain
- make every admitted family lower through the same kernel -> spatial -> topo
  story
- define the exact out-of-class rejection posture for unsupported requests
- define family-attributed breadth and coverage artifacts
- ensure the common path remains ergonomic while the advanced path remains
  inspectable

#### Do not start the next phase until

- every admitted family uses the canonical phase chain
- no admitted family requires bespoke runtime semantics outside the declared
  Query mapping
- out-of-class requests fail typed and locally
- family coverage is generic enough that one cube or one prism proves almost
  nothing by itself
- construction family closure is real enough that later work can build on
  families instead of examples

### Phase 4: Freeze Construction-Time Spatial Birth Truth

Turn the spatial birth seam into real authority for admitted primitive
construction.

#### Why this phase comes now

Only after real family workflows exist does it make sense to freeze what
construction-time geometry meaning must persist. Otherwise we would either
over-generalize the birth seam before real workflows exist, or under-specify it
and force Milestone 5 to guess what these constructions meant.

This phase exists because later booleans, fillets, and curved work must not be
forced to infer what constructed topology meant geometrically.

It must prove:

- constructed topology entities are born with explicit admitted spatial meaning
- the birth meaning is authoritative for the admitted primitive workflow
- topology truth and spatial birth truth remain distinct but linked
- the milestone does not overclaim broad rebinding or continuity closure

Required direct outputs for this phase include:

  - construction-born spatial digest rows
  - topology-to-birth mapping rows
  - construction birth completeness rows
  - typed rejection rows for impossible birth attachments
  - one canonical construction artifact that binds:
    - primitive intent identity
    - scaffold digest
    - spatial birth digest
    - topology lowering digest
    - Query runtime receipt digests
    - certification digests
  - Query-owned projection-consumption receipts or equivalent typed fact
    surfaces whenever construction diagnostics need identity, membership,
    source-reference, or continuity facts from read or write artifacts

#### Do this in this phase

- make construction-born face/edge/vertex meaning explicit for the admitted
  workflow set
- bind construction-born spatial meaning into one canonical construction
  artifact family
- define completeness versus impossibility for birth attachment
- define which spatial truths are authoritative now and which are still
  intentionally deferred
- ensure diagnostics and certification consume spatial truth through Query-owned
  fact and inspection surfaces where appropriate

#### Do not start the next phase until

- admitted primitives are no longer geometry-blind births
- the canonical construction artifact can explain scaffold, birth, topology,
  Query receipt, and certification truth together
- impossible birth attachments reject typed and locally
- the milestone is no longer relying on later rebinding work to explain what
  current constructions meant geometrically

### Phase 5: Freeze Replay, Branch, Diagnostics, And Hostile Family Proof

Turn primitive construction into a hostile certification target, not a demo
feature.

#### Why this phase comes now

Replay, branch parity, diagnostics, and hostile proof only mean something after
the authority split, artifact chain, family workflows, and birth truth are
stable. This final phase certifies the system we built in the earlier phases;
it must not be used to discover what the architecture should have been.

Milestone 4 must directly inherit the relevant parts of:

- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
- [topo-test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/topo-test-requirements.md)

At minimum, hostile Milestone 4 certification must exercise:

- primitive family closure over admitted family ladders
- primitive-body topology closure
  - accepted and rejected replay parity
  - accepted and rejected branch-local parity
  - shell-with-hole and wire-body workflow hostility
  - topology validator locality for constructed bodies
  - construction-breadth and certification-breadth proof
  - support-matrix, basis, inspection, and projection-consumption parity for
    the Query runtime surfaces Milestone 4 teaches publicly
  - explicit failure when a construction workflow would require a Worth-local
    runtime workaround instead of an admitted Query surface

Named hostile scenario families for Milestone 4 should include, at minimum:

- `PrimitiveFamilyParameterSweep`
- `ShellWithHoleConstructionHostility`
- `WireBodyConstructionParity`
- `OutOfClassPrimitiveCleanFail`
- `ConstructionBirthParity`

The marquee showpiece suite for Milestone 4 should be:

- `PrimitiveConstructionCorpusReplaySiege`

This suite should exist to prove that primitive construction is not a handful
of body generators but a real authority-carrying runtime path.

It should execute the same authored primitive corpus across all admitted
Milestone 4 family ladders through:

- direct current-head construction
- branch-local construction from the same authoritative base
- replayed construction from recorded history
- shuffled-but-equivalent authoring order where the contract says order should
  not matter
- mixed accepted and rejected workloads in the same run

The corpus should include, at minimum:

- simplex-like solids:
  - smallest admitted
  - generic admitted
  - hostile high-cardinality admitted members
- orthotope / box-like bodies with multiple aspect-ratio classes
- prisms over multiple side counts
- pyramids over multiple side counts
- wire bodies over multiple edge counts
- shell-with-hole bodies over multiple outer-face and inner-hole counts
- explicit out-of-class requests for every admitted family

The suite must prove that for every admitted accepted case, the direct,
branch-local, and replayed lanes produce the same:

- `primitive_construction_digest`
- `construction_birth_truth_digest`
- topology legality outcome
- branch-local parity outcome
- replay parity outcome
- certification artifact rows

And for every rejected case, the same:

- typed rejection class
- rejection locality
- failure digest
- replayed rejection outcome
- branch-local rejection outcome

It should also expose explicit breadth evidence for:

- construction breadth
- spatial birth attachment breadth
- topology certification breadth

If any of those widen unexpectedly, the suite should emit explicit widening or
fallback evidence rather than silently passing on timing alone.

The remaining Query proof/reporting surfaces for this phase are concrete and
must not be treated as optional polish. Before Phase 5 can close, it must
directly ship:

- `query_graph_composition_parity_report`
- `query_existing_truth_binding_report`
- `query_projection_consumption_receipt_report`
- `query_boundary_gap_register`
- `query_no_local_runtime_workaround_audit`
- `PrimitiveConstructionCorpusReplaySiege`

These are not all the same kind of unfinished work:

- `BranchPreview` runtime admission is already required and must already be
  real in the implemented Worth construction path by the time this phase is
  closing
- the Query items above are the remaining proof/reporting closeout surfaces
  that certify how Worth uses Query, not a license to re-open runtime
  authority design late
- broader later-domain capabilities such as full spatial rebinding, mature
  continuity closure, curved-carrier closure, and broad boolean pipelines
  remain intentionally deferred beyond Milestone 4

#### Do this in this phase

- build the hostile corpus and parity suites on top of the canonical artifact
  family
- prove accepted and rejected parity across current-head, branch-local, and
  replayed execution
- freeze `BranchPreview` as an already-admitted Worth-to-Query runtime boundary
  requirement for the branch-local construction flows Milestone 4 teaches
- build and certify `query_graph_composition_parity_report`
- build and certify `query_existing_truth_binding_report`
- build and certify `query_projection_consumption_receipt_report`
- build and certify `query_boundary_gap_register`
- build and certify `query_no_local_runtime_workaround_audit`
- prove Query surface parity for authoring, basis, preview, inspection,
  graph-composition, existing-truth reuse, and projection-consumption use
- emit the final machine-checkable closeout reports
- emit the Query gap register and anti-workaround audit as first-class closeout
  artifacts

#### Do not declare the milestone complete until

- admitted primitive construction is certified as a workflow class across
  authority, spatial birth, topology legality, replay, and branch pressure
- the showpiece corpus suite is green
- `BranchPreview` is already really admitted in the Worth runtime paths that
  claim branch-local construction support, not merely visible in the support
  contract
- the phase-typed chain, canonical artifact, and Query anti-bypass surfaces are
  all proven in closeout artifacts
- the remaining Query closeout surfaces are shipped as named artifacts:
  - `query_graph_composition_parity_report`
  - `query_existing_truth_binding_report`
  - `query_projection_consumption_receipt_report`
  - `query_boundary_gap_register`
  - `query_no_local_runtime_workaround_audit`
- no remaining required runtime gap is being hidden by a Worth-local workaround

### Phase 5.5: Freeze Primitive Realization Stability And Conditioning Truth

Turn primitive realization from hidden geometry rescue logic into explicit,
certified kernel truth.

#### Why this phase comes now

Phase 5 proves that the current primitive construction path is real, hostile,
and Query-honest. That does not automatically mean the primitive realization
substrate is principled enough for the later kernel.

The siege already taught the milestone an important lesson: even when the
authority chain, replay parity, branch parity, and Query proof surfaces are
real, the underlying primitive realization path can still be too
implementation-shaped. If one family only survives near-threshold inputs
because its geometry helper happened to normalize a better vector, while
another family fails because its helper happened to normalize a worse one, then
the milestone has built a working path without yet building a granite
foundation.

This phase exists to correct that before documentation freezes the wrong story.

Milestone 4 must stop treating primitive realization like:

- semantic primitive family
- one hidden implementation path
- maybe a quiet numerical fallback if the helper gets nervous

and instead promote the following to first-class truth:

- semantic primitive family
- sanctioned realization strategy
- conditioning witness
- admitted stability class
- typed exhaustion and rejection after sanctioned strategies are exhausted

That is the shape later booleans, blends, offsets, sweeps, healing, and curved
carrier work can actually inherit.

This phase is intentionally later than Phase 5, not because it is optional, but
because the hostile corpus, parity lanes, and Query surfaces from Phase 5 are
what expose where the primitive substrate is still too implicit. Phase 5 shows
us the hard cases. Phase 5.5 turns those hard cases into architecture.

#### What this phase must establish

Milestone 4 primitive construction must grow a permanent realization model that
is broader than any one family such as prisms or pyramids.

At minimum, the model must separate:

- `PrimitiveSemanticIntent`
  - the primitive family and semantic parameters
  - not the realization algorithm
- `PrimitiveRealizationStrategy`
  - sanctioned realization modes such as:
    - `DirectWorld`
    - `LocalNormalized`
    - `ExactSupport`
    - `RejectedIllConditioned`
  - the exact names may differ, but the concept must be explicit
- `PrimitiveConditioningWitness`
  - the measurable evidence that explains why one strategy was sufficient or
    insufficient
  - for example:
    - scale and span summaries
    - aspect or skinniness indicators
    - cross-product or support-normal magnitude classes
    - transform normalization scale
    - exactness or escalation usage markers
- `PrimitiveStabilityClass`
  - such as:
    - `StableDirect`
    - `StableAfterEscalation`
    - `RejectedBelowConditioningFloor`
- `PrimitiveRealizationReport`
  - one certified report that binds:
    - semantic family
    - strategy selected
    - strategies attempted
    - conditioning witness summary
    - stability class
    - typed exhaustion or rejection reason when blocked

The key rule is:

No primitive family may hide numerical rescue inside a geometry helper.

If primitive birth requires local normalization, exact-support reconstruction,
or any other sanctioned escalation, that must survive as artifact truth and
certification truth all the way out of the stack.

#### Crate ownership in this phase

- `worth-geom`
  - owns primitive realization strategies
  - owns conditioning witnesses
  - owns the decision logic for when direct realization is sufficient, when
    sanctioned escalation is required, and when the input is honestly
    ill-conditioned
- `worth-spatial`
  - owns preservation of realization provenance into construction-born spatial
    truth
  - owns the birth-facing explanation of whether admitted construction-time
    geometry meaning was born directly or only after sanctioned escalation
- `worth-kernel`
  - owns the canonical artifact and result surfaces that expose realization
    strategy, conditioning witness summary, stability class, and exhaustion
    truth
  - owns the upgraded siege and closeout reports that prove this information is
    real, not debug-only
- `worth-topo`
  - remains topology-authoritative and geometry-free
  - must not become the owner of primitive conditioning or realization
    semantics
  - may consume the results of realized birth truth, but may not reinterpret
    geometric stability locally

#### Query posture in this phase

This phase must not invent a second runtime story for realization stability.
`forge-query` should make the phase easier, not incidental.

The realization and conditioning truth from this phase must travel through the
same canonical Query-backed construction artifact and inspection path that
Phase 5 already hardened.

At minimum, this phase must preserve:

- one canonical runtime-backed artifact boundary for realization truth
- one inspection-backed path for reading realization and stability facts back
- one branch / preview / replay parity story over realization truth
- one anti-workaround audit posture that forbids hiding realization rescue in
  private Worth-local runtime state

This phase should therefore use Query to make these questions machine-checkable:

- did the same primitive request choose the same sanctioned realization
  strategy across current-head, replay, and branch-local lanes?
- did a case that required escalation advertise that fact in its artifact and
  certification surfaces?
- did a rejected ill-conditioned case reject only after sanctioned strategies
  were exhausted?

Query helps because it already provides:

- one canonical artifact boundary
- one public inspection surface
- one replay and branch/preview runtime posture
- one public anti-bypass language

This phase must reuse those strengths rather than inventing a second
"conditioning runtime" under the table.

#### Do this in this phase

- define the primitive realization strategy model in `worth-geom`
- define the primitive conditioning witness model in `worth-geom`
- define the primitive stability classification model
- make primitive family realization select among sanctioned strategies rather
  than one hidden implementation path
- define typed exhaustion and rejection after sanctioned strategies are
  exhausted
- preserve realization provenance into the spatial birth truth chain
- widen the canonical construction artifact so realization strategy,
  conditioning witness summary, stability class, and exhaustion truth are part
  of the sanctioned result surface
- widen the hostile siege so it proves all three classes of result:
  - direct-stable
  - escalation-stable
  - exhausted-and-rejected
- add threshold-near admitted/rejected pairs that certify exact class
  boundaries rather than only obvious invalid inputs
- add family-local cases that force different rejection localities under the
  same primitive family where that distinction is real
- make any sanctioned precision fallback mechanically observable and certified
  instead of helper-local
- ensure no primitive family can silently widen its own numerical floor without
  updating the realization and conditioning reports

#### This phase must replace, not supplement

Phase 5.5 is not allowed to layer a principled realization model on top of the
old implicit one and leave both alive.

This phase must explicitly replace:

- primitive-family-specific hidden realization logic that decides success or
  failure without emitting realization strategy, conditioning witness, and
  stability truth
- helper-local numerical rescue paths that can succeed or fail without leaving
  artifact-visible provenance
- family-specific threshold fixtures that merely encode today's accidental
  implementation floor instead of a principled conditioning and exhaustion
  policy
- artifact surfaces that collapse primitive realization into only
  admitted-versus-rejected outcome without exposing how the result was realized
- siege assertions that compare only outcome digests while ignoring the
  realization and stability class that produced them
- duplicate or drifting stability judgments across crates

This phase must cut, not merely de-emphasize:

- silent `normalize-and-hope` behavior in geometry helpers
- hidden local-frame or precision-escalation rescue that does not survive into
  the canonical artifact
- any family-local rule that silently hardens or softens a conditioning floor
  without updating the sanctioned realization-policy layer
- any test whose real meaning is only "this particular threshold currently
  passes"

After this phase, the new source of truth must be:

- one realization-policy layer in `worth-geom`
- one conditioning witness model
- one stability classification model
- one canonical artifact path carrying realization truth through
  geom -> spatial -> kernel certification surfaces
- one siege story that proves direct-stable, escalation-stable, and
  exhausted-and-rejected behavior across runtime lanes

#### Do not start the next phase until

- every admitted primitive artifact carries explicit realization strategy and
  stability truth
- every sanctioned fallback or escalation path is visible in artifact and
  certification surfaces
- primitive rejection after ill-conditioning is typed and certified as
  exhaustion of sanctioned strategies, not as vague geometry failure
- the siege proves:
  - direct-stable cases
  - escalation-stable cases
  - exhausted-and-rejected cases
- at least one near-threshold family pair demonstrates that the system
  distinguishes:
  - direct success
  - escalated success
  - exhausted rejection
- no primitive family still relies on hidden helper-local numerical rescue
- the Query-backed artifact, inspection, replay, branch, and anti-bypass
  surfaces all preserve realization and stability truth without introducing a
  second runtime story
- later docs would describe a principled realization substrate rather than a
  pile of implementation luck

### Phase 5.5.1: Freeze The Primitive Authoring DX Surface Before The Compound Siege

Turn primitive construction authoring into a scalable spec -> intent -> phase
progression surface before the compound adversarial suite depends on it.

#### Why this phase comes now

Phase 5.5 made realization strategy, conditioning, stability, and exhaustion
explicit kernel truth. That is necessary, but it is not sufficient.

If primitive authoring still teaches callers to memorize positional
constructors, then the kernel will carry principled lower layers under a weak
human-facing surface. That is exactly the wrong substrate for:

- compound adversarial suite authoring
- future primitive family growth
- later boolean, blend, sweep, and curved-carrier intent definition
- public docs that are supposed to teach architecture instead of folklore

`dx_laws.md` is decisive here:

- object specs encode shape
- builders encode progression
- semantic intent must be first-class
- the common path should read like intent
- the advanced path should expose the next lower layer
- friendly APIs must lower into inspectable plans instead of hiding the proof
  chain

Primitive construction therefore needs a real authoring model, not just nicer
constructors.

This phase belongs before Phase 5.6 because the compound primitive suite should
be authored on top of the same scalable primitive surface that the rest of the
kernel will inherit. If Phase 5.6 has to invent its own fixture DSL to stay
readable, then the public primitive DX is not good enough yet.

#### What this phase must establish

Primitive authoring must become a three-layer model:

1. family-local object spec
2. semantic primitive intent
3. explicit advanced path for phase progression and execution boundaries

The long-term shape must be:

- object specs show the whole family definition at once
- semantic intent is the portable unit of meaning
- admission, scaffold generation, lowering, and execution remain later phases
  instead of being hidden inside a cute constructor

At the call site, the common path should trend toward:

```rust
let pyramid = RegularPyramidSpec {
    center: [0.0, 0.0, 0.0],
    sides: 3,
    radius: 1.0e-200,
    height: 1.0e-200,
};

let intent = PrimitiveConstructionIntent::regular_pyramid(pyramid);
let result = prepare_primitive_construction_result(intent)?;
```

while the advanced path should still be able to read like:

```rust
let intent = PrimitiveConstructionIntent::regular_pyramid(pyramid);
let admitted = intent.admit()?;
let scaffold = admitted.scaffold()?;
let prepared = scaffold.prepare_result(&mut workspace)?;
let artifact = prepared.artifact();
```

The exact type names may differ, but the semantic split must survive.

#### Required family-local object specs

At minimum, this phase must define explicit family-local object specs for the
admitted primitive ladder:

- `SimplexSolidSpec`
- `OrthotopeSpec`
- `RegularPrismSpec`
- `RegularPyramidSpec`
- `WireBodySpec`
- `ShellWithHoleSpec`

These specs must:

- use named fields rather than positional meaning
- preserve genuine family-local semantics instead of flattening them
- remain readable in hostile suites and family-boundary drift cases
- support future family-local widening without exploding unrelated call sites

This phase must not replace family-local specs with one flattened generic bag
such as:

- `PrimitiveSpec { family, center, scale, sides, radius, height, loops, ... }`

That is not scalable DX. It is an implementation leak.

#### Required semantic intent surface

This phase must introduce one first-class primitive intent layer above raw
request plumbing.

The key distinction is:

- family-local specs describe the authored shape
- semantic intent describes what the caller means
- lowered request, admission, scaffold generation, and execution remain
  downstream phase transitions

That intent layer must be usable across:

- direct result preparation
- replay and branch-local parity setup
- inspection-backed certification setup
- corpus-row and hostile-suite authorship
- future simulation, queueing, or serialized replay if the kernel grows there

The portable unit should therefore be the primitive intent object, not the
positional constructor invocation.

#### Common path versus advanced path

The public DX must explicitly distinguish:

- common path:
  - reads like semantic intent
  - does not force the caller to think in terms of digests, geometry bits, or
    scaffold plumbing
- advanced path:
  - exposes the next lower boundary where caller responsibility changes
  - keeps admission, scaffold generation, artifact preparation, and runtime
    execution reachable and inspectable

This is the DX-law split the primitive layer must teach from here forward.

#### Builders versus object specs

This phase must follow the object-spec versus builder rule rigorously:

- family specs should be ordinary structs or equivalent object-spec surfaces by
  default
- fluent builders are allowed only where they encode real progression or staged
  hostile-fixture assembly
- a builder chain must not replace a clearer object spec when the caller is
  defining the whole primitive at once

So the primary taught surface should be:

- family-local spec object
- semantic intent from that spec

not:

- a mega-chain that happens to set fields one by one

#### Test-authoring DX requirements

This phase is not complete unless it makes the hostile suite world easier to
author honestly.

At minimum, the new authoring model must support:

- readable one-row hostile fixture setup without constructor archaeology
- stable named-field diffs when a primitive case changes
- family-local fixture presets for threshold, drift, collapse, escalation, and
  exhaustion rows where those presets are useful
- reuse of the same authored primitive across:
  - direct
  - replay
  - branch / preview
  - inspection
  - certification

The intended result is that Phase 5.6 compound rows can be expressed as obvious
authored specs rather than opaque positional tuples.

#### Required implementation posture

This phase may keep existing positional constructors as thin convenience
wrappers or migration seams where useful, but they must stop being the main
taught primitive language.

The primary truthful surface must become:

- family-local specs
- semantic intent created from those specs

Positional constructors may remain temporarily, but they must not remain:

- the main hostile-suite authoring surface
- the main public-doc teaching surface
- the main expansion seam for future primitive families

#### This phase must replace, not supplement

Phase 5.5.1 must replace, not merely supplement:

- positional primitive constructors as the main authoring language
- family meaning hidden in argument order
- test fixtures that encode primitive semantics through constructor memorization
- per-suite authoring helpers that bypass the canonical primitive language
- family growth that widens constructor signatures instead of widening
  family-local specs

#### Do not do these things

- do not build one mega `PrimitiveBuilder` that mixes every family into one
  option bag
- do not flatten family-local semantics into anonymous generic fields
- do not hide lower execution boundaries behind friendly helpers that secretly
  lower, admit, and execute everything at once
- do not make hostile suites depend on memorizing argument position
- do not introduce fluent builders where a plain object spec is clearer

#### Required deliverables

At minimum, this phase must produce:

- one explicit family-local spec layer for the admitted primitive ladder
- one semantic primitive intent surface above raw request plumbing
- one common-path primitive construction surface that accepts semantic intent
  cleanly
- one advanced-path surface that still exposes phase progression honestly
- one compatibility or migration story for existing positional constructors
- one hostile-suite authoring surface good enough that Phase 5.6 can use specs
  and intent instead of positional constructor archaeology

#### Do not start the next phase until

- the primary public primitive authoring language is no longer positional-first
- a future engineer or AI agent can read a hostile primitive case and
  understand its shape without memorizing constructor argument order
- family-local primitive growth can happen by widening family-local specs
  instead of widening one flattened constructor surface
- the common path reads like semantic intent
- the advanced path still exposes lower execution boundaries honestly
- Phase 5.6 can be authored on top of this surface without inventing a
  parallel fixture DSL
- future docs for primitive construction can teach:
  - object spec
  - semantic intent
  - common path
  - advanced path
  instead of teaching a pile of positional constructors

### Phase 5.5.2: Freeze Spatial Intent, Placement, Motion, And Anchor Semantics Before The Compound Siege

Turn primitive placement, movement, rotation, reorientation, and prepositional
spatial language into one planned semantic system before compound adversarial
suites and later persistent-naming work depend on it.

#### Why this phase comes now

Phase 5.5.1 fixes the primitive authoring DX around intrinsic family meaning:

- family-local specs
- semantic primitive intent
- common path versus advanced path

That is necessary, but it still leaves one dangerous blind spot if it stops
there:

- primitive creation can still smuggle world placement into creation fields
- movement and rotation can still be deferred into future ad hoc transform APIs
- persistent naming can still get forced to attach to unstable world-born
  topology instead of semantic anchors
- compound hostile tests can still quietly inherit canonical-world assumptions
  instead of proving a real spatial intent substrate

`MENTALITY.md` is decisive here:

- this is a foundation problem, not a later polish problem
- later booleans, blends, sweeps, import healing, and branch-local edits will
  inherit whatever spatial intent substrate we freeze now
- later analytic-curved carriers, freeform / NURBS carriers, chamfers, fillets,
  and blend-junction feature families will also inherit it
- if creation, placement, and movement are not separated before those later
  features exist, the codebase will grow around the wrong semantic center and
  force painful retrofits

This phase therefore belongs before Phase 5.6 because the compound primitive
suite should pressure the same spatial-intent language that future product code
will use. If Phase 5.6 has to invent per-suite placement helpers, ad hoc axis
fields, or transform folklore just to author hostile workloads, then the public
spatial-intent surface is not ready yet.

#### Adversarial constraint for this phase

Worth must be able to express all of these as different semantic acts without
collapsing them into the same raw transform story:

- create a primitive with intrinsic shape meaning only
- place a newly created primitive into world or reference-frame context
- move an already-existing shape without pretending its intrinsic identity
  changed
- rotate or reorient an already-existing shape without losing semantic anchor
  identity
- attach later relational or prepositional properties without widening family
  creation specs into transform bags
- preserve anchor identity and replay parity across direct, replay,
  branch-local, inspection, and certification surfaces
- preserve the same spatial-intent meaning whether the target is:
  - a primitive body
  - an analytic curved carrier
  - a freeform / NURBS carrier
  - a later chamfer, fillet, or blend-owned feature result

The naive implementation that this phase must rule out is:

- primitive specs that mix shape fields and world-placement fields
- one generic transform object pushed through every call site as the public DX
- movement and rotation added later as unrelated helper APIs
- persistent naming attached to incidental generated topology ids instead of
  semantic anchors
- motion semantics that only work for rigid primitive examples and silently fork
  once surfaces, fillets, or freeform features arrive
- anchor semantics that only know about face / edge / vertex ordinals and have
  no path to parameter-space, workplane, tangent, normal, or feature-owned
  anchors later

#### What this phase must establish

Spatial intent must be planned as one layered semantic system:

1. intrinsic shape truth
2. embedding or placement truth
3. edit intent for already-existing shapes
4. relational or prepositional truth
5. semantic anchor truth for persistent naming
6. geometry-carrier anchor truth for curved and freeform future work
7. history-bearing motion truth for replay, merge, and intent inspection later

The long-term shape must be:

- intrinsic shape answers:
  - what shape is this?
- placement answers:
  - where is this shape embedded?
  - what canonical frame has been mapped into world or reference context?
- edit intent answers:
  - is this creating a new shape, moving an old one, rotating it, or
    reorienting it?
- prepositional truth answers:
  - what is this on, toward, inside, aligned with, relative to, or offset from?
- anchor truth answers:
  - what stable semantic parts survive placement and later edits?
- carrier-anchor truth answers:
  - what point, tangent, normal, frame, or parameter-space anchor survives when
    topology-local names are not enough?
- history-bearing motion truth answers:
  - what authored operation happened, on what anchor basis, with what preserved
    or ambiguous continuity story?

This phase must explicitly freeze these intent families as distinct semantic
surfaces rather than one flattened transform vocabulary:

- `Create`
- `Move`
- `Rotate`
- `Reorient`
- `Offset`
- later planned but not necessarily implemented here:
  - `Mirror`
  - `Pattern`
  - `Project`
  - `Align`
  - `Attach`

These must be treated as authored intent families, not as convenience wrappers
over one shared "apply transform" primitive. The plan must preserve the idea
that:

- `Create` publishes new intrinsic shape truth plus embedding intent
- `Move` changes embedding of existing truth
- `Rotate` changes orientation relative to an anchor basis
- `Reorient` changes frame interpretation or directional alignment
- `Offset` changes relative displacement or stand-off semantics

That distinction is required so later history, merge, replay, and interaction
surfaces can inspect what was meant rather than infer it from a final matrix.

#### Common-path DX the milestone should be aiming toward

The public language should read like authored spatial meaning, not transform
algebra.

Representative creation shape:

```rust
let intent = Primitive::pyramid(
    RegularPyramid {
        sides: 5,
        radius: 2.0,
        height: 4.0,
    },
)
.at([10.0, 0.0, 3.0])
.facing([0.0, 1.0, 1.0]);
```

Representative edit shapes:

```rust
let move_it = Move::shape(shape_id).to([12.0, 0.0, 3.0]);
```

```rust
let rotate_it = Rotate::shape(shape_id)
    .about(shape_id.anchor("apex"))
    .toward([0.0, 0.0, 1.0]);
```

```rust
let orient_it = Reorient::shape(shape_id)
    .aligned_with(workplane_id);
```

Representative prepositional or relational shape:

```rust
let placement = Move::shape(shape_id)
    .so(shape_id.anchor("base_face"))
    .lies_on(workplane_id);
```

```rust
let pointing = Reorient::shape(shape_id)
    .so(shape_id.anchor("apex"))
    .points_toward(target_point);
```

The milestone does not need to freeze every final method name here, but it must
freeze the public language class:

- prepositional verbs on top
- canonical placement or frame substrate underneath
- no matrix-first or transform-first public teaching surface
- no primitive-only vocabulary trap that leaves surfaces, blends, or feature
  results needing a second spatial language later

#### Required semantic vocabulary

This phase must explicitly plan a reusable spatial vocabulary broad enough for
primitives now and later shapes beyond primitives.

At minimum the public design must account for:

- location:
  - `at(...)`
  - `to(...)`
  - `from(...)`
  - `between(...)`
- direction and orientation:
  - `facing(...)`
  - `toward(...)`
  - `aligned_with(...)`
  - `parallel_to(...)`
  - `perpendicular_to(...)`
- relational placement:
  - `on(...)`
  - `in(...)`
  - `inside(...)`
  - `relative_to(...)`
- displacement:
  - `offset_by(...)`
  - `translated_by(...)`
  - `rotated_about(...)`
- constraint-shaped phrasing:
  - `so(anchor).lies_on(...)`
  - `so(anchor).points_toward(...)`
  - `so(anchor_a).matches(anchor_b)`

The point of this vocabulary is not API theater. It is to preserve one common
semantic language across:

- primitive creation
- later edit intents
- persistent naming
- hostile test authoring
- future sketch, surface, sweep, import, and boolean work
- future NURBS, chamfer, fillet, and blend-junction work

#### Canonical local frames and semantic anchors

This phase must plan primitive placement on top of canonical local shape truth,
not on top of already-world-embedded geometry.

Every admitted primitive family must therefore grow a canonical local frame and
semantic anchor model.

Examples that the plan must account for:

- `RegularPyramid`
  - `apex`
  - `base`
  - `base_face`
  - `base_edge(i)`
  - `base_vertex(i)`
  - `side_face(i)`
- `RegularPrism`
  - `top_face`
  - `bottom_face`
  - `side_face(i)`
  - `top_vertex(i)`
  - `bottom_vertex(i)`
- `Orthotope`
  - `min_x_face`
  - `max_x_face`
  - `min_y_face`
  - `max_y_face`
  - `min_z_face`
  - `max_z_face`
  - canonical corners and edges
- `SimplexSolid`
  - `vertex(i)`
  - `face(i)`
  - canonical local vertex ordering

The plan must be explicit that:

- the primitive is born in canonical local space
- semantic anchors are assigned in canonical local space
- placement embeds that anchored local truth into world or reference context
- movement or reorientation later must preserve anchor identity rather than
  recreating topology folklore from scratch

That is the foundation that later persistent naming must inherit.

This phase must also say explicitly that primitive anchors are only the first
anchor class, not the whole future anchor story.

The longer-lived shared anchor model must have room for:

- semantic part anchors:
  - `apex`
  - `base_face`
  - `side_face(i)`
- topological anchors:
  - `vertex(i)`
  - `edge(i)`
  - `face(i)`
- frame anchors:
  - local frame origin
  - local primary axis
  - workplane basis
- geometric anchors:
  - point-on-curve
  - tangent
  - normal
  - centerline
- parameter-space anchors:
  - `u`
  - `v`
  - trim endpoint
  - seam anchor
- feature-owned anchors:
  - fillet spine
  - chamfer support edge
  - blend junction anchor

The point is not to fully solve those later domains here. The point is to make
it impossible for this phase to accidentally freeze a primitive-only anchor
model that later curved and blend milestones would have to replace.

#### Required internal substrate

The public DX should be prepositional and semantic, but this phase must also
freeze the internal substrate that those surfaces lower into.

At minimum the internal design must account for:

- one reusable placement or frame substrate
- one reusable orientation substrate
- one reusable anchor-reference substrate
- one reusable edit-intent substrate for existing shapes
- one clear lowering point from semantic prepositional language into canonical
  placement / motion plans
- one explicit reference-frame substrate that can name:
  - world frame
  - workplane frame
  - shape-local frame
  - feature-local frame
  - carrier-derived tangent / normal frame
- one explicit motion-plan substrate that can survive replay and later history
  inspection as authored meaning instead of final transform residue

This phase may plan types like:

- `Placement`
- `Orientation`
- `CanonicalFrame`
- `AnchorRef`
- `SpatialIntent`
- `PlacementConstraint`

or better names if implementation earns them.

What matters is not the exact nouns. What matters is that:

- raw transforms are not the primary public semantic unit
- prepositional language lowers into one inspectable substrate
- the same substrate can be reused beyond primitive creation later
- curved carriers and blend features do not need to invent a second spatial
  anchoring language

#### Required ownership split

This phase must not leave ownership implied. The spatial-intent system is not a
primitive helper layer and not a construction-local staging area. It is a
permanent cross-domain substrate with an explicit crate split.

The required ownership posture is:

- `worth-kernel`
  - owns authored public intent verbs and orchestration-facing façades
  - examples:
    - `Create`
    - `Move`
    - `Rotate`
    - `Reorient`
    - `Offset`
  - owns the public common path, advanced path, artifact path, and report path
    that consume spatial intent
- `worth-spatial`
  - owns spatial meaning, not just primitive implementation details
  - examples:
    - placement semantics
    - anchor semantics
    - reference-frame semantics
    - motion-plan semantics
    - lowering from authored spatial intent into construction- and
      certification-consumable spatial plans
  - this is the permanent home of the shared spatial-intent substrate
- `worth-geom`
  - owns only the math and coordinate machinery needed by the spatial layer
  - examples:
    - vector math
    - frame math
    - transform math
    - local / world coordinate conversion
  - it must not own authored spatial-intent semantics
- `worth-topo`
  - consumes anchored and spatially-lowered truth where needed
  - it must not own placement, anchor, or motion language

The architectural shorthand is:

- kernel owns the verbs
- spatial owns the meaning
- geom owns the math
- topo consumes the results

That split is required so later curves, freeform carriers, fillets, and
history-bearing motion can reuse the same system without importing the wrong
crate authority.

#### Required directory topology

This phase must also freeze the intended physical topology so future
implementation does not create a structurally dishonest home for the subsystem.

The shared substrate must not be organized under:

- `primitives/`
- `construction/`
- `tests/`
- any milestone-, prompt-, or provenance-shaped folder

except for family-specific adapters that consume the shared substrate.

The intended permanent topology is:

- in `worth-spatial`
  - one dedicated spatial-intent subtree that owns the reusable substrate
  - recommended structural axes include:
    - placement
    - motion
    - anchors
    - frames
    - lowering
- in `worth-kernel`
  - one façade-facing subtree for authored spatial verbs and higher-level
    orchestration entry points
  - this may sit beside construction rather than inside it if that better
    preserves the ownership boundary
- in `worth-geom`
  - math subtrees only, with no authored semantic verbs

The exact final names may evolve if implementation earns a better classification,
but the structural law may not:

- the shared spatial-intent substrate must have a permanent domain home
- primitive construction may be one client of that domain
- primitive construction may not become the owner of that domain
- no "temporary for now" construction-local or primitive-local staging tree is
  allowed

#### This phase must replace, not supplement

Phase 5.5.2 must replace, not merely supplement:

- creation specs that carry world placement as if placement were intrinsic shape
- movement APIs that pretend moving an existing shape is the same as creating a
  new one somewhere else
- any future temptation to widen primitive family specs with one-off direction
  or axis fields instead of a shared spatial-intent layer
- public teaching surfaces that expose raw transform math before semantic
  spatial meaning
- anchor naming schemes that attach to incidental topology enumeration instead
  of canonical semantic parts
- any primitive-only spatial semantics that would force NURBS, sweep, chamfer,
  fillet, or blend work to fork the language later
- any motion model that stores only final placement state and throws away the
  authored operation meaning needed for replay, merge, interaction, and audit

#### Do not do these things

- do not couple placement to primitive creation so tightly that later movement
  or rotation has no independent semantic identity
- do not add one-off fields like `direction`, `axis`, or `rotation` to each
  family spec as a substitute for a shared spatial-intent layer
- do not make the main public DX look like matrices, quaternions, basis
  constructors, or raw transform structs
- do not postpone anchor semantics until "persistent naming later"
- do not let hostile suites invent their own placement DSL outside the canonical
  spatial-intent surface
- do not flatten create, move, rotate, and reorient into one vague transform
  helper
- do not assume every future spatial edit is adequately described by one point
  plus one direction vector
- do not let the shared spatial language stop at rigid primitives if the later
  roadmap already knows it must carry surfaces, freeform carriers, and fillet
  feature results

#### Required planning deliverables

At minimum, this phase must produce a frozen plan for:

- one intrinsic-shape versus placement split for primitive authoring
- one semantic create / move / rotate / reorient / offset family of surfaces
- one prepositional public vocabulary broad enough for later relational edits
- one canonical local frame and semantic-anchor model per admitted primitive
  family
- one lowering story from prepositional language into inspectable placement and
  motion plans
- one persistent-naming compatibility story proving anchors are assigned before
  world embedding
- one reuse story showing how this substrate can serve later non-primitive
  shapes
- one generic anchor taxonomy broad enough for:
  - primitive semantic anchors
  - topology-local anchors
  - frame anchors
  - geometric and parameter-space anchors
  - feature-owned anchors for chamfer / fillet / blend families later
- one authored-motion history story proving that create, move, rotate,
  reorient, and offset can become inspectable authoritative operations instead
  of transient UI sugar

#### Required acceptance evidence from the planning phase

This planning phase is not done when the document says "we should support
placement someday." It is done only when the milestone text makes these things
unambiguous:

- creation and movement are different semantic operations
- intrinsic shape and placement are different semantic truths
- prepositional spatial language is the intended public DX
- canonical local anchors are the intended persistent-naming substrate
- raw transforms are a lower-layer implementation tool, not the public semantic
  story
- Phase 5.6 can author compound hostile cases on top of this model without
  inventing suite-local placement folklore
- later NURBS, freeform, chamfer, fillet, and blend milestones have a clearly
  reusable spatial and anchor substrate instead of an implied fork point

#### Do not start the next phase until

- the plan clearly distinguishes:
  - intrinsic shape
  - placement
  - movement
  - rotation or reorientation
  - relational or prepositional constraints
  - semantic anchors
- later persistent naming has a believable semantic substrate to inherit
- compound hostile cases can be described in terms of spatial meaning rather
  than transform archaeology
- future implementation can scale to primitives, then later sketches, surfaces,
  sweeps, imported bodies, and boolean operands without replacing the language
  again
- the plan is explicit enough that later motion, interaction, and history
  milestones will inherit one spatial-intent story instead of inventing
  separate transform, anchor, and continuity semantics

### Phase 5.5.3: Freeze Motion Reference, Resolution, And Failure Semantics Before The Compound Siege

Turn movement, rotation, reorientation, and spatial relation targeting into a
first-class certified resolution system before the compound siege and before
later curve, surface, NURBS, fillet, and blend work inherit a vague or
carrier-ambiguous motion model.

#### Why this phase comes now

Phase 5.5.2 freezes the shared spatial-intent substrate:

- authored verbs
- placement and frame semantics
- anchor semantics
- constraint-style phrasing
- the ownership split between kernel, spatial, geom, and topo

That is necessary, but it is not yet the full principled motion foundation.
Without one more correction phase, Worth is still in danger of freezing the
wrong motion semantics:

- `parallel_to(...)` and `perpendicular_to(...)` can still be interpreted too
  loosely
- carrier-level references like "curve" or "surface" can still be treated as
  if they implied one stable direction automatically
- ambiguous and degenerate motion targets can still get handled as ad hoc
  helper failures instead of typed semantic outcomes
- fallback-derived tangent, normal, or frame truth can still disappear into
  anonymous geometry folklore
- later replay, merge, persistent naming, fillet, and freeform work can still
  inherit "final transform state" instead of authored motion meaning

`MENTALITY.md` points the way here just as strongly as it did for primitive
realization:

- this is a foundation correction, not future polish
- later curve, surface, NURBS, fillet, blend, and interaction work will
  inherit whatever motion-reference policy is frozen now
- the compound suite in Phase 5.6 should pressure a principled motion
  resolution substrate, not invent its own target-resolution folklore

This phase therefore belongs before Phase 5.6 because the compound suite must
exercise the same motion-resolution semantics that later product code and later
MetaBoss-tier hostile suites will rely on.

#### The core semantic rule this phase must freeze

No motion or placement operation may target a geometric carrier when the actual
directional, positional, or frame witness is under-specified.

That means:

- `parallel_to(curve)` is not a valid final semantic form
- `perpendicular_to(surface)` is not a valid final semantic form
- `aligned_with(feature)` is not a valid final semantic form when the feature
  exposes more than one meaningful axis or frame

Instead, the system must require or derive an explicit witness such as:

- a world direction
- a frame axis
- a curve tangent at a parameter-space anchor
- a surface normal at a parameter-space anchor
- a feature-owned axis, rail, spine, or junction frame

If the witness cannot be named uniquely and honestly, the system must reject or
exhaust with typed truth rather than guessing.

#### What this phase must establish

Motion must be frozen as one layered semantic system:

1. subject truth
2. anchor truth
3. reference-frame truth
4. directional or positional witness truth
5. resolution strategy truth
6. ambiguity / degeneracy / exhaustion failure truth
7. history-bearing authored-motion truth

The long-term shape must be:

- subject answers:
  - what existing or newly-created thing is being moved or reoriented?
- anchor answers:
  - what semantic or geometric anchor is the operation attached to?
- reference answers:
  - what world, workplane, local, feature, or carrier-derived frame is the
    operation expressed against?
- witness answers:
  - what point, axis, tangent, normal, or derived frame is actually being used?
- resolution answers:
  - was the witness direct, frame-derived, carrier-derived, fallback-derived,
    or exhausted?
- failure answers:
  - was the request ambiguous, undefined, unsupported, degenerate, or
    exhausted?
- history answers:
  - what authored motion act happened, and what semantic identity was preserved
    across replay and certification?

#### Required motion-reference model

This phase must freeze a reusable motion-reference vocabulary broad enough for
primitives now and later non-primitive carriers.

At minimum the design must account for:

- world references:
  - explicit world point
  - explicit world direction
- frame references:
  - frame origin
  - frame axis
  - full frame witness
- carrier-derived references:
  - curve tangent at parameter anchor
  - surface normal at parameter anchor
  - surface tangent-u at parameter anchor
  - surface tangent-v at parameter anchor
  - later carrier-derived frame witnesses where the domain can supply them
- feature-owned references:
  - fillet spine
  - chamfer support direction
  - blend rail
  - blend junction frame

This does not require full curved-carrier implementation in Milestone 4, but it
does require the semantic shape to be frozen now so those later domains do not
fork the language.

#### Required reusable ownership and directory topology

This phase must not organize reusable semantic concerns under the first verb
that happens to consume them.

The structural law is:

- if a concern will apply to more than one future verb, it must not live under
  a verb-specific subtree
- if a concern defines reusable spatial meaning, it belongs in
  `worth-spatial`
- if a concern defines public authored verbs or orchestration-facing surfaces,
  it belongs in `worth-kernel`
- if a concern is pure math, it belongs in `worth-geom`

That means the permanent home for reusable motion-reference and witness
semantics must be a shared subtree in `worth-spatial`, not:

- under `move/`
- under `rotate/`
- under `construction/`
- under `primitives/`
- under any suite-local or provenance-local folder

The intended permanent skeleton for the shared reusable substrate is:

```text
crates/worth-spatial/src/spatial_intent/
  refs/
  resolution/
  constraints/
  lowering/
```

Where:

- `refs/`
  - owns anchors, frames, witnesses, carrier references, and feature-owned
    references
- `resolution/`
  - owns tolerance, resolution classes, failure classes, fallback posture, and
    policy profiles that affect witness resolution
- `constraints/`
  - owns reusable relation semantics such as lies-on, points-toward, matches,
    and future relational carriers
- `lowering/`
  - owns lowering from semantic motion references into executable or
    certification-consumable plans

The intended permanent skeleton for the verb-facing kernel layer is:

```text
crates/worth-kernel/src/spatial_intent/
  create/
  motion/
  relations/
  lowering/
```

Where:

- kernel owns the common path and advanced path authoring surfaces
- spatial owns the reusable meaning they consume

This phase must not leave later developers guessing where tolerance, witness
resolution, or motion-resolution policy belongs.

#### Required failure and policy model

This phase must explicitly freeze typed motion-resolution failure classes.

At minimum the shared substrate must have room for exact distinctions such as:

- `AmbiguousReference`
- `UndefinedReference`
- `UnsupportedReferenceRole`
- `DegenerateCarrier`
- `CoincidentTarget`
- `InvalidAxisOrDirection`
- `ExhaustedResolutionStrategies`

The exact final names may evolve, but these semantic distinctions may not be
collapsed into one generic "bad motion input" bucket.

This phase must also make it explicit that tolerance and policy profiles belong
to reusable resolution semantics, not to one specific verb.

That means:

- tolerance must not be coupled only to moving things
- witness-resolution policy profiles must not be coupled only to
  `parallel_to(...)` or `perpendicular_to(...)`
- the same tolerance and policy substrate must be reusable later by:
  - snapping
  - grazing
  - contact classification
  - boolean candidate detection
  - host placement
  - fillet and continuity checks
  - NURBS and curved-carrier witness resolution

#### Required sanctioned resolution posture

Just as Phase 5.5 made primitive realization strategies explicit, this phase
must make motion witness resolution explicit.

At minimum the motion system must distinguish:

- direct witness resolution
  - explicit point, explicit vector, explicit frame axis
- frame-derived witness resolution
  - workplane normal, frame axis, feature-local basis
- carrier-derived witness resolution
  - tangent, normal, or parameter-space-derived witness
- sanctioned fallback-derived witness resolution
  - only when the motion substrate explicitly authorizes a fallback
- exhausted witness resolution
  - no sanctioned strategy produced a usable witness

If fallback or approximation is used, that must survive as inspectable motion
truth instead of disappearing into final placement residue.

#### Required Query posture

This phase must make Forge Query feel native to motion semantics, not bolted on
later.

The plan must explicitly require that authored motion truth be preserved across:

- direct preparation
- replay
- branch / preview
- inspection
- projection-consumption
- anti-bypass audit

And that the Query-backed reports can certify:

- what subject was moved
- what anchor basis was used
- what witness was requested
- what witness was actually resolved
- what resolution class was used
- whether failure happened at ambiguity, undefined-reference, unsupported-role,
  degeneracy, or exhaustion boundaries

Motion truth must therefore become part of the same canonical artifact and
proof posture that primitive realization already uses, rather than becoming a
side-channel or UI-only interpretation.

#### Required DX targets

This phase must also satisfy the DX laws explicitly. The goal is not cute
syntax; it is organized truth with complexity placed at the exact level where
the caller must make a responsible decision.

The public DX for motion resolution must therefore distinguish:

- common path:
  - reads like authored spatial intent
  - examples:
    - `shape.parallel_to(frame_axis(...))`
    - `shape.toward(world_point(...))`
    - `shape.aligned_with(frame(...))`
- advanced path:
  - exposes the next lower semantic boundary explicitly
  - examples:
    - build motion intent
    - inspect or compile witness plan
    - inspect resolution class and failure topology
    - lower into a Query-backed authoritative artifact
- unsafe or degraded path:
  - must make weakened guarantees explicit rather than pretending to be normal
    intent authoring

The phase must not leave developers with only:

- raw vectors
- raw frames
- generic bags of options
- final-placement-only objects

At minimum the built surface must aim for:

- semantic nouns for witnesses and targets rather than naked tuples wherever
  responsibility is nontrivial
- inspectable plans or reports before execution when witness resolution,
  fallback, or exhaustion is at issue
- explanation surfaces that let a developer answer:
  - what witness was requested?
  - what witness was resolved?
  - why did it fail or fall back?
- API shapes where expensive or ambiguous work looks expensive or ambiguous
  rather than masquerading as a cheap property setter

This phase must be explicit that later carrier-derived motion should lower
through inspectable witness plans, not helper calls that immediately discard
the semantic steps.

#### Required adversarial cases this phase must plan for

This phase must explicitly plan for at least these motion edge cases so they do
not become ad hoc later:

- zero-length or non-finite move, offset, rotation-axis, or facing vectors
- `toward(...)` where the target point is coincident with the anchor origin
- `between(...)` where both endpoints are identical
- near-parallel and near-perpendicular frame targets
- frame-alignment requests under huge world translation and tiny local scale
- ambiguous carrier requests such as:
  - whole curve with no parameter
  - whole surface with no tangent-family or normal witness choice
  - feature with multiple plausible axes
- degenerate carrier-derived requests such as:
  - cusp or zero tangent
  - singular or collapsed surface frame
  - unstable local frame near a seam or pole
- periodic or multi-solution carrier references where more than one witness is
  plausible
- open-shell and wire-direction cases where anchor continuity matters but
  topology class differs from closed solids
- motion requests whose semantic meaning should remain stable even if the final
  world transform would look similar to another authored act

#### This phase must replace, not supplement

Phase 5.5.3 must replace, not merely supplement:

- the idea that motion semantics can be reconstructed from final placement
  state alone
- vague carrier-level target verbs that quietly guess one witness
- anonymous vector derivation inside helper code
- transform-first APIs that erase whether the user meant move, rotate,
  reorient, offset, align, or match
- soft error handling that collapses ambiguity, undefined reference, and
  exhaustion into one generic rejection
- motion proof surfaces that certify only final coordinates and not the motion
  resolution story that produced them

#### Do this in this phase

- implement one generic motion-reference model in `worth-spatial` rather than
  collapsing every request directly into raw vectors
- implement one generic direction / target / frame witness model broad enough
  to represent:
  - explicit world directions and points
  - frame axes and frame origins
  - parameter-space and feature-owned future witnesses without forking the
    type system later
- implement one typed motion-resolution-class model that preserves whether a
  witness was:
  - direct
  - frame-derived
  - carrier-derived
  - fallback-derived
  - exhausted
- implement one typed motion-failure-class model that preserves distinctions
  such as:
  - ambiguous
  - undefined
  - unsupported
  - degenerate
  - coincident
  - exhausted
- replace raw-vector-first `parallel_to(...)`, `perpendicular_to(...)`,
  `aligned_with(...)`, and related lowering with witness-bearing lowering
  surfaces
- preserve authored motion truth in one canonical artifact/report family that
  Query-backed proof surfaces can carry through:
  - direct preparation
  - replay
  - branch / preview
  - inspection
  - projection-consumption
  - anti-bypass audit
- add hostile proof lanes that certify:
  - ambiguous carrier-style requests are rejected honestly
  - degenerate and undefined witnesses are not confused with bad numeric input
  - fallback-derived witnesses stay visible as truth instead of disappearing
    into final placement
- add compile-fail or sealed-construction privacy proof where motion resolution
  internals must not leak past the facade

#### Required implementation deliverables

At minimum, this phase must ship:

- one implemented generic motion-reference model
- one implemented direction / target / frame witness model
- one implemented typed motion-resolution-class model
- one implemented typed motion-failure-class model
- one implemented sanctioned fallback / exhaustion policy for witness
  resolution
- one implemented Query-backed artifact and report story for authored motion
  truth
- one hostile suite of motion-resolution cases that later Phase 5.6 rows can
  reuse directly
- one implemented policy for when a request is:
  - admissible
  - ambiguous
  - undefined
  - unsupported
  - degenerate
  - exhausted

#### Required acceptance evidence from implementation

This phase is not done when the document merely says "later curves will need
tangents." It is done only when the built system makes these things
machine-checkably true:

- motion targets a witness, not a vague carrier
- ambiguous carrier-level requests are rejected or require a more specific ref
- direct, frame-derived, carrier-derived, fallback-derived, and exhausted
  witness resolution are distinct semantic outcomes in code and proof surfaces
- Query proof surfaces preserve authored motion truth, not just final
  placement
- the common path reads like motion intent while the advanced path exposes the
  next lower semantic and proof boundary explicitly
- later curves, surfaces, NURBS, fillets, and blends can reuse one motion
  reference substrate instead of inventing their own

#### Do not start the next phase until

- the motion-reference and failure model is explicit enough that
  `parallel_to(curve)` has a principled answer
- the plan clearly distinguishes witness ambiguity from witness degeneracy
- Query-backed motion truth has a believable permanent home in the same proof
  posture as primitive realization truth
- Phase 5.6 can author grazing, relocation, and reorientation hostility on top
  of this model without inventing suite-local direction folklore
- later curve, surface, freeform, fillet, and blend milestones have a clearly
  reusable motion-resolution substrate to inherit

### Phase 5.5.4: Freeze Intent Conflict, Candidate Arbitration, And Escalation Semantics Before The Compound Siege

Turn overlap, contact, alignment, containment, and host-style interaction
ambiguity into a first-class semantic system before Phase 5.6 and before later
boolean, BIM, fillet, and interaction work force the stack to guess user intent
silently.

#### Why this phase comes now

Phase 5.5.2 freezes spatial-intent verbs and anchor semantics. Phase 5.5.3
freezes motion witnesses, motion resolution, and typed motion failure. That
still leaves one major source of future pain if it is not addressed now:

- one user act can imply several plausible semantic outcomes
- overlap is not itself the user intent
- contact is not itself the user intent
- entering a volume is not itself the user intent
- alignment pressure is not itself the user intent

If Worth stops at motion and witness resolution, later interaction surfaces will
still be tempted to guess:

- move only
- snap flush
- align frames
- attach relationally
- nest inside
- merge booleanly
- subtract or cut
- join host and opening

That is where many of the worst CAD and BIM bugs come from:

- silent semantic boundary crossing
- one hidden default standing in for many plausible intents
- "fixes" buried in obscure settings after the wrong operation already
  happened

This phase exists to freeze the opposite posture:

- detect plausible candidate intents explicitly
- classify when one candidate is clearly dominant versus when several are
  plausible
- surface blocked future candidates honestly when downstream capability does
  not exist yet
- let the application ask rather than guess when the semantic boundary would be
  crossed silently

This phase belongs before Phase 5.6 because the compound suite should not only
pressure motion truth. It should also pressure interaction ambiguity,
pre-contact hostility, and blocked future-operation candidates on top of a real
arbitration substrate instead of ad hoc suite logic.

#### The core semantic rule this phase must freeze

When one authored act plausibly implies more than one semantic outcome, Worth
must not silently cross semantic boundaries.

That means:

- physical overlap is not enough to imply merge
- entering another shape is not enough to imply nesting or cutting
- grazing a face is not enough to imply snap or attach
- touching a host-like surface is not enough to imply hosting
- aligned placement is not enough to imply frame adoption

The system may auto-resolve only when one candidate intent is clearly dominant
and safe. Otherwise it must:

- preserve a typed conflict or candidate set
- rank or classify plausible outcomes
- expose whether a candidate is blocked by missing downstream capability
- allow the app layer to ask what the user intended

#### What this phase must establish

Intent conflict must be frozen as one layered semantic system:

1. authored act truth
2. observed spatial relation truth
3. candidate intent truth
4. conflict classification truth
5. arbitration or escalation policy truth
6. blocked-by-missing-capability truth
7. preserved chosen-intent truth for replay and certification later

The long-term shape must be:

- authored act answers:
  - what did the user explicitly do?
  - move, rotate, reorient, offset, place, align, or constrain
- observed relation answers:
  - what contact, overlap, containment, grazing, alignment, or host-like facts
    became true?
- candidate intent answers:
  - what plausible semantic outcomes are consistent with those facts?
- conflict answers:
  - is one outcome clearly dominant, or are multiple plausible?
- arbitration answers:
  - should the system auto-resolve, preserve multiple candidates, or ask?
- capability answers:
  - is one plausible candidate blocked because merge, cut, join, or hosting
    does not exist yet?
- preserved choice answers:
  - if the user or policy selects one candidate, how does that choice survive
    replay, inspection, and later audit?

#### Required candidate-intent model

This phase must freeze a reusable candidate-intent taxonomy broad enough for
primitive hostility now and later BIM / CAD interaction semantics.

At minimum the design must account for candidates such as:

- `MoveOnly`
- `SnapFlush`
- `AlignFrames`
- `AttachRelationally`
- `NestInside`
- `MergeCandidate`
- `SubtractCandidate`
- `CutOpeningCandidate`
- `JoinCandidate`

The exact final names may evolve, but the milestone must make it explicit that:

- motion intent
- relational intent
- topological intent
- host-style intent

are not the same thing and may all be plausible from one interaction.

#### Required reusable ownership and directory topology

This phase must not bury conflict, arbitration, blocked-capability, or ranking
semantics under one interaction verb such as `move/` or `snap/`.

The structural law is:

- if a concern will apply to more than one interaction family, it must not
  live under a single interaction family
- reusable conflict and arbitration meaning belongs in `worth-spatial`
- public authoring verbs and app-facing orchestration hooks belong in
  `worth-kernel`

That means the permanent home for reusable candidate/conflict/arbitration
semantics must be a shared subtree in `worth-spatial`, not:

- under `move/`
- under `snap/`
- under `merge/`
- under app-only UI code
- under suite-local or provenance-local directories

The intended permanent skeleton for the shared reusable substrate is:

```text
crates/worth-spatial/src/spatial_intent/
  arbitration/
```

Where:

- `candidates.rs`
  - owns typed candidate intents
- `conflicts.rs`
  - owns conflict classes and relation facts
- `ranking.rs`
  - owns candidate ranking and explanation posture
- `blocked.rs`
  - owns blocked-capability truth
- `escalation.rs`
  - owns arbitration and clarification posture

The intended permanent kernel-facing skeleton is:

```text
crates/worth-kernel/src/spatial_intent/
  arbitration/
```

Where:

- kernel owns the common path and advanced path authoring surfaces that expose
  candidate/conflict truth
- kernel does not become the owner of reusable arbitration semantics

This phase must leave later booleans, joins, host cuts, and BIM-style
interaction flows with one obvious semantic home instead of forcing them to
invent parallel conflict systems.

#### Required conflict families

This phase must explicitly plan at minimum these conflict classes:

- contact conflicts:
  - move-only versus snap versus attach
- alignment conflicts:
  - directional alignment versus frame adoption versus host alignment
- containment conflicts:
  - move-inside versus nest-inside versus embed/cut-host semantics
- topology-boundary conflicts:
  - remain separate versus merge versus subtract versus join
- host-relationship conflicts:
  - touch host versus attach to host versus cut opening in host
- capability-availability conflicts:
  - one candidate is plausible but blocked because the required downstream
    feature does not exist yet

#### Required arbitration and escalation posture

This phase must freeze one explicit arbitration posture.

At minimum the design must distinguish:

- `SingleClearIntent`
  - one candidate is dominant and safe to apply automatically
- `MultiplePlausibleIntents`
  - more than one candidate is plausible and the system should preserve the
    ambiguity instead of guessing
- `UnsafeToAssume`
  - auto-resolution would cross a semantic boundary without enough authority
- `BlockedCandidateSet`
  - one or more strong candidates exist, but the best one is blocked by
    missing downstream capability

The app layer may present these through:

- dropdown choice
- command palette
- inline affordance
- preview-time clarification

But the core crates must expose the semantic conflict and candidate truth, not
the widget.

#### Required blocked-capability semantics

This phase must explicitly plan how the system preserves plausible future
intent even when the required downstream capability does not exist yet.

Examples that must be covered in the plan:

- overlap suggests `MergeCandidate`, but booleans do not exist yet
- host penetration suggests `CutOpeningCandidate`, but host-opening semantics
  do not exist yet
- contact suggests `JoinCandidate`, but join semantics do not exist yet

The system must be able to say:

- this candidate is plausible
- this candidate is blocked
- these other lower-scope candidates are still available now

That is the foundation for user-friendly "Did you mean...?" flows that do not
force silent fallback to the wrong meaning.

#### Required Query posture

This phase must also make Forge Query feel native to interaction ambiguity.

The plan must explicitly require that the core artifact and proof surfaces can
preserve:

- authored act truth
- observed relation truth
- candidate intent set truth
- arbitration classification truth
- blocked-capability truth
- chosen-intent truth when a policy or user resolves the ambiguity

And that Query-backed reports can certify:

- when a conflict was absent
- when a single clear intent existed
- when multiple plausible intents existed
- when the system preserved ambiguity instead of guessing
- when a candidate was blocked by missing capability rather than being absent

#### Required DX targets

This phase must also satisfy the DX laws explicitly. The user-friendly target
is not "the kernel shows a dropdown." The user-friendly target is that the core
system exposes structured ambiguity truth so the app can ask the right question
at the right time instead of forcing the developer to reverse-engineer intent
from overlap after the fact.

The public DX for conflict and arbitration must therefore distinguish:

- common path:
  - reads like ordinary authored intent when no conflict exists
- advanced path:
  - exposes candidate intents, conflict class, blocked-capability state,
    ranking, and explanation before commitment
- human-escalation path:
  - looks like a real human boundary
  - does not masquerade as an ordinary local method call
  - gives the app layer enough structured information to ask:
    - "Did you mean move only, snap flush, nest inside, or merge?"

The phase must not leave developers with only:

- a hidden default policy
- a single boolean like `auto_snap`
- a generic conflict string
- app-local heuristics that must rediscover kernel intent after the fact

At minimum the built surface must aim for:

- typed candidate sets rather than loose labels
- typed blocked-candidate reasons rather than TODO comments
- inspectable arbitration reports before commitment when the ambiguity crosses
  semantic boundaries
- explanation surfaces that let a developer answer:
  - what user act was observed?
  - what relation facts were observed?
  - why were these candidate intents produced?
  - why was one candidate blocked?
  - why did the system auto-resolve or refuse to assume?
- a clear separation between core semantic truth and app-specific presentation
  so UI teams can build dropdowns, previews, or command palettes without
  inventing a second arbitration model

The intended kernel-facing common path should converge toward one artifact-style
surface rather than a pile of sibling helpers. At minimum the target shape
should be explicit enough that the implementation can honestly grow toward a
surface like:

- `PrimitiveIntentConflict::analyze(...)`
- `PrimitiveIntentConflict::analyze_with_capabilities(...)`
- `PrimitiveIntentConflict::clarification_request()`
- `PrimitiveIntentConflict::resolve_by_policy()`
- `PrimitiveIntentConflict::resolve_by_choice(...)`
- `PrimitiveIntentConflict::analysis()`

The exact final names may evolve, but the DX target must be explicit that:

- the common path returns one inspectable conflict artifact
- clarification and resolution are phase-progressive operations on that
  artifact, not disconnected utility calls
- the advanced path can still expose the lower `SpatialIntentArbitrationAnalysis`
  truth directly when the caller needs it

#### Required adversarial cases this phase must plan for

This phase must explicitly plan for at least these conflict-heavy edge cases:

- moving one closed solid into another without boolean semantics
- dragging one face into flush contact with another face
- rotating an open shell into near-host alignment where host semantics could be
  implied but should not be guessed
- moving a wire endpoint into near-coincidence with another anchor where snap,
  attach, or move-only are all plausible
- pushing a shape inside another where:
  - `MoveOnly`
  - `NestInside`
  - `MergeCandidate`
  are all plausible, but some may be blocked
- interactions where the best future candidate is blocked and the app must be
  able to ask whether the user wants one of the currently available
  alternatives instead

#### This phase must replace, not supplement

Phase 5.5.4 must replace, not merely supplement:

- hidden default policies that silently cross semantic boundaries
- interaction code that guesses merge, cut, snap, attach, or host semantics
  from contact alone
- app-only folklore about what the user "probably meant"
- conflict handling that loses blocked future-intent candidates instead of
  preserving them explicitly
- proof surfaces that only certify final placement and not the conflict or
  arbitration story that led there

#### Do this in this phase

- implement one generic candidate-intent model in the permanent shared
  substrate rather than leaving conflict handling to UI folklore
- implement one generic intent-conflict classification model that can preserve:
  - single-clear intent
  - multiple plausible intents
  - unsafe-to-assume situations
  - blocked candidate sets
- implement one generic arbitration / escalation policy model that preserves
  when the core system:
  - auto-resolves
  - preserves multiple candidates
  - requires clarification
  - marks a candidate blocked by missing capability
- implement one blocked-capability candidate model so future merge, cut, join,
  and hosting semantics can be surfaced honestly before those capabilities
  exist
- implement one Query-backed artifact and report story for:
  - observed relation truth
  - candidate-intent truth
  - conflict classification truth
  - blocked-candidate truth
  - chosen-intent truth when ambiguity is resolved
- add hostile proof lanes for:
  - overlap without silent merge
  - contact without silent snap
  - containment without silent nest or cut
  - blocked-but-plausible future candidates
- keep UI out of the core crates:
  - core crates emit candidate/conflict truth
  - app layers choose dropdowns, palettes, or other clarification widgets

#### Required implementation deliverables

At minimum, this phase must ship:

- one implemented generic candidate-intent model
- one implemented intent-conflict classification model
- one implemented arbitration / escalation policy model
- one implemented blocked-capability candidate model
- one implemented Query-backed artifact and report story for conflict and
  candidate truth
- one implemented policy for when the system:
  - auto-resolves
  - preserves multiple candidates
  - asks for clarification
  - marks a candidate blocked
- one hostile suite of contact, containment, and grazing ambiguity cases that
  Phase 5.6 can reuse directly

#### Required acceptance evidence from implementation

This phase is not done when the document merely says "the UI can ask." It is
done only when the built system makes these things machine-checkably true:

- overlap and contact do not imply one semantic outcome automatically
- multiple plausible intents can be preserved as first-class truth
- blocked future candidates are first-class truth, not just TODO comments
- the app layer has a principled place to ask what the user meant instead of
  guessing
- the common path stays simple when no conflict exists, while the advanced path
  exposes candidate, conflict, and blocked-capability truth before commitment
- later booleans, joins, host cuts, fillets, and BIM-style interaction flows
  can reuse one conflict/arbitration substrate instead of inventing their own

#### Do not start the next phase until

- the plan is explicit enough that "move this shape inside that shape" has a
  principled answer
- the plan distinguishes motion truth from candidate-intent truth
- the plan distinguishes blocked future candidates from absent candidates
- Query-backed proof posture has a believable permanent home for conflict and
  arbitration truth
- Phase 5.6 can pressure contact, grazing, and mixed-topology ambiguity on top
  of this model without inventing suite-local arbitration folklore

### Phase 5.5.5: Freeze Preview, Identity Continuity, And Policy Profile Semantics Before The Compound Siege

Turn preview, simulation, identity continuity, and reusable policy profiles
into first-class shared semantics before Phase 5.6 and before later snapping,
host placement, booleans, fillets, and NURBS work force these concerns to be
rediscovered piecemeal.

#### Why this phase comes now

Phases 5.5.2 through 5.5.4 establish:

- authored spatial verbs
- motion witnesses and motion-resolution truth
- candidate intents, conflict classes, and blocked-capability truth

That still leaves three big foundation concerns if we stop there:

- preview remains an app-local ghosting trick instead of a certified semantic
  surface
- identity continuity remains implied instead of typed
- tolerance and arbitration profiles remain scattered concerns instead of one
  reusable authority

Those concerns are broader than motion:

- preview will be needed for movement, snapping, conflict clarification,
  booleans, cuts, joins, and host placement
- identity continuity will be needed for movement, attach/detach, merge/split,
  persistent naming, fillets, blends, and host relationships
- tolerance and policy profiles will be needed across motion, snapping,
  grazing, contact classification, boolean candidate detection, and curved
  carrier witness resolution

This phase belongs before Phase 5.6 because the compound suite should not only
exercise motion and conflict. It should exercise previews, continuity truth,
and policy-profile behavior on top of one explicit shared substrate.

#### What this phase must establish

This phase must freeze three shared semantic systems:

1. preview and simulation truth
2. identity continuity truth
3. reusable policy profile truth

The long-term shape must be:

- preview answers:
  - what would happen if this authored act were committed?
  - what witness, conflict, candidate, blocked, and relation facts would be
    observed?
  - what warnings or ambiguities would be surfaced before commitment?
- identity continuity answers:
  - what semantic identity is preserved?
  - what anchor identity is preserved?
  - what identity is reinterpreted?
  - what identity is split, merged, or blocked pending explicit choice?
- policy profile answers:
  - what tolerance, grazing, snapping, arbitration, and auto-resolve posture is
    in effect?
  - what profile applies in conservative CAD mode, BIM-host mode, exact-model
    mode, or ask-first mode?

#### Required reusable ownership and directory topology

These concerns are shared semantic authorities and must not be buried under one
verb or one UI surface.

The intended permanent shared skeleton in `worth-spatial` is:

```text
crates/worth-spatial/src/spatial_intent/
  preview/
  continuity/
  resolution/
```

Where:

- `preview/`
  - owns simulation, preview artifacts, and explanation posture
- `continuity/`
  - owns identity continuity, anchor continuity, and continuity outcome types
- `resolution/`
  - owns reusable tolerance and policy profile semantics

The intended permanent kernel-facing skeleton is:

```text
crates/worth-kernel/src/spatial_intent/
  preview/
  arbitration/
```

Where:

- kernel owns the public DX and orchestration-facing preview or clarification
  surfaces
- spatial owns the reusable meaning they consume

#### Required preview and simulation model

This phase must freeze one reusable preview substrate that can carry:

- motion previews
- witness-resolution previews
- candidate-intent previews
- blocked-candidate previews
- grazing and proximity previews
- later boolean, host, and join previews

The preview system must not be treated as UI-only sugar. It must be a semantic
layer that can be:

- inspected
- tested
- replayed
- explained
- certified for parity later

#### Required identity continuity model

This phase must freeze one reusable identity continuity model broad enough for
later persistent naming and topology-changing operations.

At minimum it must distinguish:

- `IdentityPreserved`
- `AnchorContinuityPreserved`
- `IdentityReinterpreted`
- `IdentitySplit`
- `IdentityMerged`
- `IdentityBlockedPendingChoice`

The exact final names may evolve, but the phase must make it explicit that:

- move-only
- snap/attach
- merge/subtract later
- host cut/opening later

do not all preserve identity in the same way.

#### Required reusable policy profile model

This phase must freeze one reusable profile substrate rather than burying
thresholds and defaults inside individual verbs.

At minimum the shared system must have room for profiles such as:

- conservative exact-modeling profile
- BIM host-friendly profile
- ask-first arbitration profile
- aggressive snap profile
- high-fidelity preview profile

And those profiles must be able to control reusable concerns such as:

- tolerance and proximity policy
- grazing thresholds
- alignment thresholds
- arbitration / auto-resolve posture
- preview richness and artifact policy

The exact profile inventory may grow later, but the substrate must exist now.

#### Required DX targets

This phase must also satisfy the DX laws explicitly.

The public DX must distinguish:

- common path:
  - ordinary authored intent when no preview or conflict inspection is needed
- advanced path:
  - explicit preview, simulation, continuity, and policy inspection surfaces
- human-escalation path:
  - explicit clarification surfaces when preview reveals ambiguity or blocked
    future candidates

Developers must be able to ask questions like:

- what would happen if I committed this?
- what candidate intents would appear?
- what would be blocked?
- what identity would be preserved or split?
- what profile or tolerance policy caused this classification?

without dropping into logs or app-local heuristics.

The intended kernel-facing common path should converge toward one envelope-style
preview surface rather than separate sibling calls for preview and continuity.
At minimum the target shape should be explicit enough that the implementation
can honestly grow toward a surface like:

- `PrimitiveIntentPreview::analyze(...)`
- `PrimitiveIntentPreview::analyze_with_capabilities(...)`
- `PrimitiveIntentPreviewAssessment::preview()`
- `PrimitiveIntentPreviewAssessment::continuity()`
- `PrimitiveIntentPreviewAssessment::analysis()`
- `PrimitiveIntentPreviewAssessment::profile()`
- `PrimitiveIntentPreviewAssessment::capabilities()`
- `PrimitiveIntentPreviewAssessment::clarification_request()`

The exact final names may evolve, but the DX target must be explicit that:

- one common-path artifact carries preview and continuity truth together
- continuity is visibly derived from the same preview analysis rather than
  recomputed through app-local folklore
- the advanced path can still expose the lower `SpatialIntentPreview` and
  `SpatialIdentityContinuityAssessment` truth directly when the caller needs it

The reusable profile substrate must also expose one principled override or
derivation surface rather than only a growing list of frozen presets. At
minimum the phase should converge toward an explicit surface like:

- `SpatialIntentPolicyProfile::derive(SpatialIntentPolicyProfileOverride)`
- or an equally explicit profile-override artifact with named fields for:
  - proximity posture
  - alignment posture
  - arbitration posture
  - preview richness

The exact final names may evolve, but the phase must make it explicit that:

- named profiles remain the common path
- local semantic overrides are a first-class advanced path
- hostile suites in Phase 5.6 must not be forced to add one-off hard-coded
  profiles every time they need one stricter or richer posture

#### Required Query posture

This phase must make Forge Query feel native to preview, continuity, and
profile semantics too.

The plan must explicitly require that the core artifact and proof surfaces can
preserve:

- preview truth
- candidate-preview truth
- blocked-preview truth
- identity continuity truth
- profile and tolerance posture truth

And that Query-backed reports can certify parity of those surfaces across:

- direct preparation
- replay
- branch / preview
- inspection
- projection-consumption

#### Do this in this phase

- implement one reusable preview and simulation substrate in `worth-spatial`
- implement one reusable identity continuity model
- implement one reusable tolerance and policy profile model
- implement one kernel-owned conflict artifact surface over the arbitration
  substrate rather than leaving the public DX as disconnected helper calls
- implement one kernel-owned preview assessment envelope that binds preview and
  continuity truth together for the common path
- implement one named profile-override or derivation surface so advanced callers
  can specialize a profile without inventing ad hoc presets
- implement one Query-backed artifact and report story for:
  - preview truth
  - continuity truth
  - profile truth
- add hostile proof lanes for:
  - preview of blocked future candidates
  - preview of multiple plausible intents
  - identity continuity differences between move-only and relational or
    topology-changing future candidates
  - profile-dependent classification differences that remain explicit rather
    than hidden

#### Required implementation deliverables

At minimum, this phase must ship:

- one implemented preview and simulation substrate
- one implemented identity continuity model
- one implemented reusable tolerance and policy profile model
- one implemented Query-backed artifact and report story for preview,
  continuity, and profile truth
- one hostile suite of preview and continuity cases that Phase 5.6 can reuse
  directly

#### Required acceptance evidence from implementation

This phase is not done when the document merely says "the app can preview it"
or "we can add profiles later." It is done only when the built system makes
these things machine-checkably true:

- preview is a semantic surface, not only a UI effect
- identity continuity is typed truth, not convention
- reusable policy profiles exist outside individual verbs
- developers can inspect why a preview, conflict, or continuity classification
  happened
- later snapping, host placement, booleans, joins, fillets, and curved
  carriers can reuse one preview/continuity/profile substrate instead of
  inventing their own

#### Do not start the next phase until

- preview, continuity, and profile semantics have a permanent shared home
- tolerance and policy profiles are not coupled only to motion
- identity continuity has a believable substrate for later persistent naming
  and topology-changing operations
- Phase 5.6 can pressure motion, conflict, preview, continuity, and profile
  behavior on top of one coherent foundation

### Phase 5.5.6: Freeze Anchor Breadth, Directional Anchors, And Reference-Translation Semantics Before The Compound Siege

Turn the remaining spatial-anchor support gaps into one explicit shared
substrate before Phase 5.6 and before later host, snapping, boolean, fillet,
and parameter-space work force the kernel to invent suite-local anchor folklore.

#### Adversarial constraint

The same authored intent must not change meaning merely because the caller uses
a different anchor vocabulary for the same spatial fact. A naive implementation
either:

- forces every non-`ShapeOrigin` case through point-anchor lowering and silently
  lies about direction, tag, or carrier-local meaning, or
- leaves every advanced anchor unsupported and forces later suites to invent
  one-off lowering paths that drift from each other under replay, branch, and
  preview pressure

This phase exists to survive that failure mode.

#### Why this phase comes now

Phases 5.5.2 through 5.5.5 already freeze:

- motion witness and motion-resolution truth
- conflict, arbitration, and blocked-capability truth
- preview, continuity, and policy-profile truth

But the anchor substrate is still materially incomplete if we stop there:

- external reference anchors are not yet a principled translation surface
- axis-bearing anchors are not yet first-class lowering inputs
- geometric tags do not yet resolve through one reusable authority
- parameter-space anchors still use a placeholder string payload rather than a
  numeric carrier-local model

Phase 5.6 should not have to decide whether a hostile case uses:

- a subject-owned point anchor
- an external reference point anchor
- a directional axis anchor
- a tagged anchor
- a carrier-local parameter-space anchor

by inventing its own suite-local lowering folklore.

This phase therefore belongs before Phase 5.6 because the compound suite must
pressure one coherent anchor substrate rather than a partially closed point-only
path plus a growing list of typed unsupported excuses.

#### What this phase must establish

This phase must freeze four anchor semantics explicitly:

1. subject-owned translation anchors
2. external reference translation anchors
3. directional and axis anchors
4. tag-backed and parameter-space anchor identity

The system must stop pretending these are all one point-anchor problem.

At minimum, the model must distinguish:

- anchors that move with the subject
- anchors that are external references only
- anchors that denote points
- anchors that denote directions or axes
- anchors that denote carrier-local parameter-space locations
- anchors that denote named feature or geometric tags requiring resolution

#### Required reusable ownership and directory topology

The anchor substrate must keep shared semantics in `worth-spatial` and expose
kernel DX through `worth-kernel` without burying anchor reasoning under one
verb.

The intended permanent shared skeleton in `worth-spatial` is:

```text
crates/worth-spatial/src/spatial_intent/
  refs/
  lowering/
```

Where:

- `refs/`
  - owns anchor, witness, carrier, and tag identity models
- `lowering/`
  - owns lowering of authored spatial semantics into point, direction, and
    placement truth

The intended kernel-facing skeleton remains:

```text
crates/worth-kernel/src/spatial_intent/
  lowering/
```

Where:

- kernel owns the public common-path lowering and diagnostics posture
- spatial owns the reusable anchor and lowering meaning they consume

#### Required anchor-semantics split

This phase must make it impossible to silently treat external references as if
they were subject-owned anchors.

At minimum, the plan must distinguish:

- subject-owned point anchors
  - `ShapeOrigin`
  - subject-owned point-like `FeatureOwned(...)`
- external reference point anchors
  - `WorldOrigin`
  - `FrameOrigin(...)`
  - external point-like `FeatureOwned(...)` resolved through the witness
    catalog when allowed
- directional anchors
  - `ShapeAxis(...)`
  - `FrameAxis(...)`
  - direction-like `FeatureOwned(...)`
- tagged anchors
  - `GeometricTag(...)`
- carrier-local anchors
  - `ParameterSpace(...)`

The exact final type names may evolve, but the substrate must stop collapsing
point ownership, external reference, direction, tag, and parameter-space into
one enum branch with ad hoc verb checks.

#### Required support closure in this phase

This phase must add honest support for the remaining anchor cases the current
substrate can reasonably carry now:

- external-reference translation semantics for authored move/offset-style
  operations
- directional lowering for:
  - `ShapeAxis(...)`
  - `FrameAxis(...)`
  - direction-like `FeatureOwned(...)`
- tag-backed anchor lowering for:
  - `GeometricTag(...)`
- target-side parity where the system already supports the same anchor meaning
  on the source side

That support must be principled. It must not silently mutate the meaning of
existing verbs.

If `Move` and `Offset` keep their current subject-owned meaning, then this
phase must either:

- introduce an explicit external-reference translation surface, or
- refactor the lowering model so the subject-owned vs external-reference
  distinction is encoded structurally before execution

What this phase must not do is let `WorldOrigin` or `FrameOrigin(...)` masquerade
as a moving subject-owned anchor without the type system admitting that the
meaning changed.

#### Required directional-anchor model

This phase must make axis and direction anchors first-class lowering inputs
instead of leaving them as typed decorations with no execution path.

At minimum, the substrate must define:

- which authored acts consume point anchors
- which authored acts consume directional anchors
- which authored acts can accept either with different lowered plans
- one reusable lowering bridge from:
  - `ShapeAxis(...)`
  - `FrameAxis(...)`
  - direction-like `FeatureOwned(...)`
  into the direction-witness substrate

The model must be explicit that axis anchors are not points and must not be
forced through point-anchor lowering.

#### Required geometric-tag model

This phase must stop treating `GeometricTag(...)` as a placeholder escape hatch.

At minimum, the plan must freeze:

- one authoritative tag-resolution seam
- one typed distinction between:
  - unresolved tag
  - ambiguous tag
  - resolved point-like tag
  - resolved direction-like tag
  - resolved unsupported tag class
- one lowering path that consumes resolved tag meaning rather than raw strings

The exact final tag inventory may evolve later, but Phase 5.6 must inherit a
real resolution substrate rather than hard-coding tag folklore inside hostile
fixtures.

#### Parameter-space deferral boundary

Robust `ParameterSpace(...)` anchor support is intentionally deferred out of
Phase 5.5.6 and into Milestone 5.

That deferral is not because parameter-space is unimportant. It is because
carrier-local parameter anchors belong with topology-to-geometry binding truth,
curve/surface binding semantics, and rebinding/continuity authority rather than
with the pre-Boolean primitive intent-lowering cleanup in this milestone.

Phase 5.5.6 must therefore preserve only the honest denial boundary:

- `ParameterSpace(...)` remains typed unsupported here
- no suite-local string parsing or fake numeric lowering may be introduced
- no operation may pretend that a display string is authoritative carrier-local
  identity

Milestone 5 must then replace that denial with a real carrier-local anchor
model aligned to the binding substrate.

#### Relationship to exact geometric predicates

This milestone must also make explicit why `worth-math` exact predicates are
necessary but not sufficient for future parameter-space support.

Exact orientation, incidence, and classification support helps only after the
system already knows:

- which carrier the anchor lives on
- which numeric parameter coordinates are authoritative
- whether the anchor denotes a point or a direction-like role

Predicates can certify geometry at that numeric location. They cannot replace
the missing anchor identity model or tell the lowering layer what carrier-local
fact the caller intended. That model belongs to Milestone 5's binding truth.

#### Required Query and proof posture

This phase must preserve typed unsupported and typed resolution-failure truth
instead of flattening it into generic unsupported-anchor errors.

The artifact and proof surfaces must be able to preserve:

- supported lowering
- typed unsupported anchor class
- typed witness failure
- typed tag-resolution failure
- typed ambiguity between point-like and direction-like meaning

And Query-backed reports must certify parity of those surfaces across:

- direct preparation
- replay
- branch / preview
- inspection
- projection-consumption

#### Do this in this phase

- freeze the anchor-semantics split between subject-owned point anchors,
  external reference anchors, directional anchors, tag-backed anchors, and
  carrier-local anchors
- implement one principled external-reference translation model instead of
  overloading subject-owned move semantics implicitly
- implement one reusable directional-anchor lowering path for:
  - `ShapeAxis(...)`
  - `FrameAxis(...)`
  - direction-like `FeatureOwned(...)`
- implement one authoritative `GeometricTag(...)` resolution seam and lowering
  path
- implement one Query-backed artifact and report story for anchor lowering
  support, typed unsupported classes, witness failures, and tag failures
- add hostile proof lanes for:
  - source-side vs target-side anchor symmetry where the semantics should match
  - subject-owned vs external-reference translation meaning
  - point-anchor vs directional-anchor lowering differences
  - typed tag failure vs typed witness failure vs typed unsupported class

#### Required implementation deliverables

At minimum, this phase must ship:

- one implemented anchor-semantics split broad enough to distinguish point,
  direction, tag, and carrier-local anchors
- one implemented external-reference translation model
- one implemented directional-anchor lowering path
- one implemented geometric-tag resolution and lowering path
- one Query-backed artifact and report story for anchor-lowering truth

#### Required acceptance evidence from implementation

This phase is not done when the document merely says "we support more anchors."
It is done only when the built system makes these things machine-checkably
true:

- external references do not silently masquerade as subject-owned anchors
- axis anchors do not silently masquerade as point anchors
- geometric tags do not bypass one shared resolution authority
- later hostile suites can author anchor-rich pressure on top of one shared
  substrate instead of inventing per-suite lowering folklore

#### Do not start the next phase until

- point, direction, tag, and carrier-local anchor semantics have a permanent
  shared home
- the remaining supported anchor breadth is closed on one reusable lowering
  substrate
- geometric-tag resolution is no longer an unowned placeholder
- Phase 5.6 can pressure anchor-rich motion, conflict, preview, continuity, and
  mixed-topology workloads without inventing suite-local anchor rules

### Phase 5.6: Freeze The Compound Primitive Adversarial Suite Before Docs

Turn the new realization-policy substrate into the next serious pre-METABOSS
proof suite: a compound primitive adversarial family that stacks multiple
failure modes at once and proves either exact success or exact structured
failure across the whole runtime and certification chain.

#### Why this phase comes now

Phase 5.5 makes primitive realization principled. Phase 5.6 proves that the
new substrate can survive compound pressure instead of only isolated threshold
or family-local hard cases.

This phase exists because the next failure mode after "hidden primitive rescue"
is "the primitive model is only robust one stressor at a time." The MetaBoss
document explicitly rejects that bar. The next suite Worth must survive is not
"one tiny pyramid" or "one huge simplex." It is a chained, multi-stressor,
proof-bearing primitive workload that teaches the kernel how to survive stacked
degeneracy before booleans magnify the blast radius.

Phase 5.6 therefore belongs before docs:

- the docs should describe the compound primitive proof substrate we actually
  trust
- later MetaBoss-tier planar, curved, and fillet suites will inherit the
  primitive proof and artifact posture frozen here
- if the primitive layer still has hidden escape hatches under stacked stress,
  documentation would only fossilize the wrong story

#### Adversarial suite target in this phase

This phase must define and pass the next named suite after the current siege:

- `PrimitiveConstructionCompoundAdversarialSiege`

The suite is not a Boolean final boss. It is the pre-Boolean primitive
compound-pressure gate that proves Worth can stack several hard conditions at
once and still either:

- realize the primitive honestly with explicit strategy / conditioning /
  stability truth, or
- fail cleanly with exact structured rejection or exhaustion truth

This suite must not be closed-solid-only. It must explicitly pressure the
admitted NMT and topology-class surfaces that later motion, naming, and
interaction work will inherit:

- closed solid families
- open shell families
- wire families
- mixed topology-class workloads containing more than one admitted family in one
  authored batch

The suite is still pre-Boolean. That means it is allowed to pressure
near-contact, frame alignment, anchor continuity, and grazing motion, but it is
not allowed to quietly smuggle in boolean-resolution requirements. Grazing in
this phase means structured pre-contact spatial pressure, not topology merging.

At minimum, the suite must include chained workload families that combine:

- large world-coordinate translation
- tiny local feature scale
- near-threshold admitted geometry
- threshold-neighbor rejected geometry
- hostile aspect ratios or altitude collapse
- near-coplanar or near-degenerate support derivation pressure
- branch / preview parity pressure
- replay parity pressure
- inspection / projection-consumption parity pressure
- family-boundary drift pressure
- authored placement pressure
- authored movement / rotation / reorientation / offset pressure from the
  `Phase 5.5.2` spatial-intent surface
- pre-Boolean grazing pressure against reference anchors, workplanes, and
  other admitted shapes without requesting topology combination

The suite must not be a single showcase part. It must be a family-driven matrix
with multiple primitive families and multiple stacked stress roles per family.

#### What the next suite should look like

The minimum honest target is a compound primitive corpus with rows such as:

- `DirectStable`
  - ordinary but nontrivial admitted cases that prove the direct path still
    stays stable under translation, scale, and replay pressure
- `EscalatedStableLocalNormalized`
  - cases that require local-coordinate normalization but still succeed
    honestly
- `EscalatedStableExactSupport`
  - cases that require exact-support style salvage but still succeed honestly
- `StructuredAdmissionRejection`
  - cases rejected before realization because the semantic primitive request is
    invalid
- `StructuredExhaustionRejection`
  - semantically meaningful cases that survive admission but exhaust all
    sanctioned realization strategies and fail with exact exhaustion truth
- `OrderingParityCase`
  - cases whose normalized truth must remain invariant under multiple authoring
    orders
- `BoundaryDriftGuardCase`
  - threshold-neighbor admitted/rejected pairs whose classification must remain
    stable unless the realization-policy layer changes intentionally
- `MotionStableRelocation`
  - admitted rows whose intrinsic truth stays stable while authored movement
    changes world embedding
- `MotionHostileReorientation`
  - admitted rows whose anchor/frame semantics are pressured by authored
    rotation or reorientation
- `PreBooleanGrazingCase`
  - admitted or clean-failing rows that approach near-contact or near-alignment
    thresholds without requesting boolean merge / split / trim semantics

The suite should prioritize hostile simplex / tetrahedron and pyramid cases,
because those families exercise face-derived support truth most honestly, but
it must also keep direct-stable comparison rows from orthotope and prism so the
proof surface still distinguishes families that genuinely need escalation from
families that do not.

It must also widen beyond closed solids so the same proof language is exercised
by:

- at least one open shell family workload
- at least one wire family workload
- at least one mixed-topology-class workload combining solid, shell, or wire
  rows under one authored order matrix

#### Required suite matrix

Phase 5.6 must not leave the compound suite implicit. The minimum required
matrix is:

- `Orthotope`
  - `DirectStable`
  - `BoundaryDriftGuardCase`
- `RegularPrism`
  - `DirectStable`
  - `BoundaryDriftGuardCase`
- `RegularPyramid`
  - `EscalatedStableLocalNormalized`
  - `EscalatedStableExactSupport`
  - `StructuredAdmissionRejection`
  - `StructuredExhaustionRejection`
  - `BoundaryDriftGuardCase`
- `SimplexSolid`
  - `DirectStable`
  - `EscalatedStableLocalNormalized`
  - `EscalatedStableExactSupport`
  - `StructuredAdmissionRejection`
  - `StructuredExhaustionRejection`
  - `BoundaryDriftGuardCase`
- `SheetPatch`
  - `DirectStable`
  - `MotionHostileReorientation`
  - `PreBooleanGrazingCase`
  - `BoundaryDriftGuardCase`
- `WireOpen`
  - `DirectStable`
  - `MotionStableRelocation`
  - `PreBooleanGrazingCase`
  - `BoundaryDriftGuardCase`
- `MixedTopologyClassBatch`
  - `OrderingParityCase`
  - `MotionStableRelocation`
  - `MotionHostileReorientation`
  - `PreBooleanGrazingCase`

The matrix may widen beyond this minimum, but it must not shrink below it.

#### Required stacked stressors by row class

Each named row class must certify a specific compound condition set rather than
just "a hard number."

- `DirectStable`
  - must combine:
    - nontrivial world translation
    - replay parity pressure
    - branch / preview parity pressure
    - inspection / projection-consumption parity pressure
  - must prove the family remains honestly direct-stable under that stacked
    workload
- `EscalatedStableLocalNormalized`
  - must combine:
    - large world-coordinate translation
    - small local feature scale
    - support derivation that is valid semantically but unstable in raw
      world-space realization
    - replay / branch / inspection parity pressure
  - must prove `LocalNormalized` was required and preserved through the full
    runtime and certification chain
- `EscalatedStableExactSupport`
  - must combine:
    - world collapse or near-world collapse
    - semantically valid primitive intent
    - direct-support derivation failure
    - exact-support salvage success
    - replay / branch / inspection parity pressure
  - must prove `ExactSupport` was required and preserved through the full
    runtime and certification chain
- `StructuredAdmissionRejection`
  - must combine:
    - an out-of-class primitive request
    - parity across direct / replay / branch-local lanes
  - must prove no realization strategies were attempted and the failure stayed
    at the admission boundary
- `StructuredExhaustionRejection`
  - must combine:
    - semantically meaningful primitive intent
    - admitted family semantics
    - exhaustion of every sanctioned realization strategy
    - parity across all runtime proof lanes
  - must prove the failure is not an admission rejection and not a vague
    geometry failure, but a typed exhaustion with preserved attempted-strategy
    history and conditioning witness truth
- `BoundaryDriftGuardCase`
  - must come in admitted / rejected neighbor pairs for the same family
  - must prove that tiny floor drift changes:
    - realization strategy
    - stability class
    - exhaustion truth
    - family-boundary artifact digest
    only when the actual realization-policy boundary changes intentionally
- `MotionStableRelocation`
  - must combine:
    - admitted intrinsic family semantics
    - authored movement through the `Phase 5.5.2` motion surface
    - nontrivial translation or offset magnitude
    - replay / branch / inspection parity pressure
  - must prove movement changes embedding truth without inventing intrinsic
    realization drift
- `MotionHostileReorientation`
  - must combine:
    - admitted intrinsic family semantics
    - authored rotate or reorient intent
    - anchor-basis dependence
    - frame-alignment or near-alignment pressure
    - replay / branch / inspection parity pressure
  - must prove anchor continuity and directional semantics survive without
    collapsing into anonymous transform residue
- `PreBooleanGrazingCase`
  - must combine:
    - admitted placement or motion intent
    - near-contact or near-alignment against a reference anchor, workplane, or
      other admitted shape
    - no request for boolean resolution, topological merge, or trimming
  - must prove one of exactly two outcomes:
    - stable non-contact success with preserved proximity / anchor truth
    - clean structured failure with exact blocking boundary and reason
  - crash, silent reinterpretation into contact, or topology combination are
    never acceptable outcomes for this row class

#### Required explicit flagship cases

At minimum, the suite must contain these flagship compound cases:

- one `RegularPyramid` case where:
  - huge translation
  - tiny local scale
  - near-degenerate support normals
  - `ExactSupport`
  - replay / branch / inspection parity
  all stack together in one admitted row
- one `RegularPyramid` case where:
  - semantically valid intent
  - sanctioned strategies are all attempted
  - exhaustion is reached honestly
  - lower-layer witness and kernel artifact both certify the same exhaustion
    story
- one `SimplexSolid` case where:
  - huge translation
  - tiny local scale
  - near-coplanar face geometry
  - `LocalNormalized` or `ExactSupport` is required
  - the ordered attempted-strategy history survives through the full chain
- one `SimplexSolid` case where:
  - semantically meaningful admitted-family intent
  - all sanctioned simplex strategies exhaust honestly
  - the resulting failure is typed as realization exhaustion rather than
    semantic invalidity
- one `SheetPatch` case where:
  - the shell is admitted and open
  - authored reorientation aligns its local normal to within a near-threshold
    angle of a reference workplane normal
  - no shell-closing or boolean semantics are requested
  - anchor and frame truth survive through replay / branch / inspection parity
- one `WireOpen` case where:
  - an endpoint-bearing wire is admitted
  - authored relocation or offset moves one endpoint into a grazing
    near-coincidence with a reference anchor or shell edge anchor
  - no merge, trim, snap, or boolean semantics are requested
  - the result stays a clean non-contact success or a typed structured
    pre-contact failure
- one mixed-topology-class batch where:
  - at least one solid, one open shell, and one wire are authored together
  - at least one row uses motion or reorientation
  - at least one row uses grazing pre-contact pressure
  - authoring-order parity must stay stable across the normalized matrix truth

These flagship cases are required because they prove the suite is actually
stacking multiple failure modes instead of only widening threshold tables.

#### Required assertions per row

Every compound-suite row must expose and prove at minimum:

- `scenario_id`
- `family`
- `row_class`
- `outcome_disposition`
- `direct_digest`
- `replay_digest`
- `branch_local_digest`
- `inspection_digest` or equivalent certified inspection parity artifact
- `projection_consumption_digest` or equivalent certified projection-consumption
  parity artifact
- `selected_realization_strategy`
- ordered `attempted_realization_strategies`
- `stability_class`
- `feature_conditioning_class`
- `support_normal_class`
- `normalization_disposition`
- `exhaustion_reason`, when exhausted
- `rejection_class`, when rejected
- `rejection_locality`, when rejected
- `blocking_boundary`, when rejected
- `construction_breadth`
- `birth_attachment_breadth`
- `certification_breadth`
- `row_digest`

For admitted rows, the suite must also prove:

- the same selected realization strategy across direct / replay / branch-local
  / inspection-backed certification lanes
- the same ordered attempted-strategy history across those lanes
- the same stability class across those lanes

For rejected rows, the suite must also prove:

- whether rejection happened:
  - before realization
  - during realization exhaustion
  - later in the chain
- that zero-breadth rows are only allowed for pre-realization rejection
- that post-admission exhaustion rows still preserve attempted-strategy and
  conditioning witness truth

#### Required suite-level assertions

The suite itself must prove:

- multiple authoring-order lanes over the same compound matrix
- normalized matrix parity across those lanes
- distinct lane digests across those lanes
- family-boundary drift reports for the threshold-neighbor cases
- lower-layer exhaustion witness parity for every family that claims structured
  exhaustion
- motion-intent parity for every row class that uses authored move / rotate /
  reorient / offset semantics
- grazing parity for every row class that claims stable pre-contact success or
  structured pre-contact failure
- no compound row loses realization truth when lifted from:
  - geom
  - spatial
  - kernel result
  - runtime proof
  - certification artifact

#### Required outputs from this phase

Phase 5.6 must emit these direct closeout artifacts:

- `primitive_construction_compound_adversarial_siege_report`
- `primitive_construction_compound_parity_report`
- `primitive_construction_family_boundary_drift_report`
- `primitive_compound_ordering_parity_report`
- `primitive_compound_exhaustion_witness_parity_report`
- `primitive_compound_motion_parity_report`
- `primitive_compound_grazing_boundary_report`
- `simplex_realization_strategy_ladder_report`
- `simplex_realization_exhaustion_witness_report`

These outputs are required milestone evidence, not optional helper reports.

#### MetaBoss-style scenario definitions

Phase 5.6 must not stop at matrices and row labels. It must define explicit
compound scenarios in the same spirit that `METABOSS.md` uses for later
final-boss suites.

The minimum required named scenarios are:

Numeric convention for this phase:

- any value written as `2^N` means the literal floating-point magnitude
  produced by `2f64.powi(N)` in the authored test case
- any value written as `1.0e-N` means the literal floating-point scientific
  notation value in the authored test case
- these numeric rows are required test magnitudes, not illustrative examples
- if a family needs an additional hostile row beyond the required minima, that
  row may widen the matrix, but it may not silently replace the required
  numeric row
- if implementation reality proves a required numeric row is semantically
  invalid rather than hostile-but-admitted, the spec must be updated
  explicitly before the test is changed

##### PCAS-1 - The World-Collapsed Simplex Storm

**Test:**
A `SimplexSolid` family with these exact rows:

- `simplex_world_collapsed_admitted_local_or_exact`
  - center:
    - `[2^548, -2^548, 2^548]`
  - semantic edge scale:
    - `1.0e-200`
  - auxiliary altitude squeeze:
    - one realized altitude component must be constructed at `1.0e-220`
- `simplex_world_collapsed_threshold_rejected`
  - center:
    - `[2^548, -2^548, 2^548]`
  - semantic edge scale:
    - `0.0`
- `simplex_world_collapsed_explicit_exhaustion`
  - center:
    - `[2^548, -2^548, 2^548]`
  - semantic edge scale:
    - `1.0e-240`
  - auxiliary altitude squeeze:
    - one realized altitude component must be constructed at `1.0e-280`

The admitted variant must stay semantically valid while becoming hostile to raw
world-space support derivation. The exhaustion variant must remain an
admitted-family simplex intent, not an out-of-class semantic rejection.

Evaluate each row through:

- direct result preparation
- replay parity
- branch-local / preview parity
- inspection parity
- projection-consumption parity
- corpus certification

with at least two authoring orders and one threshold-neighbor admitted /
rejected pair.

**Failure modes triggered:**

- raw world-space support derivation collapse
- support-normal degeneracy under `1.0e-200` to `1.0e-280` local feature and
  altitude pressure
- scale-separation pressure between `2^548` world translation magnitude and
  `1.0e-200`-class local feature scale
- order-sensitive drift if attempted-strategy history is not preserved
- branch / replay / inspection divergence if realization truth is not carried
  canonically

**Required infrastructure:**

- a real simplex realization ladder in `worth-geom`
- simplex conditioning witnesses that capture near-coplanarity and scale
  separation honestly
- simplex support derivation that can escalate through sanctioned strategies
  rather than direct-only luck
- runtime-proof and certification surfaces that preserve ordered attempted
  strategies
- family-boundary drift reporting for simplex threshold neighbors

**Acceptance rule:**
The case must either:

- succeed with explicit `LocalNormalized` or `ExactSupport` truth preserved
  across every runtime and certification surface, or
- fail with typed realization exhaustion that preserves ordered attempted
  strategies, conditioning witness truth, and exact rejection locality

Admission rejection is not an acceptable answer for the admitted-world-
collapsed variant.

##### PCAS-2 - The Pyramid Floor Drift Chain

**Test:**
Construct a `RegularPyramid` scenario family with:

- `pyramid_direct_stable_comparison`
  - center:
    - `[128.0, -64.0, 32.0]`
  - sides:
    - `5`
  - radius:
    - `2.0`
  - height:
    - `4.0`
- `pyramid_threshold_admitted_exact_support`
  - center:
    - `[0.0, 0.0, 0.0]`
  - sides:
    - `3`
  - radius:
    - `1.0e-200`
  - height:
    - `1.0e-200`
- `pyramid_threshold_rejected_neighbor`
  - center:
    - `[0.0, 0.0, 0.0]`
  - sides:
    - `3`
  - radius:
    - `1.0`
  - height:
    - `0.0`
- `pyramid_semantic_exhaustion`
  - center:
    - `[0.0, 0.0, 0.0]`
  - sides:
    - `3`
  - radius:
    - `0.0`
  - height:
    - `1.0`

Run the same family through:

- direct preparation
- replay
- branch-local / preview
- inspection and projection-consumption parity
- family-boundary certification
- lower-layer exhaustion witness certification

and verify the same family under multiple authoring orders.

**Failure modes triggered:**

- world-space face-normal degeneration
- tiny-scale threshold drift around the `1.0e-200` admitted floor
- accidental floor movement hidden behind fixture changes
- mismatch between lower-layer exhaustion truth and kernel artifact truth
- mixed admitted / exhausted / rejected classification drift inside one family

**Required infrastructure:**

- full pyramid realization ladder already frozen in Phase 5.5
- exact-support salvage that is artifact-visible
- lower-layer exhaustion witnesses bound into family-boundary certification
- compound ordering parity artifacts
- boundary-drift artifacts whose digests change only when the true floor moves

**Acceptance rule:**
The family must prove:

- direct-stable rows stay direct-stable
- escalated rows preserve exact strategy and ordered attempts across all lanes
- exhausted rows are typed as realization exhaustion rather than vague geometry
  failure
- threshold-neighbor pairs produce stable family-boundary classifications and
  digest drift only where the realization-policy boundary actually changes

##### PCAS-3 - The Mixed Primitive Ordering Avalanche

**Test:**
Build one compound corpus containing admitted and rejected rows from:

- `Orthotope`
- `RegularPrism`
- `RegularPyramid`
- `SimplexSolid`
- `SheetPatch`
- `WireOpen`

The minimum required numeric rows inside that mixed corpus are:

- `orthotope_direct_stable`
  - center:
    - `[2^120, -2^120, 2^120]`
  - extents:
    - `[1.0e-120, 2.0e-120, 3.0e-120]`
- `orthotope_boundary_neighbor_rejected`
  - center:
    - `[0.0, 0.0, 0.0]`
  - extents:
    - `[1.0, 0.0, 2.0]`
- `regular_prism_direct_stable`
  - center:
    - `[2^200, 2^200, -2^200]`
  - sides:
    - `3`
  - radius:
    - `1.0e-150`
  - height:
    - `1.0e-150`
- `regular_prism_boundary_neighbor_rejected`
  - center:
    - `[0.0, 0.0, 0.0]`
  - sides:
    - `3`
  - radius:
    - `0.0`
  - height:
    - `2.0`
- `sheet_patch_reorient_grazing_workplane`
  - origin:
    - `[2^180, 0.0, -2^180]`
  - face count:
    - `5`
  - local edge scale:
    - `1.0e-80`
  - grazing angle:
    - the final authored orientation must land within `1.0e-12` radians of the
      reference workplane normal without crossing into contact reinterpretation
- `wire_open_endpoint_graze`
  - origin:
    - `[-2^140, 2^140, 0.0]`
  - segment count:
    - `6`
  - endpoint proximity:
    - one authored move or offset must leave the terminal endpoint within
      `1.0e-14` of the reference anchor without merge, snap, or trim semantics
- the required `RegularPyramid` rows from `PCAS-2`
- the required `SimplexSolid` rows from `PCAS-1`

and evaluate it under at least these authoring orders:

- canonical
- reversed
- rejected-first
- family-clustered
- escalation-clustered

The corpus must include both direct-stable and escalated-stable rows, plus
structured admission rejection and structured realization exhaustion rows.

**Failure modes triggered:**

- ordering-sensitive normalization or digest drift
- silent collapse of ordered attempted-strategy history into counts
- family interaction drift where one family's hard case hides another's
  regression
- parity artifacts that stay green while normalized matrix truth diverges
- mixed topology-class drift inside one authored matrix
- shell-frame grazing drift hidden by order clustering
- wire endpoint grazing drift hidden by family clustering
- motion-intent truth loss when placement or movement is flattened into final
  coordinates only

**Required infrastructure:**

- normalized matrix digests for the full compound corpus
- distinct lane digests for each authoring order
- row-level scenario identity strong enough to survive reordering
- mixed-workload certification that keeps admitted, rejected, and exhausted
  rows distinct
- motion and placement truth preserved through the compound artifact, not
  collapsed into anonymous transform residue
- report shapes that can carry solid, shell, and wire rows together without
  forcing shell or wire semantics into a closed-solid-only envelope

**Acceptance rule:**
All authoring orders must produce:

- the same normalized matrix truth
- distinct lane digests
- stable row identities
- stable realization and rejection classifications per scenario
- stable topology-class identity for shell and wire rows
- stable authored motion and grazing semantics per scenario

Any divergence must surface as a structured parity failure, not as a silently
different corpus artifact.

##### PCAS-4 - The Primitive Compound Gate

**Test:**
Run the full Phase 5.6 compound suite as one milestone-closeout gate, combining
the simplex storm, the pyramid floor-drift chain, and the mixed primitive
ordering avalanche into one machine-checkable certification bundle.

The closeout gate must therefore include at minimum all of these exact numeric
rows:

- `simplex_world_collapsed_admitted_local_or_exact`
- `simplex_world_collapsed_threshold_rejected`
- `simplex_world_collapsed_explicit_exhaustion`
- `pyramid_direct_stable_comparison`
- `pyramid_threshold_admitted_exact_support`
- `pyramid_threshold_rejected_neighbor`
- `pyramid_semantic_exhaustion`
- `orthotope_direct_stable`
- `orthotope_boundary_neighbor_rejected`
- `regular_prism_direct_stable`
- `regular_prism_boundary_neighbor_rejected`
- `sheet_patch_reorient_grazing_workplane`
- `wire_open_endpoint_graze`

**Failure modes triggered:**

- every compound primitive failure mode admitted by this milestone:
  - scale separation
  - world collapse
  - near-degenerate support derivation
  - threshold drift
  - authoring-order drift
  - branch / replay / inspection parity drift
  - lower-layer versus kernel artifact disagreement
  - mixed topology-class batching
  - motion-intent drift
  - pre-Boolean grazing misclassification

**Required infrastructure:**

- full realization-policy ladder for the primitive families covered in the
  suite
- full runtime-proof parity chain
- family-boundary drift certification
- lower-layer exhaustion witness parity
- motion-intent parity artifacts
- grazing-boundary artifacts for pre-contact shell and wire pressure
- canonical artifact and certification surfaces strong enough to explain exact
  success or exact structured failure

**Acceptance rule:**
The suite closes only when every named compound scenario either:

- produces a correct certified result with stable realization truth across all
  runtime and certification surfaces, or
- fails cleanly with structured admission or realization-exhaustion artifacts
  that identify the exact trigger, affected family, ordered attempted
  strategies, conditioning witness class, and blocking boundary

Crashing, hanging, vague geometry failure, silent floor drift, or parity drift
without exact artifact localization are all unacceptable.

#### Infrastructure that must exist before this suite can close

This phase must not just add harder tests on top of missing substrate. It must
build the missing infrastructure required to make those tests principled.

At minimum, this phase must close these infrastructure seams:

- a real simplex / tetrahedron realization ladder in `worth-geom`
  - `DirectWorld`
  - `LocalNormalized` when justified
  - `ExactSupport` when justified
  - typed exhaustion when sanctioned strategies run out
- simplex / tetrahedron conditioning witnesses rich enough to explain:
  - translation scale pressure
  - feature scale pressure
  - aspect-ratio or altitude collapse pressure
  - support-normal degeneracy pressure
  - normalization disposition
- lower-layer exhaustion witnesses for any primitive family whose sanctioned
  ladder can truly exhaust under semantically meaningful pressure
- explicit distinction between:
  - semantically invalid request rejection
  - realization exhaustion after admitted semantics
  - topology or artifact failure later in the chain
- runtime-proof and certification surfaces that preserve full ordered attempted
  strategy history, not just selected strategy or count
- compound-case scenario identity and digesting rules strong enough that
  multiple stacked failure modes cannot collapse into the same report row by
  accident
- motion-intent and placement artifacts strong enough that compound shell,
  wire, and mixed-topology rows do not flatten authored movement,
  reorientation, or grazing semantics into final-coordinate residue
- topology-class-aware report and parity shapes strong enough to carry closed
  solid, open shell, wire, and mixed-batch rows without silently narrowing the
  suite back to closed-solid semantics

#### Query posture in this phase

The compound suite must reuse the Query-backed runtime proof surfaces from
Phase 5, not bypass them.

At minimum, this phase must prove that compound primitive truth preserves:

- direct outcome parity
- replay parity
- branch-local / preview parity
- inspection parity
- projection-consumption receipt parity
- Query authority-lane and support-posture parity

This phase must not accept a compound suite that is only a lower-layer geometry
test. The suite must prove that the same stacked primitive truth survives the
whole Worth runtime and certification chain.

#### Do this in this phase

- define the `PrimitiveConstructionCompoundAdversarialSiege` artifact family
- add the next hostile primitive corpus roles that explicitly stack multiple
  failure modes instead of isolating them one at a time
- widen the suite beyond closed solids so `SheetPatch`, `WireOpen`, and
  mixed-topology-class batches are first-class hostile rows rather than later
  add-ons
- widen simplex / tetrahedron realization infrastructure until it can
  participate honestly in the realization-policy ladder rather than coasting on
  a trivial direct-only path
- add any additional lower-layer exhaustion witness suites required by the new
  compound primitive families
- preserve compound-case realization truth through:
  - `worth-geom`
  - `worth-spatial`
  - `worth-kernel`
  - runtime proof
  - corpus certification
  - family-boundary certification
- add chained compound-case assertions that explicitly prove:
  - direct-stable versus escalated-stable distinctions
  - semantic admission rejection versus post-admission exhaustion
  - stable ordered attempted-strategy history
  - stable conditioning and normalization truth
  - stable family-boundary classifications under threshold-neighbor pressure
- add multiple authoring-order lanes for the compound cases rather than a
  single canonical ordering
- add at least one compound case where huge translation, tiny local scale, and
  near-degenerate support geometry are all present simultaneously
- add at least one shell reorientation grazing case and at least one wire
  endpoint grazing case that use the `Phase 5.5.2` motion surface directly
- ensure every compound case either:
  - succeeds with exact certified realization truth, or
  - fails with exact structured exhaustion or rejection truth

#### This phase must replace, not supplement

Phase 5.6 is not allowed to bolt a "harder test suite" on top of soft
primitive classifications.

This phase must replace:

- any remaining direct-only primitive family story that exists only because the
  family has not yet been pushed through compound-pressure realization
- any test whose meaning is merely "this threshold currently passes" without
  stating which compound failure modes are present
- any compound-case reporting that compresses stacked failure modes into one
  vague admitted / rejected result without strategy, conditioning, stability,
  and exhaustion truth
- any lower-layer witness that is used as a stand-in for a real compound
  primitive family until the corresponding primitive family has been evaluated
  honestly

#### Do not start the next phase until

- the next compound primitive suite exists as a first-class certification
  artifact family rather than only as scattered tests
- simplex / tetrahedron no longer coast on a trivial direct-only story when a
  sanctioned realization ladder is structurally required
- at least one compound primitive family proves:
  - direct-stable
  - escalated-stable via `LocalNormalized`
  - escalated-stable via `ExactSupport`
  - structured exhaustion after sanctioned strategies are exhausted
- compound-case parity is proven across:
  - direct
  - replay
  - branch-local / preview
  - inspection
  - projection-consumption receipt
- family-boundary certification can guard compound floor drift, not only
  single-threshold drift
- every compound primitive failure still lands in one exact structural bucket:
  - admission rejection
  - realization exhaustion
  - later-chain rejection
- the suite demonstrates stacked failure modes explicitly enough that it is a
  believable pre-METABOSS gate rather than a renamed threshold corpus
- the docs phase would describe a compound primitive proof substrate we would
  actually trust under future MetaBoss pressure

### Phase 6: Freeze Milestone 4 Crate Documentation And Reader Onboarding Surfaces

Turn the shipped Milestone 4 feature surface into crate-local documentation
that future engineers and AI agents can read directly instead of reverse-
engineering the implementation.

#### Why this phase comes now

The docs should describe the system that actually shipped. If this phase comes
earlier, the docs will drift with the implementation. If it is skipped, the
milestone leaves a working substrate that later engineers still have to rediscover
from code.

This phase is also where the milestone protects its hard-won Query integration
story from being lost to history.

#### Do this in this phase

- create or update `docs/README.md` in every Worth crate touched by Milestone 4
- make each touched crate `docs/README.md` explicitly teach that crate's docs
  style, including:
  - what that crate owns
  - what that crate does not own
  - whether its docs are workflow-first, semantic-first, authority-first, or
    pure-geometry-first
  - how and when the reader should jump to neighboring crate docs
- organize docs into folders per category rather than one flat dump
- write one owning doc per shipped public feature surface
- write one owning boundary doc per major Milestone 4 handoff surface
- fold examples into the owning feature doc instead of scattering example-only
  siblings
- make every relevant feature doc teach:
  - what the feature is
  - why you use it
  - stable entry points
  - common path
  - advanced path
  - Query integration, when present
  - inspection / debugging
  - anti-patterns
  - current limits
  - related docs
- make every relevant boundary doc teach:
  - allowed upstream inputs
  - required downstream outputs
  - forbidden bypasses
  - binding artifacts or receipts
  - Query usage, if the boundary touches Query
- document the Query runtime usage story explicitly in the relevant Worth docs
  so later engineers do not reintroduce local runtime folklore
- use the `feature doc writer` skill when it exists in the working environment;
  if it is not available, use an equivalent feature-doc workflow that still
  enforces one-doc-per-feature ownership

At minimum, the milestone should leave behind usable crate-doc surfaces for:

- `worth-kernel`
  - kernel overview
  - execution context and artifact policy
  - primitive construction
  - shell-with-hole construction
  - wire-body construction
  - construction simulation
  - construction replay
  - construction results and diagnostics
  - kernel-to-spatial
  - worth-to-query
- `worth-spatial`
  - spatial overview
  - construction-time birth bindings
  - birth completeness and impossibility
  - birth truth artifacts
  - spatial-to-topo
- `worth-topo`
  - topology authority overview, if widened by Milestone 4
  - any new construction-authority execution or certification surfaces widened
    by Milestone 4
- `worth-geom`
  - geometry overview, if widened by Milestone 4
  - any new public scaffold or carrier-generation feature surfaces widened by
    Milestone 4

#### Do not declare the milestone complete until

- a new engineer can find the right feature doc from the crate `docs/README.md`
  without knowing the implementation tree first
- each touched crate README teaches the intended reading style of that crate's
  docs explicitly instead of assuming the reader will infer it
- the common path and advanced path for each shipped feature surface are taught
  in the owning doc
- the Query integration story is explicit anywhere the Worth feature depends on
  Query
- the major Milestone 4 handoffs are covered by boundary docs rather than
  buried across several feature docs
- there is one owning doc per shipped public feature instead of several
  overlapping partial explanations
- the crate docs are detailed enough that a future AI agent can read the docs
  first and only then drop to the code for implementation detail

### Phase 7: Backfill Worth Crate Documentation For Pre-Milestone-4 Surfaces

Use the Milestone 4 docs model to backfill the older Worth surfaces so future
engineers and AI agents can treat the Worth crate docs as the primary learning
path, not just the Milestone 4 slices.

#### Why this phase comes now

Milestone 4 must document what it ships before it tries to rewrite the rest of
Worth's history. Once the new docs model is proven on the freshly built
surface, the project should use the same model to backfill the older Worth
domains rather than leaving the rest of the series undocumented.

This phase exists because one well-documented milestone on top of three older,
poorly documented crates still leaves future agents falling back to code
archaeology.

#### Do this in this phase

- audit the pre-existing public surfaces in `worth-topo`, `worth-geom`, and any
  already-visible Worth crate APIs that Milestone 4 depends on
- map those surfaces to:
  - owning feature docs
  - owning boundary docs
  - crate README reading-order links
- backfill the missing crate-map, feature, and boundary docs using the same
  docs model Milestone 4 established
- make cross-crate "read next" links explicit so readers can move from:
  - kernel to spatial
  - spatial to topo
  - Worth to `forge-query`
  - newer milestone surfaces back to the foundational Worth docs they inherit
- use the `feature doc writer` skill when it exists in the working environment;
  if it is not available, use an equivalent feature-doc workflow that still
  enforces one-doc-per-feature and one-doc-per-boundary ownership

This phase does not exist to rewrite speculative future docs. It exists to
backfill the already-shipped Worth surfaces that future work will actually
build on.

#### Do not declare the milestone complete until

- the Worth docs model no longer applies only to the Milestone 4 slice
- older public Worth surfaces that Milestone 4 depends on have owning docs and
  readable crate-map entry points
- an engineer can start at the touched Worth crate READMEs and follow the docs
  graph across kernel, spatial, topo, geom, and Query without dropping into
  source immediately
- the backfill is concrete enough that future AI agents can learn the built
  Worth substrate from docs first instead of rediscovering it file by file

## Must Ship

- one explicit `worth-kernel` primitive construction boundary
- one explicit `worth-spatial` construction-time birth boundary
- one explicit kernel -> spatial -> topology primitive lowering chain
- one explicit Query-backed runtime authoring story using:
  - `ForgeQueryWorkspace`
  - direct write surfaces for ordinary authoritative construction steps
  - `workspace.compose_graph(...)` for graph-shaped same-batch authoring
  - existing-truth surfaces for authoritative target reuse and verification
  - preview / branch surfaces for isolated branch-local construction work
  - inspection and projection-consumption surfaces for typed post-execution
    explanation
  - explicit Query boundary-gap escalation instead of Worth-local workaround
    semantics
- admitted primitive family construction workflows over the Milestone 4 family
  ladder
- one phase-typed construction artifact chain from admitted intent through
  certified result
- one canonical construction artifact family that binds kernel, spatial,
  topology, Query, and certification truth into one inspectable result
- direct topology certification of constructed results
- direct spatial birth certification of constructed results
- accepted and rejected replay parity over admitted primitive workflows
- accepted and rejected branch-local parity over admitted primitive workflows
- one compound primitive adversarial certification suite that stacks multiple
  realization stressors in one workload family before docs begin
- one frozen spatial-intent and motion planning model for create / move /
  rotate / reorient / offset semantics before compound primitive adversarial
  suites become the default authoring pressure
- one frozen motion-reference, witness-resolution, and motion-failure planning
  model before compound primitive adversarial suites and later curved-carrier
  motion semantics become the default authoring pressure
- one frozen intent-conflict, candidate-arbitration, and blocked-capability
  planning model before contact-heavy hostile suites and later boolean or BIM
  interaction semantics become the default authoring pressure
- one explicit DX target for motion witness authoring and one explicit DX target
  for intent-conflict arbitration so the common path, advanced path, and
  human-escalation path stay usable as the system scales
- one frozen preview, identity-continuity, and reusable policy-profile model
  before compound hostile suites and later snapping, host, boolean, and fillet
  interaction semantics become the default authoring pressure
- direct rejection-locality artifacts for construction failure
- direct breadth and proof artifacts for construction, assembly, birth
  attachment, and certification work
- one explicit Query boundary-gap register and no-local-runtime-workaround
  audit
- one crate-local documentation surface per touched Worth crate, organized by
  category with one owning doc per shipped feature and one owning doc per major
  Milestone 4 boundary
- one explicit backfill pass over the older Worth crate documentation surfaces
  that Milestone 4 depends on
- a machine-checkable Milestone 4 closeout surface rather than nested helper
  drift

## Must Preserve

- Milestone 1 authority and naming semantics
- Milestone 2 derived-read, invalidation, and rebuild boundaries
- Milestone 3 geometry-free topology editing authority
- `worth-topo` geometry purity
- `worth-geom` purity from topology handles and truth authority
- Query's public runtime authority boundaries:
  - ordinary runtime-backed work enters through `ForgeQueryWorkspace`
  - support posture is read from the public support matrix, not inferred from
    method names
  - basis, preview, branch, inspection, and projection consumption remain
    Query-owned runtime semantics rather than Worth-local reinventions
- explicit separation between:
  - kernel construction intent
  - spatial birth truth
  - topology truth
  - derived inspection and diagnostics
  - certification

Milestone 4 may widen the Worth stack, but it may not blur these boundaries.

## Acceptance Evidence

Milestone 4 is not done because a few primitive constructors work on:

- one cube
- one tetrahedron
- one prism
- one shell-with-hole showcase

Milestone 4 is done only when it emits direct machine-checkable proof surfaces
over the admitted primitive workflow class.

At minimum, Milestone 4 closeout must include:

- `primitive_construction_digest`
- `primitive_construction_phase_chain_report`
- `primitive_family_coverage_matrix`
- `primitive_construction_replay_parity_report`
- `primitive_construction_branch_local_parity_report`
- `construction_birth_truth_digest`
- `construction_birth_completeness_report`
- `canonical_construction_artifact_report`
- `construction_rejection_locality_report`
- `construction_phase_compile_fail_report`
- `worth_query_antibypass_audit_report`
- `query_runtime_authoring_surface_report`
- `query_graph_composition_parity_report`
- `query_existing_truth_binding_report`
- `query_basis_and_preview_parity_report`
- `query_projection_consumption_receipt_report`
- `query_inspection_parity_report`
- `query_boundary_gap_register`
- `query_no_local_runtime_workaround_audit`
- `primitive_realization_strategy_report`
- `primitive_conditioning_witness_report`
- `primitive_stability_class_report`
- `primitive_realization_exhaustion_report`
- `primitive_spatial_intent_plan`
- `primitive_motion_reference_plan`
- `primitive_motion_resolution_policy_report`
- `primitive_motion_dx_surface_report`
- `primitive_intent_conflict_plan`
- `primitive_intent_arbitration_policy_report`
- `primitive_intent_conflict_dx_surface_report`
- `primitive_preview_surface_report`
- `primitive_identity_continuity_report`
- `primitive_policy_profile_report`
- `primitive_construction_compound_adversarial_siege_report`
- `primitive_construction_compound_parity_report`
- `primitive_construction_family_boundary_drift_report`
- `primitive_realization_exhaustion_witness_report`
- `crate_docs_surface_report`
- `feature_doc_coverage_matrix`
- `boundary_doc_coverage_matrix`
- `worth_docs_backfill_report`

Milestone 4 closeout should also expose direct aggregate surfaces for:

- primitive-body topology closure
- family-attributed construction breadth
- family-attributed spatial birth breadth
- topology certification breadth
- accepted versus rejected construction outcome distribution
- primitive realization strategy distribution across admitted and rejected
  cases
- primitive stability-class distribution across direct, escalated, and
  exhausted cases
- topology validator locality for constructed bodies
- construction-time fallback or widening where any breadth claim could not stay
  narrow
- Query support posture by consumed family, including explicit deferred or
  unsupported neighbors that Milestone 4 intentionally does not overclaim
- crate-doc coverage by touched Worth crate and shipped feature family

The closeout should elevate `PrimitiveConstructionCorpusReplaySiege` from a
mere test name into a first-class artifact family by emitting a
machine-checkable parity matrix at minimum containing:

- primitive family
- parameter role
- admitted versus rejected outcome
- direct construction digest
- branch-local digest
- replay digest
- construction birth digest
- realization strategy
- stability class
- rejection class when blocked
- rejection locality and blocking boundary when blocked
- construction breadth
- birth attachment breadth
- certification breadth

Required workload surface:

- primitive-corpus coverage for admitted Milestone 4 family ladders
- arbitrary admitted shell-building workflows over arbitrary admitted face
  counts
- arbitrary admitted body-construction workflows over the admitted primitive
  family ladder
- arbitrary admitted wire-body workflows
- arbitrary admitted shell-with-hole workflows in the admitted class
- branch-local and replayed construction histories

Must verify:

- construction intent lowers through the canonical kernel -> spatial -> topology
  pipeline
- the phase-typed construction chain makes skipped or out-of-order progression
  unrepresentable in the implementation surface
- admitted primitive workflows remain topology-authoritative
- admitted primitive workflows remain spatially explicit at birth
- accepted and rejected primitive workflows replay identically
- accepted and rejected primitive workflows preserve branch-local parity
- topology validators and failure locality remain explicit for constructed
  bodies
- construction-time geometry meaning is not being retrofitted later or inferred
  from topology archaeology
- Worth runtime-backed primitive construction never requires direct lower-runtime
  access below the documented Query crate surfaces
- Worth runtime-backed primitive construction never closes a required boundary
  by inventing Worth-local runtime semantics where Query should own the
  contract
- same-batch graph-shaped construction uses Query graph-composition evidence
  instead of caller-owned batch choreography
- primitive realization truth is explicit:
  - semantic primitive family is distinct from realization strategy
  - realization strategy is distinct from stability class
  - sanctioned fallback and exhaustion are explicit artifact truth rather than
    hidden helper behavior
- the final inspectable construction result is one canonical artifact family,
  not a caller-reconstructed braid of unrelated receipts
- typed identity, membership, source, target, and continuity facts used by
  diagnostics or certification come from Query projection consumption or
  inspection receipts rather than payload archaeology
- the shipped crate docs teach the common path, advanced path, and Query
  boundary story for each public Milestone 4 feature surface

Milestone 4 closes only when admitted primitive construction workflows operate
generically across the admitted family ladder, constructed bodies are born with
explicit construction-time spatial meaning, and later milestones can inherit
that substrate honestly.

## Architectural Notes

- `worth-kernel` should be created in this milestone as a fresh Worth crate
  boundary, not as a port of `forge-kernel`
- `worth-spatial` should also become real in this milestone
- Milestone 4 intentionally does **not** close the full Milestone 5 spatial
  binding story
- the construction scaffold should be geometry-rich enough for later booleans,
  fillets, and curved work to inherit, but Milestone 4 should only implement
  the admitted construction subset of the long-term spatial and kernel
  structure
- old `forge-kernel` and `forge-spatial` code may be mined for ideas or
  hostile scenarios, but they do not define the Worth architecture
- `worth-topo` certification should remain the owner of topology legality proof
  for constructed results even when the workload is authored by `worth-kernel`
- direct machine-checkable proof surfaces should be favored over helper-only
  nested trees
- over-300-line files should be reviewed aggressively during implementation,
  especially in kernel lowering, spatial birth, and certification code

### Allowed Structural Axes

Top-level folders in `worth-kernel` and `worth-spatial` should be created only
when they preserve one dominant axis of meaning such as:

- authority boundary
- workflow family
- truth / derived / diagnostic lifecycle
- proof or certification regime
- runtime / state / policy substrate

Top-level folders should **not** be created for:

- milestone-local phases or provenance
- implementation technique
- generic activity buckets such as `operations`, `helpers`, or `utils`
- tool or vendor names
- temporary growth buckets

Milestone 4 should therefore establish the permanent structural map now and
implement only the first admitted subset inside it, rather than creating
milestone-shaped folders that later need to be renamed or dissolved.

### Target Spatial Structure

`worth-spatial` should start with its long-term domain skeleton even though
Milestone 4 only closes the first admitted subset of it.

```text
crates/worth-spatial/src/
  lib.rs
  facade.rs

  bindings/
    contracts/
    face_surface/
    edge_curve/
    coedge_pcurve/
    vertex_geometry/
    coupled_split_merge/
    seams_periodicity/
    diagnostics/
    certification/

  classification/
    planar_exact/
    coincidence/
    tangency/
    inclusion/
    side_of_surface/
    topology_sensitive/
    diagnostics/
    certification/

  continuity/
    g0/
    g1/
    g2/
    edge_chain/
    face_network/
    seam_pole/
    diagnostics/
    certification/

  intersections/
    curve_curve/
    curve_surface/
    surface_surface/
    graph_build/
    imprint_preparation/
    tangent_events/
    coplanar_overlap/
    diagnostics/
    certification/

  healing/
    sewing/
    gap_crack_hole/
    trim_network/
    degeneracy_resolution/
    import_pathologies/
    diagnostics/
    certification/

  identity/
    binding_identity/
    persistent_name_support/
    structural_identity/
    replay_support/
    branch_local/
    certification/

  projection/
    truth_surfaces/
    diagnostic_surfaces/
    runtime_boundary/

  certification/
    closeout/
    hostile_suites/
    scale_sweeps/
    support/

  test_support/
    carrier_builders/
    binding_scenarios/
    tangent_cases/
    overlap_cases/
    periodic_surface_cases/
    assertion_support/
```

Milestone 4 should primarily populate:

- `bindings/`
  - the construction-time admitted subset of topology/geometry attachment truth
- `identity/`
  - only the subset needed for construction-born identity and replay parity
- `projection/`
  - only the runtime-facing surfaces needed for the admitted Milestone 4 proof
- `certification/`
  - only the Milestone 4 closeout and hostile proof slice
- `test_support/`
  - only the primitive-construction support the admitted proof surface needs

Milestone 5 and later should widen these existing folders rather than renaming
the structure.

### Target Kernel Structure

`worth-kernel` should also start with its long-term workflow and substrate
skeleton even though Milestone 4 only closes the construction slice.

```text
crates/worth-kernel/src/
  lib.rs
  facade.rs

  construction/
    intents/
      primitive_families/
      body_families/
      wire_families/
      shell_hole_families/
    scaffolds/
      simplex/
      orthotope/
      prism/
      pyramid/
      wire_body/
      shell_with_hole/
    lowering/
      kernel_to_spatial/
      kernel_to_topology/
      diagnostics/
    replay/
    branch_local/
    certification/

  booleans/
    intents/
    planning/
    classification/
    imprint/
    split/
    selection/
    stitching/
    cleanup/
    replay/
    branch_local/
    diagnostics/
    certification/

  blends/
    fillets/
      constant_radius/
      variable_radius/
      chains/
      setbacks/
      corner_patches/
      propagation/
    chamfers/
      distance/
      angle/
      two_distance/
      chains/
    blends_common/
      support_selection/
      self_intersection/
      patch_network/
      continuity_contracts/
    replay/
    branch_local/
    diagnostics/
    certification/

  curves_surfaces/
    loft/
    sweep/
    revolve/
    extrude/
    offset_shell_thicken/
    draft_deform/
    analytic_carriers/
    nurbs_carriers/
    trim_authoring/
    replay/
    diagnostics/
    certification/

  feature_graph/
    feature_contracts/
    dependency_graph/
    regeneration/
    invalidation/
    publication/
    replay/
    diagnostics/

  state/
    kernel_state/
    drafts/
    transactions/
    journals/
    checkpoints/
    counters/

  policy/
    exactness/
    degeneracy/
    healing/
    determinism/
    performance_budget/
    timeout/
    capability_switches/

  proof/
    invariants/
    dual_path/
    causal_replay/
    witnesses/
    hostile_suites/
    scale_sweeps/
    closeout/

  projection/
    truth_surfaces/
    derived_surfaces/
    diagnostic_surfaces/
    runtime_boundary/

  test_support/
    primitive_corpus/
    boolean_corpus/
    blend_corpus/
    nurbs_corpus/
    branch_histories/
    hostile_workloads/
    assertion_support/
```

Milestone 4 should primarily populate:

- `construction/`
- the minimum `state/` scaffolding needed for authoritative execution and
  replay-safe construction runs
- the minimum `proof/`, `projection/`, and `test_support/` slices needed for
  Milestone 4 certification

Milestone 4 should **not** create temporary milestone-local top-level folders
that later have to be migrated into this permanent skeleton.

Recommended subdomain split for implementation:

- `worth-kernel`
  - `construction/` plus the minimum supporting `state/`, `proof/`,
    `projection/`, and `test_support/` slices
- `worth-spatial`
  - `bindings/` plus the minimum supporting `identity/`, `projection/`,
    `certification/`, and `test_support/` slices
- `worth-topo`
  - topology authority and certification surfaces only; no geometry-bearing
    expansion

The exact folder names may vary, but the responsibility split should remain
visible and enforceable.

## Sequencing Notes

- Milestone 4 is the first milestone where `worth-kernel` and `worth-spatial`
  become required Worth crates
- Milestone 4 belongs immediately after Milestone 3 because primitive
  construction needs a real topology-edit substrate before anything boolean-like
  can be honest
- Milestone 4 belongs before full spatial binding closure because construction
  birth truth is a narrower and more foundational boundary than broad rebinding
- Milestone 5 should widen the spatial truth boundary from construction birth
  into full topology/geometry binding and rebinding closure
- Milestone 6 should widen exact planar hostility and structural identity over
  the now-honest construction and binding substrate
- booleans, fillets, and later NURBS work should consume Milestone 4 rather
  than replacing it

Passing Milestone 4 does not mean:

- broad rebinding is closed
- exact planar hostility is closed
- booleans are ready
- curved carrier truth is closed

Passing Milestone 4 does mean:

- Worth can construct admitted primitive and body families through a real
  kernel/spatial/topology authority chain
- constructed bodies are no longer topology-only births waiting for later
  interpretation
- later milestones inherit construction truth instead of retrofitting it
