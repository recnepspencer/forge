# Worth Forge Query Runtime Rewrite Gate: Acceptance And Endgame

This appendix holds the acceptance obligations for the main rewrite plan:
[forge-query-runtime-rewrite-plan.md](./forge-query-runtime-rewrite-plan.md).
The detailed kernel hard-break output standard and deletion bar live in
[forge-query-runtime-kernel-hard-break.md](./forge-query-runtime-kernel-hard-break.md).

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
- deletion of compatibility-mirror runtime assembly and query-row-to-fake-
  relational-record reconstruction from the ordinary Worth runtime path
- Forge Query hardening patches for every generic runtime/query capability this
  rewrite exposes as missing.
- Updated roadmap/spec language making this gate a dependency of Milestone 3.

## Must Delete Or Privatize

The following concepts may remain internally as algorithms or proof packet
types, but must not remain ordinary public runtime entrypoints:

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

## Acceptance Evidence

### Worth Evidence

- `cargo test -p worth-schema`
- `cargo test -p worth-topo`
- a query-native Worth workspace construction test
- a topology write receipt test proving affected live and computed surfaces
- a topology live view read/observe test over authored truth
- a materialized/interpreted/validated computed-surface test
- a topology-operator application test over at least one admitted topology edit
  family
- a query-native graph-shaped edit application test over at least one admitted
  topology edit family that requires Query graph composition rather than scalar
  batch mutation
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
- graph-composition breadth where graph-shaped edits are admitted
- validation row count
- fallback count and fallback class
- branch-local read/write breadth where branch behavior is admitted

Whole-view fallback may remain as explicit debt only if it is visible in
receipts, computed inspection, diagnostics, or certification counters.

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
