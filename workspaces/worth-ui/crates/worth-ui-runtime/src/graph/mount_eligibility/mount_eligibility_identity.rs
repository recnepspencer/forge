use crate::declaration::stable_text_digest;
use crate::graph::UiGraphNodeIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UiGraphMountEligibilityIdentity {
    digest: u64,
}

impl UiGraphMountEligibilityIdentity {
    pub(crate) fn graph_owned_seed_slot(node_identity: UiGraphNodeIdentity) -> Self {
        Self {
            digest: stable_text_digest("mount-eligibility-identity")
                ^ node_identity.digest().rotate_left(19),
        }
    }

    pub fn digest(self) -> u64 {
        self.digest
    }
}
