mod selected_slice;
mod selection_policy;

pub use selected_slice::WorthGraphReadAccessSelectedVerticalSlice;
pub(crate) use selection_policy::select_first_vertical_slice;
pub use selection_policy::WorthGraphReadAccessSliceSelectionReason;
