use super::WorthUiActiveApplicationSession;
use crate::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedFrameOutcome, UiPresentationDeadline,
    UiSurfaceBindingCoordinatePosture, UiSurfaceBindingGeneration, UiSurfaceBindingProfile,
};
use std::collections::HashMap;

use super::native_observation_settlement::UiNativeObservationIngressSettlement;

#[path = "native_application_shell/component_presence.rs"]
mod component_presence;
#[path = "native_application_shell/launch.rs"]
mod launch;
#[path = "native_application_shell/presentation_recovery.rs"]
mod presentation_recovery;
#[path = "native_application_shell/query_close.rs"]
mod query_close;
pub(crate) use query_close::UiNativeApplicationQueryCloseObservation;
#[path = "native_application_shell/shutdown.rs"]
mod shutdown;
#[path = "native_application_shell/viewport_measurement.rs"]
mod viewport_measurement;
pub use shutdown::{WorthUiNativeApplicationCleanup, WorthUiNativeApplicationShutdownReceipt};

/// High-level native lifecycle for one downstream application composition root.
pub struct WorthUiNativeApplicationShell {
    pub(super) session: WorthUiActiveApplicationSession,
    binding: UiSurfaceBindingGeneration,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    scale_factor_milli: u32,
    mounted_rows: Vec<NativeMountedRow>,
    mounted_row_indices: HashMap<Box<str>, usize>,
    client_physical_size: Option<[u32; 2]>,
    viewport_measurement_pending: bool,
    viewport_measurement_authority:
        Box<[super::mounted_allocation_establishment::UiNativeViewportMeasurementAuthority]>,
    pending_surface_reconciliation: Option<crate::mounting::UiMountedSurfaceReconciliationBinding>,
    runtime_derived_state_reconstruction:
        Option<worth_ui_host_native::UiNativeClientDerivedStateReconstructionObservation>,
}

struct NativeMountedRow {
    graph_node: crate::graph::UiGraphNodeIdentity,
    mounted: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
    latest_mounted: worth_ui_host_contract::UiMountedInstanceIdentity,
}

#[derive(Debug)]
pub enum WorthUiNativeApplicationShellLaunchDenial {
    RuntimeLaunch,
    RuntimeLaunchCleanup(crate::runtime::WorthUiRuntimeLaunchDenial),
    SemanticSurfaceCreation,
    HostSurfaceRegistration,
    MountedInstanceCreation,
    ViewportAllocation(super::WorthUiMountedAllocationEstablishmentDenial),
    ApplicationCleanup(WorthUiNativeApplicationCleanup),
}

impl WorthUiNativeApplicationShell {
    pub(crate) fn admit_native_observation_batches(
        &mut self,
    ) -> UiNativeObservationIngressSettlement {
        self.session.drain_and_admit_host_observation_batches()
    }

    pub(crate) fn cancel_mounted_presentation(
        &mut self,
        in_flight: crate::mounting::UiMountedPresentationInFlight,
    ) -> crate::mounting::UiMountedFrameOutcome {
        self.session.cancel_mounted_presentation(in_flight)
    }

    pub(crate) fn presentation_attribution(
        &self,
        outcome: &UiMountedFrameOutcome,
        prior: Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
    ) -> Option<worth_ui_host_native::UiNativeClientPresentationAttribution> {
        let receipt = match outcome {
            UiMountedFrameOutcome::Published(receipt)
            | UiMountedFrameOutcome::Unchanged(receipt)
            | UiMountedFrameOutcome::Reconciled(receipt) => receipt,
            _ => return None,
        };
        let binding = *receipt.bindings().first()?;
        self.presentation_attribution_for(
            receipt.frame(),
            binding,
            receipt.attempt(),
            prior,
            matches!(outcome, UiMountedFrameOutcome::Unchanged(_)),
        )
    }

    pub(crate) fn presentation_attribution_for(
        &self,
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        binding: UiSurfaceBindingGeneration,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        prior: Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
        unchanged: bool,
    ) -> Option<worth_ui_host_native::UiNativeClientPresentationAttribution> {
        let attribution = self
            .session
            .mounted
            .native_paint_attribution(frame, binding);
        if let Some(attribution) = attribution {
            return Some(
                worth_ui_host_native::UiNativeClientPresentationAttribution::reported(
                    [
                        frame.diagnostic_value(),
                        attribution.surface.diagnostic_value(),
                        binding.diagnostic_value(),
                        attribution.mounted_instance.diagnostic_value(),
                        attribution.node_receipt.diagnostic_value(),
                        attempt.diagnostic_value(),
                    ],
                    [
                        attribution.authored_provenance_digest,
                        attribution.authored_semantic_identity_digest,
                    ],
                ),
            );
        }
        let prior = prior?;
        (!unchanged).then(|| {
            worth_ui_host_native::UiNativeClientPresentationAttribution::reported(
                [
                    frame.diagnostic_value(),
                    prior.surface(),
                    binding.diagnostic_value(),
                    prior.mounted_instance(),
                    prior.node_receipt(),
                    attempt.diagnostic_value(),
                ],
                [
                    prior.authored_provenance_digest(),
                    prior.authored_semantic_identity_digest(),
                ],
            )
        })
    }

    pub(crate) fn rebind_native_surface_scale(
        &mut self,
        scale_factor_milli: u32,
    ) -> Result<(), ()> {
        if self.pending_surface_reconciliation.is_some() {
            return Err(());
        }
        let affected = self.binding;
        let profile = UiSurfaceBindingProfile::new(
            scale_factor_milli,
            UiSurfaceBindingCoordinatePosture::LogicalPoints,
            1,
        )
        .map_err(|_| ())?;
        self.binding = self
            .session
            .rebind_host_surface(
                self.binding,
                UiHostSurfacePresentationMode::NativeDisplay,
                profile,
            )
            .map_err(|_| ())?
            .binding_generation();
        self.scale_factor_milli = scale_factor_milli;
        self.pending_surface_reconciliation = Some(
            crate::mounting::UiMountedSurfaceReconciliationBinding::new(affected, self.binding),
        );
        Ok(())
    }

    pub fn update_intent_boolean_fact(
        &mut self,
        fact: &crate::facade::intent::UiIntentApplicationFact<
            crate::facade::intent::UiIntentBoolean,
        >,
        value: bool,
    ) -> Result<
        crate::facade::intent::UiIntentApplicationFactUpdateReceipt,
        crate::facade::intent::UiIntentApplicationFactUpdateDenial,
    > {
        self.session.update_intent_boolean_fact(fact, value)
    }

    pub fn update_intent_unsigned64_fact(
        &mut self,
        fact: &crate::facade::intent::UiIntentApplicationFact<
            crate::facade::intent::UiIntentUnsigned64,
        >,
        value: u64,
    ) -> Result<
        crate::facade::intent::UiIntentApplicationFactUpdateReceipt,
        crate::facade::intent::UiIntentApplicationFactUpdateDenial,
    > {
        self.session.update_intent_unsigned64_fact(fact, value)
    }

    pub const fn rebind_deadline_at(
        &self,
        tick: u64,
    ) -> crate::runtime::rebind::UiRebindSessionDeadline {
        self.session.rebind_deadline_at(tick)
    }

    pub const fn rebind_cancellation_request(
        &self,
    ) -> crate::runtime::rebind::UiRebindCancellationRequest {
        self.session.rebind_cancellation_request()
    }

    /// Execute and present one ordinary native frame.
    pub fn present_frame(
        &mut self,
        deadline_tick: u64,
        now_tick: u64,
    ) -> Result<UiMountedFrameOutcome, super::WorthUiMountedFrameExecutionStop<'_>> {
        let request = self.session.mounted_frame_request();
        if let Some(replacement) = self.pending_surface_reconciliation {
            let replacements = [replacement];
            let outcome = self
                .session
                .execute_mounted_rebound_frame_with_application_presentation(
                    request,
                    &replacements,
                    UiPresentationDeadline::at_tick(deadline_tick),
                    now_tick,
                    |_| {},
                )?;
            self.pending_surface_reconciliation = None;
            return Ok(outcome);
        }
        let outcome = self
            .session
            .execute_mounted_frame_with_application_presentation(
                request,
                UiPresentationDeadline::at_tick(deadline_tick),
                now_tick,
                None,
                |_| {},
            )?;
        Ok(outcome)
    }

    pub(crate) fn prepare_frame(
        &mut self,
    ) -> Result<crate::mounting::UiPreparedMountedFrame, super::WorthUiMountedFrameExecutionStop<'_>>
    {
        let request = self.session.mounted_frame_request();
        if let Some(replacement) = self.pending_surface_reconciliation {
            let replacements = [replacement];
            let frame = self
                .session
                .prepare_mounted_reconciliation_frame_with_application_presentation(
                    request,
                    &replacements,
                    |_| {},
                )?;
            self.pending_surface_reconciliation = None;
            return Ok(frame);
        }
        self.session
            .prepare_mounted_frame_with_application_presentation(request, |_| {})
    }

    pub(crate) fn prepare_superseding_frame(
        &mut self,
        predecessor: &crate::mounting::UiPreparedMountedFrame,
    ) -> Result<crate::mounting::UiPreparedMountedFrame, super::WorthUiMountedFrameExecutionStop<'_>>
    {
        let request = self.session.mounted_frame_request();
        self.session
            .prepare_mounted_superseding_frame_with_application_presentation(
                request,
                predecessor,
                |_| {},
            )
    }

    pub(crate) fn present_prepared_frame(
        &mut self,
        frame: crate::mounting::UiPreparedMountedFrame,
        deadline_tick: u64,
        now_tick: u64,
    ) -> UiMountedFrameOutcome {
        self.session.present_prepared_mounted_frame_internal(
            frame,
            UiPresentationDeadline::at_tick(deadline_tick),
            now_tick,
        )
    }

    pub(crate) fn present_prepared_superseding_frame(
        &mut self,
        frame: crate::mounting::UiPreparedMountedFrame,
        predecessor: crate::mounting::UiMountedSupersedingPresentationBasis,
        deadline_tick: u64,
        now_tick: u64,
    ) -> UiMountedFrameOutcome {
        self.session
            .present_prepared_superseding_mounted_frame_internal(
                frame,
                predecessor,
                UiPresentationDeadline::at_tick(deadline_tick),
                now_tick,
            )
    }

    pub fn apply_component_semantic_text(
        &mut self,
        changes: &[super::UiNativeComponentSemanticTextChange],
    ) -> Result<(), super::UiNativeApplicationProgramDenial> {
        self.session
            .admit_application_semantic_text(changes)
            .map_err(|_| super::UiNativeApplicationProgramDenial::SemanticTextUpdateRejected)
    }

    pub(crate) fn apply_theme_token_values(
        &mut self,
        changes: &[super::UiNativeThemeTokenValueChange],
    ) -> Result<(), ()> {
        self.session.admit_application_theme_values(changes)
    }

    pub fn complete_frame_presentation(
        &mut self,
        in_flight: crate::mounting::UiMountedPresentationInFlight,
        now_tick: u64,
    ) -> UiMountedFrameOutcome {
        self.session
            .complete_mounted_presentation(in_flight, now_tick)
    }

    pub(crate) fn admit_duplicate_native_presentation_observation(
        &mut self,
        presentation: worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
    ) -> Result<(), ()> {
        self.session
            .admit_duplicate_native_presentation_observation(presentation)
    }

    pub(crate) fn retry_rejected_frame_presentation(
        &mut self,
        rejected: crate::mounting::UiMountedRejectedFrame,
        deadline: UiPresentationDeadline,
        now_tick: u64,
    ) -> UiMountedFrameOutcome {
        self.session.present_prepared_mounted_frame_internal(
            rejected.into_frame(),
            deadline,
            now_tick,
        )
    }

    pub fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        self.session.generation_identity()
    }
}
