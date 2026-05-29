# Worth Forge Query Runtime Rewrite Gate: Detailed Phases

This appendix refreshes the rewrite gate against the current `forge-query`
foundation.

The old version of this appendix still treated `worth-schema` as though it only
needed a light vocabulary freeze before the rest of the runtime rewrite could
continue. That is no longer honest.

`forge-query` now owns much more of the public kernel than the earlier gate
assumed. It is not only the declaration-lowering layer. It owns the public
grammar for admitted operating worlds, binding, orchestration, grouped meaning,
contributions, support/readiness, invariant registration, lower-runtime support
and explanation, signal/continuation follow-ons, retained inspection, and typed
recovery. Because of that, the first hard break in the Worth rewrite is not
"teach `worth-schema` a few more query constants." The first hard break is to
strip `worth-schema` back down to real schema authority and delete the
schema-local pseudo-runtime surfaces that Query now supersedes.

This appendix therefore starts with a schema-first purge. Later `worth-topo`
and broader runtime phases are sequenced after that purge rather than in
parallel with it.

This appendix is intentionally aggressive about that purge.

If deleting a public `worth-schema` type reveals that other crates were using it
as shared runtime-facing vocabulary, that discovery does not justify keeping the
type in `worth-schema`. It means the shared vocabulary boundary was placed in the
wrong crate and must be refactored to the Query-owned side instead.

## Goal

Make `worth-schema` a true schema crate again before broader Worth runtime
rewrite work continues.

That means:

- `worth-schema` keeps domain vocabulary, schema registration, and lower
  authority substrate facts.
- `worth-schema` stops exporting public runtime policy, public support/readiness
  posture, public invariant rollout posture, public trace/explanation products,
  public grouped/contribution/workflow-shaped runtime vocabulary, and public
  pseudo-Query artifact systems that Query now owns.
- later `worth-topo` and runtime rewrite phases build on that narrower and more
  honest foundation.

## Why This Rewrite Starts Here

Today `worth-schema` mixes three very different responsibilities:

1. real schema ownership
2. transition-era Query adapter policy
3. transition-era public runtime and explanation surfaces

The real schema ownership is still correct:

- platform entity and relation catalogs
- platform aspect vocabulary
- schema basis and collection vocabulary
- schema registry/bootstrap
- lower authority lowering contracts

The transition-era surfaces are now architectural debt:

- schema-owned query mutation gating
- schema-owned support contracts about what Query can or cannot do
- schema-owned runtime invariant rollout plans
- schema-owned public boundary envelopes and failures
- schema-owned public trace narration and runtime explanation helpers
- schema-owned shared runtime-facing vocabulary that only exists because
  downstream crates need names for Query-owned support, invariant, workflow,
  boundary, or recovery concepts

As long as these all remain public through the same facade, Worth still has two
foundations:

- Query as the public runtime kernel
- `worth-schema` as a shadow runtime policy, vocabulary, and artifact layer

That dual-foundation shape is exactly what this rewrite gate is supposed to
eliminate.

## Governing Document Summaries

### `MENTALITY.md`

Solve the reusable foundation problem first. Do not preserve a smaller local
adapter layer just because deleting it is disruptive.

### `arch_laws.md`

Authority, derivation, and boundary artifacts must stay structurally separate.
If Query owns the public runtime artifact family, `worth-schema` must not export
competing public artifacts that claim the same semantic job.

### `composition_laws.md`

Vocabulary, policy, orchestration, support, explanation, and certification are
different responsibilities and must not remain collapsed into the same crate
surface.

### `domain_structure_laws.md`

The tree and facade must make it obvious which types are schema authority and
which types are runtime products. If a reader cannot tell the difference from
the export surface, the structure is lying.

### `perf_laws.md`

Runtime support, inspection, and explanation work must not be hidden inside
cheap-looking schema helpers. The public surface should expose real runtime
boundaries through Query rather than schema-local wrappers.

### `VISION.md`

Worth truth is canonical once; everything else is derived honestly. That means a
schema crate should not also become a second runtime-facing derivation system.

### `worth_roadmap.md`

Worth runtime/query work must enter through Forge Query, and missing generic
runtime capability belongs in Query rather than in a local Worth workaround.

### `test-requirements.md`

The proof bar is structural. The rewrite must leave machine-checkable evidence
that public runtime surfaces were actually deleted or demoted rather than
quietly preserved behind renamed APIs.

### `forge-query` `9.3.7`

Query now owns public domain capability contribution, support, invariant,
workflow, explanation, and lower-runtime artifact materialization. Downstream
domains should not keep exporting local pseudo-Query versions of those surfaces.

### `forge-query` `9.3.8`

Query is now the beginning platform entry. Domains should enter through Query,
keep one admitted operating world, progress through one declaration pipeline,
and use Query-owned inspection and recovery rather than rebuilding a second
entry/runtime model locally.

## Adversarial Constraint

Equivalent Worth domain meaning must not require or permit two public runtime
stories:

- one through `forge-query`
- one through `worth-schema`

The schema-first rewrite fails if a downstream engineer can still reach for
`worth-schema` to get a public answer about runtime support, runtime invariant
posture, boundary artifact meaning, trace explanation, or next-step repair that
should instead come from Query.

The rewrite also fails if deleting those schema surfaces reveals that schema
authority and runtime-facing Query vocabulary were collapsed together. In that
case the work is to split them correctly, not to restore the collapsed surface.

## Product Decision Lock

- `worth-schema` owns Worth platform vocabulary, schema registration, and lower
  authority substrate contracts only.
- `forge-query` owns the public runtime/query artifact model.
- `forge-query` also owns any shared public runtime-facing vocabulary that is
  required precisely because Query owns that runtime/query model.
- `forge-query` owns the ordinary public surfaces for:
  - operating-world entry
  - next-input binding
  - declaration progression
  - grouped meaning, grouped products, and grouped contributions
  - declaration-scoped contributions
  - support posture
  - invariant registration
  - capability gaps and invariant denials
  - workflow posture
  - lower-runtime support and lower-runtime explanation
  - explanation posture
  - inspection
  - recovery
- `worth-schema` keeps only narrowly-scoped internal substrate types that are
  mechanically necessary for lower authority to talk to Query. Those types are
  not allowed to remain broad public facade products.
- if a public `worth-schema` enum, struct, or helper currently appears to be
  "shared vocabulary" only because multiple crates need the same runtime-facing
  Query concept, that surface must move to Query rather than being preserved in
  schema for compatibility.
- transition-era compile coverage that protects obsolete schema public surfaces
  is not a preservation requirement. It is migration pressure and should be
  deleted.

## Schema Surface Classification Lock

Every public `worth-schema` surface touched by this gate must land in exactly
one of these buckets.

### Keep In Schema

These remain public schema-owned vocabulary:

- platform aspect vocab
- platform entity and relation catalogs
- schema-basis and collection vocab
- query aspect-path constants and string conversion helpers, but only if they
  are truly schema/truth vocabulary rather than public names for Query-owned
  runtime posture
- schema registry/bootstrap
- lower authority topology truth and mutation vocabulary that is not pretending
  to be a Query-owned runtime product

Important restriction:

- a type belongs in this bucket only if its meaning is truly schema-owned
- "multiple crates currently import it" is not enough
- "it names a runtime-facing support / invariant / workflow / explanation /
  recovery concept" is positive evidence that it belongs in Query instead

### Demote To Internal Substrate

These are allowed only as temporary internal migration residue and must not
survive as broad public facade products:

- raw authority evidence carriers
- raw trace anchors or evidence helpers used immediately beside authority
  lowering
- authority-local boundary packets that only exist to feed Query integration
  or lower tests

### Delete Or Migrate Out

These are not allowed to remain public `worth-schema` architecture:

- query mutation gating and support contracts
- runtime invariant rollout plans
- invariant identity vocabulary when its real job is to serve as shared
  runtime-facing Query registration, denial, or support terminology
- declaration-builder vocabulary when its real job is to serve as public
  runtime-facing Query declaration grammar rather than schema-owned naming
- public schema-owned support posture artifacts
- public schema-owned boundary envelope and failure artifacts
- public schema-owned runtime trace narration
- public schema-owned explanation helpers whose real job is now covered by
  Query support, inspection, explanation, or recovery lanes
- public schema-owned shared names for grouped, contribution, workflow,
  boundary, inspection, support, or recovery posture that Query already owns

## Phases

### Phase 1: Freeze The Real `worth-schema` Nucleus

Phase 1 is now specified concretely in
[worth-schema-phase-1-audit.md](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-schema-phase-1-audit.md).

Before deleting anything, freeze what truly belongs in `worth-schema` so later
removal work does not accidentally cut through real authority.

This phase must produce one explicit keep-list covering:

- `data/aspects/*`
- `data/entities/*`
- `data/relations/*`
- schema registry/bootstrap surfaces that define truth structure rather than
  runtime posture
- `data/query/mod.rs` vocabulary types such as:
  - `QueryAspectPath`
  - `QueryCollection`
  - `QuerySchemaBasis`
  - `QueryLiveField`
- declaration-builder vocabulary in `data/query/declarations.rs`, followed by a
  hard verdict on whether it is genuinely schema-owned naming or should move
  into Query with the rest of the public runtime-facing declaration grammar

This phase must also produce one explicit challenge list for suspicious
vocabulary that looks shared today but may actually belong in Query instead,
including:

- invariant enums
- support and traceability identifiers
- boundary and explanation identity types
- declaration-builder wrappers and surface-name grammars
- any runtime-facing family or posture names currently exported through schema

Phase 1 is complete only when the spec states plainly which public exports are
the non-negotiable schema nucleus and why.

### Phase 2: Audit And Classify Every Public `worth-schema` Export

Run a full export-by-export audit of the current public facade and assign every
surface to one locked bucket:

- keep in schema
- demote to internal substrate
- delete or migrate out

This phase must classify the current public surface at least to the level
captured in the standalone Phase 1 audit doc.

Phase 2 is complete only when there is no remaining "maybe this still belongs
here" ambiguity for the public facade.

### Phase 3: Delete Public Query-Policy Surfaces From `worth-schema`

Remove the transition-era Query policy layer from the public schema boundary.

This phase must cut or privatize public surfaces such as:

- `QueryMutationAdmissionBlocker`
- `QueryMutationAdmissionReport`
- `QueryMutationAdmission`
- `QueryMutationSupportContract`
- `query_mutation_support_contract(...)`
- `admit_query_mutation_batch(...)`

The important rule is architectural, not textual:

- `worth-schema` is not allowed to remain the public place that answers whether
  Query can support, deny, defer, widen, or classify a runtime-facing family

That posture now belongs to Query support/readiness, contribution, grouped,
inspection, and recovery surfaces.

Phase 3 is complete only when ordinary downstream callers can no longer rely on
schema-local query mutation gating as a public runtime product.

### Phase 4: Delete Public Runtime Invariant Rollout Surfaces

Remove the public API that treats `worth-schema` as the owner of runtime
invariant rollout posture.

This phase must cut or privatize:

- `BootstrapInvariantPlan`
- `bootstrap_invariant_plan()`
- `BootstrapRuntimeInvariant`
- `BootstrapRuntimeInvariantPlan`
- `bootstrap_runtime_invariant_plan()`

Invariant identity enums are not protected just because they currently serve as
shared names. Public runtime installation and runtime-facing denial posture must
flow through Query's invariant registration and capability-denial model, and any
vocabulary whose real job is to support that runtime-facing layer moves with it.

Phase 4 is complete only when `worth-schema` no longer teaches runtime
invariant rollout as one of its public jobs.

The downstream `worth-topo` closeout for this slice lives in:

- [worth-topo Phase 4 Runtime Invariant Closeout](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-phase-4-runtime-invariant-closeout.md)

This phase must force one of two outcomes:

- invariant names that are truly schema authority remain in schema
- invariant names that are really Query-facing registration, denial, support,
  or recovery vocabulary move into Query

Leaving them in schema merely because other crates already import them is a
spec violation.

### Phase 5: Remove Public Boundary, Trace, And Explanation Products

Cut the broad public schema tracing/explanation facade.

This phase must remove from the ordinary public story:

- `BoundaryEnvelope`
- `BoundaryFailure`
- `DecisionTrace`
- public trace-anchor and trace-evidence products as broad consumer-facing
  runtime artifacts
- `NarratedTrace`
- `explain_*`
- `narrate_*`

This phase requires the public facade to stop teaching those types as the
ordinary runtime artifact family. Any lower substrate evidence that still exists
afterward is migration residue on the path to a narrower internal boundary, not
a second public story.

After this phase:

- Query receipts, envelopes, inspection artifacts, support artifacts,
  explanation artifacts, grouped products, contribution-composed products, and
  recovery briefs are the public runtime story
- any remaining schema-local trace/evidence types are narrow internal substrate
  tools only and should be treated as candidates for further collapse or rename
  if they still masquerade as public-grade runtime artifacts

Phase 5 is complete only when the public `worth-schema` facade no longer reads
like a shadow runtime SDK.

### Phase 6: Rebind Authority-Lowering Code To The Narrower Boundary

Once the public surface is reduced, rework the internal authority-adjacent code
to fit the new boundary honestly and aggressively.

This phase must:

- audit `data/authority/gateway.rs` against the narrowed ownership model
- cut every raw authority evidence type that is not mechanically necessary after
  the facade purge
- stop naming internal substrate packets as though they are the ordinary public
  boundary products
- keep authority lowering separate from Query-owned inspection, support, and
  explanation products
- keep authority-lowering residue separate from Query-owned binding,
  orchestration, grouped, contribution, and recovery products too

This phase is not about preserving the old trace model privately by default. It
is about deleting that model until only mechanically necessary lower-authority
substrate remains.

### Phase 7: Rewrite Public API Tests, Compile Boundaries, And Docs

After the deletion/demotion work, rewrite the public proof surface so tests
protect the new boundary instead of the old one.

This phase must update:

- [crates/worth-schema/tests/public_api_contract.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/tests/public_api_contract.rs)
- any compile-fail tests that still guard the old broad runtime products
- crate docs and README-level stories that still present schema as a public
  runtime/support/explanation layer

The new public contract should prove:

- schema vocabulary remains available
- schema runtime-policy surfaces are gone
- schema runtime-artifact surfaces are gone
- schema runtime-facing shared vocabulary that truly belongs to Query is gone
- callers are mechanically pushed toward Query for runtime-facing work

### Phase 8: Re-sequence Later Worth Runtime Rewrite Work On Top Of The Narrower Schema

Only after the schema purge closes should broader Worth runtime rewrite work
continue in `worth-topo`, `worth-spatial`, and the remaining gate phases.

That follow-on work should now assume:

- `worth-schema` is vocabulary-first
- Query is the only public runtime spine
- no new topo/spatial refactor may reintroduce schema-local public runtime
  policy or artifact systems just because they are convenient

## Must Ship

- one explicit keep/demote/delete classification for the public schema surface
- a reduced public `worth-schema` facade aligned with that classification
- removal or privatization of query-policy, invariant-rollout, and public
  trace/explanation runtime surfaces
- removal or migration of public schema vocabulary that was only "shared"
  because other crates needed names for Query-owned runtime concepts
- migration of any falsely "shared" runtime-facing vocabulary from
  `worth-schema` into Query when the audit proves that Query is the real owner
- updated public API tests proving the narrower boundary
- updated rewrite-gate language that treats the schema purge as the first hard
  break instead of as a small preliminary cleanup

## Must Preserve

- schema-owned truth vocabulary
- schema-owned registry/bootstrap meaning
- lower authority topology truth vocabulary and mutation meaning
- explicit authority/derivation separation
- the rule that missing generic runtime capability belongs in Query rather than
  in a new local workaround

This section is intentionally not preserving "existing shared vocabulary
placement." Shared placement is preserved only when the ownership boundary is
correct.

## Acceptance Evidence

- public API tests proving removed runtime-policy and runtime-artifact exports
  are no longer part of the broad facade, and that Worth semantic catalogs now
  enter through the sanctified `platform::*` boundary instead of the root
  facade
- compile-fail or visibility tests proving deleted surfaces are no longer
  externally reachable
- crate docs and rewrite-gate docs that now describe `worth-schema` as
  vocabulary/schema authority rather than runtime facade
- proof that any public shared runtime-facing vocabulary discovered during the
  audit was either:
  - deleted because it was obsolete, or
  - moved into Query because Query is the real owner
- proof that declaration-builder wrappers either remained as true
  schema-owned naming helpers or moved out with the rest of the Query-facing
  declaration grammar
- grep-level proof that ordinary downstream examples no longer teach:
  - `admit_query_mutation_batch(...)`
  - `query_mutation_support_contract(...)`
  - bootstrap runtime invariant plans
  - schema-owned narrated runtime traces

## Sequencing Notes

- This schema-first purge belongs before further runtime rewrite widening
  because every later crate depends on whether `worth-schema` is a narrow truth
  vocabulary crate or a broad pseudo-runtime crate.
- `worth-topo` and `worth-spatial` should not be allowed to migrate onto a
  cleaner Query surface while still depending on a dirty schema public boundary.
- If this phase feels large, that is evidence that it is foundational, not that
  it should be postponed.
