#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedObservationBasisRetentionDenial {
    FrameTransitionInFlight,
    UnknownFrame,
    ExpiredFrame,
    CapacityExceeded {
        required_leases: usize,
        required_structural_bytes: usize,
        budget: super::UiMountedRetentionClassBudget,
    },
    AccountingOverflow,
}
