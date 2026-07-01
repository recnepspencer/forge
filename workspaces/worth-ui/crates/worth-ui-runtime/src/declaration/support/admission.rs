use crate::declaration::{
    UiDeclaredPostureAdmission, UiDeclaredPostureApplicability, UiDeclaredPostureContract,
};

use super::support_row_schema::DECLARATION_SUPPORT_ROW_SCHEMA;
use super::{
    UiDeclarationSupportMilestoneExpectation, UiDeclarationSupportRow,
    UiDeclarationSupportRowSchemaKind, UiDeclarationSupportSnapshot,
    UiDeclarationSupportSnapshotAdmission, UiDeclarationSupportSnapshotAdmissionDenial,
    UiDeclarationUnsupportedPosture,
};

pub(crate) fn admit_declaration_support_snapshot(
    declared_posture_admission: &UiDeclaredPostureAdmission,
) -> UiDeclarationSupportSnapshotAdmission {
    let posture = match declared_posture_admission.admitted_contract() {
        Ok(posture) => posture,
        Err(denial) => {
            return UiDeclarationSupportSnapshotAdmission::Denied(
                UiDeclarationSupportSnapshotAdmissionDenial::DeclaredPostureNotAdmitted {
                    denial: denial.clone(),
                },
            );
        }
    };

    let rows = DECLARATION_SUPPORT_ROW_SCHEMA.map(|schema| match schema.kind {
        UiDeclarationSupportRowSchemaKind::QueryBinding => support_row_for_query_binding(posture),
        UiDeclarationSupportRowSchemaKind::ServiceUsage => support_row_for_service_usage(posture),
        UiDeclarationSupportRowSchemaKind::TouchMeaning => support_row_for_touch_meaning(posture),
        UiDeclarationSupportRowSchemaKind::MeasurementPolicy => {
            support_row_for_measurement_policy(posture)
        }
        UiDeclarationSupportRowSchemaKind::HostCapability => {
            support_row_for_host_capability(posture)
        }
    });

    UiDeclarationSupportSnapshotAdmission::Admitted(UiDeclarationSupportSnapshot::new(rows))
}

fn support_row_for_query_binding(posture: &UiDeclaredPostureContract) -> UiDeclarationSupportRow {
    let lane = posture.query_binding();
    let unsupported_posture = unsupported_posture_for(lane.applicability());
    match lane.admitted().copied() {
        Some(admitted) => UiDeclarationSupportRow::with_query_binding(
            UiDeclarationSupportRowSchemaKind::QueryBinding,
            lane.applicability(),
            admitted,
            unsupported_posture,
        ),
        None => UiDeclarationSupportRow::without_admitted_fact(
            UiDeclarationSupportRowSchemaKind::QueryBinding,
            lane.applicability(),
            unsupported_posture,
        ),
    }
}

fn support_row_for_service_usage(posture: &UiDeclaredPostureContract) -> UiDeclarationSupportRow {
    let lane = posture.service_usage();
    let unsupported_posture = unsupported_posture_for(lane.applicability());
    match lane.admitted().copied() {
        Some(admitted) => UiDeclarationSupportRow::with_service_usage(
            UiDeclarationSupportRowSchemaKind::ServiceUsage,
            lane.applicability(),
            admitted,
            unsupported_posture,
        ),
        None => UiDeclarationSupportRow::without_admitted_fact(
            UiDeclarationSupportRowSchemaKind::ServiceUsage,
            lane.applicability(),
            unsupported_posture,
        ),
    }
}

fn support_row_for_touch_meaning(posture: &UiDeclaredPostureContract) -> UiDeclarationSupportRow {
    let lane = posture.touch_meaning();
    let unsupported_posture = unsupported_posture_for(lane.applicability());
    match lane.admitted().copied() {
        Some(admitted) => UiDeclarationSupportRow::with_touch_meaning(
            UiDeclarationSupportRowSchemaKind::TouchMeaning,
            lane.applicability(),
            admitted,
            unsupported_posture,
        ),
        None => UiDeclarationSupportRow::without_admitted_fact(
            UiDeclarationSupportRowSchemaKind::TouchMeaning,
            lane.applicability(),
            unsupported_posture,
        ),
    }
}

fn support_row_for_measurement_policy(
    posture: &UiDeclaredPostureContract,
) -> UiDeclarationSupportRow {
    let lane = posture.measurement_policy();
    let unsupported_posture = unsupported_posture_for(lane.applicability());
    match lane.admitted().copied() {
        Some(admitted) => UiDeclarationSupportRow::with_measurement_policy(
            UiDeclarationSupportRowSchemaKind::MeasurementPolicy,
            lane.applicability(),
            admitted,
            unsupported_posture,
        ),
        None => UiDeclarationSupportRow::without_admitted_fact(
            UiDeclarationSupportRowSchemaKind::MeasurementPolicy,
            lane.applicability(),
            unsupported_posture,
        ),
    }
}

fn support_row_for_host_capability(posture: &UiDeclaredPostureContract) -> UiDeclarationSupportRow {
    let lane = posture.host_capability();
    let unsupported_posture = unsupported_posture_for(lane.applicability());
    match lane.admitted().cloned() {
        Some(admitted) => UiDeclarationSupportRow::with_host_capability(
            UiDeclarationSupportRowSchemaKind::HostCapability,
            lane.applicability(),
            admitted,
            unsupported_posture,
        ),
        None => UiDeclarationSupportRow::without_admitted_fact(
            UiDeclarationSupportRowSchemaKind::HostCapability,
            lane.applicability(),
            unsupported_posture,
        ),
    }
}

const fn unsupported_posture_for(
    applicability: UiDeclaredPostureApplicability,
) -> Option<UiDeclarationUnsupportedPosture> {
    match applicability {
        UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted => Some(
            UiDeclarationUnsupportedPosture::ArchitecturallyOwnedButNotYetAdmitted {
                expected_in: UiDeclarationSupportMilestoneExpectation::Milestone32,
            },
        ),
        UiDeclaredPostureApplicability::Required
        | UiDeclaredPostureApplicability::Optional
        | UiDeclaredPostureApplicability::NotApplicable => None,
    }
}
