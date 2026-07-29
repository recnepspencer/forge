#[derive(Debug, Eq, PartialEq)]
pub struct UiCommittedScrollExtentChangedFact {
    allocation_truth_revision: crate::runtime::UiAllocationTruthRevision,
    source_identity_digests: Box<[u64]>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCommittedPortalAnchorChangedFact {
    allocation_truth_revision: crate::runtime::UiAllocationTruthRevision,
    source_identity_digests: Box<[u64]>,
}

macro_rules! committed_fact {
    ($fact:ty) => {
        impl $fact {
            pub(crate) fn new(
                allocation_truth_revision: crate::runtime::UiAllocationTruthRevision,
                source_identity_digests: Box<[u64]>,
            ) -> Self {
                Self {
                    allocation_truth_revision,
                    source_identity_digests,
                }
            }

            pub const fn allocation_truth_revision(
                &self,
            ) -> crate::runtime::UiAllocationTruthRevision {
                self.allocation_truth_revision
            }

            pub fn source_identity_digests(&self) -> &[u64] {
                &self.source_identity_digests
            }
        }
    };
}

committed_fact!(UiCommittedScrollExtentChangedFact);
committed_fact!(UiCommittedPortalAnchorChangedFact);
