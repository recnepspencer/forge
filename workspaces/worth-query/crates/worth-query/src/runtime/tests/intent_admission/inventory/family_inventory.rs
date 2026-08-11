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
