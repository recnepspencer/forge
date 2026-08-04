use super::admission::RefreshAdmissionClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoalescingDecision {
    NotNeeded,
    Admitted { bundle_count: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveCoalescingError {
    BundleCountTooSmall,
    Forbidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveRefreshError {
    ForbiddenAdmissionClass(RefreshAdmissionClass),
}
