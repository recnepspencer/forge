# Milestone 9.7 Engineering Spec: Concurrent Read Authority And Deterministic Submission

> **Status:** Draft
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Primary predecessors:** [milestone-9.6.md](./milestone-9.6.md), [milestone-9.5.md](./milestone-9.5.md), [milestone-9.4.md](./milestone-9.4.md)
>
> **Purpose:** decompose the Query runtime into authority-typed subsystems so
> committed-snapshot reads scale concurrently across consumers while truth
> mutation and derived maintenance remain single-owner and deterministic —
> before Milestone `10` freezes store-backed shapes around the current
> single-borrow topology.

## Goal

Give Query a real multi-consumer operating topology: sealed, basis-bound,
`Send + Sync` shared read contexts that exploit the immutability MVCC already
guarantees; one deterministic submission seam through which all mutation and
declaration work enters in total order; and a published-artifact rule that
keeps derived computation single-owner so concurrent readers can never
perturb evaluation — all without changing canonical query meaning or adding a
second semantics path beside the workspace.

## Why This Milestone Exists

Every workspace operation today — including read-shaped ones like opening a
preview — takes `&mut self`, and sessions are lifetime-bound borrows. The
borrow checker therefore enforces one operation in flight per workspace,
forever, regardless of how concurrent `forge-relational`'s MVCC substrate is
underneath. Snapshot immutability is a structural fact the type system has
never been told.

Without this milestone:

- server-grade consumers must choose between a global lock (throughput cliff,
  MVCC wasted) and branch-per-connection (truth-fork semantics spent on a
  transport problem, subscription sharing defeated, abandoned forks to
  collect) — both are exactly the consumer-invented runtime folklore the
  Query hard prohibitions exist to prevent
- Milestone `10` freezes store-backed adapter and execution shapes around the
  single-borrow topology, making the `Send` boundary retrofit dramatically
  more expensive than the decomposition costs now
- determinism remains an accident of single-threadedness instead of a stated
  property of total commit order plus ordered maintenance, pinned by receipts

`arch_laws.md` Law 1 already states the design: read-path constructs borrow
only the observation subsystem, and snapshot isolation is a structural
consequence of correct decomposition. This milestone is that law applied to
the Query runtime.

## Governing Summaries

- `MENTALITY.md`: solve the hard problem first — concurrency topology is
  load-bearing infrastructure that must precede the store-backed features
  that would otherwise freeze around its absence.
- `arch_laws.md`: Law 1 (autonomous subsystems; read path borrows only
  observation), Law 10 (methods must not hold artificial locks against data
  they do not touch), Law 18 (phase-typed observation), Law 27 (parallel
  admission only through planner-carried structural disjointness), Law 33
  (authority versus derived state), Law 36 (checkpoint plus bounded journal),
  Law 41 (sealed proof-carrying types).
- `composition_laws.md`: the read authority, submission seam, and publication
  boundary are named structural homes, not behavior smeared through the
  existing workspace internals.
- `domain_structure_laws.md`: authoritative state, derived state, and
  observation must not share structural space; the decomposition must be
  visible in the tree, not just in the types.
- `perf_laws.md`: throughput scales exclusively with data independence;
  reference counting is disguised contention; locality and equivalence
  contracts come before tuning. The read hot path must be lock-free and
  refcount-quiet by structure.
- `forge_query_roadmap.md`: Query owns typed expression, lowering, and result
  shaping; relational owns truth; signal owns reactive evaluation. This
  milestone redistributes *access topology* only — no authority moves.

## Adversarial Constraint

N concurrent shared read contexts under sustained commit pressure, preview and
branch session churn, and live maintenance load must produce byte-identical
results and receipts for the same `(canonical declaration, basis capability)`
pair as a fully serialized execution of the same schedule — while journal
replay of the submission stream reconstructs identical truth, identical
published derived artifacts, and identical receipts, with zero locks acquired
on the committed-read hot path and zero derived evaluations triggered by any
reader.

This milestone fails if any covered path:

- lets a reader observe non-committed, mid-mutation, or torn state
- lets read concurrency change any result, receipt, digest, or delivery
  content (timing may vary; values may not)
- lets a reader trigger signal-graph evaluation or mutate memoization state
- serializes committed reads through a lock, a refcount storm, or the
  mutation owner
- creates a second query semantics path beside the workspace facade

## Product Decision Lock

- Decomposition per arch law 1: the read lane borrows only observation; write
  authority remains fully accessible beside concurrent readers.
- Committed snapshots become sealed, basis-bound, `Send + Sync` read
  contexts. Mutation keeps exactly one exclusive owner.
- Readers never evaluate. Derived results are published, digest-stamped
  artifacts consumed through the existing projection-consumption lane;
  missing or stale derivations surface as the existing async result-state
  vocabulary, not as reader-side evaluation.
- The submission stream is the journal. Total intake order at the mutation
  owner is the deterministic basis for replay.
- The workspace facade survives unchanged as the single-owner convenience
  over the decomposed authorities. Existing consumers compile untouched.
- No `Mutex`/`RwLock` smearing. Concurrency arrives only through structural
  immutability proofs; any future parallel-write work must arrive through
  planner-carried disjointness proofs per arch law 27, not through this
  milestone.
- All new receipts, journal identity, and published-artifact digests lower
  through the Milestone `9.6` canonical evidence-identity primitive.

## Phase Plan

### Phase 1: Backend Adapter Authority-Lane Decomposition Boundary

Split the backend adapter contracts by authority lane — committed read,
declaration intake, patch consumption, mutation — so no trait object mixes
read access with mutation access, and place `Send + Sync` bounds on the read
lane contracts. Zero behavior change.

**Relevant subsystems**
- `runtime` backend contracts and parts assembly
- runtime builder
- test-support and downstream adapter implementations

**Relevant Query source surfaces**
- [runtime/backend/contracts.rs](../../crates/forge-query/src/runtime/backend/contracts.rs)
- [runtime/backend/parts.rs](../../crates/forge-query/src/runtime/backend/parts.rs)
- [runtime/builder.rs](../../crates/forge-query/src/runtime/builder.rs)

**Relevant APIs and product surfaces**
- `ForgeQueryRuntimeSourceAdapter` and sibling adapter contracts, decomposed
  so read methods (`live_entities`-shaped) and consumption/mutation methods
  (`drain_live_patches`-, `declare_live_view`-shaped) live behind separate
  authority-typed contracts

**Warnings**
- Do not change any adapter behavior in this phase; the split is topology
  only, certified by parity.
- Do not leave a convenience super-trait that re-merges the lanes for
  ergonomics; that is the artificial lock returning with a friendlier name.
- Do not let test-support adapters keep the merged shape; they are part of
  the covered surface.

**Test requirements**
- Add a `Backend Authority Lane Decomposition Parity Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: run a representative covered workload through the
  pre-split and post-split assemblies and prove identical results, receipts,
  and digests.
- Adversarial boundary localization: a compile-fail contract proving a
  read-lane adapter reference cannot reach declaration, consumption, or
  mutation methods.

**Engineering decisions**
- Lane membership of every existing adapter method is decided and recorded in
  this phase; no method remains ambiguously dual-lane.
- `Send + Sync` bounds land on read-lane contracts here even though nothing
  is concurrent yet, so drift is caught at the earliest boundary.

**Open questions**
- None.

### Phase 2: Runtime Assembly Lifecycle Enforcement Boundary

Freeze lane-aware runtime construction: parts intake and builder assembly
consume the decomposed lane contracts, and omitting or failing to propagate a
required lane is a compile-time error at every construction and fork site per
arch law 9.

**Relevant subsystems**
- runtime builder and backend-parts assembly
- construction lifecycle propagation

**Relevant Query source surfaces**
- [runtime/builder.rs](../../crates/forge-query/src/runtime/builder.rs)
- [runtime/backend/parts.rs](../../crates/forge-query/src/runtime/backend/parts.rs)

**Relevant APIs and product surfaces**
- `ForgeQueryRuntime::builder()` lane-typed part intake
- `build_backend_from_parts()` over decomposed lane contracts

**Warnings**
- Do not enforce lane completeness with runtime checks where typestate or
  exhaustive struct expressions can make omission uncompilable.
- Do not let optional lanes blur into required lanes; optionality is part of
  the lane contract, not a builder default.

**Test requirements**
- Add a `Lane-Aware Assembly Enforcement Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: prove a fully assembled lane-typed runtime is
  behaviorally identical to the Phase 1 parity baseline for the covered
  workload.
- Adversarial localization: compile-fail contracts proving builder assembly
  without a required lane, and lane addition without propagation through
  every construction site, both fail to compile.

**Engineering decisions**
- Assembly enforcement is compile-time per the `MENTALITY.md` enforcement
  hierarchy; runtime posture checks remain only for genuinely dynamic backend
  capability.
- This phase changes construction shape only; no execution semantics move.

**Open questions**
- None.

### Phase 3: Snapshot Generation Pinning And Retirement Boundary

Freeze the snapshot retention substrate the shared read lane stands on:
generation-indexed pinning of committed snapshots with explicit retirement,
so read contexts can hold immutable bases without per-operation reference
counting and without unbounded retention.

**Relevant subsystems**
- snapshot retention and generation pinning (new boundary home inside
  `runtime`)
- basis capability lifecycle

**Relevant Query source surfaces**
- [runtime/state_snapshot.rs](../../crates/forge-query/src/runtime/state_snapshot.rs)
- [runtime/workspace.rs](../../crates/forge-query/src/runtime/workspace.rs)

**Relevant APIs and product surfaces**
- generation-indexed snapshot pin and retirement surfaces consumed by the
  Phase 4 read context (internal authority surfaces, not ordinary consumer
  API)

**Warnings**
- Do not pin snapshots with per-call `Arc` traffic; perf law names refcount
  storms as disguised contention. Generation-indexed pinning with explicit
  retirement is the required shape.
- Do not let retention become unbounded; retirement is part of this boundary,
  not a later cleanup task.

**Test requirements**
- Add a `Snapshot Generation Pinning Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: prove a pinned generation observes identical
  snapshot content for its full pin lifetime under sustained commit pressure.
- Adversarial residue: prove that after a hostile pin/retire schedule, zero
  orphaned generations and zero unretired pins remain, as exact counter
  assertions.
- Adversarial contention proof: exact counters proving zero lock acquisitions
  and bounded (structurally constant) refcount operations on the pinned-read
  hot path.

**Engineering decisions**
- Pin identity is generation-indexed (index + generation) per perf-law
  identity guidance, not key-based lookup.
- Retirement is explicit and observable; generation drain is a counted event,
  not a destructor side effect.

**Open questions**
- None.

### Phase 4: Sealed Basis-Bound Shared Read Context Boundary

Admit the shared read lane: a sealed read context minted only at commit
boundaries, carrying its basis capability proof, `Send + Sync`, cheaply
shareable across threads, and holding its snapshot through the Phase 3
pinning substrate.

**Relevant subsystems**
- read authority (new boundary home inside `runtime`)
- basis capability lifecycle
- snapshot generation pinning (consumed from Phase 3)

**Relevant Query source surfaces**
- [runtime/workspace.rs](../../crates/forge-query/src/runtime/workspace.rs)
- [runtime/workspace_queries.rs](../../crates/forge-query/src/runtime/workspace_queries.rs)
- [runtime/state_snapshot.rs](../../crates/forge-query/src/runtime/state_snapshot.rs)

**Relevant APIs and product surfaces**
- the sealed shared read context constructor on the workspace (the only legal
  mint, per arch law 41)
- read execution over the context for admitted one-shot query families
- `workspace.state(...)`-shaped readiness over the shared lane

**Target shape (illustrative, not frozen API)**

The single-borrow topology this phase opens up, as it constrains consumers
today (`crates/worth-kernel/src/construction/runtime_proof/runtime_basis.rs`):

```rust
// BEFORE: even pure inspection reports demand exclusive access,
// so no two of these can ever run at once
pub fn prepare_primitive_construction_branch_preview_runtime_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: impl PrimitiveConstructionAuthoringInput,
) -> Result<..., PrimitiveConstructionRuntimeBasisError>
```

The target shape after this phase:

```rust
// AFTER: read-shaped work moves to a shared, Send + Sync, basis-pinned
// context; N reports run on N threads against the same committed basis
let read_ctx: ForgeQuerySharedReadContext = workspace.shared_read_context()?;

let reports: Vec<_> = std::thread::scope(|s| {
    intents.iter()
        .map(|intent| s.spawn(|| prepare_parity_report(&read_ctx, intent))) // &, not &mut
        .map(|handle| handle.join().unwrap())
        .collect()
});
// byte-identical receipts to serialized execution — certified in Phase 10
```

**Warnings**
- Do not let the read context be constructible from a raw snapshot id, branch
  id, or label; it exists only as the product of basis capability admission.
- Do not give the read context any method whose execution writes — including
  "convenience" cache warming or lazy index repair.

**Test requirements**
- Add a `Shared Read Context Concurrency Equivalence Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: N threads reading through shared contexts under
  commit pressure produce byte-identical results and receipts to the same
  schedule executed serially.
- Adversarial denial: a context whose snapshot generation has been retired
  fails closed with a typed stale-basis stop; it must never silently rebind
  to a newer snapshot.

**Engineering decisions**
- Read contexts are immutable values; refreshing to a newer basis is an
  explicit re-mint through the workspace, never an in-place mutation.
- Receipts emitted through the shared lane record the basis proof and lower
  identity through the `9.6` evidence primitive.

**Open questions**
- None.

### Phase 5: Deterministic Submission Intake And Journal Boundary

Admit the single-writer submission seam: typed intake of canonical
declaration and mutation work into the mutation owner, where total intake
order is the journal, admission flows through the existing intent admission
lattice, and completed work returns the existing receipt and envelope
artifacts.

**Relevant subsystems**
- submission intake (new boundary home inside `runtime`)
- intent admission lattice
- write receipts and boundary envelopes
- journal identity

**Relevant Query source surfaces**
- [runtime/workspace.rs](../../crates/forge-query/src/runtime/workspace.rs)
- [runtime/workspace_declaration.rs](../../crates/forge-query/src/runtime/workspace_declaration.rs)

**Relevant APIs and product surfaces**
- the typed submission surface accepting canonical declarations and admitted
  intent families
- ordered submission receipts carrying journal position identity
- the existing admission, receipt, and envelope vocabulary — reused, not
  duplicated

**Target shape (illustrative, not frozen API)**

```rust
// AFTER: mutation work enters as canonical declarations through one ordered
// intake; the receipt records the journal position the work committed at
let receipt = workspace.submissions().submit(construction_declaration)?;
assert!(receipt.journal_position().is_committed());

// admission stops are the same typed lattice outcomes as today:
// denied/advisory/violation with decision traces — submission adds ordering,
// not a second admission vocabulary
```

**Warnings**
- Do not build a second admission path inside the seam; submissions enter the
  existing lattice or they do not enter.
- Do not let the seam accept raw payloads, closures, or host callbacks; the
  intake vocabulary is canonical declarations and admitted intents only.
- Do not make submission ordering depend on thread scheduling accidents;
  intake order is assigned at one authority point and recorded in the
  receipt.

**Test requirements**
- Add a `Deterministic Submission Journal Replay Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: submit an interleaved multi-producer workload,
  record the journal, replay it, and prove identical truth, receipts, and
  digests — including across process restart of the runtime-backed harness.
- Adversarial rejection: duplicate submission of the same canonical identity
  and submission against a stale basis both stop typed at admission, with
  decision traces preserved, and leave no journal residue.

**Engineering decisions**
- The journal is the submission stream itself; checkpoint-plus-journal
  reconstruction per arch law 36 is certified here at runtime-backed scope,
  while durable journal persistence remains Milestone `10`/`11` scope.
- Journal position identity lowers through the `9.6` evidence primitive.

**Open questions**
- None.

### Phase 6: Consumer-Facing Journal Replay Boundary

Admit the public replay surface over recorded journal segments, so downstream
applications stop re-deriving state by folding locally retained change
artifacts. Today both `workflow-editor` and `hadwiger-research` implement
replay as consumer-side refolds because no runtime-owned replay lane exists.

**Relevant subsystems**
- consumer replay surface (new boundary home over the Phase 5 journal)
- basis capability lifecycle
- journal segment identity

**Relevant Query source surfaces**
- [runtime/workspace.rs](../../crates/forge-query/src/runtime/workspace.rs)
- [runtime/workspace_declaration.rs](../../crates/forge-query/src/runtime/workspace_declaration.rs)

**Relevant APIs and product surfaces**
- typed replay requests binding a basis capability to a journal segment
  identity
- replay outputs as ordinary receipts and envelopes — not a parallel result
  vocabulary

**Warnings**
- Do not expose raw journal entries as the replay vocabulary; consumers
  request replay outcomes, not journal internals.
- Do not let replay become a second execution semantics; replay re-derives
  through the same admission, execution, and receipt paths the original run
  used.

**Test requirements**
- Add a `Consumer Journal Replay Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: a consumer-shaped test replays a recorded journal
  segment through the public replay surface and proves the reconstructed
  state, receipts, and digests match the original run exactly — without the
  consumer retaining or refolding any change artifacts of its own.
- Adversarial denial: replay requests against a mismatched basis, an unknown
  segment identity, or a cross-scheme digest stop typed at admission and
  leave no replay residue.

**Engineering decisions**
- The replay surface is consumer product surface, not internal determinism
  machinery.
- Replay request and segment identity lower through the `9.6` evidence
  primitive.

**Open questions**
- None.

### Phase 7: Maintenance-Owner Publication Boundary

Close the owner half of the rule that keeps determinism alive under
concurrency: derived computation is evaluated only by the single maintenance
owner, which publishes atomic, digest-stamped result artifacts in commit
order.

**Relevant subsystems**
- signal-facing maintenance ownership
- published artifact registry

**Relevant Query source surfaces**
- [runtime/workspace_queries.rs](../../crates/forge-query/src/runtime/workspace_queries.rs)
- [projection_consumption/receipt.rs](../../crates/forge-query/src/projection_consumption/receipt.rs)

**Relevant APIs and product surfaces**
- maintenance-owner publication receipts binding artifact digest to basis and
  commit position
- the published artifact registry the Phase 8 reader lane consumes

**Warnings**
- Do not let lazy pull escape the maintenance owner; evaluation legality
  lives here, before any reader lane exists to abuse it.
- Do not let publication and the Phase 5 journal develop independent
  orderings; the maintenance owner is the same single-writer locus as the
  submission seam or is fed by it in commit order.

**Test requirements**
- Add a `Maintenance Owner Publication Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: for every covered derived family, prove the
  published artifact at basis B equals a from-authority recomputation at
  basis B (arch law 33 honesty).
- Adversarial atomicity: prove an observer of the registry during
  republication sees either the old artifact or the new one — never a blend —
  and that publication order matches commit order exactly under hostile
  scheduling.

**Engineering decisions**
- Publication is atomic per artifact: digest-stamped, basis-bound, commit-
  ordered, and lowered through the `9.6` evidence primitive.
- Two orderings may not exist; publication order is commit order.

**Open questions**
- None.

### Phase 8: Reader Consumption And Async Posture Boundary

Close the reader half: shared read contexts consume published artifacts
through the projection-consumption lane, missing or republishing derivations
surface as typed async result-state, and no reader-reachable path can trigger
evaluation.

**Relevant subsystems**
- projection consumption
- async result-state surfaces
- shared read context (from Phase 4)

**Relevant Query source surfaces**
- [projection_consumption/source.rs](../../crates/forge-query/src/projection_consumption/source.rs)
- [projection_consumption/receipt.rs](../../crates/forge-query/src/projection_consumption/receipt.rs)

**Relevant APIs and product surfaces**
- published derived-artifact consumption through the shared read context
- the existing async result-state vocabulary (`pending`, `current`, `stale`,
  `revalidating`, `superseded`) carrying not-yet-published posture

**Target shape (illustrative, not frozen API)**

```rust
// AFTER: a reader consuming a derivation gets the last published,
// digest-stamped artifact through the projection-consumption lane —
// it can never trigger evaluation itself
match read_ctx.consume_projection(derived_facts_declaration)? {
    ForgeQueryProjectionConsumption::Current(facts) => {
        // facts.receipt() binds artifact digest to basis and commit position
    }
    ForgeQueryProjectionConsumption::ResultState(state) => {
        // not-yet-published or republishing posture arrives as the existing
        // async result-state vocabulary: pending / stale / revalidating —
        // never as a reader-side recomputation
    }
}
```

**Warnings**
- Do not let any reader-reachable path call into signal evaluation; lazy pull
  remains legal only inside the Phase 7 maintenance owner.
- Do not invent a new "derived read" API; consumption flows through the
  projection-consumption lane Milestone `9.5` made first-class.
- Do not erase merged `9.4` temporal/async meaning when publication posture
  carries it; a time-driven artifact that has not republished is `stale` or
  `revalidating`, never silently recomputed reader-side.

**Test requirements**
- Add a `Published Artifact Reader Isolation Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: concurrent readers consuming a covered derived
  family during republication observe either the old artifact or the new one
  with matching receipts — never a blend — and identical fact content to a
  serialized consumer of the same schedule.
- Adversarial leakage: exact-zero counter assertion that no reader execution
  path performed a signal-graph evaluation, plus a hostile probe proving a
  reader requesting an unpublished derivation receives typed `pending`
  posture rather than a result.

**Engineering decisions**
- Reader consumption is receipt-backed projection consumption; retained rows
  and payload bags do not return as the derived-read vocabulary.
- Async posture preservation is part of this boundary's closure, not a
  temporal follow-up.

**Open questions**
- None.

### Phase 9: Workspace Facade Re-Expression Boundary

Re-express `ForgeQueryWorkspace` as the single-owner convenience facade over
the decomposed authorities so existing consumers compile unchanged, the new
lanes are reachable from the facade, and construction lifecycle propagation
is compile-enforced.

**Relevant subsystems**
- workspace facade
- runtime construction and lifecycle propagation
- support/admission reporting for the new families

**Relevant Query source surfaces**
- [runtime/workspace.rs](../../crates/forge-query/src/runtime/workspace.rs)
- [runtime/builder.rs](../../crates/forge-query/src/runtime/builder.rs)
- [runtime/support/profile.rs](../../crates/forge-query/src/runtime/support/profile.rs)
- [runtime/support_matrix.rs](../../crates/forge-query/src/runtime/support_matrix.rs)

**Relevant APIs and product surfaces**
- `ForgeQueryWorkspace` — unchanged signatures for existing families, plus
  mint points for shared read contexts and the submission seam
- support matrix and admission rows for the new `SharedRead` and
  `Submission` facade families, fail-closed where a backend posture cannot
  honor them

**Warnings**
- Do not deprecate the single-owner workspace; kernel-grade single-threaded
  consumers are first-class, not legacy.
- Do not expose the decomposed internals as public topology; the facade and
  the sealed artifacts are the only surface per domain structure law 21.
- Do not let the new families ship visible-but-unadmitted without fail-closed
  admission rows; support honesty is part of this phase, not Phase 6 polish.

**Test requirements**
- Add a `Facade Lane Parity And Lifecycle Propagation Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: the same covered operation executed through the
  facade convenience path and through the decomposed lanes produces identical
  receipts and digests.
- Adversarial localization: a compile-fail contract proving that adding a new
  runtime authority subsystem without propagating it through every
  construction and fork site fails to compile (arch law 9), and that
  downstream consumer code cannot reach decomposed internals past the facade.

**Engineering decisions**
- Existing downstream call sites (including `worth-kernel`) must compile
  without modification; this is an acceptance criterion, not a hope.
- New facade families enter the support matrix with the same admission,
  teaching-posture, and fail-closed discipline as every existing row.

**Open questions**
- None.

### Phase 10: Hostile Concurrency And Determinism Certification Boundary

Close the milestone with one hostile certification program that drives all
lanes simultaneously and proves the adversarial constraint end to end.

**Relevant subsystems**
- runtime-backed certification harness (the Milestone `9.5` raw runtime read
  bootstrap is the required entry)
- all Phase 1–9 boundaries

**Relevant Query source surfaces**
- [tests/support/public_bridge_runtime/mod.rs](../../crates/forge-query/tests/support/public_bridge_runtime/mod.rs)
- [runtime/tests/support/bridge/runtime_support.rs](../../crates/forge-query/src/runtime/tests/support/bridge/runtime_support.rs)

**Warnings**
- Do not certify axes in isolation; the matrix must combine concurrent
  readers, submission pressure, preview/branch churn, derived republication,
  and replay in one program.
- Do not accept threshold-shaped assertions where exact counters are
  achievable; lock count and reader-evaluation count are exact zeros, not
  "low".

**Test requirements**
- Add a `Milestone 9.7 Concurrency Determinism Hostile Certification Matrix`
  to [test-requirements.md](./test-requirements.md) and close it in this
  phase.
- Adversarial equivalence: the full hostile schedule — N readers, M
  submitters, derived republication, preview/branch churn — produces
  byte-identical receipts, results, and published-artifact digests to its
  serialized replay, across repeated runs.
- Adversarial residue: after the hostile schedule, prove zero orphaned
  snapshot generations, zero unretired read pins, zero journal gaps, and
  zero delivery residue, each as an exact counter assertion.

**Engineering decisions**
- This phase is necessary but not sufficient: the hostile certification matrix
  must exist before the end-cap closure phases can certify that the underlying
  counters, pinning substrate, journal identity, replay lane, and public-bridge
  consumer proofs are runtime-owned rather than modeled.
- Certification artifacts lower through the `9.6` evidence primitive so the
  matrix itself is replay-comparable.

**Open questions**
- None.

### Phase 11: Runtime-Owned Shared Read Pinning And Retirement Closure Boundary

Replace the current snapshot-copy shared-read implementation with the runtime-
owned substrate the milestone actually promised: generation-indexed pinning,
explicit retirement, typed stale-basis denial, and mechanically observable
residue counters so shared read authority is backed by structural runtime state
rather than copied materialization snapshots.

**Relevant subsystems**
- runtime shared-read substrate
- snapshot generation pinning and retirement
- basis capability lifecycle

**Relevant Query source surfaces**
- [runtime/shared_read.rs](../../crates/forge-query/src/runtime/shared_read.rs)
- [runtime/state.rs](../../crates/forge-query/src/runtime/state.rs)
- [runtime/workspace_shared_read.rs](../../crates/forge-query/src/runtime/workspace_shared_read.rs)

**Relevant APIs and product surfaces**
- `ForgeQuerySharedReadContext` as a sealed, basis-bound, `Send + Sync`,
  runtime-backed read authority
- runtime-owned shared-read pin / retire counters and stale-basis denial
  surfaces consumed by hostile certification

**Warnings**
- Do not satisfy this phase by copying more runtime state into the shared-read
  context; copied snapshots are not pinning.
- Do not model pin retirement through `Drop` heuristics alone; retirement must
  be explicit, observable, and certifiable.
- Do not silently rebind an old shared-read context to a newer committed
  snapshot when its pinned generation is retired; stale basis must fail typed.

**Test requirements**
- Add a `Runtime-Owned Shared Read Pinning Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: a pinned shared-read context under sustained commit
  pressure continues to observe the exact pinned basis and produces
  byte-identical receipts and facts for its full legal lifetime.
- Adversarial denial: once the pinned generation is retired, the same context
  fails closed with a typed stale-basis stop and never silently refreshes.
- Adversarial residue: hostile pin / retire schedules prove exact-zero orphaned
  generations and exact-zero unretired pins through runtime-owned counters,
  not inferred or hard-coded values.

**Engineering decisions**
- Pin identity is a real runtime artifact with explicit generation semantics,
  not a formatted `snapshot_token` convention.
- Shared-read contexts remain cheap consumer artifacts, but the thing they hold
  is runtime-owned structural authority rather than copied derived rows.

**Open questions**
- None.

### Phase 12: Typed Submission Journal Identity And Consumer Replay Closure Boundary

Close the submission seam as a real journaled authority lane: typed journal
position identity on committed receipts, typed journal-segment identity for
consumer replay requests, and a public replay surface that re-derives through
the runtime instead of asking consumers or tests to infer replay from commit
string conventions.

**Relevant subsystems**
- submission intake and receipt identity
- journal segment identity
- consumer-facing replay surface

**Relevant Query source surfaces**
- [runtime/workspace_submission.rs](../../crates/forge-query/src/runtime/workspace_submission.rs)
- [runtime/backend/receipts.rs](../../crates/forge-query/src/runtime/backend/receipts.rs)
- [runtime/public_api.rs](../../crates/forge-query/src/runtime/public_api.rs)

**Relevant APIs and product surfaces**
- typed journal-position accessors on submission receipts
- typed journal-segment / replay-request product surfaces on the workspace
  facade
- consumer replay results returned through the ordinary receipt / envelope
  vocabulary

**Warnings**
- Do not keep treating `commit_identity` string suffixes as journal order;
  journal order must be a typed product surface.
- Do not expose raw journal internals when the consumer need is replay outcome.
- Do not build a second replay semantics path beside ordinary submission,
  admission, execution, and receipt lowering.

**Test requirements**
- Add a `Typed Journal Identity And Consumer Replay Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: a multi-producer submission schedule records typed
  journal positions whose replay reconstructs identical truth, receipts,
  published artifacts, and digests.
- Adversarial rejection: stale-basis replay, unknown journal segment identity,
  and cross-scheme replay identity requests all fail typed and leave zero
  journal residue.
- Adversarial localization: no certification or support proof may derive
  journal order by parsing `commit_identity` text.

**Engineering decisions**
- Journal order is represented as a Query-owned typed artifact, not as a naming
  convention emergent from mutation receipts.
- Consumer replay is a first-class public Query lane because downstream refolds
  are precisely the folklore this milestone is supposed to eliminate.

**Open questions**
- None.

### Phase 13: Certification Honesty And Public-Bridge Reader-Lane Closure Boundary

Close the milestone honestly by forcing every remaining proof to run through
the real product lanes: exact-zero hostile counters must be runtime-observed,
public-bridge certification must consume published artifacts through typed
projection consumption rather than row spelunking, and the milestone may not
claim closure until modeled proofs are gone.

**Relevant subsystems**
- hostile certification harness
- public-bridge runtime certification harness
- projection-consumption product lane

**Relevant Query source surfaces**
- [runtime/tests/support/bridge/hostile_certification.rs](../../crates/forge-query/src/runtime/tests/support/bridge/hostile_certification.rs)
- [runtime/tests/support/bridge/hostile_certification_schedule.rs](../../crates/forge-query/src/runtime/tests/support/bridge/hostile_certification_schedule.rs)
- [tests/support/public_bridge_runtime/hostile_certification.rs](../../crates/forge-query/tests/support/public_bridge_runtime/hostile_certification.rs)

**Relevant APIs and product surfaces**
- hostile exact-zero counter evidence
- public-bridge hostile certification artifact
- published-artifact consumption through projection-consumption receipts

**Warnings**
- Do not leave hard-coded zero helpers in the certification path; that is a
  fake proof.
- Do not let public-bridge certification read materialization rows directly
  from bindings when the milestone claims projection-consumption isolation.
- Do not count a phase test as sufficient if it can still pass after the
  underlying runtime guarantee regresses.

**Test requirements**
- Add a `Certification Honesty Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: the runtime hostile matrix and the public-bridge
  hostile matrix both produce replay-stable artifacts while consuming published
  derived facts only through the typed reader lane.
- Adversarial proof integrity: each exact-zero hostile counter is sourced from
  runtime-owned measurement state, and sabotage tests prove the certification
  fails when those counters are intentionally perturbed.
- Adversarial boundary localization: public-bridge hostile certification loses
  compile access to any direct internal materialization-reading shortcuts that
  would bypass projection consumption.

**Engineering decisions**
- The milestone closes on proof honesty, not merely on feature-shaped surface
  availability.
- A certification artifact that does not become false under the targeted
  regression is not acceptance evidence.

**Open questions**
- None.

## Must Ship

- authority-lane-decomposed backend contracts with `Send + Sync` read lanes
- sealed basis-bound shared read contexts with generation-pinned snapshots
  and lock-free committed reads backed by runtime-owned pin / retire substrate
- the deterministic submission seam with journal-ordered receipts over the
  existing admission lattice, including typed journal identity and the
  consumer-facing journal-segment replay surface
- the published derived-artifact rule with reader evaluation structurally
  impossible
- the re-expressed workspace facade with unchanged existing consumer surface
  and fail-closed admission for the new families
- the hostile concurrency/determinism certification matrix with runtime-owned
  exact-zero counters and public-bridge reader-lane honesty

## Must Preserve

- canonical query meaning across serialized and concurrent execution —
  timing may change, values may not
- lower-crate authority boundaries: relational owns truth, signal owns
  evaluation, the bridge owns causal routing; only access topology moves
- the single-owner workspace as a first-class consumer surface
- merged `9.4` temporal/async meaning and `9.5` projection-consumption
  semantics inside the published-artifact lane
- basis capability lifecycle as the only path to read authority

## Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the Milestone `9.7` certification suites added to
  [test-requirements.md](./test-requirements.md) pass with narrow
  machine-checkable artifacts
- N concurrent readers under write pressure produce byte-identical receipts
  and results to serialized execution, with exact-zero lock and
  reader-evaluation counters
- shared-read authority is backed by runtime-owned generation pinning and
  retirement, with typed stale-basis denial and exact-zero residue counters
- journal replay reconstructs identical truth, receipts, and published
  artifacts through typed journal identity rather than parsed receipt strings
- existing downstream consumers compile unchanged against the re-expressed
  facade
- the new facade families carry honest support/admission rows that fail
  closed where unbacked
- public-bridge hostile certification consumes published derived artifacts
  through the typed projection-consumption lane rather than direct
  materialization reads

## Sequencing Notes

- This milestone belongs after [milestone-9.6.md](./milestone-9.6.md) because
  journal identity, submission receipts, and published-artifact digests must
  be born on the canonical evidence-identity scheme.
- It belongs before Milestone `10` as a hard gate: store-backed execution
  must inherit lane-correct adapter contracts and the concurrency topology,
  not retrofit `Send` boundaries through frozen store-backed shapes.
- Durable journal persistence, store-backed replay reconstruction, and
  restart-stable published-artifact reload remain Milestone `10`/`11` scope.
