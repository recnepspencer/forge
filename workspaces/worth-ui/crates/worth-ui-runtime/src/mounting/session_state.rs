mod identity;
mod inspection;
mod interaction;
mod layout_reconstruction;
mod projection;
mod publication;
mod raster_cache_reconstruction;
mod replacement;

pub(crate) use publication::{
    UiMountedHostObservationTransition, UiMountedObservationValidationBasis,
    UiMountedPublicationTransition,
};
pub(crate) use replacement::{
    UiMountedGraphReplacementAdmission, UiMountedGraphReplacementInFlight,
    UiMountedGraphReplacementPreparation, UiMountedGraphReplacementPresentation,
    UiMountedGraphReplacementSuccessor,
};

use std::collections::BTreeMap;

use worth_ui_host_contract::UiMountedPresentationAttemptIdentity;

/// Mounted lifecycle authority retained by one active application session.
pub(crate) struct WorthUiMountedSessionState {
    identity: super::UiMountedIdentityState,
    retention: super::UiMountedFrameRetentionCoordinator,
    presentation: super::UiMountedPresentationCoordinator,
    publication_reservations:
        BTreeMap<UiMountedPresentationAttemptIdentity, super::UiMountedFramePublicationCandidate>,
    reconciliation_reservations: BTreeMap<
        UiMountedPresentationAttemptIdentity,
        super::UiMountedFrameReconciliationCandidate,
    >,
}

impl WorthUiMountedSessionState {
    pub(crate) fn new(
        host_session: crate::facade::WorthUiHostSessionIdentity,
        retention_budget: super::UiMountedFrameRetentionBudget,
        presentation_async: Option<
            crate::native_platform::text_presentation::UiPresentationAsyncRuntime,
        >,
    ) -> Result<Self, super::UiMountedIdentityDenial> {
        Ok(Self {
            identity: super::UiMountedIdentityState::new(host_session)?,
            retention: super::UiMountedFrameRetentionCoordinator::with_budget(retention_budget),
            presentation: super::UiMountedPresentationCoordinator::new(presentation_async),
            publication_reservations: BTreeMap::new(),
            reconciliation_reservations: BTreeMap::new(),
        })
    }

    pub(crate) fn has_active_presentation_attempt(&self) -> bool {
        self.presentation.has_active_attempt()
    }

    pub(crate) fn shutdown_presentation(
        &mut self,
        host: &crate::facade::WorthUiHostSessionAuthority,
    ) -> (
        super::UiMountedPresentationShutdownReport,
        Vec<super::UiMountedPresentationOutcome>,
        Option<crate::native_platform::text_presentation::UiPresentationAsyncTerminalCleanup>,
    ) {
        self.presentation.shutdown(host.effect_port())
    }

    pub(crate) fn assert_shutdown_resolved(&self) {
        assert!(
            self.publication_reservations.is_empty(),
            "shutdown resolves every retained mounted publication reservation"
        );
        assert!(
            self.reconciliation_reservations.is_empty(),
            "shutdown resolves every retained mounted reconciliation reservation"
        );
    }

    pub(crate) const fn native_client_resource_peaks(&self) -> [usize; 2] {
        [
            self.identity.peak_qualified_layouts(),
            self.presentation.peak_raster_cache_entries(),
        ]
    }
}
