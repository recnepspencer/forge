# Worth Forge Query Runtime Rewrite Gate

> **Status:** Proposed rewrite gate before continuing Milestone 3
>
> **Primary owners:** `worth-schema`, `worth-topo`, and `forge-query`
>
> **Roadmap position:** between completed Worth Milestone 2 and active Worth
> Milestone 3
>
> **Detailed hard-break spec:** [forge-query-runtime-kernel-hard-break.md](./forge-query-runtime-kernel-hard-break.md)
>
> **Core rule:** Worth does not work around Forge Query gaps. Worth exposes
> them, hardens Forge Query, and then builds on the hardened query surface.

Detailed phase definitions live in:

- [Detailed phases](./forge-query-runtime-rewrite-plan-phases.md)
- [Acceptance, evidence, and endgame obligations](./forge-query-runtime-rewrite-plan-acceptance.md)

## Goal

Rewrite the current Worth schema/topology runtime surface so Worth consumes
Forge Query as its direct public runtime/query spine, with no backwards
compatibility obligation for the existing Worth reader, authority, or edit
runner APIs.

The detailed production hard-break requirements for that rewrite now live in
[forge-query-runtime-kernel-hard-break.md](./forge-query-runtime-kernel-hard-break.md).
This main gate document remains the overview and sequencing shell; the
hard-break spec is the authority for the exact runtime shape, DX output
standard, and adversarial deletion bar.

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

Solve the hard structural problem first: make Forge Query the runtime spine
before more Worth edit behavior builds on old orchestration.

### `arch_laws.md`

Protect enforceable authority/derivation, proof-bearing phase boundaries, and
facade ownership. Old Worth direct runtime entrypoints must become unavailable,
not merely discouraged.

### `perf_laws.md`

Protect semantic-delta-bounded work and visible cost. Worth edit fallout must
route through Query live/computed dependencies with counters and receipts, not
through hidden broad rebuilds or manual invalidation.

### `domain_laws.md`

Protect responsibility-shaped modules and domain-aligned naming. The rewrite
must not create generic adapter bags.

### Worth and Forge Query vision/roadmap docs

Worth keeps truth canonical and derived projections rebuildable. Forge Query is
the typed, composable, aspect-aware runtime/query layer. Any generic capability
Worth exposes as missing must be hardened in `forge-query`, not patched locally.

## Adversarial Constraint

Worth topology truth, naming truth, topology edits, derived topology reads,
branch-local reads, replay certification, and diagnostic inspection must flow
through Forge Query's public workspace facade without preserving legacy Worth
runtime APIs or inventing Worth-local substitute query machinery.

The rewrite fails if:

- Worth keeps `WorthTopologyReader`, `WorthTopologyAuthority`, or
  `WorthTopologyEditRunner` as ordinary public runtime entrypoints.
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

## Phase Map

The detailed phase text lives in
[forge-query-runtime-rewrite-plan-phases.md](./forge-query-runtime-rewrite-plan-phases.md).
The short phase map is:

1. Freeze query-native Worth vocabulary.
2. Harden Forge Query for Worth runtime gaps.
3. Replace Worth authority with aspect-native Query authority.
4. Replace Worth reads with live views and materialization.
5. Rebuild derived topology as computed surfaces.
6. Finish query-native authoritative write execution.
7. Rewrite topology editing onto Query receipts.
8. Cut legacy public runtime surfaces.
9. Rewrite certification and closeout around query-native proof.
10. Update docs, roadmaps, and public examples.

## Remaining Breakpoints

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

The remaining work must proceed in this order:

1. finish query-native authoritative write execution
2. rewrite topology edit execution onto Query
3. cut legacy public runtime surfaces
4. close certification and closeout on the query-native path only
5. clean docs and examples so they describe only the surviving public story

This order is acceptance-critical. Doing legacy deletion before edit execution
would strand admitted workflows; doing certification closeout before deletion
would risk certifying a coexistence architecture we do not want.

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

## Acceptance Overview

The detailed acceptance/evidence matrix now lives in
[forge-query-runtime-rewrite-plan-acceptance.md](./forge-query-runtime-rewrite-plan-acceptance.md).
At the top level, the rewrite is not complete until:

- Worth ships a Forge Query-backed topology workspace/runtime assembly.
- topology truth writes, reads, derived surfaces, and at least one edit family
  execute query-natively.
- old reader/authority/runner execution APIs are no longer required by
  external Worth use.
- generic Query gaps exposed by Worth are hardened upstream and documented as
  such.

## Self-Check

- **Does this solve a structural problem?** Yes. It replaces duplicate runtime
  orchestration with Forge Query as the single public query/runtime spine.
- **Is the adversarial constraint load-bearing?** Yes. It forbids compatibility
  surfaces and Worth-local workarounds, forcing missing generic capability into
  Forge Query.
- **Does it preserve authority boundaries?** Yes. Truth remains relational,
  query/runtime facade remains Query, topology meaning remains Worth.
- **Does it define proof obligations?** Yes. The acceptance appendix names
  Worth, Query, and cross-system evidence.
- **Can implementation map to types/modules/tests?** Yes. The phase appendix
  names vocabulary, workspace assembly, write authority, live views, computed
  surfaces, edits, certification, and docs.
- **Does it belong in sequence?** Yes. It is a gate between completed derived
  topology and active topology editing.
