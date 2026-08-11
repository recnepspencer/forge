#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryBindingFreshness {
    Retained,
    ExpiredAtSelectedCheckpoint,
}

pub const fn classify_binding_freshness(
    selected_checkpoint_generation: u64,
    lease_expiry_generation: u64,
) -> RecoveryBindingFreshness {
    if selected_checkpoint_generation >= lease_expiry_generation {
        RecoveryBindingFreshness::ExpiredAtSelectedCheckpoint
    } else {
        RecoveryBindingFreshness::Retained
    }
}
