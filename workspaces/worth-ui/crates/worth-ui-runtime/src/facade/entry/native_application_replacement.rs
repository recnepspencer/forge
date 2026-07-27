use super::{
    WorthUiApplicationCutoverReceipt, WorthUiMountedApplicationReplacementOutcome,
    WorthUiMountedReplacementPreparationOutcome, WorthUiNativeApplicationShell,
};
use crate::mounting::{UiMountedFramePublicationReceipt, UiMountedFrameRequest};
use worth_ui_host_contract::UiPresentationDeadline;

pub enum WorthUiNativeApplicationReplacementOutcome {
    Published {
        application: WorthUiApplicationCutoverReceipt,
        mounted: UiMountedFramePublicationReceipt,
    },
    SemanticNoOp(Box<super::WorthUiApplicationSemanticNoOpReceipt>),
}

#[derive(Debug)]
pub enum WorthUiNativeApplicationReplacementDenial {
    CandidatePreparation,
    CandidateAllocation(super::WorthUiMountedAllocationEstablishmentDenial),
    CandidateLowering,
    CandidateStaging,
    FrameBoundaryUnavailable,
    CutoverPreparation,
    RejectedBeforeEffects,
    PresentationInFlight,
    PresentationIndeterminate,
    RetentionDenied,
    AdmissionDenied,
    CompletionDenied,
}

impl WorthUiNativeApplicationShell {
    /// Publish one proof-carrying whole-application successor.
    pub fn replace_application(
        &mut self,
        submission: crate::runtime::WorthUiWatchedCandidateSubmission,
        deadline_tick: u64,
        now_tick: u64,
    ) -> Result<WorthUiNativeApplicationReplacementOutcome, WorthUiNativeApplicationReplacementDenial>
    {
        let mut prepared = self
            .session
            .prepare_replacement(submission)
            .map_err(|_| WorthUiNativeApplicationReplacementDenial::CandidatePreparation)?;
        let catalog = self
            .session
            .admit_native_replacement_allocation_catalog(&mut prepared)
            .map_err(WorthUiNativeApplicationReplacementDenial::CandidateAllocation)?;
        let lowered = self
            .session
            .lower_prepared_replacement(*prepared)
            .map_err(|_| WorthUiNativeApplicationReplacementDenial::CandidateLowering)?;
        let pending = self
            .session
            .stage_prepared_replacement(lowered)
            .map_err(|_| WorthUiNativeApplicationReplacementDenial::CandidateStaging)?;
        let boundary = self
            .session
            .execute_framework_turn(|_| {})
            .map_err(|_| WorthUiNativeApplicationReplacementDenial::FrameBoundaryUnavailable)?
            .into_completion()
            .into_execution()
            .map_err(|_| WorthUiNativeApplicationReplacementDenial::FrameBoundaryUnavailable)?
            .into_activation_boundary();
        let prepared = self
            .session
            .prepare_mounted_replacement(
                pending,
                catalog,
                boundary,
                None,
                UiMountedFrameRequest::all_bound_surfaces(),
            )
            .map_err(|_| WorthUiNativeApplicationReplacementDenial::CutoverPreparation)?;
        match prepared {
            WorthUiMountedReplacementPreparationOutcome::SemanticNoOp(receipt) => Ok(
                WorthUiNativeApplicationReplacementOutcome::SemanticNoOp(receipt),
            ),
            WorthUiMountedReplacementPreparationOutcome::Prepared(replacement) => map_presentation(
                replacement.present(UiPresentationDeadline::at_tick(deadline_tick), now_tick),
            ),
        }
    }

    pub fn capabilities(&self) -> &crate::facade::registry::snapshot::CapabilitySnapshot {
        self.session.capabilities()
    }
}

fn map_presentation(
    outcome: WorthUiMountedApplicationReplacementOutcome<'_>,
) -> Result<WorthUiNativeApplicationReplacementOutcome, WorthUiNativeApplicationReplacementDenial> {
    match outcome {
        WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted,
        } => Ok(WorthUiNativeApplicationReplacementOutcome::Published {
            application,
            mounted,
        }),
        WorthUiMountedApplicationReplacementOutcome::RejectedBeforeEffects(_) => {
            Err(WorthUiNativeApplicationReplacementDenial::RejectedBeforeEffects)
        }
        WorthUiMountedApplicationReplacementOutcome::InFlight(_) => {
            Err(WorthUiNativeApplicationReplacementDenial::PresentationInFlight)
        }
        WorthUiMountedApplicationReplacementOutcome::PresentationIndeterminate(_) => {
            Err(WorthUiNativeApplicationReplacementDenial::PresentationIndeterminate)
        }
        WorthUiMountedApplicationReplacementOutcome::RetentionDenied(_) => {
            Err(WorthUiNativeApplicationReplacementDenial::RetentionDenied)
        }
        WorthUiMountedApplicationReplacementOutcome::AdmissionDenied(_) => {
            Err(WorthUiNativeApplicationReplacementDenial::AdmissionDenied)
        }
        WorthUiMountedApplicationReplacementOutcome::CompletionDenied(_) => {
            Err(WorthUiNativeApplicationReplacementDenial::CompletionDenied)
        }
    }
}
