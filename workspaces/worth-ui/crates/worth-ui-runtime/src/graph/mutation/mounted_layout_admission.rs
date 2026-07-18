use std::collections::BTreeSet;

use crate::graph::{
    UiGraphAuthority, UiGraphMountedReceiptTransition, UiGraphMutationCommitResult,
    UiGraphMutationStage, UiGraphNodeIdentity, UiGraphParticipationAxis,
    UiGraphParticipationStatus,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphMountedLayoutAdmissionDenial {
    EmptyTransitionSet,
    DuplicateNode(UiGraphNodeIdentity),
    ForeignMountedReceipt(UiGraphNodeIdentity),
    PriorMountedPostureMismatch(UiGraphNodeIdentity),
    NextMountedPostureNotAdmitted(UiGraphNodeIdentity),
}

impl UiGraphAuthority<'_> {
    /// Commit mounted-receipt transitions as the proof that permits the same
    /// nodes to enter the layout-planning lane. Every transition is rebound to
    /// this exact graph before successor authority is minted.
    pub fn commit_mounted_layout_admissions(
        self,
        transitions: Vec<UiGraphMountedReceiptTransition>,
    ) -> Result<UiGraphMutationCommitResult, UiGraphMountedLayoutAdmissionDenial> {
        if transitions.is_empty() {
            return Err(UiGraphMountedLayoutAdmissionDenial::EmptyTransitionSet);
        }

        let mut admitted_nodes = BTreeSet::new();
        for transition in &transitions {
            let node = transition.authority_record().graph_node_identity();
            if transition.graph_authority_identity() != self.snapshot().authority_identity() {
                return Err(UiGraphMountedLayoutAdmissionDenial::ForeignMountedReceipt(
                    node,
                ));
            }
            if !admitted_nodes.insert(node) {
                return Err(UiGraphMountedLayoutAdmissionDenial::DuplicateNode(node));
            }
            let Some(snapshot_node) = self
                .snapshot()
                .nodes()
                .iter()
                .find(|entry| entry.graph_node_identity() == node)
            else {
                return Err(UiGraphMountedLayoutAdmissionDenial::ForeignMountedReceipt(
                    node,
                ));
            };
            let Some(slot) = self.snapshot().mounted_receipt_slot_for_node(node) else {
                return Err(UiGraphMountedLayoutAdmissionDenial::ForeignMountedReceipt(
                    node,
                ));
            };
            if transition.authority_record() != (*slot).into() {
                return Err(UiGraphMountedLayoutAdmissionDenial::ForeignMountedReceipt(
                    node,
                ));
            }
            if transition.prior_mounted_axis_participation()
                != snapshot_node
                    .participation_posture()
                    .axis(UiGraphParticipationAxis::Mounted)
            {
                return Err(UiGraphMountedLayoutAdmissionDenial::PriorMountedPostureMismatch(node));
            }
            if transition.next_mounted_axis_participation().status()
                != UiGraphParticipationStatus::Admitted
            {
                return Err(
                    UiGraphMountedLayoutAdmissionDenial::NextMountedPostureNotAdmitted(node),
                );
            }
        }

        Ok(UiGraphMutationCommitResult::new(
            UiGraphMutationStage::mounted_layout_admitted_successor(self.snapshot(), &transitions)
                .commit(),
        ))
    }
}
