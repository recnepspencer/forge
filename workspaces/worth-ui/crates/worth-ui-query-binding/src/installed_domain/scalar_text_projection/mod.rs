mod definition;
mod executor;
mod operation;

pub(crate) use definition::scalar_text_projection_definition;
pub(crate) use executor::WorthUiScalarTextProjectionExecutor;
pub use operation::{WorthUiScalarTextProjection, WorthUiScalarTextProjectionFamily};

pub(crate) const PLATFORM_PULSE_STATUS_IDENTITY: &str = "platform.pulse.status";
pub(crate) const LOWERING_FAMILY: &str = "worth-ui-scalar-text-projection-v1";
