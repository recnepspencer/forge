# Worth Forge Query Runtime Rewrite Gate

> **Status:** Proposed rewrite gate before continuing Milestone 3
>
> **Primary owners:** `worth-schema`, `worth-topo`, and `forge-query`
>
> **Roadmap position:** between completed Worth Milestone 2 and active Worth
> Milestone 3
>
> **Core rule:** Worth does not work around Forge Query gaps. Worth exposes
> them, hardens Forge Query, and then builds on the hardened query surface.

## Goal

Rewrite the current Worth schema/topology runtime surface so Worth consumes
Forge Query as its direct public runtime/query spine, with no backwards
compatibility obligation for the existing Worth reader, authority, or edit
runner APIs.

The result should make Worth a pressure test for Forge Query's stabilized
runtime facade while preserving Worth's domain responsibility boundaries:

- `worth-schema` owns Worth domain vocabulary, aspect names, schema basis
  declarations, and query-facing schema constants.
- `worth-topo` owns topology semantics, topology-only edit contracts,
  topology materialization/interpretation/validation algorithms, and topology
  certification meaning.
- `forge-query` owns live views, computed surfaces, reads, writes, materialized
  derived state, state snapshots, support/admission gates, inspection, and the
  generic runtime/query capabilities Worth needs.
- Worth writes must be aspect-native. JSON payloads may remain generic app
  examples in Forge Query docs, but Worth topology writes must not encode
  authoritative topology meaning as opaque JSON blobs that the runtime has to
  rediscover.

## Why This Rewrite Deletes Code

This gate is not only a dependency migration. It is a code-deletion and
runtime-ownership rewrite.

Worth should come out of this gate with less local orchestration because Forge
Query becomes responsible for the generic runtime loops that Worth currently
has to spell out itself.

The intended leverage is:

- one mutation pipeline instead of separate Worth-local author/apply/fallout/
  explain pipelines
- automatic fallout routing from touched aspects into live and computed
  surfaces instead of manual invalidation or hand-routed recompute
- retained derived topology state as Forge Query computed surfaces instead of
  a Worth-owned read/runtime shell
- branch and preview behavior expressed through Forge Query basis/lane APIs
  instead of special Worth execution modes
- receipts, state snapshots, and inspection replacing custom explanation glue
- eventual query-owned writeback and write-trigger loops shrinking future
  topology edit-runner code instead of growing another Worth-local runtime

If an implementation batch merely renames Worth calls while preserving the same
Worth-local orchestration burden, it has not satisfied the spirit of this
rewrite.

## Governing Document Summaries

### `MENTALITY.md`

Protects adversarial-design-first engineering. This rewrite must solve the
hard structural problem first: making Forge Query the runtime spine before more
Worth edit behavior builds on old orchestration.

### `arch_laws.md`

Protects enforceable authority/derivation, proof-bearing phase boundaries, and
facade ownership. The rewrite must make old Worth direct runtime entrypoints
unavailable rather than merely discouraged.

### `perf_laws.md`

Protects semantic-delta-bounded work and visible cost. Worth edit fallout must
route through query live/computed dependencies with counters and receipts, not
through hidden broad rebuilds or manual invalidation.

### `domain_laws.md`

Protects responsibility-shaped modules and domain-aligned naming. The rewrite
must not create generic adapter bags; each new module must represent a real
domain or runtime responsibility.

### Worth Vision

Protects the thesis that the spec graph is truth and every derived projection
is rebuildable and traceable. Worth must use Forge Query to ask for and derive
truth, not create another read/runtime model.

### Worth Roadmap

Protects the rule: commit truth canonically once and derive everything else
honestly. This rewrite belongs before Milestone 3 because topology editing
would otherwise deepen dependency on a soon-to-be-replaced direct runner shape.

### Worth Test Requirements

Protects exact success or exact structured failure with machine-checkable
artifacts. Certification must be rewritten onto Forge Query receipts, live
views, computed materializations, state, and inspection rather than proving
legacy and query paths side by side.

### Worth Topology Certification Requirements

Protects topology hostility proof, topology purity, replay parity, and no
toy-shape closure. The query rewrite must preserve the topology certification
bar and add query-native proof of wakeup, invalidation, and derived
materialization behavior.

### Forge Query Vision

Protects the role of Forge Query as the typed, composable, aspect-aware layer
for asking truth. Worth is a first serious geometry-kernel consumer and should
harden missing generic query/runtime capabilities rather than bypass them.

### Forge Query Roadmap

Protects the operating rule: declare query intent once, lower it once, execute
it against canonical truth. Any Worth-needed runtime feature that belongs
generically in Query must become Forge Query roadmap work, implementation work,
and certification work.

### Forge Query Test Requirements

Protects canonical query meaning under builder variation, branch divergence,
live churn, policy, temporal/async support gates, and runtime/store variation.
Worth migration tests must contribute new domain-pressure lanes to these proof
families where they expose generic Query gaps.

### Forge Query Runtime API Public Stabilization Closeout

Protects the stable workspace facade currently safe for downstream runtime
work: live views, computed, effects, preview/branch, writes, reads, observe,
materialize, state, inspection, and support/admission. Worth may build on this
stable synchronous runtime surface now and must fail closed around deferred
temporal/async/store/durable features.

### Forge Query Runtime Authoritative Mutation Evidence Plan

Protects the next write-heavy downstream dependency contract: target evidence,
existing-truth identity binding, batch/session authority evidence, and admitted
naming/continuity mutation evidence must become first-class Query surfaces so
Worth does not rebuild authority explanation, causality, lineage, or
provenance locally during Phase 6.

## Adversarial Constraint

Worth topology truth, naming truth, topology edits, derived topology reads,
branch-local reads, replay certification, and diagnostic inspection must flow
through Forge Query's public workspace facade without preserving legacy Worth
runtime APIs or inventing Worth-local substitute query machinery; if any
generic runtime/query capability is missing, Forge Query must be hardened so
the same capability becomes available to future domains through the same
public facade.

The rewrite fails if:

- Worth keeps `WorthTopologyReader`, `WorthTopologyAuthority`, or
  `WorthTopologyEditRunner` as public compatibility entrypoints.
- Worth exposes a shadow live-read, invalidation, materialization, or
  inspection mechanism that overlaps Forge Query responsibilities.
- Worth adds domain-specific workarounds for generic query/runtime gaps.
- Forge Query support metadata says a family is unsupported/deferred while
  Worth relies on it as though it were stable.
- certification proves old and new paths equivalent instead of certifying the
  new query-native path as the only path.
- derived topology cannot be destroyed and rebuilt from authoritative truth
  plus declared query/computed contracts.

## Product Decision Lock

- This is a rewrite, not a compatibility migration.
- Old Worth public runtime entrypoints may be deleted or made private wherever
  the query-native surface replaces them.
- Worth call sites and certification suites should move forward to the new
  surface rather than receive compatibility adapters.
- Forge Query is allowed to change as part of this work.
- If Worth exposes a missing generic runtime/query feature, the fix belongs in
  `forge-query` unless the missing behavior is truly Worth-specific topology
  meaning.
- Worth must remain geometry-free in `worth-topo`.
- Worth must use aspect-native write declarations rather than
  `serde_json::Value` payload blobs for topology truth and topology edits.
- `workspace.intent(...)` is not part of the stable Worth rewrite path until
  Forge Query admits intent support.
- Temporal, async/resource, store-backed, and durable restart behavior remain
  fail-closed unless their Forge Query support rows become admitted.

## Capability Classification Rule

Every implementation batch must classify each needed capability before coding:

- **Worth domain meaning:** belongs in `worth-schema` or `worth-topo`.
- **Generic runtime/query capability:** belongs in `forge-query` first or in
  the same batch before Worth consumes it.
- **Lower-runtime authority:** remains owned by `forge-relational`,
  `forge-runtime-bridge`, or `forge-signal`, then exposed through Forge Query.
- **Deferred support family:** must fail closed through Forge Query support and
  admission rather than receive a Worth workaround.

This rule is acceptance-critical. A Worth-local workaround for a generic Query
gap is a spec violation.

## Phases

### Phase 1: Freeze Query-Native Worth Vocabulary

Define the Worth query vocabulary in `worth-schema` without runtime behavior:

- collection names for topology entities, topology relations, persistent names,
  topology diagnostics, materialized topology, interpreted topology, validation
  reports, and equivalence contracts
- aspect path constants for topology structure, ownership, boundary, radial,
  naming, diagnostics, lineage, geometry-safe opaque binding identifiers, and
  fallback evidence
- schema-basis identifiers for authoritative topology truth and each derived
  topology surface
- conversion helpers from `WorthAspect` and topology edit touched-aspect sets
  into Forge Query aspect path strings
- proof that no query vocabulary type executes mutation, materialization, or
  validation behavior

Phase 1 is complete only when query declarations can use Worth vocabulary
without stringly scattered aspect names in `worth-topo`.

### Phase 2: Harden Forge Query For Worth Runtime Gaps

Before Worth rewrites public runtime surfaces, identify and close the generic
Forge Query gaps exposed by Worth's current code.

Likely hardening candidates:

- public declaration-construction seams that let downstream crates author
  live/computed declarations with domain-owned vocabulary without reaching into
  runtime-private builder constructors or rebuilding Query declaration logic
- aspect-native query/runtime mutation families that let domain code author
  inserts, updates, deletes, clears, and ordered batches in terms of touched
  aspects rather than generic payload blobs
- receipt and inspection contracts that preserve authored mutation meaning,
  touched aspects, and touched-surface fallout instead of forcing Worth to
  rediscover meaning from lower-runtime payload lowering
- write receipts that retain domain authority evidence and expose it through
  `workspace.inspect(...)`
- relational-runtime-backed live sources whose affected-live-view routing is
  based on declared aspects and collections
- computed maintainers that can rebuild from retained live snapshots when a
  delta-only maintainer cannot honestly derive the surface
- explicit whole-refresh fallback evidence on computed topology surfaces
- support/admission rows for any new handle, receipt, state, or inspection
  family introduced by domain-backed writes

Phase 2 must not add Worth-specific semantics to Forge Query. It should add
generic runtime/query primitives that Worth then consumes as one downstream
domain.

### Phase 3: Replace Worth Authority With Aspect-Native Query Authority

Remove the public direct authority path for topology commits and make
aspect-native `workspace.insert(...)`, `workspace.update(...)`,
`workspace.delete(...)`, and `workspace.batch(...)` the ordinary authoritative
mutation entrypoints for Worth topology truth.

Implementation shape:

- Worth edit contracts lower into aspect-native Forge Query mutation families
  with explicit touched-aspect meaning preserved in the public write surface.
- If lower-runtime adapters still need compatibility lowering internally, that
  lowering stays below the Forge Query facade and must not become the Worth
  authoring model.
- The Forge Query write authority invokes the underlying relational authority
  path and emits one canonical write receipt.
- Worth trace envelopes become retained receipt/inspection evidence rather
  than the public mutation API result.
- Touched aspects, changed scopes, mutation origins, branch-local application
  posture, and performance counters are carried in the receipt/inspection
  contract.

Delete or privatize public use of:

- `WorthTopologyAuthority::apply_topology_intent_traced`
- `WorthTopologyAuthority::apply_topology_intent_on_branch_traced`
- public direct minting of `VerifiedTopologyCommit` outside the query write
  authority

Phase 3 is complete only when admitted topology truth mutation enters through
aspect-native Forge Query mutation surfaces in tests and public examples, with
`workspace.write(...)` treated only as a compatibility seam rather than the
target Worth API.

Current literal blocker families that must be eliminated rather than worked
around:

- batch authoring needs admitted symbolic create-reference support for same-batch
  topology graph construction
- existing-truth edits need admitted authoritative identity binding between
  Worth authority identities and Query entity identities
- persistent-name truth still needs admitted projected naming writeback so Worth
  does not reintroduce a shadow runtime just to pair naming entities with their
  target edges

These are now upstreamed explicitly into the Forge Query side quest in
[_docs/forge-query/runtime-authoritative-mutation-evidence-plan.md](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/forge-query/runtime-authoritative-mutation-evidence-plan.md),
which now spans the Query public contract and the bridge carry-forward contract
as one end-to-end authority-evidence hardening spec.

### Phase 4: Replace Worth Reads With Live Views And Materialization

Remove `WorthTopologyReader` as a public runtime read orchestrator and replace
its public responsibilities with query handles:

- authoritative topology truth snapshots use `workspace.live_view(...)`
- current truth reads use `workspace.read(...)`
- incremental fallout uses `workspace.observe(...)`
- derived topology products use `workspace.computed(...)`
- derived rows use `workspace.materialize(...)`
- readiness/posture uses `workspace.state(...)`
- explanations use `workspace.inspect(...)`

Existing Worth algorithms remain domain algorithms:

- materialization remains topology meaning
- interpretation remains topology meaning
- validation remains topology meaning
- diagnostics and equivalence contracts remain Worth certification meaning

They must no longer be public runtime orchestration surfaces.

This phase is where Worth should stop owning manual read orchestration.
Automatic wakeup, retained state, observation, and inspection belong to Forge
Query once the domain algorithm has been declared.

Phase 4 is complete only when the public `worth-topo` facade no longer
requires `WorthTopologyReader` as an external runtime entrypoint, and external
read workflows are expressed through query assembly, certification helpers, or
direct Forge Query handles instead.

### Phase 5: Rebuild Derived Topology As Computed Surfaces

Create retained Forge Query computed surfaces for the Milestone 2 derived
pipeline:

- `worth.topology.materialized`
- `worth.topology.interpreted`
- `worth.topology.validation`
- `worth.topology.diagnostics`
- `worth.topology.equivalence_contract`

Each computed declaration must explicitly name:

- upstream live or computed dependencies
- read aspects
- produced aspects
- incremental or whole-refresh fallback posture
- equivalence basis where reuse is claimed
- retained inspection evidence

If Forge Query's computed maintainer contract cannot honestly express a Worth
derived phase, Phase 5 pauses and Phase 2 expands Forge Query before Worth
continues.

### Phase 6: Finish Query-Native Authoritative Write Execution

Complete the authority-side break so Worth topology truth mutation enters
through Forge Query as the real ordinary path, not as migration scaffolding
beside the old authority API.

This phase is narrower than topology editing. It is about the canonical truth
write path itself.

Implementation shape:

- topology truth inserts, updates, deletes, clears, and ordered write batches
  lower into aspect-native Forge Query mutation authoring
- Query-side authoritative mutation evidence must be admitted strongly enough
  that existing-truth target binding, projected naming writeback, and later
  continuity-sensitive mutation do not require Worth-local recovery glue
- public Worth examples and proof lanes stop teaching direct authority-path
  commit entrypoints as the ordinary path
- write receipts remain the canonical authority evidence surface
- branch-local authoritative writes only use admitted Forge Query branch/basis
  capability; unsupported branch families still fail closed
- any remaining generic write-authority gap discovered here gets fixed in
  `forge-query` before Worth continues

Delete or privatize public use of:

- direct `WorthTopologyAuthority` commit entrypoints as the ordinary
  application surface
- public caller construction paths that mint verified topology commits outside
  the query write authority

Phase 6 is complete only when:

- authoritative topology truth mutation is proven through aspect-native Forge
  Query mutation surfaces
- no new proof, example, or runtime path treats direct authority APIs as the
  preferred public story
- write receipts and inspection carry the authority evidence Worth needs

### Phase 7: Rewrite Topology Editing Onto Query Receipts

Rebuild Milestone 3 topology editing on the query-native substrate.

The public edit surface should be topology-domain authoring only:

- edit contracts
- edit batches
- changed-scope declarations
- naming-preservation or naming-ambiguity conclusions
- derived-region declarations

Execution belongs to Forge Query:

- edits commit through aspect-native `workspace.insert(...)`,
  `workspace.update(...)`, `workspace.delete(...)`, or `workspace.batch(...)`
- affected live views come from write receipts
- affected computed surfaces come from write receipts
- local recompute evidence comes from computed inspection
- branch/preview behavior uses admitted Forge Query branch/preview APIs only

The intended outcome is not just different call sites. It is that Worth edit
execution no longer owns its own fallout router. Touched aspects, live wakeup,
computed wakeup, fallback posture, and explanation should flow from Forge Query
receipts and retained inspection/state evidence.

Old runner-style APIs must not remain as public convenience surfaces.

Phase 7 is complete only when:

- at least one admitted topology edit family executes entirely through Query
- edit fallout routing comes from Query receipts / retained computed evidence
  rather than Worth-local orchestration
- branch-local edit behavior uses admitted Query branch facilities or fails
  typed and early

### Phase 8: Cut Legacy Public Runtime Surfaces

Once write authority and edit execution are query-native, remove the old
public runtime story aggressively rather than preserving a coexistence period.

This phase is the clean-break deletion pass.

Delete or privatize:

- `WorthTopologyReader` as a public runtime orchestrator
- public read helpers that bypass Query live/computed/state/inspection handles
- public direct `WorthTopologyAuthority` commit APIs that remain exposed after
  Phase 6
- public `WorthTopologyEditRunner` execution APIs in their legacy
  authority/reader-owned form

This phase is complete only when:

- external Worth users no longer need reader/authority/runner runtime
  entrypoints to perform admitted work
- the public facade tells one coherent query-native runtime story
- compile-time or public-surface tests prove the legacy runtime entrypoints are
  no longer required

### Phase 9: Rewrite Certification And Closeout Around Query-Native Proof

Rewrite Milestone 1 and Milestone 2 certification, then Milestone 3 edit
certification, so the query-native surface is the only certified surface.

Certification must assert:

- topology truth writes enter through aspect-native mutation surfaces rather
  than payload-first compatibility paths
- topology live views wake only when their declared aspects are touched
- derived topology computed surfaces wake only through declared dependencies
- materialized/interpreted/validated topology digests match the old semantic
  expectations, without retaining the old public API
- write receipts expose affected live and derived surfaces
- `workspace.inspect(...)` exposes Worth authority, derived, and diagnostic
  evidence
- completed Milestone 1 read certification runs through the canonical Worth
  query assembly, using live reads plus retained `workspace.state(...)` and
  `workspace.inspect(...)` evidence instead of the old reader-owned staging
  path as the real engine
- completed Milestone 2 derived-read certification runs through the canonical
  Worth query assembly, using `workspace.state(...)` and `workspace.inspect(...)`
  on the retained validation/equivalence surfaces rather than treating the old
  reader-owned staging path as the real engine
- unsupported Forge Query families fail typed and early
- no Worth public compatibility entrypoint remains necessary for closeout
- the query-native path has actually collapsed Worth-local orchestration rather
  than preserving a second invalidation or explanation runtime under new names

Phase 9 is complete only when:

- Milestone 1, Milestone 2, and active Milestone 3 proof surfaces certify the
  query-native path as the only real runtime path
- any temporary old-vs-new migration parity assertions have been removed or
  reduced to narrow historical semantic checks rather than runtime
  coexistence proof
- closeout evidence no longer depends on old public runtime shapes

### Phase 10: Update Docs, Roadmaps, And Public Examples

Update the Worth roadmap and milestone specs so this rewrite gate is not an
orphan:

- mark Milestone 3 as depending on this gate
- replace old reader/authority/runner examples with query-native examples
- update closeout expectations to name Forge Query receipts, state, and
  inspection surfaces
- update Forge Query docs/tests when Worth hardens a generic capability
- remove compatibility language from Worth docs where it would preserve old
  public runtime shapes

Phase 10 is complete only when:

- the docs describe the remaining public runtime story honestly
- old reader/authority/runner examples are gone
- downstream guidance names the query-native write/read/edit/certification
  story consistently

## Remaining Breakpoints

This section is the explicit endgame map for the rest of the rewrite.

- `WorthTopologyAuthority` ordinary public commit path:
  killed by Phase 6 and fully removed from the public runtime story by Phase 8
- `WorthTopologyEditRunner` legacy execution shape:
  rewritten in Phase 7 and removed from the old public form in Phase 8
- `WorthTopologyReader` public orchestration role:
  functionally bypassed by completed query-native read/certification work and
  removed as a public runtime dependency in Phase 8
- legacy runtime coexistence in certification:
  eliminated in Phase 9

## Endgame Sequence

The remaining work should proceed in this order:

1. finish query-native authoritative write execution
2. rewrite topology edit execution onto Query
3. cut legacy public runtime surfaces
4. close certification and closeout on the query-native path only
5. clean the docs and examples so they describe only the surviving public story

This order is acceptance-critical. Doing legacy deletion before edit execution
would strand admitted workflows; doing certification closeout before deletion
would risk certifying a coexistence architecture we do not want.

## Must Ship

- Worth query vocabulary in `worth-schema`.
- A Forge Query-backed Worth topology workspace/runtime assembly.
- Query-native authoritative topology write path.
- Query-native topology live views.
- Query-native computed surfaces for materialization, interpretation,
  validation, diagnostics, and equivalence contracts.
- Query-native topology edit execution.
- Query-native certification harnesses for completed Milestone 1 and Milestone
  2 proof surfaces and active Milestone 3 edit surfaces.
- Forge Query hardening patches for every generic runtime/query capability this
  rewrite exposes as missing.
- Updated roadmap/spec language making this gate a dependency of Milestone 3.

## Must Delete Or Privatize

The following concepts may remain internally as algorithms or proof packet
types, but must not remain public runtime entrypoints:

- direct `WorthTopologyReader` orchestration
- direct `WorthTopologyAuthority` commit APIs
- direct `WorthTopologyEditRunner` execution APIs
- public caller construction of verified topology commits outside the query
  write authority
- JSON-blob topology write entrypoints where an aspect-native command can
  express the same mutation
- public read helpers that bypass Forge Query live/computed/state/inspection
  handles
- manual invalidation or derived-fallout APIs that overlap Forge Query write
  receipts and computed dependency routing

## Must Preserve

- `forge-relational` remains authoritative for truth.
- Forge Query remains the query/runtime facade, not the domain meaning owner.
- `worth-schema` remains vocabulary/schema meaning, not runtime execution.
- `worth-topo` remains topology-only and geometry-free.
- Derived topology remains rebuildable from authoritative truth plus declared
  query/computed contracts.
- Existing Worth admitted primitive-family semantics remain intact.
- Existing Worth validator/rejection/failure-locality meanings remain intact.
- Worth certification remains family-shaped and hostile, not demo-shaped.
- Forge Query deferred support families remain fail-closed unless admitted by
  their owning milestones.

## Acceptance Evidence

The rewrite is not complete until the following evidence exists.

### Worth Evidence

- `cargo test -p worth-schema`
- `cargo test -p worth-topo`
- a query-native Worth workspace construction test
- a topology write receipt test proving affected live and computed surfaces
- a topology live view read/observe test over authored truth
- a materialized/interpreted/validated computed-surface test
- a query-native edit application test over at least one admitted topology edit
  family
- a query-native branch-local test only for branch behavior admitted by Forge
  Query support
- a query-native certification closeout test for Milestone 1 evidence
- a query-native certification closeout test for Milestone 2 evidence
- a public-surface test proving old reader/authority/runner execution APIs are
  no longer required by external Worth use

### Forge Query Evidence

For every generic Forge Query capability hardened during this rewrite:

- focused unit tests for the new Query capability
- support/admission synchronization tests if the capability affects public
  support posture
- inspection/state tests if the capability introduces new retained evidence
- runtime API stabilization tests if the public facade contract changes
- documentation update under `crates/forge-query/docs` when the capability is
  user-facing

Baseline commands:

- `cargo fmt -p forge-query --check`
- `cargo check -p forge-query --tests`
- `cargo test -p forge-query`
- `cargo test -p forge-query runtime_api_stabilization`
- `cargo test -p forge-query runtime_public_support`

### Cross-System Evidence

- A Worth-driven Forge Query hardening test demonstrating at least one
  topology-domain pressure case as a generic Query capability, not a
  Worth-local workaround.
- A no-silent-widening test proving unsupported Query families fail before
  Worth can rely on them.
- A documentation trace from the Worth rewrite gate to Forge Query docs or
  roadmap rows for every generic capability added.

## Complexity And Counter Obligations

Each hot path must expose counters at the boundary where the cost claim is
made:

- topology write lowering breadth
- touched aspect count
- affected live view count
- considered computed surface count
- affected computed surface count
- materialization entity/relation breadth
- topology relation traversal breadth
- validation row count
- fallback count and fallback class
- branch-local read/write breadth where branch behavior is admitted

The rewrite may initially mark some locality-specific incremental behavior as
explicit debt, but whole-view fallback must be visible in receipts, computed
inspection, diagnostics, or certification counters.

## Allowed Debt

- Fine-grained region-local recompute may remain debt if whole-refresh fallback
  is explicit, counted, and inspected.
- Durable store-backed replay may remain debt because Forge Query marks it as
  Milestone 10/11 scope.
- Temporal and async/resource behavior may remain debt because Forge Query
  marks them as Milestone 9.4+ scope.
- Ergonomic Worth helper builders may remain debt after the query-native spine
  exists.

Not allowed as debt:

- public compatibility wrappers for old runtime entrypoints
- Worth-local substitute query/live/computed/inspection systems
- hidden broad recompute
- opaque JSON write payloads for Worth topology truth where aspect-native
  mutation meaning is available
- unsupported Forge Query family usage without admission
- certification that depends on old public runtime APIs

## Sequencing Notes

This gate belongs before Milestone 3 because Milestone 3 widens topology
editing. Widening edit workflows on top of old Worth direct runner APIs would
increase the migration surface and certify the wrong architecture.

Once this gate closes, Milestone 3 should be rewritten or amended so topology
editing is specified in query-native terms from the beginning:

- edit authoring remains Worth topology meaning
- edit execution uses aspect-native `workspace.insert(...)`,
  `workspace.update(...)`, `workspace.delete(...)`, and `workspace.batch(...)`
- edit fallout is receipt-driven and computed-surface-driven
- edit inspection is `workspace.inspect(...)`
- edit certification is query-native

## Self-Check

- **Does this solve a structural problem?** Yes. It replaces duplicate runtime
  orchestration with Forge Query as the single public query/runtime spine.
- **Is the adversarial constraint load-bearing?** Yes. It forbids compatibility
  surfaces and Worth-local workarounds, forcing missing generic capability into
  Forge Query.
- **Does it preserve authority boundaries?** Yes. Truth remains relational,
  query/runtime facade remains Query, topology meaning remains Worth.
- **Does it define proof obligations?** Yes. It requires Worth, Query, and
  cross-system evidence.
- **Can implementation map to types/modules/tests?** Yes. The phase list names
  vocabulary, workspace assembly, write authority, live views, computed
  surfaces, edits, certification, and docs.
- **Does it belong in sequence?** Yes. It is a gate between completed derived
  topology and active topology editing.
