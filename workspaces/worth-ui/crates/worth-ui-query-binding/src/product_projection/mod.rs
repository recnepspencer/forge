mod backend;
mod bridge;
mod installation;
mod source_lifecycle;
mod source_record;

pub use installation::{
    WorthUiQueryHostInstallationRequest, WorthUiScalarProjectionHostCompletion,
    WorthUiScalarProjectionHostPlan, WorthUiScalarProjectionInstallationError,
};
pub use source_lifecycle::{
    WorthUiScalarProjectionAdvance, WorthUiScalarProjectionAdvanceError,
    WorthUiScalarProjectionInstallation, WorthUiScalarProjectionLiveOwner,
    WorthUiScalarProjectionPublicationCompletion, WorthUiScalarProjectionSourceCloseReceipt,
    WorthUiScalarProjectionSourceCloseError,
};
pub use source_record::WorthUiScalarProjectionSourceRecord;

pub(crate) use backend::{
    shared_source_state, SharedSourceState, WorthUiExternalScalarSourceBackend,
};
pub(crate) use bridge::platform_pulse_bridge;
