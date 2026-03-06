//! Lineage-aware operation recorder — the "passport stamp" for entity creation.
//!
//! DOMAIN: System-wide provenance (Phase 3 causality).
//!
//! `LineageRecorder` replaces the scattered `(&OpSignature, &mut u64)` parameters
//! carried by entity-creating functions. It bundles operation identity with lineage
//! policy so that stamping provenance is the natural call path — impossible to forget.
//!
//! Follows the recorder lifecycle protocol documented in
//! `docs/engineering/CROSS_CUTTING_LIFECYCLE.md`:
//!   1. Constructed from `OperationScope` (per-operation)
//!   2. Written to during execution (`stamp`, `stamp_derived`, `stamp_deletion`)
//!   3. Drained into `TopologyState.lineage_events` at commit (via `LineageStore`)
//!   4. Sealed forever after
//!
//! INVARIANTS:
//! - `OperationLineageContext` is frozen at construction — never mutated.
//! - `ordinal` is the only mutable state (monotonic per-invocation).
//! - Every `stamp()` call generates a unique `OpSignature` (op_name + ordinal).

use smallvec::SmallVec;

use forge_core::{EntityRef, KernelError};

use super::super::data::lineage::lineage_record::{Lineage, LineageEvent, OpSignature};
use super::super::data::lineage::tracking_store::LineageStore;
use crate::identity::OperationId;

/// Sentinel value for unset feature IDs. Debug builds assert against this.
pub const FEATURE_ID_UNSET: u64 = 0;

/// Explicit system-level feature ID for internal operations with no user feature.
pub const FEATURE_ID_SYSTEM: u64 = u64::MAX;

/// How this recorder stamps new entities.
///
/// The mode is set at construction time and never changes during execution.
/// Shared_operations functions call `recorder.stamp()` without knowing the mode —
/// the recorder decides what lineage to produce (strategy pattern).
#[derive(Debug, Clone)]
pub enum LineageMode {
    /// Fresh entity, no parent (primitives, initial mesh construction).
    Root,
    /// Derived from a single parent chain (fillets, Euler operators).
    Derived { parent: Lineage },
    /// Merged from N parent chains (Boolean operations).
    /// Uses `SmallVec<[Lineage; 2]>` — zero heap allocation for the common 2-parent case.
    Merged { parents: SmallVec<[Lineage; 2]> },
}

/// Frozen per-operation identity. Never mutated during execution.
///
/// Separated from `LineageRecorder` to prevent accidental mutation of
/// operation identity (feature_id, op_name, mode) during long pipelines.
#[derive(Debug, Clone)]
pub struct OperationLineageContext {
    /// The originating feature's identity.
    pub feature_id: u64,
    /// Static operator name (from `TopoOperator::NAME` or `"build_halfedge_mesh"`).
    pub op_name: &'static str,
    /// How entities created via this recorder should be stamped.
    pub mode: LineageMode,
}

/// Lineage-aware operation recorder.
///
/// Only mutable state is `ordinal` (monotonic counter for unique signatures).
/// Created per-operation from `OperationScope::lineage_recorder()`.
///
/// # Example
/// ```rust,ignore
/// let mut recorder = scope.lineage_recorder("build_halfedge_mesh", LineageMode::Root);
/// let vid = draft.insert_vertex(data);
/// recorder.stamp(draft.lineage_store_mut(), vid);
/// ```
#[derive(Debug)]
pub struct LineageRecorder {
    context: OperationLineageContext,
    invocation_id: OperationId,
    ordinal: u64,
}

impl LineageRecorder {
    /// Create a new recorder with the given context.
    ///
    /// Prefer `OperationScope::lineage_recorder()` which enforces `feature_id != 0`.
    pub fn new(context: OperationLineageContext, invocation_id: impl Into<OperationId>) -> Self {
        Self {
            context,
            invocation_id: invocation_id.into(),
            ordinal: 0,
        }
    }

    /// Read-only access to the frozen operation context.
    pub fn context(&self) -> &OperationLineageContext {
        &self.context
    }

    /// The feature ID this recorder is stamping for.
    pub fn feature_id(&self) -> u64 {
        self.context.feature_id
    }

    /// The operator name this recorder is stamping for.
    pub fn op_name(&self) -> &'static str {
        self.context.op_name
    }

    /// The invocation ID this recorder was created with.
    ///
    /// Used by `MutableDraft::stamp_merged_children_of` to propagate the real
    /// invocation identity into sub-recorders rather than using a dummy value.
    pub fn invocation_id(&self) -> OperationId {
        self.invocation_id
    }

    /// Current ordinal (for testing/debugging — not part of the public API contract).
    pub fn current_ordinal(&self) -> u64 {
        self.ordinal
    }

    /// Generate the next unique `OpSignature`.
    ///
    /// Each call increments the ordinal, producing a globally unique
    /// `(op_name, invocation_id * 10_000 + ordinal)` pair. This guarantees
    /// every entity gets a unique `ancestry_hash`.
    fn next_sig(&mut self) -> OpSignature {
        self.ordinal += 1;
        OpSignature::with_id(
            self.context.op_name,
            self.invocation_id.get() * 10_000 + self.ordinal,
        )
    }

    /// Stamp an entity using the configured mode.
    ///
    /// - `Root` → `Lineage::root(feature_id, sig)`
    /// - `Derived` → `Lineage::derive(&parent, sig)`
    /// - `Merged` → `Lineage::merge` across all parents
    ///
    /// Calls `store.apply()` internally — the single invariant-enforcing choke point.
    pub fn stamp(&mut self, store: &mut LineageStore, entity: impl Into<EntityRef>) {
        let sig = self.next_sig();
        let entity_ref = entity.into();
        let lineage = match &self.context.mode {
            LineageMode::Root => Lineage::root(self.context.feature_id, sig),
            LineageMode::Derived { parent } => Lineage::derive(parent, sig),
            LineageMode::Merged { parents } => {
                // N-ary merge: fold parents pairwise using Lineage::merge
                match parents.len() {
                    0 => Lineage::root(self.context.feature_id, sig),
                    1 => Lineage::derive(&parents[0], sig),
                    _ => {
                        // Use merge for the first two, then derive from the result
                        // for any additional parents.
                        let mut result = Lineage::merge(
                            &Some(parents[0].clone()),
                            &Some(parents[1].clone()),
                            &sig,
                        );
                        for parent in &parents[2..] {
                            let next_sig = OpSignature::with_id(
                                self.context.op_name,
                                self.ordinal, // same ordinal — this is one logical creation
                            );
                            result =
                                Lineage::merge(&Some(result), &Some(parent.clone()), &next_sig);
                        }
                        result
                    }
                }
            }
        };
        store.apply(LineageEvent::EntityCreated {
            entity: entity_ref,
            entity_snapshot: None,
            lineage,
        });
    }

    /// Stamp an entity with explicit derived lineage from a known parent.
    ///
    /// Used by Euler operators that know their specific parent entity's lineage.
    /// Ignores the recorder's configured mode — the caller explicitly provides
    /// the parent chain.
    pub fn stamp_derived(
        &mut self,
        store: &mut LineageStore,
        entity: impl Into<EntityRef>,
        parent: &Lineage,
    ) {
        let sig = self.next_sig();
        let lineage = Lineage::derive(parent, sig);
        store.apply(LineageEvent::EntityCreated {
            entity: entity.into(),
            entity_snapshot: None,
            lineage,
        });
    }

    /// Record entity deletion.
    ///
    /// Reads the entity's current lineage from the store and emits
    /// an `EntityDeleted` event preserving the lineage for replay.
    pub fn stamp_deletion(
        &mut self,
        store: &mut LineageStore,
        entity: impl Into<EntityRef>,
    ) -> Result<(), KernelError> {
        let entity_ref = entity.into();
        let lineage =
            store
                .get_lineage(&entity_ref)
                .cloned()
                .ok_or_else(|| KernelError::InternalError {
                    message: format!(
                        "LineageRecorder::stamp_deletion: entity {:?} has no lineage in store",
                        entity_ref
                    ),
                    context: None,
                })?;
        store.apply(LineageEvent::EntityDeleted {
            entity: entity_ref,
            entity_snapshot: None,
            lineage,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_core::EntityKind;

    fn make_recorder(mode: LineageMode) -> LineageRecorder {
        LineageRecorder::new(
            OperationLineageContext {
                feature_id: 42,
                op_name: "test_op",
                mode,
            },
            1,
        )
    }

    #[test]
    fn stamp_root_creates_root_lineage() {
        let mut recorder = make_recorder(LineageMode::Root);
        let mut store = LineageStore::new();
        let entity = EntityRef::new(EntityKind::Face, 0, 0);

        recorder.stamp(&mut store, entity.clone());

        assert_eq!(store.active_count(), 1);
        let lineage = store.get_lineage(&entity).unwrap();
        assert_eq!(lineage.get_creation_op().get_name(), "test_op");
        assert_eq!(lineage.get_origin_features(), &[42]);
    }

    #[test]
    fn stamp_derived_creates_child_lineage() {
        let parent = Lineage::root(42, OpSignature::with_id("parent_op", 1));
        let mut recorder = make_recorder(LineageMode::Derived {
            parent: parent.clone(),
        });
        let mut store = LineageStore::new();
        let entity = EntityRef::new(EntityKind::Vertex, 5, 0);

        recorder.stamp(&mut store, entity.clone());

        let lineage = store.get_lineage(&entity).unwrap();
        assert_eq!(lineage.get_creation_op().get_name(), "test_op");
        assert_eq!(
            lineage.get_parent_ancestry_hashes(),
            &[parent.get_ancestry_hash()]
        );
    }

    #[test]
    fn stamp_merged_creates_compound_lineage() {
        let parent_a = Lineage::root(1, OpSignature::with_id("op_a", 1));
        let parent_b = Lineage::root(2, OpSignature::with_id("op_b", 1));
        let mut recorder = make_recorder(LineageMode::Merged {
            parents: SmallVec::from_vec(vec![parent_a.clone(), parent_b.clone()]),
        });
        let mut store = LineageStore::new();
        let entity = EntityRef::new(EntityKind::Face, 10, 0);

        recorder.stamp(&mut store, entity.clone());

        let lineage = store.get_lineage(&entity).unwrap();
        assert_eq!(lineage.get_creation_op().get_name(), "test_op");
        // Should have both parent hashes
        let parent_hashes = lineage.get_parent_ancestry_hashes();
        assert_eq!(parent_hashes.len(), 2);
    }

    #[test]
    fn ordinals_are_monotonically_increasing() {
        let mut recorder = make_recorder(LineageMode::Root);
        let mut store = LineageStore::new();

        for i in 0..5 {
            recorder.stamp(&mut store, EntityRef::new(EntityKind::Face, i, 0));
        }

        assert_eq!(recorder.current_ordinal(), 5);
        assert_eq!(store.active_count(), 5);
    }

    #[test]
    fn each_stamp_produces_unique_ancestry_hash() {
        let mut recorder = make_recorder(LineageMode::Root);
        let mut store = LineageStore::new();

        let mut hashes = Vec::new();
        for i in 0..10 {
            let entity = EntityRef::new(EntityKind::Face, i, 0);
            recorder.stamp(&mut store, entity.clone());
            hashes.push(store.get_lineage(&entity).unwrap().get_ancestry_hash());
        }

        // All hashes should be unique (guaranteed by per-entity ordinal)
        let unique: std::collections::HashSet<_> = hashes.iter().collect();
        assert_eq!(
            unique.len(),
            hashes.len(),
            "ancestry hashes must be unique per entity"
        );
    }

    #[test]
    fn stamp_derived_explicit_ignores_configured_mode() {
        // Even though mode is Root, stamp_derived uses the explicit parent
        let mut recorder = make_recorder(LineageMode::Root);
        let mut store = LineageStore::new();
        let parent = Lineage::root(42, OpSignature::with_id("parent", 1));
        let entity = EntityRef::new(EntityKind::HalfEdge, 7, 0);

        recorder.stamp_derived(&mut store, entity.clone(), &parent);

        let lineage = store.get_lineage(&entity).unwrap();
        assert_eq!(
            lineage.get_parent_ancestry_hashes(),
            &[parent.get_ancestry_hash()]
        );
    }

    #[test]
    fn stamp_deletion_removes_entity() {
        let mut recorder = make_recorder(LineageMode::Root);
        let mut store = LineageStore::new();
        let entity = EntityRef::new(EntityKind::Edge, 3, 0);

        recorder.stamp(&mut store, entity.clone());
        assert_eq!(store.active_count(), 1);

        recorder.stamp_deletion(&mut store, entity.clone()).unwrap();
        assert_eq!(store.active_count(), 0);
        assert_eq!(store.events().len(), 2); // created + deleted
    }

    #[test]
    fn stamp_deletion_fails_for_untracked_entity() {
        let mut recorder = make_recorder(LineageMode::Root);
        let mut store = LineageStore::new();
        let entity = EntityRef::new(EntityKind::Face, 99, 0);

        let result = recorder.stamp_deletion(&mut store, entity);
        assert!(result.is_err());
    }

    #[test]
    fn context_is_immutable() {
        let recorder = make_recorder(LineageMode::Root);
        let ctx = recorder.context();
        assert_eq!(ctx.feature_id, 42);
        assert_eq!(ctx.op_name, "test_op");
    }
}
