# Milestone 9.10 Engineering Spec: Graph Read Access Planning And Automatic Index Provisioning

> **Status:** Draft
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Primary predecessor:** [milestone-9.9.md](./milestone-9.9.md)
>
> **Purpose:** make declared graph reads compile into proof-bearing,
> budget-admitted access plans so covered read surfaces cannot perform hidden
> N+1 traversal, cannot build unbounded in-memory indexes, and cannot teach AI
> agents a stronger graph/index mental model than the runtime can enforce.

## Goal

Forge Query must derive graph read access needs from the declared read shape,
plan the required adjacency/index posture before execution, and either execute
through an admitted bounded plan or fail with a typed posture that explains the
missing support, budget excess, or required materialization lane.

The product target is not "cache more graph data." The product target is:

```text
read declaration
-> canonical graph read access shape
-> selectivity and cardinality estimate
-> required access/index posture
-> admitted inline, paged streaming, persistent-index-required,
   store-backed-index-required, async-materialized,
   access-capability-registration-required, or denied
-> receipt proving no hidden N+1 and no unbounded background indexing
```

## Why This Milestone Exists

The current docs correctly point agents away from caller-owned graph loops and
toward Query-owned graph/index views, but the runtime is not yet robust enough
to honor that promise across broad boolean predicates, multi-relation edge
walks, dense graph neighborhoods, policy/tenant narrowing, and wild traversal
shapes. Milestone `9.9` gives read/touch descriptors and obligation dispatch
authority. Milestone `9.10` uses that foundation to make graph access planning
itself a first-class Query authority boundary before store-backed execution
inherits the runtime-backed read model.

## Governing Summaries

- `MENTALITY.md`: protect the adversarial constraint first; the spec must build
  the foundation that survives worst-case graph reads instead of shipping a
  convenience cache and hoping to optimize later.
- `arch_laws.md`: semantic intent must compile into execution strategy,
  applicability must be pre-solved at entry, lowered plans must be the only
  executor input, and cost boundaries must be explicit in API shape.
- `composition_laws.md`: planning, estimation, admission, execution,
  diagnostics, and certification must live in named files and functions rather
  than collapsing into broad read helpers.
- `domain_structure_laws.md`: authoritative truth, derived indexes, cached
  access structures, and diagnostic artifacts need separate structural homes;
  graph/index projections must be visibly derived and rebuildable.
- `perf_laws.md`: breadth must be bounded by semantic delta, expensive work
  must reject before construction, access patterns must reflect traversal
  locality, and every performance claim needs counters.
- `forge_query_roadmap.md`: Query owns typed expression, lowering, result
  shaping, and planning while lower runtimes keep truth authority; this
  milestone belongs before store-backed execution so store pushdown inherits a
  complete access-plan contract rather than discovering it later.

## Adversarial Constraint

For every covered graph read declaration, including broad boolean predicates,
multi-relation edge walks, dense frontier expansion, policy/tenant narrowed
reads, relationship-proof reads, reusable read families, live-promoted reads,
and preview/branch reads, Query must decide the access posture before expensive
execution begins. It must be impossible for covered surfaces to:

- perform caller-owned N+1 edge lookups
- materialize unbounded adjacency, frontier, visited, proof, or result buffers
- hide broad graph traversal behind cheap-looking APIs
- build background indexes without a lifecycle, budget, and receipt
- confuse persistent, runtime, ephemeral, streaming, async, and denied postures
- execute with a graph access plan that cannot be replayed, inspected, and
  compared through structural counters

## Product Decision Lock

- Automatic index provisioning means **automatic plan derivation**, not
  automatic eager materialization.
- Inline graph reads may use existing indexes, admitted runtime indexes, or
  bounded ephemeral indexes only within certified memory and breadth budgets.
- Dangerous broad reads must return typed outcomes such as
  `RequiresPersistentIndex`, `RequiresAsyncMaterialization`,
  `AdmittedPagedStreaming`, or `DeniedBudgetExceeded` before the hot path burns
  time or RAM.
- No graph read edge case may be recorded as an unnamed gap or catch-all
  bucket. Every case must have a named admission posture, required-capability
  posture, async/materialized posture, store-owned capability posture, or typed
  denial with receipt evidence.
- Store-owned durable index work must appear as a typed required-capability
  posture with an owning milestone, not as a missing runtime edge case.
- AI-facing docs must teach the exact boundary: Query learns access needs from
  declarations, but Query does not allocate the universe to satisfy an unsafe
  declaration.

## Phase Plan

### Phase 1: Graph Read Access Shape Vocabulary

Freeze a sealed `ForgeQueryGraphReadAccessShape` derived from canonical read
graphs, not from executor observations. The shape must encode root posture,
scope class, relation directions, traversal operators, maximum depth, frontier
fanout posture, predicate families, ordering, result-shape pressure, policy/
tenant/relationship-proof narrowing, basis posture, and whether the read is
one-shot, reusable, live-promoted, preview, or branch-scoped.

**Relevant subsystems**
- `crates/forge-query/src/runtime/workspace_queries.rs`
- `crates/forge-query/src/runtime/runtime_read_intents.rs`
- `crates/forge-query/src/runtime/surface/read_composition.rs`
- `crates/forge-query/src/policy_narrowing/`
- `crates/forge-query/src/relationship_proof/`

**Relevant APIs**
- `ForgeQueryReadGraph`
- `ForgeQueryReadFamily`
- `ForgeQueryReadReceipt`
- new: `ForgeQueryGraphReadAccessShape`
- new: `ForgeQueryGraphReadAccessShapeDigest`

**Warnings**
- Do not infer access shape from elapsed execution behavior; that turns a plan
  into a profiler artifact.
- Do not collapse relation direction into relation kind. Direction is a cost
  boundary and must survive canonicalization.

**Test requirements**
- Adversarial equivalence: semantically equivalent declarations with different
  builder ordering produce the same access-shape digest.
- Adversarial rejection: changing relation direction, maximum depth, predicate
  family, policy basis, or result-shape breadth changes the digest or produces
  a typed incompatibility.
- Adversarial locality: a one-hop direct edge read and a bounded successor walk
  over the same relation do not collapse into one access shape.

**Engineering decisions**
- Access shape is derived after read graph canonicalization and before
  execution admission.
- Shape fields are private; only Query-owned lowering can construct the sealed
  artifact.

**Open questions**
- None.

### Phase 2: Boolean Predicate Normalization And Selectivity Inputs

Normalize boolean predicate expressions into proof-bearing selectivity input
trees before graph expansion. The planner must know which branches are anchored,
which are broad, which can intersect before traversal, and which OR-heavy
branches require streaming or materialization.

**Relevant subsystems**
- `crates/forge-query/src/query_ast/`
- `crates/forge-query/src/planning/`
- `crates/forge-query/src/runtime/surface/read_composition.rs`

**Relevant APIs**
- predicate expression nodes
- `ForgeQueryReadGraph`
- new: `ForgeQueryBooleanSelectivityShape`
- new: `ForgeQueryPredicateSelectivityClass`

**Warnings**
- Do not evaluate predicates to discover selectivity. This phase produces
  planner input, not data-plane filtering.
- Unknown selectivity is not optimistic. Unknown broadness must classify as
  risky until a later phase proves otherwise.

**Test requirements**
- Adversarial equivalence: `A AND (B AND C)` and `(C AND A) AND B` normalize to
  identical selectivity shape when the predicates are semantically equivalent.
- Adversarial rejection: broad OR over traversal-bearing branches does not
  lower to inline execution unless it has a budgeted posture; otherwise it
  returns a typed streaming, materialization, required-capability, or denial
  posture.
- Adversarial ordering: selective anchored predicates are visible as eligible
  pre-traversal constraints, while broad predicates stay explicitly broad.

**Engineering decisions**
- Normalization preserves enough branch identity for diagnostics and receipts.
- The selectivity tree is an input to planning only; it is not an executor-side
  predicate interpreter.

**Open questions**
- None.

### Phase 3: Required Index Set Derivation

Derive a `ForgeQueryRequiredGraphIndexSet` from access shape and selectivity
inputs. It must name directional adjacency requirements, reverse adjacency,
relation-kind filters, lifecycle filters, predicate indexes, ordering support,
dedup/visited requirements, and relationship-proof support without deciding
whether those structures already exist.

**Relevant subsystems**
- `crates/forge-query/src/runtime/support/`
- `crates/forge-query/src/planning/`
- `crates/forge-relational/src/validation/`

**Relevant APIs**
- new: `ForgeQueryRequiredGraphIndexSet`
- new: `ForgeQueryRequiredGraphIndexRow`
- new: `ForgeQueryGraphIndexRequirementKind`
- `ForgeQueryGraphReadAccessShape`

**Warnings**
- Requirement derivation is not index construction. Mixing these steps hides
  missing support until allocation time.
- Do not model indexes as strings. Requirement rows need typed relation,
  direction, predicate, and lifecycle fields.

**Test requirements**
- Adversarial parity: equivalent access shapes derive byte-identical required
  index sets with stable row ordering.
- Adversarial localization: changing one traversal relation changes only the
  relevant requirement row and receipt digest.
- Adversarial completeness: every traversal-bearing covered read derives at
  least one directional adjacency requirement or returns a typed
  required-capability or denial posture.

**Engineering decisions**
- Requirement rows include a rebuild basis that proves the index is derived
  from authoritative truth.
- Requirement set digest participates in read receipt identity.

**Open questions**
- None.

### Phase 4: Access Cost Model And Budget Contract

Build a runtime-backed cost model that estimates frontier breadth, edge touches,
candidate roots, intermediate set size, index bytes, result bytes, proof bytes,
and allocation lifecycle before execution begins.

**Relevant subsystems**
- `crates/forge-query/src/planning/`
- `crates/forge-query/src/runtime/support/`
- `crates/forge-query/src/runtime/state_snapshot.rs`

**Relevant APIs**
- new: `ForgeQueryGraphReadCostEstimate`
- new: `ForgeQueryGraphReadBudget`
- new: `ForgeQueryGraphReadBudgetClass`
- new: `ForgeQueryGraphReadComplexityContract`

**Warnings**
- Elapsed time is not the contract. Structural counters are the contract.
- Estimates may be conservative; they may not under-report unknown broadness to
  admit unsafe execution.

**Test requirements**
- Adversarial budget: a dense boolean traversal with low-selectivity predicates
  predicts a broad posture and cannot enter inline indexed execution.
- Adversarial equivalence: two equivalent access shapes over the same runtime
  statistics produce identical cost-estimate digests.
- Adversarial memory: estimated index bytes include adjacency, reverse
  adjacency, frontier, visited, dedup, proof, and result buffers when relevant.

**Engineering decisions**
- Default inline ephemeral index memory budget is small and explicit; raising it
  requires a typed budget admission in a later phase.
- Cost model status distinguishes `Measured`, `Estimated`, `UnknownConservative`,
  and `RequiresCapabilityRegistration`.

**Open questions**
- None.

### Phase 5: Access Admission Outcomes

Freeze typed access admission outcomes and an exhaustive graph read access-case
registry so budget, capability, store-ownership, and support decisions are
visible before execution. The system must be able to admit inline indexed,
paged streaming, prebuilt/persistent-index-backed, async materialized, require
store-backed capability, require access capability registration, or deny with a
structured reason.

**Relevant subsystems**
- `crates/forge-query/src/intent_admission/`
- `crates/forge-query/src/runtime/surface/read_composition.rs`
- `crates/forge-query/src/runtime/support/`

**Relevant APIs**
- new: `ForgeQueryGraphReadAccessAdmission`
- new: `ForgeQueryGraphReadAccessAdmissionPosture`
- new: `ForgeQueryGraphReadAccessDenial`
- new: `ForgeQueryGraphReadBudgetExceededDenial`
- new: `ForgeQueryGraphReadAccessCase`
- new: `ForgeQueryGraphReadAccessCaseRegistry`
- new: `ForgeQueryGraphReadRequiredCapabilityOwner`

**Warnings**
- `DeniedBudgetExceeded` is not a soft warning. Covered execution must not run
  unless the caller re-declares the work through an admitted posture.
- `RequiresPersistentIndex` and `RequiresAsyncMaterialization` must be product
  postures, not string messages.
- Generic catch-all admission variants are prohibited. Unknown or
  not-yet-admitted shapes must classify as a named required capability, a
  store-owned requirement, or a typed denial.

**Test requirements**
- Adversarial rejection: a read whose estimated frontier exceeds inline budget
  returns `DeniedBudgetExceeded` or `RequiresAsyncMaterialization` before any
  edge scan counter increments.
- Adversarial parity: the same admitted read through one-shot and reusable
  family fronts produces the same access admission digest.
- Adversarial DX: denial carries required index rows, exceeded budget fields,
  and suggested posture without exposing internal executor topology.
- Adversarial edge cases: every public read operator, relation direction,
  predicate family, basis posture, live/preview/branch posture, and
  relationship-proof combination maps to an access-case registry row before any
  executor can run.

**Engineering decisions**
- Admission outcome attaches to the read receipt even on denied paths.
- Denials lower through typed stop classes rather than message matching.
- The access-case registry is the implementation checklist for this milestone:
  adding a new graph read capability requires adding the case row, tests,
  receipt posture, support-row behavior, and docs row in the same phase.

**Open questions**
- None.

### Phase 6: Existing Index Inventory And Support Rows

Expose runtime and lower-runtime graph index inventory through Query support
rows. Query must be able to answer whether a required index already exists,
whether it is runtime-maintained, lower-runtime-owned, persistent, ephemeral,
store-owned-required, access-capability-registration-required, or denied by a
typed support posture.

**Relevant subsystems**
- `crates/forge-query/src/runtime/support/`
- `crates/forge-query/src/facade/`
- `crates/forge-relational/src/`

**Relevant APIs**
- new: `ForgeQueryGraphIndexInventory`
- new: `ForgeQueryGraphIndexSupportRow`
- new: `ForgeQueryGraphIndexPosture`
- `ForgeQueryRequiredGraphIndexSet`

**Warnings**
- Inventory is not authority over truth. Indexes remain derived structures.
- Do not report `Verified` for a row unless rebuild basis, invalidation basis,
  and access complexity are certified.

**Test requirements**
- Adversarial parity: the same runtime assembly publishes identical inventory
  digests across repeated support-report calls.
- Adversarial drift: a missing required index localizes to the exact
  requirement row and cannot masquerade as broad support.
- Adversarial authority: inventory rows cannot be constructed by consumers or
  patched through facade escape hatches.

**Engineering decisions**
- Inventory rows record lifecycle owner, derivation basis, invalidation basis,
  supported relation direction, and complexity contract.
- Store-backed persistent indexes must be visible as
  `RequiresStoreBackedPersistentIndex` rows with owning milestones; runtime-
  backed admission must not pretend they exist.

**Open questions**
- None.

### Phase 7: Bounded Ephemeral Index Provisioning

Build bounded ephemeral index provisioning for small admitted graph reads. The
runtime may build transaction/read-scope adjacency or predicate-support
structures only when the budget admits the allocation and lifecycle scope is
explicit.

**Relevant subsystems**
- `crates/forge-query/src/runtime/`
- `crates/forge-query/src/planning/`
- `crates/forge-query/src/runtime/surface/read_composition.rs`

**Relevant APIs**
- new: `ForgeQueryEphemeralGraphIndexPlan`
- new: `ForgeQueryEphemeralGraphIndexScope`
- new: `ForgeQueryEphemeralGraphIndexReceipt`

**Warnings**
- Ephemeral does not mean untracked. Allocation lifecycle, byte budget, and
  rebuild basis must be visible.
- Do not keep ephemeral indexes alive past their admitted scope because a later
  read "might use them."

**Test requirements**
- Adversarial budget: ephemeral provisioning rejects before allocation when the
  estimated index bytes exceed the inline ephemeral budget.
- Adversarial cleanup: after the read scope ends, ephemeral index lifecycle
  counters prove no orphan resource remains.
- Adversarial equivalence: rebuilding the same bounded ephemeral index from the
  same snapshot produces the same index digest and access receipt.

**Engineering decisions**
- Ephemeral indexes are read-scope or family-execution-scope resources, not
  authoritative state and not hidden caches.
- The receipt records actual allocated bytes, touched nodes, touched edges, and
  lifecycle scope.

**Open questions**
- None.

### Phase 8: Paged Streaming Frontier Execution

Add an admitted paged streaming posture for reads that are too broad for inline
materialization but can be executed safely as bounded frontier slices with
opaque cursors and checkpointed progress.

**Relevant subsystems**
- `crates/forge-query/src/runtime/workspace_queries.rs`
- `crates/forge-query/src/runtime/surface/`
- `crates/forge-query/src/runtime/state_snapshot.rs`

**Relevant APIs**
- new: `ForgeQueryGraphReadStreamingPlan`
- new: `ForgeQueryGraphReadFrontierCursor`
- new: `ForgeQueryGraphReadStreamingReceipt`
- `ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming`

**Warnings**
- Streaming must not silently change result semantics. Page boundaries are
  delivery mechanics, not query meaning.
- Cursor identity must be basis-bound and opaque; callers cannot forge frontier
  continuation state.

**Test requirements**
- Adversarial convergence: executing all pages of a streaming plan produces the
  same canonical result set as an admitted full execution on a small reference
  graph.
- Adversarial memory: max resident frontier and visited buffers stay within the
  page budget across a dense graph scenario.
- Adversarial cursor: replaying, skipping, or forging a cursor fails with typed
  basis/continuation denial.

**Engineering decisions**
- Streaming is a distinct execution posture with receipts; it is not a fallback
  hidden behind ordinary inline reads.
- Page receipts aggregate into one access-plan digest for replay and inspection.

**Open questions**
- None.

### Phase 9: Persistent Index Requirement Declaration

Ship declared persistent-index requirements for read families whose safe
execution depends on prebuilt adjacency, predicate, ordering, or relation
support. Before `forge-store` owns durable index persistence, runtime-backed
persistent requirements must remain support-row honest.

**Relevant subsystems**
- `crates/forge-query/src/runtime/builder.rs`
- `crates/forge-query/src/runtime/support/`
- `crates/forge-query/docs/authoring/read-composition.md`

**Relevant APIs**
- new: `ForgeQueryPersistentGraphIndexRequirement`
- new: `ForgeQueryGraphReadFamilyIndexContract`
- `ForgeQueryReadFamily`

**Warnings**
- Persistent-index requirement is not a promise that durable storage exists
  before store-backed milestones close.
- Do not let callers attach arbitrary strings as index names; index identity is
  derived from typed requirement rows.

**Test requirements**
- Adversarial rejection: a read family requiring a persistent index cannot
  execute inline when the inventory lacks the matching admitted row.
- Adversarial equivalence: identical read-family declarations derive identical
  persistent index requirement digests.
- Adversarial support honesty: store-backed persistent index rows return
  `RequiresStoreBackedPersistentIndex` with owner, basis, and missing-capability
  evidence until the owning store milestone admits them.

**Engineering decisions**
- Runtime-backed persistent requirements can resolve to existing runtime indexes
  or produce `RequiresPersistentIndex`.
- Durable persistence of indexes remains Milestone `10`/`11` scope unless
  `forge-store` supplies the admitted row.

**Open questions**
- None.

### Phase 10: Async Materialized Graph Read Jobs

Add an explicit async materialization posture for legitimate large graph reads.
The inline read path must be able to return `RequiresAsyncMaterialization`, and
the async path must run under checkpoints, cancellation, memory caps, and
receipt-backed progress.

**Relevant subsystems**
- `crates/forge-query/src/application/declaration_entry_orchestration/`
- `crates/forge-query/src/runtime/runtime_read_intents.rs`
- `crates/forge-query/src/runtime/surface/`

**Relevant APIs**
- new: `ForgeQueryGraphReadMaterializationRequest`
- new: `ForgeQueryGraphReadMaterializationJob`
- new: `ForgeQueryGraphReadMaterializationProgress`
- new: `ForgeQueryGraphReadMaterializationReceipt`

**Warnings**
- Async materialization is not an escape hatch to bypass budgets; it has its own
  stricter lifecycle and cancellation contract.
- Materialized output is derived state and must remain rebuildable from
  authoritative truth plus the admitted plan.

**Test requirements**
- Adversarial cancellation: cancelling a large graph materialization releases
  allocated frontier/index resources and preserves a typed cancellation receipt.
- Adversarial replay: re-running a completed materialization from the same
  snapshot and access plan produces the same materialization digest.
- Adversarial denial: a large read cannot smuggle into async execution without
  an admitted materialization request and budget envelope.

**Engineering decisions**
- Async jobs are declaration/admission-owned Query artifacts, not host task
  handles.
- Job progress counters include touched edges, frontier pages, allocated bytes,
  emitted rows, and checkpoint count.

**Open questions**
- None.

### Phase 11: Ordinary Read Surface Integration

Integrate access planning into `compose_read`, reusable read families,
read-family intent execution, basis-context execution, and helper fronts. The
executor must consume an admitted access plan and must not rediscover strategy
from raw read graph shape.

**Relevant subsystems**
- `crates/forge-query/src/runtime/workspace_queries.rs`
- `crates/forge-query/src/runtime/runtime_read_intents.rs`
- `crates/forge-query/src/runtime/surface/read_composition.rs`

**Relevant APIs**
- `compose_read`
- `define_read_family`
- `execute_read_family`
- `execute_read_family_in_basis_context`
- `read_family_intent`
- `ForgeQueryGraphReadAccessAdmission`

**Warnings**
- Do not integrate only the shortest happy path. Reusable families and intent
  fronts are covered surfaces.
- The executor must not branch over access strategy beyond consuming the lowered
  plan variant.

**Test requirements**
- Adversarial parity: `compose_read`, `execute_read_family`, and
  `read_family_intent(...).execute()` produce equivalent access-plan evidence
  for the same declaration and basis.
- Adversarial rejection: unregistered, missing-capability, or over-budget access
  shapes stop at admission and never increment edge-scan execution counters.
- Adversarial no-N+1: covered helper fronts prove exact-zero per-result
  neighbor lookup loops through counters and bypass audit.

**Engineering decisions**
- `ForgeQueryReadReceipt` gains an access-plan summary and complexity counters.
- Denied read surfaces still publish an access-admission envelope.

**Open questions**
- None.

### Phase 12: Live Read And Subscription-Adjacent Access Planning

Carry graph read access planning into live-promoted reads and subscription-
adjacent read declarations where the same canonical read meaning is maintained
over time. The runtime must distinguish one-shot access cost from maintenance
cost.

**Relevant subsystems**
- `crates/forge-query/src/runtime/tests/live_state/`
- `crates/forge-query/src/runtime/surface/`
- `crates/forge-query/src/subscription_*` surfaces

**Relevant APIs**
- live read intent surfaces
- subscription family selection diagnostics
- new: `ForgeQueryLiveGraphReadAccessPlan`

**Warnings**
- Do not claim live maintenance parity from one-shot planning alone.
- Live access indexes must not couple mutation cost to every derived view unless
  admitted by support rows.

**Test requirements**
- Adversarial parity: one-shot and live-promoted reads with the same declaration
  share compatible access-shape and required-index digests.
- Adversarial maintenance: mutation updates touch only semantically affected
  access structures, with exact counters proving bounded breadth.
- Adversarial denial: a read that is safe one-shot but unsafe to maintain live
  returns a live-specific support denial rather than silently degrading.

**Engineering decisions**
- Live planning records maintenance breadth separately from initial execution
  breadth.
- Store-backed durable live access maintenance returns
  `RequiresStoreBackedLiveAccessMaintenance` with owner, support-row evidence,
  and receipt posture unless admitted by store support.

**Open questions**
- None.

### Phase 13: Preview, Branch, Policy, Tenant, And Relationship-Proof Parity

Ensure preview sessions, branch-scoped reads, policy/tenant narrowed reads, and
relationship-proof reads use the same access-planning pipeline without
promoting lower-authority labels, snapshots, or proof descriptors into raw
planner strings.

**Relevant subsystems**
- `crates/forge-query/src/runtime/preview/`
- `crates/forge-query/src/runtime/branch.rs`
- `crates/forge-query/src/policy_narrowing/`
- `crates/forge-query/src/relationship_proof/`

**Relevant APIs**
- preview read surfaces
- branch read surfaces
- `admit_policy_tenant_context`
- `admit_relationship_proofs`
- `ForgeQueryGraphReadAccessShape`

**Warnings**
- Policy and relationship proof narrowing affect both access eligibility and
  cost; they cannot be applied after edge expansion.
- Branch and preview identity must remain typed basis artifacts, not strings
  embedded in access-plan digests.

**Test requirements**
- Adversarial parity: current-head and preview/branch reads with equivalent
  admitted basis derive compatible access requirements and distinct basis-bound
  digests.
- Adversarial leakage: policy-denied or relationship-proof-denied reads do not
  build adjacency, frontier, visited, or result buffers before denial.
- Adversarial tenant: tenant narrowing changes access shape and cost estimate
  before execution, not after result filtering.

**Engineering decisions**
- Access planning consumes admitted basis/narrowing artifacts, not raw caller
  context.
- Denial receipts distinguish access budget failure from policy/tenant/
  relationship-proof denial.

**Open questions**
- None.

### Phase 14: Consumer Bypass Audit For Graph Reads

Extend consumer-kit audit machinery to detect caller-owned graph read folklore:
manual loops over relation rows, per-node neighbor lookups, ad hoc adjacency
maps, surface-local graph caches, manual broad boolean scans, and hidden
fallbacks that bypass Query access planning.

**Relevant subsystems**
- `crates/forge-query/src/consumer_kit/`
- `crates/worth-topo/src/`
- `crates/worth-kernel/src/`

**Relevant APIs**
- hard-prohibition registry and boundary audit surfaces
- new: graph-read access bypass registry rows
- new: reference-consumer residue reports

**Warnings**
- False positives against docs/comments/literals are not acceptable; the audit
  must be precise enough for normal development.
- Do not treat existing helper loops as harmless because they are test support
  if they claim production proof.

**Test requirements**
- Adversarial detection: seeded N+1 relation loops, ad hoc adjacency maps, and
  manual frontier scans are detected in covered source sets.
- Adversarial false-positive: comments, docs, string literals, and unrelated
  collection iteration do not trigger graph-read bypass findings.
- Adversarial adoption: reference consumers delete or explicitly classify all
  covered graph-read folklore.

**Engineering decisions**
- Bypass rows are keyed by authority violation, not by fragile text pattern
  alone.
- Reference consumers may keep explicit residue only with typed owner, reason,
  and follow-on closure path.

**Open questions**
- None.

### Phase 15: Reference Adoption In worth-topo Graph Reads

Migrate worth-topo graph read surfaces that currently rely on local topology
walks, runtime-boundary read lowering, or test harness convenience into the
Query access-planning lane where covered. The goal is not to delete domain
meaning; it is to delete access-path folklore.

**Relevant subsystems**
- `crates/worth-topo/src/projection/runtime_boundary/`
- `crates/worth-topo/src/projection/read_views/`
- `crates/worth-topo/src/certification/`

**Relevant APIs**
- topology read handles
- Query read composition helpers
- `ForgeQueryGraphReadAccessAdmission`

**Warnings**
- Topology remains the domain authority for topology meaning; Query owns access
  planning and receipts.
- Do not migrate by wrapping old loops behind new names. The access receipt must
  prove the planned path ran.

**Test requirements**
- Adversarial parity: migrated topology reads produce the same domain result as
  the old certified reference scenarios on small graphs.
- Adversarial no-N+1: hostile dense topology scenarios prove exact-zero
  caller-owned per-edge/per-node lookup loops on covered paths.
- Adversarial broadness: broad boolean/topology search cases route to streaming,
  persistent-index-required, async, or denial posture instead of inline RAM
  expansion.

**Engineering decisions**
- Adoption starts with the read surfaces that AI_README currently teaches as
  graph/index-shaped.
- Existing topo tests may retain ergonomic helpers only if helpers cross into
  typed Query access plans before ledger/read execution.

**Open questions**
- None.

### Phase 16: Reference Adoption In worth-kernel Construction Reads

Migrate worth-kernel construction and phase-chain read/check surfaces that
currently reconstruct topology or access paths locally into declared Query read
access plans where covered.

**Relevant subsystems**
- `crates/worth-kernel/src/construction/`
- `crates/worth-kernel/src/construction/phase_chain/`
- `crates/worth-kernel/src/query_adoption/`

**Relevant APIs**
- construction read/check surfaces
- Query read access admission APIs
- consumer-kit bypass audit APIs

**Warnings**
- Construction legality remains obligation-authority work from Milestone `9.9`;
  this phase targets graph read access path planning.
- Do not use kernel construction as a hidden broad scan escape hatch.

**Test requirements**
- Adversarial parity: covered construction reads/checks produce the same
  certified construction decisions when lowered through Query access plans.
- Adversarial rejection: over-budget construction graph scans fail before local
  graph materialization or return an async/materialized posture.
- Adversarial residue: covered kernel files have exact-zero unclassified manual
  graph-read access folklore.

**Engineering decisions**
- Kernel consumes Query access receipts; it does not derive index adequacy from
  local graph shape.
- Any offline-only residue must be typed and excluded from ordinary runtime
  support posture.

**Open questions**
- None.

### Phase 17: Public Docs, AI_README, And DX Target

Update public docs and AI orientation so agents learn the exact contract:
Query derives access plans from declarations, provisions only admitted bounded
index postures, and returns typed required-capability, materialization, or
denial postures instead of hiding N+1 or RAM-expansive work.

**Relevant subsystems**
- `crates/forge-query/docs/AI_README.md`
- `crates/forge-query/docs/authoring/read-composition.md`
- `crates/forge-query/docs/foundations/support-matrix-and-admission.md`
- new: `crates/forge-query/docs/authoring/graph-read-access-planning.md`

**Relevant APIs**
- public read composition facade
- access-plan support rows
- graph read receipt access-plan summaries

**Warnings**
- Do not document the aspirational index story as if every graph shape is
  admitted. Support rows must name what is real.
- Do not teach "automatic indexing" without teaching budget denial and async/
  persistent/streaming postures.

**Test requirements**
- Adversarial agreement: AI_README, feature docs, support rows, and public
  certification name the same access postures, required-capability cases, and
  typed denials.
- Adversarial DX: the documented happy path shows a graph read declaration that
  produces an access receipt without caller-owned loops.
- Adversarial denial docs: broad boolean examples show typed budget denial and
  legitimate escape hatches rather than "increase limit and retry."

**Engineering decisions**
- AI_README gets a distinct category for graph read access planning, separate
  from graph touch obligation authority.
- Docs include the canonical DX target:

```rust
let result = workspace.compose_read(|read| {
    read.anchored_frontier_detail(topology_schema, anchor)
        .through_relation("HalfEdgeNext")
        .through_relation("HalfEdgeTwin")
        .max_depth(4)
        .where_field("tenant_id").equals(current_tenant)
        .project(["half_edge_id", "face_id", "next_id"])
})?;

let access = result.receipt().graph_access_plan();
assert!(access.proves_no_caller_owned_n_plus_one());
assert!(access.max_resident_bytes() <= admitted_budget.max_resident_bytes());
```

**Open questions**
- None.

### Phase 18: Architectural Certification Closeout

Close the milestone with hostile certification across access shape, budgeting,
index requirements, admission outcomes, execution postures, no-N+1 audits,
memory counters, reference adoption, and docs/support agreement.

**Relevant subsystems**
- `crates/forge-query/tests/`
- `_docs/forge-query/test-requirements.md`
- `crates/worth-topo`
- `crates/worth-kernel`

**Relevant APIs**
- graph access plan APIs from this milestone
- support matrix rows
- consumer-kit bypass audit APIs

**Warnings**
- A passing small graph suite does not prove this milestone. Dense, broad, and
  high-branching hostile workloads are mandatory.
- Do not close while any covered surface can execute graph reads without an
  access-plan receipt.

**Test requirements**
- Adversarial matrix: every access posture is certified across representative
  local, anchored, frontier, broad boolean, policy/tenant, relationship-proof,
  reusable family, preview/branch, and live-promoted reads.
- Adversarial memory: exact counters prove dense broad reads deny, stream, or
  materialize under budget without exceeding declared resident-byte limits.
- Adversarial no-N+1: bypass audit and runtime counters prove exact-zero
  covered caller-owned N+1 graph read paths.
- Adversarial replay: access-plan receipts replay to the same digests and
  counter envelopes under identical runtime truth.

**Engineering decisions**
- Certification matrix names runtime-backed support separately from store-owned
  durability capability requirements.
- Closure requires reference-consumer adoption, not just Query-local APIs.

**Open questions**
- None.

## Must Ship

- sealed graph read access shape vocabulary derived from canonical read graphs
- boolean predicate normalization and selectivity input artifacts
- required graph index set derivation with typed directional/lifecycle rows
- cost estimates, memory/breadth budgets, and complexity contracts
- typed access admission outcomes:
  `AdmittedInlineIndexed`, `AdmittedPagedStreaming`,
  `RequiresPersistentIndex`, `RequiresAsyncMaterialization`,
  `RequiresStoreBackedPersistentIndex`,
  `RequiresAccessCapabilityRegistration`,
  `RequiresStoreBackedLiveAccessMaintenance`, and `DeniedBudgetExceeded`
- exhaustive access-case registry covering admitted, required-capability,
  materialized, streaming, store-owned, and typed-denial graph read cases
- existing index inventory and support rows
- bounded ephemeral index provisioning with lifecycle receipts
- paged streaming frontier execution
- persistent index requirement declarations for read families
- async graph read materialization jobs
- ordinary read, reusable family, intent, live, preview, branch, policy/tenant,
  and relationship-proof integration
- consumer bypass audit for graph read N+1 and manual access folklore
- worth-topo and worth-kernel reference adoption
- public docs and AI_README category for graph read access planning
- architectural certification matrix with no-N+1 and memory-budget proof

## Must Preserve

- `forge-relational` remains the authority for truth semantics and relational
  state mechanics.
- Query owns typed read declaration, lowering, access planning, admission,
  receipts, and support posture.
- Indexes, access plans, materializations, and receipts are derived from
  authoritative truth and must be rebuildable.
- Store-backed durable indexes and restart-stable access artifacts are explicit
  Milestone `10`/`11` capability requirements until the owning milestones admit
  them.
- Graph touch obligation authority from Milestone `9.9` remains separate from
  graph read access planning; the former decides which obligations fire, while
  this milestone decides how reads access graph truth safely.

## Acceptance Evidence

- covered graph reads execute only through proof-bearing access plans
- hidden N+1 graph traversal is mechanically impossible on covered public lanes
- broad boolean and dense frontier reads deny, stream, require persistent index,
  or require async materialization before expensive work begins
- access-plan receipts expose required indexes, selected posture, estimated and
  actual counters, resident-byte budgets, touched-edge counts, frontier counts,
  and fallback/denial posture
- memory and breadth counters have exact hostile tests rather than elapsed-time
  claims
- docs, support rows, AI_README, and certification agree about admitted,
  required-capability, materialization, store-owned, and typed-denial graph read
  access postures
- every edge case named in docs, tests, support rows, or the access-case
  registry maps to a concrete admission result and receipt field
- worth-topo and worth-kernel covered graph read surfaces delete or classify
  local access-path folklore

## Sequencing Notes

- This milestone belongs after Milestone `9.9` because it consumes graph read
  shape and obligation-envelope vocabulary without overloading obligation
  dispatch with access-path planning.
- This milestone belongs before Milestone `10` because store-backed execution
  and pushdown should inherit a complete runtime-backed access-plan contract,
  not discover N+1 prevention, memory budgets, and index posture later.
- Milestone `10` may later add store-backed persistent index realization and
  pushdown parity, but it should not redefine the access-shape, budget, denial,
  and receipt model established here.
- Milestone `11` may later make saved access artifacts durable, but restart-
  stable persistence is not required for runtime-backed closure here.
