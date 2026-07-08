//! Narrow inspection bridge — dispatch classifier, admission, routing, and bridge receipts.

mod admission;
mod boundary_access;
mod dispatch;
pub(crate) mod obligation_routes;
mod routes;
pub(crate) mod support_routing;

pub use dispatch::{classify_inspection_dispatch, InspectionDispatchLane};
pub(crate) use routes::route_inspection;
pub use super::inspection::UiInspectionAiHarness;
pub use super::inspection_observation::UiInspectionFacadeObservation;
pub use super::inspection_receipt::UiInspectionReceipt;
pub use super::measurement_inspection_evidence::UiMeasurementInspectionEvidenceBundle;
pub use worth_ui_inspection::UiInspectionClosureReport;