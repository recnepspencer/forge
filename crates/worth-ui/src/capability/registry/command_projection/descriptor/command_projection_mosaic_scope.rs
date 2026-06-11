use crate::capability::MosaicPlacementPolicyId;

/// Optional mosaic placement scope for surface-bound command projections.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommandProjectionMosaicScope {
    placement_policy: MosaicPlacementPolicyId,
}

impl CommandProjectionMosaicScope {
    pub fn placement_policy(placement_policy: MosaicPlacementPolicyId) -> Self {
        Self { placement_policy }
    }

    pub fn placement_policy_id(&self) -> &MosaicPlacementPolicyId {
        &self.placement_policy
    }

    pub(crate) fn digest_basis(&self) -> String {
        self.placement_policy.as_str().to_owned()
    }
}
