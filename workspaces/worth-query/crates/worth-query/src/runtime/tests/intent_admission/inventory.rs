use super::*;

#[test]
fn intent_admission_inventory_lists_current_runtime_floor() {
    let inventory = worth_query_intent_admission_coverage_inventory();
    let implemented = inventory
        .rows()
        .iter()
        .filter(|row| row.status() == WorthQueryIntentAdmissionCoverageStatus::Implemented)
        .map(|row| row.entrypoint())
        .collect::<Vec<_>>();

    assert!(implemented.contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent));
    assert!(implemented
        .contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent));
    assert!(implemented.contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite));
    assert!(implemented.contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite));
    assert!(implemented.contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily));
    assert!(implemented
        .contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext));
    assert!(implemented.contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead));
    assert!(implemented
        .contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization));
    assert!(
        implemented.contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection)
    );
    assert!(
        implemented.contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection)
    );
    assert!(implemented
        .contains(&WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting));
}

#[test]
fn family_inventory_freezes_current_read_common_path() {
    let family_inventory = worth_query_intent_admission_family_inventory();
    let read_row = family_inventory
        .rows()
        .iter()
        .find(|row| row.family() == WorthQueryIntentAdmissionFamily::ReadExecutionIntent)
        .expect("read family row should exist");

    assert_eq!(
        read_row.raw_authoring_constructor(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "WorthQueryRawIntentAdmissionRequest::read_family_entrypoint(...); WorthQueryRawIntentAdmissionRequest::read_family_in_basis_context_entrypoint(...); WorthQueryRawIntentAdmissionRequest::live_read_entrypoint(...)"
        )
    );
    assert_eq!(
        read_row.common_path_front_door(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "workspace.compose_read(declaration); workspace.execute_read_family(&family); workspace.execute_read_family_with_access_plan(&family, plan); workspace.execute_read_family_in_basis_context(&family, &context); workspace.execute_read_family_in_basis_context_with_access_plan(&family, &context, plan); workspace.read_family_intent(&family).execute(); workspace.read_family_in_basis_context_intent(&family, &context).execute(); workspace.read(&view); workspace.read_live_intent(&view).execute()"
        )
    );
    assert_eq!(
        read_row.advanced_path_front_door(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "workspace.read_family_intent(&family).review()?.admit()?.execute(); workspace.read_family_in_basis_context_intent(&family, &context).review()?.admit()?.execute(); workspace.read_live_intent(&view).review()?.admit()?.execute()"
        )
    );
}

#[test]
fn only_inspection_neighbor_remains_planned() {
    let inventory = worth_query_intent_admission_coverage_inventory();
    let planned = inventory
        .rows()
        .iter()
        .filter(|row| row.status() == WorthQueryIntentAdmissionCoverageStatus::PlannedNeighbor)
        .collect::<Vec<_>>();

    assert_eq!(planned.len(), 1);
    assert_eq!(
        planned[0].entrypoint(),
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred
    );
    assert_eq!(planned[0].execution_seam(), None);
}

#[test]
fn family_inventory_freezes_inspection_materialization_common_path() {
    let family_inventory = worth_query_intent_admission_family_inventory();
    let row = family_inventory
        .rows()
        .iter()
        .find(|row| {
            row.family() == WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent
        })
        .expect("inspection-materialization family row should exist");

    assert_eq!(
        row.raw_authoring_constructor(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "WorthQueryRawIntentAdmissionRequest::generic_inspection_entrypoint(...); WorthQueryRawIntentAdmissionRequest::derived_materialization_entrypoint(...); WorthQueryRawIntentAdmissionRequest::derived_inspection_entrypoint(...)"
        )
    );
    assert_eq!(
        row.common_path_front_door(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "workspace.materialize_result(&view)?; workspace.materialize_intent(&view).execute(); workspace.inspections()?.inspect(&target); workspace.inspections()?.inspect_intent(target).execute(); workspace.inspect_derived_intent(&view).execute()"
        )
    );
    assert_eq!(
        row.advanced_path_front_door(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "workspace.inspections()?.inspect_intent(target).review()?.admit()?.execute(); workspace.materialize_intent(&view).review()?.admit()?.execute(); workspace.inspect_derived_intent(&view).review()?.admit()?.execute()"
        )
    );
}

#[test]
fn family_inventory_freezes_authoritative_mutation_common_path() {
    let family_inventory = worth_query_intent_admission_family_inventory();
    let row = family_inventory
        .rows()
        .iter()
        .find(|row| row.family() == WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent)
        .expect("authoritative mutation family row should exist");

    assert_eq!(
        row.common_path_front_door(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "runtime.write(command); runtime.write_intent(command).execute(); runtime.write_batch(commands); runtime.write_batch_intent(commands).execute(); workspace.write_intent(command).execute(); workspace.write_batch_intent(commands).execute(); workspace.insert(collection, declaration); workspace.update(entity_identity, declaration); workspace.delete(entity_identity); workspace.delete_with(entity_identity, declaration); workspace.submissions()?.submit(command); workspace.submissions()?.submit_batch(commands)"
        )
    );
    assert_eq!(
        row.advanced_path_front_door(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "runtime.write_intent(command).review()?.admit()?.execute(); runtime.write_batch_intent(commands).review()?.admit()?.execute(); workspace.write_intent(command).review()?.admit()?.execute(); workspace.write_batch_intent(commands).review()?.admit()?.execute()"
        )
    );
}

#[test]
fn family_inventory_freezes_lower_runtime_routing_common_path() {
    let family_inventory = worth_query_intent_admission_family_inventory();
    let row = family_inventory
        .rows()
        .iter()
        .find(|row| {
            row.family() == WorthQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent
        })
        .expect("routing family row should exist");

    assert_eq!(
        row.raw_authoring_constructor(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "WorthQueryRawIntentAdmissionRequest::existing_truth_probe_entrypoint(...)"
        )
    );
    assert_eq!(
        row.common_path_front_door(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "runtime.probe_existing(request); runtime.probe_existing_intent(request).execute(); workspace.probe_existing_intent(request).execute()"
        )
    );
    assert_eq!(
        row.advanced_path_front_door(),
        WorthQueryIntentAdmissionSurfaceDescriptor::Available(
            "runtime.probe_existing_intent(request).review()?.admit()?.execute(); workspace.probe_existing_intent(request).review()?.admit()?.execute()"
        )
    );
}

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

#[test]
fn coverage_inventory_marks_projection_and_inspection_advisory_classes_as_exercised() {
    let inventory = worth_query_intent_admission_coverage_inventory();
    let projection = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint() == WorthQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption
        })
        .expect("projection row should exist");
    let inspection = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint() == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection
        })
        .expect("unified inspection row should exist");

    assert_eq!(
        projection.advisory_decision_class(),
        WorthQueryIntentAdmissionDecisionClass::ProjectionWarningBearingAdmission
    );
    assert_eq!(
        inspection.advisory_decision_class(),
        WorthQueryIntentAdmissionDecisionClass::InspectionDetailRedactionAdvisory
    );
}
