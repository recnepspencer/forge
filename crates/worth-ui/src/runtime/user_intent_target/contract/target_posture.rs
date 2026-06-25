#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiUserIntentTargetPosture {
    Bound,
    Unmounted,
    Stale,
    Ambiguous,
    OutOfScope,
    Denied,
    Unsupported,
}
