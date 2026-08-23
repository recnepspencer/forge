use super::*;

#[test]
fn index_query_keeps_the_admitted_root_after_same_and_sibling_reference_movement() {
    let mut runtime = runtime_with_index_field_aspects();
    let alpha = create_entity_outcome(&mut runtime, "alpha");
    let alpha_id = changed_entities(&alpha)[0];
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.exact-observation".to_owned(),
        kind: DerivedIndexKind::EntityField {
            field_locator: aspect_field_locator(aspect_key("name"), field_key("name")),
        },
        branch_scoped: true,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: alpha.commit.commit_id,
            branch_id: BranchId("main".to_owned()),
            index_ids: vec![index.index_id],
        });
    let snapshot = snapshot_for_owner_branch(&mut runtime, &BranchId("main".to_owned()));

    runtime
        .history_authority()
        .fork_branch_from(
            BranchId("index-sibling".to_owned()),
            &BranchId("main".to_owned()),
        )
        .expect("sibling begins at the observed root");
    update_entity(&mut runtime, alpha_id, "main-new");
    create_entity_outcome_on_branch(
        &mut runtime,
        "sibling-only",
        BranchId("index-sibling".to_owned()),
    );

    runtime.performance_access().reset_counters();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("admitted snapshot retains its exact planning context");
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "exact-observation-index".to_owned(),
        context_id: context,
        scope: crate::facade::query::QueryScope::EntityFieldEquals {
            field_locator: aspect_field_locator(aspect_key("name"), field_key("name")),
            value: string_aspect_value("alpha"),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        access_contract: QueryAccessContract::DerivedIndexWithStorageParity,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(9_170_601),
        target_count_hint: 1,
    };
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("exact observation plans after both references move");
    let authoritative = runtime
        .read_truth()
        .execute_query_plan(plan.clone())
        .expect("authoritative exact-root query succeeds");
    let indexed = runtime
        .index_access()
        .execute_query_plan_with_index_parity(plan, IndexParityMode::CertificationParity)
        .expect("indexed exact-root query succeeds");

    assert!(build.failed_indexes.is_empty());
    assert_eq!(indexed.execution.result, authoritative.result);
    assert_eq!(indexed.execution.result.entities.len(), 1);
    assert_eq!(indexed.execution.result.entities[0].entity_id, alpha_id);
    assert_eq!(
        indexed.access_path,
        QueryAccessPath::DerivedIndexGeneration {
            generation_id: build.generations[0].generation_id,
        }
    );
    let counters = runtime.performance_access().counters();
    assert_eq!(counters.visibility_cache_miss_reconstructions, 0);
}
