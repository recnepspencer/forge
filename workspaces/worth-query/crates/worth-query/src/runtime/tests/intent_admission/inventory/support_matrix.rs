use super::*;

#[test]
fn support_matrix_matches_inventory_for_implemented_rows() {
    let inventory = worth_query_intent_admission_coverage_inventory();
    let support_matrix = worth_query_intent_admission_support_matrix();

    assert_eq!(inventory.rows().len(), support_matrix.rows().len());

    for (coverage, support) in inventory.rows().iter().zip(support_matrix.rows().iter()) {
        assert_eq!(coverage.family(), support.family());
        assert_eq!(coverage.entrypoint(), support.entrypoint());
        assert_eq!(coverage.execution_boundary(), support.execution_boundary());

        match coverage.status() {
            WorthQueryIntentAdmissionCoverageStatus::Implemented => {
                assert_eq!(
                    support.posture(),
                    WorthQueryIntentAdmissionSupportPosture::Admitted
                );
            }
            WorthQueryIntentAdmissionCoverageStatus::PlannedNeighbor => {
                assert_eq!(
                    support.posture(),
                    WorthQueryIntentAdmissionSupportPosture::Deferred
                );
            }
        }
    }
}

#[test]
fn coverage_inventory_carries_read_execution_metadata_as_typed_fields() {
    let inventory = worth_query_intent_admission_coverage_inventory();
    let read_current = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint() == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily
        })
        .expect("read family row should exist");
    let read_basis = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint()
                == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext
        })
        .expect("basis-context read row should exist");
    let live_read = inventory
        .rows()
        .iter()
        .find(|row| row.entrypoint() == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead)
        .expect("live read row should exist");

    for row in [read_current, read_basis] {
        assert_eq!(
            row.eligibility_authority(),
            WorthQueryIntentAdmissionEligibilityAuthority::ReadCompositionExecutionAuthority
        );
        assert_eq!(
            row.admitted_plan_kind(),
            WorthQueryIntentAdmissionPlanKind::ReadExecutionPlan
        );
        assert_eq!(
            row.admitted_execution_handoff(),
            WorthQueryIntentAdmissionExecutionHandoffInventory::Available(
                "WorthQueryReadExecutionHandoff"
            )
        );
        assert_eq!(
            row.result_artifact(),
            WorthQueryIntentAdmissionResultArtifact::WorthQueryReadResult
        );
        assert_eq!(
            row.execution_boundary(),
            WorthQueryIntentAdmissionExecutionBoundary::CoveredSeam(
                crate::facade::runtime::WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute
            )
        );
        assert_eq!(
            row.advisory_decision_class(),
            WorthQueryIntentAdmissionDecisionClass::AdvisoryNotYetExercisedOnCoveredEntrypoint
        );
        assert_eq!(
            row.violation_decision_class(),
            WorthQueryIntentAdmissionDecisionClass::AdmissionOrExecutionViolation
        );
    }

    assert_eq!(
        live_read.eligibility_authority(),
        WorthQueryIntentAdmissionEligibilityAuthority::ReadCompositionExecutionAuthority
    );
    assert_eq!(
        live_read.admitted_plan_kind(),
        WorthQueryIntentAdmissionPlanKind::ReadExecutionPlan
    );
    assert_eq!(
        live_read.admitted_execution_handoff(),
        WorthQueryIntentAdmissionExecutionHandoffInventory::Available(
            "WorthQueryLiveReadExecutionHandoff"
        )
    );
    assert_eq!(
        live_read.result_artifact(),
        WorthQueryIntentAdmissionResultArtifact::WorthQueryLiveReadResult
    );
    assert_eq!(
        live_read.execution_boundary(),
        WorthQueryIntentAdmissionExecutionBoundary::CoveredSeam(
            crate::facade::runtime::WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute
        )
    );
}

#[test]
fn support_matrix_marks_read_rows_as_implemented_floor() {
    let support = worth_query_intent_admission_support_matrix();
    let read_rows = support
        .rows()
        .iter()
        .filter(|row| row.family() == WorthQueryIntentAdmissionFamily::ReadExecutionIntent)
        .collect::<Vec<_>>();

    assert_eq!(read_rows.len(), 3);
    for row in read_rows.iter().copied().filter(|row| {
        row.entrypoint() != WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead
    }) {
        assert_eq!(
            row.posture(),
            WorthQueryIntentAdmissionSupportPosture::Admitted
        );
        assert_eq!(
            row.detail(),
            WorthQueryIntentAdmissionSupportDetail::ImplementedReadExecutionFloor
        );
    }
    let live_row = read_rows
        .iter()
        .find(|row| row.entrypoint() == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead)
        .expect("live read support row should exist");
    assert_eq!(
        live_row.posture(),
        WorthQueryIntentAdmissionSupportPosture::Admitted
    );
    assert_eq!(
        live_row.detail(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedLiveReadExecutionFloor
    );
}

#[test]
fn support_matrix_marks_inspection_materialization_rows_as_implemented_floor() {
    let support = worth_query_intent_admission_support_matrix();
    let implemented_rows = support
        .rows()
        .iter()
        .filter(|row| {
            row.family() == WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent
                && row.posture() == WorthQueryIntentAdmissionSupportPosture::Admitted
        })
        .collect::<Vec<_>>();

    assert_eq!(implemented_rows.len(), 3);
    let materialize = implemented_rows
        .iter()
        .find(|row| {
            row.entrypoint()
                == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization
        })
        .expect("derived materialization row should exist");
    let inspect = implemented_rows
        .iter()
        .find(|row| {
            row.entrypoint() == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection
        })
        .expect("derived inspection row should exist");
    let unified = implemented_rows
        .iter()
        .find(|row| {
            row.entrypoint() == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection
        })
        .expect("unified inspection row should exist");

    assert_eq!(
        materialize.detail(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedDerivedMaterializationFloor
    );
    assert_eq!(
        inspect.detail(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedDerivedInspectionFloor
    );
    assert_eq!(
        unified.detail(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedUnifiedInspectionFloor
    );
}

#[test]
fn support_matrix_marks_existing_truth_probe_routing_as_implemented_floor() {
    let support = worth_query_intent_admission_support_matrix();
    let row = support
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint()
                == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting
        })
        .expect("probe routing support row should exist");

    assert_eq!(
        row.family(),
        WorthQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent
    );
    assert_eq!(
        row.posture(),
        WorthQueryIntentAdmissionSupportPosture::Admitted
    );
    assert_eq!(
        row.detail(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedExistingTruthProbeRoutingFloor
    );
}
