//! Provenance stamping API for `MutableDraft`.
//!
//! DOMAIN: Production-grade API for Euler operators to declare lineage.
//! Handles borrow-splitting internally so operators don't juggle
//! `lineage_store()` vs `lineage_store_mut()`.

use super::mutable_draft::MutableDraft;
use crate::provenance::{LineageMode, LineageRecorder, OperationLineageContext};

impl MutableDraft {
    // ── Provenance Stamping API ─────────────────────────────────────────
    //
    // These methods are the production-grade API for Euler operators to
    // declare lineage. They handle the borrow-splitting internally so
    // operators don't have to juggle `lineage_store()` vs `lineage_store_mut()`.

    /// Stamp multiple child entities as derived from a single parent.
    ///
    /// Used by creation operators (SplitEdge, MakeEdgeFace, MakeEdgeVertex, etc.)
    /// to declare: "these new entities were born from this parent entity."
    ///
    /// # Panics (debug builds)
    ///
    /// Panics if the parent entity has no lineage in the store. After
    /// `build_halfedge_mesh` completes, every entity MUST have lineage.
    /// A missing parent indicates a wiring bug upstream, not a recoverable condition.
    pub fn stamp_children_of(
        &mut self,
        recorder: &mut LineageRecorder,
        parent: forge_core::EntityRef,
        children: &[forge_core::EntityRef],
    ) {
        let parent_lineage = self.lineage_store.get_lineage(&parent).cloned();
        debug_assert!(
            parent_lineage.is_some(),
            "stamp_children_of: parent {:?} has no lineage — wiring bug upstream",
            parent
        );
        if let Some(ref lineage) = parent_lineage {
            for &child in children {
                recorder.stamp_derived(&mut self.lineage_store, child, lineage);
            }
        }
    }

    /// Stamp multiple child entities as merged from multiple parents.
    ///
    /// Used by Boolean operations where a new entity derives from entities
    /// on two (or more) different bodies.
    ///
    /// # Panics (debug builds)
    ///
    /// Panics if any parent entity has no lineage in the store.
    pub fn stamp_merged_children_of(
        &mut self,
        recorder: &mut LineageRecorder,
        parents: &[forge_core::EntityRef],
        children: &[forge_core::EntityRef],
    ) {
        let parent_lineages: Vec<_> = parents
            .iter()
            .map(|p| {
                let lineage = self.lineage_store.get_lineage(p).cloned();
                debug_assert!(
                    lineage.is_some(),
                    "stamp_merged_children_of: parent {:?} has no lineage — wiring bug upstream",
                    p
                );
                lineage
            })
            .flatten()
            .collect();

        if parent_lineages.len() != parents.len() {
            return; // release-mode graceful degradation
        }

        let mode = LineageMode::Merged {
            parents: parent_lineages.into(),
        };
        for &child in children {
            let context = OperationLineageContext {
                feature_id: recorder.feature_id(),
                op_name: recorder.op_name(),
                mode: mode.clone(),
            };
            let mut merge_recorder = LineageRecorder::new(context, recorder.invocation_id());
            merge_recorder.stamp(&mut self.lineage_store, child);
        }
    }
}
