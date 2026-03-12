use crate::tests::support::*;

#[test]
fn chip_profile_emits_dense_patch_surface_details() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let _ = create_relation_in_partition(&mut runtime, left, right, "bridge", PartitionId(29));
    let publication = runtime.publication_access();
    let patch = publication.latest_patch().unwrap();

    assert_eq!(
        patch.compatibility,
        PatchCompatibilityClass::DenseCompatible
    );
    assert!(patch
        .records
        .iter()
        .all(|record| matches!(record.detail, PatchDetail::DenseBitset(_))));
}

#[test]
fn chip_profile_preserves_relation_traversal_with_compressed_adjacency_backend() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
    let source = create_entity_in_partition(&mut runtime, "source", PartitionId(7));
    let target_a = create_entity_in_partition(&mut runtime, "target-a", PartitionId(7));
    let target_b = create_entity_in_partition(&mut runtime, "target-b", PartitionId(9));
    let relation_a =
        create_relation_in_partition(&mut runtime, source, target_a, "r-a", PartitionId(7));
    let relation_b =
        create_relation_in_partition(&mut runtime, source, target_b, "r-b", PartitionId(12));
    let version_id = runtime.history_access().latest_commit().unwrap().version_id;

    assert_eq!(
        runtime.config().storage.adjacency_policy.backend,
        crate::facade::AdjacencyBackend::CompressedFanoutAdjacency
    );
    assert_eq!(
        runtime.outgoing_relations_for_entity(source, version_id),
        vec![relation_a, relation_b]
    );
    assert_eq!(
        runtime.incoming_relations_for_entity(target_b, version_id),
        vec![relation_b]
    );
}

#[test]
fn chip_profile_compiled_artifacts_are_derived_from_committed_truth() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let _ = create_relation_in_partition(&mut runtime, left, right, "bridge", PartitionId(29));
    let commit = runtime.history_access().latest_commit().unwrap().clone();

    let artifact = runtime
        .compile_execution_artifact(
            commit.commit_id,
            vec![PartitionId(7), PartitionId(11), PartitionId(29)],
        )
        .unwrap();

    assert_eq!(artifact.source_commit_id, commit.commit_id);
    assert_eq!(artifact.source_version_id, commit.version_id);
    assert_eq!(artifact.source_branch_id, BranchId("main".to_string()));
    assert_eq!(
        runtime.compiled_artifact_compatibility(artifact.artifact_id),
        crate::facade::CompiledArtifactCompatibility::Compatible
    );
}

#[test]
fn compiled_artifact_rejects_stale_topology_after_later_commit() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
    let original = create_entity_outcome(&mut runtime, "seed");
    let artifact = runtime
        .compile_execution_artifact(original.commit.commit_id, vec![PartitionId::main()])
        .unwrap();
    let _later = create_entity_outcome(&mut runtime, "later");

    assert_eq!(
        runtime.compiled_artifact_compatibility(artifact.artifact_id),
        crate::facade::CompiledArtifactCompatibility::StaleVersion
    );
}
