use crate::runtime::{
    WorthUiHeaderFrameRebindDenial, WorthUiHeaderFrameRebindStatus,
    WorthUiProjectionRebindPlanDenial, WorthUiRuntimeChangeActivationPosture,
    WorthUiRuntimeChangeAdmissionDenial,
};

pub(super) fn map_runtime_change_denial(
    denial: WorthUiRuntimeChangeAdmissionDenial,
) -> WorthUiHeaderFrameRebindDenial {
    match denial {
        WorthUiRuntimeChangeAdmissionDenial::RuntimeInstanceMismatch => {
            WorthUiHeaderFrameRebindDenial::RuntimeEvidenceMismatch
        }
        WorthUiRuntimeChangeAdmissionDenial::ActivatedFamilyWithoutChangedFacts => {
            WorthUiHeaderFrameRebindDenial::RuntimeChange(denial)
        }
    }
}

pub(super) fn map_rebind_denial(
    denial: WorthUiProjectionRebindPlanDenial,
    capability_path: bool,
) -> WorthUiHeaderFrameRebindDenial {
    match denial {
        WorthUiProjectionRebindPlanDenial::RuntimeEvidenceMismatch => {
            WorthUiHeaderFrameRebindDenial::RuntimeEvidenceMismatch
        }
        WorthUiProjectionRebindPlanDenial::ReloadNotActivated if capability_path => {
            WorthUiHeaderFrameRebindDenial::CapabilityReloadNotActivated
        }
        WorthUiProjectionRebindPlanDenial::ReloadNotActivated => {
            WorthUiHeaderFrameRebindDenial::ReloadNotActivated
        }
    }
}

pub(super) fn header_status(
    posture: WorthUiRuntimeChangeActivationPosture,
    previous_frame_digest: u64,
    rebound_frame_digest: u64,
) -> WorthUiHeaderFrameRebindStatus {
    match posture {
        WorthUiRuntimeChangeActivationPosture::EquivalentNoOp => {
            WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload
        }
        WorthUiRuntimeChangeActivationPosture::Denied => {
            WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
        }
        WorthUiRuntimeChangeActivationPosture::Activated
        | WorthUiRuntimeChangeActivationPosture::Mixed(_) => {
            if previous_frame_digest == rebound_frame_digest {
                WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation
            } else {
                WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
            }
        }
        WorthUiRuntimeChangeActivationPosture::ReadyForFrameBoundary => {
            WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
        }
    }
}
