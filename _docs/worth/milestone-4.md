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
- finish `Phase 5` before starting `Phase 6`
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

#### Do this in this phase

- build the hostile corpus and parity suites on top of the canonical artifact
  family
- prove accepted and rejected parity across current-head, branch-local, and
  replayed execution
- prove Query surface parity for authoring, basis, preview, inspection, and
  projection-consumption use
- emit the final machine-checkable closeout reports
- emit the Query gap register and anti-workaround audit as first-class closeout
  artifacts

#### Do not declare the milestone complete until

- admitted primitive construction is certified as a workflow class across
  authority, spatial birth, topology legality, replay, and branch pressure
- the showpiece corpus suite is green
- the phase-typed chain, canonical artifact, and Query anti-bypass surfaces are
  all proven in closeout artifacts
- no remaining required runtime gap is being hidden by a Worth-local workaround

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
- rejection class when blocked
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
