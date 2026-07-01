use crate::graph::{
    UiGraphAxisParticipation, UiGraphCloseoutReport, UiGraphGeneration,
    UiGraphInspectionSupport, UiGraphLookupSurface, UiGraphMountedReceiptMutation,
    UiGraphMountedReceiptTransition, UiGraphNodeIdentity, UiGraphSnapshot,
    UiGraphSnapshotComparable, UiGraphWorldProfile,
};

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
}
