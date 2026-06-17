# Forge Query Future Roadmap

## Purpose

This document defines the future work for `forge-query`.

It is a future-only roadmap. It does not assume the query layer is already
productized, and it does not treat query as thin convenience syntax over
runtime reads. It exists to sequence the work required to make asking for
truth as rigorous, typed, replay-honest, and live-promotable as the rest of
the Forge stack.

The operating rule for this roadmap is:

`declare query intent once, lower it once, execute it against canonical truth`

That rule governs every milestone:

1. query meaning must be expressed as typed structures rather than strings,
   ad hoc host closures, or runtime-only conventions
2. planning, narrowing, and legality checks must happen before the hot path
   executes reads or live maintenance
3. `forge-query` may compose `forge-relational`, `forge-store`,
   `forge-signal`, and the runtime bridge, but it must not steal authority
   from any of them
4. live delivery, historical reads, and persisted query artifacts must remain
   derived from canonical truth rather than inventing shadow read models

## Adversarial Constraint

`forge-query` must survive the following hostile condition:

> A large branch-bearing truth graph with aspect-rich entities, lineage-heavy
> identity evolution, schema drift, live subscriptions, historical reads,
> query-shaped diffs, and policy-aware masking must produce the same typed
> query result, the same narrowing decision, and the same explanation of why
> that result changed regardless of whether the read came from in-memory truth,
> store-backed historical restore, or replayed live maintenance.

If any supported path:

- lets execution rediscover semantics that should have been lowered into the
  query plan
- forces whole-entity scans when the query declared narrower aspect or scope
- makes live subscriptions interpret raw CDC instead of query-shaped intent
- changes query meaning depending on backend/storage path
- makes historical or saved-query features depend on opaque host glue instead
  of canonical query artifacts
- allows policy masking, branch targeting, or lineage traversal to drift
  between one-shot reads and live promotion

then `forge-query` has failed.

## Roadmap Rules

Rules for every remaining query item:

- each milestone must describe a real query capability boundary, not just
  "add some builders" or "wire one more adapter"
- each milestone must preserve the ownership split:
  `forge-relational` owns truth semantics, `forge-store` owns durable
  persistence, `forge-signal` owns reactive evaluation, and `forge-query`
  owns typed query expression, lowering, and result shaping
- every milestone must distinguish canonical query artifacts from derived
  runtime conveniences
- no milestone is complete until it has machine-checkable acceptance evidence
  through typed plan assertions, parity scenarios, replay checks, or hostile
  subscription/history cases
- sequence numbers express logical dependency order, not a promise that every
  later milestone must wait for every earlier integration detail to land
- every milestone must say what is blocked on `forge-store` so the roadmap
  stays honest while store is still unfinished
- every milestone must declare its own adversarial constraint
- every hot-path milestone must declare named complexity contracts and exact
  counter proof obligations
- any knowingly incomplete first ship must be marked as explicit debt rather
  than implied completeness
- named certification suites in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  are the authoritative acceptance source for milestone closure

## Operating Modes

The roadmap preserves these query operating modes explicitly:

- `Runtime-backed mode`: queries plan and execute directly against
  `forge-relational` snapshot-backed reads
- `Store-backed mode`: queries execute against admitted `forge-store` surfaces
  without changing canonical query meaning
- `Live-promoted mode`: the same canonical query meaning is maintained through
  query-shaped incremental updates
- `Ephemeral artifact mode`: saved queries, templates, or host-bound bindings
  may exist without durable persistence before `forge-store` completes; this
  mode must remain explicit debt rather than implied product completion

## Obligation Surface Convention

Every milestone in this roadmap must be read as four separate obligation
surfaces, even when a section heading still says `Must Ship` for readability:

- `surface primitives`: the concrete query types, plan artifacts, metadata, or
  protocol-facing structures that must exist
- `semantic guarantees`: the meaning that those primitives must preserve under
  execution, live maintenance, history, policy, and replay
- `proof obligations`: the parity checks, hostile scenarios, and machine-
  checkable evidence that must exist before the milestone is honest
- `store-gated completion debt`: any completion claim that must stay open until
  `forge-store` can supply the durable artifact support the query layer depends
  on

If a roadmap line item only names API surface but does not also name semantic
or proof obligations, it is incomplete.

## Platform Framework Stance

`forge-query` is not a read-only helper crate. It is the intended
platform-level framework surface for ordinary domain and application code.

That means:

- ordinary developers should be able to stay inside `forge-query` for the
  majority of read, live, branch-workflow, mutation-orchestration, and
  delivery-shape work they perform
- `forge-query` may expose pass-through, lowering, orchestration, and unified
  configuration surfaces over `forge-relational`, `forge-signal`, and the
  runtime bridge
- lower crates remain the semantic authorities for truth mutation, merge
  semantics, reactive scheduling, preview-session lifecycle, writeback safety,
  persistence, and durable transport contracts
- "platform-level facade" therefore means one daily-driver import with
  authority-preserving lowering, not one giant crate that steals ownership
  from the runtimes below it

The roadmap must stay honest to both halves of that claim:

- if a domain developer would reasonably expect to do it through the query
  framework, it needs an explicit roadmap home here
- if the lower crate remains authoritative for the semantics, the roadmap must
  say so explicitly instead of implying that `forge-query` became a second
  truth, merge, or writeback engine

## Early Cross-Feature Proof Gates

The hardest failures in `forge-query` live at feature intersections rather than
inside isolated feature families. The roadmap therefore requires these
cross-feature proof gates before final certification:

- `Milestone 5` must prove live + ordering + projection parity for admitted
  query families, and must preserve the architectural seams that let
  `Milestone 9` add live + policy masking parity without redefining live query
  meaning
- `Milestone 5.1` must prove region-scoped invalidation narrowing and
  change-stream-backed delivery contracts without collapsing live query meaning
  into raw partition events or transport-local stream glue
- `Milestone 5.2` must prove preview-session query contexts preserve branch
  workflow basis identity, preview lifecycle identity, and promoted-result
  comparison semantics without ambient host orchestration
- `Milestone 5.3` must prove frontier-aware planning and deterministic parallel
  admission preserve canonical execution meaning while making planning cost
  posture explicit
- `Milestone 5.4` must prove structural correspondence and historical
  materialization-path metadata remain explicit and ambiguity-honest
- `Milestone 5.5` must prove query-authored mutation, merge, and writeback
  declarations lower into lower-crate authorities without duplicating mutation
  semantics or hiding branch-workflow truth behind host glue
- `Milestone 5.6` must prove the unified facade and unified runtime
  configuration remain authority-preserving rather than bag-shaped or
  semantics-erasing
- `Milestone 6` must prove historical + diff + result-shape parity for the
  same declared query shape
- `Milestone 7` must prove lineage + correspondence + branch-scoped comparison
  parity for admitted identity-evolution scenarios
- `Milestone 8` must prove view-shape-specific live semantics for at least
  table/detail plus one grouped or temporal view
- `Milestone 9` must prove tenant schema variation + validation + delivery-
  shape parity, and policy masking + historical reads + saved/scope-composed
  queries where those surfaces already exist
- `Milestone 9.1` must prove query-owned subscription declaration families and
  lowering preserve the same canonical query meaning across policy, tenant,
  basis, and view-shape variations without inventing a second live-query
  semantics path or a fake one-size-fits-all subscription kind
- `Milestone 9.2` must prove subscription-family-backed live delivery,
  sharing, continuation, and preview isolation preserve query-shaped parity
  for the same canonical query and admitted live family
- `Milestone 9.3` must prove Query's automatic subscription-family selection
  path remains bridge-honest, diagnostically sufficient, and
  certification-ready rather than smuggling capabilities above the bridge or
  host runtime
- `Milestone 9.3.1` must prove Query inspection can expose one bridge-owned
  cross-runtime causal explanation envelope joining relational authority,
  bridge routing/evaluation, signal invalidation/evaluation, lineage,
  provenance, and replay posture without requiring domains to spelunk lower
  runtimes
- `Milestone 9.3.2` must prove Query basis is a capability lifecycle, not a
  raw branch/snapshot identifier, so observation, mutation, replay, inspection,
  and materialization all consume phase-typed basis proofs
- `Milestone 9.3.3` must prove Query effects execute through one
  authority-scoped effect pipeline that lowers intent once and prevents
  executors from re-deciding authority, basis, strategy, or artifact policy
- `Milestone 9.3.4` must prove materialized projections expose declared,
  typed, receipt-backed fact consumption so consumers can use projection facts
  without reopening source authority
- `Milestone 9.3.5` must prove every Query-crossing intent resolves through a
  structured admission decision lattice with success, advisory, and violation
  traces before construction, lowering, or covered bridge-backed execution
- `Milestone 9.3.6` must prove lower-runtime contact is capability-routed
  through contractual boundary envelopes rather than scattered direct bridge,
  relational, signal, or store access
- `Milestone 9.3.7` must prove downstream domains can contribute typed domain
  capability posture through one public Query-owned contribution seam across
  admission, support, traceability, invariant, workflow, continuity, aftermath,
  and explanation categories, so Query can materialize canonical runtime
  artifacts without forcing domains to mint local pseudo-Query layers
- `Milestone 9.3.8` must prove serious downstream domains can enter Query as a
  first-class platform boundary through one Query-owned declaration,
  progression, routing, inspection, orchestration, and certification seam
  rather than rebuilding local pre-Query declaration, preparation, and handoff
  worlds above relational, the runtime bridge, signal, `forge-proof`, and
  `forge-foundational`
- the `Runtime API Public Stabilization Gate` must freeze the ordinary public
  workspace/handle/state/aspect/effect/intent/inspection contract after the
  runtime API facade consumes 9.1 through 9.3.8, so domain runtimes can build
  now and temporal/async milestones extend the same model later rather than
  adding parallel APIs
- the `Runtime Authoritative Mutation Evidence Gate` must freeze the ordinary
  public mutation-evidence contract after aspect-native mutation and runtime
  facade stabilization are in place, so write-heavy downstream domains do not
  rebuild target recovery, existing-truth identity binding, causality /
  provenance recovery, or authority explanation above Query, and so the Query
  public contract and bridge carry-forward contract remain one end-to-end
  evidence story rather than two drifting halves
- `Milestone 9.4` must prove the merged runtime-backed temporal/async query
  surface as one ordinary Query product lane: basis binding, declaration
  families, result-state, mixed-cause delivery, downstream delivery contract,
  and hostile certification all have to preserve canonical query meaning
  without making Query the owner of clocks, async lifecycle execution, or
  lower-cause ordering authority
- `Milestone 9.5` must prove the reusable query composition, core view-shape,
  grouped composition, projection-consumption, and preserved temporal/async
  reuse debts are closed as production-ready Query productization lanes before
  store-backed and durable milestones build on top of them
- `Milestone 9.6` must prove evidence identity, stop-class matching, and
  session label identity are runtime-owned structural contracts — digests
  survive formatting drift, every covered stop class is matchable without
  string operations, and session labels carry canonical identity with typed
  collision posture
- `Milestone 9.7` must prove N concurrent shared read contexts under commit
  pressure produce byte-identical results and receipts to serialized
  execution, that journal replay reconstructs identical truth and published
  artifacts, and that no reader can acquire a lock on the committed-read hot
  path or trigger derived evaluation
- `Milestone 9.8` must prove a downstream crate can author evidence reports,
  enforce the no-bypass contract, pin support posture, and obtain a valid
  honestly-postured in-memory test runtime using only Query-shipped kit
  surfaces, with closure proven by reference-consumer adoption and deletion
  of the hand-rolled equivalents
- `Milestone 9.9` must prove complete graph touch obligation authority: every
  obligation kind executable, compose/batch/read lanes dispatching, policy-aware
  mutation gates live, explicit relational graph-composition execution point,
  exact-zero duplicate enforcement, and full reference adoption in `worth-topo`
  and `worth-kernel` construction with architectural certification closure
- `Milestone 10` must prove store-backed execution and historical parity for
  admitted shared capability families
- `Milestone 11` must prove durable saved-query, cursor, and artifact reload
  semantics without changing canonical query meaning
- `Milestone 12` must prove blob-backed delivery and large-object query
  semantics without collapsing query meaning into ad hoc file plumbing

## Critical Path And Store Dependencies

The query roadmap has one hard product path and several store-gated completion
tracks.

Critical path:

- `Milestone 1` -> `Milestone 2` -> `Milestone 3` -> `Milestone 4` ->
  `Milestone 5` -> `Milestone 5.1` -> `Milestone 5.2` -> `Milestone 5.3` ->
  `Milestone 5.4` -> `Milestone 5.5` -> `Milestone 5.6` -> `Milestone 6` ->
  `Milestone 7` -> `Milestone 8` -> `Milestone 9` -> `Milestone 9.1` ->
  `Milestone 9.2` -> `Milestone 9.3` -> `Milestone 9.3.1` ->
  `Milestone 9.3.2` -> `Milestone 9.3.3` -> `Milestone 9.3.4` ->
  `Milestone 9.3.5` -> `Milestone 9.3.6` -> `Milestone 9.3.7` ->
  `Milestone 9.3.8` -> `Runtime API Public Stabilization Gate` ->
  `Runtime Authoritative Mutation Evidence Gate` -> `Milestone 9.4` ->
  `Milestone 9.5` -> `Milestone 9.6` -> `Milestone 9.7` -> `Milestone 9.8` ->
  `Milestone 9.9` -> `Milestone 10` -> `Milestone 11` -> `Milestone 12` ->
  `Milestone 13`

Store-gated completion tracks:

- `Milestone 3` can ship a runtime-backed execution path first, but full
  store-pushdown and store-parity are blocked on `forge-store`
- `Milestone 5.1` can close runtime-backed region-scoped invalidation and
  stream-contract semantics first, but durable stream continuation is still
  postponed to `Milestone 11`
- `Milestone 5.2` can ship runtime-backed preview-session query contexts and
  branch-workflow basis semantics first, but durable preview replay and
  persisted workflow artifacts remain dependent on later durable milestones
- `Milestone 5.5` can ship runtime-backed mutation/merge/writeback lowering
  first, but durable workflow continuation and persisted branch-workflow
  artifacts remain postponed to `Milestone 11`
- `Milestone 5.6` can ship the unified facade and unified configuration first,
  but any configuration sections that claim durable resume or store-backed
  guarantees must remain explicit debt until the relevant later milestones
  close
- `Milestone 6` can ship runtime-backed branch/head and admitted basis-
  variation semantics first, but durable point-in-time restore and snapshot-
  plus-tail parity are postponed to `Milestone 10`
- `Milestone 8` can ship scopes, templates, and view semantics first, but
  durable saved-query reload is postponed to `Milestone 11`
- `Milestone 9` can ship policy-aware narrowing and delivery-shape semantics
  first, but durable cursor resume and persisted delivery metadata are
  postponed to `Milestone 11`
- `Milestone 9.1` can ship runtime-backed subscription declaration, lowering,
  and admission first, but durable subscription artifact persistence and
  restart-stable reload are postponed to `Milestone 11`
- `Milestone 9.2` can ship runtime-backed subscription lifecycle, sharing,
  continuation, and preview isolation first, but durable continuation,
  checkpoint survival, and restart-stable subscription metadata are postponed
  to `Milestone 11`
- `Milestone 9.3` can ship runtime-backed subscription diagnostics, bridge
  parity, and certification first, but any claims about durable subscription
  replay or store-backed restart parity must remain explicit debt until
  `Milestone 10` and `Milestone 11` close
- `Milestone 9.3.1` can ship runtime-backed cross-runtime causal diagnostics
  and Query inspection first, but durable causal archives, store-backed replay
  reconstruction, and restart-stable expanded explanation reload remain
  explicit debt until `Milestone 10` and `Milestone 11` close
- `Milestone 9.3.2` can ship runtime-backed basis capability lifecycles first,
  but durable basis reload, store-restored snapshot plus tail reconstruction,
  and restart-stable basis envelopes remain explicit debt until `Milestone 10`
  and `Milestone 11` close
- `Milestone 9.3.3` can ship runtime-backed authority-scoped effect execution
  first, but store-backed effect replay, durable workflow continuation, and
  restart-stable effect envelopes remain explicit debt until `Milestone 10` and
  `Milestone 11` close
- `Milestone 9.3.4` can ship runtime-backed projection consumption receipts
  first, but persisted materialized fact receipts, durable projection
  consumption reload, and store-backed reconstruction remain explicit debt
  until `Milestone 10` and `Milestone 11` close
- `Milestone 9.3.5` can ship runtime-backed admission decision lattices and
  decision traces first, but durable decision-log archives and restart-stable
  trace materialization remain explicit debt until `Milestone 11` closes
- `Milestone 9.3.6` can ship runtime-backed lower-runtime capability routing
  first, but store-backed route parity and durable route replay remain explicit
  debt until `Milestone 10` and `Milestone 11` close
- `Milestone 9.3.7` must close the full domain capability contribution seam
  across all named category families rather than shipping a split between
  finished categories and half-closed category shells that would force later
  rewrites
- `Milestone 9.3.8` must establish Query-owned platform entry for serious
  downstream domain work as one end-to-end seam rather than a split between
  declaration, preparation, and runtime handoff milestones that still leave
  local pseudo-Query scaffolding above lower authorities
- the `Runtime API Public Stabilization Gate` can freeze the stable
  runtime-backed public facade, golden DX transcripts, handle/state/aspect
  contracts, inspection contract, and temporal/async support gates without
  implementing temporal/async behavior; any temporal, async, store-backed, or
  durable behavior remains deferred to its owning milestone
- the `Runtime Authoritative Mutation Evidence Gate` can freeze the stable
  runtime-backed mutation-evidence, existing-truth binding, and admitted
  naming/continuity evidence contract without claiming temporal, async,
  store-backed, durable, or lower-runtime semantic completion beyond the
  admitted public facade
- `Milestone 9.4` can ship the merged runtime-backed temporal/async query
  surface first, but durable temporal replay, persisted async continuation,
  restart-stable saved artifacts, and store-backed temporal/async parity remain
  postponed to `Milestone 10` and `Milestone 11`
- `Milestone 9.5` must close runtime-backed productization debt in reusable
  composition, core view shapes, grouped planning, projection consumption, and
  preserved temporal/async reuse without claiming store-backed saved-query
  reload, durable temporal/async reuse, or restart-stable artifact
  continuation before `Milestone 10` and `Milestone 11`
- `Milestone 9.6` can close runtime-backed canonical evidence identity, typed
  stop classes, and session label identity first, but durable digest archives
  and restart-stable identity reload remain explicit debt until `Milestone 10`
  and `Milestone 11` close
- `Milestone 9.7` can close runtime-backed concurrent read authority,
  deterministic submission, and published-artifact reads first, but durable
  journal persistence, store-backed replay reconstruction, and restart-stable
  published-artifact reload remain explicit debt until `Milestone 10` and
  `Milestone 11` close
- `Milestone 9.8` can close the runtime-backed consumer kit and reference
  adoption first, but persisted support snapshots, durable audit archives, and
  store-backed kit artifacts remain explicit debt until `Milestone 10` and
  `Milestone 11` close
- `Milestone 9.9` closes complete graph touch obligation authority including
  policy-aware graph mutation execution, all obligation kind executors, and full
  reference adoption in `worth-topo` and `worth-kernel` construction
- `Milestone 10` is the first intentionally store-gated execution milestone
- `Milestone 11` is the intentionally store-gated durable artifact milestone
- `Milestone 12` is the intentionally store-gated blob/media milestone

The roadmap is therefore split intentionally between runtime/semantic
milestones that can progress now against canonical runtime truth, and late
store-backed milestones that close parity, durability, portability, and
blob-backed delivery once `forge-store` is ready.

## Milestone 1: Typed Query Expression And Result Shape Foundation

### Goal

Make query intent a first-class typed artifact before execution, live
promotion, or persistence concerns are allowed to sprawl.

### Adversarial Constraint

Different builder paths, helper layers, and host binding surfaces must
normalize to the same canonical query artifact and result-shape meaning for the
same declared intent, with no downstream phase allowed to recover missing
meaning from ambient host context.

### Why This Milestone Exists

`forge-query` cannot honestly plan, validate, optimize, or subscribe to reads
until it has one canonical representation of:

- what is being queried
- which aspects are being projected
- what result shape the caller expects
- what scope the query is allowed to traverse

Without this milestone, every later feature would be forced to reverse-engineer
host-specific query builders or execution closures.

### Must Ship

- one public `forge-query` facade and crate boundary
- typed query expression families for:
  - entity/detail reads
  - collection reads
  - aspect projection
  - bounded relation traversal
  - typed result shapes
- composable query fragments rather than string-based query construction
- canonical query identity/digest surfaces for structurally equal queries
- explicit query context shell that can later carry branch, history, policy,
  and live-promotion parameters without changing query meaning
- diagnostics that explain how a query expression normalized into its canonical
  typed form

### Must Preserve

- query is expression authority, not truth authority
- result shapes stay typed rather than degrading into dynamic maps
- structurally equal queries normalize identically
- host code does not bypass the facade and invent alternate query ASTs

### Complexity / Proof Obligations

- name canonicalization and query-identity contracts in terms of clause count,
  projection width, traversal clause count, and result-shape width
- expose exact counters for normalized clause count, projection entry count,
  result-shape field count, and canonicalization fallback count
- prove canonical query digest parity for equivalent construction paths

### Allowed Debt

- ergonomic builder sugar may remain `Debt` if canonical query identity and
  facade-normalized meaning are already frozen and parity-proven
- alternate host-local query ASTs may not exist as debt

### Sequencing Notes

This belongs first because every later milestone depends on one canonical query
artifact and one canonical result-shape artifact rather than host-specific
construction paths.

### Parallelization Notes

Once canonical query identity is frozen, Milestone 2 validation work and early
Milestone 3 planning work can proceed in parallel.

### Store Dependency

This milestone is not blocked on `forge-store`.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Canonical Query Normalization Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- equivalent query builder paths normalize into the same canonical query
  artifact
- typed result shapes remain structurally aligned with the declared projection
- query identity survives round-tripping through the public facade without
  host-specific loss of meaning

## Milestone 2: Schema-Aware Validation, Aspect Projection, And Predicate Semantics

### Goal

Make query legality explicit so invalid, over-broad, or schema-dishonest
queries fail before execution.

### Adversarial Constraint

No illegal, schema-mismatched, over-broad, or structurally ambiguous query may
advance into planning by relying on runtime fallback, host-side repair, or
implicit whole-entity widening.

### Why This Milestone Exists

The vision only works if queries are constrained enough for the runtime to
optimize and narrow. That requires the query layer to understand schema,
aspects, field types, relation kinds, and bounded traversal legality before it
attempts execution.

### Must Ship

- schema-aware query validation at construction time where compile-time proof
  is not available
- typed predicate expressions over aspect fields
- workflow-aware predicate families as first-class typed predicates rather than
  host-local post-filters
- explicit aspect projection legality checks
- structured content aspect query legality over schema-declared content blocks
  where the schema admits queryable structured content
- bounded traversal legality checks over declared relation kinds and depth
- typed ordering declarations over query-visible fields
- validation diagnostics for:
  - unknown aspects
  - incompatible field predicates
  - illegal traversal edges
  - invalid result-shape bindings
  - schema-version mismatch at query construction

### Must Preserve

- validation must consume authoritative schema semantics from
  `forge-relational`
- query construction must not execute storage reads to decide legality
- aspect-aware reads remain the default instead of silently widening to
  whole-entity fetches
- illegal queries fail explicitly before hot execution

### Complexity / Proof Obligations

- name validation contracts for predicate validation, projection validation,
  traversal validation, and result-shape binding validation
- expose exact counters for validated predicates, validated projection items,
  rejected clauses, traversal legality checks, and whole-entity widening
  denials
- prove that schema-invalid queries fail before any execution-path admission

### Allowed Debt

- compile-time proof coverage may remain `Debt` where construction-time proof
  already exists and is canonical
- any silent widening from illegal or unsupported projection/predicate forms
  may not ship as debt

### Sequencing Notes

This belongs before planning because the planner must consume already-legal
query artifacts instead of rediscovering legality during execution lowering.

### Parallelization Notes

Can proceed in parallel with the canonical-artifact hardening of late
Milestone 1. It should finish before core Milestone 3 plan lowering closes.

### Store Dependency

This milestone is not blocked on `forge-store`.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Schema-Aware Rejection And Projection Legality Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- schema-invalid queries fail during construction with typed diagnostics
- workflow-aware predicates normalize and validate like any other predicate
  family instead of bypassing the canonical query artifact
- structured content queries fail explicitly when the schema/content contract
  does not admit the requested projection or predicate
- legal aspect projections and predicates lower deterministically
- traversal legality remains bounded and explainable rather than becoming
  host-defined graph walking

## Milestone 3: Query Planning And Snapshot-Backed Execution

Status:
Closed on 2026-04-14 for runtime-backed one-shot execution. Store-backed
parity, store pushdown, and durable snapshot-plus-tail semantics remain
explicit debt until `forge-store` can support them honestly.

### Goal

Lower typed query intent into executable plans that read canonical truth
through stable snapshots instead of ad hoc runtime access.

### Adversarial Constraint

Runtime-backed execution, store-backed execution where admitted, and type-bound
host binding paths must all converge to the same query result meaning for the
same canonical query and basis, without the executor rediscovering legality,
projection, or scope semantics that the planner should already have fixed.

### Why This Milestone Exists

Typed query expressions are not enough on their own. The query layer needs an
explicit planner that decides:

- what truth surfaces must be read
- what aspects and relations can be narrowed
- what execution path is legal for this query shape
- what result-shaping work belongs before or after runtime reads

This is the milestone where `forge-query` stops being syntax and becomes an
actual query subsystem.

### Must Ship

- query planner that lowers typed expressions into proof-carrying execution
  plans
- snapshot-backed execution contracts for one-shot reads
- explicit separation between planning, execution, and result shaping
- authoritative-runtime execution path against `forge-relational`
- type-bound execution descriptors that let higher layers bind consumer inputs
  to canonical query plans without redefining query semantics in route or
  handler glue
- result metadata showing what aspects, scopes, and ordering bases were used
- execution diagnostics for fallback, widening, and unsupported plan shapes
- counters for planned read breadth, projected aspect count, traversal breadth,
  and fallback path selection

### Must Preserve

- execution must read from stable snapshots rather than live mutable truth
- executor must not rediscover legality or narrowing decisions already solved
  by the planner
- query results remain derived from canonical truth surfaces
- type-bound binding descriptors remain query-owned metadata while route,
  server, or UI plumbing stays outside `forge-query`
- unsupported plan shapes fail explicitly instead of silently widening

### Complexity / Proof Obligations

- name planning and execution contracts for plan build, plan normalization,
  snapshot-backed execution, and fallback-path admission
- expose exact counters for planned read breadth, projected aspect count,
  traversal breadth, fallback broadening count, and snapshot basis resolution
- prove runtime-backed/store-backed parity where both paths are admitted
- prove an admitted runtime route/plan distinction through certification so
  planner-owned route semantics do not collapse into one happy-path lane

### Allowed Debt

- store-backed pushdown may remain `Debt` while runtime-backed execution is
  canonical and store parity is still blocked on `forge-store`
- executor rediscovery of planner-owned semantics may not ship as debt

### Sequencing Notes

This belongs before collection scale semantics, live promotion, and historical
reads because all of those depend on proof-carrying plans and snapshot-honest
execution.

### Parallelization Notes

Core runtime-backed planning should finish before the rest of the roadmap
builds on it. Store-backed plan variants can advance in parallel as
`forge-store` matures.

### Store Dependency

- Core runtime-backed execution is not blocked on `forge-store`.
- Full completion of store-aware execution parity, snapshot-plus-tail restore
  parity, and honest store pushdown is blocked on `forge-store` milestones for
  canonical commit persistence, snapshots, and durable restore.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Planner / Executor / Binding Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- identical query/context input lowers to identical execution plans
- one-shot execution reads through stable snapshots and returns the declared
  typed result shape
- type-bound execution descriptors round-trip to the same canonical plan as
  direct query invocation
- planner and executor diagnostics can explain when a query used a narrowed
  path versus an authoritative fallback path
- runtime-backed execution and any admitted store-backed execution agree for
  the same truth basis where store support exists

## Milestone 4: Collection Semantics, Ordering, Pagination, And Bounded Traversal

Status:
Closed on 2026-04-14 for runtime-backed collection semantics. Live
maintenance, historical collection replay, durable cursor continuation, and
store-backed collection parity remain intentionally deferred to later
milestones.

### Goal

Make large-surface reads honest and product-grade rather than leaving
collection behavior as host-local loops.

### Adversarial Constraint

Large collections, ordered pages, bounded traversals, aggregations, rollups,
and CDC-shaped outputs must remain proportional to declared query breadth and
must not secretly devolve into unbounded scans, unstable cursor semantics, or
host-side recomputation.

### Why This Milestone Exists

A query layer that can only do detail reads is not a real product surface.
Forge Query needs first-class collection semantics that keep cardinality,
ordering, and traversal cost explicit enough for both the runtime and the
caller to reason about.

### Must Ship

- collection query planning as a first-class path rather than repeated
  detail-query composition
- typed ordering semantics
- opaque cursor-based pagination
- bounded result-set declarations
- bounded relation materialization and eager loading contracts
- aggregation query families with explicit grouping and tolerance declarations
- relational rollup query families over declared relation edges
- query-time derived field declarations that are validated and planned as part
  of the canonical query artifact rather than post-processing host code
- CDC-shaped output/result families for integration-facing query consumers
- collection/result metadata that explains cursor position, ordering basis,
  and truncation behavior
- counters for collection width, page width, traversal depth, and
  materialization breadth

### Must Preserve

- offset/limit must not masquerade as stable pagination
- query APIs must reveal cardinality and traversal cost honestly
- bounded traversal must stay bounded by declared depth/scope rather than
  drifting into arbitrary graph walks
- rollups and query-time derived fields must remain derived result semantics
  rather than accidental stored authority
- CDC-shaped output must remain query-shaped and projection-honest rather than
  degenerating into raw runtime CDC
- collection execution remains snapshot-stable under active mutation

### Complexity / Proof Obligations

- name collection planning, cursor-advance, bounded traversal, aggregation,
  rollup, and CDC-shaped rendering contracts
- expose exact counters for collection width, page width, traversal depth,
  materialization breadth, aggregate input breadth, and CDC output width
- prove cursor stability and query-shaped CDC parity for the same basis

### Allowed Debt

- admitted fast paths for specific aggregation or rollup families may remain
  `Debt` if the exact fallback path is explicit and parity-proven
- unstable cursor semantics or host-postprocessed derived fields may not ship
  as debt

### Sequencing Notes

This belongs after core planning because collection semantics are the first
place where cost dishonesty and query-shaped output drift become large-scale
product risks.

### Parallelization Notes

Collection execution, rollup/aggregation semantics, and CDC-shaped output
formatting can advance in parallel once Milestone 3 has frozen plan and basis
artifacts.

### Store Dependency

- Core collection semantics are not blocked on `forge-store`.
- Restart-stable cursor durability and persisted page-resume semantics are
  blocked on later `forge-store` durability work and should not be claimed in
  this milestone.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Collection, Cursor, Rollup, And CDC Shape Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- ordered collection queries return stable cursor-advancable pages for one
  snapshot basis
- bounded traversal and eager materialization stay within declared scope
- aggregation, rollup, and query-time derived-field results remain tied to the
  declared truth basis and declared projection rather than host recomputation
- CDC-shaped output matches the same canonical query meaning as ordinary result
  execution for the same query and basis
- collection counters and diagnostics explain why a query touched the breadth
  it touched

## Milestone 5: Live Query Promotion And Incremental Result Maintenance

Status:
Closed on 2026-04-15 for runtime-backed live promotion, query-shaped patching,
replay parity, suppression, and policy-hardening. Historical/policy-composed
live semantics, durable continuation, and store-backed live parity remain
intentionally deferred to later milestones.

### Goal

Make any admitted read query promotable to a live query without changing the
query expression, while keeping live maintenance query-shaped instead of
event-shaped.

### Adversarial Constraint

A live-promoted query that starts from a stable basis and consumes truth
changes incrementally must converge to the same result as repeated fresh
re-execution of the canonical query, regardless of update order, suppression,
or subscription longevity.

### Why This Milestone Exists

The Forge Query vision breaks if reads, subscriptions, and reactive refreshes
are separate products. This milestone makes live promotion a property of query
execution context rather than a parallel API surface.

### Must Ship

- live-promotion context for admitted query families
- query-to-signal lowering for incremental maintenance
- bridge-aware invalidation metadata sufficient to decide query relevance
- query-shaped incremental result patches for:
  - detail reads
  - ordered collections
  - bounded materialized relations
- suppression of irrelevant truth changes before result delivery
- diagnostics explaining why a truth change did or did not affect a live query
- counters for invalidation breadth, patch width, and suppressed updates

### Must Preserve

- live promotion must not change query meaning
- signal scheduling remains owned by `forge-signal`
- patch-to-invalidation routing remains owned by the runtime bridge
- consumers must receive query-shaped changes rather than raw CDC they have to
  reinterpret
- live maintenance must remain snapshot- and basis-honest

### Complexity / Proof Obligations

- name invalidation-relevance, live patch construction, and suppression
  contracts
- expose exact counters for invalidation breadth, live patch width, suppressed
  updates, and full re-execution fallbacks
- prove live + ordering + projection parity for admitted query families, and
  prove that later live + policy masking semantics can compose without
  redefining live query meaning

### Allowed Debt

- some query families may remain non-live-promotable as explicit `Debt` while
  admitted families are fully parity-proven
- raw CDC delivery disguised as query-shaped maintenance may not ship as debt

### Sequencing Notes

This belongs before historical/diff and view-shape semantics because it is the
first proof that query meaning survives time rather than just one-shot reads.

### Parallelization Notes

Can progress in parallel with early Milestone 6 context work once Milestone 3
planning and Milestone 4 collection semantics are stable.

### Store Dependency

- Core live promotion is not blocked on `forge-store`.
- Durable subscription resume across restart is blocked on `forge-store`
  durability for persisted cursors/checkpoints and must not be claimed here.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Live Promotion Convergence And Suppression Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- the same query expression can execute as one-shot or live without semantic
  drift
- irrelevant truth changes are suppressed before query-shaped patch delivery
- query-shaped live patches preserve ordering, membership, and projection
  semantics
- replaying the same canonical truth changes yields the same live query result
  evolution

## Milestone 5.1: Region-Scoped Live Invalidation And Delivery Contracts

### Goal

Make live query narrowing and delivery contracts region-aware, stream-honest,
and explicitly query-shaped instead of broad aspect-only invalidation or ad hoc
CDC transport glue.

### Adversarial Constraint

When a truth change only affects a semantically bounded region or partition of
the query's declared scope, live maintenance must narrow to that region and
emit delivery metadata that preserves the same query meaning without widening
to full-aspect or full-collection recomputation.

### Why This Milestone Exists

Milestone 5 closed the first honest live substrate, but it still leaves one
important production-grade gap: the lower runtimes can already speak in region-
and partition-scoped invalidation and change-stream contracts, while the query
roadmap only guarantees broad live relevance and query-shaped patches.

That is not enough for geometry-grade live delivery, integration-grade stream
contracts, or query surfaces that want to narrow below whole-aspect scope.

### Must Ship

- region- or partition-aware live invalidation metadata for admitted live query
  families
- query-declared locality predicates or materially equivalent plan-owned region
  narrowing surfaces
- change-stream-backed delivery contract lowering for query-shaped CDC/live
  output where the bridge admits it
- diagnostics and counters explaining region matches, region suppressions, and
  stream-contract admission or denial

### Must Preserve

- region-scoped narrowing remains derived from planner-owned query semantics and
  lower-runtime locality contracts rather than host heuristics
- delivery contracts stay query-shaped instead of exposing raw partition events
  as the consumer contract
- durable stream continuation remains deferred until later durable milestones

### Complexity / Proof Obligations

- name region-match, partition-narrowing, and change-stream lowering contracts
- expose exact counters for matched regions, suppressed region changes,
  stream-contract admissions, and region-widening denials
- prove region-scoped live suppression and change-stream-backed delivery remain
  parity-safe with the same canonical live query meaning

### Allowed Debt

- unsupported region families may remain explicit `Debt`
- raw partition or raw CDC events masquerading as query delivery may not ship
  as debt

### Sequencing Notes

This belongs immediately after Milestone 5 because it is live-maintenance
hardening, not a later historical or policy feature.

### Parallelization Notes

Can progress in parallel with early preview-context and planning-hardening work
once the Milestone 5 live substrate is frozen.

### Store Dependency

- Runtime-backed region narrowing and stream-contract semantics are not blocked
  on `forge-store`.
- Durable stream resume and persisted checkpoints remain deferred to
  `Milestone 11`.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- region-scoped live invalidation stays narrower than broad aspect invalidation
  when lower-runtime locality contracts admit that narrowing
- query-shaped delivery contracts can lower into formal stream contracts
  without changing query meaning
- unsupported region or stream-contract combinations fail typed and early

## Milestone 5.2: Preview Session Query Contexts And Branch Workflow Foundations

Status:
Closed on 2026-04-16 for runtime-backed preview-session query contexts,
promotion-parity comparison, preview-live composition, and workflow-foundation
artifacts. Durable preview replay reload, persisted workflow artifacts, and
store-backed preview parity remain intentionally deferred.

### Goal

Make speculation and preview sessions first-class query contexts so branch-
native preview workflows stay inside the query framework instead of devolving
into raw bridge orchestration.

### Adversarial Constraint

A query bound to a preview or speculative session must preserve explicit basis,
preview-lifecycle identity, and preview-versus-promoted comparison semantics
without ambient host glue deciding what session it really targeted or how the
result should be interpreted.

### Why This Milestone Exists

The vision already wants AI, workflow, and geometry users to operate against
branch-local truth and speculative branches. The runtime bridge already has a
real preview-session lifecycle. If the query roadmap does not expose that as a
native basis context, developers will drop out of query-land for one of the
most important branch-native workflows in the product.

### Must Ship

- query contexts that can bind to `BridgePreviewSession` or materially
  equivalent preview-session artifacts
- explicit preview-session basis metadata and preview-lifecycle metadata on
  query plans/results
- distinction between read-only preview evaluation and promotable preview
  evaluation
- query-native comparison surfaces for preview result versus promoted result
- preview-live admission, maintenance, drift denial, and explicit rebind over
  the corresponding admitted Milestone 5 and 5.1 live families
- branch-workflow foundation artifacts that later mutation/merge milestones can
  extend without redefining preview semantics

### Must Preserve

- preview-session lifecycle authority remains owned by the runtime bridge
- preview contexts do not become host-local branch aliases
- preview queries preserve ordinary canonical query meaning apart from the
  explicitly declared preview basis
- preview-live may not silently fall back to authoritative live truth

### Complexity / Proof Obligations

- name preview-basis resolution, preview-lifecycle identity, and promoted-
  result comparison contracts
- name preview-live admission, drift, and explicit-rebind contracts
- expose exact counters for preview-session admissions, preview-basis
  resolutions, preview/promotion comparison runs, preview-live admissions,
  preview-live drift denials, preview-live explicit rebinds, and invalid
  preview-context denials
- prove preview-session query contexts remain parity-safe with the same
  canonical query shape and declared preview basis

### Allowed Debt

- unsupported preview families may remain explicit `Debt`
- ambient host orchestration that silently selects or rewrites the preview
  basis may not ship as debt

### Sequencing Notes

This belongs before general branch/history expansion because preview workflows
are a special but load-bearing form of basis identity that later branch and
merge work must inherit.

### Parallelization Notes

Can progress in parallel with region-scoped live hardening and planning
hardening once Milestone 5 proof-bearing live artifacts are stable.

### Store Dependency

- Runtime-backed preview-session query contexts are not blocked on
  `forge-store`.
- Durable preview replay and persisted branch-workflow artifacts remain later
  durable work.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- preview-session-bound queries preserve explicit basis and lifecycle identity
- preview-versus-promoted comparison remains query-native and typed
- preview-live remains basis-explicit and either maintained, denied, or
  explicitly rebound without silent retargeting
- unsupported preview-session query combinations fail before semantic drift

## Milestone 5.3: Frontier-Aware Planning And Deterministic Parallel Admission

### Goal

Make planning consume signal-frontier cost posture and deterministic parallel-
admission knowledge instead of planning in isolation and hoping the executor
figures it out later.

### Adversarial Constraint

Planning decisions for bulk queries, live maintenance, and multi-query bundles
must preserve the same canonical query meaning whether they execute serially or
in parallel, while breadth and parallel admission decisions remain explicit
rather than rediscovered by runtime heuristics.

### Why This Milestone Exists

Milestone 3 established proof-bearing plans, but not yet the stronger planning
story that a platform framework should have when `forge-signal` already owns
frontier and deterministic parallel-admission machinery. Query should consume
that structural knowledge at plan time instead of acting as if every cost
decision is local and serial by default.

### Must Ship

- frontier-aware planning metadata for admitted query families
- deterministic parallel-admission metadata on planned execution routes for
  admitted bulk/live families
- diagnostics that explain why a route admitted parallel execution or fell back
  to serial execution
- counters for predicted breadth, realized breadth, parallel admissions, and
  serial fallbacks

### Must Preserve

- `forge-signal` remains authoritative for frontier and parallel-admission
  semantics
- the executor consumes lowered admission decisions instead of speculating
  about parallel safety at runtime
- serial and parallel lanes must preserve identical canonical query meaning

### Complexity / Proof Obligations

- name frontier-breadth, parallel-admission, and serial-fallback contracts
- expose exact counters for frontier lookups, predicted breadth, realized
  breadth, admitted parallel batches, and serial fallback decisions
- prove deterministic serial/parallel parity for admitted planned families

### Allowed Debt

- unsupported query families may remain serial-only as explicit `Debt`
- executor-side speculative parallel admission may not ship as debt

### Sequencing Notes

This belongs after Milestone 5 because it hardens planning using already-frozen
live and query artifacts without reopening Milestone 3 itself.

### Parallelization Notes

Can progress in parallel with preview-session and correspondence/historical
hardening once the required lower-runtime planning inputs are stable.

### Store Dependency

This milestone is not blocked on `forge-store`.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- frontier-aware planning decisions are explicit and digest-bearing
- admitted serial and parallel lanes remain semantically identical
- unsupported parallel families fail closed or remain explicit debt

## Milestone 5.4: Structural Correspondence And Historical Evaluation Contracts

### Goal

Strengthen the branch/history/identity story so structural correspondence and
historical materialization-path honesty become explicit query artifacts instead
of implied bridge details.

### Adversarial Constraint

Correspondence and historical queries must stay explicit about whether they
were resolved through lineage, structural fingerprinting, retained snapshots,
delta replay, or full reconstruction, without silently collapsing ambiguity or
materialization-path differences into one vague "comparison result."

### Why This Milestone Exists

The vision already mentions structural fingerprints and rich historical reads,
but the current roadmap reads too lineage-centric and too generic about
historical basis. This milestone closes that precision gap before later branch,
history, and workflow milestones build more surface area on top.

### Must Ship

- structural-fingerprint-based correspondence as a first-class query artifact
  beside lineage-based correspondence
- query result metadata describing historical materialization path for admitted
  historical reads
- explicit compatibility/admission contracts for historical evaluation where
  the lower runtimes cannot serve a request honestly
- diagnostics for structural ambiguity, lineage/structural disagreement, and
  unsupported historical materialization paths

### Must Preserve

- lineage remains authoritative continuity; structural correspondence remains
  advisory unless explicitly promoted by lower-truth semantics
- historical evaluation authority remains in lower runtimes, not host caches
- ambiguity and materialization-path differences remain explicit in results

### Complexity / Proof Obligations

- name structural-correspondence, historical-materialization-path, and
  compatibility-admission contracts
- expose exact counters for structural candidates considered, ambiguity
  denials, historical-path admissions, and path-compatibility denials
- prove correspondence and historical artifacts remain typed, replay-safe, and
  ambiguity-honest

### Allowed Debt

- unsupported structural families or historical materialization paths may
  remain explicit `Debt`
- silent collapse of ambiguity or hidden materialization-path substitution may
  not ship as debt

### Sequencing Notes

This belongs before the broader current Milestone 6 and Milestone 7 work
because those milestones should build on explicit correspondence and historical
contracts rather than implying them.

### Parallelization Notes

Can progress in parallel with planning hardening and early branch-workflow
surfaces once the required lower-runtime artifacts are stable.

### Store Dependency

- Runtime-backed structural correspondence and admitted historical-path
  metadata are not blocked on `forge-store`.
- Durable historical restore remains later store-backed work.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- structural correspondence is explicit and distinct from lineage continuity
- historical query results expose materialization-path meaning where admitted
- unsupported or ambiguous cases fail typed and early

## Milestone 5.5: Query-Orchestrated Mutation, Merge, And Writeback Declarations

### Goal

Make `forge-query` a real platform workflow surface for domain developers by
letting query-authored mutation, merge, and writeback declarations lower into
relational and bridge authorities without forcing developers to drop into raw
lower-crate APIs for common branch-native workflows.

### Adversarial Constraint

Mutation intents, merge intents, conflict preview, post-merge inspection, and
query-triggered writeback declarations must all preserve explicit authority
boundaries, conflict meaning, and replay/delivery honesty without turning
`forge-query` into a second mutation engine or hiding branch-workflow truth
behind host glue.

### Why This Milestone Exists

If `forge-query` is the daily-driver framework surface, it cannot stop at reads
and live subscriptions. Domain developers need to stay inside the query facade
for context resolution, preview, merge inspection, commit lowering, and
writeback-trigger declaration. Otherwise the platform fractures precisely at
the workflows branch-native products use most.

### Must Ship

- query-authored mutation intents that lower into relational commit strategy
  requests
- query-authored branch-workflow declarations for at least:
  - preview / compare
  - conflict inspection
  - merge intent
  - post-merge result inspection
- query-triggered writeback declarations that lower into bridge writeback
  declarations where admitted
- diagnostics and counters for mutation-intent lowering, merge admission,
  conflict classification, and writeback admission or denial

### Must Preserve

- `forge-relational` remains authoritative for commit strategy, merge
  semantics, and mutation truth
- the runtime bridge remains authoritative for preview-session lifecycle,
  writeback safety, idempotence, causality, and replay artifacts
- `forge-query` owns declaration, lowering, orchestration surface, and result
  shaping, not a second mutation engine

### Complexity / Proof Obligations

- name mutation-intent lowering, merge-intent lowering, conflict-preview, and
  writeback-declaration contracts
- expose exact counters for lowered mutation intents, merge previews, merge
  denials, writeback admissions, and writeback denials
- prove branch-native workflow declarations lower into lower-crate authorities
  without semantic drift or hidden host orchestration

### Allowed Debt

- unsupported mutation or writeback families may remain explicit `Debt`
- host-local branch workflow glue that bypasses canonical lowering may not ship
  as debt for any admitted workflow family

### Sequencing Notes

This belongs before later history/policy/store milestones because it corrects
the biggest platform-level omission in the current roadmap: query as the
application-facing workflow framework, not just the read surface.

### Parallelization Notes

Can progress in parallel with unified-facade/config work once the authority
boundaries for commit, merge, preview, and writeback lowering are frozen.

### Store Dependency

- Runtime-backed mutation/merge/writeback lowering is not blocked on
  `forge-store`.
- Durable workflow continuation and persisted workflow artifacts remain later
  work.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- admitted mutation and merge workflow declarations lower into relational
  authorities without semantic drift
- admitted writeback declarations lower into bridge authorities without hiding
  causality or safety semantics
- unsupported workflow families fail typed and early

## Milestone 5.6: Unified Application Facade And Unified Runtime Configuration

### Goal

Make `forge-query` the explicit daily-driver facade and configuration surface
for ordinary domain/application code without erasing lower-crate authority
boundaries or collapsing configuration into a bag.

### Adversarial Constraint

A unified facade and unified runtime configuration must let developers use the
platform through one coherent surface while preserving subsystem ownership,
typed capability boundaries, and structurally sectioned configuration rather
than flattening the stack into ambiguous pass-through glue.

### Why This Milestone Exists

The product story for `forge-query` only fully lands when developers can treat
it as the main framework import instead of shopping among `forge-relational`,
`forge-signal`, and the runtime bridge. But that facade must stay authority-
preserving and architecture-shaped, or it just becomes a bag of convenience
APIs and config fields.

### Must Ship

- one explicit application-facing facade posture for `forge-query`
- pass-through or composed public surfaces for admitted lower-runtime
  capabilities that application developers should access through query
- unified `ForgeQueryConfig` or materially equivalent configuration surface
  sectioned by subsystem ownership
- capability advertisement and diagnostics explaining which composed surfaces
  are admitted, deferred, or unsupported

### Must Preserve

- the facade is unified for developers but not semantics-erasing for the
  underlying runtimes
- configuration must mirror subsystem boundaries rather than becoming a flat
  bag
- unsupported composed capabilities remain explicit rather than implied by one
  broad "platform config" type

### Complexity / Proof Obligations

- name facade composition, capability advertisement, and configuration-section
  resolution contracts
- expose exact counters for capability lookups, configuration-section
  resolutions, and unsupported-composition denials
- prove unified facade and unified configuration surfaces preserve lower-crate
  authority and admitted capability boundaries

### Allowed Debt

- unsupported composed surfaces may remain explicit `Debt`
- flat bag-shaped unified configuration and semantics-erasing facade shortcuts
  may not ship as debt

### Sequencing Notes

This belongs after the workflow-composition milestone because the facade and
config surfaces should compose real admitted platform capabilities, not merely
promise them.

### Parallelization Notes

Can progress in parallel with the earliest branch/history/policy preparation as
long as unsupported capabilities remain explicitly gated.

### Store Dependency

- Core unified facade and unified configuration work is not blocked on
  `forge-store`.
- Any config fields that claim durable resume, store-backed parity, or durable
  artifact support remain gated by the later store-backed milestones.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- application-facing facade composition is explicit and authority-preserving
- unified runtime configuration stays sectioned by subsystem responsibility
- capability advertisement and support metadata remain in sync with admitted
  composed surfaces

## Milestone 6: Branch-Scoped, Historical, And Diff Query Contexts

### Goal

Make branch targeting, time-travel, and comparison queries first-class context
and query capabilities rather than separate host APIs.

### Adversarial Constraint

Current-state reads, branch-scoped reads, historical reads, and diff queries
over the same canonical query must preserve identical query meaning apart from
their declared basis difference, with no hidden basis substitution, host cache
repair, or result-shape drift.

### Why This Milestone Exists

Forge systems are branch-native and history-native. If `forge-query` cannot
express "read this branch," "read this commit," and "show me the difference
between these two truth bases" through the same query model, then developers
will be pushed back into ad hoc runtime calls that fracture the stack.

### Must Ship

- branch-scoped query contexts
- historical query contexts targeting branch heads, commits, or admitted
  snapshots
- compatibility with preview-session-derived bases and admitted historical
  materialization-path metadata established by Milestones 5.2 and 5.4
- diff-query expression families for comparing two declared truth bases
- result metadata that names the basis used for every branch/historical/diff
  query
- diagnostics for unsupported historical bases, ambiguous comparison bases, and
  scope mismatch
- parity surfaces between present-state reads and historical reads for the same
  declared query shape

### Must Preserve

- branch and history meaning remain owned by `forge-relational` and
  `forge-store`
- historical queries do not mutate truth or fabricate history through host
  caches
- preview-session basis identity and historical materialization-path identity
  remain explicit rather than ambient host context
- diff queries return query-shaped comparison artifacts rather than raw storage
  deltas
- basis identity remains explicit end-to-end

### Complexity / Proof Obligations

- name branch-basis resolution, historical-basis resolution, and diff-shaping
  contracts
- expose exact counters for historical basis lookups, diff input breadth,
  comparison-scope width, and unsupported-basis denials
- prove historical + diff + result-shape parity for the same canonical query

### Allowed Debt

- durable store-backed historical execution may remain `Debt` until
  `forge-store` can support it honestly
- hidden basis substitution or history reconstruction through ambient host
  caches may not ship as debt

### Sequencing Notes

This belongs after Milestones 5.2 and 5.4 because basis identity across time,
preview lifecycle, and historical materialization path are the next major
places where query meaning can fracture.

### Parallelization Notes

Branch/head context work can begin earlier, but full historical/diff closure
should follow stable Milestone 3 planning plus the preview-basis and
historical-contract hardening from Milestones 5.2 and 5.4.

### Store Dependency

- Branch-head reads and any retained-history reads already exposed by the
  runtime are not blocked on `forge-store`.
- Full completion for durable point-in-time restore, snapshot-targeted
  execution, restart-stable historical parity, and store-backed diff execution
  is blocked on `forge-store` milestones for snapshots, delta layering, and
  replication-safe artifact identity.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Historical / Diff / Basis Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- the same declared query shape can run against current and historical truth
  bases without changing its structural meaning
- diff queries produce structured, typed change sets aligned to the declared
  projection and scope
- where store-backed historical execution exists, it matches runtime-backed
  historical truth for the same basis

## Milestone 7: Lineage, Correspondence, And Identity-Evolution Queries

### Goal

Make identity evolution queryable without exposing raw lineage internals or
forcing consumers to hand-assemble history walks.

### Adversarial Constraint

Identity-evolving queries must preserve explicit continuity, ambiguity, and
rejection semantics across branch-local, historical, and comparison reads
without silently promoting correspondence guesses into authoritative identity.

### Why This Milestone Exists

Branch and history alone are not enough for truth systems where entities can be
replaced, split, merged, or corresponded across versions. Query consumers need
first-class lineage-aware reads if they are going to compare, inspect, and
subscribe to evolving truth honestly.

### Must Ship

- lineage traversal query expressions
- correspondence-aware query expressions for cross-branch or cross-version
  comparison where the runtime admits them, including structural-fingerprint-
  backed correspondence where admitted
- identity-evolution result shapes that distinguish:
  - current entity truth
  - historical antecedents
  - replacements/splits
  - ambiguous or rejected correspondences
- diagnostics for unsupported lineage traversals and ambiguous correspondence
- query metadata that exposes the lineage/correspondence basis used for a
  result

### Must Preserve

- lineage semantics remain owned by `forge-relational`
- correspondence must not silently become authoritative identity
- structural correspondence must remain explicit about when it is advisory,
  ambiguous, or rejected
- ambiguous identity evolution fails or reports ambiguity explicitly
- lineage-aware reads remain query-shaped and replay-honest

### Complexity / Proof Obligations

- name lineage traversal and correspondence-resolution contracts
- expose exact counters for lineage steps traversed, correspondence candidates
  considered, ambiguous correspondence denials, and fallback identity breaks
- prove lineage + correspondence + branch-scoped comparison parity for
  admitted identity-evolution cases

### Allowed Debt

- unsupported lineage or correspondence classes may remain explicit `Debt`
  while admitted classes are fully typed and parity-proven
- silent continuity through ambiguous correspondence may not ship as debt

### Sequencing Notes

This belongs after Milestones 5.4 and 6 because identity evolution only
becomes meaningful once basis-aware reads and explicit correspondence
contracts already exist.

### Parallelization Notes

Can progress in parallel with early view-shape and scope work once Milestone 6
has frozen branch/history basis artifacts and Milestone 5.4 has frozen the
explicit correspondence vocabulary.

### Store Dependency

- Core lineage-aware query semantics are not blocked on `forge-store`.
- Restart-stable lineage/correspondence parity across persisted history is
  blocked on `forge-store` durable lineage artifact support and should be
  treated as completion debt until store lands that support.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Lineage And Correspondence Query Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- lineage traversal yields typed, explainable results across admitted identity
  evolution classes
- correspondence-aware comparison stays explicit about ambiguity and rejection
- lineage-aware reads over replayed or restored truth match original query
  meaning for the same lineage basis

## Milestone 8: Scopes, Templates, And View Shapes

### Goal

Turn query composition into a reusable product surface rather than leaving every
host to rebuild domain vocabulary and presentation intent locally.

### Adversarial Constraint

Reusable scopes, templates, and admitted view shapes must expand
to the same canonical query meaning as direct construction, and view-shape
semantics must affect planning and live maintenance structurally rather than
surviving only as cosmetic type tags.

### Why This Milestone Exists

Typed query primitives are necessary, but not sufficient. Developers need
schema-validated reuse surfaces such as named scopes, parameterized templates,
and intent-driven view shapes if Forge Query is going to be the normal way
people ask for truth.

### Must Ship

#### Surface Primitives

- composable named scopes as first-class query fragments
- query templates with typed parameter slots
- intent-driven view shapes for at least:
  - table/detail
  - grouped/kanban-style collections
  - timeline/chart-style results where admitted
  - inspector-style detail projection with live aspect focus
- diagnostics for invalid parameter binding, incompatible scope composition,
  and unsupported view-shape/query combinations
- query-shape metadata that higher layers can use for delivery and live-patch
  formatting

#### Semantic Guarantees

- scopes must preserve the same canonical query meaning as their expanded form
- templates must bind parameters without introducing host-local semantic drift
- each admitted view shape must affect planning, invalidation narrowing,
  delivery formatting, and live patch semantics explicitly rather than acting
  as display-only sugar
- inspector/detail view behavior must preserve narrow aspect-focused live
  projection instead of widening to whole-entity reads

#### Proof Obligations

- scope-composed queries and directly-authored queries must normalize to the
  same canonical artifact
- at least one grouped or temporal view must prove view-shape-specific live
  patch semantics rather than only type-surface existence
- inspector/detail live projection must prove aspect-focused invalidation
  narrowing under change

#### Store-Gated Completion Debt

- durable saved-query persistence, portability, and restart-stable workspace
  artifacts are postponed to `Milestone 11`

### Must Preserve

- scopes and templates remain typed query artifacts rather than macro-like
  string substitution
- view shapes do not become presentation-owned shadow query languages
- host layers consume query-shape metadata instead of redefining the query

### Complexity / Proof Obligations

- name scope expansion, template binding, and view-shape lowering contracts
- expose exact counters for scope fragments expanded, template parameters
  bound, and view-shape-specific patch events
- prove view-shape-specific live semantics for at least table/detail plus one
  grouped or temporal view

### Allowed Debt

- absent view-shape families may remain explicit `Debt`
- any shipped view shape that lacks planning, invalidation, delivery, and live
  semantics may not ship as debt
- durable saved-query semantics are intentionally deferred to `Milestone 11`

### Sequencing Notes

This belongs after lineage/history and the workflow/facade insertions because
composition and presentation intent must sit on top of already-honest query
meaning and platform workflow surfaces rather than inventing either.

### Parallelization Notes

Scopes/templates and view-shape semantics can progress in parallel once
Milestones 4 through 6 plus 5.5 and 5.6 have stabilized collection, live,
basis, and platform-facade behavior.

### Store Dependency

- Scopes, templates, and view-shape semantics are not blocked on `forge-store`.
- Durable saved-query persistence, portability, and restart-stable workspace
  artifacts are blocked on `forge-store`; until then, any saved-query-like
  surfaces may exist only as ephemeral or host-local artifacts and must not be
  marketed as complete.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Scope / Template / View-Shape Semantic Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- scopes and templates compose into the same canonical query artifacts as
  equivalent direct construction
- admitted view shapes affect planning, invalidation narrowing, delivery
  formatting, and live patch semantics rather than only surfacing as type tags
- inspector/detail live projection remains aspect-focused under change

## Milestone 9: Policy-Aware Narrowing, Tenant Scope, And Delivery Contracts

Status:
Closed on 2026-04-21 for runtime-backed policy-aware narrowing, tenant
truth/schema basis admission, relationship-proof admission/denial, policy-aware
execution seam lowering, live admission, delivery contracts, and certification.
Store-backed execution parity, durable policy cursors, durable artifact reload,
durable delivery metadata reload, restart-stable subscription metadata, and
durable tenant/query artifact portability remain intentionally deferred to
Forge Store and later milestones. See
[milestone-9-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/milestone-9-closeout.md).

### Goal

Make policy, masking, and multi-tenant narrowing structural query concerns
rather than after-the-fact filtering glued on by hosts.

### Adversarial Constraint

Policy masks, tenant branch narrowing, tenant-scoped schema variation, and
relationship-proof denials must never leak masked truth, alter query meaning
across one-shot/live/historical modes, or depend on post-read redaction.

### Why This Milestone Exists

The query layer is the last place where over-broad reads can still be stopped
before truth is touched. If policy-aware narrowing and tenant scoping are not
structural here, then higher layers will either over-read and redact later or
invent incompatible policy behavior across read, live, and historical surfaces.

### Must Ship

#### Surface Primitives

- aspect-level policy masking in query planning
- branch-level access scoping in query validation/execution context
- tenant-scoped branch narrowing where the platform resolves tenant truth by
  branch
- tenant-scoped schema awareness so query validation and projection use the
  tenant's active schema rather than assuming one global schema
- graph-native relationship-proof predicate/query families for admitted
  relationship-calculus style access or legality proofs
- delivery-shape metadata for server-facing transport layers
- policy composition rules for admitted mutation, merge, writeback, and
  streamed-delivery declarations exposed through the query framework
- diagnostics for masked aspects, denied branches, ambiguous tenant context,
  tenant-schema mismatch, relationship-proof denial, and policy/query
  incompatibility

#### Semantic Guarantees

- policy masking must happen before execution so masked aspects are never read
- tenant scoping must narrow both truth basis and schema basis explicitly
- relationship-proof queries must remain typed query semantics rather than
  host-local authorization callbacks
- delivery-shape metadata must preserve the exact masked/projected result
  meaning seen by the caller
- policy and tenant context must compose with admitted mutation, merge,
  writeback, and stream-contract declarations without post-read repair
- one-shot, live, historical, and saved/scope-composed queries must honor the
  same policy and tenant basis

#### Proof Obligations

- policy-masked and unmasked variants of the same query must prove that masked
  aspects never enter the plan or live-maintenance path
- tenant-specific schema variation must prove validation and result-shape
  parity across at least two divergent tenant schema states
- relationship-proof queries must prove explicit denial and non-leakage when
  the proof chain is broken
- delivery-shape metadata must stay parity-safe across one-shot, live, and
  historical execution for the same policy basis

#### Store-Gated Completion Debt

- durable delivery cursors, restart-stable subscription metadata, and persisted
  tenant/query artifacts remain incomplete until `forge-store` lands the
  required durable support

### Must Preserve

- policy authority remains owned by schema/platform layers rather than by query
  host code
- masked aspects must not be read and then discarded later
- tenant scoping must narrow truth basis explicitly rather than through ambient
  hidden filters
- delivery metadata remains derived from the canonical query and policy basis

### Complexity / Proof Obligations

- name policy masking, tenant basis resolution, tenant-schema validation, and
  delivery-shape derivation contracts
- expose exact counters for masked projection entries, tenant basis resolutions,
  tenant-schema validation branches, relationship-proof denials, and delivery
  metadata derivations
- prove tenant schema variation + validation + delivery-shape parity, and
  policy masking parity across one-shot, live, and historical execution

### Allowed Debt

- durable tenant/query artifact persistence may remain `Debt` until
  `forge-store` supports it
- policy masking by post-read redaction or host-local authorization callbacks
  may not ship as debt

### Sequencing Notes

This belongs after scopes, view shapes, and the workflow/facade insertions
because policy and tenant narrowing must govern the full composed query and
platform surface, not just primitive query forms.

### Parallelization Notes

Relationship-proof and tenant-schema work can progress in parallel, but final
closure should wait until scopes, saved queries, delivery shapes, and admitted
workflow/facade surfaces are stable enough to prove parity across them.

### Store Dependency

- Core policy-aware narrowing is not blocked on `forge-store`.
- Store-backed policy execution parity is blocked on `forge-store` and remains
  Milestone 10 scope.
- Durable delivery cursors, restart-stable subscription metadata, and
  persisted tenant/query artifacts are blocked on `forge-store` and remain
  Milestone 11 scope.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Policy, Tenant Schema, And Relationship-Proof Boundary Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- masked aspects never appear in the execution plan or result
- denied branch or tenant access fails before storage/runtime reads execute
- tenant-scoped schema variation changes validation/projection behavior
  explicitly and deterministically
- broken relationship-proof chains fail closed without leaking masked or
  unauthorized truth
- the same policy basis produces the same query narrowing for one-shot, live,
  and historical execution

## Milestone 9.1: Query-Owned Subscription Declaration Families, Lowering, And Admission

### Goal

Make subscriptions first-class query artifacts by lowering admitted live query
meaning into explicit subscription declaration families and bridge-facing
subscription plans rather than treating long-lived observation as host-side
glue around live queries.

### Adversarial Constraint

The same canonical query, live family, policy basis, tenant basis, and
view-shape intent must lower into the same subscription declaration family and
bridge subscription plan regardless of whether the caller authored it
directly, through a scope/template/saved artifact, or through a runtime facade
helper, with no path allowed to infer subscription meaning from ambient host
observer state or a fake default subscription kind.

### Why This Milestone Exists

Milestone 9 froze policy-aware narrowing, tenant/schema basis resolution,
relationship-proof admission, and caller-visible delivery shape. Those
artifacts now define what a live query is allowed to mean.

What still does not exist is one query-owned answer to:

- what subscription identity corresponds to this canonical live query
- what basis does that subscription bind to
- what bridge subscription contract should it lower into
- what makes two live requests the same subscription versus different
  subscriptions
- why was a subscription admitted or denied

Without this milestone, Forge Query can continue to speak about "live" while
still relying on hidden runtime conventions for the actual subscription
mechanism.

### Must Ship

- canonical query-owned subscription declaration-family artifacts derived from
  admitted live query meaning
- subscription family selection, identity, and equivalence surfaces distinct
  from raw query identity, delivery shape, and bridge stream identity
- lowering from live query plans into bridge-native subscription declarations
  and admission requests
- basis-bound subscription declaration for current, branch-local, historical,
  and admitted preview contexts where the live family supports them
- policy-aware, tenant-aware, and relationship-proof-aware subscription
  admission
- view-shape-aware subscription shaping for admitted detail, collection,
  grouped, and inspector-style live families
- explicit lowering from query declaration families into admitted bridge
  declaration families and admitted `forge-signal` observation and delivery
  strategies
- diagnostics that explain subscription declaration, lowering, basis binding,
  and denial

### Must Preserve

- `forge-query` remains the owner of query semantics and result shaping, not
  the owner of bridge subscription protocol semantics
- subscription lowering must consume the same canonical policy/tenant/basis
  artifacts as one-shot and historical execution
- unsupported subscription combinations fail typed and early instead of
  widening into raw CDC or host observer callbacks
- equivalent live requests normalize to one subscription-family meaning before
  bridge activation
- `forge-query` does not invent its own observer semantics; it chooses among
  admitted family lowerings that already map into bridge and `forge-signal`
  strategy space

### Complexity / Proof Obligations

- name subscription declaration, lowering, basis binding, and admission
  contracts
- expose exact counters for admitted subscription declarations, denied
  declarations, grouped/detail lowering variants, basis-bound declarations, and
  bridge-lowering fallback count
- prove that equivalent live query inputs lower to equivalent
  subscription-family artifacts and bridge declarations

### Allowed Debt

- durable subscription artifact persistence and reload may remain `Debt` until
  `forge-store` supports them
- host-local subscription assembly, ambient observer inference, or CDC-shaped
  fallback may not ship as debt

### Sequencing Notes

This belongs immediately after Milestone 9 because policy narrowing, tenant
basis, and caller-visible delivery shape must be frozen before Query can lower
subscriptions honestly.

### Parallelization Notes

Bridge-facing lowering and query-owned declaration identity can progress in
parallel, but final closure should wait until both agree on canonical
subscription equivalence and denial semantics.

### Store Dependency

- Core runtime-backed subscription declaration and lowering are not blocked on
  `forge-store`.
- Store-backed restart parity remains Milestone 10 scope.
- Durable subscription artifacts and reload semantics remain Milestone 11
  scope.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Query Subscription Declaration And Lowering Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- equivalent live query inputs lower to the same canonical
  subscription-family declaration and bridge-facing subscription plan
- policy, tenant, basis, and view-shape differences that change live meaning
  also change subscription meaning explicitly
- unsupported or ambiguous subscription bindings fail before activation
- no admitted path interprets raw CDC or one fixed baked-in subscription kind
  as a substitute for query-shaped subscription intent

## Milestone 9.2: Subscription-Family-Backed Live Delivery, Sharing, And Lifecycle Parity

### Goal

Make active query subscriptions first-class runtime objects with explicit
lifecycle, sharing, continuation, and preview isolation so live maintenance is
honestly lowered through admitted subscription families rather than merely
"live-promoted" in name.

### Adversarial Constraint

One-shot reads, live-promoted queries, shared-equivalent subscriptions,
continuation after identity evolution, and preview-scoped subscriptions must
all preserve the same canonical query meaning and caller-visible query-shaped
delivery for the admitted live family, without consumer pacing, fanout shape,
or preview churn redefining what the query means.

### Why This Milestone Exists

Milestone 9.1 can declare and lower subscriptions, but declaration alone is
not enough for the product surface we actually want.

Forge Query still needs to own:

- active subscription handles and lifecycle
- sharing and deduplication for equivalent query subscriptions
- query-shaped delivery over active subscriptions
- continuation and remap semantics when identity evolves
- preview-scoped subscription behavior and discard/promotion boundaries

Without this milestone, Query would have subscription declarations on paper but
still no honest runtime story for active long-lived subscriptions.

### Must Ship

- query subscription handles and active lifecycle surfaces
- query-shaped delivery contracts for subscription-backed maintenance
- equivalent-subscription sharing and multi-consumer fanout semantics
- continuation and remap handling for admitted identity-evolution scenarios
- explicit preview-scoped query subscription behavior, discard semantics, and
  promotion-boundary interaction
- grouped-baseline, derived-view, and admitted view-shape integration for
  subscription-backed live maintenance
- active family-aware delivery behavior that preserves which admitted
  subscription family and `forge-signal` strategy were selected
- diagnostics and counters for fanout, continuation, preview discard, and
  subscription delivery behavior

### Must Preserve

- one-shot and subscription-backed lanes preserve the same canonical query
  meaning apart from the explicitly declared live lifecycle
- sharing does not redefine query meaning or delivery semantics
- preview subscriptions remain isolated from authoritative subscriptions unless
  promoted through an explicit authority boundary
- continuation remains explicit under lineage, correspondence, and branch
  divergence rather than being inferred from host cache coincidence
- active lifecycle semantics remain family-aware rather than collapsing all
  admitted live families into one generic runtime lane

### Complexity / Proof Obligations

- name subscription lifecycle, sharing, continuation, preview isolation, and
  delivery contracts
- expose exact counters for active subscriptions, shared fanout width,
  continuation remaps, preview discard residue checks, and delivery batches or
  patch groups
- prove parity across one-shot, subscription-backed live, shared-consumer, and
  preview/discard lanes for admitted families
- prove at least two admitted subscription families remain parity-safe under
  sharing, continuation, and query-shaped delivery

### Allowed Debt

- durable continuation checkpoints and restart-stable subscription metadata may
  remain `Debt` until `forge-store` supports them
- consumer-local caches, hidden fanout heuristics, or preview residue after
  discard may not ship as debt

### Sequencing Notes

This belongs after Milestone 9.1 because Query needs canonical subscription
declarations before it can honestly own active subscription lifecycle and
sharing.

### Parallelization Notes

Lifecycle and sharing work can progress in parallel with preview isolation and
continuation work, but final closure should wait until grouped/derived
delivery, preview discard, and continuation all prove parity for the same
admitted subscription families.

### Store Dependency

- Core runtime-backed subscription lifecycle, sharing, continuation, and
  preview isolation are not blocked on `forge-store`.
- Store-backed restart and snapshot-plus-tail continuation remain Milestone 10
  scope.
- Durable subscription checkpoints, reload, and restart-stable metadata remain
  Milestone 11 scope.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Subscription Lifecycle Sharing And Preview Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- active subscription delivery remains query-shaped and parity-safe with
  one-shot meaning for the same admitted live family
- equivalent subscriptions can share one active maintenance path without
  changing meaning
- continuation across admitted identity-evolution scenarios remains explicit
  and typed
- discarded preview subscriptions leave no authoritative query residue

## Milestone 9.3: Subscription Family Diagnostics, Bridge Parity, And Runtime Certification

### Goal

Make Query's automatic subscription-family selection path bridge-honest,
diagnosable, and certified so first-class subscriptions are not just
implemented, but explainable as an authority-preserving lowering into the
bridge and signal layers.

### Adversarial Constraint

For every admitted query subscription family, Query's automatic subscription
path must be explainable as the same semantic request a careful manual host
could assemble through canonical query artifacts plus bridge subscription
contracts plus admitted `forge-signal` observation and delivery strategies,
with diagnostics and certification sufficient to prove that Query is not
inventing hidden semantics above the bridge.

### Why This Milestone Exists

Milestones 9.1 and 9.2 can make subscriptions real, but they still leave one
dangerous gap:

- Query could automate subscriptions in a way that is impossible to explain in
  bridge terms
- diagnostics could stop at "live handle exists" instead of exposing lowered
  subscription meaning
- runtime support claims could outrun actual certified subscription families

This milestone exists to close that honesty gap before store-backed and durable
milestones build on top of it.

### Must Ship

- query-owned subscription inspector and diagnostics artifacts
- explicit bridge-parity explanation surfaces showing how query subscription
  declarations lower into bridge subscription declarations and admitted
  lifecycle behavior
- support and admission reporting for subscription-capable query families
- runtime certification rows for declaration, lifecycle, sharing, continuation,
  preview isolation, and bridge parity
- canonical subscription bundle artifacts sufficient for offline diagnosis of
  admitted runtime-backed subscription paths
- diagnostics that report which admitted query family, bridge family, and
  `forge-signal` strategy lowering were selected

### Must Preserve

- bridge remains the authority for bridge subscription protocol semantics
- signal remains the authority for observation execution and scheduling
- diagnostics richness may change retained detail but not subscription meaning
- unsupported subscription families remain explicit non-admitted surfaces
  rather than "experimental magic"
- certification must prove family selection and lowering honesty, not merely
  one generic subscription lifecycle

### Complexity / Proof Obligations

- name bridge-parity explanation, subscription diagnostics, support reporting,
  and runtime certification contracts
- expose exact counters for bridge-parity comparisons, unsupported family
  denials, diagnostics bundle emissions, and certified subscription family
  coverage
- prove that every admitted automatic subscription family has a bridge-facing
  explanation and at least one hostile certification path
- prove that admitted family variation is visible and mechanically distinct in
  diagnostics and certification artifacts

### Allowed Debt

- durable subscription artifact replay and store-backed restart certification
  may remain `Debt` until Milestones 10 and 11 close
- undocumented hidden lowering paths or uncertified supported subscription
  families may not ship as debt

### Sequencing Notes

This belongs after Milestone 9.2 because Query needs actual subscription
lifecycle behavior before it can certify bridge parity and diagnostic
sufficiency honestly.

### Parallelization Notes

Inspector work, support reporting, and certification bundle work can progress
in parallel, but final closure should wait until the same admitted
subscription-family matrix is covered by diagnostics, capability reporting, and
hostile certification.

### Store Dependency

- Runtime-backed subscription diagnostics, bridge parity, and certification are
  not blocked on `forge-store`.
- Store-backed subscription execution parity remains Milestone 10 scope.
- Durable subscription continuation and replay remain Milestone 11 scope.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Query Subscription Bridge Parity And Diagnostic Sufficiency Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- every admitted automatic subscription family can be explained through
  canonical query-owned subscription artifacts and bridge-facing lowering
- diagnostics can localize declaration, basis, lifecycle, continuation,
  preview, and bridge-parity failures mechanically
- support metadata and admitted runtime behavior stay in sync for
  subscription-capable query families
- diagnostics can distinguish declaration-family changes from ordinary
  lifecycle-instance changes

## Milestone 9.3.1: Cross-Runtime Causal Diagnostics And Query Inspection

### Goal

Make Query inspection the ordinary public surface for explaining why a
query-observed result changed, did not change, failed, was denied, or replayed
differently across relational, runtime bridge, and signal boundaries.

### Adversarial Constraint

A downstream domain must be able to ask Query inspection for a causal
explanation of a query observation and receive one typed, machine-checkable
artifact anchored to the Query operational receipt that produced the
observation. Its digests and evidence references must join relational
authority, bridge routing/evaluation/source/structural/stream/preview/writeback
records, signal invalidation/evaluation/forensic availability, lineage,
provenance, and replay posture without direct imports from runtime-bridge
diagnostics, relational internals, or signal graph internals.

### Why This Milestone Exists

Milestone 9.3 proves subscription-family diagnostics and bridge parity, but it
does not yet close the general "why did this happen?" boundary. If domains have
to assemble explanations by reaching into the runtime bridge, relational, and
signal layers themselves, Query has failed to expose the explanation surface
that its public inspection contract needs.

This milestone gives that boundary a Query-owned roadmap home while preserving
lower-runtime authority: the bridge owns the cross-runtime causal envelope,
relational owns truth authority, signal owns invalidation/evaluation evidence,
and Query owns inspection admission, redaction, and public materialization.

### Specification

The governing milestone spec is
[milestone-9.3.1.md](./milestone-9.3.1.md).

### Must Ship

- Query inspection APIs and artifacts for cross-runtime causal explanations
- causal observation anchors and evidence-reference contracts derived from
  existing Query operational artifacts rather than post-hoc lower-runtime
  searches
- ordered implementation gates for anchor adapters, evidence-reference indexes,
  Query admission, bridge envelope assembly, Query materialization, and
  certification; each gate must close before the next production phase consumes
  it
- a bridge-owned causal explanation envelope carrying relational authority,
  bridge route/evaluation/source/structural/stream/preview/writeback/replay,
  signal invalidation/evaluation/forensic availability, lineage, provenance,
  replay posture, and materialization-policy digests
- success/advisory/violation admission artifacts so redacted or narrowed
  explanations do not collapse into a binary admitted/denied wall
- forge-proof-backed or equivalent proof-bearing progression for phase ordering,
  sealed witness minting, fixed-shape evidence-reference collections, and
  Query/bridge trust-boundary readmission
- first-class performance contracts and slope counters for anchor derivation,
  evidence-reference resolution, admission, bridge envelope assembly,
  redaction, materialization, and public artifact serialization
- typed denial artifacts for missing bridge route evidence, missing signal
  evidence, incompatible relational authority, policy-redacted diagnostics, and
  unsupported explanation families
- cold-path richness controls so expanded explanation detail never widens
  hot-path query execution or signal invalidation
- support metadata and certification rows for runtime-backed causal
  explanation families, with durable/store-backed replay called out as later
  milestone debt

### Must Preserve

- relational remains the authority for truth, commits, snapshots, and
  relational decision evidence
- runtime bridge remains the authority for bridge protocol, route/evaluation,
  writeback, preview, historical materialization, and cross-runtime envelope
  assembly
- signal remains the authority for observation, invalidation, scheduling,
  lineage, and provenance evidence
- Query remains the authority for canonical query intent, inspection admission,
  redaction, result-shape context, and public artifacts
- downstream domains consume explanations through Query inspection rather than
  stitching lower-runtime diagnostics locally

### Complexity / Proof Obligations

- prove changed, suppressed, denied, branch/preview, and replayed observations
  all produce typed causal inspection artifacts
- prove bridge, relational, and signal digests agree with the lower-runtime
  records they summarize
- prove Query consumes existing lower-runtime diagnostic, forensic, causality,
  provenance, and authority records rather than building a parallel diagnostics
  authority
- prove each phase emits exact performance counters and slope digests before the
  next phase consumes its artifact
- prove forge-proof shape digests or equivalent proof-shape artifacts prevent
  phase skipping, raw collection substitution, stale proof reuse, and forged
  lower-runtime authority witnesses
- prove missing evidence fails as typed diagnostic denial and redacted or
  narrowed evidence becomes typed advisory detail rather than best-effort
  narrative output
- prove diagnostic richness affects only cold-path materialization, not query
  meaning, planning, subscription semantics, or signal invalidation
- prove Worth-style consumers can delete direct explanation stitching across
  runtime bridge, relational, and signal internals once this artifact exists

### Allowed Debt

- durable causal explanation archives, restart-stable expanded narratives, and
  store-backed replay reconstruction may remain deferred to Milestones 10 and
  11
- domain-specific prose renderers may remain domain-owned if they consume the
  Query causal inspection artifact instead of lower-runtime internals

### Sequencing Notes

This belongs after Milestone 9.3 because bridge-honest subscription diagnostics
prove the narrow live-query explanation lane first. It belongs before the
Runtime API Public Stabilization Gate because inspection is part of the public
runtime API contract and should not be frozen while this boundary is still
being handled as domain glue.

### Store Dependency

Runtime-backed causal diagnostics are not blocked on `forge-store`. Durable
causal archives, persisted expanded inspection narratives, store-backed replay
reconstruction, and restart-stable causal envelope reload remain Milestone 10
and Milestone 11 scope.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Cross-Runtime Causal Explanation Envelope Test` in
  [test-requirements.md](./test-requirements.md)
  passes with canonical machine-checkable artifacts
- Query inspection can explain query-observed changed, suppressed, denied,
  branch/preview, and replayed outcomes through one typed public artifact
- bridge-owned causal envelopes carry digests for relational authority, bridge
  route/evaluation/source/structural/stream/preview/writeback/replay, signal
  invalidation/evaluation/forensic availability, lineage, provenance, replay
  posture, and materialization policy
- public Query artifacts preserve lower-runtime authority names instead of
  flattening them into Query-owned narrative facts
- downstream domains can consume causal explanations without direct imports
  from runtime bridge diagnostics, relational runtime internals, or signal graph
  internals

## Milestone 9.3.2: Query Basis Capability Lifecycle

### Goal

Make every Query basis a phase-typed capability lifecycle rather than a raw
branch, head, preview, snapshot, historical, tenant, or policy identifier.

### Specification

The governing milestone spec is
[milestone-9.3.2.md](./milestone-9.3.2.md). The closeout record is
[milestone-9.3.2-closeout.md](./milestone-9.3.2-closeout.md).

### Adversarial Constraint

A consumer must not be able to observe, mutate, replay, inspect, or materialize
against a basis unless Query has proven that the requested basis family is
eligible for that operation and has emitted a self-describing basis envelope
that names its authority, scope, visibility, lifecycle, and permitted next
transitions.

### Phase Contract

```text
RawBasisIntent
  -> NormalizedBasisIntent
  -> BasisEligibility | DeniedBasisCapability
  -> AdmittedBasisCapability
  -> ScopedExecutionOrObservationBasis
  -> LowerRuntimeBoundBasis
  -> BasisUseReceipt
  -> SelfDescribingBasisEnvelope
  -> BasisLifecycleCertificationBundle
```

### Must Ship

- phase-typed basis capability families for current head, branch, preview,
  snapshot, historical, tenant-scoped, and policy-scoped usage
- normalized basis intent, typed denied capability artifacts, lower-runtime
  readmission, and lifecycle certification closure
- basis eligibility decisions before read, mutation, replay, inspection, or
  materialization surfaces can be constructed
- basis use receipts that distinguish observation, mutation, replay,
  inspection, and materialization permission
- typed denials for stale, inaccessible, incompatible, or operation-ineligible
  basis requests

### Must Preserve

- relational remains the authority for branch, commit, snapshot, and head truth
- Query owns public basis capability admission and use receipts
- raw lower-runtime basis identifiers do not become public capability tokens
- temporal/resource basis extensions in Milestone 9.4 and the follow-on reuse
  hardening in Milestone 9.5 extend the same lifecycle rather than inventing
  parallel basis APIs

### Acceptance Evidence

This milestone is complete only when Query can prove equivalent basis intents
normalize to the same capability envelope, illegal transitions are
unrepresentable or typed failures, and downstream domains can read, inspect,
and prepare mutation surfaces without raw relational branch or snapshot IDs.

## Milestone 9.3.3: Authority-Scoped Effect Execution Pipeline

### Goal

Make Query-authored effects execute through one authority-scoped pipeline that
lowers semantic intent once and requires the executor to consume only lowered,
proof-bearing plans.

### Specification

The governing milestone spec is
[milestone-9.3.3.md](./milestone-9.3.3.md). The closeout record is
[milestone-9.3.3-closeout.md](./milestone-9.3.3-closeout.md).

### Adversarial Constraint

No executor may re-decide authority scope, basis posture, invariant scope,
preview policy, route strategy, artifact policy, or diagnostic richness. If
those decisions are needed, they must be proven before execution and carried in
the lowered effect execution plan.

### Phase Contract

```text
RawEffectIntent
  -> AuthorityEligibility
  -> AuthorityScopedEffectPlan
  -> LoweredEffectExecutionPlan
  -> EffectExecutionReceipt
  -> SelfDescribingEffectEnvelope
```

### Must Ship

- authority-scoped effect plans for current-head, branch, preview, workflow,
  topology, and query-authored writeback families where admitted
- lowered execution plans that carry basis, authority, invariant, artifact, and
  diagnostic policy proofs
- effect execution receipts that expose primary result, decision trace,
  structural deltas, integrity markers, and performance counters
- compile-fail or facade-boundary tests proving executors cannot accept raw
  intents or weaker plan types

### Must Preserve

- domain handlers produce declarative effects rather than framework ceremony
- runtime bridge and relational remain authorities for their execution
  semantics
- Query owns public effect admission, lowering, and receipt shaping
- branch mutation and preview mutation are parameters of the shared lifecycle,
  not separate `execute_on_branch`-style APIs

### Acceptance Evidence

This milestone is complete only when admitted effect families execute through
the same proof-widening pipeline, rejected families fail before construction or
lowering, and execution counters prove strategy decisions were resolved
upstream instead of rediscovered in the executor.

## Milestone 9.3.4: Declared Projection Consumption And Materialized Fact Receipts

### Goal

Make consumption of materialized Query projections a declared, typed,
receipt-backed contract so consumers can use projection facts without reopening
the source authority.

### Specification

The governing milestone spec is
[milestone-9.3.4.md](./milestone-9.3.4.md).

### Adversarial Constraint

A consumer that has received a materialized projection must not fish in
relational truth, bridge internals, signal internals, or domain-specific caches
to discover IDs, memberships, labels, topology facts, workflow facts, table
facts, or geometry facts that should have been declared as consumed projection
facts.

### Phase Contract

```text
ProjectionConsumptionDeclaration
  -> ProjectionConsumptionEligibility | DeniedProjectionConsumption
  -> MaterializedProjectionContract
  -> ConsumedProjectionFactSet
  -> ProjectionConsumptionReceipt
  -> SelfDescribingProjectionConsumptionEnvelope
  -> ProjectionConsumptionCertificationBundle
```

### Must Ship

- projection consumption declarations for materialized query views
- typed consumed fact sets for entity identities, relation identities,
  memberships, labels, derived facts, shape facts, and view-local identities
  where the projection admits them
- materialized projection contracts that bind consumed facts to the query,
  basis, policy, view shape, and materialization digest that produced them
- topology/Worth certification as the first hostile lane, without making the
  milestone topology-specific

### Must Preserve

- relational owns authoritative truth; materialized projections are derived
- Query owns projection contracts, declared consumption, and receipts
- domain consumers own how they use admitted projection facts, not how those
  facts are rediscovered from authority
- future geometry, workflow, table, and design projections reuse the same
  lifecycle instead of adding local lookup helpers

### Acceptance Evidence

This milestone is complete only when a certification program can declare the
projection facts it consumes, receive typed fact receipts bound to one
materialization, and avoid direct source-authority reads for fact discovery.

## Milestone 9.3.5: Intent Admission Decision Lattice And Decision Trace

The governing milestone spec is
[milestone-9.3.5.md](./milestone-9.3.5.md).

Status:

- Closed on 2026-05-18 via
  [milestone-9.3.5-closeout.md](./milestone-9.3.5-closeout.md)

### Goal

Make every Query-crossing intent resolve through a structured admission
decision lattice before construction, command lowering, execution, or
diagnostic materialization, and make covered admitted paths cross into real
bridge-backed execution through one canonical typed handoff.

### Adversarial Constraint

No Query surface may collapse admission into a binary `Result` wall that loses
actionable context. Success, advisory, and violation outcomes must all carry
structured decision traces, machine-readable context, and enough phase proof
for downstream code to proceed, adapt, or fail closed without reconstructing
the decision.

### Phase Contract

```text
RawIntent
  -> IntentEligibility
  -> AdmissionDecision
  -> AdmittedIntentPlan | AdvisoryDecision | ViolationDecision
  -> AdmittedExecutionHandoff | AdvisoryStop | ViolationStop
  -> DecisionTraceEnvelope
```

### Must Ship

- a shared admission decision lattice for reads, basis use, projection
  consumption, effect execution, inspection, diagnostic materialization, and
  lower-runtime capability routing
- structured success, advisory, and violation decision variants with typed
  context
- typed execution handoffs for every covered family whose admitted form already
  binds to a real bridge/runtime execution seam
- decision traces that record policy, capability, invariant, basis, projection,
  and lower-runtime routing decisions where applicable
- compile-fail or construction-boundary tests proving rejected or advisory-only
  intents cannot be lowered as admitted plans

### Must Preserve

- eligibility precedes expensive construction and domain object assembly
- diagnostics can enrich admission traces without changing the operational
  result
- each authority keeps ownership of its decision evidence while Query owns the
  public admission envelope
- binary convenience results may exist only as derived summaries, not as the
  canonical admission artifact

### Acceptance Evidence

This milestone is complete only when all admitted 9.3.x surfaces share the
decision lattice, failure and advisory cases are as inspectable as successful
cases, lower phases consume proof-bearing admitted plans rather than
revalidating raw intents, and covered execution paths consume typed admitted
handoffs rather than rediscovering admission from raw requests.

## Milestone 9.3.6: Lower-Runtime Capability Routing And Boundary Envelopes

### Goal

Make all Query contact with relational, runtime bridge, signal, and later store
surfaces pass through capability-routed lower-runtime boundary envelopes rather
than scattered direct imports or compatibility shortcuts.

### Specification

The governing milestone spec is
[milestone-9.3.6.md](./milestone-9.3.6.md).

### Adversarial Constraint

If direct bridge, relational, signal, and Query-runtime bridge paths coexist,
they must share one lifecycle abstraction or be marked as explicit
compatibility debt. A domain or certification program must not be able to
choose a lower-runtime path by convenience and silently bypass Query's basis,
admission, projection, effect, or inspection contracts.

### Phase Contract

```text
LowerRuntimeCapabilityRequest
  -> CapabilityEligibility
  -> LowerRuntimeRoutePlan
  -> BoundaryExecutionReceipt
  -> LowerRuntimeBoundaryEnvelope
```

### Must Ship

- lower-runtime capability routing for admitted relational, bridge, signal, and
  store-adjacent contacts
- boundary envelopes that name authority, route, capability, cost posture,
  failure topology, and retained evidence
- compatibility-debt records for any remaining direct lower-runtime paths,
  including exit criteria and certification coverage
- facade and compile-boundary tests proving ordinary consumers use Query's
  routed capability lane rather than lower-runtime internals

### Must Preserve

- lower runtimes remain autonomous subsystems with contractual facades
- Query routes capabilities; it does not absorb lower-runtime truth,
  scheduling, storage, or bridge protocol authority
- shared lifecycle is the abstraction; traversal, data topology, failure mode,
  and cost posture remain explicit parameters
- cost and failure boundaries are not flattened by a generic adapter bag

### Acceptance Evidence

This milestone is complete only when every 9.3.x public capability can name its
lower-runtime route, receipt, and boundary envelope, and any remaining direct
path is intentionally tracked compatibility debt rather than an accidental
escape hatch.

## Milestone 9.3.7: Domain Capability Contributions And Canonical Runtime Materialization

### Goal

Allow downstream domains to contribute typed semantic capability posture to
Query through one public contribution seam, while Query remains the sole owner
of canonical runtime artifacts across admission, support, traceability,
workflow, continuity, aftermath, and explanation surfaces.

### Specification

The governing milestone spec is
[milestone-9.3.7.md](./milestone-9.3.7.md).

### Adversarial Constraint

A serious downstream domain must be able to tell Query "this declaration is
advisory", "this declaration is violating", "this declaration carries
declaration-scoped support or traceability", "this declaration has workflow
promotion or discard posture", "this declaration preserves or splits
continuity", "this declaration establishes aftermath facts", or "this
declaration requires explanation context" without minting local pseudo-Query
artifacts, without calling crate-private constructors, and without flattening
semantic posture into generic strings or ad hoc JSON.

### Phase Contract

```text
DomainCapabilityContributionRequest
  -> DomainCapabilityContributionEligibility
  -> AdmittedDomainCapabilityContribution
  -> CanonicalRuntimeMaterialization
  -> QueryAdmissionDecision
   | QuerySupportTraceability
   | QueryWorkflowArtifacts
   | QueryContinuityArtifacts
   | QueryAftermathArtifacts
   | QueryExplanationArtifacts
```

### Must Ship

- one public Query-owned domain capability contribution lifecycle
- typed domain contribution families for:
  - admission posture
  - declaration-scoped support and traceability posture
  - invariant and capability posture
  - workflow / preview posture
  - continuity / lineage posture
  - consequence / aftermath posture
  - explanation / inspection posture
- canonical materializers that turn admitted domain contributions into:
  - `ForgeQueryIntentAdvisoryDecision`
  - `ForgeQueryIntentViolationDecision`
  - declaration-scoped support/traceability artifacts
  - graph-composition capability / invariant-facing artifacts where applicable
- fully closed workflow, continuity, aftermath, and explanation contribution
  families on top of the shared contribution substrate
- compile-boundary and certification proof that canonical Query decision
  artifacts remain Query-owned and cannot be minted directly by domains

### Must Preserve

- Query remains owner of canonical runtime admission artifacts
- downstream domains own semantic meaning, not canonical artifact authority
- public constructors for canonical advisory and violation artifacts do not
  become a free-for-all bypass
- declaration-scoped support and invariant posture stays typed and
  machine-checkable rather than collapsing into free-form messages

### Acceptance Evidence

This milestone is complete only when semantically equivalent domain-authored
advisory or violation posture materializes into the same canonical Query
decision/support artifacts regardless of builder path, while semantically
different posture diverges predictably and illegal direct artifact minting
fails closed.

## Milestone 9.3.8: Query-As-Beginning Platform Entry

### Specification

The governing milestone spec is
[milestone-9.3.8.md](./milestone-9.3.8.md). The closeout record is
[milestone-9.3.8-closeout.md](./milestone-9.3.8-closeout.md).

### Goal

Make `forge-query` the true first-class platform entry for serious downstream
domain work so declarations, progression, authority routing, preparation,
continuation, inspection, ergonomics, and certification all begin inside one
Query-owned public seam rather than being split across local pseudo-Query
layers above relational, the runtime bridge, signal, `forge-proof`, and
`forge-foundational`.

### Adversarial Constraint

A geometry-kernel-grade domain must be able to enter Forge through Query once
and stay inside one honest public lifecycle while expressing declaration
meaning, support posture, continuity posture, preparation readiness, runtime
continuation, and inspection needs without reconstructing a second semantic
world in host glue or domain-local adapters.

### Must Ship

- one Query-owned platform-entry seam that begins at domain entry and extends
  through the full boundary stack captured in
  [milestone-9.3.8.md](./milestone-9.3.8.md)
- a phase-locked implementation that follows the milestone's explicit
  boundary order rather than re-splitting the work into disconnected
  declaration, preparation, and runtime-handoff mini-products
- public route-plan, boundary-receipt, and boundary-envelope artifacts for the
  covered lower-authority crossings
- a framework-quality ordinary lane that compiles onto lower authorities
  without caller-owned choreography
- unified inspection, support/readiness, documentation, and certification for
  the resulting platform-entry lifecycle

### Must Preserve

- Query remains the front door and orchestration surface rather than a second
  truth, continuation, or derived-execution engine
- relational, the runtime bridge, signal, `forge-proof`, and
  `forge-foundational` remain the authorities for the semantics they already
  own
- each internal phase remains a real capability boundary rather than a vague
  implementation bucket
- the milestone may be large, but it must still close as one coherent product
  capability instead of shipping fragmented half-seams

### Acceptance Evidence

This milestone is complete only when serious downstream domains can enter
Query once, progress through the covered boundary phases without rebuilding
local pseudo-Query layers, and receive public route/receipt/envelope,
inspection, and certification artifacts that converge across equivalent paths
while divergent posture and illegal shortcuts fail typed and early.

The late collaboration extension inside `9.3.8` also depends on shared
lower-authority hardening work in `forge-signal`, `forge-relational`, and
`forge-runtime-bridge` so Query can consume retained branch, merge, lineage,
preview, policy, and strategy posture instead of reconstructing collaboration
meaning from host-local glue. The first of those shared hardening specs is
[`../forge_signal/collaboration_branching_hardening_plan.md`](../forge_signal/collaboration_branching_hardening_plan.md).

## Runtime API Public Stabilization Gate

### Goal

Freeze the ordinary public runtime API contract after the runtime facade has
consumed Milestones 9.1 through 9.3.8, so downstream domain runtimes can build
against named surfaces and typed handles now while Milestones 9.4 and 9.5
later extend the same API model with temporal/async semantics and the related
productization cleanup.

### Adversarial Constraint

A serious domain runtime must be able to build workflow, geometry, table, or
application features against the public Query facade now, without lower-runtime
plumbing, and without any later temporal/async milestone forcing a parallel API
or sync-to-async rewrite.

### Specification

The governing stabilization spec is
[runtime-api-public-stabilization-plan.md](./runtime-api-public-stabilization-plan.md).

### Must Ship

- golden DX transcript tests for workflow, geometry, table, and composed
  adversarial runtime surfaces
- final public vocabulary for workspace, durable surfaces, handles, state,
  aspects, computeds, effects, intents, branch/preview reuse, and inspection
- support matrix rows distinguishing stable runtime-backed surfaces from
  deferred temporal/async/store/durable surfaces
- compile-fail rejection of lower-runtime plumbing and temporal/async pre-claim
  shortcuts from ordinary public API usage

### Must Preserve

- no domain semantics move into `forge-query`
- temporal/async behavior remains deferred until Milestones 9.4 and 9.5
- lower runtimes remain authorities for truth, signal execution, bridge
  protocol, temporal scheduling, async lifecycle, store parity, and durability
- later temporal/async milestones extend the stabilized handle/state/aspect
  contract rather than adding sibling public APIs

### Store Dependency

This gate is not blocked on `forge-store`. It must explicitly mark store-backed
and durable claims as later milestone debt.

### Acceptance Evidence

This gate is complete only when `forge-query` can prove that the final public
facade supports the golden DX transcripts through meaningful assertions over
receipts, lanes, aspects, delivery, residue, support posture, and inspection,
while unsupported temporal/async neighbors fail typed and early.

## Runtime Authoritative Mutation Evidence Gate

### Goal

Freeze the ordinary public mutation-evidence contract after runtime facade
stabilization so downstream write-heavy domains can build on aspect-native
authority surfaces without rebuilding target recovery, existing-truth identity
binding, naming-writeback evidence, or continuity-sensitive inspection locally.

### Adversarial Constraint

Direct writes, ordered batches, authoritative imports, preview-local mutation,
projected naming attachment, continuity-sensitive updates, and domain-authored
writeback lowering must preserve the same canonical target-class meaning, the
same target identity evidence, and the same typed explanation of what was
actually targeted regardless of whether the target was new, preexisting,
referenced earlier in the same batch, or bound through admitted naming or
continuity evidence.

### Specification

The governing hardening spec is
[runtime-authoritative-mutation-evidence-plan.md](./runtime-authoritative-mutation-evidence-plan.md).

The required follow-on hardening spec for generic graph-shaped authoring and
identity-preserving existing-target mutation is
[runtime-generic-graph-authoring-plan.md](./runtime-generic-graph-authoring-plan.md).

The admitted graph-authoring and geometry-pressure closure for that follow-on
hardening is now frozen in
[runtime-generic-graph-authoring-closeout.md](./runtime-generic-graph-authoring-closeout.md).

### Must Ship

- explicit declared-versus-resolved target evidence in public receipts and
  inspection
- honest aggregate batch/session authority evidence for multi-write mutation
  sessions
- first-class runtime-carried causality and provenance so lineage and source
  explanation come through the public authority lane by construction
- admitted existing-truth identity binding with typed denial for unresolved or
  incompatible bindings
- admitted naming-aware and continuity-aware mutation evidence families that
  either preserve explicit outcome meaning or fail typed and early
- domain-agnostic public authoring surfaces for:
  - identity-preserving existing-target relation updates
  - first-class same-batch graph composition with a widening mixed-shape
    capability contract
  - bridge-backed backend-verified existing-truth checks on admitted runtime
    families
- support-matrix and certification rows covering the hardened mutation surface

### Must Preserve

- aspect-native CRUD remains the ordinary public mutation story
- touched-aspect fallout meaning stays explicit and auditable alongside target
  evidence
- lower runtimes remain authoritative for truth, naming, writeback, and
  lineage semantics
- unsupported identity-binding, naming, or continuity families fail closed
  rather than degrading into best-effort target recovery

### Store Dependency

This gate is not blocked on `forge-store`. Durable restart, store-backed replay,
and persisted mutation artifact reload remain later-milestone debt.

### Acceptance Evidence

This gate is complete only when `forge-query` can prove that public receipts,
inspection bundles, support metadata, and executable admission behavior agree
on target evidence, existing-truth binding, and admitted naming/continuity
neighbors, while downstream domains can delete local target-recovery glue
instead of merely wrapping it. The gate is not fully closed until the authoring
surfaces and certification obligations in
[runtime-generic-graph-authoring-plan.md](./runtime-generic-graph-authoring-plan.md)
are also satisfied. That follow-on closure now exists in
[runtime-generic-graph-authoring-closeout.md](./runtime-generic-graph-authoring-closeout.md).

## Milestone 9.4: Runtime-Backed Temporal And Async Query Surface

### Goal

Close the full runtime-backed temporal and async Query product surface so
application code can declare, admit, inspect, execute, and consume temporal
wakes, async/resource lifecycle, and mixed truth/time/async delivery through
one canonical Query facade before store-backed and durable follow-on work
begins.

### Adversarial Constraint

For the same canonical Query declaration, truth-view basis, temporal posture,
tenant/policy context, preview posture, and async source family, Query must
produce the same admitted Query basis, the same result-state meaning, the same
mixed-cause delivery ordering, and the same explanation artifacts regardless
of whether the observed change came from a relational truth patch, a time-only
wake, an async completion, a retry or revalidation path, replay, restart or
resume, or preview promotion or discard.

### Why This Milestone Exists

Bridge Milestone 17 now closes the lower-authority temporal and async law:
temporal bridge basis, time-aware subscription admission, async source
declaration identity, completion causality, mixed-cause ordering,
restart/resume posture, preview residue law, and offline certification
bundles. Query now has to project that into one ordinary product surface
instead of splitting temporal, async, mixed-cause, and certification into
separate roadmap milestones.

Without this milestone:

- downstream domains would have to reach around Query for temporal or async
  behavior
- app surfaces would invent local pending, fulfilled, stale, cancelled, or
  superseded meaning that drifts from bridge-closed causality law
- time-only changes would be treated as diagnostics noise or fake truth patches
- `forge-server` and later consumers would inherit half-Query, half-host
  delivery semantics instead of one typed contract

### Must Ship

- temporal query basis descriptors and time-aware subscription lowering through
  the stabilized Query runtime facade
- async/resource query families with typed result-state and completion-cause
  semantics
- mixed truth/time/async delivery ordering, coalescing, and replay-equivalent
  query-shaped delivery metadata
- restart-aware runtime-backed temporal and async continuation posture where
  admitted
- typed support metadata, diagnostics, and hostile certification for the full
  merged temporal/async surface

### Must Preserve

- Query owns public product meaning and result shape, not clock truth, wake
  scheduling, async lifecycle truth, retry policy, or mixed-cause authority
- `forge-runtime-bridge` remains authoritative for temporal basis, async
  identity, completion causality, mixed-cause ordering, restart/resume basis,
  preview residue law, and certification bundle shape
- `forge-signal` remains authoritative for temporal eligibility,
  previous-value semantics, wake readiness, async lifecycle, retry, timeout,
  cancellation, supersession, and revalidation policy
- historical truth basis remains distinct from temporal execution basis at
  every public Query boundary

### Sequencing Notes

This milestone absorbs the old roadmap `9.4`, `9.5`, `9.6`, and `9.7` split
into one Query milestone. The detailed internal execution plan now lives in
[milestone-9.4.md](./milestone-9.4.md).

It belongs after the runtime API stabilization and authoritative mutation
evidence gates because Query must extend one already-stabilized ordinary
runtime facade.

It belongs before Milestone 10 because store-backed execution parity should
not be forced to discover temporal, async, mixed-cause, and certification
semantics while also closing durable backend parity.

### Store Dependency

- Runtime-backed temporal basis, async families, mixed-cause delivery, and
  certification are not blocked on `forge-store`.
- Store-backed historical restore, persisted temporal replay, and durable
  async continuation remain later scope for Milestones `10` and `11`.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the merged temporal/async certification suites in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  pass with canonical machine-checkable artifacts
- temporal query basis, time-only delivery, async/resource result-state, and
  mixed truth/time/async ordering all remain Query-shaped and replay-equivalent
- unsupported ambient clocks, unbound timers, stale async completions, raw
  timer folklore, and unsupported mixed-cause neighbors fail typed and early
- the milestone-close proof matches the merged [milestone-9.4.md](./milestone-9.4.md)
  closure instead of leaving separate old `9.5` through `9.7` gaps behind

## Milestone 9.5: Query Productization Debt Cleanup For Reuse, View Shapes, And Typed Consumption

### Goal

Close the remaining runtime-backed Query productization debt in reusable
composition, core view-shape families, grouped composition, retained-artifact
projection consumption, and preserved temporal/async reuse so the ordinary
product facade stops carrying visible-but-not-fully-hardened lanes into the
store-backed and durable milestones.

### Adversarial Constraint

For the same canonical query declaration, scope/template expansion, view-shape
family, retained result artifact, basis/remask posture, and preserved
temporal/async reuse posture, Query must produce the same canonical
declaration identity, the same support/admission posture, the same
fact-consumption contract, and the same delivery/reuse meaning regardless of
whether the caller reaches the lane through direct composition, grouped
composition, inspector/grouped reuse, or retained artifact consumption.

### Why This Milestone Exists

The old roadmap split temporal/async work into `9.4` through `9.7`, but that
semantic work now closes inside one merged [milestone-9.4.md](./milestone-9.4.md).
That leaves a different remaining problem: several ordinary productization
lanes are admitted, documented, and in daily use, but still carry explicit
debt markers or structurally unfinished reuse/productization work.

Without this milestone:

- store-backed and durable milestones will freeze half-hardened composition and
  view-shape semantics
- direct consumers will keep hitting special-case pack/bind/decode seams where
  Query claims a typed fact-consumption lane
- support and profile surfaces will keep advertising core product families as
  admitted-but-debt instead of actually closed

### Must Ship

- hardened named-scope expansion and template-instantiation support profiles
  that can move from `debt` to production-ready runtime-backed closure
- hardened core view-shape families for `table`, `detail`,
  `inspector_detail_observed`, `inspector_detail_focused`, and
  `kanban_grouped`, including their support/profile and ordinary product
  semantics
- grouped template/composition closure so grouped planning is no longer an
  admitted public lane carrying explicit composition debt
- projection-consumption source-family closure for retained derived artifact
  bindings and live artifact bindings where Query intends them to participate
  as first-class typed fact sources
- hardened runtime-backed preserved reuse across the inspector/grouped
  temporal/async neighbors so the covered reuse surface carries merged `9.4`
  meaning end to end
- simple public raw runtime bootstrap for a valid bridge-backed read runtime
  so hostile runtime-backed tests do not need custom assembly just to reach the
  ordinary read lane

### Must Preserve

- scope/template expansion remains canonical declaration composition rather
  than string substitution or host-local query rewriting
- view shape remains part of planning, delivery, and reuse semantics rather
  than display-only sugar
- projection consumption remains the typed fact lane; it must not collapse back
  into row-bag decoding folklore
- temporal/async reuse must preserve the merged `9.4` runtime-backed meaning
  rather than erasing time/async posture during scope/template/view reuse

### Sequencing Notes

The detailed debt-close execution plan for this milestone now lives in
[milestone-9.5.md](./milestone-9.5.md).

This milestone belongs immediately after the merged
[milestone-9.4.md](./milestone-9.4.md) closure so runtime-backed temporal and
async meaning is already frozen before reusable composition, retained
artifacts, and preserved view-shape reuse try to carry it forward.

It belongs before Milestones `10` and `11` because store-backed and durable
milestones should not inherit half-hardened runtime-backed productization
surfaces as if they were final.

### Store Dependency

- This milestone is not blocked on `forge-store`.
- Durable saved-query reload, restart-stable continuation, and store-backed
  temporal/async reuse remain later scope for Milestones `10` and `11`.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the explicit `debt` markers for named-scope expansion, template
  instantiation, and the admitted core view-shape families are removed
- grouped composition docs and support/profile surfaces no longer describe the
  admitted grouped planning lane as composition debt
- retained derived artifact bindings and live artifact bindings no longer force
  product code through special-case runtime-owned fact seams where Query claims
  a first-class projection-consumption lane
- inspector/grouped temporal/async reuse neighbors carry canonical
  runtime-backed semantics across the covered reuse surface
- hostile runtime-backed read tests no longer need a custom minimal
  bridge-backed harness just to obtain a valid raw runtime read path

## Milestone 9.6: Product Boundary Debt Closure For Evidence Identity, Typed Stop Classes, And Session Label Identity

> **Status:** Closed for non-spatial identity-boundary scope on `query-repair`;
> `worth-spatial public_api_contract` remains a named postponed external gate.
> Non-spatial Phases 8–12 are reconciled in
> [milestone-9.6-attack-plan.md](./milestone-9.6-attack-plan.md), and Milestone
> 9.7 is unblocked for forge-query identity-boundary sequencing.

### Goal

Make evidence identity, stop-class matching, and session label identity
runtime-owned structural contracts so consumers never format runtime values
into digests, string-match error messages in decision paths, or mint
free-form session labels against a runtime built on canonical identity.

### Adversarial Constraint

For the same runtime fact — admission denial, basis admission, receipt,
support row, session identity — Query must produce the same canonical
evidence identity and the same typed stop-class meaning under `Debug` derive
reordering, field renaming, message rewording, separator injection inside
field values, and session label collision pressure.

### Why This Milestone Exists

The first serious downstream consumer hashes `format!("{:?}", value)` strings
joined with `|` for evidence identity, matches runtime denials with
`message.contains(...)`, and opens preview/branch sessions with free-form
string labels. Each is a Query-owned contract carried by consumer folklore.
Concurrency receipts (`9.7`) and the consumer kit (`9.8`) must be born on
canonical identity rather than migrated onto it later.

### Must Ship

- one sealed, scheme-versioned canonical evidence-identity primitive
- migration of covered Query-owned digest surfaces onto that primitive with
  zero format-string residue
- typed stop-class matching across covered denial paths, including typed
  family payloads on admission denials
- canonical session label identity for preview/branch entry with explicit
  collision posture
- support/profile, docs, and hostile certification closure for all three
  boundaries

### Must Preserve

- the existing rich error topology, extended into matchability rather than
  flattened
- the existing public facade shape; no parallel digest, error, or label APIs
- human-readable diagnostics as presentation atop typed contracts

### Complexity / Proof Obligations

- name the canonical encoding and digest contracts and prove digest stability
  under formatting drift and separator injection with exact assertions
- prove typed stop-class coverage with a consumer-shaped zero-string-ops
  matching suite that survives message rewording
- prove session label collision posture with typed collision stops

### Allowed Debt

- durable digest archives and restart-stable identity reload remain explicit
  debt until `Milestone 10` and `Milestone 11`
- no covered surface may keep the format-string digest scheme as debt

### Sequencing Notes

The detailed execution plan lives in [milestone-9.6.md](./milestone-9.6.md).
This milestone belongs immediately after `Milestone 9.5` and before
`Milestone 9.7`, whose receipts, journal identity, and published-artifact
digests must use the canonical scheme from birth.

### Parallelization Notes

Phases are mostly independent per boundary; digest migration must complete
before `9.7` begins emitting new receipt families.

### Store Dependency

- This milestone is not blocked on `forge-store`.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- covered digest surfaces emit scheme-versioned canonical digests with zero
  format-string construction
- a consumer-shaped matching suite handles every covered stop class without
  string operations, surviving hostile message rewording
- preview/branch entry flows through canonical label identity with typed
  collision posture
- docs, support profiles, and certification agree the boundaries are closed

## Milestone 9.7: Concurrent Read Authority And Deterministic Submission

### Goal

Decompose the Query runtime into authority-typed subsystems so
committed-snapshot reads scale concurrently across consumers while truth
mutation and derived maintenance remain single-owner and deterministic,
without changing canonical query meaning or adding a second semantics path
beside the workspace.

### Adversarial Constraint

N concurrent shared read contexts under sustained commit pressure, preview
and branch churn, and live maintenance load must produce byte-identical
results and receipts for the same canonical declaration and basis capability
as fully serialized execution — while journal replay reconstructs identical
truth, receipts, and published derived artifacts, with zero locks on the
committed-read hot path and zero derived evaluations triggered by readers.

### Why This Milestone Exists

Every workspace operation takes `&mut self`, so the borrow checker enforces
one operation in flight per workspace regardless of MVCC immutability
underneath. Server-grade consumers would otherwise improvise a global lock or
branch-per-connection — both prohibited folklore. Store-backed shapes in
`Milestone 10` must inherit lane-correct contracts rather than retrofit
`Send` boundaries later.

### Must Ship

- backend adapter contracts decomposed by authority lane with `Send + Sync`
  read lanes (Phases 1–2)
- runtime-owned published-artifact registry authority with registry/mint
  inventory and scans (Phase 11)
- generation-indexed pinning with lock-free hot path, pin/retire inventory, and
  runtime-owned residue counters (Phase 12)
- shared read contexts with full pinning-boundary closure in-phase (Phase 13)
- typed journal position identity with journal inventory and scans (Phase 14)
- consumer-facing journal-segment replay with journal-boundary closure (Phase 15)
- the published derived-artifact rule: readers consume digest-stamped
  published results through projection consumption; only the maintenance
  owner evaluates
- the re-expressed workspace facade with unchanged existing consumer surface
  and fail-closed admission rows for the new families (Phase 9)
- real concurrent hostile certification with in-phase sabotage proof (Phase 16)
- public-bridge projection-consumption honesty (Phase 17)
- derived milestone closure posture and closeout doc (Phase 18)

### Must Preserve

- canonical query meaning across serialized and concurrent execution
- lower-crate authority boundaries; only access topology moves
- the single-owner workspace as a first-class consumer surface
- merged `9.4` temporal/async meaning and `9.5` projection-consumption
  semantics inside the published-artifact lane

### Complexity / Proof Obligations

- name the read-context mint, submission intake, and publication contracts
- expose exact counters for lock acquisitions (zero), reader evaluations
  (zero), snapshot generation pins/retirements, journal positions, and
  publication breadth
- prove byte-identical concurrent-versus-serialized receipts and journal
  replay parity

### Allowed Debt

- durable journal persistence, store-backed replay reconstruction, and
  restart-stable published-artifact reload remain explicit debt until
  `Milestone 10` and `Milestone 11`
- lock-based or evaluation-leaking read paths may not ship as debt

### Sequencing Notes

The detailed execution plan lives in [milestone-9.7.md](./milestone-9.7.md).
This milestone belongs after `Milestone 9.6` so its receipts and digests are
born canonical, and before `Milestone 10` as a hard gate so store-backed
shapes inherit the concurrency topology.

### Parallelization Notes

Phases 1–10 may overlap at the topology layer (adapter decomposition,
read-context scaffold, submission seam, facade families, interim hostile
schedule). Phases 11–18 are the mandatory honesty end-cap: each phase owns its
substrate and proof together — inventory slices, scans, hostile schedules, and
sabotage close inside the phase that ships the work. Sequence:
**11 → 12 → 13** (pinning closes in Phase 13), **14 → 15** (journal closes in
Phase 15), **16 → 17** (certification with in-phase sabotage, public-bridge
honesty), **18** (aggregated closeout only). Milestone `9.7` may not report
`Closed` until Phase 18.

### Store Dependency

- This milestone is not blocked on `forge-store`.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- concurrent readers under write pressure produce byte-identical receipts and
  results to serialized execution with exact-zero lock and reader-evaluation
  counters
- journal replay reconstructs identical truth, receipts, and published
  artifacts
- existing downstream consumers compile unchanged against the re-expressed
  facade
- the new facade families carry honest fail-closed support/admission rows

## Milestone 9.8: Downstream Consumer Product Kit For Evidence Reports, Boundary Audits, And Support Pinning

### Goal

Ship the runtime-owned kit that eliminates consumer-side folklore around
Query's product contracts — declarative evidence-report scaffolding, a
shipped boundary-bypass audit, and exportable, pinnable support snapshots —
proven by reference-consumer adoption rather than API presence.

### Adversarial Constraint

A downstream domain crate must be able to author a digest-bearing evidence
report, enforce the no-bypass contract, and pin its support-posture
dependencies using only Query-shipped kit surfaces, with every divergence
class — escaped digest fields, prohibited seam usage, pinned posture
regression, folklore resurrection — failing mechanically in the consumer's
build.

### Why This Milestone Exists

The reference consumer pays roughly 250 hand-rolled lines per evidence
report, enforces the hard prohibitions with `include_str!` source greps, and
re-derives support posture as hand-built gap rows. Each is a runtime-owned
contract materialized by consumer folklore, and every future consumer would
reinvent or skip it. Milestones `9.6` and `9.7` harden what Query says; this
milestone hardens what Query gives consumers to build with.

### Must Ship

- the declarative evidence-report composition kit over the `9.6` canonical
  evidence-identity primitive
- runtime-owned bypass enforcement: sealed or visibility-tightened seams plus
  one shipped audit artifact derived from a single prohibition registry
- the serialized, versioned support snapshot and typed consumer pinning
  contract with build-failing drift detection
- the shipped in-memory consumer test backend with honest fail-closed support
  posture, replacing hand-implemented adapter assemblies and hand-fabricated
  receipts in consumer test suites
- reference-consumer adoption with deletion of `worth-kernel`'s hand-rolled
  report plumbing, grep audit, and gap-row assembly in covered surfaces
- support/profile, docs, and hostile certification closure for the kit
  families

### Must Preserve

- the `9.6` canonical evidence-identity scheme as the only digest authority
  the kit can express
- one support truth: the snapshot is a digest-bound derived projection of the
  live matrix
- the reference consumer's evidence semantics through migration
- the Query facade as the only consumer surface

### Complexity / Proof Obligations

- name the kit report, audit, and pinning contracts
- prove kit-versus-hand-rolled report parity, structural (non-textual) bypass
  detection with zero false positives on comments and literals, and
  pin-localized build failure under posture regression
- prove adoption residue with exact-zero assertions on covered consumer
  surfaces

### Allowed Debt

- persisted support snapshots, durable audit archives, and store-backed kit
  artifacts remain explicit debt until `Milestone 10` and `Milestone 11`
- shipping the kit without reference-consumer adoption may not be claimed as
  closure

### Sequencing Notes

The detailed execution plan lives in [milestone-9.8.md](./milestone-9.8.md).
This milestone belongs after `Milestone 9.7` so the kit covers the
concurrency-era facade families, and before `Milestone 10` closure so real
consumer adoption pressure-tests the frozen runtime-backed surface.

### Parallelization Notes

Kit phases may overlap early `Milestone 10` work where staffing allows, since
store execution does not consume kit surfaces; reference adoption and
certification close strictly last.

### Store Dependency

- This milestone is not blocked on `forge-store`.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- a kit-authored report reproduces a hand-rolled report's semantics with
  canonical-scheme digests, and misuse fails typed or fails to compile
- the shipped audit detects seeded bypasses structurally from a downstream
  crate's test suite with zero textual false positives
- a pinned posture regression fails exactly the pinned consumers' builds with
  typed findings
- a downstream-shaped test suite obtains a valid workspace from the shipped
  in-memory backend with zero hand-implemented adapter traits and zero
  hand-fabricated receipts
- covered `worth-kernel` surfaces carry zero remaining hand-rolled digest,
  audit, or gap-row folklore

## Milestone 9.9: Graph Touch Obligation Authority

### Goal

Establish graph touch obligation dispatch as a complete Query authority
boundary — typed obligation kinds, three-state verdicts, canonical dispatch
artifacts, index-backed selection, relational execution bridge, duplicate-rule
elimination, and mechanical consumer anti-folklore — certified
architecturally and proven by reference-consumer deletion of parallel
legality in `worth-topo` and `worth-kernel`.

### Adversarial Constraint

Obligation dispatch must be a pure function of touch descriptor, operating
world, and assembly index — on every lane that reaches authoritative
execution: write-batch intent admission, declaration-entry orchestration,
read-family execution, and preview/branch mutation where applicable — with
exact-zero false negatives, false positives, duplicate rule implementations,
manual pre-check residue, and dispatch-plan drift on covered surfaces under
property-test certification.

### Why This Milestone Exists

Query teaches register invariants and consume typed graph-composition
denials, but the product surface still trains manual invariant-pack callbacks
and duplicate enforcement across pre-checks, materialized-view validators,
and commit-boundary invariants. `worth-topo` calls `compose_graph` yet
enforces loop wiring in three places; `worth-kernel` carries host-local
layout legality and motion sequencing guarded by `unreachable!`. This
milestone ships obligation authority as foundation infrastructure before
store-backed execution inherits another legality layer.

### Must Ship

- complete obligation authority model with multi-obligation dispatch envelopes
- graph touch descriptors for mutation and read lanes
- registration, auto-indexing, assembly index with complexity contracts
- relational graph-composition execution point and rule migration
- authoritative write-batch intent admission integration (canonical dispatch seam)
- declaration-entry and contribution-orchestration dispatch
- read-family and read-composition dispatch (all four entry points)
- preview and branch mutation obligation parity
- policy-aware graph mutation execution and operating-context gate dispatch
- advisory, capability-gap, and preflight-sequencing executors
- envelope attachment to receipts, decision traces, and mutation evidence
- derived read validation re-homed; consumer obligation bypass audit
- kernel construction operating context wiring
- primitive construction birth compose execution (all covered families)
- full reference adoption in `worth-topo` and `worth-kernel` construction
- public docs and AI_README category
- architectural hostile certification matrix closure

### Must Preserve

- relational invariant execution authority
- typed graph-composition domain-invariant denial contracts on block paths
- declaration legality and support admission as upstream lanes obligations consume

### Complexity / Proof Obligations

- every obligation kind × representative touch in certification matrix
- false-fire/false-miss, replay equivalence, complexity contracts
- exact-zero duplicate rule implementations and adoption manifest residue
- policy-aware mutation gate parity with operating context changes

### Allowed Debt

- store-backed obligation envelope durability remains Milestone `10`/`11` scope
- shipping authority surfaces without full reference adoption may not be claimed
  as closure

### Sequencing Notes

The detailed execution plan lives in [milestone-9.9.md](./milestone-9.9.md).
Twenty phases: vocabulary and relational execution point (1–5); intent
admission integration before surface-specific wiring (6–9); remaining
executors and envelope attachment (10–12); re-homing and bypass audit (13–14);
kernel operating context before birth compose (15–16); adoption (17–18);
docs then certification close (19–20).

### Parallelization Notes

Relational execution point and policy-aware mutation work may overlap where
staffing allows; adoption and certification close strictly last.

### Store Dependency

- Runtime-backed obligation authority is not blocked on `forge-store`.
- Durable obligation envelope persistence is Milestone `10`/`11` scope.

### Acceptance Evidence

- every obligation kind executes in certification matrix across representative
  touches and lanes
- write-batch intent admission carries obligation dispatch — manual
  invariant-pack pre-hook eliminated on covered paths
- primitive construction birth executes compose_graph with obligation routing
- compose, batch, read-family, preview/branch, and declaration-entry lanes
  dispatch with canonical envelopes on receipts and decision traces
- policy-aware mutation gates and preflight sequencing certified
- full topo milestone-one catalog and kernel phase-chain adoption residue
- bypass audit and architectural certification matrix pass

## Milestone 10: Store-Backed Execution, Pushdown, And Historical Parity

### Goal

Close the store-backed execution and historical parity claims that runtime-only
milestones intentionally left open.

### Adversarial Constraint

Store-backed execution and historical restore must preserve the exact same
canonical query meaning, basis identity, and result semantics as the
runtime-backed path for the same admitted capability.

### Why This Milestone Exists

Milestones 4 through 9.3.7 can build the semantic query surface against
runtime truth first. This milestone exists to close the backend-parity boundary
later, once `forge-store` can participate honestly in execution and basis
restore.

### Must Ship

- store-backed execution parity for admitted query families
- honest pushdown of projections, predicates, ordering, and bounded traversal
  where `forge-store` can support them without changing query meaning
- store-backed historical and diff execution over persisted snapshots, deltas,
  and retained history where supported
- diagnostics that distinguish runtime-backed execution from store-backed
  execution and explain any fallback

### Must Preserve

- `forge-store` remains the owner of persistence and durable artifact survival
- pushdown must not change query semantics, policy narrowing, or result shape
- unsupported store capabilities fail explicitly instead of being faked by host
  glue

### Complexity / Proof Obligations

- name store-backed plan admission, pushdown execution, historical restore, and
  store-backed diff execution contracts
- expose exact counters for pushdown admissions, pushdown fallbacks,
  historical restore steps, diff input breadth, and backend parity checks
- prove store-backed parity for execution and admitted historical/diff
  semantics

### Allowed Debt

- unsupported store capability classes may remain explicit `Debt` while
  admitted classes are parity-proven
- backend-shaped query artifacts or meaning-changing pushdown may not ship as
  debt

### Sequencing Notes

This belongs late because it is the first milestone that truly depends on
`forge-store` as an execution substrate rather than merely as future debt.

### Parallelization Notes

Can progress incrementally as `forge-store` milestones land, but final closure
must wait for durable snapshots, retained history, and honest backend parity.

### Store Dependency

This milestone is intentionally blocked on `forge-store`.

It depends materially on:

- canonical commit persistence
- snapshots and point-in-time restore
- structural delta layering and retained-history survivability
- durable basis identity sufficient for runtime/store parity

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Store-Backed Query Durability And Portability Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- store-backed execution returns the same results as runtime-backed execution
  for the same declared basis and query shape
- historical and diff queries survive restart and restore without changing
  meaning

## Milestone 11: Durable Query Artifacts, Saved Queries, And Delivery Continuations

### Goal

Close the durable artifact and continuation claims that cannot be honest until
`forge-store` can persist canonical query artifacts and delivery checkpoints.

### Adversarial Constraint

Saved queries, durable cursors, replay checkpoints, and imported/exported query
artifacts must preserve the same canonical query meaning, parameter semantics,
basis semantics, and continuation point they claimed before restart, transfer,
or reload.

### Why This Milestone Exists

The runtime-backed query model can become structurally sound long before the
store can preserve query artifacts across restart. This milestone exists to
keep artifact durability explicit instead of letting "saved query" or "cursor
resume" become host-local convenience surfaces with undefined semantics.

### Must Ship

- durable saved-query persistence and reload
- durable query-template and scope-composed artifact persistence where the
  platform admits them
- durable cursor/checkpoint persistence for query-shaped delivery where the
  server/runtime contract admits it
- portability of saved-query and delivery artifacts across restart and
  import/export capsule boundaries
- diagnostics that distinguish runtime-backed ephemeral artifacts from
  durable/reloaded artifacts and explain any incompatibility

### Must Preserve

- saved queries remain canonical query artifacts, not backend-shaped blobs
- durable cursors must not outlive or misidentify the truth basis they claim to
  acknowledge
- imported/exported artifacts must preserve semantic identity rather than only
  enough data to "mostly work"
- unsupported durability classes fail explicitly instead of degrading into
  host-local caches

### Complexity / Proof Obligations

- name durable saved-query reload, durable cursor continuation, artifact
  portability, and restart/replay contracts
- expose exact counters for saved-query reload checks, durable cursor resume
  steps, artifact import/export validation, and restart-stable continuation
  proofs
- prove durable artifact identity and continuation parity across restart

### Allowed Debt

- unsupported durable artifact classes may remain explicit `Debt` while
  admitted classes are parity-proven
- host-local saved-query or cursor shims may not ship as debt once the
  milestone claims durability

### Sequencing Notes

This belongs after store-backed execution parity because durable artifacts only
matter once the backend can preserve the bases and execution semantics those
artifacts refer to.

### Parallelization Notes

Can progress incrementally as `forge-store` lands persistent artifact support,
but final closure must wait for restart-stable artifact identity and
continuation semantics.

### Store Dependency

This milestone is intentionally blocked on `forge-store`.

It depends materially on:

- durable artifact storage
- restart-stable schema/basis identity
- durable cursor/checkpoint survival
- import/export and integrity-verifiable artifact identity

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Durable Query Artifact And Continuation Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- persisted saved queries reload to the same canonical query identity
- durable delivery cursors resume to the same query-shaped continuation point
- imported/exported query artifacts preserve canonical query identity and basis
  meaning where the platform admits them
- restart and replay do not alter parameter binding or continuation semantics

## Milestone 12: Blob-Backed Query Delivery And Large-Object Semantics

### Goal

Make blob/media-backed query results first-class, query-shaped, and
basis-honest instead of forcing hosts to smuggle large-object handling around
the query system.

### Adversarial Constraint

Blob-backed projections, structured-content media references, large-object
delivery handles, and upload-associated query results must preserve the same
canonical query meaning, policy masking, basis identity, and replay semantics
as non-blob query surfaces, without degrading into opaque file plumbing.

### Why This Milestone Exists

Most query systems treat blobs as an embarrassing side channel: the query
returns metadata and some other system handles the "real file." Forge Query can
do better, but only once the store can persist large objects, stable handles,
and replay-safe basis identity honestly.

### Must Ship

- blob/media reference projections as first-class query result semantics where
  the schema admits them
- basis-honest delivery handles for large objects and media payloads
- query-shaped blob upload/result association semantics where the platform
  admits upload-backed truth
- policy-aware masking and non-leakage for blob-backed aspects
- diagnostics for unsupported blob delivery classes, expired handles, upload/
  basis mismatch, and portability failures

### Must Preserve

- blobs remain store-owned persisted objects, not query-owned authority
- query meaning must remain identical whether the result shape includes scalar
  fields, structured content, or blob/media references
- delivery handles must never bypass policy masking or basis identity
- uploads must not introduce a second ad hoc query semantics path

### Complexity / Proof Obligations

- name blob-handle derivation, upload association, replay-safe delivery, and
  portability contracts
- expose exact counters for blob handle resolutions, large-object delivery
  admissions, denied blob projections, and upload/query association checks
- prove blob-backed results remain parity-safe with the same canonical query
  and policy basis

### Allowed Debt

- unsupported large-object delivery classes may remain explicit `Debt`
- opaque host-side blob handling may not ship as the claimed query solution

### Sequencing Notes

This belongs after durable query artifacts because blob/media semantics depend
on stable persisted handles, replay-safe basis identity, and durable artifact
ports.

### Parallelization Notes

Can begin once `forge-store` blob/object support is structurally honest, but
final closure should follow durable query-artifact identity so blob handles can
compose with saved queries, history, and delivery continuations honestly.

### Store Dependency

This milestone is intentionally blocked on `forge-store`.

It depends materially on:

- durable blob/object persistence
- replay-safe blob handle identity
- policy-safe object retrieval
- import/export support for blob-backed artifacts where admitted

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Blob-Backed Query Delivery And Upload Parity Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts

- blob/media-backed query results preserve canonical query identity and policy
  masking
- upload-associated query results remain basis-honest and replay-safe
- durable blob handles survive restart/export semantics where the platform
  claims they do

## Milestone 13: Generic And Domain Query Certification

### Goal

Prove the completed query layer under hostile read, history, policy, live
maintenance, durability, and blob-backed delivery scenarios rather than only
through milestone-local demos.

### Adversarial Constraint

Every admitted query capability, across runtime-backed and store-backed paths,
must survive hostile replay, basis variation, policy variation, live
maintenance, durable reload, blob-backed delivery, and domain-specific
workloads without changing canonical query meaning or certification artifacts.

### Why This Milestone Exists

`forge-query` is the main consumer-facing read surface for the stack. It needs
the same certification discipline as the runtimes below it, especially because
small semantic drift here would make the rest of the architecture invisible to
developers.

### Must Ship

- a dedicated `forge-query` certification matrix or
  `test-requirements.md` equivalent if one does not yet exist
- generic certification suites covering at minimum:
  - canonical query normalization parity
  - schema-aware validation rejection
  - snapshot-backed execution parity
  - collection/pagination stability
  - live-promotion equivalence
  - region-scoped live narrowing and stream-contract parity
  - preview-session basis and promotion parity
  - frontier-aware planning and deterministic parallel parity
  - structural correspondence and historical materialization-path parity
  - query-authored workflow/mutation lowering parity
  - unified facade/configuration boundary parity
  - live + policy masking parity
  - historical/diff parity
  - historical + diff + result-shape parity
  - lineage/correspondence query parity
  - lineage + correspondence + branch-scoped comparison parity
  - policy-masking and tenant-scope correctness
  - tenant schema variation + validation + delivery-shape parity
  - store-backed execution parity where admitted
  - durable artifact and continuation parity where admitted
  - blob-backed query delivery parity where admitted
- domain certification suites covering at minimum:
  - geometry/topology neighborhood query truth
  - AI/speculative-branch comparison reads
  - geometry and workflow branch-preview/merge reads plus query-authored merge
    lowering
  - web collection/detail/live workflow reads
  - chip/netlist cone and historical diff reads
- machine-checkable artifact bundles for plans, results, diagnostics, live
  patch evolution, durable reload, and blob-backed delivery

### Must Preserve

- certification must prove existing capability boundaries rather than smuggling
  in missing features
- certification must distinguish runtime-backed and store-backed evidence where
  those paths differ
- query artifacts remain canonical and typed across original execution and
  replay/certification re-run
- beta support claims must not outrun admitted-family, fallback-honesty, and
  certification-matrix proof

### Complexity / Proof Obligations

- name certification bundle construction and replay-verification contracts
- expose exact counters for certification scenarios executed, parity checks
  performed, and capability rows covered from the Vision Coverage Appendix
- prove full appendix coverage rather than milestone-local spot checks only

### Allowed Debt

- none on coverage or machine-checkable certification artifacts
- missing store-backed suites may remain blocked, but they may not be silently
  omitted once Milestones 10 through 12 claim support

### Sequencing Notes

This belongs last because it is for proving the completed query subsystem, not
for discovering what should have been architected earlier.

### Parallelization Notes

Runtime-backed certification can begin before `forge-store` is complete.
Store-backed and durable/blob-backed suites close in parallel as Milestones 10
through 12 become honest.

### Store Dependency

- Runtime-backed certification can begin before `forge-store` is finished.
- Full roadmap completion for this milestone is blocked on `Milestones 10`
  through `12`, because the final certification program must include
  store-backed execution, durable artifact, and blob-backed delivery scenarios
  once those exist.

### Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the `Query Certification Matrix Sufficiency Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  passes with canonical machine-checkable artifacts
- the `Admitted Query Family Boundary Test`, `Fallback Non-Leakage / No Silent
  Widening Test`, `Cross-Feature Composition Matrix Test`, `Reference
  Semantics Test`, `Saved Artifact Semantic Freeze Test`, `Schema Evolution
  Compatibility Test`, `Diagnostic Sufficiency Test`, and `Beta Support Matrix
  Enforcement Test` in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
  pass for the surfaces `forge-query` claims as shipped or beta-supported

- every shipped query capability has at least one hostile certification path
- machine-checkable certification bundles can localize planning, execution,
  policy, live-maintenance, and history failures
- store-backed certification agrees with runtime-backed certification wherever
  both paths are admitted

## Per-Milestone Format

Every milestone in this roadmap uses the same shape:

- `Goal`
- `Adversarial Constraint`
- `Why This Milestone Exists`
- `Must Ship`
  interpreted as:
  - `Surface Primitives`
  - `Semantic Guarantees`
  - `Proof Obligations`
  - `Store-Gated Completion Debt`
- `Must Preserve`
- `Complexity / Proof Obligations`
- `Allowed Debt`
- `Sequencing Notes`
- `Parallelization Notes`
- `Store Dependency`
- `Acceptance Evidence`

The store-dependency section is mandatory here because `forge-query` can make
real progress before `forge-store` is complete, but some completion claims
would be dishonest without durable storage support.

## Completion Standard

`forge-query` is roadmap-complete only when:

- typed query expression, validation, planning, execution, collection
  semantics, live promotion, region-scoped live narrowing, preview-session
  query contexts, frontier-aware planning, structural correspondence,
  query-authored workflow/mutation lowering, unified facade/configuration,
  authoritative mutation evidence, historical reads, lineage traversal,
  composition, policy-aware narrowing, temporal query basis semantics, async
  resource query families, mixed truth/time/async delivery semantics, and
  temporal/async query certification are all shipped
- every store-gated completion item is either shipped through `Milestone 12`
  or still explicitly marked as blocked rather than implied
- runtime-backed and store-backed query execution remain parity-safe for every
  admitted shared capability
- saved queries, durable cursors, and historical execution artifacts remain
  canonical and restart-stable where the platform claims they exist
- generic and domain certification programs both pass with machine-checkable
  evidence

## Vision Coverage Appendix

This appendix is the traceability layer for the rule in
`forge_query_vision.md`: if a capability is named in the vision and not yet
built, it is roadmap work. Every named capability must have a roadmap home,
canonical artifact boundary, proof path, and certification path, even when the
answer is "store-gated" or "shared with another subsystem."

| Vision Capability | Roadmap Home | Canonical Artifact(s) | Acceptance Proof | Certification Path |
| --- | --- | --- | --- | --- |
| Typed composable query expressions | Milestone 1 | Canonical query AST, query digest, facade-normalized query artifact | Equivalent builders normalize identically | Milestone 13 query normalization parity |
| Aspect-aware projection | Milestones 1-2 | Projection descriptors, validated aspect masks | Legal projection lowers deterministically | Milestone 13 normalization + execution parity |
| Filter and predicate expressions | Milestone 2 | Typed predicate nodes | Schema-invalid predicates reject early | Milestone 13 validation rejection |
| Workflow-aware predicates | Milestone 2 | Workflow predicate nodes in canonical query artifact | Workflow predicates validate and normalize canonically | Milestone 13 validation rejection + web workflow domain suite |
| Ordering | Milestones 2 and 4 | Ordering descriptors in plan/result metadata | Ordered reads preserve declared basis | Milestone 13 collection/pagination stability |
| Pagination and opaque cursors | Milestone 4; durable completion in Milestone 11 | Cursor descriptors, page metadata, durable cursors | Page advancement stable for one basis; durable resume later | Milestone 13 collection/pagination stability + store-backed parity |
| Bounded result sets | Milestone 4 | Bound/limit descriptors in plan metadata | Truncation stays explicit and basis-honest | Milestone 13 collection stability |
| Bounded relational materialization | Milestone 4 | Relation materialization descriptors, traversal bounds | Eager materialization stays within declared scope | Milestone 13 execution parity + geometry/chip domain suites |
| Subgraph-scoped queries | Milestone 4 | Scope/traversal boundary descriptors | Traversal breadth remains bounded and explainable | Milestone 13 geometry/chip domain suites |
| Relation traversal expressions | Milestones 1, 2, and 4 | Traversal nodes, validated relation-edge constraints | Illegal traversals reject; legal traversals stay bounded | Milestone 13 validation rejection + domain suites |
| Aggregation queries | Milestone 4 | Aggregation descriptors, grouping metadata | Aggregates stay tied to declared basis | Milestone 13 execution parity |
| Tolerance-aware aggregation | Milestones 4 and 5 | Tolerance policy metadata, live suppression metadata | Suppression does not change aggregate meaning | Milestone 13 live + policy masking parity and aggregation cases |
| Relational rollups | Milestone 4 | Rollup descriptors over relation edges | Rollups remain derived from declared truth basis | Milestone 13 execution parity + domain suites |
| Query-time derived fields | Milestone 4 | Derived-field declarations in canonical query/result shape | Derived fields are planned, not host-postprocessed | Milestone 13 execution parity |
| CDC-shaped output | Milestone 4; durable portability in Milestone 10 | Query-shaped CDC result families, delivery metadata | CDC-shaped output matches ordinary query meaning | Milestone 13 execution parity + store-backed parity |
| Snapshot-backed execution | Milestone 3 | Proof-carrying execution plan, snapshot basis metadata | Same query/context lowers to same plan and result | Milestone 13 snapshot-backed execution parity |
| Type-bound execution / generalized route-model binding | Milestone 3, shared integration with server/cloud | Type-bound execution descriptors tied to canonical plans | Bound descriptors round-trip to same plan as direct execution | Milestone 13 normalization parity; server/cloud integration suites later |
| Live read-to-subscribe promotion | Milestone 5 | Live execution context, query-to-signal lowering metadata | One-shot and live execution preserve semantics | Milestone 13 live-promotion equivalence |
| Incremental result maintenance | Milestone 5 | Query-shaped live patch artifacts, suppression metadata | Live patches preserve ordering/membership/projection | Milestone 13 live-promotion equivalence |
| Query-to-signal bridging | Milestone 5, shared with runtime bridge | Query relevance metadata, bridge-facing invalidation descriptors | Truth changes map to query-shaped maintenance honestly | Milestone 13 live equivalence + bridge-adjacent suites |
| Cross-runtime causal diagnostics | Milestone 9.3.1, shared with runtime bridge, relational, and signal | Bridge causal explanation envelopes, Query causal inspection artifacts, causal materialization receipts | Query inspection explains why an observation changed, was suppressed, was denied, or replayed without direct lower-runtime access | Milestone 9.3.1 causal explanation certification + Milestone 13 diagnostics suites |
| Query basis capability lifecycle | Milestone 9.3.2, shared with relational and runtime bridge | Basis capability envelopes, basis eligibility records, basis use receipts | Observation, mutation, replay, inspection, and materialization consume phase-typed basis proofs instead of raw branch/snapshot identifiers | Milestone 9.3.2 basis lifecycle certification + Milestone 13 branch/history suites |
| Authority-scoped effect execution | Milestone 9.3.3, shared with runtime bridge and relational | Authority-scoped effect plans, lowered effect execution plans, effect execution receipts | Query effects execute only from lowered proof-bearing plans; executors do not re-decide authority, basis, strategy, or artifact policy | Milestone 9.3.3 effect execution certification + Milestone 13 workflow/mutation suites |
| Declared projection consumption | Milestone 9.3.4 | Projection consumption declarations, materialized projection contracts, consumed fact receipts | Consumers use typed facts from declared materializations without reopening source authority | Milestone 9.3.4 projection consumption certification + Milestone 13 projection/domain suites |
| Intent admission decision lattice | Milestone 9.3.5 | Admission decisions, advisory/violation variants, admitted execution handoffs, decision trace envelopes | Query-crossing intents resolve before construction/lowering, and covered runtime-backed paths cross into execution through typed admitted handoffs with structured success, advisory, and violation context | Milestone 9.3.5 admission lattice certification + Milestone 13 diagnostics suites |
| Lower-runtime capability routing | Milestone 9.3.6, shared with runtime bridge, relational, signal, and store | Lower-runtime route plans, boundary execution receipts, lower-runtime boundary envelopes | Lower-runtime contact is capability-routed and receipt-backed; remaining direct paths are explicit compatibility debt | Milestone 9.3.6 boundary routing certification + Milestone 13 support-matrix suites |
| Domain-authored capability contributions | Milestone 9.3.7 | Domain capability contribution requests, admitted domain capability artifacts, canonical runtime materializers, declaration-scoped support traceability artifacts, workflow/continuity/aftermath/explanation contribution families | Domains contribute semantic capability posture through one public Query seam while Query keeps canonical runtime artifact ownership across major category families | Milestone 9.3.7 domain capability certification + Milestone 13 diagnostics/support suites |
| Query-as-beginning platform entry for serious downstream domains | Milestone 9.3.8, shared with forge-proof, forge-foundational, forge-relational, forge-runtime-bridge, and forge-signal | Typed domain entry surfaces, canonical declaration artifacts, progression states, route plans, boundary receipts, boundary envelopes, support/readiness snapshots, orchestration artifacts, certification bundles, and collaboration-entry prerequisites from shared lower-authority hardening specs | Serious downstream domains enter Forge through one Query-owned seam that covers declaration, preparation, continuation, inspection, and lower-authority routing without rebuilding local pseudo-Query layers; later collaboration-facing phases consume retained lower-authority branch, merge, lineage, preview, policy, and strategy posture instead of reopening host glue | Milestone 9.3.8 platform-entry certification + Milestone 13 diagnostics/support/workflow suites |
| Temporal query basis and time-aware subscriptions | Milestone 9.4, shared with runtime bridge and signal temporal execution | Temporal query context descriptors, temporal subscription declaration metadata, bridge temporal basis requests | Truth basis and temporal execution basis stay distinct; time-only deliveries remain query-shaped | Milestone 9.4 temporal/async certification + Milestone 13 live/history suites |
| Time-only query result delivery | Milestone 9.4 | Temporal cause metadata, time-aware delivery batches, previous-value comparison basis | Clock wakes can change admitted query results without raw signal events or ambient timers | Milestone 9.4 temporal/async certification |
| Async/resource query families | Milestone 9.4, shared with runtime bridge and signal async resources | Async resource query declarations, result-state descriptors, completion-causality artifacts | Stale, cancelled, retried, failed, and superseded completions remain query-shaped and basis-bound | Milestone 9.4 temporal/async certification |
| Mixed truth/time/async delivery | Milestone 9.4 | Mixed-cause delivery metadata, cause ordering receipts, coalescing/suppression diagnostics | Host event arrival order cannot change canonical query-shaped delivery meaning | Milestone 9.4 temporal/async certification + Milestone 13 cross-feature suites |
| Temporal/async query certification | Milestone 9.4 | Temporal/async certification bundles, support matrix rows, diagnostic sufficiency bundles, reference workload artifacts | Every advertised runtime-backed temporal/async query family has hostile proof and fail-closed unsupported-neighbor coverage | Milestone 9.4 completion itself + Milestone 13 query certification matrix |
| Scope/template composition hardening | Milestone 9.5 | Canonical scope-expansion artifacts, template-instantiation artifacts, scope/template support rows | Reusable composition stays canonical and support-typed instead of remaining admitted debt | Milestone 9.5 debt-close certification + Milestone 13 composition suites |
| Core view-shape family hardening | Milestone 9.5 | Production-ready `table`, `detail`, inspector-detail, and grouped-view-shape artifacts | Core view families stop advertising admitted-but-debt runtime-backed posture | Milestone 9.5 debt-close certification + Milestone 13 view-shape suites |
| Grouped composition closure | Milestone 9.5 | Grouped template/composition artifacts, grouped planning support profiles | Grouped planning no longer carries explicit composition debt in ordinary product docs or support surfaces | Milestone 9.5 debt-close certification |
| Retained-artifact projection consumption hardening | Milestone 9.5 | Retained derived-artifact and live-artifact source-family bindings, typed fact receipts | Projection consumption remains the ordinary typed fact lane rather than special-case pack/bind/decode folklore | Milestone 9.5 debt-close certification + Milestone 13 projection/domain suites |
| Preserved temporal/async reuse-neighbor closure | Milestone 9.5 | Inspector/grouped preserved reuse artifacts, preserved runtime-backed semantics, reuse digests | Covered temporal/async reuse neighbors carry merged `9.4` meaning across the full runtime-backed reuse surface | Milestone 9.5 debt-close certification + Milestone 13 cross-feature suites |
| Raw runtime read bootstrap hardening | Milestone 9.5 | Valid bridge-backed read-runtime bootstrap artifact, raw runtime bootstrap support posture, hostile read-runtime harness entry surface | Hostile runtime-backed read tests can reach the ordinary raw read lane without custom bridge-backed assembly folklore | Milestone 9.5 debt-close certification + Milestone 13 runtime/read harness suites |
| Region-scoped live invalidation | Milestone 5.1 | Region/partition-aware invalidation metadata, locality predicates, region-scoped suppression metadata | Live narrowing stays below broad aspect scope where lower-runtime locality contracts admit it | Milestone 13 live equivalence + geometry domain suites |
| Change-stream-backed delivery contracts | Milestones 5.1, 9, and 11 | Stream-lowered delivery declarations, delivery metadata, durable stream checkpoints | Query-shaped delivery lowers into formal stream contracts without semantic drift | Milestone 13 delivery-shape + durable continuation parity |
| Preview-session query contexts | Milestone 5.2 | Preview-session basis metadata, preview-lifecycle metadata, preview/promotion comparison artifacts, preview-live admission and drift artifacts | Preview-bound queries preserve explicit basis and lifecycle meaning, and preview-live remains basis-explicit under maintenance, denial, and explicit rebind | Milestone 13 branch/history/workflow suites |
| Frontier-aware planning | Milestone 5.3 | Frontier-derived planning metadata, breadth posture, parallel-admission posture | Planner consumes lower-runtime frontier posture without executor rediscovery | Milestone 13 planning parity + performance suites |
| Deterministic parallel admission | Milestone 5.3 | Parallel-admission decisions on planned routes, serial fallback diagnostics | Serial and parallel admitted lanes remain semantically identical | Milestone 13 planning parity + performance suites |
| Branch-scoped reads | Milestone 6 | Branch-targeting query context metadata | Same query shape runs against different branches honestly | Milestone 13 historical/diff parity |
| Time-travel reads | Milestone 6; durable completion in Milestone 10 | Historical basis descriptors, snapshot/commit targets | Historical basis is explicit and parity-safe | Milestone 13 historical/diff parity + store-backed parity |
| Diff queries | Milestone 6 | Structured diff query artifacts, comparison-basis metadata | Diff results align with declared projection/scope | Milestone 13 historical + diff + result-shape parity |
| Branch comparison views | Milestone 6 with Milestone 8 view-shape semantics | Comparison basis metadata, view-shape metadata | Branch comparisons preserve basis identity and result meaning | Milestone 13 historical + diff parity |
| Historical evaluation contracts | Milestone 5.4 with completion in Milestone 6 | Historical materialization-path metadata, compatibility/admission artifacts | Historical reads stay explicit about how truth was materialized and whether the request was honestly admissible | Milestone 13 historical/diff parity + diagnostics suites |
| Lineage traversal queries | Milestone 7 | Lineage traversal descriptors, lineage-basis metadata | Lineage results stay typed and explainable | Milestone 13 lineage/correspondence parity |
| Correspondence queries | Milestones 5.4 and 7 | Lineage-backed and structural-fingerprint-backed correspondence descriptors, ambiguity/rejection metadata | Ambiguous correspondence stays explicit and advisory correspondence never silently becomes continuity | Milestone 13 lineage/correspondence parity |
| Inspector-pattern detail with live aspect projection | Milestones 5 and 8 | Inspector/detail view-shape metadata, aspect-focused live patch artifacts | Inspector live projection proves narrow invalidation | Milestone 13 live equivalence + view-shape cross-feature suites |
| View shapes: table/detail | Milestone 8 | View-shape descriptors, delivery/live-patch metadata | View shape affects planning and live semantics, not only typing | Milestone 13 web/detail workflow suites |
| View shapes: kanban/grouped | Milestone 8 | Grouped view metadata, group-splice patch artifacts | Group membership changes preserve view semantics | Milestone 13 view-shape cross-feature suites |
| View shapes: timeline/chart | Milestones 4 and 8 | Temporal/grouping metadata, tolerance/suppression metadata | Temporal/chart semantics stay explicit and basis-honest | Milestone 13 historical/diff + aggregation suites |
| Named scopes | Milestone 8 | Scope fragments in canonical query artifact | Scope expansion equals direct construction | Milestone 13 normalization parity |
| Query templates with parameter slots | Milestone 8 | Template descriptors, parameter binding metadata | Parameterized instantiation preserves canonical meaning | Milestone 13 normalization parity |
| Saved and named query definitions | Milestone 8; durable completion in Milestone 11 | Saved-query canonical artifact, durable saved-query records | Reloaded saved query preserves identity and meaning | Milestone 13 normalization parity + store-backed parity |
| Result shape declarations for delivery contracts | Milestones 1 and 9 | Typed result shapes, delivery-shape metadata | Delivery metadata remains identical to canonical masked/projected result | Milestone 13 delivery-shape parity |
| Policy-aware aspect masking | Milestone 9 | Policy masks in plan metadata | Masked aspects never enter execution plan | Milestone 13 live + policy masking parity |
| Branch-level access scoping | Milestone 9 | Branch-access validation metadata | Denied branches fail before reads execute | Milestone 13 policy/tenant correctness |
| Automatic tenant branch scoping | Milestone 9 | Tenant branch-resolution metadata | Tenant context narrows truth basis explicitly | Milestone 13 policy/tenant correctness |
| Tenant-scoped schema awareness | Milestone 9 | Tenant schema-basis metadata, tenant-aware validation artifacts | Validation uses tenant schema rather than a global default | Milestone 13 tenant schema variation + validation parity |
| Graph-native relationship proofs | Milestone 9, shared with schema/platform policy authority | Relationship-proof predicate/query nodes, denial metadata | Broken proof chains deny explicitly without data leakage | Milestone 13 policy/tenant correctness |
| Multi-tenant query architecture | Milestone 9; durable completion in Milestone 11 | Tenant basis metadata, durable tenant/query artifacts | Tenant-scoped reads remain parity-safe across restart where supported | Milestone 13 policy/tenant correctness + store-backed parity |
| Structured content aspect queries | Milestone 2; live/update consequences in Milestones 5 and 10 | Structured content projection/predicate descriptors | Structured content legality and live narrowing stay explicit | Milestone 13 validation rejection + live equivalence |
| Query planning and optimization | Milestone 3; store-aware completion in Milestone 10 | Proof-carrying execution plans, store pushdown diagnostics | Planner lowers once; executor does not rediscover semantics | Milestone 13 snapshot parity + store-backed parity |
| Delivery contracts for integrations | Milestones 4, 9, and 11 | CDC/result delivery metadata, durable delivery cursors | Delivery contracts remain query-shaped and basis-honest | Milestone 13 delivery-shape + store-backed parity |
| Query-authored mutation intents | Milestone 5.5 | Mutation-intent declarations, lowered commit-strategy request descriptors, context-derived observation artifacts | Query-authored mutation workflows lower into relational authorities without semantic drift | Milestone 13 workflow/mutation suites |
| Branch-native workflow orchestration | Milestones 5.2 and 5.5 | Preview/compare/merge workflow declarations, conflict inspection artifacts, post-merge inspection artifacts | Branch workflows stay inside the query framework while preserving lower-crate authority boundaries | Milestone 13 workflow/mutation + branch suites |
| Query-triggered writeback declarations | Milestone 5.5 | Writeback-trigger declarations, lowered bridge writeback descriptors, causality/admission metadata | Query-triggered writeback stays declaration-owned by query and execution-owned by the bridge | Milestone 13 workflow/mutation + diagnostics suites |
| Runtime authoritative mutation evidence | Runtime Authoritative Mutation Evidence Gate | Declared/resolved target evidence, batch/session authority evidence, existing-truth binding descriptors, naming/continuity evidence bundles | Downstream write-heavy domains receive authority evidence through the public facade without local target-recovery glue | Milestone 13 workflow/mutation + diagnostics suites |
| Unified application facade | Milestone 5.6 | Authority-preserving public facade surface, capability registry, support metadata | Domain developers can use query as the daily-driver import without erasing lower-crate ownership | Milestone 13 support-matrix + certification suites |
| Unified runtime configuration | Milestone 5.6 | Sectioned `ForgeQueryConfig`, subsystem-owned config sections, capability-gated config metadata | Unified config remains architecture-shaped rather than bag-shaped | Milestone 13 support-matrix + diagnostics suites |
| Store-backed pushdown and execution parity | Milestone 10 | Store-backed plan variants, fallback diagnostics | Store-backed results equal runtime-backed results | Milestone 13 store-backed execution parity |
| Durable saved queries and cursors | Milestone 11 | Durable saved-query records, durable cursor/checkpoint records | Restart preserves canonical identity and continuation point | Milestone 13 durable artifact parity |
| Import/export portability of query artifacts | Milestone 11 | Portable query artifact bundles and basis identity | Imported/exported artifacts preserve canonical meaning | Milestone 13 durable artifact parity |
| Blob/media-backed query delivery | Milestone 12 | Blob/media reference projections, durable delivery handles, upload-associated result metadata | Blob-backed results preserve canonical query meaning and policy masking | Milestone 13 blob-backed delivery parity |
| Query certification matrix | Milestone 13 | Certification bundles for plans, results, diagnostics, live evolution, durable reload, and blob-backed delivery | Every capability has hostile proof, not just local demos | Milestone 13 completion itself |

If a future query capability is added to `forge_query_vision.md`, this appendix
must gain a row in the same patch or the roadmap is incomplete.

## Companion Documents

- [forge_query_vision.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/forge_query_vision.md)
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/test-requirements.md)
- [milestone-9.3.1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/milestone-9.3.1.md)
- [milestone-9.3.2.md](./milestone-9.3.2.md)
- [milestone-9.3.3.md](./milestone-9.3.3.md)
- [milestone-9.3.4.md](./milestone-9.3.4.md)
- [milestone-9.3.5.md](./milestone-9.3.5.md)
- [milestone-9.3.6.md](./milestone-9.3.6.md)
- [milestone-9.3.7.md](./milestone-9.3.7.md)
- [milestone-9.4.md](./milestone-9.4.md)
- [milestone-9.5.md](./milestone-9.5.md)
- [runtime-api-public-stabilization-plan.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/runtime-api-public-stabilization-plan.md)
- [runtime-authoritative-mutation-evidence-plan.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-query/runtime-authoritative-mutation-evidence-plan.md)
- [forge_runtime_bridge_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/forge_runtime_bridge_roadmap.md)
- [forge_relational_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_roadmap.md)
- [forge_store_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_roadmap.md)
- [MENTALITY.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/MENTALITY.md)
- [architectural_guidelines.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/architectural_guidelines.md)
- [domain_standards.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/domain_standards.md)
- [performance_guidelines.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/performance_guidelines.md)
