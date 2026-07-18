use crate::evidence::{preflight_evidence_expansion, UiEvidenceExpansion, UiEvidenceRef};
use crate::graph::inspection::UiGraphEvidenceRecord;
use crate::graph::{
    UiGraphAxisParticipation, UiGraphCloseoutReport, UiGraphGeneration, UiGraphInspectionSupport,
    UiGraphLookupSurface, UiGraphMountedReceiptMutation, UiGraphMountedReceiptTransition,
    UiGraphNodeIdentity, UiGraphSnapshot, UiGraphSnapshotComparable, UiGraphWorldProfile,
};
use crate::obligations::touch::UiGraphTouchAuthority;

#[derive(Clone, Copy)]
pub struct UiGraphAuthority<'a> {
    snapshot: &'a UiGraphSnapshot,
}

impl<'a> UiGraphAuthority<'a> {
    pub(crate) const fn new(snapshot: &'a UiGraphSnapshot) -> Self {
        Self { snapshot }
    }

    pub fn generation(self) -> UiGraphGeneration {
        self.snapshot.generation()
    }

    pub fn node_count(self) -> usize {
        self.snapshot.node_count()
    }

    /// Enumerate graph-owned node identities without asking callers to
    /// reconstruct membership from declaration or topology indexes.
    pub fn node_identities(self) -> impl Iterator<Item = UiGraphNodeIdentity> + 'a {
        self.snapshot
            .nodes()
            .iter()
            .map(|node| node.graph_node_identity())
    }

    /// Enumerate exactly the graph nodes that require allocation-catalog
    /// coverage. This is a projection of graph authority, not caller-owned
    /// participation inference.
    pub fn allocation_planning_node_identities(
        self,
    ) -> impl Iterator<Item = UiGraphNodeIdentity> + 'a {
        self.snapshot.allocation_planning_node_identities()
    }

    pub fn mounted_receipt_slot_count(self) -> usize {
        self.snapshot.mounted_receipt_slot_count()
    }

    pub fn world_profile(self) -> &'a UiGraphWorldProfile {
        self.snapshot.world_profile()
    }

    pub fn lookup(self) -> UiGraphLookupSurface<'a> {
        self.snapshot.lookup()
    }

    pub fn inspection(self) -> UiGraphInspectionSupport<'a> {
        self.snapshot.inspection()
    }

    pub fn evidence_ref_for_node(
        self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<UiEvidenceRef> {
        self.lookup().graph_node(graph_node_identity).map(|lookup| {
            UiGraphEvidenceRecord::for_snapshot(
                self.snapshot,
                lookup.value().graph_node_identity().digest(),
            )
            .reference()
        })
    }

    pub fn expand_evidence_ref(
        self,
        evidence_ref: UiEvidenceRef,
        requested_richness: worth_ui_inspection::UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        let current_generation =
            worth_ui_inspection::UiEvidenceAuthorityGeneration::new(self.generation().as_u64());
        if let Some(preflight) =
            preflight_evidence_expansion(current_generation, evidence_ref, requested_richness)
        {
            return preflight;
        }

        UiEvidenceExpansion::new(
            evidence_ref,
            requested_richness,
            worth_ui_inspection::UiEvidenceExpansionOutcome::Unsupported,
            None,
            Box::new([]),
            None,
        )
    }

    pub fn touches(self) -> UiGraphTouchAuthority<'a> {
        UiGraphTouchAuthority::new(self.snapshot)
    }

    pub fn compare_to(self, other: Self) -> UiGraphSnapshotComparable {
        self.snapshot.compare_to(other.snapshot)
    }

    pub fn mounted_receipt_mutation_for_node(
        self,
        graph_node_identity: UiGraphNodeIdentity,
        prior_mounted_axis_participation: UiGraphAxisParticipation,
        next_mounted_axis_participation: UiGraphAxisParticipation,
    ) -> Option<UiGraphMountedReceiptMutation> {
        self.snapshot.mounted_receipt_mutation_for_node(
            graph_node_identity,
            prior_mounted_axis_participation,
            next_mounted_axis_participation,
        )
    }

    pub fn mounted_receipt_transition_for_node(
        self,
        graph_node_identity: UiGraphNodeIdentity,
        prior_mounted_axis_participation: UiGraphAxisParticipation,
        next_mounted_axis_participation: UiGraphAxisParticipation,
    ) -> Option<UiGraphMountedReceiptTransition> {
        self.snapshot.mounted_receipt_transition_for_node(
            graph_node_identity,
            prior_mounted_axis_participation,
            next_mounted_axis_participation,
        )
    }

    pub fn closeout_report(self) -> UiGraphCloseoutReport {
        UiGraphCloseoutReport::milestone33()
    }

    pub(crate) fn snapshot(self) -> &'a UiGraphSnapshot {
        self.snapshot
    }
}
