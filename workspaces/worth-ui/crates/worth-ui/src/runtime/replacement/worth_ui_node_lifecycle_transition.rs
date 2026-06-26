#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiNodeLifecycleTransition {
    Preserve,
    Replace,
    Drop,
    Create,
    Move,
    Rebind,
    LaneChange,
}
