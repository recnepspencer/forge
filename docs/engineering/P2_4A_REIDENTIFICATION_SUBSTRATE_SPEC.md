# P2-4A Engineering Specification

# Persistent Re-identification Substrate

# Lineage Linkage + Delta / Audit Integration

**Status:** Contract-locked, pre-implementation
**Version:** 1.0
**Prereqs:** P2-1 (trace adjuncts), P2-2 (operation finalization), P2-4 (resolution result contract)
**Crates affected:** `forge-topo`, `forge-kernel`, `forge-core`, `forge-io`

---

## 1. Purpose and Scope

### 1.1 Why this spec exists

The `P2-4` resolution contract defines a `LineageReidentified` fallback route. Without
this substrate, that route is a label — not a real capability. Any system that
claims `ResolutionRoute::LineageReidentified` without this substrate is producing
fake lineage fallback claims. This spec closes that gap.

The goal is not a name registry. The goal is a **persisted, queryable, versioned
index** that can answer the question:

> Given a `PersistentName` (ancestry_hash + kind + ordinal) that resolves to
> zero entities in the current arena, enumerate its candidate descendant or
> ancestor entities from the recorded lineage linkage, with typed evidence and
> deterministic ordering, so a re-identification decision can be emitted and audited.

### 1.2 Current state — exact gap analysis

What exists:

| Component                                               | Location                                | Status                            |
| ------------------------------------------------------- | --------------------------------------- | --------------------------------- |
| `Lineage` (ancestry_hash, creation_op, origin_features) | `forge-topo::history::lineage`          | ✓ Implemented                     |
| `LineageEvent` (Created/Deleted/Modified per entity)    | `forge-topo::history::lineage`          | ✓ Implemented                     |
| `LineageStore` (draft-local entity → lineage map)       | `forge-topo::history::lineage_store`    | ✓ Implemented                     |
| `PersistentName` (ancestry_hash + kind + ordinal)       | `forge-topo::naming::schema`            | ✓ Implemented                     |
| `resolve_name` / `resolve_selector`                     | `forge-topo::naming::eval`              | ✓ Implemented, current-arena only |
| `TopologyState::lineage_events`                         | `forge-topo::state`                     | ✓ Persisted (accumulated Vec)     |
| `OpSignature` (name + invocation_id)                    | `forge-topo::history::lineage`          | ✓ Implemented                     |
| `ReplayLog` / `ReplayEntry` (pre/post hash + params)    | `forge-topo::history::replay`           | ✓ Implemented                     |
| `ResolutionTracePayload` + route/match enums            | `forge-core::tracing::resolution_trace` | ✓ Implemented                     |

What is **missing** — the exact gap:

1. **No persisted predecessor/successor linkage index.**
   `LineageEvent::EntityCreated { entity, lineage }` stores the entity's
   `Lineage` (which carries the parent's `ancestry_hash` encoded in the derived
   hash chain), but it does **not** store an explicit reference to the parent
   entity. Scanning `lineage_events` by ancestry hash prefix requires O(N) full
   scan and still cannot enumerate _which specific entity_ was the predecessor.

2. **`LineageStore` is draft-local and discarded on commit.**
   The live `BTreeMap<EntityRef, Lineage>` that maps entities to lineages exists
   only during a `MutableDraft`. After `commit()`, only the raw `LineageEvent`
   `Vec` survives in `TopologyState`. There is no committed-state index for
   lineage-backed candidate lookup.

3. **No forward linkage in `LineageEvent`.**
   `EntityCreated` records the child's lineage but not _which entity was its
   predecessor_. The `ancestry_hash` chain can verify lineage depth but cannot
   enumerate "who descended from entity X" without full-log scans.

4. **No schema versioning on lineage records.**
   The `LineageEvent` enum and `Lineage` struct carry no schema version. A
   compatibility gate for `LineageReidentified` requires versioned lookups.

5. **No typed compatibility outcome for re-identification.**
   The resolver today can only react to `Vec<EntityKey>` (zero = missing, many
   = split). It cannot distinguish:
   - linkage data unavailable (no P2-4A substrate)
   - linkage incompatible (schema mismatch)
   - linkage present but entity genuinely deleted (no candidates)

6. **`LineageDelta` is count-only.**
   The envelope `LineageDelta { created, deleted, modified }` is correct for
   accounting but has no re-identification payload. Detailed provenance must be
   in a separate typed channel (audit adjunct), not squeezed here.

7. **Lineage events are index-only today (`forge_core::EntityRef`).**
   `LineageStore` / `LineageEvent` currently preserve `EntityKind + index` but
   not generation. P2-4A V1 link records require generation-safe snapshot refs
   (`TopoSnapshotHandleRef`) to satisfy ABA safety and deterministic auditability.
   This is a prerequisite substrate upgrade; builders must not fabricate
   generations (e.g. `0`) or silently degrade to index-only records.

### 1.3 Non-goals

- Full persistent naming rollout for all kernel features
- Database-backed audit storage
- Curved geometry or NURBS lineage
- Forward-linkage for all entity kinds on day 1 (Face is the priority)

---

## 2. Architecture

### 2.1 Layering

```
forge-topo / forge-core:
  ┌──────────────────────────────────────────────────┐
  │  ReidentificationLinkRecord  (new, in forge-topo) │
  │  ReidentificationLinkStore   (new, in forge-topo) │
  │  ReidentificationLinkIndex   (new, in forge-topo) │
  │  Versioned schema (LinkSchemaVersion enum)        │
  └──────────────────────────────────────────────────┘
       ↑ generated from LineageStore at commit time
       ↑ persisted into TopologyState
       ↑ queried by resolver in forge-topo::naming

forge-kernel (P2-4 integration):
  ┌──────────────────────────────────────────────────┐
  │  ReidentificationQuery / Candidate / Evidence    │
  │  ReidentificationCompatibility                   │
  │  Resolver bridge: PersistentName → Candidates    │
  │  ReidentificationAuditPayload (adjunct family)   │
  └──────────────────────────────────────────────────┘
       ↑ feeds ResolutionResult<T> for P2-4 contract
       ↑ emitted via P2-2 finalization as trace adjunct
```

Dependency rule: the link store and index live in `forge-topo`. `forge-topo`
uses topo-local snapshot ref types in persisted linkage records and query/index
APIs. `forge-core` trace/audit payloads must not depend on `forge-topo` concrete
types; they receive kernel/finalization-layer summaries converted from topo
records. The resolver bridge lives in `forge-kernel` where `P2-4` integration
lands.

### 2.2 The three phases of re-identification

```
Phase A: BUILD (at commit time, inside MutableDraft::commit)
  LineageStore (draft-local) ──▶ ReidentificationLinkRecord set
  Records are inserted into the committed TopologyState's
  ReidentificationLinkIndex.

Phase B: QUERY (at resolution time, inside P2-4 LineageReidentified phase)
  PersistentName (ancestry_hash, kind, ordinal)
  ──▶ ReidentificationLinkIndex::find_candidates(query)
  ──▶ Vec<ReidentificationCandidate>
  ──▶ deterministic sort
  ──▶ ReidentificationEvidence
  ──▶ fed into ResolutionResult<T> as Resolved / Ambiguous / Missing

Phase C: EMIT (at P2-2 finalize time)
  ReidentificationEvidence
  ──▶ ReidentificationAuditPayload (trace adjunct, keyed by DecisionId)
  ──▶ attached to FinalizationSummary / audit record
```

### 2.3 P2-4A prerequisite: Generational lineage-event references (production gate)

Before implementing `build_link_records_from_store(...)` and commit-time index
construction, Forge must upgrade lineage event/reference storage to preserve
generational topology identity for created/modified/deleted entities.

This is a **required prerequisite**, not an optional enhancement. Without it,
P2-4A would either:
- fabricate generations (incorrect, unsafe),
- degrade to index-only linkage (violates spec and ABA guarantees), or
- produce mixed-quality records that break audit/replay trust.

#### 2.3.1 Contract goal

- `LineageStore` / `LineageEvent` preserve generation-safe entity identity when
  recording topology provenance events.
- P2-4A builders can construct `TopoSnapshotHandleRef` without consulting stale
  arena state or inventing generations.
- Legacy index-only lineage events remain deserializable and explicitly marked
  as structurally limited evidence.

#### 2.3.2 Data model upgrade (topo-local)

Add a topo-local generational lineage reference type (example naming):

```rust
// forge-topo::history::lineage
pub struct LineageEntityRef {
    pub kind: EntityKind,
    pub index: u32,
    pub generation: u32,
}
```

`LineageEvent` must use this topo-local generational ref type (or carry both a
legacy `EntityRef` and a generational ref during migration).

`forge_core::EntityRef` remains valid for crate-neutral tracing contexts where
generation is intentionally unavailable. It is **not sufficient** for P2-4A
linkage record construction.

#### 2.3.3 Compatibility and storage/replay impact

- Existing serialized `LineageEvent` payloads must remain readable.
- Legacy events without generation must deserialize into an explicit legacy form
  (or defaulted form) that downstream code treats as limited evidence.
- Replay/counterfactual tooling must distinguish:
  - `LegacyIndexOnlyLineageEvent` (usable for coarse provenance)
  - `GenerationalLineageEvent` (usable for P2-4A linkage)
- P2-4A builders must fail closed or emit typed limited-compatibility evidence
  when only legacy lineage events are available.

#### 2.3.4 Recording path migration (required order)

1. Introduce `LineageEntityRef` + conversion helpers from typed handles.
2. Update `LineageEvent` and `LineageStore` to record generational refs.
3. Migrate arena/draft lineage recording callsites so generation is captured at:
   - creation
   - deletion
   - mutation
4. Add backward-compatible serde handling for pre-upgrade lineage events.
5. Only then implement `build_link_records_from_store(...)`.

#### 2.3.5 Consumers impacted (must be audited)

- `forge-topo` replay log / replay helpers
- kernel causal chain / counterfactual analysis readers
- proof validation tools that inspect lineage events
- P2-4A link-record builder and resolver evidence generation
- audit/export paths that serialize lineage history

#### 2.3.6 Adversarial tests (prerequisite suite)

- same index, different generation produce distinct lineage events
- deletion + slot reuse does not alias lineage event identity
- legacy lineage event deserializes and is marked limited/legacy
- P2-4A builder rejects or marks legacy-only lineage history as incompatible
- mixed legacy+generational history produces deterministic, typed compatibility outcome

---

## 3. Data Model

### 3.1 `LinkSchemaVersion`

```rust
// forge-topo::history::lineage_link
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkSchemaVersion(pub u32);

impl LinkSchemaVersion {
    pub const V1: Self = Self(1);
}
```

**Contract:** Every `ReidentificationLinkRecord` and `ReidentificationLinkIndex`
must carry this version. Readers must check version before interpreting records.
An unknown version must produce `ReidentificationCompatibility::SchemaVersionMismatch`.

### 3.2 `ReidentificationLinkRecord`

This is the elemental unit of the substrate. It represents "entity B was created
from entity A by operation O."

```rust
// forge-topo::history::lineage_link
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReidentificationLinkRecord {
    /// Schema version this record was encoded with.
    pub schema_version: LinkSchemaVersion,

    /// The entity that was created (the child in the lineage graph).
    /// Uses a topo-local snapshot ref type — explicitly snapshot-scoped and
    /// debug/provenance-only when persisted across epochs.
    pub child_snapshot: TopoSnapshotHandleRef,

    /// Ancestry hash of the child entity.
    ///
    /// This is the `Lineage::ancestry_hash` of the child entity.
    /// It is what `PersistentName::ancestry_hash` captures at time-of-naming.
    pub child_ancestry_hash: u128,

    /// Ancestry hashes of the immediate parent entities, if known.
    ///
    /// V1 must support the *shape* of compound parentage even if some producers
    /// only populate a single parent. This prevents hard-coding a single-parent
    /// substrate that blocks future fillet/NURBS provenance.
    pub parent_ancestry_hashes: Vec<u128>,

    /// Linkage shape for the recorded parent set.
    pub parent_linkage_mode: ParentLinkageMode,

    /// Optional snapshot ref of the parent entity (when resolvable at creation time).
    ///
    /// NOT required for re-identification: re-identification works via ancestry_hash chains.
    /// Provided as an additional audit reference when available.
    pub parent_snapshot: Option<TopoSnapshotHandleRef>,

    /// Typed classification of how the child entity came to exist.
    ///
    /// This is the authoritative semantic origin class. Callers must not infer
    /// origin semantics from `creation_op_name` string prefixes.
    pub origin_kind: EntityOriginKind,

    /// Human/audit operation identity label that produced the child.
    ///
    /// **Naming convention for non-Euler origins (NURBS invariant guard):**
    /// - Euler operators: use the exact operator name, e.g. `"split_edge"`, `"join_faces"`.
    /// - Constraint-solver origins: use `"cst:<kind>:<feature_id>"`, e.g.
    ///   `"cst:g1_continuity:42"`. The feature_id component ensures invocations
    ///   are distinguishable across features while remaining deterministic.
    /// - Geometric intersection origins: use `"isect:<surface_a_hash>:<surface_b_hash>"`.
    ///   These entities should also emit `EntityOriginKind::GeometricIntersection`
    ///   in their compatibility gate rather than producing unreliable forward-link records.
    ///
    /// Implementations MUST NOT use free-form or empty strings for this field.
    /// This field is not a substitute for `origin_kind`.
    pub creation_op_name: String,

    /// Invocation ID of the operation (from OpSignature::invocation_id).
    ///
    /// Combined with creation_op_name, this uniquely identifies the
    /// specific operation invocation within a draft.
    pub creation_op_invocation: u64,

    /// The epoch of the TopologyState this record was committed in.
    pub epoch: u64,

    /// Feature IDs that contributed to this entity's lineage.
    ///
    /// Stored in sorted order for deterministic comparison.
    pub origin_features: Vec<u64>,
}
```

**Invariants:**

- `child_snapshot` must not be reused across generations without a new record.
- `child_ancestry_hash` must match the `Lineage::ancestry_hash` of the child entity.
- `parent_linkage_mode` and `parent_ancestry_hashes` must agree:
  - `None` => empty parent hash set
  - `Single` => exactly one parent hash
  - `Compound` => two or more parent hashes
- `parent_ancestry_hashes` must be deduplicated and sorted ascending for
  deterministic serialization and candidate ordering.
- `creation_op_name` and `creation_op_invocation` must not both be zero/empty
  (except for root entities where `parent_linkage_mode == None`).
- `origin_features` must be sorted ascending.

### 3.2A `TopoSnapshotHandleRef` and `ParentLinkageMode`

`forge-topo` persists topo-local snapshot refs in linkage records. These are
snapshot-scoped debug/provenance references and must never be treated as
authoritative cross-epoch identity.

```rust
// forge-topo::history::lineage_link
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopoSnapshotHandleRef {
    pub kind: EntityKind,
    pub index: u32,
    pub generation: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentLinkageMode {
    None,
    Single,
    Compound,
}
```

### 3.3 `ReidentificationLinkIndex`

The committed-state queryable index. This is what replaces the missing
committed `BTreeMap<EntityRef, Lineage>` capability.

```rust
// forge-topo::history::lineage_link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReidentificationLinkIndex {
    /// Schema version of all records in this index.
    pub schema_version: LinkSchemaVersion,

    /// Primary index: ancestry_hash → records where this is the CHILD hash.
    ///
    /// Used for direct re-identification: "who has ancestry_hash X right now?"
    /// BTreeMap ensures deterministic iteration order.
    by_child_hash: BTreeMap<u128, Vec<ReidentificationLinkRecord>>,

    /// Secondary index: parent_ancestry_hash → child records (one-hop children).
    ///
    /// Used for one-hop child queries: "who was born from entity with hash X?"
    by_parent_hash: BTreeMap<u128, Vec<ReidentificationLinkRecord>>,

    /// The topology epoch this index was built from.
    pub epoch: u64,
}
```

**Key methods (contracted API):**

```rust
impl ReidentificationLinkIndex {
    /// Build an index from the link records produced by a committed draft.
    pub fn build(records: Vec<ReidentificationLinkRecord>, epoch: u64) -> Self;

    /// Find candidates whose child_ancestry_hash matches the query hash.
    ///
    /// Returns records in deterministic order (sorted by child_snapshot
    /// index + generation). Never returns in map-iteration order.
    pub fn find_by_child_hash(
        &self,
        hash: u128,
        kind: EntityKind,
    ) -> Vec<&ReidentificationLinkRecord>;

    /// Find children of a given parent hash (one-hop).
    ///
    /// Returns records where `parent_hash` appears in the record's
    /// `parent_ancestry_hashes` set (one-hop child relation).
    /// Sorted deterministically.
    pub fn find_children_of(
        &self,
        parent_hash: u128,
        kind: EntityKind,
    ) -> Vec<&ReidentificationLinkRecord>;

    /// Schema version check for compatibility gate.
    pub fn is_compatible(&self, required: LinkSchemaVersion) -> ReidentificationCompatibility;

    /// Number of records in the index.
    pub fn record_count(&self) -> usize;
}
```

### 3.4 `ReidentificationCompatibility`

```rust
// forge-topo::history::lineage_link
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationCompatibility {
    /// Index present and schema version is compatible.
    Available,
    /// No link index was built for this topology state (pre-4A operations).
    Unavailable,
    /// Index present but schema version is newer than this reader supports.
    SchemaVersionMismatch { recorded: u32, supported: u32 },
    /// Index present and compatible, but no records for the requested entity kind.
    MissingLinkage { kind: EntityKind },
    /// Query mode is not supported by this implementation.
    UnsupportedMode { mode: ReidentificationMode },
    /// Entity origin class has no forward-link records in this substrate.
    ///
    /// Reserved for future entity origins (e.g. NURBS intersection vertices,
    /// constraint-solver-derived control points) where the creation event has
    /// no parent EntityRef and thus cannot produce a forward-link record.
    /// Distinct from MissingLinkage: the index is healthy, but this class of
    /// entity is structurally unrepresentable in V1 forward linkage.
    UnsupportedEntityOrigin { origin: EntityOriginKind },
}

/// Classification of how an entity came to exist.
///
/// Used to produce typed compatibility outcomes rather than generic
/// `MissingLinkage` when a specific entity origin class is not supported
/// by the V1 re-identification substrate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityOriginKind {
    /// Born from a named Euler operator (the V1-supported case).
    EulerOperator,
    /// Born from geometric intersection of two surfaces (no parent EntityRef).
    /// Example: NURBS-NURBS intersection vertex.
    GeometricIntersection,
    /// Born from a constraint solver convergence (no named Euler operator).
    /// Example: G1-continuity constraint-derived control point.
    ConstraintSolver,
    /// Origin unknown or not yet classified.
    Unknown,
}
```

**Contract:** Callers must not treat `Unavailable` and `MissingLinkage` as the
same outcome. `Unavailable` means no substrate was built (pre-P2-4A epoch).
`MissingLinkage` means the substrate exists but has no data for this kind.
`UnsupportedEntityOrigin` means the entity class is structurally untrackable
by V1 forward linkage (e.g., NURBS intersection vertices).

### 3.5 `ReidentificationQuery`

```rust
// forge-topo::history::lineage_link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReidentificationQuery {
    /// The persistent name we want to resolve via lineage.
    pub target: PersistentNameRef,
    /// Enumeration mode.
    pub mode: ReidentificationMode,
}

/// Canonical persistent name reference (no snapshot handle exposure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentNameRef {
    pub ancestry_hash: u128,
    pub kind: EntityKind,
    pub ordinal: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationMode {
    /// Find entities whose own ancestry_hash matches the target hash.
    /// The primary/default mode.
    Descendants,
    /// Find entities that are ancestors of the target hash.
    /// Only supported for specific operation types.
    Ancestors,
    /// Intersect both (requires explicit feature opt-in).
    Hybrid,
}
```

**Ordinal contract (V1):** `PersistentNameRef::ordinal` is evaluated against the
deterministically ordered **live candidate set after compatibility checks, mode
selection, and entity-kind filtering**, and before any historical-audit-only
candidate augmentation. This rule is binding for V1 and must be used by both
direct and lineage-backed resolution paths.

**V2 note:** If historical replay requires alternative ordinal semantics, add an
explicit `OrdinalSource` enum (e.g. `CreationTime`, `PostFilterLiveSet`) rather
than changing V1 behavior in place.

### 3.6 `ReidentificationCandidate`

```rust
// forge-topo::history::lineage_link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReidentificationCandidate {
    /// Snapshot-scoped reference. Explicitly labeled (debug/provenance only).
    pub snapshot_ref: TopoSnapshotHandleRef,

    /// Persistent summary derived from this candidate's lineage linkage.
    /// `None` if the candidate was deleted and has no current live entity.
    pub derived_persistent_ref: Option<PersistentNameRef>,

    /// Whether this is a live resolver candidate or historical audit evidence.
    ///
    /// V1 resolver outputs must return `Live` only.
    pub candidate_state: ReidentificationCandidateState,

    /// How this candidate was found.
    pub match_kind: ReidentificationMatchKind,

    /// The link record that produced this candidate.
    pub link_evidence: ReidentificationLinkRecord,

    /// Deterministic sort key (computed once, not derived from transient state).
    pub rank_key: CandidateRankKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationMatchKind {
    /// Child hash exactly matches the query target hash.
    ExactChildHash,
    /// Child was produced by an operation that consumed the target entity.
    DescendantOfTarget,
    /// Found via ancestor traversal (Ancestors mode).
    AncestorOfTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationCandidateState {
    Live,
    HistoricalDeleted,
}

/// Deterministic sort key. All fields must be plain data, no floats, no pointers.
///
/// **NURBS ordinal invariant guard:** The tertiary key (`snapshot_index`) prevents
/// ordinal disambiguation from becoming circular for NURBS split siblings. When
/// multiple siblings share the same `child_hash_bytes` (same ancestry hash, different
/// created entities), they are ordered by `snapshot_index` at creation time.
/// This makes ordinal assignment deterministic without depending on the ancestry hash
/// itself. Implementations must assign ordinals by stable sort on `CandidateRankKey`
/// — never by ancestry_hash comparison alone.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CandidateRankKey {
    /// Primary: entity kind discriminant (u8).
    pub kind_discriminant: u8,
    /// Secondary: child ancestry hash (u128, big-endian bytes for ordering).
    pub child_hash_bytes: [u8; 16],
    /// Tertiary: snapshot index (creation-time arena slot).
    ///
    /// Used as ordinal tie-breaker for entities sharing an ancestry hash
    /// (e.g., NURBS trim siblings with identical topological lineage).
    pub snapshot_index: u32,
    /// Quaternary: snapshot generation.
    pub snapshot_generation: u32,
    /// Quinary: match kind discriminant.
    pub match_kind_discriminant: u8,
}
```

**Ordering contract:** `CandidateRankKey` implements `Ord`. Candidate lists must
always be sorted by `rank_key` before being returned to callers or placed in
trace payloads. No caller may depend on the order of unsorted candidate sets.

### 3.7 `ReidentificationEvidence`

```rust
// forge-topo::history::lineage_link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReidentificationEvidence {
    /// Whether the link index was present and compatible.
    pub compatibility: ReidentificationCompatibility,
    /// Schema version of the index that was queried.
    pub index_schema_version: Option<u32>,
    /// Epoch range of topology states consulted.
    pub epochs_consulted: (u64, u64),
    /// Number of records scanned in the primary index.
    pub records_scanned: u32,
    /// Number of candidates before kind/ordinal filter.
    pub candidates_pre_filter: u32,
    /// Number of candidates after filter (= final candidate count).
    pub candidates_post_filter: u32,
    /// The query mode used.
    pub mode_used: ReidentificationMode,
    /// Ordinal filter was applied (true when PersistentName.ordinal > 0).
    pub ordinal_filter_applied: bool,
    /// Suspected structural cause when re-identification failed, if diagnosable.
    ///
    /// `None` means no specific cause was identified (or re-identification succeeded).
    /// Populated by the resolver when a known failure category is detectable —
    /// for example, `ToleranceSnapVariant` when the entity count in the arena differs
    /// from a prior epoch by exactly the expected snap-merge delta.
    /// This field is advisory only: audit/replay tooling may use it for triage
    /// but must not make correctness decisions based on it.
    pub suspected_cause: Option<ReidentificationFailureCause>,
}

/// Advisory classification of why re-identification may have failed.
///
/// Never authoritative — used for audit triage, not control flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationFailureCause {
    /// Entity was genuinely deleted with no surviving descendants.
    EntityDeleted,
    /// Topology differs from a prior epoch due to a tolerance snap or merge;
    /// identity across the snap is not recoverable from lineage alone.
    ToleranceSnapVariant,
    /// Entity class origin is not supported by V1 forward linkage
    /// (e.g., NURBS intersection vertex).
    UnsupportedOriginClass { origin: EntityOriginKind },
    /// Re-identification was not attempted because the substrate was unavailable.
    SubstrateNotBuilt,
}
```

---

## 4. Build Phase: Linkage Record Construction

### 4.1 Where records are built

Records are built from the `LineageStore` at draft-commit time, inside
`MutableDraft::commit()` (and `commit_with_mode()`), before the `LineageStore`
is drained.

The existing commit code does this:

```rust
let new_events = self.lineage_store.drain_events();
let mut all_events = std::mem::take(&mut self.prior_lineage_events);
all_events.extend(new_events);
```

P2-4A adds a parallel path:

```rust
let link_records = build_link_records_from_store(&self.lineage_store, self.next_epoch);
let new_index = ReidentificationLinkIndex::build(link_records, self.next_epoch);
```

The committed `TopologyState` gains a new field:

```rust
pub struct TopologyState {
    // existing fields ...
    lineage_events: Arc<Vec<LineageEvent>>,
    /// Re-identification link index for P2-4A lineage fallback.
    /// None for states committed before P2-4A was deployed.
    reidentification_index: Option<Arc<ReidentificationLinkIndex>>,
}
```

### 4.2 `build_link_records_from_store`

```rust
// forge-topo::history::lineage_link
pub fn build_link_records_from_store(
    store: &LineageStore,
    epoch: u64,
) -> Vec<ReidentificationLinkRecord>;
```

**Algorithm:**

1. For each `EntityCreated` event in `store.events()`:
   a. Extract `entity: EntityRef` and `lineage: Lineage`.
   b. `child_ancestry_hash = lineage.get_ancestry_hash()`.
   c. `creation_op = lineage.get_creation_op()`.
   d. `origin_features = sorted(lineage.get_origin_features())`.
   e. Look up `parent_ancestry_hashes` + `parent_linkage_mode` from the lineage.
   - **Critical:** parent linkage shape must be stored explicitly in `Lineage`.
     See §4.3.
     f. Build `ReidentificationLinkRecord`.

2. Ignore `EntityModified` and `EntityDeleted` events in V1.
   (Deletion tracking is a V2 feature; V1 focuses on creation linkage.)

   **V2 extension point:** introduce an explicit record-kind discriminator
   (e.g. `ReidentificationLinkRecordKind::{Creation, Modification, Deletion}`)
   or split record structs, rather than overloading V1 creation-link semantics.

3. Return records sorted by `(epoch, child_snapshot.index, child_snapshot.generation)`.

### 4.3 Required `Lineage` schema upgrade

**This is a breaking change to the `Lineage` struct.**

Currently `Lineage::derive` computes `ancestry_hash = compute_hash(parent.ancestry_hash, &op)`
but does not store the parent's `ancestry_hash` explicitly. To build forward-link
records we need the parent hash.

**Option A (Preferred):** Add multi-parent-capable linkage fields to `Lineage`.

```rust
pub struct Lineage {
    origin_features: SmallVec<[u64; 2]>,
    creation_op: OpSignature,
    ancestry_hash: u128,
    /// Ancestry hashes of the immediate parent entities, if applicable.
    /// Empty for root entities created without a parent.
    parent_ancestry_hashes: SmallVec<[u128; 2]>,  // NEW
    /// Linkage shape for the stored parent ancestry hashes.
    parent_linkage_mode: ParentLinkageMode,       // NEW
}
```

`Lineage::derive` sets one parent hash with `parent_linkage_mode = Single`.
`Lineage::root` sets empty parent hash set with `parent_linkage_mode = None`.
`Lineage::merge` must populate compound parent hashes with
`parent_linkage_mode = Compound` when multiple parents are known.
If a producer cannot provide parent hashes for a supported origin class, it must
emit a typed compatibility/incompatibility outcome rather than silently degrading
to "no parent".

This is a **schema extension**. Existing serialized `Lineage` values deserialize
with empty parent set + `parent_linkage_mode = None` (serde `default`). Reader
compatibility is preserved, but legacy records without parent linkage must be
treated as structurally limited evidence.

**Option B (Rejected):** Scan lineage events to reconstruct parent hashes at
build time. Requires O(N²) matching and is fragile. Rejected.

### 4.4 `Lineage` accessor addition

```rust
impl Lineage {
    /// The ancestry hashes of the immediate parent entities, if tracked.
    pub fn get_parent_ancestry_hashes(&self) -> &[u128] { ... }

    /// Linkage shape for the stored parent ancestry hashes.
    pub fn get_parent_linkage_mode(&self) -> ParentLinkageMode { ... }
}
```

---

## 5. Query Phase: Re-identification Resolver

### 5.1 `ReidentificationResolver`

```rust
// forge-topo::naming::reidentify
pub struct ReidentificationResolver<'a> {
    index: &'a ReidentificationLinkIndex,
    arena: &'a TopologyArena,
}

impl<'a> ReidentificationResolver<'a> {
    pub fn new(index: &'a ReidentificationLinkIndex, arena: &'a TopologyArena) -> Self;

    /// Execute a re-identification query.
    ///
    /// Returns the ordered candidate set and evidence.
    /// Never panics. All failure modes return typed evidence with
    /// ReidentificationCompatibility != Available.
    pub fn resolve(
        &self,
        query: &ReidentificationQuery,
    ) -> (Vec<ReidentificationCandidate>, ReidentificationEvidence);
}
```

**Algorithm for `resolve`:**

1. **Compatibility gate.** Call `self.index.is_compatible(LinkSchemaVersion::V1)`.
   If not `Available`, return `(vec![], evidence_with_compatibility_status)`.

2. **Primary lookup.** Call `index.find_by_child_hash(query.target.ancestry_hash, kind)`.
   Count `records_scanned`.

3. **Live filter.** For each candidate record, check if the `child_snapshot` entity
   is still live in the current `arena`.
   - V1 `resolve()` returns only `ReidentificationCandidateState::Live` candidates.
   - Historical/deleted matches may be recorded in evidence/adjunct payloads for
     audit triage, but must not be returned as resolution candidates in V1.

4. **Ordinal filter.** If `query.target.ordinal > 0`, apply ordinal filtering to
   the deterministically ordered live candidate set defined in the V1 ordinal
   contract (§3.5). Set `ordinal_filter_applied = true`.

5. **Sort.** Sort candidates by `CandidateRankKey`. No insertion-order dependency.

6. **Build evidence.** Populate all fields of `ReidentificationEvidence`.

7. **Return.** Return `(sorted_candidates, evidence)`.

### 5.2 Integration with P2-4 resolver

The P2-4 `LineageReidentified` route phase calls:

```rust
// forge-kernel::operations::resolution (or equivalent location)
fn attempt_lineage_reidentification(
    target: &PersistentName,
    state: &TopologyState,
) -> (Vec<ReidentificationCandidate>, ReidentificationEvidence) {
    match state.reidentification_index() {
        None => {
            // Pre-P2-4A state: substrate unavailable.
            (vec![], ReidentificationEvidence::unavailable())
        }
        Some(index) => {
            let resolver = ReidentificationResolver::new(index, state.arena());
            let query = ReidentificationQuery {
                target: PersistentNameRef::from(target),
                mode: ReidentificationMode::Descendants,
            };
            resolver.resolve(&query)
        }
    }
}
```

This replaces any current stub or `Incompatible` short-circuit in the
`LineageReidentified` phase.

---

## 6. Emit Phase: Audit Payload

### 6.1 `ReidentificationAuditPayload`

This is a trace adjunct family (P2-1 contract) emitted via P2-2 finalization.

```rust
// forge-core::tracing (topo-free payload summaries only)
// Populated from topo/kernel records via summary conversion at finalization boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReidentificationAuditPayload {
    pub decision_id: DecisionId,
    pub query: PersistentNameRef,
    pub compatibility: ReidentificationCompatibility,
    pub mode_used: ReidentificationMode,
    pub candidate_count: u32,
    pub ordered_candidates: Vec<ReidentificationCandidateSummary>,
    pub evidence: ReidentificationEvidence,
    pub schema_version: LinkSchemaVersion,
    pub final_outcome: ReidentificationOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationOutcome {
    /// Exactly one candidate found and confirmed live.
    Resolved,
    /// More than one candidate found after ordinal filter (split-equivalent).
    Ambiguous,
    /// No candidates found, substrate was available.
    MissingEntity,
    /// No candidates found because substrate was unavailable.
    SubstrateUnavailable,
    /// Substrate incompatible (schema version mismatch or unsupported mode).
    Incompatible,
}

/// Compact candidate summary (no full link record, audit-safe).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReidentificationCandidateSummary {
    /// Core-level snapshot summary used only for debug/audit correlation.
    pub snapshot_ref: SnapshotHandleRef,
    pub child_ancestry_hash_hex: String,
    pub match_kind: ReidentificationMatchKind,
    pub rank_key: CandidateRankKey,
}
```

### 6.2 Adjunct registration (P2-1 contract)

```rust
// Adjunct payload kind key (stable snake_case):
pub const REIDENTIFICATION_ADJUNCT_KIND: &str = "reidentification_v1";
pub const REIDENTIFICATION_ADJUNCT_VERSION: u32 = 1;
```

The `ReidentificationAuditPayload` is attached to the `DecisionId` of the
resolution decision via the P2-1 `TraceAdjunctRecord` mechanism.

### 6.3 P2-2 finalization integration

`OperationFinalizer::collect_success` / `collect_error` must:

1. Drain re-identification adjunct payloads from the same `ModelingContext`
   adjunct sink used by policy/resolution payloads (or from an operation-local
   adjunct bundle explicitly merged by `OperationFinalizer`).
2. Serialize each as a `TraceAdjunctRecord` with the above kind/version.
3. Attach to the finalized audit bundle.

No re-identification audit payload may be emitted outside the P2-2 finalization
path for migrated operations. Split-path emission is a violation.

---

## 7. `TopologyState` API Extension

### 7.1 New field and accessor

```rust
// forge-topo::state::TopologyState
pub struct TopologyState {
    // ... existing fields ...
    reidentification_index: Option<Arc<ReidentificationLinkIndex>>,
}

impl TopologyState {
    /// The re-identification link index for this state.
    ///
    /// Returns `None` if this state was committed before P2-4A was deployed
    /// (i.e., the index was never built for this epoch).
    pub fn reidentification_index(&self) -> Option<&ReidentificationLinkIndex>;
}
```

### 7.2 `commit()` changes

```rust
// Inside MutableDraft::commit()
let link_records = build_link_records_from_store(&self.lineage_store, self.next_epoch);
let new_index = if link_records.is_empty() {
    None
} else {
    Some(Arc::new(ReidentificationLinkIndex::build(link_records, self.next_epoch)))
};

Ok(TopologyState {
    epoch: self.next_epoch,
    // ... existing fields ...
    reidentification_index: new_index,
})
```

**Performance note / requirement:** `build_link_records_from_store` is O(k) in
the number of creation events in the draft, but curved/fillet workflows may make
`k` large. Implementations must surface metrics for:
- link records built
- index build duration
- query records scanned
- `reid_records_built_per_commit`
- `reid_query_latency_us` (tagged by `EntityKind` and `ReidentificationMode`)
- `reid_candidate_count_post_filter` (distribution/histogram preferred)
These metrics must flow through operation metrics/audit paths so substrate cost
remains observable as geometry complexity grows.

**Compaction note (deferred):** Do not add record-dropping compaction (e.g.
epoch-window pruning) in V1. Compaction changes replay/re-identification
semantics and must be specified together with P2-5 audit/replay retention policy.

---

## 8. Compatibility and Versioning Contracts

### 8.1 Forward compatibility rule

A reader at `LinkSchemaVersion::V1` that encounters a record with a higher version
must return `ReidentificationCompatibility::SchemaVersionMismatch`.

It must NOT silently skip records or use partial fields.

### 8.2 Backward compatibility rule

A `TopologyState` with `reidentification_index = None` (pre-P2-4A) is valid.
The `P2-4` resolver must respond with `ReidentificationCompatibility::Unavailable`
and emit a `ResolutionRoute::LineageReidentified`-not-attempted trace, routing
to `Incompatible` typed outcome.

This is the one place `None` is acceptable: it means "feature not built yet"
and must be distinguishable from "feature built, no data."

### 8.3 Serialization schema

`ReidentificationLinkIndex` must serialize cleanly (serde derive) and round-trip
through `forge-io` JSON path. It will be embedded in `VersionedModel` / `VersionedAuditRecord`
as an optional sidecar.

`child_snapshot` / `parent_snapshot` references in serialized linkage records are
snapshot-scoped debug/provenance references only. They must never be treated as
authoritative cross-epoch identity in replay, audit, or persistent naming APIs.

### 8.4 `LineageDelta` immutability

`LineageDelta { created, deleted, modified }` must NOT gain any re-identification
fields. It is an accounting envelope. Re-identification data travels exclusively
in `ReidentificationAuditPayload` (adjunct channel). Tests must verify this:
the `LineageDelta` produced by operations using the P2-4A substrate must remain
count-only.

---

## 9. File and Crate Layout

### New files (all in `forge-topo`)

```
crates/forge-topo/src/topology/history/
├── lineage.rs              # MODIFY: add compound-capable parent linkage fields to Lineage
├── lineage_store.rs        # existing
├── lineage_link.rs         # NEW: LinkSchemaVersion, ReidentificationLinkRecord,
│                           #      ReidentificationLinkIndex, ReidentificationCompatibility,
│                           #      ReidentificationQuery, PersistentNameRef,
│                           #      ReidentificationMode, ReidentificationCandidate,
│                           #      ReidentificationMatchKind, CandidateRankKey,
│                           #      ReidentificationEvidence, ReidentificationOutcome
│                           #      build_link_records_from_store
└── mod.rs                  # MODIFY: expose lineage_link module

crates/forge-topo/src/topology/naming/
├── reidentify.rs           # NEW: ReidentificationResolver
└── mod.rs                  # MODIFY: expose reidentify module

crates/forge-topo/src/topology/
├── state.rs                # MODIFY: add reidentification_index to TopologyState + MutableDraft commit
└── mod.rs                  # no change
```

### New files (in `forge-core`)

```
crates/forge-core/src/tracing/
├── reidentification_trace.rs  # NEW: ReidentificationAuditPayload,
│                              #      ReidentificationCandidateSummary
│                              #      (topo-free summary payloads only)
│                              #      (adjunct kind constants)
└── mod.rs                     # MODIFY: expose reidentification_trace
```

### Modified files (in `forge-kernel`)

```
crates/forge-kernel/src/operations/resolution.rs  # MODIFY: add attempt_lineage_reidentification
crates/forge-kernel/src/core/context.rs           # MODIFY: thread reidentification evidence
                                                  #         into P2-2 OperationFinalizer path
```

### New test files

```
crates/forge-topo/src/topology/history/
└── tests/reidentification_tests.rs  # per §10 adversarial suite

crates/forge-topo/src/topology/naming/
└── tests/reidentify_tests.rs        # resolver behavior tests
```

---

## 10. Adversarial Test Suite (Must-Have)

All tests must be straight-line code (no loops, no conditionals) per CONVENTIONS.md.

### 10.1 Determinism tests

```rust
/// Identical operation sequences produce identical link index record sets.
#[test]
fn linkage_records_deterministic_for_identical_ops() { ... }

/// Identical query on identical index produces identical candidate sort order.
#[test]
fn candidate_ordering_deterministic_across_repeated_queries() { ... }

/// Record serialization to JSON is deterministic for same input.
#[test]
fn link_record_json_deterministic() { ... }
```

### 10.2 Generation-reuse / ABA safety tests

```rust
/// Generation bump after deletion prevents old snapshot ref from appearing
/// as a live candidate in re-identification results.
#[test]
fn generation_reuse_does_not_alias_stale_snapshot_ref() { ... }

/// Two entities at same arena index but different generations produce
/// distinct CandidateRankKeys.
#[test]
fn distinct_rank_keys_for_same_index_different_generations() { ... }
```

### 10.3 Substrate availability / compatibility tests

```rust
/// Pre-P2-4A TopologyState (reidentification_index = None) produces
/// ReidentificationCompatibility::Unavailable, not MissingLinkage.
#[test]
fn pre_substrate_state_returns_unavailable_not_missing() { ... }

/// Index with higher schema version produces SchemaVersionMismatch.
#[test]
fn future_schema_version_returns_schema_mismatch() { ... }

/// Entity kind not in index produces MissingLinkage, not Unavailable.
#[test]
fn missing_entity_kind_returns_missing_linkage_not_unavailable() { ... }
```

### 10.4 Lineage integrity tests

```rust
/// Re-identified candidate ancestry_hash matches the query target hash exactly.
#[test]
fn candidate_ancestry_hash_matches_query_target() { ... }

/// Deleted entity is not returned as live candidate.
#[test]
fn deleted_entity_not_returned_as_live_candidate() { ... }

/// Parent ancestry hash in link record matches parent entity's actual lineage.
#[test]
fn parent_hash_in_link_record_matches_parent_lineage() { ... }
```

### 10.5 `LineageDelta` purity tests

```rust
/// LineageDelta from an operation using P2-4A substrate is count-only.
/// No re-identification payload bleeds into LineageDelta fields.
#[test]
fn lineage_delta_remains_count_only_with_p2_4a_substrate() { ... }
```

### 10.6 Replay/audit bridge tests

```rust
/// ReidentificationAuditPayload is deterministic for identical candidate set.
#[test]
fn audit_payload_deterministic_for_identical_candidates() { ... }

/// ReidentificationOutcome::Resolved is distinct from SubstrateUnavailable.
#[test]
fn resolved_and_unavailable_are_distinct_outcomes() { ... }

/// Incompatible schema version maps to ReidentificationOutcome::Incompatible,
/// not MissingEntity.
#[test]
fn schema_mismatch_maps_to_incompatible_outcome() { ... }
```

### 10.7 Finalization path tests

```rust
/// ReidentificationAuditPayload is attached to the finalized trace adjunct bundle.
#[test]
fn reidentification_payload_attached_to_finalization_adjunct() { ... }

/// No reidentification payload emitted outside the P2-2 finalization path.
/// (Tests that no manual `persist_trace` or ad hoc side-channel bypass is used.)
#[test]
fn reidentification_payload_only_in_finalization_bundle() { ... }
```

---

## 11. API Contracts (Machine-Verifiable)

### 11.1 `ReidentificationLinkIndex::find_by_child_hash` contract

- **Precondition:** `self.is_compatible(LinkSchemaVersion::V1) == Available`
- **Postcondition:** Returns records in ascending `CandidateRankKey` order.
- **Postcondition:** Records where `child_snapshot.kind != kind` are excluded.
- **Postcondition:** No record is returned twice for the same child_snapshot.

### 11.2 `ReidentificationResolver::resolve` contract

- **Precondition:** `query.mode` is supported by this implementation.
- **Postcondition:** `evidence.candidates_post_filter == result.len()`.
- **Postcondition:** `result` is sorted ascending by `rank_key`.
- **Postcondition:** If `evidence.compatibility != Available`, `result` is empty.
- **Postcondition:** No `ReidentificationCandidate` in result has a `snapshot_ref`
  that appears in the result more than once (deduplicated by snapshot ref).
- **Panic guarantee:** Never panics. All error paths return typed evidence.

### 11.3 `build_link_records_from_store` contract

- **Postcondition:** Result is sorted by `(epoch, child_snapshot.index, child_snapshot.generation)`.
- **Postcondition:** Every record has `schema_version == LinkSchemaVersion::V1`.
- **Postcondition:** `origin_features` in every record is sorted ascending.
- **Postcondition:** Function is pure — calling it twice with the same store produces
  byte-identical output.

### 11.4 `TopologyState::reidentification_index` contract

- Returns `None` iff the state was produced by a `commit()` path that did not
  invoke `build_link_records_from_store` (pre-P2-4A states).
- Returns `Some(index)` iff the commit path ran and produced ≥ 1 link records.
  An empty draft (no entity creation) produces `None`.
- The returned index, if `Some`, has `index.schema_version == LinkSchemaVersion::V1`.

### 11.5 Candidate-state and historical-evidence contract

- `ReidentificationResolver::resolve` returns only
  `ReidentificationCandidateState::Live` candidates in V1.
- Historical/deleted matches, if collected for audit triage, must be emitted in
  evidence/adjunct payloads and explicitly labeled; they must not participate in
  ordinal resolution or successful identity resolution in V1.

---

## 12. Implementation Sequence (Required Order)

1. **`Lineage` schema upgrade** (add compound-capable parent linkage fields).
   - Deserializes existing data safely (serde `#[serde(default)]`).
   - Update `Lineage::derive`, `Lineage::root`, `Lineage::merge`.
   - Update lineage tests.

2. **`lineage_link.rs`** — the data model (`LinkSchemaVersion`, `ReidentificationLinkRecord`,
   `ReidentificationLinkIndex`, `ReidentificationCompatibility`,
   `ReidentificationQuery`, `PersistentNameRef`, `ReidentificationMode`,
   `ReidentificationCandidate`, `ReidentificationMatchKind`, `CandidateRankKey`,
   `ReidentificationEvidence`, `ReidentificationOutcome`, `build_link_records_from_store`).

3. **`TopologyState` integration** (add `reidentification_index` field,
   update both `commit()` paths).

4. **`ReidentificationResolver`** in `forge-topo::naming::reidentify`.

5. **`ReidentificationAuditPayload`** in `forge-core::tracing::reidentification_trace`.

6. **`forge-kernel` integration** — `attempt_lineage_reidentification` + P2-2
   finalization adjunct attachment.

7. **Adversarial test suite** (§10), running against all above.

8. **Checklist update** — fill evidence fields in FOUNDATION_PHASE2_CONTRACTS_CHECKLIST.md §P2-4A.

---

## 13. Definition of Done

**P2-4A is complete when:**

- [ ] `Lineage` parent linkage fields support `None / Single / Compound` and are populated by `derive()` / `merge()`
- [ ] `build_link_records_from_store` is pure, deterministic, and covered by round-trip test
- [ ] `ReidentificationLinkIndex` is built on `commit()` and lives in `TopologyState`
- [ ] `ReidentificationResolver::resolve` handles all four `ReidentificationCompatibility` variants
- [ ] `ReidentificationCompatibility::Unavailable` ≠ `MissingLinkage` (tested adversarially)
- [ ] `ReidentificationCompatibility::SchemaVersionMismatch` is tested adversarially
- [ ] `LineageDelta` is still count-only (verified by adversarial test)
- [ ] `ReidentificationAuditPayload` is attached via P2-2 finalization path only
- [ ] `ResolutionRoute::LineageReidentified` is only emitted when backed by real substrate
- [ ] All §10 adversarial tests pass
- [ ] Checklist §P2-4A has code + test + command + notes evidence filled

---

## 14. Cross-Reference to Dependent Specs

| Contract                         | Dependency                                                         |
| -------------------------------- | ------------------------------------------------------------------ |
| P2-4 `LineageReidentified` route | **requires P2-4A** before emitting real route                      |
| P2-2 `OperationFinalizer`        | P2-4A emits via P2-2 adjunct attachment                            |
| P2-1 `TraceAdjunctRecord`        | P2-4A audit payload uses adjunct family mechanism                  |
| P2-5 Replay/audit bridge         | classifies P2-4A evidence as exact / counterfactual / incompatible |

For P2-5: `ReidentificationCompatibility::Available` + `ReidentificationOutcome::Resolved`
maps to `ReplayCompatibility::Compatible` (if capture data sufficient). `Ambiguous` or
`MissingEntity` maps to `CounterfactualOnly`. `Unavailable` / `Incompatible` maps to
`ReplayCompatibility::RequiresWitness`. The P2-5 spec must formalize this mapping
against the `ReidentificationAuditPayload`.

---

## 15. NURBS Forward-Compatibility Invariants

This section records six named invariants that must be preserved by all future
implementations that extend the P2-4A substrate. They exist because NURBS
geometry creates entity origin patterns that are structurally incompatible with
non-careful extensions of V1 lineage linkage.

**These invariants are binding contracts, not suggestions.** Any NURBS
implementation that violates them has inherited a broken identity foundation.

---

### INV-1: Ancestry hash is a topological-operation identity

> `Lineage::ancestry_hash` encodes only `OpSignature::name` and `invocation_id`.
> It must **never** encode floating-point geometry, UV coordinates, knot vectors,
> control point positions, or any value derived from floating-point arithmetic.

**Why:** NURBS reparameterization changes UV coordinates without changing geometry.
A naïve V2 that folds UV anchors into the ancestry hash would break re-identification
for any entity touched by reparameterization, even when the B-rep is semantically
unchanged. The hash is an operation identity, not a geometric fingerprint.

**Enforcement:** Any PR that adds a floating-point argument to `Lineage::compute_hash`
or its callers must be rejected. If a geometry-sensitive identity is needed, it must
live in a separate `GeometricFingerprint` field on the entity data, never in `Lineage`.

---

### INV-2: Ordinal disambiguation must not be circular on shared ancestry hash

> When multiple sibling entities share the same `ancestry_hash` (e.g., NURBS trim
> siblings produced by the same operation), ordinal assignment must be derived from
> a key that is **not itself the ancestry hash**. The canonical secondary key is
> `snapshot_index` at creation time (the arena slot assigned during the draft).

**Why:** NURBS face splitting via trim curves produces siblings with identical
topological lineage. If ordinal assignment depends only on ancestry_hash ordering,
it becomes circular: siblings are indistinguishable from each other, and ordinal 1
cannot be reliably re-identified.

**Enforcement:** `CandidateRankKey::snapshot_index` is the ordinal tie-breaker.
The `PersistentName::ordinal` field must be assigned by stable sort on
`CandidateRankKey`, documented and tested. No implementation may assign ordinals
by ancestry_hash comparison alone.

---

### INV-3: NURBS intersection vertices declare `UnsupportedEntityOrigin`

> Entities born from geometric surface-surface intersection (e.g., NURBS-NURBS
> intersection vertices, trim endpoints) must emit
> `ReidentificationCompatibility::UnsupportedEntityOrigin { origin: EntityOriginKind::GeometricIntersection }`
> rather than attempting to build unsupported parent-hash forward-link records.

**Why:** Intersection vertices have no parent `EntityRef` — they are born from an
algorithm, not an Euler operator. The `by_parent_hash` secondary index in the
`ReidentificationLinkIndex` will have no forward entries for these entities.
Silently returning `MissingEntity` (instead of `UnsupportedEntityOrigin`) would
mislead callers into thinking the entity was deleted.

**V1 behavior:** These entities' creation events are recorded in the lineage log
but the link-building step (`build_link_records_from_store`) must detect
`EntityOriginKind::GeometricIntersection` and either omit the forward-link record
or explicitly tag it as `UnsupportedEntityOrigin` in the compatibility outcome.

**Future path (V2):** A compound-parent linkage record (two parent hashes + their
snap tolerance) can enable re-identification of intersection vertices if both
parent surfaces survive. This is a V2 extension, not a V1 goal.

---

### INV-4: Constraint-solver op names must follow the naming convention

> Entities created by a constraint solver (G1-continuity, tangency constraints,
> sketch solvers) must use a structured `creation_op_name` of the form
> `"cst:<kind>:<feature_id>"`, e.g. `"cst:g1_continuity:42"`.

**Why:** If constraint-derived entities use empty strings or non-deterministic
names, their link records become ungroupable and re-identification across feature
rebuild is impossible. The `feature_id` component ensures invocations are
distinguishable across features while remaining deterministic.

**Enforcement:** The `creation_op_name` field must pass a format validator at
build time that rejects empty strings and enforces the `"cst:"` prefix for
constraint origins. `EntityOriginKind::ConstraintSolver` entities that produce
a malformed name must fail closed, not silently produce a broken link record.

**Note:** Constraint-solver entities may additionally emit
`UnsupportedEntityOrigin { origin: EntityOriginKind::ConstraintSolver }` if the
constraint kind does not support deterministic parent linkage. This is preferable
to producing a link record with misleading provenance.

---

### INV-5: Commit-time lineage completeness check

> The `build_link_records_from_store` function must verify that every entity
> created in the draft (as counted by the arena's post-commit entity count delta)
> has a corresponding `EntityCreated` event in the `LineageStore`. If the counts
> diverge, the mismatch must be recorded in `ReidentificationEvidence` as
> `suspected_cause: Some(ReidentificationFailureCause::SubstrateNotBuilt)`
> and the affected entity kind must emit `MissingLinkage` on query.

**Why:** NURBS may introduce lazily-initialized entity types (tessellation patches,
UV-domain submeshes) that are not routed through the standard Euler operator
path and therefore never call `record_creation` on the `LineageStore`. If these
entities land in the arena without lineage, they will silently return `Missing`
on re-identification even though they exist and are live. The completeness check
converts this silent failure into a diagnosable, typed outcome.

**Scope:** The check is advisory in V1 (logged, not a commit failure). It becomes
a hard commit gate in V2 once all entity creation paths are known to produce lineage.

---

### INV-6: `LineageDelta` must remain count-only, permanently

> `LineageDelta { created, deleted, modified }` must never gain re-identification
> payload fields, geometry hashes, or entity-level provenance. It is an
> accounting envelope. This constraint applies to all future schema versions.

**Why:** `LineageDelta` is embedded in `OperationResult<T>`, which is the universal
return type for every kernel operation. Adding structured re-identification data
to it would bloat every operation result and couple the accounting path to the
identity substrate's versioning. The audit adjunct channel (§6) exists precisely
to avoid this coupling.

**Enforcement:** Any future PR that adds a non-count field to `LineageDelta`
must be rejected and redirected to `ReidentificationAuditPayload` or a new
typed adjunct family. A test (`lineage_delta_remains_count_only_with_p2_4a_substrate`)
must remain in the adversarial suite permanently and must not be removed.

---

### NURBS invariant summary

| Invariant                                              | What it guards                          | When it fires                                      |
| ------------------------------------------------------ | --------------------------------------- | -------------------------------------------------- |
| INV-1: Hash is topological only                        | Reparameterization safety               | Any attempt to fold UV/geometry into ancestry hash |
| INV-2: Ordinal not derived from hash                   | Split sibling disambiguity              | NURBS trim siblings with identical lineage         |
| INV-3: Intersection vertices → UnsupportedEntityOrigin | Correct compatibility signal            | NURBS-NURBS intersection vertex re-identification  |
| INV-4: Constraint op name convention                   | Deterministic constraint entity linkage | Constraint solver entity creation                  |
| INV-5: Commit-time completeness check                  | Silent missing entity detection         | Lazily-initialized NURBS entity types              |
| INV-6: LineageDelta count-only forever                 | Accounting/identity decoupling          | Any future attempt to enrich LineageDelta          |
