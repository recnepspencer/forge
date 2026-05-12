# Worth Forge Query Runtime Kernel Hard-Break Spec

> **Status:** Proposed hard-break spec for the active Forge Query runtime rewrite
> gate, refreshed against the shipped 2026-05-01 `forge-query` runtime
> graph-authoring and authority-evidence closeouts
>
> **Owners:** `worth-schema`, `worth-topo`, `forge-query`,
> `forge-runtime-bridge`
>
> **Roadmap position:** refinement of the existing `Forge Query Runtime Rewrite
> Gate`, still blocking further Worth Milestone 3 expansion until Worth uses
> the real bridge-backed Query kernel path instead of the compatibility mirror
>
> **Shipped upstream closeouts this spec now assumes are real:**
> - [../forge-query/runtime-authoritative-mutation-evidence-closeout.md](../forge-query/runtime-authoritative-mutation-evidence-closeout.md)
> - [../forge-query/runtime-generic-graph-authoring-closeout.md](../forge-query/runtime-generic-graph-authoring-closeout.md)

## Goal

Define the exact production-grade hard break required to make Forge Query the
real Worth runtime kernel rather than a compatibility mirror over Worth-owned
authority and read orchestration.

## Why This Spec Exists

The current rewrite gate already rejects public compatibility stories, but the
code and spec were out of sync about what Query still needed to invent versus
what Query has already shipped.

That upstream distinction matters now.

As of the shipped Query closeouts, the generic runtime substrate already
admits:

- bridge-backed authoritative mutation evidence
- same-batch graph composition through:
  - `workspace.compose_graph(...)`
  - `workspace.compose_graph_with_invariant_pack(...)`
- typed graph-program denial and denied-path admission traces
- distinct domain-invariant denial with attempted-shape summary
- verified existing-target update / retarget / supersession / retirement lanes
- composition-level assumption/read-set and lineage summaries
- geometry-pressure proof for:
  - `LoopSuccessorRewire`
  - `FailedNonManifoldAdmission`
  - `FaceInnerLoopInsertion`
  - `EdgeSplit`

So this spec is no longer about asking Query to invent a generic graph-shaped
mutation substrate from scratch.

This spec now exists to forbid the remaining mirror-runtime shape inside Worth
and replace it with one runtime story:

`Worth domain intent -> Forge Query runtime -> bridge-backed write/read authority -> canonical relational truth`

Today, Worth still has query-shaped seams while relying on mirror-era
infrastructure such as:

- `worth_topology_query_workspace(...)` built with
  `ForgeQueryRuntime::builder().compatibility_in_memory_collections(...)`
- `WorthTopologyQueryAssembly::import_read_view(...)` as a truth-import step
- `materialized_topology_from_query_rows(...)` rebuilding fake relational read
  records from Query rows
- `WorthTopologyReader` as a direct runtime read orchestrator
- `WorthTopologyEditRunner` and `WorthTopologyAuthority` as the real ordinary
  write path

That is still not a hard break. It is a compatibility mirror with a
query-native facade.

## Hard Part

The hard part is no longer inventing generic mixed-shape graph authoring.
Query already ships that.

The hard part is deleting the remaining Worth-local mirror behaviors without
accidentally reintroducing a second runtime story in nicer clothes.

The hard part is keeping six things separate that the current Worth shape
collapses:

- canonical relational truth
- Query as the real runtime/query facade over that truth
- Worth domain lowering and interpretation
- Query-owned graph authoring, receipts, inspection, and denied-path evidence
- derived materialization / diagnostics / certification surfaces
- temporary migration helpers that must never become the surviving kernel

The design fails if:

- the new runtime constructor still secretly assembles a compatibility mirror
- the new write path still routes ordinary execution through the old public
  `WorthTopologyAuthority` facade under a Query-shaped wrapper
- read/materialization still round-trips through fake relational record
  reconstruction because the old materializer remained structurally central
- topology edits keep bypassing shipped Query graph composition and verified
  existing-target lanes by rebuilding local graph orchestration in Worth
- branch-local or certification flows keep a second runtime story "just for
  now" and later make that fallback permanent
- the public DX looks Query-native while the real operational truth still
  depends on old Worth orchestration

This spec therefore has to define one canonical runtime path, one canonical
artifact family, and one exact deletion bar for the mirror-runtime shape.

## Explicit Assumptions

- Forge Query already has real bridge-backed runtime assembly and write-
  authority seams; this spec is about building the missing Worth production
  adapters and deleting the compatibility mirror, not inventing a second query
  engine.
- Forge Query's shipped public graph-authoring contract is now a dependency,
  not a speculative future:
  - `workspace.compose_graph(...)`
  - `workspace.compose_graph_with_invariant_pack(...)`
  - graph-composition support rows
  - graph-composition denial and domain-invariant denial artifacts
  - verified existing-target mutation lanes
  - composition-level assumption/read-set and lineage summaries
- `forge-relational` remains the authority for commit, branch, lineage, and
  replay semantics.
- `forge-runtime-bridge` remains the authority for truth-to-derived causality,
  invalidation routing, and write/read bridge semantics.
- `forge-query` remains the authority for typed workspace/runtime DX, mutation
  authoring, receipt/inspection/state/materialization surfaces, and public
  support posture.
- Worth continues to own topology vocabulary, topology edit meaning, topology
  interpretation, and topology-domain certification meaning.
- Query-backed materialization may decode Query rows into Worth domain
  projection types, but it must not rebuild fake authoritative relational
  records to re-enter old orchestration.
- Worth topology edits that need symbolic same-batch construction, verified
  rewires, retarget, supersession, or domain-invalidity evidence should use the
  shipped Query graph-authoring contract rather than asking Worth to re-own
  those generic runtime semantics.
- Unsupported Query capabilities must fail typed and early rather than
  triggering Worth-local substitute runtime behavior.

## Product Decision Lock

- This is a kernel replacement, not a compatibility migration.
- The surviving ordinary Worth runtime path must be bridge-backed Query all the
  way down.
- The compatibility in-memory Query workspace is not an allowed end-state seam.
- The old public `WorthTopologyAuthority`, `WorthTopologyReader`, and old-form
  `WorthTopologyEditRunner` are not allowed to survive as ordinary runtime
  entrypoints.
- Query receipts, state, materialization, and inspection are the canonical
  ordinary runtime artifact families.
- Query graph-composition receipts, inspection, admission traces, and
  domain-invariant summaries are the canonical ordinary runtime artifact
  families for graph-shaped topology edits.
- Worth-specific envelopes may survive only as retained or derived proof where
  they do not compete with the Query runtime contract.
- If a generic runtime/query capability is missing, the fix belongs in
  `forge-query` or its production adapters, not in a Worth-local mirror or
  fallback runtime.

## Adversarial Constraint

Worth must survive the following hostile condition without preserving any
mirror-runtime or dual-authority fallback:

> The same admitted topology workflow, certification workflow, materialization
> workflow, and graph-shaped edit workflow must produce the same canonical
> truth mutation, the same Query receipt and inspection meaning, the same
> denied-path classification, and the same derived materialization /
> diagnostics conclusions whether it is executed live, branch-local, replayed,
> or historically reopened, with no path allowed to import truth into a fake
> workspace, reconstruct fake relational records from Query rows, or route
> ordinary writes through direct `WorthTopologyAuthority` entrypoints outside
> the Forge Query runtime.

If any ordinary Worth path:

- commits through direct Worth authority and only mirrors into Query later
- reads through direct Worth reader orchestration and only imports into Query
  for secondary inspection
- reconstructs fake `EntityReadRecord` / `RelationReadRecord` values from Query
  rows to feed domain materializers
- rebuilds local graph-program orchestration instead of using shipped Query
  graph composition for same-batch symbolic, verified, lineage-carrying, or
  domain-invalid edit families
- requires UI, certification, or editor consumers to choose between Worth
  authority artifacts and Query artifacts
- exposes a public or ordinary-internal runtime path that bypasses the real
  bridge-backed Query runtime

then the hard break has failed.

## Phases

### Phase 1: Freeze The Shipped Query Dependency Contract

Lock exactly which upstream Query surfaces this rewrite is allowed to depend on
and which old assumptions are now obsolete.

This phase must explicitly treat the following as already shipped generic
runtime substrate rather than as future Query hardening:

- `workspace.insert(...)`
- `workspace.update(...)`
- `workspace.delete(...)`
- `workspace.batch(...)`
- `workspace.compose_graph(...)`
- `workspace.compose_graph_with_invariant_pack(...)`
- graph-composition capability rows and extension-hook rows
- graph-composition admission traces and domain-invariant summaries
- verified existing-target update / retarget / supersession / retirement lanes
- composition-level assumption/read-set and lineage summaries

This phase forbids the refreshed Worth spec from claiming that generic
same-batch graph composition, verified existing-target rewires, or
domain-invalid graph denial still need to be invented in Query before Worth can
continue.

### Phase 2: Build The Production Worth Query Runtime Assembly

Introduce the real runtime constructor and the Worth-owned adapter seams needed
to assemble Forge Query against canonical runtime truth.

This phase must produce:

- a production `worth_topology_runtime(...)` constructor over the real
  relational runtime
- Worth-backed Forge Query source, schema, and write-authority adapters
- bridge-backed live/computed routing over the real runtime rather than over a
  compatibility mirror
- explicit support/admission posture for every admitted Worth query runtime
  family

This phase is complete only when the ordinary Worth workspace no longer depends
on `compatibility_in_memory_collections(...)`.

### Phase 3: Replace Mirror Read And Materialization Orchestration

Delete the round-trip where Worth reads real truth, imports it into Query, then
rebuilds fake relational records from Query rows.

The surviving read/materialization path must:

- read through the real Query runtime
- materialize through Query live/computed surfaces directly
- inspect through Query inspection directly
- certify through Query-backed state and inspection artifacts directly

This phase must explicitly kill the ordinary use of:

- `WorthTopologyQueryAssembly::import_read_view(...)`
- `materialized_topology_from_query_rows(...)`
- `entity_record_from_query_row(...)`
- `relation_record_from_query_row(...)`
- direct `WorthTopologyReader` orchestration as the ordinary public/runtime
  path

### Phase 4: Replace Direct Worth Authority With Query-Owned Authority

Move the real ordinary Worth truth write path onto the bridge-backed Forge
Query runtime.

The surviving write path must:

- author topology writes through Forge Query mutation families
- lower them through Query-owned Worth adapters into canonical runtime truth
- return Query receipts as the canonical mutation artifact
- preserve Worth-specific authority meaning as retained Query receipt and
  inspection evidence, not as a parallel public envelope

This phase is complete only when direct `WorthTopologyAuthority` commit APIs
are not the ordinary path for any surviving public or ordinary-internal Worth
workflow.

### Phase 5: Rewrite Graph-Shaped Topology Edits Onto Query Composition

Rebuild topology edit execution so edit authoring remains Worth-domain
meaningful, but graph-shaped execution itself is Query-native all the way down.

This phase must move the first admitted Worth edit families onto the shipped
Query graph-authoring substrate rather than onto Worth-local orchestration.

The surviving edit path must:

- use plain Query mutation families when the edit is honestly scalar
- use `workspace.compose_graph(...)` when one edit needs symbolic same-batch
  declaration, existing-target identity preservation, lineage, or verified
  existing-truth checks in one canonical program
- use `workspace.compose_graph_with_invariant_pack(...)` when the edit may be
  substrate-valid but topology-invalid and needs domain-owned rejection without
  collapsing into generic support denial
- route fallout from Query receipts and computed surfaces
- drive edit inspection and certification from Query artifacts directly
- deny typed and early when a needed Query mutation family is still unsupported

This phase is complete only when `WorthTopologyEditRunner` no longer owns the
real ordinary execution path in its current authority/reader-owned form.

### Phase 6: Delete The Old Shape And Close On The New One

Delete the old public and ordinary runtime shapes aggressively once the real
Query-backed kernel exists.

Delete or privatize:

- `WorthTopologyReader` as an ordinary public/runtime orchestrator
- `WorthTopologyAuthority` as an ordinary commit API
- `WorthTopologyEditRunner` in its old direct-authority form
- truth-import compatibility paths that exist only to feed Query mirrors
- Query-row-to-fake-relational-record translation helpers that exist only
  because the runtime is not yet truly Query-native
- any Worth-local graph-program builder whose only job is to emulate shipped
  Query graph-composition semantics above the facade

This phase closes only when the surviving Worth public/runtime story is
Query-native in implementation, evidence, docs, and examples.

## DX Output Standard

The hard break is not complete because we renamed APIs. It is complete when the
ordinary developer surface looks like the real kernel.

## Required Surface Primitives

The implementation must map to concrete surviving surfaces, not only to prose:

- one production constructor:
  - `worth_topology_runtime(&mut RelationalRuntime, workspace_name) -> Result<ForgeQueryWorkspace, ...>`
- one production assembly declaration:
  - `WorthTopologyQueryAssembly::declare(&mut ForgeQueryWorkspace) -> Result<..., ...>`
- one production write artifact family:
  - `ForgeQueryWriteReceipt`
  - `ForgeQueryBatchWriteReceipt`
- one production graph-shaped write family:
  - `workspace.compose_graph(...)`
  - `workspace.compose_graph_with_invariant_pack(...)`
- one production inspection family:
  - `workspace.inspect(...)`
- one production state/materialization family:
  - `workspace.read(...)`
  - `workspace.materialize(...)`
  - `workspace.state(...)`
- one production support/inspection contract:
  - public support metadata that says exactly which Worth runtime families are
    admitted on the real bridge-backed path
- one production edit execution surface:
  - Worth edit authoring lowered into Query mutation authoring on the same
    runtime, including graph-shaped composition where the edit really needs it

If a proposed implementation cannot point to these exact surviving surfaces, it
is not implementing this spec honestly.

The surviving DX must have this shape:

- one runtime constructor:
  - `worth_topology_runtime(&mut runtime, workspace_name)`
- one assembly declaration flow:
  - `WorthTopologyQueryAssembly::declare(&mut workspace)`
- one ordinary write path:
  - Query mutation authoring and Query receipts
- one ordinary read/materialization path:
  - `workspace.read(...)`
  - `workspace.materialize(...)`
  - `workspace.state(...)`
  - `workspace.inspect(...)`
- one ordinary edit path:
  - Worth edit contracts lowered into Query mutation authoring on the same
    runtime
  - graph-shaped edits using Query graph composition instead of a Worth-local
    mirror program builder when symbolic same-batch, verified rewire, lineage,
    or domain-invalidity evidence is required

The DX must not require ordinary consumers to:

- import relational truth into a fake workspace
- reconstruct relational records from Query rows
- choose between Worth authority artifacts and Query artifacts
- manually wire invalidation, derived fallout, or certification bridging

## Concrete Before / After Standard

### Before: compatibility mirror and reconstruction

This is the current wrong shape from `worth-topo`:

```rust
let mut workspace = worth_topology_query_workspace("worth.milestone-one.certification")?;
let assembly = WorthTopologyQueryAssembly::declare(&mut workspace)?;
let receipt = assembly.import_read_view(&mut workspace, read_view, &read_basis)?;
let entity_rows = workspace.read(assembly.entities());
let relation_rows = workspace.read(assembly.relations());
let materialized = materialized_topology_from_query_rows(&entity_rows, &relation_rows)?;
```

And `materialized_topology_from_query_rows(...)` currently does this:

```rust
let entity_records = entity_rows
    .iter()
    .map(entity_record_from_query_row)
    .collect::<Result<Vec<_>, _>>()?;
let relation_records = relation_rows
    .iter()
    .enumerate()
    .map(|(index, row)| relation_record_from_query_row(index as u64, row))
    .collect::<Result<Vec<_>, _>>()?;
WorthTopologyMaterializer::materialize_from_records(&entity_records, &relation_records)
```

That round-trip is exactly what this spec forbids.

### After: real Query-backed runtime

The surviving shape must look like this:

```rust
let mut workspace = worth_topology_runtime(&mut runtime, "worth.milestone-one.certification")?;
let assembly = WorthTopologyQueryAssembly::declare(&mut workspace)?;

let write_receipt = workspace.compose_graph_with_invariant_pack(
    worth_topology_invariant_pack(),
    |graph| {
        // Worth-domain lowering authors Query mutations here against real runtime truth.
        // Existing-target rewires, supersession, symbolic same-batch declarations,
        // and domain-invalid rejection all stay in the ordinary Query receipt story.
        Ok(())
    },
)?;

let topology_rows = workspace.read(assembly.entities());
let materialized_rows = workspace.materialize(assembly.materialized());
let receipt_inspection = workspace.inspect(&write_receipt)?;
let validation_state = workspace.state(assembly.validation())?;
```

Scalar truth-only edits may still look like this:

```rust
let write_receipt = workspace.batch(|batch| {
    Ok(())
})?;
```

The key difference is architectural, not cosmetic:

- no truth import from `RelationalReadView`
- no fake `EntityReadRecord` / `RelationReadRecord` reconstruction
- no second runtime story
- Query receipt / state / inspection are the canonical surfaces
- graph-shaped edits use shipped Query graph composition rather than a
  Worth-local mirror program builder

The surviving materialization path may decode Query-derived rows into domain
projection types, but it must never reconstruct fake authoritative relational
records just to re-enter old Worth materializers.

## First-Ship Scope Rule

This spec must define a conservative first ship so implementation cannot hide a
naive trap behind "we'll make the real runtime later."

### In Scope For First Ship

- one real bridge-backed Worth Query runtime constructor
- one real Query-backed topology assembly over that runtime
- admitted topology write families already supported honestly by Forge Query
- admitted topology read/materialization/certification paths over the same
  runtime
- at least one admitted topology edit family executing on that same runtime
  through Query graph composition rather than through Worth-local graph
  orchestration
- explicit typed failure for unsupported branch, naming, continuity, geometry,
  or edit families

### Explicit First-Ship Debt

- broader edit-family coverage after one admitted family proves the kernel path
- finer-grained recompute or locality optimization where current Query support
  still exposes explicit whole-view debt
- ergonomics polish after the real runtime path and deletion bar are proven
- later Worth-specific edit DSL refinement after the Query-native execution
  contract is already proven

### Not Allowed As Debt

- compatibility in-memory runtime assembly
- truth import as the ordinary certification/materialization path
- row-to-fake-record reconstruction as the ordinary materialization path
- direct Worth authority or reader orchestration as a hidden ordinary path
- Worth-local graph-program execution that exists only because the code did not
  move onto shipped Query graph composition yet
- public examples or tests that still teach the old runtime shape as normal

## Must Ship

- a production Worth Query runtime constructor over the real runtime
- Query-backed Worth source / schema / write-authority adapters
- Query-native materialization / diagnostics / certification paths
- Query-native topology edit execution
- graph-shaped topology edit execution that uses shipped Query graph composition
  when the edit honestly needs symbolic same-batch, verified existing-target,
  lineage, or domain-invalidity semantics
- explicit DX and support documentation for the surviving runtime shape
- compile-fail or privacy boundaries preventing external minting of proof-
  bearing runtime artifacts where appropriate

## Must Preserve

- `forge-relational` remains truth authority
- `forge-query` remains the typed runtime/query facade, not a second truth
  engine
- `forge-runtime-bridge` remains the causality and invalidation bridge
- authority and derivation remain structurally separate
- unsupported Query families remain typed and fail-closed rather than causing
  Worth-local substitute runtime behavior

## Mechanical Enforcement Requirements

- the production runtime constructor must require an explicit adapter bundle or
  equally exhaustive construction surface so omitted adapters are a compile
  error rather than a runtime surprise
- compatibility workspace construction must not remain reachable through the
  ordinary public/runtime facade
- `WorthTopologyReader`, `WorthTopologyAuthority`, and the old-form
  `WorthTopologyEditRunner` must not remain on the ordinary public facade
- any temporary import-only or reconstruction-only helpers that survive during
  implementation must be `pub(crate)` or test-only and must fail
  compile-fail/public-API tests if exposed through the surviving facade
- the production write path must not merely wrap the old public
  `WorthTopologyAuthority` facade under a Query-shaped shell; reusable lower
  semantics may be extracted below the facade, but the old facade must not
  remain the engine

## Required Contract Surfaces

The implementation must make these contracts concrete enough that code can map
to them honestly:

- `WorthTopologyRuntimeAdapters`
  - exhaustive production adapter bundle for runtime construction
- `WorthTopologyRuntimeSupport`
  - public support artifact describing admitted Worth runtime families on the
    real bridge-backed path
- `WorthTopologyRuntimeFailure`
  - typed runtime assembly / capability / basis failures for the public facade
- `WorthTopologyOperatorExecution`
  - typed edit execution result family over Query receipts/inspection rather
    than direct authority envelopes
- `WorthTopologyGraphEditLowering`
  - typed lowering seam that decides when an edit stays scalar Query mutation
    and when it must become Query graph composition
- `WorthTopologyRuntimeCloseout`
  - support/closeout artifact binding the admitted kernel families and deletion
    posture

Equivalent names are acceptable if the responsibilities remain distinct.

## Compile-Time Boundary Rule

- ordinary external callers must not be able to construct the production
  runtime without supplying the full required adapter set
- ordinary external callers must not be able to select compatibility workspace
  construction through the surviving facade
- ordinary external callers must not be able to mint proof-bearing runtime
  support or closeout artifacts directly
- ordinary external callers must not be able to call the old direct authority
  or reader runtime entrypoints through the surviving facade
- ordinary external callers must not be taught or forced to choose between a
  Worth-local graph edit executor and Query graph composition for the same
  admitted workflow class
- compile-fail tests must exist for:
  - compatibility runtime construction through the surviving facade
  - exposure of old direct authority/reader/edit entrypoints on the surviving
    facade
  - external construction of proof-bearing support/closeout artifacts

## Required Adversarial Cases

- compatibility-assembly denial:
  prove the surviving ordinary Worth runtime does not assemble through
  `compatibility_in_memory_collections(...)`
- mirror-read denial:
  prove ordinary certification/materialization paths do not require
  `import_read_view(...)`
- row-reconstruction denial:
  prove ordinary materialization does not rebuild fake
  `EntityReadRecord` / `RelationReadRecord` values from Query rows
- dual-artifact denial:
  prove ordinary consumers can rely on Query receipt/state/inspection as the
  canonical runtime artifact family without consulting direct Worth authority
  envelopes
- edit-path parity:
  prove at least one admitted edit family lowers into Query mutation authoring,
  executes through the same runtime, and routes fallout through Query receipts
  and computed surfaces
- graph-composition adoption:
  prove at least one admitted Worth graph-shaped edit family lowers into
  `workspace.compose_graph(...)` or
  `workspace.compose_graph_with_invariant_pack(...)` instead of a Worth-local
  mirror program
- domain-invalidity honesty:
  prove a topology-invalid but substrate-supported Worth edit returns the Query
  domain-invariant denial lane rather than collapsing into local Worth failure
  folklore
- branch-local honesty:
  prove admitted branch-local behavior uses the same Query-backed runtime or
  fails typed and early where Query support is not yet admitted
- replay / historical parity:
  prove the same authoritative history produces the same Query-native
  materialization and inspection meaning without mirror import glue
- public-surface denial:
  prove compile-fail / public-API tests reject ordinary dependence on
  `WorthTopologyReader`, `WorthTopologyAuthority`, old-form
  `WorthTopologyEditRunner`, and compatibility workspace construction
- unsupported-neighbor fail-closed behavior:
  prove unsupported Query mutation/read families deny typed and early instead
  of triggering Worth-local substitute orchestration

## Complexity / Counter Obligations

- runtime assembly must expose whether the production bridge-backed path or an
  explicit denied/unsupported path was taken; silent fallback is forbidden
- write receipts or their inspection surfaces must expose touched-aspect count,
  affected live-view count, affected computed-surface count, and any explicit
  fallback class
- graph-shaped edit receipts or their inspection surfaces must expose the
  graph-composition program, resolution map, lifecycle outcomes, and any
  admitted assumption/read-set or lineage summaries required by the workflow
- materialization/certification paths must expose row breadth and fallback
  breadth so whole-view rebuild debt cannot hide behind a Query-native facade
- hostile tests must assert exact counter presence for:
  - touched aspect breadth
  - affected live/computed breadth
  - graph-composition breadth where graph-shaped edits are admitted
  - materialization breadth
  - fallback class / fallback count

Minimum named paths:

- `worth_query_runtime_construction`
- `worth_query_write_execution`
- `worth_query_materialization`
- `worth_query_edit_execution`
- `worth_query_certification`

The spec is not closed by "it feels cleaner." Each named path needs visible
cost posture and explicit debt where optimization is not yet certified.

## Acceptance Evidence

- a query-native Worth runtime construction test over the production runtime,
  not compatibility memory collections
- a hostile read/materialization certification test proving the system no
  longer imports truth just to feed Query
- a hostile write-path test proving ordinary topology mutation enters through
  the real Query runtime and returns canonical Query receipts
- a hostile edit-path test proving at least one admitted edit family executes
  entirely through the same Query runtime
- a hostile graph-shaped edit-path test proving at least one admitted Worth
  edit family uses Query graph composition on the real runtime and preserves
  the exact receipt / inspection evidence the workflow class needs
- public-surface tests proving direct reader/authority/runner execution APIs
  are no longer required by ordinary external Worth use
- docs/examples that show only the surviving Query-native kernel story

## Architectural Notes

- This spec belongs before further Worth Milestone 3 widening because topology
  editing on top of the compatibility mirror would certify the wrong kernel.
- This spec is intentionally stricter than a migration-safe adapter plan. It
  rejects old migration seams as an end-state because this runtime is the foundation
  for later editor, rendering, and cube-generation work.
- If a required capability is missing in Forge Query, the fix belongs in
  `forge-query`, not in a new Worth-local runtime substitute.
- "Query-native" in this spec means Query is the real runtime path, not that
  Worth mutations or reads happen elsewhere and later become Query-shaped.
- The upstream Query closeouts changed the shape of this spec. Generic
  graph-shaped mutation authoring is now closed upstream, so the remaining
  burden here is consumption, deletion, and Worth-domain workflow closure.

## Sequencing Notes

- This spec refines the existing `Forge Query Runtime Rewrite Gate`; it does
  not add a parallel milestone.
- Once this spec closes, the next active frontier is Worth Milestone 3/Phase 7
  style edit widening on top of the real Query-backed kernel, not more kernel
  ambiguity cleanup.
- The sequencing change from the older draft is important:
  - before the upstream Query closeouts, this spec had to leave room for Query
    to grow generic graph substrate first
  - after the upstream Query closeouts, this spec must spend that substrate on
    the real Worth hard break rather than pretending the missing generic
    capability still explains the mirror-runtime debt
- The next meaningful frontier after this spec is not "teach Query more graph
  basics." It is "prove Worth uses the shipped Query graph and evidence
  contract for real workflow closure."
