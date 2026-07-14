use super::{CheckpointReadInterlockDenial, CheckpointRootEpochTransition};
use crate::StablePhysicalReadReceipt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointReadInterlockPlan {
    pre_publication_read: StablePhysicalReadReceipt,
    transition: CheckpointRootEpochTransition,
}

impl CheckpointReadInterlockPlan {
    pub fn admit(
        pre_publication_read: StablePhysicalReadReceipt,
        transition: CheckpointRootEpochTransition,
    ) -> Result<Self, CheckpointReadInterlockDenial> {
        let observed = pre_publication_read.read_plan_release().root();
        let expected = transition.old_current_root();
        if observed != expected {
            return Err(
                CheckpointReadInterlockDenial::PrePublicationReadReceiptMismatch {
                    expected,
                    observed,
                },
            );
        }
        Ok(Self {
            pre_publication_read,
            transition,
        })
    }

    pub const fn pre_publication_read(&self) -> StablePhysicalReadReceipt {
        self.pre_publication_read
    }

    pub const fn transition(&self) -> &CheckpointRootEpochTransition {
        &self.transition
    }
}
