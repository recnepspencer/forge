use topology::derived_invalidation_family_catalog::DerivedTopologyProductFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupTopologyInputState {
    NotRequired,
    DerivedProductReceiptRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupTopologyInputPosture {
    state: EvidenceLookupTopologyInputState,
    required_family: Option<DerivedTopologyProductFamilyIdentity>,
}

impl EvidenceLookupTopologyInputPosture {
    pub(crate) const fn not_required() -> Self {
        Self {
            state: EvidenceLookupTopologyInputState::NotRequired,
            required_family: None,
        }
    }

    pub(crate) const fn derived_product_receipt_required(
        family: DerivedTopologyProductFamilyIdentity,
    ) -> Self {
        Self {
            state: EvidenceLookupTopologyInputState::DerivedProductReceiptRequired,
            required_family: Some(family),
        }
    }

    pub const fn state(&self) -> EvidenceLookupTopologyInputState {
        self.state
    }

    pub const fn required_family(&self) -> Option<DerivedTopologyProductFamilyIdentity> {
        self.required_family
    }

    pub fn required_family_identity(&self) -> Option<&'static str> {
        self.required_family
            .map(DerivedTopologyProductFamilyIdentity::as_str)
    }

    pub const fn requires_topology_receipt(&self) -> bool {
        matches!(
            self.state,
            EvidenceLookupTopologyInputState::DerivedProductReceiptRequired
        )
    }
}
