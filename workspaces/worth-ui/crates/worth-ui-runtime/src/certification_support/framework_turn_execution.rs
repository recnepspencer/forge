//! Certification-only access to the runtime-owned framework transition.

use crate::facade::{WorthUiActiveApplicationSession, WorthUiActiveFrameworkTurnCompletion};
use crate::runtime::WorthUiFrameworkTurn;

/// SUPPORT AUTHORITY for tests that must inspect or execute lane-local work.
///
/// Ordinary product callers execute a complete mounted frame instead.
pub trait WorthUiFrameworkTurnCertificationExt {
    fn execute_framework_turn(
        &mut self,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<
        WorthUiActiveFrameworkTurnCompletion<'_>,
        crate::mounting::UiMountedPublicationLeaseDenial,
    >;
}

impl WorthUiFrameworkTurnCertificationExt for WorthUiActiveApplicationSession {
    fn execute_framework_turn(
        &mut self,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<
        WorthUiActiveFrameworkTurnCompletion<'_>,
        crate::mounting::UiMountedPublicationLeaseDenial,
    > {
        WorthUiActiveApplicationSession::execute_framework_turn(self, collect_sources)
    }
}
