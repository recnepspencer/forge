# Worth Touched Graph Authority Gate

> **Status:** Draft
>
> **Purpose:** make touched graph authority the mandatory Worth spine before
> `Milestone 7.5` resumes broad planar boolean work.

## Goal

This gate closes the gap left by the Query Graph Authority Hardening Gate. That
gate deleted local graph-authority ceremony and proved Worth can consume Query
graph touch obligation authority. It did not make every topology operator and
boolean stage derive validators, invariants, invalidation, replay, evidence
lookup, conflict posture, caching, diagnostics, and public proof from one typed
touched graph basis.

By the end of this gate, an operator may not say "I ran the right validators."
It must produce a proof-bearing touched graph basis, and the framework must
derive every downstream consequence from that basis.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first design. The hard
  condition is that boolean, NURBS, extrusion, and fillet chains will touch
  small graph regions inside large topology; global validation, repeated
  rediscovery, and hand-maintained validator lists will fail under that scale.
- `arch_laws.md`: protects structural contracts and proof-bearing phases.
  Laws 2, 4, 22, 27, 30, 41, and 42 require mutations to declare what they
  invalidate, pipeline phases to derive execution from structure, batch facts
  to be carried forward once, and weaker representations to never promote
  themselves into authority.
- `composition_laws.md`: protects named semantic decomposition. Touched graph
  request, admission, expansion, validator selection, invalidation, replay,
  evidence lookup, conflict analysis, and diagnostics must be separate
  responsibilities, not one broad operator helper.
- `domain_structure_laws.md`: protects visible authority boundaries. The tree
  must show where touched graph truth is born, where Query consumes it, where
  topology derives legality, where spatial evidence is indexed, and where
  residue is contained.
- `perf_laws.md`: protects semantic-delta-bounded execution. Work breadth must
  scale with touched graph breadth, not global topology size, broad evidence
  scans, or repeated selector rediscovery.
- `_docs/worth/milestone-7-roadmap.md`: protects this as the hard prerequisite
  between the Query Graph Authority Hardening Gate and `Milestone 7.5`.
- `_docs/worth/milestone-7.5.md`: protects overlap-region extraction as the
  next consumer. `7.5` must consume touched graph products rather than adding
  its own validator, evidence, or dirty-region folklore.

## Adversarial Constraint

Given a hostile boolean chain that repeatedly splits edges, reconstructs loops,
extracts overlap regions, and prepares later face assembly inside a large B-rep,
each operation must derive all graph-legality work from the exact graph basis it
touched. Validator breadth, invariant breadth, derived invalidation breadth,
replay breadth, evidence lookup breadth, conflict posture, cache invalidation,
undo scope, public proof, and diagnostics must scale with the touched graph
closure and must not fall back to static global packs, broad relation scans,
operator-local validator lists, or "remember to run this" convention.

If two operators with different touched bases select the same validator,
invalidation, replay, evidence, and conflict sets merely because they share a
milestone, this gate has failed.

If an undeclared graph mutation can commit, replay, or certify without being
rejected as outside the touched basis, this gate has failed.

## Product Decision Lock

- This gate is a hard break, not a slow migration. Delete or collapse old
  static/global ceremony as soon as the touched graph replacement exists.
- `forge-query` owns graph touch descriptors, selector matching, obligation
  selection, support posture, execution proof, and query-gap posture.
- `worth-topo` owns topology truth, topology touched basis construction, graph
  closure expansion, topology validator derivation, and topology legality
  diagnostics.
- `worth-spatial` owns spatial evidence, boolean stage receipts, evidence
  lookup products, and spatial graph-affecting evidence indexed by its own
  spatial touch authority and Query touch descriptors.
- `worth-kernel` owns workload composition, public closeout pressure, and
  cross-crate proof that no lower-authority touched graph substitute can pass.
- Static validator packs, broad global closeout validators, raw evidence scans,
  and compatibility adapters are not stepping stones. They are deletion targets
  unless explicitly retained as capped residue with owner, cap, removal trigger,
  and compile-fail or certification proof that they cannot act as authority.

## Touched Graph Product Ladder

The implementation must produce explicit proof ladders. Names may evolve, but
the proof boundaries may not be collapsed.

Topology branch:

1. raw topology operator intent
2. admitted topology operator intent
3. declared topology touched graph basis
4. expanded topology touched graph closure
5. selected Query graph obligations
6. selected topology validators and relational invariants
7. derived invalidation plan
8. replay scope
9. conflict and independence proof
10. cache and equivalence invalidation proof
11. undo and transaction scope
12. executed topology touched graph receipt
13. public topology authority proof
14. diagnostics and explanation surface

Spatial evidence branch:

1. sealed boolean or spatial evidence receipt
2. spatial touch authority product
3. Query touch descriptor or Query adoption proof
4. spatial evidence lookup key and lookup product
5. spatial replay scope
6. spatial cache or equivalence proof
7. public spatial authority proof
8. diagnostics and explanation surface

The branches may share schema vocabulary, digest recipes, counters, and Query
descriptor language. They may not share constructors, admission authority, or
proof ownership. No later phase may accept an earlier proof type when a
stronger artifact exists.

## Authority Product Matrix

Every implementation phase must name its authority transition in code and tests:

| Phase | Input Authority | Output Authority | Owner | Forbidden Substitute | Deletion Target |
| --- | --- | --- | --- | --- | --- |
| 2 | topology vocabulary and admitted topology meaning | sealed topology touched graph basis vocabulary, digest, and counters | `worth-topo` with shared `worth-schema` vocabulary | raw ids, strings, copied Query rows, mutation records, spatial receipts | schema projection/admission traits, type-name guards, topology geometry-only bridge |
| 3 | admitted topology operator intent | declared topology touched graph basis | `worth-topo` | operator validator arrays, mutation records as proof, local touch languages | bypass fronts and operator-local validator declarations |
| 4 | sealed boolean or spatial evidence receipt | spatial touch authority, Query descriptor/adoption proof, evidence lookup key | `worth-spatial` with shared `worth-schema` vocabulary and `forge-query` descriptors | raw evidence rows, broad stage scans, copied receipt fields, public schema constructors | public projection/admission experiments, topology geometry-only lowering |
| 5 | topology basis or spatial Query descriptor | selected Query graph obligations | `forge-query` | Worth-local selector copies, in-memory adoption as execution proof | broad selector adapters and local selector forks |
| 6 | expanded topology closure | selected topology validators and relational invariants | `worth-topo` | global validator packs, expectation arrays | static invariant packs and ordinary operator-local global validation |
| 7 | expanded topology closure | derived invalidation and dirty propagation plan | `worth-topo` | whole-view rebuild as default, hidden projection expansion | broad derived rebuild paths |
| 8 | topology basis or spatial touch authority | evidence lookup plan and lookup product | `worth-spatial` | raw evidence vectors, broad receipt scans | raw public evidence scans |
| 9 | topology/spatial proof products | replay, undo, and transaction scope | owning crate plus `worth-kernel` composition | global replay, re-query rollback authority | replay/undo paths without proof scope |
| 10 | touched closures and spatial authority products | conflict, independence, and batch-admission proof | `worth-topo`, `worth-spatial`, `forge-query` | speculative lock-first conflict discovery | batch shortcuts without structural proof |
| 11 | touched proof plus source authority digest | cache/equivalence proof | owning product crate | pointer identity, row count, operator family | cache keys without touched authority |
| 12 | executed proof products | public read-only authority proof and diagnostics | public facades | raw constructors, support pins, local ceremony | public escape hatches |
| 13 | certified phase products | cross-crate closeout matrix | `worth-kernel` closeout pressure | prose claims, untested residue | uncapped adapters and stale doc claims |

This matrix is normative. If code needs an authority product not named here, the
spec is incomplete and must be updated before implementation continues.

## Cross-Crate Boundary Invariants

- `worth-schema` defines shared vocabulary, digest/counter recipes, and envelope
  shapes. It does not admit arbitrary caller projections into authority.
- `worth-topo` admits topology intent and produces topology touched graph
  authority. It does not broker spatial geometry truth.
- `worth-spatial` admits sealed spatial/boolean evidence and produces spatial
  touch authority, Query descriptors, and evidence lookup products. It may feed
  topology birth intent, but its geometry evidence receipts do not become
  topology authority by passing through `worth-topo`.
- `forge-query` selects and proves graph obligations. Worth may translate into
  Query descriptors, but it may not fork selector semantics.
- `worth-kernel` composes and audits the cross-crate chain. It cannot promote a
  lower-authority local handoff into executed graph authority.

## Delete-First Rule

When a phase creates the named sealed replacement for an old authority path, the
same phase must delete the replaced production path or mechanically cap it as
certification-only residue with owner, cap, removal trigger, and a test proving
it cannot satisfy ordinary authority APIs. Compatibility adapters, feature-gated
bridges, public raw constructors, copied-row admission, and type-name guards are
not acceptable slow conversions.

The adapter reflex is explicitly wrong for this gate. An adapter is not a safe
middle step merely because it compiles, preserves callers, or reduces immediate
diff size. In authority code, an adapter usually means the old authority is
still alive beside the new proof path. That is duplicated authority, not
migration. The default action is deletion and caller cleanup. Keeping an adapter
requires proof that it is unreachable from production authority, mechanically
capped, named as residue or a Query/runtime gap, and scheduled for removal. If
that proof is missing, the adapter is a phase failure even when tests are green.

## Phase 1: Inventory And Hard Break Plan

Freeze the list of every static/global validator, invariant, invalidation,
replay, evidence, conflict, cache, undo, and diagnostic path that currently
acts without a touched graph basis.

Relevant subsystems:
- `crates/worth-topo/src/topology_operators`
- `crates/worth-topo/src/validation`
- `crates/worth-topo/src/projection`
- `crates/worth-spatial/src/workload_platform`
- `crates/worth-kernel/src/query_graph_authority_gate`
- `crates/forge-query/src/runtime/mutation/graph_composition`

Engineering decisions:
- The inventory is code, not prose. It must be typed, queryable, and exposed to
  closeout certification.
- Each row names owner, current authority source, touched graph replacement,
  deletion action, and removal trigger.
- Rows may not use `keep` for old static/global surfaces. Allowed dispositions
  are `delete`, `collapse`, `certification-only`, `residue`, or `query-gap`.

Warnings:
- Do not preserve a broad validator pack because later phases still call it.
  That is the reason to break callers.
- Do not count doc references as inventory coverage.

Test requirements:
- Adversarial discovery test: adding a static validator/invariant entry without
  a touched graph replacement row fails inventory certification.
- Adversarial deletion test: a row marked `delete` or `collapse` still exported
  through an ordinary facade fails closeout.
- Composition test: every touched Rust file remains under the workspace line
  cap or is split before the phase closes.

Open questions:
- None.

## Phase 2: Topology Touched Graph Basis Types

Freeze the topology type system that every topology operator must use to state
what graph meaning it touched. This phase is topology-only; spatial geometry
evidence authority is a separate product in Phase 4.

Relevant subsystems:
- `worth-topo` topology operator mutation records
- `worth-topo` topology relation and aspect vocabulary
- `forge-query` graph touch descriptors
- `worth-schema` shared touched graph vocabulary, counters, and digest recipes

Engineering decisions:
- A topology touched graph basis is a sealed proof object, not strings, raw ids,
  copied Query rows, mutation records, spatial receipts, or caller-filled
  descriptor structs.
- The basis distinguishes touched topology entities, relations, relation kinds,
  aspects, topology scopes, graph lifecycle posture, and operating world.
- The basis carries stable digest and counters for topology breadth.
- `worth-schema` may define shared vocabulary and digest/counter contracts, but
  must not expose public caller-implementable admission traits, public
  admission constructors, or runtime type-name allowlists that let weaker
  values promote themselves into authority.
- Production `worth-topo` does not consume spatial geometry-only evidence
  receipts. Any current geometry-only topology lifecycle, lowering, or bridge is
  deleted, collapsed into certification-only proof, or moved to the later
  spatial authority phase.

Warnings:
- Do not let raw `Vec<String>`, raw `Aspect`, raw relation ids, or copied Query
  descriptors stand in for the proof-bearing basis.
- Do not collapse topology touch and geometry touch. Fillets and NURBS need both,
  but topology basis construction is not geometry evidence admission.
- Do not repair this phase by adding private names around a public projection
  trait or a `std::any::type_name` guard. That is a lower-authority shortcut,
  not proof.

Test requirements:
- Compile-fail test: external callers cannot construct a touched graph basis
  from raw strings, ids, or copied descriptor rows.
- Parity test: the same operator intent produces the same touched graph digest
  across replay and benign input ordering.
- Drift test: adding a new touched aspect without updating the digest/counters
  fails certification.
- Boundary test: no production `worth-topo` module lowers or admits spatial
  geometry-only evidence as topology touched graph authority.
- Schema authority test: external callers cannot implement schema projection
  traits, call public geometry admission constructors, or bypass sealing through
  copied receipt fields.

Open questions:
- None.

## Phase 3: Topology Operator Intent To Touched Basis

Freeze the rule that every topology operator produces a topology touched graph
basis before lowering to Query or executing local topology work.

Relevant subsystems:
- topology operator declaration entry
- mutation sequence and mutation records
- planar boolean split and loop topology birth intent
- future overlap-region extraction request

Engineering decisions:
- Operator semantic intent compiles into topology touched graph basis
  construction.
- Mutation sequence records and boolean topology birth intent may feed the
  topology basis, but may not be the basis.
- Existing helper fronts that bypass basis construction must be deleted, not
  wrapped.
- Spatial geometry evidence receipts do not pass through this topology phase;
  they are handled by Phase 4.

Warnings:
- Do not let an operator declare validators directly.
- Do not let 7.5 overlap extraction invent a local touched graph language.

Test requirements:
- Adversarial differentiation test: two operators with different touched bases
  select different validator and invalidation sets.
- Adversarial omission test: an operator that mutates a relation/aspect outside
  its touched basis is denied before commit.
- Public facade test: operators expose touched-basis proof/status without
  exposing raw constructors.

Open questions:
- Which existing scalar operator fronts should be deleted outright once basis
  construction exists?

## Phase 4: Spatial Geometry Evidence Touch Authority

Freeze spatial and boolean geometry evidence as its own sealed touch authority,
using shared schema vocabulary and Query touch descriptors without laundering
that authority through topology basis construction.

Relevant subsystems:
- `worth-spatial` boolean evidence receipts
- `worth-spatial` evidence ledger and stage indexes
- `worth-schema` shared touch vocabulary, counters, and digest recipes
- `forge-query` graph touch descriptors and adoption proof
- kernel boolean workload handoff surfaces that consume spatial evidence

Engineering decisions:
- A graph-affecting spatial receipt produces two sibling products when both are
  needed: a sealed spatial touch authority product for Worth evidence lookup and
  replay, and a Query touch descriptor or adoption proof for Query obligation
  selection. Neither product is allowed to impersonate the other.
- `worth-schema` supplies shared names and digest/counter structure only; it
  does not own public admission from arbitrary caller projections.
- Spatial evidence identity includes stage, receipt digest, operating world,
  touched graph digest or descriptor digest, and lookup counters.
- Spatial authority may carry a reference to topology birth intent when a
  boolean stage creates topology work, but that reference is input to
  `worth-topo` topology admission, not proof that geometry evidence is topology
  authority.
- Production spatial evidence does not route through `worth-topo` geometry-only
  lifecycle or topology lowering.
- Current public projection/admission experiments, runtime type-name allowlists,
  and topology geometry-only bridges are deletion targets for this phase unless
  they were already deleted in Phase 2.

Warnings:
- Do not make `worth-topo` the broker for spatial geometry truth.
- Do not let raw evidence rows, broad stage scans, copied receipt fields, or
  public schema constructors satisfy spatial touch authority.
- Do not hide a slow conversion path as an adapter. Delete it when the sealed
  spatial product exists.

Test requirements:
- Compile-fail test: external callers cannot fake a spatial geometry receipt,
  implement a projection trait, hand-fill a lookup product, or mint Query touch
  authority from copied fields.
- Positive path test: a sealed boolean receipt produces a stable spatial touch
  digest and Query touch descriptor/adoption proof.
- Product separation test: spatial evidence lookup accepts the spatial touch
  authority product, while Query obligation selection accepts only the Query
  descriptor/adoption proof; swapping those products fails.
- Boundary test: no production `worth-topo` module consumes spatial
  geometry-only evidence as touched graph authority.
- Replay test: replay of the same boolean chain produces the same spatial touch
  authority identity and counters.
- Deletion test: the old projection/admission/type-name bridge and topology
  geometry-only lowering are absent or certification-only with a capped residue
  row and removal trigger.

Open questions:
- Which spatial exact-predicate products deserve independent geometry touch
  descriptors before NURBS and fillets?

## Phase 5: Query Obligation Selection From Touched Basis

Freeze Query graph obligation selection as a consumer of topology touched graph
basis translations and spatial Query descriptors, not topology-local or
spatial-local selector copies.

Relevant subsystems:
- `ForgeQueryGraphTouchDescriptor`
- `ForgeQueryGraphTouchSelector`
- Query graph obligation index and selection lookup
- topology operator adoption catalog

Engineering decisions:
- Worth may translate topology touched basis into Query descriptors, and
  `worth-spatial` may produce Query descriptors from sealed spatial evidence,
  but Query owns selector semantics for both.
- Broad collection-only or lifecycle-only selector use is allowed only as
  explicit capped residue.
- Selection counters report attempted buckets, matched descriptors, deduplicated
  candidates, rejected candidates, and selected obligations.

Warnings:
- Do not fork Query selector matching in Worth.
- Do not let spatial evidence lookup products stand in for Query descriptors.
- Do not certify an operator by proving only in-memory adoption when execution
  proof is required.

Test requirements:
- Adversarial selector test: same collection with wrong mutation family or
  touched aspect does not select the obligation.
- Query-gap test: any missing selector capability becomes a Query-owned gap row
  instead of a Worth-local substitute.
- Performance test: selection breadth scales with touched descriptor breadth,
  not global registration count.

Open questions:
- Does Query need a richer descriptor for topology scope closure beyond
  collection/aspect?

## Phase 6: Validator And Invariant Derivation

Freeze topology validator and invariant selection as a derived consequence of
the touched graph closure.

Relevant subsystems:
- `worth-topo/src/validation`
- `worth-topo/src/validation/reference_integrity`
- Query graph-scoped custom invariant registrations
- topology operator closeout certification

Engineering decisions:
- `TopologyValidator::derived_validation_report` may remain only for whole-view
  certification or explicit residue. Operator-local legality consumes touched
  graph closure.
- Static milestone-one invariant packs are deleted or collapsed into a
  touched-basis registry.
- Each validator family declares the touched graph predicates that require it.

Warnings:
- Do not keep "run all validators" as the ordinary operator path.
- Do not let validator expectation arrays become the new manual selector list.

Test requirements:
- Adversarial selection test: touching loop successor selects loop wiring and
  not unrelated shell-only validators.
- Adversarial expansion test: touching radial adjacency expands to the radial
  fan closure before validator selection.
- Rejection test: an operator that requires a validator family with no touched
  basis rule fails planning or closeout.
- Line-cap/composition test: any validator registry file that grows toward a
  bucket must be split by predicate family.

Open questions:
- None.

## Phase 7: Derived Invalidation And Dirty Propagation

Freeze derived topology invalidation as a plan derived from touched graph
closure.

Relevant subsystems:
- materialized topology view
- interpreted traversal views
- radial ring, vertex disk, loop, wire, face, shell, and body projections
- boolean stage products that birth new topology intent

Engineering decisions:
- Touched graph basis produces a dirty-region propagation plan before derived
  rebuild work starts.
- Propagation distinguishes direct touches from closure-expanded touches.
- Derived products state which touched basis invalidates them.

Warnings:
- Do not rebuild all derived topology for ordinary local operators.
- Do not hide dirty expansion inside projection rebuild code.

Test requirements:
- Parity test: two replay-equivalent touched bases produce the same dirty-region
  plan and derived invalidation digest.
- Rejection test: a derived product without an invalidation contract cannot be
  consumed after a graph-affecting operation.
- Performance test: counters prove invalidation breadth equals closure breadth,
  not total topology breadth.

Open questions:
- Which closure expansions are conservative until better topology indexes exist,
  and what removal triggers close them?

## Phase 8: Evidence Lookup And Boolean Stage Indexing

Freeze spatial and boolean evidence lookup around spatial touch authority and
related topology touched graph identity rather than broad stage scans.

Relevant subsystems:
- spatial evidence ledger
- edge split chain ledger
- loop reconstruction ledger
- future overlap-region ledger
- kernel workload evidence closeout

Engineering decisions:
- Every graph-affecting boolean receipt carries or references the spatial touch
  authority product it proved, and may separately reference topology touched
  graph identity when the stage birthed topology work.
- Evidence lookup products are keyed by spatial touch authority identity, stage,
  receipt digest, and, where relevant, topology touched graph identity.
- Raw evidence rows and broad stage vectors are deletion targets.
- Evidence lookup does not rediscover geometry or topology meaning from raw
  rows; it consumes the sealed products created in Phase 4.

Warnings:
- Do not let 7.5 search all loop receipts to rediscover overlap participants.
- Do not hand-fill lookup products from raw rows.

Test requirements:
- Adversarial lookup test: wrong spatial touch digest or mismatched topology
  touched graph digest cannot retrieve or satisfy a stage receipt.
- No-raw-scan test: public contracts and compile-fail tests reject raw evidence
  vectors as lookup products.
- Replay test: evidence lookup identity is stable across replay of the same
  boolean chain.
- Product swap test: Query descriptors cannot be passed as evidence lookup
  products, and evidence lookup products cannot satisfy Query obligation
  selection.

Open questions:
- Which exact-predicate products require their own spatial touch authority
  subtype before NURBS and fillets?

## Phase 9: Replay, Undo, And Transaction Scope

Freeze replay and undo boundaries as consumers of touched graph proof.

Relevant subsystems:
- topology mutation replay
- boolean ledger replay
- Query receipts and envelopes
- transaction rollback/undo surfaces

Engineering decisions:
- Replay scope is derived from touched graph closure and carried as a proof
  object.
- Undo scope is the minimal reversible graph patch scope, not the entire
  operation family.
- Transaction boundaries expose touched graph digest, selected validators,
  invalidated derived products, and evidence receipts.

Warnings:
- Do not replay global topology to prove local edits.
- Do not make rollback re-query authority that the touched graph receipt already
  captured.

Test requirements:
- Replay parity test: same touched graph basis produces identical replay scope
  and receipt digests.
- Hidden mutation test: mutation outside undo scope fails transaction closeout.
- Undo minimality test: rollback does not affect unrelated graph components.

Open questions:
- Which undo cases remain certification-only until transactional topology patch
  storage is stronger?

## Phase 10: Conflict, Independence, And Batch Admission

Freeze independence proof and conflict denial as touched graph products.

Relevant subsystems:
- topology operator batch planning
- boolean chain planning
- Query graph obligation operating worlds
- future parallel admission surfaces

Engineering decisions:
- Two operations may be admitted together only when their touched graph closures
  prove disjointness or compatible aspect-level overlap.
- Conflict posture distinguishes entity conflict, relation conflict, aspect
  conflict, closure conflict, evidence conflict, and validator conflict.
- Batch admission cannot discover conflicts by speculative execution.

Warnings:
- Do not implement lock-first conflict detection as a substitute for structural
  touched graph proof.
- Do not collapse "same entity" and "same aspect" into one conflict class.

Test requirements:
- Independence test: two disjoint touched graph closures batch-admit and keep
  separate replay/evidence receipts.
- Conflict test: same relation with incompatible aspect touch denies before
  execution.
- Validator conflict test: disjoint direct touches that expand into the same
  closure conflict are denied or serialized with a named reason.

Open questions:
- Which compatible same-relation aspect combinations are worth admitting now?

## Phase 11: Cache, Equivalence, And Reuse Contracts

Freeze reuse as a touched graph equivalence claim.

Relevant subsystems:
- derived topology caches
- spatial evidence products
- boolean stage outputs
- future NURBS, extrusion, chamfer, and fillet preparation products

Engineering decisions:
- Cache keys include touched graph digest, source authority digest, stage, and
  equivalence policy.
- Reuse is denied when touched graph closure, operating world, validator set, or
  evidence receipt set differs.
- Geometry-only equivalence and topology-touch equivalence are distinct.

Warnings:
- Do not reuse a derived product because the same operator family ran.
- Do not let pointer identity, row count, or filename provenance become reuse
  authority.

Test requirements:
- Equivalence test: benign ordering noise produces the same cache/equivalence
  key.
- Drift test: same operator family with different touched graph closure cannot
  reuse the prior product.
- Compile-fail test: public callers cannot mint cache equivalence proof.

Open questions:
- Which caches should remain rebuild-only until proof maintenance is cheaper
  than recomputation?

## Phase 12: Public API, Diagnostics, And Explainers

Freeze the ordinary public surfaces that expose touched graph proof without
leaking constructors or internals.

Relevant subsystems:
- `worth-topo::facade`
- `worth-spatial::facade`
- `worth-kernel::facade`
- public contract and compile-fail harnesses

Engineering decisions:
- Public APIs expose read-only touched graph digest, touched scope counters,
  selected obligation ids, selected validator ids, invalidated derived products,
  evidence lookup ids, conflict posture, and denial explanation.
- Diagnostics explain why a validator ran or did not run from touched graph
  facts.
- Public callers cannot construct touched graph authority, selected validator
  rows, evidence lookup proof, cache proof, or independence proof.

Warnings:
- API presence is not proof. Exact public facade roots and accessors must be
  certified.
- Do not expose raw internals for tests. Use compile-fail fixtures to prove the
  boundary.

Test requirements:
- Public proof test: ordinary facade returns touched graph proof/status without
  raw constructors.
- Diagnostic honesty test: a rejection localizes to exact touched graph fact,
  selected validator, or missing Query capability.
- Compile-fail test: raw strings, copied rows, support pins, local ceremony, or
  raw receipts cannot satisfy public touched graph authority APIs.

Open questions:
- None.

## Phase 13: Cross-Crate Closeout And 7.5 Readiness

Close the gate only when every category consumes touched graph authority or is
explicitly deleted, capped residue, or Query-gap.

Relevant subsystems:
- cross-crate closeout certification
- `Milestone 7.5` readiness gate
- line-cap and composition checks
- closeout documentation

Engineering decisions:
- Closeout reports counts by category: validators/invariants, invalidation,
  replay, Query obligations, evidence lookup, dirty propagation, locality proof,
  conflict, cache/equivalence, undo, public API, diagnostics, boolean-chain
  planning, geometry-touch separation, and residue pressure.
- `Milestone 7.5` may not start broad overlap extraction until this closeout is
  green.
- Any slow-conversion adapter still used by production is a finding unless it is
  certification-only or capped residue with removal trigger.

Warnings:
- Do not close with "good enough for now" compatibility paths.
- Do not let generated closeout documents claim cleaner deletion than code
  certifies.

Test requirements:
- Cross-category matrix test: every touched graph use case category has an
  implemented consumer, deletion row, capped residue row, or Query-gap row.
- Hard-break test: old static/global paths fail if reintroduced through public
  facades or production module exports.
- Line-cap test: every touched Rust code/test file is under the workspace cap or
  explicitly allowlisted.
- Doc consistency test: closeout doc category counts match the certified
  closeout matrix.

Open questions:
- None.

## Non-Negotiable Hard Breaks

- Delete or collapse `TopologyValidator` global operator-local use. Whole-view
  certification may remain only as a named whole-view path.
- Delete or collapse milestone-one invariant packs as ordinary operator
  legality authority.
- Delete broad evidence scans after touched graph lookup products exist.
- Delete operator-local validator declarations after touched-basis derivation
  exists.
- Delete compatibility adapters in the same phase that replaces their last
  production caller.
- Treat adapters, bridge modules, compatibility shims, transitional facades, and
  "just for now" conversion helpers as hostile until proven otherwise. Their
  burden is proof of non-authority; the implementation does not get the benefit
  of the doubt.
- Delete public raw constructors and copied-row escape hatches immediately.

## Automation Runner Requirements

Any automated runner for this gate must require plan entries for these
categories before implementation starts:

- phase authority transition: input authority, output authority, owner,
  forbidden substitutes, deletion targets, and downstream consumers
- touched basis type boundary
- operator intent lowering
- spatial geometry evidence touch authority
- Query obligation selection
- validator/invariant derivation
- derived invalidation and dirty propagation
- evidence lookup indexing
- replay, undo, and transaction scope
- conflict and independence proof
- cache/equivalence contracts
- public API and diagnostics
- deletion/hard-break cleanup
- tests, compile-fail, line-cap, and composition QA

Every phase plan must include a concrete "delete now" section that names the
production paths being deleted or capped in that phase. If the plan keeps a
replacement adapter because a downstream phase is not ready, it must name the
exact downstream phase, cap, removal trigger, and test proving the adapter cannot
satisfy ordinary authority APIs.

Plans must not use comforting adapter language such as "temporarily bridge",
"preserve compatibility", "wrap existing path", "shim until later", or
"incrementally migrate" without immediately classifying the surface as deleted,
collapsed, certification-only, capped residue, or Query/runtime gap. If an
adapter remains in production because cleanup is tedious, the plan is wrong.

Review turns must prioritize missed composition, line-cap violations, broad
files, vague buckets, and static/global compatibility paths before ordinary
behavioral gaps. A phase that passes behavior tests but leaves a slow conversion
path in production is regressed, not passed.

## Final Acceptance

This gate is complete only when:

- every graph-affecting operator and boolean stage produces a touched graph
  basis before execution
- Query obligation selection consumes the touched basis
- validator and invariant selection derives from touched basis closure
- derived invalidation, evidence lookup, replay, undo, conflict, and cache
  plans consume the same basis
- public facades expose proof/status without raw construction
- old static/global paths are deleted or mechanically sealed as residue
- line-cap and composition checks are green for touched code and tests
- `7.5` readiness is explicitly certified against this gate

