#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PreviewPerturbationClass {
    ActiveBinding,
    LifecycleExplicitness,
    NoRediscovery,
    PreviewLiveAdmission,
    PreviewLiveDrift,
    InvalidBasis,
    StaleLifecycle,
    PromotionLinkageDenied,
    ReplayLinkageDenied,
    PromotionEligibilityBoolForbidden,
}
