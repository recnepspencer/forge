use super::*;

#[test]
fn intent_admission_inventory_lists_current_runtime_floor() {
    let inventory = forge_query_intent_admission_coverage_inventory();
    let implemented = inventory
        .rows()
        .iter()
        .filter(|row| row.status() == ForgeQueryIntentAdmissionCoverageStatus::Implemented)
        .map(|row| row.entrypoint())
        .collect::<Vec<_>>();

    assert!(implemented.contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent));
    assert!(implemented
        .contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent));
    assert!(implemented.contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite));
    assert!(implemented.contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite));
    assert!(implemented.contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily));
    assert!(implemented
        .contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext));
    assert!(implemented.contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead));
    assert!(implemented
        .contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization));
    assert!(
        implemented.contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection)
    );
    assert!(
        implemented.contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection)
    );
    assert!(implemented
        .contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting));
}

#[test]
fn family_inventory_freezes_current_read_common_path() {
    let family_inventory = forge_query_intent_admission_family_inventory();
    let read_row = family_inventory
        .rows()
        .iter()
        .find(|row| row.family() == ForgeQueryIntentAdmissionFamily::ReadExecutionIntent)
        .expect("read family row should exist");

    assert_eq!(
        read_row.raw_authoring_constructor(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "ForgeQueryRawIntentAdmissionRequest::read_family_entrypoint(...); ForgeQueryRawIntentAdmissionRequest::live_read_entrypoint(...)"
        )
    );
    assert_eq!(
        read_row.common_path_front_door(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "workspace.read_family_intent(&family).execute(); workspace.read_live_intent(&view).execute()"
        )
    );
    assert_eq!(
        read_row.advanced_path_front_door(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "workspace.read_family_intent(&family).review()?.admit()?.execute(); workspace.read_live_intent(&view).review()?.admit()?.execute()"
        )
    );
}

#[test]
fn only_inspection_neighbor_remains_planned() {
    let inventory = forge_query_intent_admission_coverage_inventory();
    let planned = inventory
        .rows()
        .iter()
        .filter(|row| row.status() == ForgeQueryIntentAdmissionCoverageStatus::PlannedNeighbor)
        .collect::<Vec<_>>();

    assert_eq!(planned.len(), 1);
    assert_eq!(
        planned[0].entrypoint(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred
    );
    assert_eq!(planned[0].execution_seam(), None);
}

#[test]
fn family_inventory_freezes_inspection_materialization_common_path() {
    let family_inventory = forge_query_intent_admission_family_inventory();
    let row = family_inventory
        .rows()
        .iter()
        .find(|row| {
            row.family() == ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
        })
        .expect("inspection-materialization family row should exist");

    assert_eq!(
        row.raw_authoring_constructor(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "ForgeQueryRawIntentAdmissionRequest::generic_inspection_entrypoint(...); ForgeQueryRawIntentAdmissionRequest::derived_materialization_entrypoint(...); ForgeQueryRawIntentAdmissionRequest::derived_inspection_entrypoint(...)"
        )
    );
    assert_eq!(
        row.common_path_front_door(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "workspace.inspect_intent(target).execute(); workspace.materialize_intent(&view).execute(); workspace.inspect_derived_intent(&view).execute()"
        )
    );
    assert_eq!(
        row.advanced_path_front_door(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "workspace.inspect_intent(target).review()?.admit()?.execute(); workspace.materialize_intent(&view).review()?.admit()?.execute(); workspace.inspect_derived_intent(&view).review()?.admit()?.execute()"
        )
    );
}

#[test]
fn family_inventory_freezes_authoritative_mutation_common_path() {
    let family_inventory = forge_query_intent_admission_family_inventory();
    let row = family_inventory
        .rows()
        .iter()
        .find(|row| row.family() == ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent)
        .expect("authoritative mutation family row should exist");

    assert_eq!(
        row.common_path_front_door(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "runtime.write_intent(command).execute(); workspace.write_intent(command).execute()"
        )
    );
    assert_eq!(
        row.advanced_path_front_door(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "runtime.write_intent(command).review()?.admit()?.execute(); workspace.write_intent(command).review()?.admit()?.execute()"
        )
    );
}

#[test]
fn family_inventory_freezes_lower_runtime_routing_common_path() {
    let family_inventory = forge_query_intent_admission_family_inventory();
    let row = family_inventory
        .rows()
        .iter()
        .find(|row| {
            row.family() == ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent
        })
        .expect("routing family row should exist");

    assert_eq!(
        row.raw_authoring_constructor(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "ForgeQueryRawIntentAdmissionRequest::existing_truth_probe_entrypoint(...)"
        )
    );
    assert_eq!(
        row.common_path_front_door(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "runtime.probe_existing_intent(request).execute(); workspace.probe_existing_intent(request).execute()"
        )
    );
    assert_eq!(
        row.advanced_path_front_door(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "runtime.probe_existing_intent(request).review()?.admit()?.execute(); workspace.probe_existing_intent(request).review()?.admit()?.execute()"
        )
    );
}

#[test]
fn support_matrix_matches_inventory_for_implemented_rows() {
    let inventory = forge_query_intent_admission_coverage_inventory();
    let support_matrix = forge_query_intent_admission_support_matrix();

    assert_eq!(inventory.rows().len(), support_matrix.rows().len());

    for (coverage, support) in inventory.rows().iter().zip(support_matrix.rows().iter()) {
        assert_eq!(coverage.family(), support.family());
        assert_eq!(coverage.entrypoint(), support.entrypoint());
        assert_eq!(coverage.execution_boundary(), support.execution_boundary());

        match coverage.status() {
            ForgeQueryIntentAdmissionCoverageStatus::Implemented => {
                assert_eq!(
                    support.posture(),
                    ForgeQueryIntentAdmissionSupportPosture::Admitted
                );
            }
            ForgeQueryIntentAdmissionCoverageStatus::PlannedNeighbor => {
                assert_eq!(
                    support.posture(),
                    ForgeQueryIntentAdmissionSupportPosture::Deferred
                );
            }
        }
    }
}

#[test]
fn coverage_inventory_carries_read_execution_metadata_as_typed_fields() {
    let inventory = forge_query_intent_admission_coverage_inventory();
    let read_current = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint() == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily
        })
        .expect("read family row should exist");
    let read_basis = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint()
                == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext
        })
        .expect("basis-context read row should exist");
    let live_read = inventory
        .rows()
        .iter()
        .find(|row| row.entrypoint() == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead)
        .expect("live read row should exist");

    for row in [read_current, read_basis] {
        assert_eq!(
            row.eligibility_authority(),
            ForgeQueryIntentAdmissionEligibilityAuthority::ReadCompositionExecutionAuthority
        );
        assert_eq!(
            row.admitted_plan_kind(),
            ForgeQueryIntentAdmissionPlanKind::ReadExecutionPlan
        );
        assert_eq!(
            row.admitted_execution_handoff(),
            ForgeQueryIntentAdmissionExecutionHandoffInventory::Available(
                "ForgeQueryReadExecutionHandoff"
            )
        );
        assert_eq!(
            row.result_artifact(),
            ForgeQueryIntentAdmissionResultArtifact::ForgeQueryReadResult
        );
        assert_eq!(
            row.execution_boundary(),
            ForgeQueryIntentAdmissionExecutionBoundary::CoveredSeam(
                crate::facade::runtime::ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute
            )
        );
        assert_eq!(
            row.advisory_decision_class(),
            ForgeQueryIntentAdmissionDecisionClass::AdvisoryNotYetExercisedOnCoveredEntrypoint
        );
        assert_eq!(
            row.violation_decision_class(),
            ForgeQueryIntentAdmissionDecisionClass::AdmissionOrExecutionViolation
        );
    }

    assert_eq!(
        live_read.eligibility_authority(),
        ForgeQueryIntentAdmissionEligibilityAuthority::ReadCompositionExecutionAuthority
    );
    assert_eq!(
        live_read.admitted_plan_kind(),
        ForgeQueryIntentAdmissionPlanKind::ReadExecutionPlan
    );
    assert_eq!(
        live_read.admitted_execution_handoff(),
        ForgeQueryIntentAdmissionExecutionHandoffInventory::Available(
            "ForgeQueryLiveReadExecutionHandoff"
        )
    );
    assert_eq!(
        live_read.result_artifact(),
        ForgeQueryIntentAdmissionResultArtifact::ForgeQueryLiveReadResult
    );
    assert_eq!(
        live_read.execution_boundary(),
        ForgeQueryIntentAdmissionExecutionBoundary::CoveredSeam(
            crate::facade::runtime::ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute
        )
    );
}

#[test]
fn support_matrix_marks_read_rows_as_implemented_floor() {
    let support = forge_query_intent_admission_support_matrix();
    let read_rows = support
        .rows()
        .iter()
        .filter(|row| row.family() == ForgeQueryIntentAdmissionFamily::ReadExecutionIntent)
        .collect::<Vec<_>>();

    assert_eq!(read_rows.len(), 3);
    for row in read_rows.iter().copied().filter(|row| {
        row.entrypoint() != ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead
    }) {
        assert_eq!(
            row.posture(),
            ForgeQueryIntentAdmissionSupportPosture::Admitted
        );
        assert_eq!(
            row.detail(),
            ForgeQueryIntentAdmissionSupportDetail::ImplementedReadExecutionFloor
        );
    }
    let live_row = read_rows
        .iter()
        .find(|row| row.entrypoint() == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead)
        .expect("live read support row should exist");
    assert_eq!(
        live_row.posture(),
        ForgeQueryIntentAdmissionSupportPosture::Admitted
    );
    assert_eq!(
        live_row.detail(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedLiveReadExecutionFloor
    );
}

#[test]
fn support_matrix_marks_inspection_materialization_rows_as_implemented_floor() {
    let support = forge_query_intent_admission_support_matrix();
    let implemented_rows = support
        .rows()
        .iter()
        .filter(|row| {
            row.family() == ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
                && row.posture() == ForgeQueryIntentAdmissionSupportPosture::Admitted
        })
        .collect::<Vec<_>>();

    assert_eq!(implemented_rows.len(), 3);
    let materialize = implemented_rows
        .iter()
        .find(|row| {
            row.entrypoint()
                == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization
        })
        .expect("derived materialization row should exist");
    let inspect = implemented_rows
        .iter()
        .find(|row| {
            row.entrypoint() == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection
        })
        .expect("derived inspection row should exist");
    let unified = implemented_rows
        .iter()
        .find(|row| {
            row.entrypoint() == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection
        })
        .expect("unified inspection row should exist");

    assert_eq!(
        materialize.detail(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedDerivedMaterializationFloor
    );
    assert_eq!(
        inspect.detail(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedDerivedInspectionFloor
    );
    assert_eq!(
        unified.detail(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedUnifiedInspectionFloor
    );
}

#[test]
fn support_matrix_marks_existing_truth_probe_routing_as_implemented_floor() {
    let support = forge_query_intent_admission_support_matrix();
    let row = support
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint()
                == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting
        })
        .expect("probe routing support row should exist");

    assert_eq!(
        row.family(),
        ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent
    );
    assert_eq!(
        row.posture(),
        ForgeQueryIntentAdmissionSupportPosture::Admitted
    );
    assert_eq!(
        row.detail(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedExistingTruthProbeRoutingFloor
    );
}
