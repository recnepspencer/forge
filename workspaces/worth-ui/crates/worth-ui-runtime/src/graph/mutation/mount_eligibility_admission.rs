use std::collections::BTreeSet;

use crate::graph::{
    UiGraphAuthority, UiGraphMountEligibilityTransition, UiGraphMutationCommitResult,
    UiGraphMutationStage, UiGraphNodeIdentity, UiGraphParticipationAxis,
    UiGraphParticipationStatus,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphMountEligibilityAdmissionDenial {
    EmptyTransitionSet,
    DuplicateNode(UiGraphNodeIdentity),
    ForeignMountEligibility(UiGraphNodeIdentity),
    PriorMountedPostureMismatch(UiGraphNodeIdentity),
    NextMountedPostureNotAdmitted(UiGraphNodeIdentity),
}

impl UiGraphAuthority<'_> {
    /// Commit mount-eligibility transitions as the proof that permits the same
    /// nodes to enter the layout-planning lane. Every transition is rebound to
    /// this exact graph before successor authority is minted.
    pub fn commit_mount_eligibility_admissions(
        self,
        transitions: Vec<UiGraphMountEligibilityTransition>,
    ) -> Result<UiGraphMutationCommitResult, UiGraphMountEligibilityAdmissionDenial> {
        if transitions.is_empty() {
            return Err(UiGraphMountEligibilityAdmissionDenial::EmptyTransitionSet);
        }

        let mut admitted_nodes = BTreeSet::new();
        for transition in &transitions {
            let node = transition.eligibility_record().graph_node_identity();
            if transition.graph_authority_identity() != self.snapshot().authority_identity() {
                return Err(UiGraphMountEligibilityAdmissionDenial::ForeignMountEligibility(node));
            }
            if !admitted_nodes.insert(node) {
                return Err(UiGraphMountEligibilityAdmissionDenial::DuplicateNode(node));
            }
            let Some(snapshot_node) = self
                .snapshot()
                .nodes()
                .iter()
                .find(|entry| entry.graph_node_identity() == node)
            else {
                return Err(UiGraphMountEligibilityAdmissionDenial::ForeignMountEligibility(node));
            };
            let Some(slot) = self.snapshot().mount_eligibility_slot_for_node(node) else {
                return Err(UiGraphMountEligibilityAdmissionDenial::ForeignMountEligibility(node));
            };
            if transition.eligibility_record() != (*slot).into() {
                return Err(UiGraphMountEligibilityAdmissionDenial::ForeignMountEligibility(node));
            }
            if transition.prior_eligibility()
                != snapshot_node
                    .participation_posture()
                    .axis(UiGraphParticipationAxis::Mounted)
            {
                return Err(
                    UiGraphMountEligibilityAdmissionDenial::PriorMountedPostureMismatch(node),
                );
            }
            if transition.next_eligibility().status() != UiGraphParticipationStatus::Admitted {
                return Err(
                    UiGraphMountEligibilityAdmissionDenial::NextMountedPostureNotAdmitted(node),
                );
            }
        }

        Ok(UiGraphMutationCommitResult::new(
            UiGraphMutationStage::mount_eligibility_admitted_successor(
                self.snapshot(),
                &transitions,
            )
            .commit(),
        ))
    }
}
