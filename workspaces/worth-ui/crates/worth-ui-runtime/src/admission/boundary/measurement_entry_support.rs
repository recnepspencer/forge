use crate::admission::{
    UiAdmissionFamily, UiAdmissionTarget, UiAdmissionWorld, UiMeasurementAdmission,
    UiMeasurementAdmissionPosture, UiMeasurementUnsupportedReason, UiSupportPosture,
    UiSupportReason, UiSupportSnapshot,
};
use crate::declaration::UiDeclarationSupportMilestoneExpectation;
use crate::obligations::selection::UiObligationSupportSelectionPosture;
use crate::obligations::touch::{UiGraphTouchDescriptor, UiGraphTouchRuntimeLane};

pub(super) fn touch_has_measurement_lane(touch: &UiGraphTouchDescriptor) -> bool {
    touch
        .aspects()
        .iter()
        .any(|fact| fact.lane() == UiGraphTouchRuntimeLane::Measurement)
}

pub(super) fn selection_target_for_touch(
    touch: &UiGraphTouchDescriptor,
    target: &UiAdmissionTarget,
) -> UiAdmissionTarget {
    let mut selection_target = UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
    .with_selection_budget(target.selection_budget());

    if let Some(query_prerequisites) = target.query_prerequisites() {
        selection_target = selection_target.with_query_prerequisites(query_prerequisites.clone());
    }

    if let Some(host_capability_report) = target.host_capability_report() {
        selection_target =
            selection_target.with_host_capability_report(host_capability_report.clone());
    }

    selection_target
}

pub(super) fn support_posture_for_measurement_obligation(
    target: &UiAdmissionTarget,
    support_posture: UiObligationSupportSelectionPosture,
    expected_world_profile: &crate::graph::UiGraphWorldProfile,
) -> UiSupportPosture {
    match support_posture {
        UiObligationSupportSelectionPosture::Supported => UiSupportPosture::Supported {
            family: UiAdmissionFamily::MeasurementRequirement,
            world: target.world().clone(),
        },
        UiObligationSupportSelectionPosture::DiagnosticOnly => UiSupportPosture::DiagnosticOnly {
            family: UiAdmissionFamily::MeasurementRequirement,
            world: target.world().clone(),
        },
        UiObligationSupportSelectionPosture::Unsupported => UiSupportPosture::Unsupported {
            family: UiAdmissionFamily::MeasurementRequirement,
            reason: UiSupportReason::MissingDeclarationSupportEvidence,
            world: target.world().clone(),
        },
        UiObligationSupportSelectionPosture::Deferred => UiSupportPosture::Deferred {
            family: UiAdmissionFamily::MeasurementRequirement,
            expected_in: UiDeclarationSupportMilestoneExpectation::Milestone32,
            world: target.world().clone(),
        },
        UiObligationSupportSelectionPosture::WrongWorld => UiSupportPosture::WrongWorld {
            family: UiAdmissionFamily::MeasurementRequirement,
            expected: UiAdmissionWorld::from_graph_world_profile(expected_world_profile.clone()),
            observed: target.world().clone(),
        },
    }
}

pub(super) fn measurement_support_snapshot_from_admission(
    measurement_admission: Option<UiMeasurementAdmission>,
) -> Option<UiSupportSnapshot> {
    let admission = measurement_admission?;
    let target = admission.target().clone();
    let posture = match admission.posture() {
        UiMeasurementAdmissionPosture::Admitted { .. } => UiSupportPosture::Supported {
            family: UiAdmissionFamily::MeasurementRequirement,
            world: target.world().clone(),
        },
        UiMeasurementAdmissionPosture::Unsupported { reason, .. } => {
            UiSupportPosture::Unsupported {
                family: UiAdmissionFamily::MeasurementRequirement,
                reason: match reason {
                    UiMeasurementUnsupportedReason::Support(reason) => *reason,
                    UiMeasurementUnsupportedReason::SelectionDidNotYieldMeasurementRequirement => {
                        UiSupportReason::MissingDeclarationSupportEvidence
                    }
                },
                world: target.world().clone(),
            }
        }
        UiMeasurementAdmissionPosture::WrongWorld { expected, observed } => {
            UiSupportPosture::WrongWorld {
                family: UiAdmissionFamily::MeasurementRequirement,
                expected: expected.clone(),
                observed: observed.clone(),
            }
        }
        UiMeasurementAdmissionPosture::Deferred { expected_in, .. } => UiSupportPosture::Deferred {
            family: UiAdmissionFamily::MeasurementRequirement,
            expected_in: *expected_in,
            world: target.world().clone(),
        },
        UiMeasurementAdmissionPosture::DiagnosticOnly { .. } => UiSupportPosture::DiagnosticOnly {
            family: UiAdmissionFamily::MeasurementRequirement,
            world: target.world().clone(),
        },
        UiMeasurementAdmissionPosture::CapabilityGated { .. }
        | UiMeasurementAdmissionPosture::StaleSupportPosture { .. } => {
            UiSupportPosture::Unsupported {
                family: UiAdmissionFamily::MeasurementRequirement,
                reason: UiSupportReason::MissingDeclarationSupportEvidence,
                world: target.world().clone(),
            }
        }
    };
    Some(UiSupportSnapshot::new(target, posture))
}
