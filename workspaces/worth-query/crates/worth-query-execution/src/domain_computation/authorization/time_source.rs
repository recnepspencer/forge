use std::time::SystemTime;

/// Failure reported by the host-installed authorization-time source.
///
/// The source is an external mechanism, not authorization. Query retains
/// ownership of timeline interpretation and maps source failure into its
/// fail-closed authorization denial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAuthorizationTimeSourceDenial {
    Unavailable,
}

/// Supplies trusted current time to one published application runtime.
///
/// The host chooses this source only while consuming the primary-graph
/// bootstrap. Operation callers cannot provide samples or replace the source.
pub trait WorthQueryAuthorizationTimeSource: Send + Sync + 'static {
    fn current_time(&self) -> Result<SystemTime, WorthQueryAuthorizationTimeSourceDenial>;
}

impl<Source> WorthQueryAuthorizationTimeSource for Box<Source>
where
    Source: WorthQueryAuthorizationTimeSource + ?Sized,
{
    fn current_time(&self) -> Result<SystemTime, WorthQueryAuthorizationTimeSourceDenial> {
        (**self).current_time()
    }
}

pub(super) struct WorthQuerySystemAuthorizationTimeSource;

impl WorthQueryAuthorizationTimeSource for WorthQuerySystemAuthorizationTimeSource {
    fn current_time(&self) -> Result<SystemTime, WorthQueryAuthorizationTimeSourceDenial> {
        Ok(SystemTime::now())
    }
}
