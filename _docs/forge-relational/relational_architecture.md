# forge-relational DX Engineering Spec

This document is a complete refactoring specification for `forge-relational`. It synthesizes an exhaustive architectural review (~158 source files) with transferable patterns from `forge-core`, `forge-topo`, and `forge-kernel`. Every item will be implemented. Breaking changes are permitted — this runtime has never shipped to production.

The spec is organized into **six execution phases** ordered by dependency: each phase's outputs become the inputs of the next. Within each phase, items can be done in any order.

> [!IMPORTANT]
> The kernel crate references in this document are intentional. When AI agents or engineers implement these changes, they should read the referenced kernel files to understand the source patterns before adapting them. The kernel is the working proof that these patterns compose well at scale.

---

## Phase A: Foundation Types

These changes have zero backward dependencies and affect the most downstream code. Doing them first minimizes merge conflict surface for everything that follows.

---

### A1 · Phantom-Tagged Identity Types

**What the kernel does**

forge-topo uses distinct handle types per entity kind (`BodyId`, `FaceId`, `EdgeId`, `VertexId`, `HalfEdgeId`, `LoopId`, `ShellId`, `LumpId`, `RegionId`) — see [handles.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-topo/src/handles.rs). Each is a newtype around a raw index, but they are compile-time distinct and cannot be confused. The substrate in forge-relational already has a `RecordId` trait in [record_arena.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/storage/substrate/record_arena.rs) that unifies `EntityId` and `RelationId` via:

```rust
pub(crate) trait RecordId: Copy + Ord + Hash + Debug + 'static {
    fn partition_id(&self) -> PartitionId;
    fn local_slot(&self) -> usize;
    fn generation(&self) -> u32;
}
```

This trait exists _because_ the four identity types in [identity/data/mod.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/identity/data/mod.rs) (`EntityId`, `RelationId`, `EntityStorageId`, `RelationStorageId`) are structurally identical — same fields, same derives, same constructors — but the compiler doesn't know that.

**What changes**

Replace the four duplicated structs with two phantom-tagged generics:

```rust
pub struct RecordId<K: RecordDomain> {
    pub partition_id: PartitionId,
    pub local_slot: LocalSlot,
    pub generation: Generation,
    _marker: PhantomData<K>,
}

pub struct StorageId<K: RecordDomain> {
    pub partition_id: PartitionId,
    pub local_slot: LocalSlot,
    _marker: PhantomData<K>,
}

pub enum EntityDomain {}
pub enum RelationDomain {}

pub type EntityId = RecordId<EntityDomain>;
pub type RelationId = RecordId<RelationDomain>;
pub type EntityStorageId = StorageId<EntityDomain>;
pub type RelationStorageId = StorageId<RelationDomain>;
```

The `RecordId` trait becomes unnecessary — generic code takes `RecordId<K>` directly. All `impl` blocks, `new()` constructors, and `storage_id()` methods consolidate to one place. Any future field addition (e.g., `shard_id`) is a single edit.

---

### A2 · Compiler-Enforced Slot Construction for RecordArena

**The problem**

[record_arena.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/storage/substrate/record_arena.rs) defines `RecordArena<K>` with **18 parallel `Vec`s**. Adding a new column requires edits in **7 locations across 3 files** — the struct definition, `with_capacity`, `reserve_additional`, `allocate_common` (two code paths: reclaim and fresh), `reset_reclaimed_slot` in both `EntityRecordKind` and `RelationRecordKind`, and `SlotView`. The compiler catches **none** of these if you forget one, because each field is independently typed.

The current `allocate_common` is 78 lines with two branches (reclaim path at lines 391-428, fresh path at lines 430-459) that must both stay in sync with each other and with all 18 fields. This is a silent-corruption bug surface.

> [!CAUTION]
> Do NOT introduce a `ColumnGroup` trait or wrap the vecs in intermediate structs. That adds abstraction over the SoA storage that obscures hot-path indexing (e.g., `arena.generations[slot]` becomes `arena.identity.generations[slot]`). The read path must stay transparent. This fix targets **construction and reset** only.

**What changes**

Introduce a `SlotInit<K>` struct that carries the caller-supplied data for a new slot. The compiler enforces completeness at every construction site:

```rust
/// Data required to initialize a new slot. The compiler errors at every
/// call site if a new field is added here but not supplied.
pub(crate) struct SlotInit<K: RecordKind> {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub payload: Option<RecordPayload>,
    pub version_id: VersionId,
    pub extra: K::Extra,
}
```

The arena gets two new methods that replace the monolithic `allocate_common`:

```rust
impl<K: RecordKind> RecordArena<K> {
    /// Allocate a new slot from init data. Handles both reclaim and fresh paths.
    /// Cold-path fields (pins, diagnostics, aspect versions) are always
    /// initialized to their default — callers never supply them.
    pub(crate) fn push_slot(&mut self, init: SlotInit<K>) -> (usize, u32, bool) {
        // ... reclaim or push to all 18 vecs, but driven by init struct
    }

    /// Reset a reclaimed slot to clean state. Driven by the same
    /// cold-path-fields-to-default discipline as push_slot.
    pub(crate) fn reset_slot(&mut self, slot: usize) {
        // ... clear all cold-path fields, push to free_list
    }
}
```

The 18 flat vecs stay exactly as they are — `SlotView` still indexes directly into `arena.generations[slot]`. What changes is that **construction and reset** go through a struct the compiler checks for completeness. The hot read path is completely untouched.

Adding a new hot-path column now means:

1. Add the field to `RecordArena` (struct)
2. Add the field to `SlotInit` — **the compiler immediately errors** at every call site that constructs a `SlotInit` without the new field
3. Handle it in `push_slot` and `reset_slot`
4. Add it to `SlotView`

That's 4 edit sites with **2 of them compiler-enforced**. Down from 7 edit sites with 0 compiler-enforced.

---

### A3 · Unified Error Hierarchy with Structured Context

**What the kernel does**

forge-core centralizes its error taxonomy in [errors/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/errors) with `KernelError` as the main error enum. Every subsystem's errors are variants, and they carry structured context. The `OperationResult<T>` envelope wraps errors alongside warnings, giving callers a single type to match.

**Current state**

forge-relational has errors scattered across 8+ modules with no common base: `TransactionCommitError`, `DurabilityError`, `BranchCreateError`, `SchemaRegistryError`, `PublicationError`, `PatchStreamReadError`, `CommitConflict`, `ReplayFailureClass`. Callers can't catch "any relational error" without matching six types. No common `From` chain means `?` doesn't propagate across subsystem boundaries.

**What changes**

Two layers. First, the wrapper enum for `?` propagation:

```rust
#[derive(Debug, thiserror::Error)]
pub enum RelationalError {
    #[error("transaction: {0}")]
    Transaction(#[from] TransactionCommitError),
    #[error("durability: {0}")]
    Durability(#[from] DurabilityError),
    #[error("history: {0}")]
    History(#[from] BranchCreateError),
    #[error("schema: {0}")]
    Schema(#[from] SchemaRegistryError),
    #[error("publication: {0}")]
    Publication(#[from] PublicationError),
    #[error("replay: {0}")]
    Replay(#[from] ReplayFailureClass),
}
```

Second — and this is where the real win lives — each subsystem error carries structured context, not just a message string:

```rust
/// Structured context attached to every subsystem error.
/// Machine-readable fields for diagnostics, not just human-readable strings.
pub struct ErrorContext {
    /// Which subsystem boundary produced this error.
    pub subsystem: RelationalSubsystem,
    /// Which runtime operation was in progress.
    pub operation: ErrorOperation,
    /// Affected record(s), if any.
    pub affected_records: Vec<RecordRef>,
    /// Affected version/transaction, if any.
    pub version_context: Option<VersionId>,
    /// Machine-actionable suggested fix, if known.
    pub suggested_fix: Option<SuggestedFix>,
}
```

Authority-path errors (commit conflicts, invariant violations, cascade failures) must carry `ErrorContext`. This makes errors machine-queryable for AI agents and debuggable for humans without parsing message strings.

Subsystem errors remain available for precise matching. The unified wrapper gives `?` propagation. The structured context gives actionable diagnostics.

---

## Phase B: Internal Cleanup

These changes improve local DX within individual modules. They don't require the runtime decomposition and are simpler to execute as standalone PRs.

---

### B1 · MutationWorkspace Combinator Audit

**Current state**

[MutationWorkspace](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/authority/mutation/types.rs) exposes access through closure-based split-borrow combinators:

```rust
fn with_draft_and_symbols<R>(&mut self, f: impl FnOnce(&mut RelationalDraft, &mut StringInterner) -> R) -> R;
fn with_draft_and_schema<R>(&mut self, f: impl FnOnce(&mut RelationalDraft, &RelationalSchemaRegistry) -> R) -> R;
fn with_draft_symbols_and_schema<R>(...) -> R;
```

**Design decision: preserve closure-based split-borrow**

The closure-based pattern is the correct Rust idiom for this problem. It enforces that callers can only hold split borrows for a scoped lifetime, which prevents the exact class of "accidentally hold `&mut draft` across a function that also needs `&mut symbols`" bugs that raw getter APIs would reintroduce. Replacing closures with exposed raw accessors would be a regression in borrow safety — the workspace is a coherent authority boundary, and the closures express that.

> [!WARNING]
> Do NOT replace the closure-based split-borrow pattern with raw getter methods. The closures enforce scoped borrow lifetimes that raw getters cannot. This was explicitly reviewed and confirmed as the right approach for Rust's ownership model.

**What changes**

Audit and prune the combinator surface. The N-choose-K explosion is the real problem — not the closure pattern itself. Remove combinator methods whose borrow combinations are unused or redundant. The target state is the **minimal set of combinators** that the mutation handlers actually need, not a combinator for every possible permutation.

If the audit reveals that most handlers need the same 3-way split, consolidate to a single `with_mutation_context` combinator rather than maintaining separate 2-way and 3-way variants:

```rust
impl MutationWorkspace<'_> {
    /// The primary mutation context: draft + symbols + schema.
    /// Most intent handlers need all three.
    pub(crate) fn with_context<R>(
        &mut self,
        f: impl FnOnce(&mut RelationalDraft, &mut StringInterner, &RelationalSchemaRegistry) -> R,
    ) -> R {
        f(self.draft, self.symbols, self.schema)
    }

    /// Read-only accessors remain direct (no closure needed for shared refs).
    pub(crate) fn config(&self) -> &MutationConfig { &self.config }
    pub(crate) fn version_id(&self) -> VersionId { self.version_id }
}
```

---

### B2 · RelationalDraft Delegation Cleanup

**Current state**

[RelationalDraft](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/storage/overlay/overlay.rs) wraps `WorkingState` and re-implements 10 methods that delegate 1:1 to `self.working`. It also implements `PartitionAccess` by delegating entirely to `self.working`.

**What changes**

Merge `touched_partitions` into `WorkingState` itself — `WorkingState` already tracks `mutation_journal: BTreeMap<PartitionId, PartitionMutationJournal>` which is effectively the same information. Then `RelationalDraft` either becomes a type alias or reduces to a thin wrapper with only the 1-2 methods that add behavior (like `commit()`), using `Deref` or the `delegate` crate for the rest.

---

### B3 · Nested Config Sections

**What the kernel does**

forge-kernel's [configuration module](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-kernel/src/configuration) separates config into nested domains. The kernel has `ModelingContext` with `ToleranceConfig`, `ValidationConfig`, and `FeatureConfig` as nested sections — not a flat struct with 25+ fields.

**Current state**

[RelationalRuntimeConfig](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/config/data/mod.rs) has **27 fields** at one level. `RelationalConfigOverride` has a parallel set of `Option<T>` fields.

**What changes**

Group into nested config sections that mirror the subsystem decomposition (Phase C):

```rust
pub struct RelationalRuntimeConfig {
    pub profile: RelationalRuntimeProfile,
    pub runtime_name: String,
    pub identity: IdentityConfig,         // initial capacities, partition setup
    pub concurrency: ConcurrencyConfig,   // mvcc, retention, visibility cache
    pub storage: StorageConfig,           // layout, adjacency, payload policy
    pub history: HistoryConfig,           // version graph, retention, branching
    pub publication: PublicationConfig,    // patch surface, coherent publication
    pub durability: DurabilityConfig,     // log, checkpoints, store layout
    pub schema: SchemaConfig,             // registry, invariants, symbols
    pub execution: ExecutionConfig,       // planning, commit authority, compiled lane
    pub diagnostics: DiagnosticsConfig,
    pub provenance: ConfigProvenance,
}
```

The override struct uses `Option<>` at the section level. This structure must align with the subsystem boundaries defined in Phase C.

> [!IMPORTANT]
> **Durability policy distinction.** The canonical durable artifact model (commit envelopes, patch records, checkpoint images) is fixed — it does not change per-profile. What becomes configurable is durability _policy_: flush frequency, checkpoint interval, retention depth, recovery strategy. Future workload presets (e.g., `AiWorkflow` vs `CertificationCore`) configure policy, not pluggable checkpoint semantics. `DurabilityConfig` must reflect this split: immutable schema + configurable policy.

---

### B4 · Unified Intent Hierarchy

**Current state**

[transactions/data/mod.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/transactions/data/mod.rs) maintains **two parallel enum hierarchies** representing the exact same mutation semantics:

- `TransactionIntent` — 8 flat variants (lines 147-177)
- `MutationIntent` → `CreateIntent` / `EntityMutationIntent` / `RelationMutationIntent` — 3 sub-enums with the same 8 leaf types (lines 218-243)
- Plus 7 separate intent structs (`BulkEntityCreateIntent`, `UpdateEntityIntent`, etc.) that mirror fields already in `TransactionIntent`

`to_mutation_intent()` is a **55-line match** mechanically converting one to the other. `From<MutationIntent> for TransactionIntent` is a **38-line match** doing the reverse. That's ~100 lines of pure translation code. On top of that, every method on `MutationIntent` (`seed_touched_partitions`, `bulk_entity_reservation`, `rollback_effect`, `existing_record_target`, `collect_relation_identities`, `collect_planned_entity_field_values`) repeats the N-variant match-and-extract pattern, totaling ~170 lines of field extraction.

**What changes**

Collapse to a single `MutationIntent` enum. `TransactionIntent` becomes a type alias. The ~100 lines of translation and the redundant intent structs vanish.

Once the hierarchy is unified, each intent variant can directly declare its invariant contract (from D4):

```rust
impl MutationIntent {
    pub const fn invariant_contract(&self) -> u32 {
        match self {
            Self::Create(CreateIntent::Entity(_)) =>
                RelationalInvariantGroup::StorageCoherence.mask()
                | RelationalInvariantGroup::IdentityCoherence.mask()
                | RelationalInvariantGroup::SchemaCompliance.mask(),
            Self::Entity(EntityMutationIntent::Delete(_)) =>
                RelationalInvariantGroup::AdjacencyIntegrity.mask()
                | RelationalInvariantGroup::StorageCoherence.mask()
                | RelationalInvariantGroup::LineageIntegrity.mask(),
            // ...
        }
    }
}
```

The commit pipeline computes `union_mask = intents.fold(0u32, |acc, i| acc | i.invariant_contract())` and intersects with `run_at` to skip untouched invariant groups. The intent IS the mode — it carries its own semantics; the pipeline is generic over it.

---

### B5 · Declarative Effect Assembly

**Current state**

Every intent handler in [authority/mutation/intents/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/authority/mutation/intents) follows the same boilerplate pattern:

```rust
fn apply(spec, workspace) -> Result<MutationEffect, CommitConflict> {
    let mut effect = MutationEffect::default();
    // 1. Do the actual domain mutation
    workspace.with_draft_and_symbols(|draft, symbols| { ... });
    // 2. Manually record the change (always same shape)
    effect.record_change(RecordRef::Entity(entity_id));
    // 3. Manually build the diagnostic (always same structure, different code/message)
    effect.record_diagnostic(RelationalDiagnosticsEntry { code: ..., message: ..., fields: ... });
    // 4. Manually build the patch record (kind maps 1:1 to intent kind)
    effect.record_patch(PatchRecord { kind: ..., target: ..., aspects: ..., detail: ... });
    Ok(effect)
}
```

Steps 2-4 are mechanically determined by the intent type + which records were touched. Every new intent handler must copy this ceremony. The diagnostic code, message, and patch kind are all derivable from the intent variant.

**What changes**

Split the intent handler return into domain effect (what actually changed) and framework-assembled observability (diagnostics, patches, change records):

```rust
/// What the intent handler returns — only the domain-specific mutation outcome.
pub(crate) enum DomainEffect {
    CreatedEntity { entity_id: EntityId, aspects: Vec<Symbol> },
    DeletedEntity { entity_id: EntityId, cascade: Vec<RelationId> },
    UpdatedEntity { entity_id: EntityId, aspects: Vec<Symbol> },
    CreatedRelation { relation_id: RelationId, source: EntityId, target: EntityId },
    // ...
}

impl DomainEffect {
    /// Framework derives the patch records from the domain effect.
    fn to_patches(&self, policy: PatchSurfacePolicy) -> Vec<PatchRecord> { ... }
    /// Framework derives the diagnostic entries from the domain effect.
    fn to_diagnostics(&self) -> Vec<RelationalDiagnosticsEntry> { ... }
    /// Framework derives the change records from the domain effect.
    fn changed_records(&self) -> Vec<RecordRef> { ... }
}
```

The dispatch layer assembles the full `MutationEffect` from the `DomainEffect`:

```rust
fn dispatch_and_assemble(
    intent: &MutationIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let domain = dispatch_intent(intent, workspace)?;
    Ok(MutationEffect::from_domain(domain, workspace.patch_surface_policy()))
}
```

Adding a new intent type means writing only the domain mutation logic — the framework handles observability. This pattern is inspired by frontend optimistic-mutation factories where `mode` drives generic cache update / rollback / invalidation behavior, and the handler only carries the API call. (The frontend code is in a separate workspace — ask the user for examples if needed.)

---

## Phase C: Runtime Decomposition

This is the keystone phase. Every subsequent phase (D, E, F) depends on the subsystem split being complete. Phases A and B should be done first to minimize conflicts.

---

### C1 · God Struct → Subsystem Split

**What the kernel does**

forge-kernel organizes into autonomous subsystem directories: [configuration/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-kernel/src/configuration), [context/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-kernel/src/context), [engine/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-kernel/src/engine), [proof/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-kernel/src/proof), [registry/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-kernel/src/registry). Each subsystem owns its state and exposes a focused API. The kernel's top-level struct doesn't contain 12 unrelated state fields.

forge-core follows the same pattern — [envelope/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/envelope), [policy/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/policy), [tracing/](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/tracing) are each self-contained with `data/`, `logic/`, `facade.rs` internals.

**Current state**

[RelationalRuntime](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/logic/runtime/state.rs) has **12 aggregate fields** and **903 lines** of methods touching unrelated concerns. Every `&mut self` borrows the entire runtime.

**What changes**

Extract each aggregate field into an autonomous subsystem wrapper:

```rust
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) storage: StorageSubsystem,
    pub(crate) visibility: VisibilitySubsystem,
    pub(crate) publication: PublicationSubsystem,
    pub(crate) history: HistorySubsystem,
    pub(crate) indexes: IndexSubsystem,
    pub(crate) lineage: LineageSubsystem,
    pub(crate) durability: DurabilitySubsystem,
    pub(crate) sequence: SequenceSubsystem,
    pub(crate) symbols: StringInterner,
    pub(crate) instrumentation: InstrumentationSubsystem,
    pub(crate) simulation: SimulationSubsystem,
}
```

Each subsystem owns its state, exposes its own API, and can be borrowed independently. The commit pipeline then borrows only the subsystems it needs:

```rust
fn commit(mut self) -> Result<CommitOutcome, TransactionCommitError> {
    let mutation = run_authoritative_mutation(&mut self.draft, &mut self.runtime.storage)?;
    let history = resolve_commit_history(&mut self.runtime.history, version_id)?;
    append_durable_commit(&mut self.runtime.durability, &envelope)?;
    finalize_publish(&mut self.runtime.visibility, &mut self.runtime.history, draft)?;
}
```

This makes the commit pipeline phases (already well-designed in [pipeline.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/authority/commit/pipeline.rs)) self-documenting — each phase's signature declares exactly which subsystems it touches.

---

### C2 · Visibility Cache Encapsulation

**What the kernel does**

forge-core contains cache management patterns where invalidation logic is contained within the cache domain — callers don't manually juggle locks.

**Current state**

The visibility cache in [state.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/logic/runtime/state.rs) lines 522–815 has ~300 lines of manual `RwLock`/`Mutex` lock-acquire-drop-reacquire ceremony. The `evict_visibility_cache_if_needed` method acquires and drops the same `Mutex` **4 separate times** in a single loop iteration. Lock ordering is implicit and undocumented.

**What changes**

As part of the subsystem split from C1, consolidate `SnapshotRegistry`'s internal locks into a single `VisibilityCache` struct:

```rust
pub(crate) struct VisibilitySubsystem {
    cache: VisibilityCache,
    active: BTreeMap<SnapshotId, SnapshotHandleBinding>,
    published_handles: BTreeMap<SnapshotId, VersionId>,
    replay_retained: BTreeMap<VersionId, ReplayRetentionState>,
    next_snapshot_id: u64,
}

struct VisibilityCache {
    inner: Mutex<VisibilityCacheInner>,
}

struct VisibilityCacheInner {
    states: BTreeMap<VersionId, SnapshotState>,
    residency: BTreeMap<VersionId, VisibilityResidency>,
    recent_window: DeterministicVersionWindowPolicy,
}

impl VisibilityCache {
    fn lookup_or_reconstruct(&self, version_id: VersionId, ...) -> Option<SnapshotState>;
    fn pin(&self, version_id: VersionId, reason: PinReason);
    fn unpin(&self, version_id: VersionId, reason: PinReason);
    fn evict_excess(&self, counters: &InstrumentationSubsystem);
}
```

Single lock scope, semantic methods, no manual lock juggling.

---

### C3 · SnapshotGuard Scope Narrowing

**Current state**

[SnapshotGuard](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/logic/runtime/mod.rs) holds `&'runtime mut RelationalRuntime`. While a guard exists, nothing else can use the runtime. The vision doc says "snapshot reads during active mutation" is first-class — this directly contradicts that.

**What changes**

After the subsystem split, `SnapshotGuard` only needs `&'runtime VisibilitySubsystem`:

```rust
pub struct SnapshotGuard<'runtime> {
    visibility: &'runtime VisibilitySubsystem,
    handle: SnapshotHandle,
}
```

The rest of the runtime remains accessible. This matches PostgreSQL's model where snapshot management is independent of query execution.

---

### C4 · Fork-Safe Runtime Construction

**Current state**

[session.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/logic/runtime/session.rs) `new()` manually initializes 11 fields and `fork()` manually clones them — some with `.clone()`, some with `.fork()` (snapshots, instrumentation). Adding a subsystem field requires editing both methods. The compiler catches neither omission. After the C1 subsystem decomposition, this gets worse as subsystem count grows.

This is the same class of bug as the `RecordArena` column problem (A2), but at the runtime level.

**What changes**

Each subsystem implements a `Subsystem` trait that declares its construction and forking behavior:

```rust
pub(crate) trait Subsystem: Sized {
    type Config;
    fn new(config: &Self::Config) -> Self;
    fn fork(&self) -> Self;
}
```

The `RelationalRuntime` struct uses these trait impls directly, and — critically — both `Runtime::new()` and `Runtime::fork()` are exhaustive struct expressions that the compiler checks for completeness:

```rust
impl RelationalRuntime {
    pub fn new(config: RelationalRuntimeConfig) -> Self {
        Self {
            storage: StorageSubsystem::new(&config),
            visibility: VisibilitySubsystem::new(&config),
            history: HistorySubsystem::new(&config),
            // ... compiler errors if a new subsystem field is added and not initialized
            config,
        }
    }

    pub fn fork(&self) -> Self {
        Self {
            storage: self.storage.fork(),
            visibility: self.visibility.fork(),
            history: self.history.fork(),
            // ... compiler errors if a new subsystem field is missing
            config: self.config.clone(),
        }
    }
}
```

The `Subsystem` trait also gives each subsystem a uniform lifecycle: the runtime can iterate subsystem diagnostics, reset subsystem state for testing, or snapshot subsystem metrics through a common protocol.

---

## Phase D: Invariant Engine

These changes create a first-class invariant scheduling system modeled directly on forge-topo's `GroupPolicyRuntime`. Every item in this phase should reference the kernel code.

> [!IMPORTANT]
> Phase D does **not** imply exposing `RelationalInvariantRuntime` or
> `InvariantEngine` directly everywhere. The correct target is a thin policy
> boundary above the engine. The engine owns invariant execution. The boundary
> owns phase/profile/domain request selection. This is necessary for future
> workloads like geometry kernels, chip simulators, and game engines, where the
> same engine must run under different invariant pressure and audit policies
> without leaking request assembly into every caller.

---

### D1 · Bitmask Invariant Groups

**Kernel reference**

Read [invariant_group.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/policy/data/invariant_group.rs) in forge-core: `InvariantGroup` is a `#[repr(u8)]` enum where each variant gets a stable bit position. Groups are scheduled via O(1) bitmask operations (`mask()` returns `1u32 << (*self as u8)`). The `APPLICABLE_BY_KIND` const array is a bitmask lookup table indexed by topology kind.

Also read [validation_checkpoint.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/policy/data/validation_checkpoint.rs): `ValidationCheckpoint` uses `#[repr(u8)]` with a `const COUNT` for fixed-size arrays.

**What changes**

Define relational equivalents:

```rust
#[repr(u8)]
pub enum RelationalInvariantGroup {
    StorageCoherence = 0,        // slot validity, lifecycle consistency
    VersionVisibility = 1,       // version bounds, snapshot correctness
    AdjacencyIntegrity = 2,      // source/target alive, bidirectional links
    IdentityCoherence = 3,       // generational freshness, no stale refs
    SchemaCompliance = 4,        // kind registration, payload validation
    LineageIntegrity = 5,        // no orphan lineage, no broken chains
    PublicationCoherence = 6,    // patch stream ordering, bundle completeness
    DurabilityConsistency = 7,   // log vs checkpoint agreement
}

impl RelationalInvariantGroup {
    pub const COUNT: usize = 8;
    pub const fn mask(&self) -> u32 { 1u32 << (*self as u8) }
}

#[repr(u8)]
pub enum RelationalCheckpoint {
    PreCommit = 0,
    PostMutation = 1,
    PrePublication = 2,
    PostCommit = 3,
    OnDemand = 4,
}

impl RelationalCheckpoint {
    pub const COUNT: usize = 5;
}
```

---

### D2 · Invariant Cost Classification

**Kernel reference**

Read the `ValidatorCost` enum in [invariant_group.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/policy/data/invariant_group.rs): `Cheap` (O(n) single pass), `Medium` (O(n log n)), `Expensive` (O(n²)). The `GroupPolicyRuntime` uses per-checkpoint cost ceilings.

**What changes**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InvariantCost {
    Touched,     // O(touched) — proportional to mutation size
    Partition,   // O(partition) — per-partition scan
    Global,      // O(global) — full storage scan
}
```

Each `RelationalInvariantGroup` variant declares its cost tier.

---

### D3 · Invariant Policy Runtime (O(1) Dispatch)

**Kernel reference**

Read [group_policy_runtime.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-topo/src/validators/group_policy_runtime.rs) in forge-topo. `GroupPolicyRuntime` is built once at construction via `resolve()`, then `should_run()` is a single `#[inline]` array-index + bitwise AND:

```rust
#[inline]
pub fn should_run(&self, group: InvariantGroup, checkpoint: ValidationCheckpoint) -> bool {
    self.run_at[checkpoint as usize] & group.mask() != 0
}
```

The `resolve()` method computes skip masks, deferred masks, and per-checkpoint run masks from applicability tables, user overrides, and topology context — all via bitwise operations.

**What changes**

Create `RelationalInvariantRuntime` with the identical pattern:

```rust
pub(crate) struct RelationalInvariantRuntime {
    skip_mask: u32,
    deferred_mask: u32,
    run_at: [u32; RelationalCheckpoint::COUNT],
    max_cost: [InvariantCost; RelationalCheckpoint::COUNT],
}

impl RelationalInvariantRuntime {
    pub fn resolve(profile: RelationalRuntimeProfile, ...) -> Self;

    #[inline]
    pub fn should_run(&self, group: RelationalInvariantGroup, cp: RelationalCheckpoint) -> bool {
        self.run_at[cp as usize] & group.mask() != 0
    }

    #[inline]
    pub fn max_cost_at(&self, cp: RelationalCheckpoint) -> InvariantCost {
        self.max_cost[cp as usize]
    }
}
```

This replaces the current `InvariantCatalog` + `Vec<InvariantRule>` approach.

The runtime should be consumed through a narrow policy boundary rather than
through raw engine calls at every call site. Commit, publication, harness, and
future domain runtimes should select from named policy entrypoints such as
"commit-boundary", "mutation-sensitive", or "audit", and those entrypoints
construct the exact engine request.

---

### D4 · Intent Contracts

**Kernel reference**

Read [contract_registry.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-topo/src/validators/contract_registry.rs) in forge-topo. Every topology operator declares which invariant groups it `MayBreak` and which are `Unrelated`:

```rust
pub const FULL_TOPO_WIRING: InvariantContract = InvariantContract {
    relation: |id| match id.group() {
        InvariantGroup::PointerCoherence => InvariantRelation::MayBreak,
        InvariantGroup::ShellClosure => InvariantRelation::Unrelated,
        // ...
    },
};
```

The checkpoint validator ORs together the `may_break` masks of all operators in the transaction and only runs those groups. This makes invariant execution proportional to _what actually changed_.

**What changes**

Define contracts for each mutation intent type:

```rust
pub const CREATE_ENTITY_CONTRACT: RelationalInvariantContract = RelationalInvariantContract {
    may_break: RelationalInvariantGroup::StorageCoherence.mask()
             | RelationalInvariantGroup::IdentityCoherence.mask()
             | RelationalInvariantGroup::SchemaCompliance.mask(),
};

pub const DELETE_ENTITY_CONTRACT: RelationalInvariantContract = RelationalInvariantContract {
    may_break: RelationalInvariantGroup::AdjacencyIntegrity.mask()
             | RelationalInvariantGroup::StorageCoherence.mask()
             | RelationalInvariantGroup::LineageIntegrity.mask(),
};
```

The commit pipeline computes `union_mask = intents.fold(0u32, |acc, i| acc | i.contract().may_break)` then intersects with `run_at` to skip untouched groups entirely.

**Commit topology inference.** The union mask also drives **pipeline phase selection**, not just invariant group selection. Different mutation topologies need different pipeline phases:

| Topology | Detected when union mask includes | Pipeline phases enabled |
|---|---|---|
| **Flat entity batch** | `StorageCoherence` only | Mutation, history, publication — skip cascade/adjacency |
| **Graph mutation** | `AdjacencyIntegrity` | Mutation, cascade checking, adjacency rebuild, history, publication |
| **Branch merge** | `MergeCoherence` | Three-way diff, conflict resolution, all standard phases |

This is inspired by a frontend pattern where flat-list CRUD managers and tree-structured CRUD managers use topology-specific optimistic update strategies — the data shape determines the cache manipulation approach. (The frontend code is in a separate workspace — ask the user for examples if needed.) Instead of the caller declaring the topology explicitly, the pipeline **infers** it from the combined intent contracts. A flat entity batch that only touches `StorageCoherence` never pays for cascade checking or adjacency rebuild.

---

### D5 · Three-State Invariant Verdicts

**Kernel reference**

Read [policy_result.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/policy/data/policy_result.rs) in forge-core. `PolicyResult<T>` distinguishes `Success(T)`, `Ambiguous { query, potential_value }`, and `HardError(E)`. The middle state carries structured context that the kernel can inspect and override.

**What changes**

Invariant checks return `InvariantVerdict` instead of `Result<(), InvariantViolation>`:

```rust
pub enum InvariantVerdict<T = ()> {
    Pass(T),
    Advisory {
        violation: InvariantViolation,
        proceed_value: T,
        advisory: InvariantAdvisory,
    },
    Violation(InvariantViolation),
}
```

This bridges the gap between the vision doc's invariant categories (`AlwaysOnStructural` → always `Violation`, `SnapshotAudit` → can be `Advisory`) and the runtime profile (`CertificationCore` blocks on advisories, `AiWorkflow` proceeds).

---

### D6 · State-Derived Invariant Context

**Kernel reference**

Read [topology_context_from_shell_metadata](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-topo/src/validators/group_policy_runtime.rs#L141-195) in forge-topo. Before resolving the validation policy, forge-topo derives a `TopologyContext` from the actual model state by reading `ShellKind` metadata (O(shells), ~1-4). The policy adapts to the actual state of the model, not just static configuration.

**What changes**

```rust
pub(crate) fn derive_invariant_context(runtime: &RelationalRuntime) -> InvariantContext {
    let entity_count = runtime.storage.entity_slot_count();
    let relation_count = runtime.storage.relation_slot_count();
    let version_depth = runtime.history.commit_count();
    let active_snapshots = runtime.visibility.active_snapshot_count();

    InvariantContext {
        scale: match entity_count {
            0..=1_000 => Scale::Small,
            1_001..=100_000 => Scale::Medium,
            _ => Scale::Large,
        },
        version_depth,
        snapshot_pressure: active_snapshots > 10,
    }
}
```

The invariant runtime adjusts cost ceilings based on this context — at large scale, pre-commit checks downgrade from `Global` to `Partition` automatically.

The policy boundary above the engine is where this derived context becomes
phase-aware configuration. The engine consumes an execution request. The
boundary decides which request profile is appropriate for a geometry-kernel
operation, chip-simulation step, game-engine frame audit, certification run, or
authoritative commit.

---

## Phase E: Commit Architecture

These changes upgrade the commit pipeline's diagnostic and return-value story. They depend on the subsystem split (C) and invariant engine (D).

---

### E1 · Commit Decision Log

**Kernel reference**

Read [decision_log.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/tracing/decision_log/decision_log.rs) in forge-core (685 lines). `DecisionLog` is a span-aware, queryable trace:

- **Flat `Vec<TraceEvent>` storage** with tree reconstruction via StartSpan/EndSpan markers
- **O(1) decision lookup** via `decision_index: HashMap<DecisionId, usize>`
- **Running summary** updated incrementally on `record()` — `summary()` is O(1)
- **Merge with ID rebasing** — two logs combine without collisions
- **Diffable `TraceSummary`** via `TraceSummary::diff()` for regression detection

Also read [traced_decision.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/tracing/decision/traced_decision.rs) for the per-decision schema (kind, tier, margin, entity scope, span assignment, topology delta) and [span.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/tracing/decision/span.rs) for the `TraceEvent` protocol.

**What changes**

Create a `CommitLog` that wraps each commit pipeline phase in a span:

```rust
pub(crate) struct CommitLog {
    events: Vec<CommitTraceEvent>,
    span_stack: Vec<CommitSpanId>,
    running_summary: CommitSummary,
}

pub(crate) enum CommitPhase {
    DraftPreparation,
    PlanMerge,
    InvariantPreCheck,
    AuthoritativeMutation,
    HistoryResolution,
    InvariantPostCheck,
    ArtifactAssembly,
    Publication,
}
```

The 7-phase pipeline in [pipeline.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/authority/commit/pipeline.rs) already has the right structure — each phase call opens a span, records decisions (conflict resolution, cascade triggers, schema validation), and closes the span. Debugging becomes `commit_log.phase_decisions(CommitPhase::AuthoritativeMutation)`.

---

### E2 · Commit Result Envelope

**Kernel reference**

Read [operation_result.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/envelope/data/operation_result.rs) in forge-core. Every kernel operation returns `OperationResult<T>` wrapping value + warnings + decision log + metrics + lineage delta + state hashes + validation results. An AI agent can reconstruct the full state transition from this envelope alone.

**What changes**

Wrap `CommitOutcome` in a `CommitResult` envelope:

```rust
pub struct CommitResult {
    pub outcome: CommitOutcome,
    pub diagnostics: Vec<RelationalDiagnosticsEntry>,
    pub patch: Vec<PatchRecord>,
    pub envelope: CanonicalCommitEnvelope,
    pub commit_log: CommitLog,
    pub phase_timing: CommitPhaseTiming,
    pub invariant_results: Vec<InvariantCheckResult>,
    pub complexity_delta: RuntimeComplexityCounters,
}
```

`txn.commit()` returns `Result<CommitResult, RelationalError>`. Everything a caller needs — for debugging, replay, CDC, or auditing — comes back in one return value.

---

## Phase F: API Surface

This is the final phase. It reflects the internal structure established by all previous phases into the public facade.

---

### F1 · Facade Namespace Organization

**What the kernel does**

forge-core exposes each domain through its own `facade.rs` — [policy/facade.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/policy/facade.rs), [envelope/facade.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-core/src/envelope/facade.rs) — with the crate root re-exporting from each. forge-topo's [validators/facade.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-topo/src/validators/facade.rs) does the same: focused re-exports grouped by domain.

**Current state**

[facade.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/facade.rs) is 106 lines re-exporting ~150 types in a completely flat namespace. Autocomplete is useless when 150 items compete.

**What changes**

Organize exports into sub-namespaces matching the subsystem decomposition:

```rust
pub mod facade {
    pub mod identity { /* EntityId, RelationId, PartitionId, Generation, ... */ }
    pub mod transactions { /* TransactionId, CommitOutcome, CommitResult, SavepointId, ... */ }
    pub mod history { /* BranchId, CommitId, VersionNode, ... */ }
    pub mod snapshots { /* SnapshotHandle, SnapshotId, SnapshotGuard, ... */ }
    pub mod publication { /* PatchRecord, PatchStreamBatch, ... */ }
    pub mod lineage { /* LineageEventRecord, CorrespondenceCandidate, ... */ }
    pub mod config { /* RelationalRuntimeConfig, profiles, policies */ }
    pub mod durability { /* DurableCheckpoint, RecoveryPlan, ... */ }
    pub mod schema { /* SchemaId, EntityKindRegistration, ... */ }
    pub mod diagnostics { /* DiagnosticCode, RelationalDiagnosticsEntry, ... */ }
    pub mod invariants { /* RelationalInvariantGroup, InvariantVerdict, ... */ }
    pub mod errors { /* RelationalError, ... */ }

    // Top-level convenience re-exports for the most common entry points
    pub use self::identity::{EntityId, RelationId};
    pub use self::transactions::{CommitOutcome, CommitResult};
    pub use self::history::BranchId;
    pub use self::errors::RelationalError;
}
```

---

### F2 · Type-Driven Read Surface (RecordProjection)

**Current state**

[read_records/mod.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/visibility/materialization/read_records/mod.rs) contains **266 lines of methods** on `impl RelationalRuntime` that all follow the same pattern: take `&impl PartitionAccess` + `partition_id` + `kind_id` + `version_id`, iterate slots, check kind/visibility, materialize records. Every consumer must thread all four parameters and handle the iteration manually.

`PartitionAccess` is the right low-level trait (like `HttpClient` is the right low-level primitive). But there's no composable surface on top of it — and more importantly, there's no way for a domain type to **declare up front** what it needs from the runtime.

**Design principle: read/write contract duality**

On the write path (B4/D4), each `MutationIntent` declares its invariant contract — "what I may break":

```rust
impl MutationIntent {
    pub const fn invariant_contract(&self) -> u32 { /* aspect mask */ }
}
```

The read path needs the exact **dual**: each domain type declares "what I depend on." These two declarations are opposite faces of the same coin — and the bridge layer computes their intersection for aspect-aware invalidation:

```
WRITE: UpdateEntity { touched_aspects: [Geometry] }
                          ↓
BRIDGE: "Which projections does this invalidate?"
         intersection(write.touched, read.required)
                          ↓
READ:  Body::required_aspects() → [Geometry, Metadata]   → INVALIDATE (Geometry ∩)
       DisplayName::required_aspects() → [Metadata]       → SKIP       (no intersection)
```

**What changes**

The domain type itself declares its read contract via a `RecordProjection` trait:

```rust
/// The type IS the read contract.
/// Dual of MutationIntent::invariant_contract() on the write path.
pub trait RecordProjection: Sized {
    /// Which kind this projection reads from.
    const KIND: KindId;

    /// Which payload aspects this type depends on.
    /// Used by SnapshotView for materialization,
    /// and by the bridge layer for aspect-aware invalidation.
    fn required_aspects() -> &'static [AspectKey];

    /// Construct from a read record. Returns None if record doesn't match.
    fn from_record(record: &EntityReadRecord) -> Option<Self>;
}

// Example domain type — declares everything up front
impl RecordProjection for Body {
    const KIND: KindId = BODY_KIND;
    fn required_aspects() -> &'static [AspectKey] {
        &[AspectKey::GEOMETRY, AspectKey::METADATA]
    }
    fn from_record(record: &EntityReadRecord) -> Option<Self> {
        Some(Body {
            id: record.entity_id,
            geometry: record.payload.get("geometry")?,
            name: record.payload.get("name")?.as_str()?.to_string(),
        })
    }
}
```

`SnapshotView` uses this trait to collapse the entire read chain into a single generic call — kind, aspects, construction, and version resolution are all derived from the type:

```rust
pub struct SnapshotView<'runtime> {
    runtime: &'runtime RelationalRuntime,
    version_id: VersionId,
    state: BorrowedWorkingState<'runtime>,
}

impl<'runtime> SnapshotView<'runtime> {
    /// Type-driven projection — kind, aspects, and construction
    /// all derived from T's RecordProjection declaration.
    pub fn project<T: RecordProjection>(&self) -> Vec<T> {
        self.state.partition_ids().iter().flat_map(|pid| {
            self.runtime
                .visible_entities_of_kind_in_partition_from_state(
                    &self.state, *pid, T::KIND, self.version_id,
                )
                .into_iter()
                .filter_map(T::from_record)
        }).collect()
    }

    /// Projection with partition filter.
    pub fn project_in<T: RecordProjection>(&self, partition_id: PartitionId) -> Vec<T> {
        self.runtime
            .visible_entities_of_kind_in_partition_from_state(
                &self.state, partition_id, T::KIND, self.version_id,
            )
            .into_iter()
            .filter_map(T::from_record)
            .collect()
    }

    /// Single-entity lookup — also type-driven.
    pub fn get<T: RecordProjection>(&self, entity_id: EntityId) -> Option<T> {
        let record = self.runtime.entity_record_for_id_at_version(
            &self.state, entity_id, self.version_id,
        )?;
        T::from_record(&record)
    }

    /// Escape hatch for ad-hoc reads that don't have a projection type.
    pub fn entities_of_kind(&self, kind_id: KindId) -> EntityQuery<'_> {
        EntityQuery { view: self, kind_id, partition_filter: None }
    }
}
```

Consumer code collapses from:

```rust
// Before — consumer threads 4 params and iterates manually
let state = runtime.current_state();
let version_id = runtime.current_version_id();
for partition_id in state.partition_ids() {
    let records = runtime.visible_entities_of_kind_in_partition_from_state(
        &state, partition_id, body_kind_id, version_id,
    );
    for record in records { ... }
}
```

To:

```rust
// After — type declares its contract, framework handles the rest
let bodies: Vec<Body> = snapshot.view().project();
```

One line. Kind, aspects, construction, version, partition iteration — all derived from `Body`'s `RecordProjection` impl.

> [!IMPORTANT]
> **Contract duality with the bridge layer.** `RecordProjection::required_aspects()` is the read-side contract. `MutationIntent::invariant_contract()` (D4) is the write-side contract. They are duals — the bridge layer intersects them for aspect-aware invalidation. When a commit touches aspects `[Geometry]`, only projections whose `required_aspects()` include `Geometry` are invalidated. This means adding a new domain type automatically wires its invalidation behavior — no manual bridge configuration needed.

---

## What Must Be Preserved

These patterns are already strong. Every refactoring item above must preserve them:

| Pattern                            | Location                                                                                                                                              | Why it's good                                        |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| `RecordKind` trait system          | [record_arena.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/storage/substrate/record_arena.rs) | Right level of SoA generics                          |
| `PartitionAccess` trait            | [access.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/storage/overlay/access.rs)               | Stays as the low-level read primitive underneath `SnapshotView` |
| 7-phase commit pipeline            | [pipeline.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/authority/commit/pipeline.rs)          | Readable, auditable, correctly separated             |
| `MutationEffect::accumulate`       | [types.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/authority/mutation/types.rs)              | Good fold/reduce for composing intent results        |
| `SlotView<'a, K>`                  | [record_arena.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/storage/substrate/record_arena.rs) | Zero-cost borrow lens into the arena                 |
| `BTreeMap` in all observable paths | everywhere                                                                                                                                            | Deterministic iteration per vision doc               |
| `DenseSlotBitSet`                  | [bitsets.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/storage/partition/bitsets.rs)           | Proper bitset for lifecycle tracking                 |
| Per-slot versioned history         | [record_arena.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/storage/substrate/record_arena.rs) | Appropriate for MVCC model                           |
| Config provenance tracking         | [config/data/mod.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/config/data/mod.rs)             | Knowing _why_ each config value is set               |
