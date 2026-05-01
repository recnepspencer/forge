# Worth Forge Query Runtime Rewrite Gate: Detailed Phases

This appendix holds the detailed phase definitions for the main rewrite plan:
[forge-query-runtime-rewrite-plan.md](./forge-query-runtime-rewrite-plan.md).

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
[_docs/forge-query/runtime-authoritative-mutation-evidence-plan.md](../forge-query/runtime-authoritative-mutation-evidence-plan.md),
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

Existing Worth algorithms remain domain algorithms, but they must no longer be
public runtime orchestration surfaces.

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

Each computed declaration must explicitly name upstream dependencies, read
aspects, produced aspects, fallback posture, equivalence basis, and retained
inspection evidence.

If Forge Query's computed maintainer contract cannot honestly express a Worth
derived phase, Phase 5 pauses and Phase 2 expands Forge Query before Worth
continues.

### Phase 6: Finish Query-Native Authoritative Write Execution

Complete the authority-side break so Worth topology truth mutation enters
through Forge Query as the real ordinary path, not as migration scaffolding
beside the old authority API.

The detailed hard-break requirements for this phase family, including the ban
on compatibility-mirror runtime assembly and row-to-record reconstruction,
live in
[forge-query-runtime-kernel-hard-break.md](./forge-query-runtime-kernel-hard-break.md).

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
- if direct authority remains reachable during migration, it is isolated behind
  an explicitly legacy seam rather than the main public facade
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

Phase 7 is complete only when at least one admitted topology edit family
executes entirely through Query, fallout routing comes from Query receipts /
retained computed evidence, and branch-local edit behavior uses admitted Query
branch facilities or fails typed and early.

### Phase 8: Cut Legacy Public Runtime Surfaces

Once write authority and edit execution are query-native, remove the old
public runtime story aggressively rather than preserving a coexistence period.

Delete or privatize:

- `WorthTopologyReader` as a public runtime orchestrator
- public read helpers that bypass Query live/computed/state/inspection handles
- public direct `WorthTopologyAuthority` commit APIs that remain exposed after
  Phase 6
- public `WorthTopologyEditRunner` execution APIs in their legacy
  authority/reader-owned form

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
- unsupported Forge Query families fail typed and early
- no Worth public compatibility entrypoint remains necessary for closeout
- the query-native path has actually collapsed Worth-local orchestration

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
