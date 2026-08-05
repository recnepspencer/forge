/// Installed execution path required by one application operation.
///
/// Specialized postures are compiler-owned cutoffs: ordinary progression and
/// generic commit must not execute them as ordinary operation programs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledApplicationOperationExecutionPosture {
    Ordinary,
    DelegationActivation,
    CapabilityRevocation,
}

impl WorthQueryInstalledApplicationOperationExecutionPosture {
    pub const fn requires_delegation_activation(self) -> bool {
        matches!(self, Self::DelegationActivation)
    }

    pub const fn requires_capability_revocation(self) -> bool {
        matches!(self, Self::CapabilityRevocation)
    }
}
