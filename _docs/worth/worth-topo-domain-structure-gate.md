# Worth Topology Domain Structure Gate

> **Status:** Closed; structural gate completed before further Milestone 3
> widening
>
> **Roadmap parent:** [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth_roadmap.md)
>
> **Primary adjacent milestone:** [milestone-3.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/milestone-3.md)
>
> **Primary predecessor side quest:** [worth-query-domain-substrate.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-query-domain-substrate.md)
>
> **Phase 1 migration map:** [worth-topo-domain-structure-migration-map.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-domain-structure-migration-map.md)
>
> **Closeout:** [worth-topo-domain-structure-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-domain-structure-closeout.md)
>
> **Test requirements:**
> - [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/test-requirements.md)
> - [topo-test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/topo-test-requirements.md)

## Goal

Refound the physical and module architecture of `worth-topo` so the crate tells
the topology domain story directly:

- authoritative B-rep topology truth
- rebuildable derived topology
- invariant-family validation
- topology-only operator families
- runtime projection surfaces
- certification and hostile proof
- narrow reusable test support

This gate is not a feature milestone and not a folder beautification pass. It is
a structural prerequisite for resuming broad Milestone 3 edit expansion without
turning `worth-topo` into a pile of query helpers, milestone artifacts, fixtures,
and provenance-shaped names.

## Why This Gate Exists

`worth-topo` is carrying real milestone progress, but its current physical story
does not match the product it is supposed to become.

The crate currently contains several structural smells that will compound as
Milestone 3 widens:

- edit and runtime mechanics live behind tool-shaped names rather than topology
  responsibilities
- query-facing projection code is mixed with domain read meaning and test
  mechanics
- certification code is partly organized by milestone provenance rather than
  proof responsibility
- fixtures and tests are scattered across multiple unrelated locations
- large files hide invariant, report, fixture, and workflow boundaries that
  should be explicit modules
- non-manifold concepts such as radial order, vertex disks, and hostile
  neighborhoods are not visible enough as first-class topology responsibilities

The result is a crate where correct work is possible, but navigation requires
implementation archaeology. That is unacceptable for a topology substrate that
must scale into primitive construction, spatial binding, booleans, NURBS,
fillets, high-valence non-manifold topology, long replay histories, and
eventual aerospace-grade certification.

## Adversarial Constraint

This gate must survive this hostile condition:

> A Parasolid-class topology engineer entering `worth-topo` under high-cardinality
> shell pressure, non-manifold radial stress, vertex-disk ambiguity, long edit
> histories, branch/replay proof pressure, and future NURBS/fillet adjacency
> pressure must be able to locate the correct responsibility without grep
> archaeology, and the crate structure must make it mechanically difficult to
> confuse topology truth, derived topology, edit execution, projection, geometry
> binding, certification proof, or historical provenance.

The gate fails if:

- a new edit family naturally lands in a tool-shaped or provenance-shaped folder
- a derived topology view looks authoritative
- a projection/runtime adapter becomes the domain read model
- certification proof code is organized primarily by milestone chronology
- fixtures remain global enough to hide which topology responsibility they serve
- `worth-topo` gains geometry-binding meaning that belongs in another crate
- NMT, radial, loop, shell, and vertex-disk responsibilities are discoverable
  only by reading implementation files
- the next correct edit is not obvious from the tree

## Product Decision Lock

- `worth-topo` owns topology truth semantics, topology operator semantics,
  topology-derived interpretation, topology validation, topology certification,
  and topology hostility proof.
- `worth-topo` remains geometry-free except for explicitly permitted opaque
  topology-safe identifiers.
- topology-to-geometry binding belongs in a separate crate and must not be
  smuggled into this rearchitecture.
- `forge-query` and the closed read-composition side quest are runtime/query
  substrates. Their names may appear at runtime boundaries, but they must not
  shape the topology domain skeleton.
- `forge-relational` remains the authoritative truth runtime. `worth-topo` may
  define topology meaning and edit contracts, but it must not invent a second
  truth runtime.
- `forge-runtime-bridge` remains the truth-to-derived causality bridge. The
  topology tree may expose projection boundary code, but it must not own bridge
  authority.
- public consumers depend on the `worth-topo` facade, not internal topology.
- provenance names belong in closeout records, audit metadata, and release
  notes, not in permanent domain folders, duplicate roots, or export aliases.
- global `fixtures`, generic `tests`, `helpers`, `utils`, `common`, and
  tool-shaped folders are not acceptable permanent homes for topology meaning.

## Target Domain Story

The target crate skeleton should read like a topology engine before it reads
like a runtime integration.

```text
crates/worth-topo/src/
  lib.rs
  facade.rs

  brep/
    topology_graph/
      body_lump_region/
      shell_face_loop/
      edge_vertex/
      half_edge/
      wire/
      containment/
      adjacency/
      radial_order/
      identity/
      naming/
    snapshots/
    canonical_ordering/

  derived_topology/
    materialized_graph/
    shell_views/
    wire_views/
    loop_cycles/
    radial_rings/
    vertex_disks/
    non_manifold_neighborhoods/
    traversal_views/

  validation/
    reference_integrity/
    ownership/
    containment/
    loop_wiring/
    radial_rings/
    shell_closure/
    vertex_disks/
    naming/
    determinism/

  topology_operators/
    contracts/
    local_rewrites/
      euler_2_manifold/
      entity_lifecycle/
      boundary_wiring/
      radial_cycles/
      vertex_disks/
      cellular_regions/
      sheet_wire_laminar/
      degeneracy_collapse/
      sewing_gluing/
    composite_programs/
    cancellation/
    naming_continuity/
    rejection_locality/
    replay/
    branch_local/
    application/

  projection/
    truth_surfaces/
    derived_surfaces/
    diagnostic_surfaces/
    read_views/
    runtime_boundary/

  certification/
    authority_closeout/
    derived_topology_closeout/
    topology_operator_closeout/
    hostile_topology_operators/
    primitive_corpus/
    scale_sweeps/
    support/

  test_support/
    brep_builders/
    primitive_corpus/
    hostile_neighborhoods/
    branch_histories/
    projected_workspaces/
    certification_assertions/
```

This skeleton is a target semantic map, not a mandate that every leaf folder
must exist on the first implementation patch. Empty ceremony is forbidden. A
folder should be created when the responsibility has real code, tests, or proof
artifacts to own.

## Boundary Semantics

### `brep`

`brep` owns authoritative topology vocabulary and topology graph meaning.

This is where a topology engineer should find the canonical B-rep concepts:
body, lump, region, shell, face, loop, half-edge, edge, vertex, wire,
containment, adjacency, radial ordering, identity, and persistent naming.

Rules:

- `brep` may define topology truth concepts and topology-safe identifiers.
- `brep` must not define geometry carriers, tolerances, p-curves, trims, surface
  equations, or binding continuity.
- `brep` must distinguish identity, naming, lineage, and later binding anchors
  rather than collapsing them into one field or helper surface.
- derived topology may consume `brep`; `brep` must not depend on derived views.

### `derived_topology`

`derived_topology` owns rebuildable topology interpretations over authority.

It is the home for materialized graph views, shell and wire views, loop cycles,
radial rings, vertex disks, non-manifold neighborhoods, and traversal products
that can be destroyed and rebuilt from authority plus declared contracts.

Rules:

- derived views are disposable and must not be manually patched by callers.
- derived views may expose domain meaning, but never authority.
- materialization, interpretation, validation support, parity, and diagnostics
  should remain separable when they have different lifecycle or proof meaning.
- whole-view rebuilds and localized rebuilds must be visible as different
  runtime/proof outcomes.

### `validation`

`validation` owns invariant-family checks over topology truth and derived
topology products.

Validation answers one question:

`is this topology state or edit result valid under this invariant family?`

The folder should make the topology brutality categories visible:
reference integrity, ownership, containment, loop wiring, radial rings, shell
closure, vertex disks, naming, and determinism.

Rules:

- validation code is organized by invariant family, not by caller, milestone,
  helper convenience, or old file location.
- validation outputs must preserve advisory, violation, and success context
  where the domain requires structured outcomes.
- validation must not become the mutation authority. It proves or rejects
  topology states and edit results; it does not smuggle edit policy.
- validator-family coverage must remain attributable in certification artifacts.
- validation must not own corpus generation, hostility orchestration, scale
  sweeps, closeout aggregation, or regression harness logic. Those are
  certification responsibilities.

### `topology_operators`

`topology_operators` owns topology-only operator meaning.

This is where Milestone 3 should become navigable. A new topology operator
family should have an obvious home by topological neighborhood and proof
responsibility: entity lifecycle, boundary wiring, radial cycles, vertex disks,
cellular regions, sheet/wire/laminar topology, degeneracy/collapse,
sewing/gluing, cancellation, naming continuity, rejection locality, replay,
branch-local behavior, and application.

The name is intentionally not plain `operations`. `operations` is too broad and
can attract reads, validators, projections, certification runners, debug
commands, construction programs, and runtime orchestration. `topology_operators`
keeps the kernel vocabulary while preserving the boundary: these are topology
truth-changing or topology-truth-delta-producing transformations.

Rules:

- topology-operator code consumes authoritative topology contracts and emits
  declarative topology effects.
- topology operators must not depend on geometry semantics, primitive
  construction policy, or spatial classification.
- edit contracts, validation, application, naming outcome, diagnostics, and
  replay proof must remain distinguishable.
- broad scans or widened derived fallout must surface explicit fallback
  evidence.

### `projection`

`projection` owns runtime-projected surfaces, not domain truth and not edit
meaning.

Projection answers one question:

`how is already-authorized topology truth, already-built derived topology, or
already-produced diagnostic evidence exposed through a runtime-facing shape?`

The folder exists because Worth must present truth, derived views, diagnostics,
and read families through runtime/query boundaries without letting those
mechanics become the topology domain architecture.

Rules:

- `truth_surfaces` projects authoritative topology truth into runtime-readable
  products.
- `derived_surfaces` projects rebuildable derived topology products.
- `diagnostic_surfaces` projects proof, rejection, fallback, and breadth
  evidence.
- `read_views` contains decoded, disposable domain read products.
- `runtime_boundary` contains Forge Query, runtime bridge, or other mechanism
  adapters that exist specifically to cross runtime boundaries.
- projection may not perform topology interpretation.
- projection may not infer topology legality.
- projection may not synthesize derived topology.
- projection may not decide edit fallout.
- projection may not repair missing authority.
- `projection/read_views` may decode and present topology read products, but it
  must not become the place where domain read meaning is invented.
- projection must not own topology operator semantics. Operator application
  belongs in `topology_operators/application`; projection may expose the result.
- projection must not be named `query_integration`. Calling it that would be
  like naming a domain folder after the transport protocol rather than the
  responsibility.

### `certification`

`certification` owns machine-checkable proof programs and closeout artifacts.

Certification answers one question:

`have we proven that the validator, edit, derived-topology, projection, replay,
branch, scale, and hostile-case systems survive the required corpus and closeout
obligations?`

It should be organized by proof responsibility rather than by milestone
provenance:

- authority closeout
- derived topology closeout
- topology operator closeout
- hostile topology operators
- primitive corpus
- scale sweeps
- certification support

Rules:

- milestone names may remain in public closeout records only when they are the
  permanent public contract, not as a legacy detour.
- permanent internal folders should use proof names, not `milestone_three`.
- certification should emit direct aggregate surfaces rather than requiring
  downstream reconstruction from nested helper artifacts.
- hostile topology proof should be discoverable as a standing certification
  program, not as scattered tests.
- certification must not become a home for one-off validators. New invariant
  checks belong in `validation`; certification may orchestrate, sample,
  aggregate, and prove them.

### `test_support`

`test_support` owns reusable test infrastructure at the narrowest honest scope.

It should contain builders, primitive corpus generation, hostile neighborhood
construction, branch histories, projected workspaces, and certification
assertions only when those tools have shared topology meaning.

Rules:

- global fixtures are forbidden as a permanent dumping ground.
- test support must reduce ceremony without hiding the domain responsibility
  under test.
- support modules should be promotable, deletable, or replaceable with their
  responsibility.
- tests must still falsify production topology rather than mirror broad helper
  convenience.

## Phases

### Phase 1: Freeze The Structural Map

Create the implementation map before moving code.

This phase must identify every current `worth-topo` file and lower it into a
proof-carrying migration map.

The map is not a casual spreadsheet. It is a required artifact that records the
semantic promise made by every move.

Required columns:

| Column | Meaning |
| --- | --- |
| `Current path` | Existing file or folder path being classified. |
| `Current role` | What responsibility the path actually serves today, not what its name claims. |
| `Target path` | Intended destination or deletion target. |
| `Responsibility class` | The target ownership regime. |
| `Public API impact` | Whether public semantics are preserved through permanent target exports or intentionally changed. |
| `Move type` | The mechanical migration kind. |
| `Tests affected` | Direct test, fixture, certification, compile-fail, or doc surfaces impacted. |
| `Risk` | Main correctness, public API, proof, or sequencing risk. |
| `Owner decision` | Explicit decision required before movement, if any. |

`Responsibility class` must be one of:

- authoritative topology truth
- derived topology
- validation
- topology operators
- projection
- certification
- test support
- public facade

`Move type` must be one of:

- `move_only`
- `split`
- `merge`
- `delete`
- `public_contract_preserve`
- `public_contract_break`

The map itself must carry proof posture:

- a `move_only` row asserts semantics are preserved and the path was only
  rehomed
- a `split` row identifies the target responsibilities created by decomposition
- a `merge` row identifies the shared responsibility that justifies unification
- a `delete` row identifies the proof or replacement that makes the path
  unnecessary
- a `public_contract_preserve` row identifies the permanent public contract
  that remains stable without adding an old-name export path
- a `public_contract_break` row identifies the intentional break, migration path,
  and required owner approval

This map should be checked in with the gate implementation or embedded in the
gate closeout. It is the lowering plan for the rearchitecture, not a private
planning note.

This phase must also produce a forbidden-name inventory for permanent topology
structure.

At minimum, the inventory must call out:

- tool-shaped folders such as query-native or integration-shaped names
- provenance-shaped folders such as milestone-only internal modules
- global fixture/test/support buckets
- large files that must split as part of the move
- public exports that would make internal topology part of the external API

This phase is complete when the team can point to one mapping table and explain
where every current file belongs, what semantic promise the move makes, what
tests prove it, and which owner decision, if any, remains unresolved.

### Phase 2: Rehome Truth, Derived Topology, And Validation

Move the non-edit topology foundation into the new domain skeleton.

This phase must establish the first real module homes for:

- authoritative B-rep topology concepts
- topology graph roles and relation vocabulary
- canonical ordering and snapshot-facing topology handles
- materialized graph products
- interpreted shell, wire, loop, radial, and vertex-disk views
- invariant-family validation

The authority and derivation boundary is the hard problem in this phase.

Rules:

- authoritative topology concepts land under `brep`
- rebuildable interpretation lands under `derived_topology`
- invariant-family checks land under `validation`
- no derived view may move into `brep`
- no validation helper may become an edit policy shortcut
- no geometry-binding concept may be introduced to make the move feel tidy

This phase is complete when Milestone 1 and Milestone 2 closeout surfaces still
map cleanly onto the new structure and derived topology remains destroyable and
rebuildable from authority.

### Phase 3: Rehome Projection And Runtime Boundary Code

Move read-family, decoded-view, runtime-bridge, and Query-facing code under the
projection boundary.

This phase consumes the closed Worth read-composition side quest without letting
Query mechanics become the topology domain skeleton.

Rules:

- decoded topology domain read products land under `projection/read_views`
- truth-facing runtime surfaces land under `projection/truth_surfaces`
- derived-facing runtime surfaces land under `projection/derived_surfaces`
- proof, fallback, breadth, denial, and diagnostic surfaces land under
  `projection/diagnostic_surfaces`
- Forge Query and bridge adapters land under `projection/runtime_boundary`
- active callers continue to enter through the `worth-topo` facade
- no caller should learn raw row joins, query-runtime internals, or fallback
  helper mechanics as the normal topology read story

This phase is complete when the phrase "query integration" is unnecessary to
understand the domain structure. The tree should say what is being projected,
not which tool performs the projection.

### Phase 4: Rehome Topology Operators

Move Milestone 3 topology-operator work into neighborhood-family and
proof-family homes.

This phase must transform the current edit shape into a topology-operator story
that can scale to arbitrary admitted workflow classes and eventually hundreds
of operator names without turning each operator into a top-level folder.

At minimum, admitted topology-operator work should classify into:

- `topology_operators/contracts`
- `topology_operators/local_rewrites/euler_2_manifold`
- `topology_operators/local_rewrites/entity_lifecycle`
- `topology_operators/local_rewrites/boundary_wiring`
- `topology_operators/local_rewrites/radial_cycles`
- `topology_operators/local_rewrites/vertex_disks`
- `topology_operators/local_rewrites/cellular_regions`
- `topology_operators/local_rewrites/sheet_wire_laminar`
- `topology_operators/local_rewrites/degeneracy_collapse`
- `topology_operators/local_rewrites/sewing_gluing`
- `topology_operators/composite_programs`
- `topology_operators/cancellation`
- `topology_operators/naming_continuity`
- `topology_operators/rejection_locality`
- `topology_operators/replay`
- `topology_operators/branch_local`
- `topology_operators/application`

Rules:

- operator execution mechanics do not live in `projection`
- topology-operator contracts do not hide under read/query terminology
- topology operators remain geometry-free
- branch/replay/naming/rejection surfaces are explicit, not report helpers
- current in-progress Milestone 3 work must either be integrated into this
  structure or explicitly backed out before closeout

This phase is complete when a new radial, loop, shell/wire, or lifecycle edit
family has one obvious home and one obvious proof path.

### Phase 5: Rehome Certification And Test Support

Move proof programs, hostile suites, fixtures, builders, and helper assertions
into responsibility-shaped homes.

This phase must dissolve the current scattered certification/test topology into
a structure that reveals what is being proven.

Rules:

- milestone closeout public stability may remain public where required
- internal certification folders use proof names rather than milestone
  chronology
- primitive corpus generation is shared only through `test_support` or
  `certification/primitive_corpus` when the sharing is semantically honest
- hostile topology suites are named by scenario family and topology pressure,
  not by implementation batch or old test location
- runtime/projection tests live with the projected surface they falsify or in a
  narrow external contract test when they are facade-level
- no broad `fixtures` folder remains as the default answer for new test data

This phase is complete when a failing topology test identifies the responsibility
that failed from its path and name before the reader opens the file.

### Phase 6: Enforce The Architecture

Add mechanical checks that prevent the new skeleton from decaying into
decorative architecture.

The first enforcement set should include:

- forbidden permanent folder-name scan for tool-shaped, provenance-shaped, and
  generic bucket names
- Rust line-cap guard compliance for production and test files
- direct-file-count review for folders that are becoming flat dumps
- public facade/deep-import checks where feasible
- geometry-dependency purity checks for `worth-topo`
- certification registration checks so hostile categories cannot disappear
  silently
- docs checks ensuring roadmap, gate spec, and developer-facing topology docs
  agree on the admitted structure

This phase is complete when the new structure is protected by compiler
visibility, tests, CI checks, or explicit certification rows rather than by
memory and good intentions.

## Must Ship

- a responsibility-mapped `worth-topo` module tree that follows the target
  domain story or documents each intentional deviation
- a narrow facade that remains the external surface while internal topology is
  free to evolve
- authoritative topology concepts under `brep`
- rebuildable derived topology under `derived_topology`
- invariant-family validation under `validation`
- Milestone 3 topology operator work under `topology_operators`
- runtime-projected surfaces under `projection`
- proof programs under responsibility-shaped `certification`
- reusable test infrastructure under narrow `test_support`
- elimination of permanent tool-shaped, provenance-shaped, and generic bucket
  folders from the topology skeleton
- migration of scattered tests and fixtures into responsibility-shaped homes
- mechanical enforcement sufficient to prevent immediate regression

## Must Preserve

- Milestone 1 authority semantics and closeout proof surfaces
- Milestone 1 persistent naming truth and same-commit graph mutation integrity
- Milestone 2 rebuildable derived topology semantics and closeout proof
  surfaces
- the closed Worth read-composition side quest and its Query-backed domain read
  facade
- Milestone 3 topology-only edit boundary and admitted workflow intent
- `worth-topo` geometry purity
- public facade stability through permanent target exports, without legacy
  facade aliases or duplicate export paths for external callers
- branch/replay parity evidence for admitted topology families
- certification artifact meaning, even when internal module paths change

## Acceptance Evidence

This gate closes only with structural and behavioral evidence.

Required evidence:

- a proof-carrying file-to-responsibility migration map produced in Phase 1,
  using the required columns and controlled `Responsibility class` / `Move type`
  vocabularies
- every `public_contract_break` row has explicit owner approval and a migration
  path before implementation
- every `split`, `merge`, or `delete` row identifies the proof surface that
  preserves or replaces the old behavior
- no permanent `query_native`, generic `fixtures`, generic `helpers`, generic
  `utils`, generic `common`, or milestone-provenance internal folder remains in
  `worth-topo`
- line-cap guard passes or every exception is explicitly justified in a roadmap
  or spec allowlist
- `worth-topo` geometry-purity check passes
- public callers can use the facade without importing internal domain modules
- Milestone 1 closeout still verifies against the moved structure
- Milestone 2 closeout still verifies against the moved structure
- the closed read-composition side quest still verifies against the moved
  projection structure
- current Milestone 3 certification and hostile topology-operator tests still
  compile and pass or are explicitly rehomed with equivalent proof names
- `cargo fmt --check`
- `cargo test -p worth-topo`
- any crate-level public API or compile-fail tests needed to protect facade
  discipline

The gate is not closed by a tree snapshot alone. It closes when the old skeleton
can no longer be the convenient place to put new work.

## Architectural Notes

### Legacy `forge-topo` Reference Bar

The old `forge-topo` skeleton is a useful seriousness reference, not a target to
copy mechanically.

Worth should inherit the good signals:

- topology concepts are visible
- operations are grouped by topology work
- validators are explicit family surfaces
- stress and brutality are real proof categories
- persistent naming and topology queries are not afterthoughts

Worth should not inherit old weaknesses:

- broad operation/test buckets
- provenance as a permanent architecture axis
- layer/template names that do not answer authority questions
- geometry or runtime mechanisms shaping topology ownership

### Non-Manifold Topology Is First-Class

The new skeleton must make NMT concepts obvious:

- `radial_order`
- `radial_rings`
- `vertex_disks`
- `non_manifold_neighborhoods`
- radial splice and reseating edit families
- hostile radial and vertex-disk certification

These names matter because non-manifold failure is not a rare corner of Worth.
It is a central pressure surface for primitive construction, booleans, blends,
branch-local edit histories, and hostile certification.

### Two-Dimensional And Three-Dimensional Edges

`worth-topo` should not split 2D and 3D edge concepts merely because later
geometry carriers differ.

The rule is:

- if the distinction is geometric embedding, carrier, p-curve, trim, tolerance,
  or surface binding, it belongs outside `worth-topo`
- if the distinction changes pure topology legality, adjacency, boundary
  membership, radial structure, or traversal meaning, it may earn a topology
  concept here

This keeps `worth-topo` topology-pure while leaving room for later spatial and
kernel crates to bind topology to geometry honestly.

### Projection Is Not Query Integration

The correct folder name is `projection` because the responsibility is projecting
truth, derived topology, diagnostics, and read views across runtime boundaries.

The wrong mental model is "Query integration." Query is a mechanism. The domain
responsibility is the projected surface and its proof posture.

### Provenance Compatibility

Some public report names may keep milestone terminology because those names are
historical audit or roadmap-facing public surfaces.

That exception does not license permanent internal folders named after
milestones, implementation batches, tickets, or recent side quests. Internal
structure should say what the code owns now, not how the code arrived.

## Sequencing Notes

This gate belongs after the closed Worth read-composition side quest and before
further Milestone 3 expansion.

Reason:

- the read-composition side quest already fixed the query/product hole that was
  blocking Milestone 3 read-heavy work
- the remaining blocker is now local to `worth-topo`: the topology crate's own
  architecture does not tell the domain story clearly enough to scale edit
  families safely
- broad Milestone 3 expansion will add more topology operators, proof,
  projection, and hostile-test code, which would calcify the bad skeleton if
  the structure is not fixed first
- Milestone 4 and later consume topology operators as a foundation, so this
  gate is cheaper and safer now than after primitive construction, spatial
  binding, booleans, NURBS, or fillets depend on the current internal topology

This gate is not a substitute for Milestone 3. It is the structural pause that
lets Milestone 3 continue without leaving a mess for every later milestone.

## Self-Check

- Does this gate solve a real structural problem rather than package work
  cosmetically? Yes. It attacks responsibility confusion that would otherwise
  compound under Milestone 3 and later topology scale.
- Is the adversarial constraint precise and load-bearing? Yes. It centers
  navigability and correctness under NMT, replay, branch, edit, and future
  NURBS/fillet pressure.
- Does the gate preserve crate authority boundaries? Yes. It keeps topology in
  `worth-topo`, geometry binding outside, query/bridge/runtime mechanics at
  projection boundaries, and public access through the facade.
- Does the gate define proof obligations rather than only implementation tasks?
  Yes. It requires migration maps, forbidden-name scans, line-cap compliance,
  purity checks, facade checks, and milestone closeout preservation.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes. The target skeleton, boundary semantics, and phases provide
  direct module homes and proof expectations.
- Does the gate belong in the roadmap sequence? Yes. It belongs after the
  closed read-composition side quest and before broader Milestone 3 widening.
