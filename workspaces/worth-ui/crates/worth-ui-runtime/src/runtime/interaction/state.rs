use worth_ui_host_contract::{UiHostObservationFamily, UiSurfaceBindingGeneration};

use crate::runtime::WorthUiActiveApplicationGenerationIdentity;

use super::draft::{UiDraftProcessingOutcome, UiDraftRuntimeState};
use super::gesture::{
    UiPointerGestureOutcome, UiPointerGestureRuntimeState, UiPointerGestureStopReason,
};
use super::{
    UiActivateInteraction, UiInteractionBatchReceipt, UiInteractionLifecycleSettlementReceipt,
    UiInteractionShutdownReport, UiInteractionStateSnapshot, UiInteractionStop,
    UiInteractionTransition, UiLocalInputRecipientAdmission, UiLocalInputRecipientBindingStop,
    UiLocalInputRecipientContract, UiLocalInputStopReason, UiSemanticInteraction,
};

pub(crate) struct UiInteractionRuntimeState {
    pointer: UiPointerGestureRuntimeState,
    draft: UiDraftRuntimeState,
    semantic_interactions: u64,
}

#[derive(Clone, Copy)]
pub(crate) enum UiInteractionLifecycleStopReason {
    ObservationQuarantined,
    ObservationInvalid,
    ObservationLoss {
        family: UiHostObservationFamily,
        affected: Option<worth_ui_host_contract::UiHostObservationSequenceRange>,
    },
    SurfaceRebound,
    MountedInstanceRemoved,
    ApplicationRebound,
    Shutdown,
}

impl UiInteractionRuntimeState {
    pub(crate) fn new() -> Self {
        Self {
            pointer: UiPointerGestureRuntimeState::new(),
            draft: UiDraftRuntimeState::new(),
            semantic_interactions: 0,
        }
    }

    pub(crate) fn ingest(
        &mut self,
        batch: crate::facade::observation_report::UiValidatedHostObservationBatch,
        mounted: &crate::mounting::WorthUiMountedSessionState,
        generation: &WorthUiActiveApplicationGenerationIdentity,
    ) -> UiInteractionBatchReceipt {
        let core = batch.canonical_core();
        let mut transitions = Vec::new();
        let mut ignored_reports = 0;
        for validated in batch.reports() {
            let report = validated.report();
            let pointer = self.pointer.process_report(core, report, mounted);
            let draft = self.draft.process_report(core, report, mounted, generation);
            if pointer.is_empty() && draft.is_empty() {
                ignored_reports += 1;
            }
            transitions.extend(pointer.into_iter().map(|outcome| match outcome {
                UiPointerGestureOutcome::Pressed(press) => {
                    UiInteractionTransition::PointerPressed(press)
                }
                UiPointerGestureOutcome::Completed(gesture) => {
                    self.record_semantic();
                    UiInteractionTransition::Semantic(UiSemanticInteraction::Activate(
                        UiActivateInteraction::from_pointer(gesture, generation.clone()),
                    ))
                }
                UiPointerGestureOutcome::Stopped(stop) => {
                    UiInteractionTransition::Stopped(UiInteractionStop::PointerGesture(stop))
                }
            }));
            transitions.extend(draft.into_iter().map(|outcome| match outcome {
                UiDraftProcessingOutcome::Mutation(receipt) => {
                    UiInteractionTransition::DraftMutation(receipt)
                }
                UiDraftProcessingOutcome::Semantic(interaction) => {
                    self.record_semantic();
                    UiInteractionTransition::Semantic(interaction)
                }
                UiDraftProcessingOutcome::Stopped(stop) => {
                    UiInteractionTransition::Stopped(UiInteractionStop::LocalInput(stop))
                }
            }));
        }
        UiInteractionBatchReceipt {
            core,
            frame_relation: batch.frame_relation(),
            disposition: batch.disposition(),
            transitions: transitions.into_boxed_slice(),
            ignored_reports,
            state: self.snapshot(),
        }
    }

    pub(crate) fn bind_local_recipient(
        &mut self,
        activation: UiActivateInteraction,
        generation: &WorthUiActiveApplicationGenerationIdentity,
        contract: UiLocalInputRecipientContract,
        mounted: &crate::mounting::WorthUiMountedSessionState,
    ) -> Result<UiLocalInputRecipientAdmission, UiLocalInputRecipientBindingStop> {
        self.draft.bind(activation, generation, contract, mounted)
    }

    pub(crate) fn commit_selection(
        &mut self,
        activation: UiActivateInteraction,
        option: worth_ui_query_binding::UiProjectionOptionReference,
        generation: &WorthUiActiveApplicationGenerationIdentity,
        mounted: &crate::mounting::WorthUiMountedSessionState,
    ) -> Result<super::UiSelectionCommitInteraction, super::UiSelectionCommitStop> {
        let interaction =
            super::semantic::commit_selection(activation, option, generation, mounted)?;
        self.record_semantic();
        Ok(interaction)
    }

    pub(crate) fn snapshot(&self) -> UiInteractionStateSnapshot {
        UiInteractionStateSnapshot::from_parts(
            self.pointer.snapshot(),
            self.draft.snapshot(),
            self.semantic_interactions,
        )
    }

    pub(crate) fn cancel_binding(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        reason: UiInteractionLifecycleStopReason,
    ) -> UiInteractionLifecycleSettlementReceipt {
        let pointer = self
            .pointer
            .cancel_binding(binding, reason.pointer_reason());
        let draft = self.draft.cancel_binding(binding, reason.local_reason());
        self.settlement(pointer, draft)
    }

    pub(crate) fn cancel_instance(
        &mut self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
        reason: UiInteractionLifecycleStopReason,
    ) -> UiInteractionLifecycleSettlementReceipt {
        let pointer = self
            .pointer
            .cancel_instance(instance, reason.pointer_reason());
        let draft = self.draft.cancel_instance(instance, reason.local_reason());
        self.settlement(pointer, draft)
    }

    pub(crate) fn unchanged_settlement(&self) -> UiInteractionLifecycleSettlementReceipt {
        UiInteractionLifecycleSettlementReceipt::new(Vec::new(), Vec::new(), self.snapshot())
    }

    pub(crate) fn cancel_all(
        &mut self,
        reason: UiInteractionLifecycleStopReason,
    ) -> UiInteractionLifecycleSettlementReceipt {
        let pointer = self.pointer.cancel_all(reason.pointer_reason());
        let draft = self.draft.cancel_all(reason.local_reason());
        self.settlement(pointer, draft)
    }

    pub(crate) fn shutdown(&mut self) -> UiInteractionShutdownReport {
        UiInteractionShutdownReport {
            settlement: Some(self.cancel_all(UiInteractionLifecycleStopReason::Shutdown)),
        }
    }

    fn settlement(
        &self,
        pointer: Vec<super::UiPointerGestureStop>,
        draft: Vec<super::UiLocalInputStop>,
    ) -> UiInteractionLifecycleSettlementReceipt {
        UiInteractionLifecycleSettlementReceipt::new(pointer, draft, self.snapshot())
    }

    fn record_semantic(&mut self) {
        self.semantic_interactions = self
            .semantic_interactions
            .checked_add(1)
            .expect("bounded semantic interaction counter exhausted");
    }
}

impl UiInteractionLifecycleStopReason {
    fn pointer_reason(self) -> UiPointerGestureStopReason {
        match self {
            Self::ObservationQuarantined => UiPointerGestureStopReason::ObservationQuarantined,
            Self::ObservationInvalid => UiPointerGestureStopReason::InvalidObservation,
            Self::ObservationLoss {
                family: UiHostObservationFamily::PointerButton,
                affected: Some(affected),
            } => UiPointerGestureStopReason::PointerButtonLoss { affected },
            Self::ObservationLoss { .. } => UiPointerGestureStopReason::InvalidObservation,
            Self::SurfaceRebound => UiPointerGestureStopReason::SurfaceRebound,
            Self::MountedInstanceRemoved => UiPointerGestureStopReason::MountedInstanceRemoved,
            Self::ApplicationRebound => UiPointerGestureStopReason::ApplicationRebound,
            Self::Shutdown => UiPointerGestureStopReason::Shutdown,
        }
    }

    fn local_reason(self) -> UiLocalInputStopReason {
        match self {
            Self::ObservationQuarantined | Self::ObservationInvalid => {
                UiLocalInputStopReason::ObservationInvalid
            }
            Self::ObservationLoss { family, .. } => UiLocalInputStopReason::ObservationLoss(family),
            Self::SurfaceRebound => UiLocalInputStopReason::SurfaceRebound,
            Self::MountedInstanceRemoved => UiLocalInputStopReason::MountedInstanceRemoved,
            Self::ApplicationRebound => UiLocalInputStopReason::ApplicationRebound,
            Self::Shutdown => UiLocalInputStopReason::Shutdown,
        }
    }
}
