/// The allocation dispatcher shares the runtime's sole causal frame epoch.
/// A type alias prevents a second epoch clock from entering the runtime.
pub type UiAllocationFrameEpoch = crate::runtime::WorthUiRuntimeFrameEpoch;
