//! Evidence vocabulary for allocation authority boundaries.

mod drag_resize_evidence;
mod portal_anchor_evidence;
mod replan_transaction_evidence;
mod scroll_owned_evidence;
mod source_gateway_evidence;
mod truth_boundary;
mod viewport_resize_evidence;

pub use drag_resize_evidence::{UiDragResizeEvidence, UiDragResizeStrategy};
pub use portal_anchor_evidence::UiPortalAnchorMovementEvidence;
pub use replan_transaction_evidence::UiAllocationReplanTransactionEvidence;
pub use scroll_owned_evidence::{UiScrollOwnedAllocationEvidence, UiScrollOwnedExtentCause};
pub use source_gateway_evidence::UiAllocationSourceGatewayEvidence;
pub use viewport_resize_evidence::UiViewportResizeEvidence;

pub use truth_boundary::UiAllocationTruthCategory;
