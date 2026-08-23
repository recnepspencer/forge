use std::collections::VecDeque;

use super::physical_recovery_tracker::{
    UiNativePhysicalRecoverySettlement, UiNativePhysicalRecoveryTracker,
};
use super::program_reconstruction::{is_text_atlas_deferred, retry_text_atlas_deferred};
use crate::facade::WorthUiNativeApplicationShell;

#[path = "program_progress/physical_progress.rs"]
mod physical_progress;
#[path = "program_progress/superseding_pair.rs"]
mod superseding_pair;

pub(super) struct UiNativeApplicationProgramProgress {
    program: crate::facade::entry::UiNativeApplicationProgram,
    pub(super) next_frame: usize,
    next_change_frame: usize,
    next_present_tick: u64,
    pub(super) pending: VecDeque<UiNativePendingProgramFrame>,
    pub(super) next_completion_tick: u64,
    attribution: Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
    attribution_basis: Option<UiNativeApplicationAttributionBasis>,
    pub(super) physical_recovery: UiNativePhysicalRecoveryTracker,
    readiness_generation: u64,
    surface_basis_barrier: Option<(usize, u64)>,
    runtime_qualification: super::runtime_qualification::UiNativeRuntimeQualificationState,
    pub(super) staged_superseding_successor: Option<UiNativeStagedSupersedingSuccessor>,
    external_close_requested: bool,
}

#[derive(Clone, Copy)]
struct UiNativeApplicationAttributionBasis {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    unchanged: bool,
}

pub(super) struct UiNativePendingProgramFrame {
    pub(super) program_frame: usize,
    pub(super) presentation: crate::mounting::UiMountedPresentationInFlight,
    pub(super) reconstruction_authority: Option<UiNativeProgramReconstructionAuthority>,
    pub(super) cancel_after_external_submission: bool,
}

pub(super) struct UiNativeStagedSupersedingSuccessor {
    pub(super) program_frame: usize,
    pub(super) frame: crate::mounting::UiPreparedMountedFrame,
}

#[derive(Clone, Copy)]
pub(super) enum UiNativeProgramReconstructionAuthority {
    Physical(worth_ui_host_native::UiNativePhysicalPresentationCorrelation),
    HostRequired,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum FrameProgress {
    Retained,
    Settled,
    RetryRequired,
    Failed,
}

impl UiNativeApplicationProgramProgress {
    pub(super) fn new(
        program: crate::facade::entry::UiNativeApplicationProgram,
        runtime_qualification: Option<
            super::super::runtime_qualification::UiNativeRuntimeQualificationPlan,
        >,
    ) -> Self {
        Self {
            program,
            next_frame: 0,
            next_change_frame: 0,
            next_present_tick: 1,
            pending: VecDeque::new(),
            next_completion_tick: 1,
            attribution: None,
            attribution_basis: None,
            physical_recovery: UiNativePhysicalRecoveryTracker::default(),
            readiness_generation: 0,
            surface_basis_barrier: None,
            runtime_qualification:
                super::runtime_qualification::UiNativeRuntimeQualificationState::new(
                    runtime_qualification,
                ),
            staged_superseding_successor: None,
            external_close_requested: false,
        }
    }

    pub(super) fn observe_readiness_generation(&mut self, generation: u64) {
        self.readiness_generation = self.readiness_generation.max(generation);
    }

    pub(super) fn should_close(&self) -> bool {
        let program_finished =
            self.program.closes_after_program() && self.next_frame >= self.program.frames().len();
        (program_finished || self.external_close_requested)
            && self.pending.is_empty()
            && self.staged_superseding_successor.is_none()
            && self.physical_recovery.is_empty()
    }

    pub(super) fn external_observation_ready(&self) -> bool {
        !self.program.closes_after_program()
            && !self.external_close_requested
            && self.next_frame >= self.program.frames().len()
            && self.pending.is_empty()
            && self.staged_superseding_successor.is_none()
            && self.physical_recovery.is_empty()
    }

    pub(super) fn request_external_close(&mut self) {
        self.external_close_requested = true;
        self.staged_superseding_successor = None;
    }

    pub(super) fn advance(&mut self, shell: &mut WorthUiNativeApplicationShell) -> Result<(), ()> {
        if self.external_close_requested {
            return Ok(());
        }
        while self.next_frame < self.program.frames().len() {
            if self.physical_recovery.has_pending() {
                break;
            }
            if self.pending.is_empty()
                && self.staged_superseding_successor.is_none()
                && self.next_frame.saturating_add(1) < self.program.frames().len()
                && self.program.frames()[self.next_frame.saturating_add(1)]
                    .starts_by_superseding_pending()
            {
                return self.admit_prepared_superseding_pair(shell);
            }
            if !self.pending.is_empty() {
                let starts_by_superseding =
                    self.program.frames()[self.next_frame].starts_by_superseding_pending();
                let may_supersede = starts_by_superseding
                    && self.pending.iter().all(|pending| {
                        pending.presentation.awaits_progress_class(
                        worth_ui_host_contract::UiHostPresentationProgressClass::PhysicalSurface,
                    )
                    });
                if !may_supersede {
                    break;
                }
            }
            let frame = &self.program.frames()[self.next_frame];
            let program_frame = self.next_frame;
            if frame.awaits_host_surface_basis_successor() {
                match self.surface_basis_barrier {
                    Some((barrier_frame, predecessor_generation))
                        if barrier_frame == program_frame
                            && self.readiness_generation > predecessor_generation =>
                    {
                        self.surface_basis_barrier = None;
                    }
                    Some((barrier_frame, _)) if barrier_frame == program_frame => break,
                    None => {
                        self.surface_basis_barrier =
                            Some((program_frame, self.readiness_generation));
                        break;
                    }
                    Some(_) => return Err(()),
                }
            }
            if program_frame == self.next_change_frame {
                shell
                    .apply_component_presence(frame.component_presence())
                    .map_err(|_| ())?;
                shell
                    .apply_component_semantic_text(frame.semantic_text())
                    .map_err(|_| ())?;
                shell
                    .apply_theme_token_values(frame.theme_values())
                    .map_err(|_| ())?;
                self.next_change_frame = self.next_change_frame.saturating_add(1);
            } else if program_frame > self.next_change_frame {
                return Err(());
            }
            let tick = self.next_present_tick;
            self.next_present_tick = self.next_present_tick.checked_add(1).ok_or(())?;
            let reconstruction = self.runtime_qualification.reconstruction_required();
            let outcome = if reconstruction {
                shell.reconstruct_current_presentation(u64::MAX, tick)?
            } else {
                shell.present_frame(u64::MAX, tick).map_err(|_| ())?
            };
            let reconstruction_authority =
                reconstruction.then_some(UiNativeProgramReconstructionAuthority::HostRequired);
            let progress = self.retain_or_attribute(
                shell,
                outcome,
                program_frame,
                None,
                reconstruction_authority,
                frame.cancels_after_external_submission(),
            )?;
            match progress {
                FrameProgress::Retained | FrameProgress::Settled => {
                    self.next_frame = self.next_frame.saturating_add(1);
                }
                FrameProgress::RetryRequired => break,
                FrameProgress::Failed => return Err(()),
            }
        }
        Ok(())
    }

    pub(super) fn attribution(
        &self,
        shell: Option<&WorthUiNativeApplicationShell>,
    ) -> Option<worth_ui_host_native::UiNativeClientPresentationAttribution> {
        self.attribution.or_else(|| {
            let shell = shell?;
            let basis = self.attribution_basis?;
            shell.presentation_attribution_for(
                basis.frame,
                basis.binding,
                basis.attempt,
                self.attribution,
                basis.unchanged,
            )
        })
    }

    pub(super) fn retain_or_attribute(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        outcome: crate::mounting::UiMountedFrameOutcome,
        program_frame: usize,
        physical_presentation: Option<
            worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
        >,
        reconstruction_authority: Option<UiNativeProgramReconstructionAuthority>,
        cancel_after_external_submission: bool,
    ) -> Result<FrameProgress, ()> {
        let outcome = apply_completion_intent(shell, outcome, cancel_after_external_submission)?;
        if let Some(basis) = attribution_basis_from_outcome(&outcome) {
            self.attribution_basis = Some(basis);
        }
        match outcome {
            crate::mounting::UiMountedFrameOutcome::InFlight(in_flight) => {
                self.pending.push_back(UiNativePendingProgramFrame {
                    program_frame,
                    presentation: in_flight,
                    reconstruction_authority,
                    cancel_after_external_submission,
                });
                Ok(FrameProgress::Retained)
            }
            crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected)
                if rejected.rejections().iter().all(|rejection| {
                    rejection.denial()
                        == worth_ui_host_contract::UiHostSurfacePresentationDenial::TextAtlasPresentationDeferred
                }) =>
            {
                Ok(FrameProgress::RetryRequired)
            }
            crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected)
                if rejected.rejections().iter().all(|rejection| {
                    rejection.denial()
                        == worth_ui_host_contract::UiHostSurfacePresentationDenial::ReconstructionRequired
                }) =>
            {
                self.next_completion_tick = self.next_completion_tick.saturating_add(1);
                let reconstruction = shell
                    .reconstruct_current_presentation(u64::MAX, self.next_completion_tick)
                    .map_err(|_| ())?;
                self.retain_or_attribute(
                    shell,
                    reconstruction,
                    program_frame,
                    None,
                    Some(UiNativeProgramReconstructionAuthority::HostRequired),
                    false,
                )
            }
            crate::mounting::UiMountedFrameOutcome::PresentationIndeterminate(indeterminate) => {
                if indeterminate.report().awaits_physical_recovery() {
                    if let Some(presentation) = physical_presentation {
                        if presentation.attempt() != indeterminate.report().attempt()
                        {
                            return Err(());
                        }
                    }
                    for binding in indeterminate.report().physical_recovery_bindings() {
                        self.physical_recovery
                            .expect(indeterminate.report().attempt(), *binding)
                            .map_err(|_| ())?;
                    }
                    if let Some(presentation) = physical_presentation.filter(|presentation| {
                        indeterminate
                            .report()
                            .physical_recovery_bindings()
                            .contains(&presentation.binding())
                    }) {
                        self.physical_recovery
                            .observe_scheduled(presentation)
                            .map_err(|_| ())?;
                    }
                    return Ok(FrameProgress::Settled);
                }
                self.next_completion_tick = self.next_completion_tick.saturating_add(1);
                let recovery = shell
                    .reconstruct_current_presentation(u64::MAX, self.next_completion_tick)
                    .map_err(|_| ())?;
                self.retain_or_attribute(shell, recovery, program_frame, None, None, false)
            }
            outcome @ (crate::mounting::UiMountedFrameOutcome::Published(_)
            | crate::mounting::UiMountedFrameOutcome::Unchanged(_)
            | crate::mounting::UiMountedFrameOutcome::Reconciled(_)) => {
                if let Some(observed) =
                    shell.presentation_attribution(&outcome, self.attribution)
                {
                    self.attribution = Some(observed);
                }
                self.runtime_qualification
                    .observe_settled_presentation(shell, reconstruction_authority.is_some())?;
                Ok(FrameProgress::Settled)
            }
            crate::mounting::UiMountedFrameOutcome::Superseded(_) => Ok(FrameProgress::Settled),
            crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(_)
            | crate::mounting::UiMountedFrameOutcome::RetentionDenied(_)
            | crate::mounting::UiMountedFrameOutcome::AdmissionDenied(_)
            | crate::mounting::UiMountedFrameOutcome::CompletionDenied(_) => {
                Ok(FrameProgress::Failed)
            }
        }
    }
}

fn apply_completion_intent(
    shell: &mut WorthUiNativeApplicationShell,
    outcome: crate::mounting::UiMountedFrameOutcome,
    cancel_after_external_submission: bool,
) -> Result<crate::mounting::UiMountedFrameOutcome, ()> {
    if !cancel_after_external_submission {
        return Ok(outcome);
    }
    match outcome {
        crate::mounting::UiMountedFrameOutcome::InFlight(in_flight)
            if in_flight.awaits_progress_class(
                worth_ui_host_contract::UiHostPresentationProgressClass::PhysicalSurface,
            ) =>
        {
            Ok(shell.cancel_mounted_presentation(in_flight))
        }
        outcome @ crate::mounting::UiMountedFrameOutcome::InFlight(_) => Ok(outcome),
        _ => Err(()),
    }
}

fn attribution_basis_from_outcome(
    outcome: &crate::mounting::UiMountedFrameOutcome,
) -> Option<UiNativeApplicationAttributionBasis> {
    let (receipt, unchanged) = match outcome {
        crate::mounting::UiMountedFrameOutcome::Published(receipt)
        | crate::mounting::UiMountedFrameOutcome::Reconciled(receipt) => (receipt, false),
        crate::mounting::UiMountedFrameOutcome::Unchanged(receipt) => (receipt, true),
        _ => return None,
    };
    Some(UiNativeApplicationAttributionBasis {
        frame: receipt.frame(),
        binding: *receipt.bindings().first()?,
        attempt: receipt.attempt(),
        unchanged,
    })
}
