use worth_ui_runtime::facade::mounted::{
    UiMountedInspectionReceipt, UiMountedInspectionRelation, UiMountedInspectionRequest,
    UiMountedRetentionClass, UiMountedRetentionClassBudget, UiMountedRetentionEvictionPosture,
    UiMountedRetentionReport,
};

use crate::mounted_application_lifecycle::published_mounted_world::PresentedObservationBasis;

use super::RetentionPressureWorld;

pub(super) fn assert_bounded_retention_truth(
    world: &RetentionPressureWorld,
    current: PresentedObservationBasis,
) {
    let report = world.session.mounted_retention_report();
    let reported_classes = report
        .classes()
        .iter()
        .map(|row| row.class())
        .collect::<Vec<_>>();
    assert_eq!(
        reported_classes,
        vec![
            UiMountedRetentionClass::Current,
            UiMountedRetentionClass::InFlight,
            UiMountedRetentionClass::ObservationBasis,
            UiMountedRetentionClass::PredecessorInspection,
            UiMountedRetentionClass::Diagnostic,
            UiMountedRetentionClass::Quarantine,
            UiMountedRetentionClass::VisualSnapshot,
            UiMountedRetentionClass::VisualOverlay,
        ]
    );
    assert_evidence_class_within_budget(&report, UiMountedRetentionClass::Current);
    assert_evidence_class_within_budget(&report, UiMountedRetentionClass::InFlight);
    assert_evidence_class_within_budget(&report, UiMountedRetentionClass::ObservationBasis);
    assert_evidence_class_within_budget(&report, UiMountedRetentionClass::PredecessorInspection);
    let observation = report.class(UiMountedRetentionClass::ObservationBasis);
    assert_eq!(observation.retained_items(), 2);
    assert_eq!(observation.active_leases(), 1);
    assert!(observation.retained_structural_bytes() > 0);
    let observation_queue = observation
        .queue_budget()
        .expect("observation retention exposes its independent queue budget");
    assert_eq!(observation_queue.item_limit(), 2);
    assert!(observation.retained_structural_bytes() <= observation_queue.structural_byte_limit());
    assert_non_evidence_retention_truth(&report);
    assert_current_is_inspectable(&world.session, current);
}

fn assert_non_evidence_retention_truth(report: &UiMountedRetentionReport) {
    assert_diagnostic_retention_truth(report);
    assert_quarantine_retention_truth(report);
    assert_visual_retention_truth(report);
}

fn assert_diagnostic_retention_truth(report: &UiMountedRetentionReport) {
    let diagnostic = report.class(UiMountedRetentionClass::Diagnostic);
    assert_eq!(
        diagnostic.posture(),
        UiMountedRetentionEvictionPosture::OmittedByPolicy
    );
    assert_eq!(diagnostic.retained_items(), 0);
    assert_eq!(diagnostic.retained_structural_bytes(), 0);
    assert_eq!(
        diagnostic.evidence_budget(),
        Some(UiMountedRetentionClassBudget::new(0, 0))
    );
}

fn assert_quarantine_retention_truth(report: &UiMountedRetentionReport) {
    let quarantine = report.class(UiMountedRetentionClass::Quarantine);
    assert_eq!(
        quarantine.posture(),
        UiMountedRetentionEvictionPosture::AdmissionBounded
    );
    assert_eq!(quarantine.retained_items(), 1);
    assert!(quarantine.retained_structural_bytes() > 0);
    let quarantine_queue = quarantine
        .queue_budget()
        .expect("quarantine retention exposes its independent queue budget");
    assert_eq!(quarantine_queue.item_limit(), quarantine.retained_items());
    assert!(quarantine.retained_structural_bytes() <= quarantine_queue.structural_byte_limit());
}

fn assert_visual_retention_truth(report: &UiMountedRetentionReport) {
    let snapshot = report.class(UiMountedRetentionClass::VisualSnapshot);
    assert_eq!(
        snapshot.posture(),
        UiMountedRetentionEvictionPosture::LeaseProtected
    );
    let overlay = report.class(UiMountedRetentionClass::VisualOverlay);
    assert_eq!(
        overlay.posture(),
        UiMountedRetentionEvictionPosture::LeaseProtected
    );
    assert_eq!(snapshot.retained_items(), 0);
    assert_eq!(snapshot.retained_structural_bytes(), 0);
    assert_eq!(
        snapshot.evidence_budget(),
        Some(UiMountedRetentionClassBudget::new(0, 0))
    );
    assert_eq!(overlay.retained_items(), 0);
    assert_eq!(overlay.retained_structural_bytes(), 0);
    assert_eq!(
        overlay.evidence_budget(),
        Some(UiMountedRetentionClassBudget::new(0, 0))
    );
}

fn assert_evidence_class_within_budget(
    report: &UiMountedRetentionReport,
    class: UiMountedRetentionClass,
) {
    let row = report.class(class);
    let budget = row
        .evidence_budget()
        .expect("frame evidence classes expose their exact budget");
    assert!(row.retained_items() <= budget.frame_limit());
    assert!(row.retained_structural_bytes() <= budget.structural_byte_limit());
    assert!(row.active_leases() <= budget.frame_limit());
    assert!(row.lease_charged_structural_bytes() <= budget.structural_byte_limit());
}

pub(super) fn assert_current_is_inspectable(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    expected: PresentedObservationBasis,
) {
    match session.inspect_mounted_frame(
        UiMountedInspectionRequest::current().for_instance(expected.instance),
    ) {
        UiMountedInspectionReceipt::Available(inspection) => {
            assert_eq!(inspection.frame(), expected.frame);
            assert_eq!(inspection.relation(), UiMountedInspectionRelation::Current);
            assert_eq!(inspection.selected_node_receipt(), Some(expected.receipt));
        }
        other => panic!("the current retained frame must stay interpretable: {other:?}"),
    }
}

pub(super) fn assert_current_frame_is_inspectable(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    expected: worth_ui_runtime::facade::mounted::UiMountedFrameIdentity,
) {
    match session.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(inspection) => {
            assert_eq!(inspection.frame(), expected);
            assert_eq!(inspection.relation(), UiMountedInspectionRelation::Current);
        }
        other => panic!("the settled current frame must be inspectable: {other:?}"),
    }
}
