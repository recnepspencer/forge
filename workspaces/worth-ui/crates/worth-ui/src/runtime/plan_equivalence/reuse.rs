#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiPlanReuseClassification {
    Reusable,
    RebuildRequired,
}
