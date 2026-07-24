struct UiHostAdapterSessionGrant;

impl worth_proof::AuthorityMarker for UiHostAdapterSessionGrant {}

/// Concrete runtime-owned capability required at every host-effect boundary.
///
/// Adapter implementations may inspect the admitted session identity, but only
/// the Worth UI runtime can construct this capability.
pub struct UiHostAdapterSessionAuthority {
    host_session_identity: u64,
    _authority: worth_proof::AuthorityWitness<UiHostAdapterSessionGrant>,
    presentation_lease_gate: worth_ui_host_contract::UiMountedPresentationLeaseGate,
}

impl UiHostAdapterSessionAuthority {
    pub(crate) fn activate(host_session_identity: u64) -> Self {
        Self {
            host_session_identity,
            _authority: worth_proof::AuthorityWitness::from_authority_marker(
                UiHostAdapterSessionGrant,
            ),
            presentation_lease_gate: Default::default(),
        }
    }

    pub fn host_session_identity(&self) -> u64 {
        self.host_session_identity
    }

    pub fn admits_mounted_presentation(
        &self,
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    ) -> bool {
        view.host_session_identity() == self.host_session_identity
            && self.presentation_lease_gate.admits(view)
    }

    pub fn admits_mounted_completion_token(
        &self,
        token: &worth_ui_host_contract::UiHostPresentationCompletionToken,
    ) -> bool {
        self.presentation_lease_gate.admits_token(token)
    }

    pub(crate) fn claim_mounted_presentation_lease(
        &self,
    ) -> Result<
        worth_ui_host_contract::UiMountedPresentationLease,
        worth_ui_host_contract::UiMountedPresentationLeaseDenial,
    > {
        self.presentation_lease_gate.claim()
    }
}

impl std::fmt::Debug for UiHostAdapterSessionAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiHostAdapterSessionAuthority")
            .field("host_session_identity", &self.host_session_identity)
            .finish_non_exhaustive()
    }
}
