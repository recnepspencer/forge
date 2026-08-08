use std::time::SystemTime;

/// Failure reported by the host-installed runtime-time source.
///
/// The source is an external mechanism, not authorization. Query retains
/// ownership of timeline interpretation and maps source failure into its
/// fail-closed authorization denial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRuntimeTimeSourceDenial {
    Unavailable,
}

/// Supplies trusted current time to one published application runtime.
///
/// The host chooses this source only while consuming the primary-graph
/// bootstrap. Operation callers cannot provide samples or replace the source.
pub trait WorthQueryRuntimeTimeSource: Send + Sync + 'static {
    fn current_time(&self) -> Result<SystemTime, WorthQueryRuntimeTimeSourceDenial>;
}

impl<Source> WorthQueryRuntimeTimeSource for Box<Source>
where
    Source: WorthQueryRuntimeTimeSource + ?Sized,
{
    fn current_time(&self) -> Result<SystemTime, WorthQueryRuntimeTimeSourceDenial> {
        (**self).current_time()
    }
}

pub(crate) struct WorthQuerySystemRuntimeTimeSource;

impl WorthQueryRuntimeTimeSource for WorthQuerySystemRuntimeTimeSource {
    fn current_time(&self) -> Result<SystemTime, WorthQueryRuntimeTimeSourceDenial> {
        Ok(SystemTime::now())
    }
}
