use super::{Action, Census, ModelReport, Posture, Predecessor, Recovery, Stage};
use worth_ui_host_native::{
    UiNativeEffectPosture, UiNativeLifecycleProtocolReport, UiNativePresentationEffectPhase,
    UiNativePresentationRecoveryClass, UiNativeProtocolCloseDisposition,
    UiNativeProtocolNextAction, UiNativeProtocolPredecessor, UiNativeProtocolResourceCensus,
};

pub(super) fn production_report(report: UiNativeLifecycleProtocolReport) -> ModelReport {
    ModelReport {
        posture: production_posture(report.effect_posture()),
        stages: report
            .completed_stages()
            .iter()
            .copied()
            .map(production_stage)
            .collect(),
        action: production_action(report.next_action()),
        predecessor: match report.predecessor() {
            UiNativeProtocolPredecessor::Retained => Predecessor::Retained,
            UiNativeProtocolPredecessor::Replaced => Predecessor::Replaced,
            UiNativeProtocolPredecessor::Released => Predecessor::Released,
        },
        close: match report.close_disposition() {
            UiNativeProtocolCloseDisposition::Open => None,
            UiNativeProtocolCloseDisposition::ClosedAt(point) => Some(point),
        },
        recovery_binding: report.recovery_binding(),
        reconstructed_bindings: report.reconstructed_bindings(),
        device_generation: report.device_generation(),
        surface_generation: report.surface_generation(),
        pending_readback_observed: report.pending_readback_observed(),
        peak: production_census(report.peak_census()),
        terminal: production_census(report.terminal_census()),
    }
}

fn production_posture(posture: UiNativeEffectPosture) -> Posture {
    match posture {
        UiNativeEffectPosture::BeforeEffects => Posture::BeforeEffects,
        UiNativeEffectPosture::Presentation(stage) => Posture::Stage(production_stage(stage)),
        UiNativeEffectPosture::Presented => Posture::Presented,
        UiNativeEffectPosture::PresentationIndeterminate => Posture::Stage(Stage::Handoff),
    }
}

fn production_stage(stage: UiNativePresentationEffectPhase) -> Stage {
    match stage {
        UiNativePresentationEffectPhase::Prepared => Stage::Prepared,
        UiNativePresentationEffectPhase::SurfaceAcquired => Stage::Acquired,
        UiNativePresentationEffectPhase::Encoded => Stage::Encoded,
        UiNativePresentationEffectPhase::Submitted => Stage::Submitted,
        UiNativePresentationEffectPhase::PresentHandoff => Stage::Handoff,
    }
}

fn production_action(action: UiNativeProtocolNextAction) -> Action {
    match action {
        UiNativeProtocolNextAction::Complete => Action::Complete,
        UiNativeProtocolNextAction::RetryAfterTimeout => Action::RetryAfterTimeout,
        UiNativeProtocolNextAction::WaitForVisibility => Action::WaitForVisibility,
        UiNativeProtocolNextAction::RejectValidation => Action::RejectValidation,
        UiNativeProtocolNextAction::Reconstruct(recovery) => {
            Action::Reconstruct(production_recovery(recovery))
        }
        UiNativeProtocolNextAction::Closed => Action::Closed,
    }
}

fn production_recovery(recovery: UiNativePresentationRecoveryClass) -> Recovery {
    match recovery {
        UiNativePresentationRecoveryClass::Resize => Recovery::Resize,
        UiNativePresentationRecoveryClass::Dpi => Recovery::Dpi,
        UiNativePresentationRecoveryClass::SurfaceOutdated => Recovery::SurfaceOutdated,
        UiNativePresentationRecoveryClass::SurfaceLost => Recovery::SurfaceLost,
        UiNativePresentationRecoveryClass::DeviceLost => Recovery::DeviceLost,
        UiNativePresentationRecoveryClass::PresentationIndeterminate => {
            Recovery::PresentationIndeterminate
        }
    }
}

fn production_census(census: UiNativeProtocolResourceCensus) -> Census {
    Census {
        queued: census.queued_readiness,
        held_attempts: 0,
        prepared_uploads: census.prepared_uploads,
        surfaces: census.surfaces,
        devices: census.devices,
        queues: census.queues,
        presentations: census.pending_presentations,
        readbacks: census.readbacks,
        reconstructions: census.reconstruction_requirements,
    }
}
