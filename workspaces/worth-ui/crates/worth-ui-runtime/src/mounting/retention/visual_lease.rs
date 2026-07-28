#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedVisualRetentionDenial {
    ExpiredFrame,
    UnknownFrame,
    CapacityExceeded {
        class: super::UiMountedRetentionClass,
        required_leases: usize,
        required_structural_bytes: usize,
        budget: super::UiMountedRetentionClassBudget,
    },
    AccountingOverflow {
        class: super::UiMountedRetentionClass,
    },
}
