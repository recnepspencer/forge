use super::super::progress::UiObservationProgress;
use super::super::turn::{
    UiAdmittedObservation, UiAdmittedObservationPayload, UiAdmittedObservationSeal,
    UiObservationAdmissionDenial, UiObservationAdmissionReceipt, UiObservationTurn,
};
use super::super::UiObservationFamily;

#[derive(Debug)]
pub struct UiCommittedScrollExtentObservation {
    allocation_truth_revision: crate::runtime::UiAllocationTruthRevision,
    source_identity_digests: Box<[u64]>,
}

#[derive(Debug)]
pub struct UiCommittedPortalAnchorObservation {
    allocation_truth_revision: crate::runtime::UiAllocationTruthRevision,
    source_identity_digests: Box<[u64]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCommittedRuntimeStateAdmissionReceipt {
    admitted: Box<[UiObservationAdmissionReceipt]>,
}

impl UiObservationTurn<'_> {
    pub fn admit_committed_runtime_state(
        &mut self,
    ) -> Result<UiCommittedRuntimeStateAdmissionReceipt, UiObservationAdmissionDenial> {
        let revision = self.runtime.allocation_truth_revision();
        let (scroll_sources, portal_sources) = {
            let authority = self.runtime.allocation_invalidation_index.borrow();
            let mut scroll_sources = authority
                .catalog
                .iter()
                .flat_map(|(_, row)| row.scroll_sources())
                .map(
                    crate::runtime::allocation_receipt::UiCommittedScrollActivationSource::identity_digest,
                )
                .collect::<Vec<_>>();
            let mut portal_sources = authority
                .catalog
                .iter()
                .filter_map(|(_, row)| row.portal_source())
                .map(
                    crate::runtime::allocation_receipt::UiCommittedPortalActivationSource::identity_digest,
                )
                .collect::<Vec<_>>();
            scroll_sources.sort_unstable();
            scroll_sources.dedup();
            portal_sources.sort_unstable();
            portal_sources.dedup();
            (scroll_sources, portal_sources)
        };
        let observations = seal_committed_runtime_state(
            revision,
            scroll_sources,
            portal_sources,
            self.session,
            self.source_basis,
        );
        let admitted = if observations.is_empty() {
            Box::new([])
        } else {
            self.admit_batch(observations)?
        };
        Ok(UiCommittedRuntimeStateAdmissionReceipt { admitted })
    }
}

fn seal_committed_runtime_state(
    revision: crate::runtime::UiAllocationTruthRevision,
    scroll_sources: Vec<u64>,
    portal_sources: Vec<u64>,
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    source_basis: u64,
) -> Vec<UiAdmittedObservation> {
    let owner_order = revision.revision();
    let mut observations = Vec::with_capacity(2);
    if !scroll_sources.is_empty() {
        let observation = UiCommittedScrollExtentObservation {
            allocation_truth_revision: revision,
            source_identity_digests: scroll_sources.into_boxed_slice(),
        };
        observations.push(UiAdmittedObservation::seal(UiAdmittedObservationSeal {
            family: UiObservationFamily::CommittedScrollExtent,
            owner_order,
            retained_bytes: observation.retained_bytes(),
            session,
            source_basis,
            progress: Some(UiObservationProgress::committed_scroll_extent(owner_order)),
            payload: UiAdmittedObservationPayload::CommittedScrollExtent(observation),
        }));
    }
    if !portal_sources.is_empty() {
        let observation = UiCommittedPortalAnchorObservation {
            allocation_truth_revision: revision,
            source_identity_digests: portal_sources.into_boxed_slice(),
        };
        observations.push(UiAdmittedObservation::seal(UiAdmittedObservationSeal {
            family: UiObservationFamily::CommittedPortalAnchor,
            owner_order,
            retained_bytes: observation.retained_bytes(),
            session,
            source_basis,
            progress: Some(UiObservationProgress::committed_portal_anchor(owner_order)),
            payload: UiAdmittedObservationPayload::CommittedPortalAnchor(observation),
        }));
    }
    observations
}

macro_rules! committed_observation_accessors {
    ($observation:ty) => {
        impl $observation {
            pub const fn allocation_truth_revision(
                &self,
            ) -> crate::runtime::UiAllocationTruthRevision {
                self.allocation_truth_revision
            }

            pub fn source_identity_digests(&self) -> &[u64] {
                &self.source_identity_digests
            }

            fn retained_bytes(&self) -> usize {
                std::mem::size_of::<Self>()
                    .saturating_add(std::mem::size_of_val(self.source_identity_digests.as_ref()))
            }
        }
    };
}

committed_observation_accessors!(UiCommittedScrollExtentObservation);
committed_observation_accessors!(UiCommittedPortalAnchorObservation);

impl UiCommittedRuntimeStateAdmissionReceipt {
    pub fn admitted(&self) -> &[UiObservationAdmissionReceipt] {
        &self.admitted
    }
}
