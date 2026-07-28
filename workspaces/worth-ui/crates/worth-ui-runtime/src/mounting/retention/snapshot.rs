#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiMountedRetentionUsageSnapshot {
    pub(crate) retained_items: usize,
    pub(crate) retained_structural_bytes: usize,
    pub(crate) active_leases: usize,
    pub(crate) lease_charged_structural_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedFrameRetentionSnapshot {
    pub(crate) current: UiMountedRetentionUsageSnapshot,
    pub(crate) in_flight: UiMountedRetentionUsageSnapshot,
    pub(crate) observation_basis: UiMountedRetentionUsageSnapshot,
    pub(crate) predecessor_inspection: UiMountedRetentionUsageSnapshot,
    pub(crate) diagnostic: UiMountedRetentionUsageSnapshot,
    pub(crate) visual_snapshot: UiMountedRetentionUsageSnapshot,
    pub(crate) visual_overlay: UiMountedRetentionUsageSnapshot,
    pub(crate) budget: super::UiMountedFrameRetentionBudget,
}
