# forge-signal Compile-Time Safety Spec

> **Status:** Pre-production. Compile-time bug class elimination.
>
> **Parent:** [signal_architecture.md](./signal_architecture.md)
>
> **Goal:** Use Rust's type system to make entire classes of bugs impossible to write. These are not code cleanup — they are structural changes that move runtime invariant checks into the compiler.

---

## Table of Contents

### Phase 3 — Foundational Compile-Time Safety

1. [R16: Branded `NodeRef<'g>` — Topological Dementia Prevention](#r16-branded-noderefg--topological-dementia-prevention)
2. [R17: `ScopedVersion` Witness — Granularity False Negative Prevention](#r17-scopedversion-witness--granularity-false-negative-prevention)
3. [R18: Private State Setters — State Machine Fracture Prevention](#r18-private-state-setters--state-machine-fracture-prevention)

### Deferred — Bug Classes Requiring Decoupling or Future Features

4. [R23: Affine Topology Tokens — Causal Inversion Prevention](#r23-affine-topology-tokens--causal-inversion-prevention)
5. [R24: `DeterministicMap<K, V>` — Nondeterminism Prevention](#r24-deterministicmapk-v--nondeterminism-prevention)
6. [R25: Generative Branch Isolation — Branch State Leakage Prevention](#r25-generative-branch-isolation--branch-state-leakage-prevention)
7. [R26: Branded `Version<'timeline>` — Monotonicity Violation Prevention](#r26-branded-versiontimeline--monotonicity-violation-prevention)
8. [R27: `DeduplicatedPlan` Newtype — Cascade Amplification Guard](#r27-deduplicatedplan-newtype--cascade-amplification-guard)
9. [R28: Quotient Types — Tolerance Poisoning & Structural Identity Divergence](#r28-quotient-types--tolerance-poisoning--structural-identity-divergence)
10. [R29: Discovery/Wiring Decoupling — Mid-Evaluation Safety](#r29-discoverywiring-decoupling--mid-evaluation-safety)
11. [R30: Linear Computational Fuel — Convergence Failure Prevention](#r30-linear-computational-fuel--convergence-failure-prevention)
12. [R31: Branded `FrameValue<'epoch>` — Temporal Aliasing Prevention](#r31-branded-framevalueepoch--temporal-aliasing-prevention)
13. [R32: Strategy Marker Traits — Evaluation Strategy Mismatch Prevention](#r32-strategy-marker-traits--evaluation-strategy-mismatch-prevention)
14. [R33: `CoalescedPlan` Newtype — Depth Explosion Guard](#r33-coalescedplan-newtype--depth-explosion-guard)
15. [Shared Abstraction Map](#shared-abstraction-map)
16. [Summary](#summary)

---

## Phase 3 — Foundational Compile-Time Safety

## R16: Branded `NodeRef<'g>` — Topological Dementia Prevention

### Problem

`NodeId` is `Copy` and carries no lifetime. After a node is tombstoned and its slot reused with a new generation, any stale `NodeId` held by external code silently points at a reincarnated node. The generation check in `is_alive()` catches this at runtime, but it is scattered across **34 call sites** — each one a manual, forgettable guard against ghost references.

### Evidence

| Call Site Category                                              | Count | Risk                                               |
| --------------------------------------------------------------- | ----- | -------------------------------------------------- |
| Invalidation traversal (`invalidation.rs`)                      | 3     | High — ghost edges corrupt propagation             |
| Evaluation / result apply (`result_apply.rs`, `suppression.rs`) | 4     | High — ghost deps cause panics or silent staleness |
| Lifecycle / GC (`lifecycle.rs`, `storage.rs`)                   | 5     | Medium — GC already expects dead references        |
| Planner / validation (`validation.rs`, `plan_builder.rs`)       | 4     | High — plans built on dead nodes                   |
| Explain / diagnostics                                           | 3     | Low — read-only                                    |
| Tests                                                           | 15    | N/A                                                |

### Design

Split `NodeId` into two types:

```rust
/// Internal, unbounded — only the graph's own storage uses this.
pub(crate) struct RawNodeId {
    index: u32,
    generation: u32,
}

/// External, lifetime-branded — issued to callers.
/// Borrows the graph, so any `&mut SignalGraph` call
/// (unregister, GC, compact) invalidates all outstanding refs.
#[derive(Copy, Clone)]
pub struct NodeRef<'g> {
    raw: RawNodeId,
    _brand: PhantomData<&'g SignalGraph>,
}
```

All public API methods that accept or return node handles use `NodeRef<'g>`. Internal edge storage (`Vec<DependencyEdge>`, subscriber lists) uses `RawNodeId`.

Any code that tries to use a `NodeRef` after calling `unregister_node()` or `run_gc_epoch()` (which take `&mut self`) **fails to compile** because the shared borrow `'g` conflicts with the exclusive borrow.

### What It Catches

- External code holding a handle across a topology mutation → **compile error**
- Passing a `NodeRef` from graph A into graph B → **compile error** (different lifetimes)

### What It Cannot Catch

Internal ghost edges within the graph's own `Vec<RawNodeId>` storage. These remain a runtime invariant managed by `is_alive()` checks inside the graph module — but that surface is small and auditable (~5 internal call sites vs 34 total today).

### Shared Abstraction

This is an instance of the **`BrandedHandle<'scope, T>`** meta-abstraction (see [Meta-Abstractions in signal_architecture.md](./signal_architecture.md#meta-abstractions)). The same pattern is reused by R21 (`SubscriberCache<'epoch>`), R26 (`Version<'timeline>`), R25 (`BranchConfig<'branch>`), and R31 (`FrameValue<'epoch>`).

### Bug Class Eliminated

"Topological Dementia" — Ghost edges and reincarnation confusion from external code holding stale handles.

---

## R17: `ScopedVersion` Witness — Granularity False Negative Prevention

### Problem

The engine can skip an evaluation it should have run because the invalidation path and the validation path use **different granularity** to answer "did this dependency meaningfully change?"

Currently `AspectVersion::get(aspect)` returns a raw `u64`. Invalidation checks partition scopes via `subscribes_to_aspect()`. But validation in `count_meaningful_input_changes()` compares the raw `u64` globally — it does not consider which partition scope the downstream cares about.

Because version extraction and scope matching are **decoupled types**, nothing prevents a developer from writing validation code that compares versions without considering scopes.

### Design

Make it impossible to obtain a comparable version without specifying the scope:

```rust
/// Opaque version witness. Cannot be constructed manually.
/// The ONLY way to get one is through `scoped_version()`.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ScopedVersion {
    version: u64,
    _private: (), // prevents manual construction
}

impl NodeEntry {
    /// The old `get_aspect_version().get(aspect) -> u64` is removed.
    /// You MUST specify the scope you are asking about.
    pub fn scoped_version(
        &self,
        aspect: Aspect,
        scope: Option<&PartitionSubscription>,
    ) -> ScopedVersion { ... }
}
```

Any code that tries to compare dependency versions without specifying a scope **fails to compile** because there is no way to construct a `ScopedVersion` without calling `scoped_version(aspect, scope)`.

Both the invalidation path and the validation path are now **structurally forced** to use the same version extraction call, making it impossible for them to disagree on granularity.

### Bug Class Eliminated

"Granularity False Negatives" — Silent staleness from validation code that skips scope-aware version comparison.

---

## R18: Private State Setters — State Machine Fracture Prevention

### Problem

`NodeEntry` has three correlated fields that represent a single conceptual state:

- `state: NodeState` (Clean / Dirty / MaybeStale)
- `dirty_aspects: AspectMask`
- `dirty_partition_scopes: SmallVec<(Aspect, PartitionSubscription)>`

All three have public setters (`set_state()`, `set_dirty_aspects()`, `clear_dirty_partition_scopes()`). This means external code can update one field without the others, creating a fractured state.

### Evidence

| Raw setter bypass                                                                  | File                | Line                         | What it does wrong                                                       |
| ---------------------------------------------------------------------------------- | ------------------- | ---------------------------- | ------------------------------------------------------------------------ |
| `set_state(Dirty)` without aspects/scopes                                          | `lifecycle.rs`      | L62                          | Sets subscriber Dirty but doesn't set which aspects or scopes            |
| `set_state(MaybeStale)` alone                                                      | `prepared_apply.rs` | L138                         | Deferred condition sets MaybeStale without updating dirty_aspects        |
| `set_state(Clean)` + `set_dirty_aspects(EMPTY)` + `clear_dirty_partition_scopes()` | `apply.rs`          | L281–283                     | Manual 3-step ceremony duplicated — forgetting one step creates fracture |
| `set_state(Clean)` + `set_dirty_aspects(EMPTY)` in easy API                        | `easy/mod.rs`       | L143–144, L204–205, L328–329 | Same 3-step ceremony duplicated 3 more times                             |

Total: **10 raw `set_state()` calls** and **10 raw `set_dirty_aspects(EMPTY)` calls** outside of the transition methods.

### Design

Make the raw setters private. The only public mutation surface becomes the three transition methods:

```rust
impl NodeEntry {
    // PRIVATE — cannot be called outside this module
    fn set_state(&mut self, state: NodeState) { ... }
    fn set_dirty_aspects(&mut self, mask: AspectMask) { ... }
    fn clear_dirty_partition_scopes(&mut self) { ... }

    // PUBLIC — the only legal ways to change a node's state
    pub fn transition_clean(&mut self) { ... }
    pub fn transition_dirty(&mut self, aspect: Aspect, scopes: &[PartitionSubscription]) { ... }
    pub fn transition_maybe_stale(&mut self, aspect: Aspect) { ... }
}
```

Code like `entry.set_state(NodeState::Dirty)` **fails to compile** because `set_state` is private. The developer is forced to call `entry.transition_dirty(aspect, scopes)`, which atomically updates all three correlated fields.

For `lifecycle.rs:62` (unregister marks subscribers dirty), the fix is `entry.transition_dirty(Aspect::all(), &[])` — explicit about what is being dirtied.

### Bug Class Eliminated

"State Machine Fracture" — Partial mutation poisoning where some state fields update but correlated fields do not.

---

## Deferred — Bug Classes Requiring Decoupling or Future Features

> [!IMPORTANT]
> These items are deferred because they either: (a) depend on executor infrastructure built in Phase 2 (R8/R9), (b) depend on features that don't exist yet (branches, frame semantics), or (c) are domain-side rather than engine-side. Each item documents what must be **decoupled** to enable compile-time enforcement and which **shared abstractions** it reuses.

---

## R23: Affine Topology Tokens — Causal Inversion Prevention

### Bug Class

"Causal Inversion (Glitches)" — A node that depends on both A and B is evaluated after A updates but before B's transitive effects from A have settled. The node sees a combination of old-B and new-A that never existed as a consistent state.

### Why Deferred

Depends on the executor (R8/R9). The token mechanism must be integrated into the plan execution loop, which doesn't exist yet in its final form.

### Required Decoupling: Plan Construction vs Plan Execution

The reason this is "partial" today is that the graph topology is dynamic, so the planner's sort is a runtime computation. But the _execution_ of that sort can be made provably correct at compile time if we **decouple the two concerns**:

- **Plan Construction** (runtime): Produces a `TypedExecutionSchedule` — a sequence of typed stages. Dynamic topology is handled here.
- **Plan Execution** (compile-time enforcement): The executor consumes stages through a token-passing protocol.

### How Compile-Time Enforcement Works

```rust
/// A zero-cost token proving that NodeId has finished evaluating.
/// CANNOT be Clone'd. CANNOT be constructed except by evaluate_node().
/// Must be moved (consumed) — Rust's affine type system enforces this.
pub struct SettledProof(NodeId);

/// The execution function physically requires you to present
/// settled proofs for every dependency this node has.
/// If you don't have the proof, you can't call this function.
pub fn evaluate_node(
    node: NodeId,
    proofs: Vec<SettledProof>, // requirement is compile-time; contents validated at runtime
) -> SettledProof {
    // ... evaluation logic ...
    SettledProof(node) // proof is produced only after successful evaluation
}
```

**Why this works:** `SettledProof` has no `Clone` impl and a private constructor. The only way to obtain one is by successfully evaluating a node. The only way to evaluate a node is by presenting proofs for its dependencies. If you try to evaluate node C before its dependency B has produced a proof, you don't have the token to pass — the code won't compile (or won't have a valid value at the call site).

**For pure compile-time (no runtime Vec check):** Use type-level HLists via `frunk`:

```rust
/// Each stage produces a type-level proof.
/// Stage 2 literally cannot be called without Stage 1's output type.
fn execute_stage_1(graph: &mut G) -> Settled<Stage1> { ... }
fn execute_stage_2(graph: &mut G, _proof: Settled<Stage1>) -> Settled<Stage2> { ... }
fn execute_stage_3(graph: &mut G, _proof: Settled<Stage2>) -> Settled<Stage3> { ... }
```

### Shared Abstraction

Instance of the **`LinearToken<T>`** pattern (affine/must-use resources). Same pattern as R30 (Fuel) and R22 (UndoToken).

---

## R24: `DeterministicMap<K, V>` — Nondeterminism Prevention

### Bug Class

"Nondeterminism Under Reordering" — Two runs with identical input produce different outputs because of HashMap iteration order, dedup order, or parallel chunk boundaries.

### Why Deferred

No architectural dependency. This is a collection-level concern that can be addressed independently at any time.

### Required Decoupling: Deterministic Ordering vs Parallelism Strategy

- **Deterministic layer:** `DeterministicMap<K, V>` wraps `HashMap` but only exposes `sorted_iter()` and `sorted_keys()`. Direct `.iter()` on the inner map is private — **compile error**.
- **Parallelism layer:** `par_chunks` boundaries don't affect logical ordering because the merge step uses the `DeterministicMap`'s canonical ordering.

### How Compile-Time Enforcement Works

```rust
/// Wraps a HashMap. Direct iteration is impossible.
pub struct DeterministicMap<K: Ord, V> {
    inner: HashMap<K, V>, // private
}

impl<K: Ord, V> DeterministicMap<K, V> {
    /// The ONLY way to iterate. Always sorted.
    pub fn sorted_iter(&self) -> impl Iterator<Item = (&K, &V)> {
        let mut pairs: Vec<_> = self.inner.iter().collect();
        pairs.sort_by_key(|(k, _)| *k);
        pairs.into_iter()
    }
    // No `.iter()` method exists. No `IntoIterator` impl.
}
```

**Why this works:** Code that writes `for (k, v) in map.iter()` (unsorted HashMap iteration) gets a compile error because `DeterministicMap` doesn't expose `.iter()`. The developer is forced to use `sorted_iter()`.

---

## R25: Generative Branch Isolation — Branch State Leakage Prevention

### Bug Class

"Branch State Leakage" — State from one branch bleeds into another through shared references, cloned-but-stale config snapshots, or inheritance during branch operations.

### Why Deferred

Branch support is a future feature. The `GhostCell`/`LCell` pattern can be applied when branches are implemented.

### How Compile-Time Enforcement Works

Uses the `qcell` crate's **Generative Lifetimes** — the Rust compiler generates a UNIQUE lifetime `'id` for every closure invocation. It is mathematically impossible for `'id` to overlap with another branch's lifetime.

```rust
use qcell::{LCell, LCellOwner};

/// The brand `'id` ensures this config ONLY exists in this specific branch.
/// You cannot extract it from one branch and use it in another.
pub struct BranchConfig<'id> {
    data: LCell<'id, InternalConfig>,
}

/// Each call to `with_new_branch` gets a unique `'id` at compile time.
/// The `for<'id>` quantifier means the closure must work for ANY lifetime,
/// which means it cannot assume `'id` equals any other branch's lifetime.
pub fn with_new_branch<F, T>(f: F) -> T
where
    F: for<'id> FnOnce(&mut LCellOwner<'id>, BranchConfig<'id>) -> T
{
    LCellOwner::scope(|mut owner| {
        let config = BranchConfig { data: LCell::new(InternalConfig::default()) };
        f(&mut owner, config)
    })
}
```

**Why this works:** If you try to take `BranchConfig<'a>` from Branch A and store it in Branch B's context (which uses `'b`), the compiler rejects the assignment because `'a != 'b`. The lifetimes are universally quantified — they cannot unify.

### Shared Abstraction

Instance of the **`BrandedHandle<'scope, T>`** meta-abstraction, specialized with `GhostCell` for interior mutability across the brand boundary.

---

## R26: Branded `Version<'timeline>` — Monotonicity Violation Prevention

### Bug Class

"Monotonicity Violation" — Version counters are assumed monotonically increasing, but rollback resets them to earlier states. Cross-timeline comparisons produce nonsense.

### Why Deferred

Only matters once rollback and branches exist. The branded version can be added when those features land.

### How Compile-Time Enforcement Works

```rust
/// A version that is only comparable with other versions from the same timeline.
/// After rollback, the timeline brand changes.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Version<'timeline> {
    value: u64,
    _brand: PhantomData<&'timeline ()>,
}

impl SignalGraph {
    /// Advancing a transaction creates a new timeline brand.
    pub fn with_transaction<F, T>(&mut self, f: F) -> T
    where
        F: for<'tx> FnOnce(&mut TransactionContext<'tx>) -> T
    {
        // rollback returns to the old timeline;
        // commit advances to a new one.
        // Either way, 'tx expires and old versions become unusable.
        ...
    }
}
```

**Why this works:** `Version<'a>` from before rollback and `Version<'b>` from after rollback have different lifetime brands. Comparing them is a compile error because `PartialOrd` requires matching lifetimes.

### Shared Abstraction

Instance of **`BrandedHandle<'scope, T>`**.

---

## R27: `DeduplicatedPlan` Newtype — Cascade Amplification Guard

### Bug Class

"Cascade Amplification" — A single root change fans out through a diamond/lattice graph, and the evaluation work grows exponentially because shared subexpressions are evaluated multiple times.

### Why Deferred

Depends on planner infrastructure (R8). The deduplication logic must be integrated into the plan builder.

### How Compile-Time Enforcement Works

This bug class is **fundamentally algorithmic** — the compiler cannot verify that deduplication was done correctly. But it CAN verify that deduplication was **attempted**:

```rust
/// Can ONLY be constructed by `deduplicate()`. No public constructor.
pub struct DeduplicatedPlan {
    inner: EvaluationPlan,
}

/// The executor ONLY accepts DeduplicatedPlan, not raw EvaluationPlan.
pub fn execute(plan: DeduplicatedPlan) -> Result<(), Error> { ... }

/// The ONLY way to construct a DeduplicatedPlan.
pub fn deduplicate(raw: EvaluationPlan) -> DeduplicatedPlan {
    // ... merge shared subexpressions ...
    DeduplicatedPlan { inner: optimized }
}
```

**Why this works:** Code that tries to pass a raw `EvaluationPlan` to `execute()` gets a compile error. The developer is forced to call `deduplicate()` first. The deduplication logic itself is runtime, but _forgetting to call it_ is a compile error.

---

## R28: Quotient Types — Tolerance Poisoning & Structural Identity Divergence

### Bug Class

"Tolerance Poisoning" — BRep healing produces a vertex at `1.0000000000001` instead of `1.0`. The DAG's version tracking uses exact comparison and bumps the version, causing unnecessary re-evaluation of the entire downstream tree.

"Structural Identity Divergence" — A boolean union produces the same shape but with a different topological decomposition (6 faces vs 4 faces). The DAG sees "topology changed" and re-evaluates 197 downstream operations.

### Why Deferred

Domain-specific. The hook (`OutputIdentity`/`VersionComparatorPolicy`) already exists in the engine. The Quotient Type wrapper is a domain-side concern, applied by geometry kernel consumers.

### How Compile-Time Enforcement Works

A **Quotient Type** is a set grouped by an equivalence relation. In Rust, we simulate this by hiding the raw data behind a newtype that only implements `PartialEq` through a domain axiom.

```rust
/// The raw geometry data is PRIVATE. You cannot do `a.0 == b.0`.
/// The only way to compare is through the domain-defined equivalence.
pub struct GeometricOutput<T>(T);

/// You MUST implement this trait to define what "equal" means for your domain.
/// Without it, `GeometricOutput<T>` does not implement `PartialEq` at all.
pub trait DomainEquivalence {
    /// Define the equivalence class. For geometry: bounded volume comparison.
    /// For topology: surface integral equivalence. For chip sim: logic table match.
    fn is_equivalent(a: &Self, b: &Self) -> bool;
}

impl<T: DomainEquivalence> PartialEq for GeometricOutput<T> {
    fn eq(&self, other: &Self) -> bool {
        // Enforced domain-specific comparison.
        // Strict bitwise `==` is statically inaccessible.
        T::is_equivalent(&self.0, &other.0)
    }
}
```

**Why this works:** The DAG's `output_identity` comparison requires `T: PartialEq`. If you wrap your output in `GeometricOutput<T>`, the compiler refuses to compare unless `T: DomainEquivalence`. The developer is _forced_ to define what "same" means. Bitwise equality is structurally inaccessible because the inner field is private.

---

## R29: Discovery/Wiring Decoupling — Mid-Evaluation Safety

### Bug Class

"Mid-Evaluation Dependency Discovery" — During parallel evaluation, an entity's AI node discovers it needs to read another entity's physics collider. Both parallel tasks try to wire new subscriber edges to the same upstream during the apply phase.

### Why Deferred

Depends on parallel execution infrastructure (R9).

### Required Decoupling: Discovery vs Wiring

The reason this is "partial" is that discovery (per-task, parallelizable) and wiring (must be serial, touches shared graph state) are fused in `PreparedDependencyCapture`.

- **Discovery** (parallel, isolated): Each task accumulates `DiscoveredDependency` records in a local `Vec`. No graph access needed. `Send + Sync`.
- **Wiring** (serial, exclusive): After all parallel tasks complete, a single `fn wire_discoveries(&mut self, batch: Vec<DiscoveredDependency>)` applies all discovered edges serially.

```rust
/// Per-task, no graph reference. Can be produced concurrently.
pub struct DiscoveredDependency {
    pub consumer: NodeId,
    pub source: NodeId,
    pub aspect: Aspect,
    pub scope: Option<PartitionSubscription>,
}

impl SignalGraph {
    /// Takes &mut self — the borrow checker ensures no concurrent access.
    pub fn wire_discoveries(&mut self, batch: Vec<DiscoveredDependency>) {
        for dep in batch {
            self.add_dependency_edge(dep.consumer, dep.source, dep.aspect, dep.scope);
        }
    }
}
```

**Why this works:** The borrow checker prevents anyone from holding `&mut SignalGraph` during the parallel discovery phase. `DiscoveredDependency` has no graph reference — it's just data. The wiring phase requires `&mut self`, forcing serial execution.

---

## R30: Linear Computational Fuel — Convergence Failure Prevention

### Bug Class

"Fixed-Point Convergence Failure" — Two combinational gates form a feedback loop. Each delta-cycle, both outputs flip. The engine evaluates, invalidates, re-evaluates forever. No error, no stack overflow — it just hangs.

### Why Deferred

Depends on executor loop structure (R8/R9).

### How Compile-Time Enforcement Works

We encode the halting guarantee using a **linear** (un-cloneable, un-forgeable) resource token that the Rust compiler tracks through every code path.

```rust
/// An uncloneable, unforgeable budget ticket.
/// Because `Fuel` does not implement `Clone`, the compiler tracks
/// exactly how many units remain at every point in the program.
pub struct Fuel(usize);

impl Fuel {
    /// Consuming fuel MOVES self, returning a new (decremented) Fuel.
    /// If 0, returns Err — the engine MUST handle the error.
    pub fn consume(self) -> Result<Fuel, ConvergenceError> {
        if self.0 == 0 {
            Err(ConvergenceError::ExceededBudget)
        } else {
            Ok(Fuel(self.0 - 1))
        }
    }
}

/// The engine MUST pass fuel in, and MUST yield fuel out.
/// A loop that re-evaluates MUST call fuel.consume() each iteration.
pub fn evaluate_stage(
    graph: &mut SignalGraph,
    stage: Stage,
    fuel: Fuel,           // moved in
) -> Result<(StageOutput, Fuel), Error> {  // moved out
    let remaining = fuel.consume()?;  // mandatory
    // ... evaluation logic ...
    Ok((output, remaining))
}
```

**Why this works:** Any loop that calls `evaluate_stage` must thread `Fuel` through. Because `Fuel` has no `Clone`, the loop cannot "cheat" by reusing old fuel. Each iteration consumes exactly one unit. When fuel reaches 0, the `?` operator forces the loop to return an error. The compiler guarantees that every execution path either terminates normally or hits the fuel limit — infinite loops are structurally impossible.

### Shared Abstraction

Instance of the **`LinearToken<T>`** pattern. Same pattern as R23 (SettledProof) and R22 (MutationReceipt).

---

## R31: Branded `FrameValue<'epoch>` — Temporal Aliasing Prevention

### Bug Class

"Temporal Aliasing" — Frame N's physics writes new positions while Frame N's rendering reads positions. Without distinguishing "last frame's stable value" from "this frame's in-progress value," a render node sees a mix of frame N and frame N-1 data.

### Why Deferred

Depends on game-engine frame semantics, which isn't implemented yet. The `AspectVersion` currently has no concept of temporal epochs.

### How Compile-Time Enforcement Works

```rust
/// A value branded with its frame epoch.
/// Values from different frames are DIFFERENT TYPES.
pub struct FrameValue<'epoch, T> {
    value: T,
    _epoch: PhantomData<&'epoch ()>,
}

impl SignalGraph {
    /// Begin a new frame. Returns a handle branded with THIS frame's lifetime.
    pub fn begin_frame<F, T>(&mut self, f: F) -> T
    where
        F: for<'frame> FnOnce(&mut FrameContext<'frame>) -> T
    {
        // 'frame is unique to this invocation.
        // After f returns, all FrameValue<'frame, _> become unusable.
        ...
    }
}

impl<'frame> FrameContext<'frame> {
    /// Read committed value from PREVIOUS frame.
    pub fn read_committed(&self, node: NodeId) -> FrameValue<'static, T> { ... }

    /// Read in-progress value from THIS frame.
    pub fn read_pending(&self, node: NodeId) -> FrameValue<'frame, T> { ... }
}
```

**Why this works:** `FrameValue<'a, T>` and `FrameValue<'b, T>` are different types when `'a != 'b`. Mixing frame N-1 values with frame N values in the same computation is a compile error because the lifetime brands don't unify.

### Shared Abstraction

Instance of **`BrandedHandle<'scope, T>`**.

---

## R32: Strategy Marker Traits — Evaluation Strategy Mismatch Prevention

### Bug Class

"Evaluation Strategy Mismatch" — A lazy node gets eagerly pulled in by a diagnostic subscriber, or an eager node is skipped because nothing demanded it this frame.

### Why Deferred

Depends on execution engine architecture (R9).

### Required Decoupling: Strategy Declaration vs Evaluation Pass

- **Strategy Declaration** (per-node, at registration): Marker types on node handles.
- **Evaluation Pass** (per-phase): Separate executor types that only accept matching markers.

```rust
pub struct EagerTag;
pub struct LazyTag;
pub struct EventDrivenTag;

/// Registration returns a typed handle.
pub fn register_eager(&mut self, ...) -> NodeRef<'_, EagerTag> { ... }
pub fn register_lazy(&mut self, ...) -> NodeRef<'_, LazyTag> { ... }

/// The eager evaluator only accepts eager nodes.
pub fn evaluate_eager(&mut self, nodes: &[NodeRef<'_, EagerTag>]) { ... }

/// A transitive read across a strategy boundary requires an explicit bridge.
pub struct StrategyBridge<From, To> { ... }
```

**Why this works:** Calling `evaluate_eager()` with a `NodeRef<'_, LazyTag>` is a type mismatch — compile error. Transitive pulls across strategy boundaries must go through an explicit `StrategyBridge`, which is its own auditable node type.

---

## R33: `CoalescedPlan` Newtype — Depth Explosion Guard

### Bug Class

"Linear Chain Depth Explosion" — A 200-deep linear feature tree creates 200 planner stages with 1 task each. Per-stage overhead dominates actual computation time.

### Why Deferred

Depends on planner infrastructure (R8).

### How Compile-Time Enforcement Works

Same newtype-guard pattern as R27:

```rust
/// Can ONLY be constructed by `coalesce()`. No public constructor.
pub struct CoalescedPlan {
    inner: DeduplicatedPlan,
}

/// The executor ONLY accepts CoalescedPlan.
pub fn execute(plan: CoalescedPlan) -> Result<(), Error> { ... }

/// The ONLY way to construct a CoalescedPlan.
/// Collapses linear chains into single-stage sequential pipelines.
pub fn coalesce(plan: DeduplicatedPlan) -> CoalescedPlan {
    // ... detect linear chains, merge into sequential pipelines ...
    CoalescedPlan { inner: optimized }
}
```

**Why this works:** The executor requires `CoalescedPlan`, which requires `DeduplicatedPlan`, which requires `EvaluationPlan`. Forgetting any optimization step is a compile error because the types don't match. This forms a **pipeline of newtype transformations**: `EvaluationPlan → DeduplicatedPlan → CoalescedPlan`.

---

## Shared Abstraction Map

> [!TIP]
> These four meta-abstraction patterns solve **all** of the deferred bug classes. Implementing one instance of each pattern provides a reusable foundation for the rest.

### Pattern 1: `BrandedHandle<'scope, T>`

| Instantiation             | R-Item         | Bug Class              |
| ------------------------- | -------------- | ---------------------- |
| `NodeRef<'graph>`         | R16            | Topological Dementia   |
| `SubscriberCache<'epoch>` | R21 (arch doc) | Representational Drift |
| `Version<'timeline>`      | R26            | Monotonicity Violation |
| `FrameValue<'epoch>`      | R31            | Temporal Aliasing      |
| `BranchConfig<'branch>`   | R25            | Branch State Leakage   |

### Pattern 2: `LinearToken<T>`

| Instantiation          | R-Item         | Bug Class           |
| ---------------------- | -------------- | ------------------- |
| `SettledProof(NodeId)` | R23            | Causal Inversion    |
| `Fuel(usize)`          | R30            | Convergence Failure |
| `MutationReceipt<T>`   | R22 (arch doc) | Rollback Amnesia    |

### Pattern 3: `NewtypeGuard<T> (pipeline of transformations)`

| Instantiation      | R-Item | Bug Class             |
| ------------------ | ------ | --------------------- |
| `DeduplicatedPlan` | R27    | Cascade Amplification |
| `CoalescedPlan`    | R33    | Depth Explosion       |

### Pattern 4: `QuotientType<T, Eq> (information hiding + trait axiom)`

| Instantiation             | R-Item | Bug Class                      |
| ------------------------- | ------ | ------------------------------ |
| `GeometricOutput<BRep>`   | R28    | Tolerance Poisoning            |
| `TopologicalOutput<Mesh>` | R28    | Structural Identity Divergence |

---

## Summary

| #   | Refactor                     | Compile-Time?        | Paradigm                  | Bug Class Eliminated                 |
| --- | ---------------------------- | -------------------- | ------------------------- | ------------------------------------ |
| R16 | Branded `NodeRef<'g>`        | Partial              | BrandedHandle             | Topological Dementia                 |
| R17 | `ScopedVersion` witness      | Full                 | Opaque Witness            | Granularity False Negatives          |
| R18 | Private state setters        | Full                 | Visibility                | State Machine Fracture               |
| R23 | Affine Topology Tokens       | Full                 | LinearToken               | Causal Inversion                     |
| R24 | `DeterministicMap`           | Full                 | Newtype                   | Nondeterminism                       |
| R25 | Generative Branch Isolation  | Full                 | BrandedHandle + GhostCell | Branch State Leakage                 |
| R26 | Branded `Version<'timeline>` | Full                 | BrandedHandle             | Monotonicity Violation               |
| R27 | `DeduplicatedPlan` newtype   | Partial (guard only) | NewtypeGuard              | Cascade Amplification                |
| R28 | Quotient Types               | Full                 | QuotientType              | Tolerance Poisoning & Identity Drift |
| R29 | Discovery/Wiring split       | Full                 | Borrow checker            | Mid-Eval Discovery                   |
| R30 | Linear Fuel                  | Full                 | LinearToken               | Convergence Failure                  |
| R31 | Branded `FrameValue<'epoch>` | Full                 | BrandedHandle             | Temporal Aliasing                    |
| R32 | Strategy Marker Traits       | Partial              | Marker traits             | Strategy Mismatch                    |
| R33 | `CoalescedPlan` newtype      | Partial (guard only) | NewtypeGuard              | Depth Explosion                      |
