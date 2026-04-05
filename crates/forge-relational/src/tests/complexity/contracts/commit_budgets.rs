use crate::facade::history::BranchId;
use crate::facade::transactions::BulkRelationCreateIntent;
use crate::facade::transactions::CommitTopology;
use crate::tests::support::*;

fn relation_integrity_cardinality_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            Vec::new(),
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "source_max_one".into(),
                source_max: Some(1),
                source_min: None,
                target_max: None,
                target_min: None,
                pair_max: None,
                pair_min: None,
                pair_min_semantics:
                    crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                minimum_enforcement:
                    crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
            }],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

fn relation_integrity_uniqueness_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            Vec::new(),
            Vec::new(),
            vec![crate::schema::data::UniquenessContractDeclaration {
                contract_id: "uniq".into(),
                scope: crate::schema::data::UniquenessScope::NormalizedSymmetricEdge,
            }],
            Vec::new(),
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

fn relation_integrity_symmetry_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![crate::schema::data::SymmetryContractDeclaration {
                contract_id: "paired_twin".into(),
                mode: crate::schema::data::SymmetryMode::PairedTwinRequired,
            }],
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

fn relation_integrity_multi_contract_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            vec![crate::schema::data::EndpointKindContractDeclaration {
                contract_id: "kind".into(),
                allowed_source_kinds: vec![KindId(1)],
                allowed_target_kinds: vec![KindId(1)],
                self_edges_allowed: false,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
            }],
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "source_max_two".into(),
                source_max: Some(2),
                source_min: None,
                target_max: None,
                target_min: None,
                pair_max: None,
                pair_min: None,
                pair_min_semantics:
                    crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                minimum_enforcement:
                    crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
            }],
            vec![crate::schema::data::UniquenessContractDeclaration {
                contract_id: "uniq".into(),
                scope: crate::schema::data::UniquenessScope::NormalizedSymmetricEdge,
            }],
            Vec::new(),
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

fn relation_integrity_endpoint_deletion_runtime() -> RelationalRuntime {
    endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations,
        CascadeDeletePolicy::RetainDanglingForAudit,
    )
}

fn relation_integrity_minimum_certification_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            vec![crate::schema::data::EndpointKindContractDeclaration {
                contract_id: "endpoint".into(),
                allowed_source_kinds: vec![KindId(1)],
                allowed_target_kinds: vec![KindId(1)],
                self_edges_allowed: true,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
            }],
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "minimum".into(),
                source_max: None,
                source_min: Some(1),
                target_max: None,
                target_min: None,
                pair_max: None,
                pair_min: Some(2),
                pair_min_semantics:
                    crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                minimum_enforcement:
                    crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
            }],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

fn schema_transition_for_subscriber_impact(
    target_schema_version_id: SchemaVersionId,
    subscriber_impact: crate::schema::data::SchemaSubscriberImpact,
) -> crate::schema::data::ProposedSchemaTransition {
    crate::schema::data::ProposedSchemaTransition {
        source_schema_id: crate::schema::data::SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(target_schema_version_id.0 - 1),
        target_schema_id: crate::schema::data::SchemaId("test".to_string()),
        target_schema_version_id,
        diff_atoms: vec![crate::schema::data::SchemaDiffAtom::new(
            crate::schema::data::SchemaElementRef::new(
                crate::schema::data::SchemaElementKind::Field,
                crate::schema::data::SchemaId("test".to_string()),
                target_schema_version_id,
                Some(KindId(1)),
                "tag",
            ),
            vec![
                crate::schema::data::SchemaStratum::StructuralShape,
                crate::schema::data::SchemaStratum::PublicationContract,
            ],
            crate::schema::data::SchemaPublicationImpact::ObservableSurfaceChanged,
            subscriber_impact,
            crate::schema::data::HistoricalInterpretationSensitivity::NotSensitive,
            crate::schema::data::SchemaDiffDetail::AddedField {
                field_name: "tag".into(),
                required: false,
                default_expression: Some("null".into()),
            },
        )
        .with_boundary_visibility_proof(match subscriber_impact {
            crate::schema::data::SchemaSubscriberImpact::ConsumableSurfaceChanged => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable
            }
            crate::schema::data::SchemaSubscriberImpact::ContractUpgradeRequired => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleRequiresContractUptake
            }
            _ => crate::schema::data::SubscriberBoundaryVisibility::NotVisible,
        })],
    }
}

#[test]
fn complexity_contract_registry_covers_runtime_hot_paths() {
    let runtime = runtime_with_test_schema();
    let contracts = runtime.performance_access().contracts();

    assert!(contracts.len() >= 6);
    assert!(contracts
        .iter()
        .all(|contract| !contract.proof_tests.is_empty()));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.partition_local_commit"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.slot_local_mutation_journal"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.relation_identity_validation"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.unique_entity_invariant_lookup"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.current_state.clone"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.snapshot_pin_maintenance"));
}

#[test]
fn complexity_budget_partition_local_commit_reports_touched_partitions() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, left, "left-updated");
    let single_partition = runtime.performance_access().counters();
    assert_eq!(single_partition.partitions_touched_by_commit, 1);
    assert_eq!(single_partition.full_state_clones, 0);

    runtime.performance_access().reset_counters();
    let _ = create_relation_in_partition(&mut runtime, left, right, "cross", PartitionId(13));
    let cross_partition = runtime.performance_access().counters();
    assert_eq!(cross_partition.partitions_touched_by_commit, 3);
    assert_eq!(cross_partition.full_state_clones, 0);
}

#[test]
fn complexity_budget_commit_topology_inference_distinguishes_flat_and_graph_mutations() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, left, "left-updated");
    let flat = runtime.performance_access().counters();
    assert_eq!(
        flat.commit_topology_flags,
        CommitTopology::FlatEntityBatch.mask()
    );

    runtime.performance_access().reset_counters();
    let _ = create_relation_in_partition(&mut runtime, left, right, "cross", PartitionId(13));
    let graph = runtime.performance_access().counters();
    assert_eq!(
        graph.commit_topology_flags,
        CommitTopology::GraphMutation.mask()
    );
}

#[test]
fn complexity_budget_bulk_create_reserves_partition_local_capacity() {
    let mut runtime = runtime_with_test_schema();
    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("bulk-entities").push(MutationIntent::Create(
            CreateIntent::BulkEntities(BulkEntityCreateIntent {
                partition_id: PartitionId(41),
                kind_id: KindId(1),
                client_keys: vec![
                    InternedString::Raw("a".to_string()),
                    InternedString::Raw("b".to_string()),
                    InternedString::Raw("c".to_string()),
                ],
                payloads: vec![
                    RecordPayload::StructuredJson(json!({"name":"a"})),
                    RecordPayload::StructuredJson(json!({"name":"b"})),
                    RecordPayload::StructuredJson(json!({"name":"c"})),
                ],
            }),
        )),
    );
    let _ = txn.commit().unwrap();
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.bulk_entity_slots_reserved, 3);
    assert_eq!(counters.bulk_relation_slots_reserved, 0);
}

#[test]
fn complexity_budget_mutation_structural_invariants_are_touched_slot_bounded() {
    let mut runtime = runtime_with_test_schema();
    let target = create_entity(&mut runtime, "target");
    for index in 0..8 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, target, "target-updated");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.entity_slots_touched_by_commit, 1);
    assert_eq!(counters.relation_slots_touched_by_commit, 0);
    assert_eq!(counters.invariant_entity_slot_scans, 1);
}

#[test]
fn complexity_budget_relation_structural_invariants_are_touched_slot_bounded() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");

    runtime.performance_access().reset_counters();
    let _ = create_relation(&mut runtime, source, target, "r0");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.entity_slots_touched_by_commit, 0);
    assert_eq!(counters.relation_slots_touched_by_commit, 1);
    assert_eq!(counters.invariant_relation_slot_scans, 1);
}

#[test]
fn complexity_budget_relation_identity_validation_avoids_partition_scan() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let _existing = create_relation(&mut runtime, source, target, "existing");
    for index in 0..12 {
        let other_source = create_entity(&mut runtime, &format!("other-source-{index}"));
        let other_target = create_entity(&mut runtime, &format!("other-target-{index}"));
        let _ = create_relation(
            &mut runtime,
            other_source,
            other_target,
            &format!("r{index}"),
        );
    }

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("dup".to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
            },
        ))),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::DuplicateRelationIdentity
    ));
    assert_eq!(counters.relation_identity_candidates_scanned, 1);
}

#[test]
fn complexity_budget_unique_entity_invariant_uses_changed_set_lookup() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::mutation_sensitive_blocking(
            InvariantRule::UniqueEntityPayloadField("name".to_string()),
        )],
        ..InvariantCatalog::default()
    });
    let target = create_entity(&mut runtime, "target");
    let _other = create_entity(&mut runtime, "other");
    runtime.index_authority().rebuild_unique_field_indexes();

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-name").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: target,
                payload: RecordPayload::StructuredJson(json!({"name":"other"})),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::InvariantViolation
    ));
    assert_eq!(counters.invariant_entity_slot_scans, 1);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_budget_commit_boundary_unique_invariant_uses_merged_plan_lookup() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::UniqueEntityPayloadField("name".to_string()),
        )],
        ..InvariantCatalog::default()
    });
    let target = create_entity(&mut runtime, "target");
    let _other = create_entity(&mut runtime, "other");
    runtime.index_authority().rebuild_unique_field_indexes();

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-name").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: target,
                payload: RecordPayload::StructuredJson(json!({"name":"other"})),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::InvariantViolation
    ));
    assert_eq!(counters.invariant_entity_slot_scans, 1);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_contract_current_state_clone_is_declared_and_measured() {
    let mut runtime = runtime_with_test_schema();
    for index in 0..8 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }

    runtime.performance_access().reset_counters();
    let entity = create_entity(&mut runtime, "target");
    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, entity, "target-updated");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.full_state_clones, 0);
    assert_eq!(counters.partitions_cloned, 0);
    assert_eq!(counters.entity_slots_cloned, 0);
    assert_eq!(counters.relation_slots_cloned, 0);
}

#[test]
fn complexity_budget_preparation_packetization_is_chunked_for_broad_deltas() {
    let mut runtime = runtime_with_test_schema_execution_model(
        crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
    );
    runtime.performance_access().reset_counters();

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("bulk-entities").push(MutationIntent::Create(
            CreateIntent::BulkEntities(BulkEntityCreateIntent {
                partition_id: PartitionId(41),
                kind_id: KindId(1),
                client_keys: (0..65)
                    .map(|index| InternedString::Raw(format!("e{index}")))
                    .collect(),
                payloads: (0..65)
                    .map(|index| {
                        RecordPayload::StructuredJson(json!({"name": format!("e{index}")}))
                    })
                    .collect(),
            }),
        )),
    );
    let outcome = txn.commit().unwrap();
    let counters = runtime.performance_access().counters();

    assert_eq!(outcome.changed_records.len(), 65);
    assert!(counters.preparation_packet_count <= 8);
    assert!(counters.preparation_packet_item_count >= outcome.changed_records.len());
    assert!(counters.preparation_packet_peak_width_total >= 16);
    assert!(counters.preparation_scope_unit_count >= 1);
    assert!(counters.preparation_staged_parallel_strategy_count >= 1);
    assert!(counters.preparation_packet_count < outcome.changed_records.len());
}

#[test]
fn complexity_budget_preparation_narrow_delta_falls_back_to_serial() {
    let mut runtime = runtime_with_test_schema_execution_model(
        crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
    );
    runtime.performance_access().reset_counters();

    let _ = create_entity_outcome(&mut runtime, "narrow");
    let counters = runtime.performance_access().counters();

    assert!(counters.preparation_packet_count <= 3);
    assert!(counters.preparation_packet_item_count >= 1);
    assert!(counters.preparation_packet_peak_width_total >= 1);
    assert!(counters.preparation_scope_unit_count >= 1);
    assert!(counters.preparation_serial_strategy_count >= 1);
}

#[test]
fn complexity_budget_relation_integrity_skips_entity_only_mutation_work() {
    let mut runtime = relation_integrity_cardinality_runtime();
    let entity = create_entity(&mut runtime, "entity-only");

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, entity, "entity-only-updated");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.relation_integrity_contracts_evaluated, 0);
    assert_eq!(counters.relation_endpoint_kind_checks, 0);
    assert_eq!(counters.relation_cardinality_checks, 0);
    assert_eq!(counters.relation_uniqueness_checks, 0);
    assert_eq!(counters.relation_symmetry_checks, 0);
    assert_eq!(counters.relation_endpoint_deletion_checks, 0);
}

#[test]
fn complexity_budget_relation_integrity_uniqueness_uses_adjacency_local_candidates() {
    let mut runtime = relation_integrity_uniqueness_runtime();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let _existing = create_relation(&mut runtime, source, target, "existing");
    for index in 0..10 {
        let other_source = create_entity(&mut runtime, &format!("other-source-{index}"));
        let other_target = create_entity(&mut runtime, &format!("other-target-{index}"));
        let _ = create_relation(
            &mut runtime,
            other_source,
            other_target,
            &format!("other-rel-{index}"),
        );
    }

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new("duplicate-unique-relation").push(
        MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("duplicate".to_string()),
                source: target,
                target: source,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"duplicate"}))),
            },
        )),
    ));
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::RelationUniquenessViolation
    ));
    assert_eq!(counters.relation_integrity_contracts_evaluated, 1);
    assert_eq!(counters.relation_uniqueness_checks, 1);
    assert_eq!(counters.relation_uniqueness_candidates_scanned, 1);
}

#[test]
fn complexity_budget_relation_integrity_symmetry_checks_only_touched_pairs() {
    let mut runtime = relation_integrity_symmetry_runtime();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("missing-twin").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("missing-twin".to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(
                    json!({"label":"missing-twin"}),
                )),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::RelationSymmetryViolation
    ));
    assert_eq!(counters.relation_integrity_contracts_evaluated, 1);
    assert_eq!(counters.relation_symmetry_checks, 1);
    assert_eq!(counters.relation_uniqueness_candidates_scanned, 0);
}

#[test]
fn complexity_budget_relation_integrity_endpoint_deletion_checks_only_deleted_endpoints() {
    let mut runtime = relation_integrity_endpoint_deletion_runtime();
    let (source, _target, _relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete-source").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id: source }),
        )),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::RelationEndpointDeletionIntegrityViolation
    ));
    assert_eq!(counters.relation_integrity_contracts_evaluated, 1);
    assert_eq!(counters.relation_endpoint_deletion_checks, 1);
    assert_eq!(counters.relation_symmetry_checks, 0);
}

#[test]
fn complexity_budget_relation_integrity_reuses_touched_scope_across_multiple_contracts() {
    let mut runtime = relation_integrity_multi_contract_runtime();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let _existing = create_relation(&mut runtime, source, target, "existing");

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new("duplicate-and-missing-twin").push(
        MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("duplicate".to_string()),
                source: target,
                target: source,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"duplicate"}))),
            },
        )),
    ));
    let _error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.relation_integrity_contracts_evaluated, 3);
    assert_eq!(
        counters.relation_uniqueness_candidates_scanned,
        1,
        "touched live relation scope should be scanned once per relation kind, not once per contract"
    );
}

#[test]
fn complexity_budget_relation_integrity_minimum_certification_reports_snapshot_breadth() {
    let mut runtime = relation_integrity_minimum_certification_runtime();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    create_relation(&mut runtime, source, target, "single");

    runtime.performance_access().reset_counters();
    let result = runtime.validation().certification_state();
    let counters = runtime.performance_access().counters();

    assert!(result.summary().publication_failure().is_some());
    assert_eq!(
        counters.relation_cardinality_minimum_certification_contracts_evaluated,
        1
    );
    assert_eq!(
        counters.relation_cardinality_minimum_certification_relation_slot_scans,
        counters.invariant_relation_slot_scans
    );
    assert_eq!(
        counters.relation_cardinality_minimum_certification_entity_slot_scans,
        counters.invariant_entity_slot_scans
    );
    assert!(counters.relation_cardinality_minimum_certification_relation_slot_scans >= 1);
    assert!(counters.relation_cardinality_minimum_certification_entity_slot_scans >= 2);
    assert!(counters.relation_cardinality_checks >= 1);
}

#[test]
fn complexity_budget_schema_transition_classification_is_changed_atom_bounded() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "anchor");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        schema_transition_for_subscriber_impact(
            SchemaVersionId(2),
            crate::schema::data::SchemaSubscriberImpact::ConsumableSurfaceChanged,
        ),
        Some(crate::schema::data::SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.schema_transition_atoms_inspected, 1);
    assert_eq!(counters.schema_changed_subtrees_inspected, 1);
    assert_eq!(counters.schema_unchanged_subtrees_reused_by_fingerprint, 0);
    assert_eq!(counters.schema_bridge_descriptors_built, 1);
    assert_eq!(counters.schema_transition_continue_visible_bridge_count, 1);
    assert_eq!(counters.schema_reconciliation_preserve_information_count, 1);
}

#[test]
fn complexity_budget_subscriber_resume_continuity_is_boundary_local() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "anchor");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        schema_transition_for_subscriber_impact(
            SchemaVersionId(2),
            crate::schema::data::SchemaSubscriberImpact::ConsumableSurfaceChanged,
        ),
        Some(crate::schema::data::SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();

    runtime.performance_access().reset_counters();
    let _ = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(10))
        .unwrap();
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.subscriber_resume_evaluations, 1);
    assert_eq!(counters.subscriber_continue_visible_bridge_count, 1);
    assert_eq!(counters.schema_normalized_descriptor_compositions, 1);
}

#[test]
fn complexity_budget_replay_verification_tracks_digest_and_deep_layers_separately() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "replayable");

    runtime.performance_access().reset_counters();
    let normal =
        runtime
            .replay_authority()
            .replay_commit(crate::replay::data::RelationalReplayRequest {
                commit_id: created.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: crate::replay::data::ReplayExecutionMode::SerialDeterministic,
                verification_mode:
                    crate::replay::data::ReplayVerificationMode::NormalRecoveryVerification,
            });
    assert!(runtime.replay().compare_outcome(&normal));
    let normal_counters = runtime.performance_access().counters();
    assert!(normal_counters.replay_digest_parity_checks > 0);
    assert_eq!(normal_counters.replay_deep_artifact_parity_checks, 0);

    runtime
        .history_authority()
        .tamper_commit_patch_for_test(created.commit.commit_id, |patch| {
            patch.records.clear();
        });

    runtime.performance_access().reset_counters();
    let audited =
        runtime
            .replay_authority()
            .replay_commit(crate::replay::data::RelationalReplayRequest {
                commit_id: created.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: crate::replay::data::ReplayExecutionMode::SerialDeterministic,
                verification_mode:
                    crate::replay::data::ReplayVerificationMode::AuditRecoveryVerification,
            });
    assert_eq!(
        audited.failure,
        Some(crate::replay::data::ReplayFailureClass::ObservableMismatch)
    );
    let audit_counters = runtime.performance_access().counters();
    assert!(audit_counters.replay_digest_parity_checks > 0);
    assert!(audit_counters.replay_deep_artifact_parity_checks > 0);
}

#[test]
fn complexity_budget_milestone5_closeout_keeps_schema_cdc_and_recovery_boundary_local() {
    let mut runtime = persisted_runtime_with_test_schema();
    let baseline = create_entity_outcome(&mut runtime, "anchor");
    let baseline_checkpoint =
        checkpoint_for_schema_version(baseline.patch_position(), SchemaVersionId(1));

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        schema_transition_for_subscriber_impact(
            SchemaVersionId(2),
            crate::schema::data::SchemaSubscriberImpact::ConsumableSurfaceChanged,
        ),
        Some(crate::schema::data::SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("after-boundary"));
    let transitioned = txn.commit().unwrap();
    let schema_counters = runtime.performance_access().counters();

    assert_eq!(schema_counters.schema_transition_atoms_inspected, 1);
    assert_eq!(schema_counters.schema_changed_subtrees_inspected, 1);
    assert_eq!(schema_counters.schema_bridge_descriptors_built, 1);
    assert_eq!(
        schema_counters.schema_transition_continue_visible_bridge_count,
        1
    );
    assert_eq!(schema_counters.replay_digest_parity_checks, 0);
    assert_eq!(schema_counters.replay_deep_artifact_parity_checks, 0);

    runtime.performance_access().reset_counters();
    let _batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            baseline_checkpoint.clone(),
            32,
        ))
        .unwrap();
    let cdc_counters = runtime.performance_access().counters();

    assert_eq!(cdc_counters.schema_transition_atoms_inspected, 0);
    assert_eq!(cdc_counters.subscriber_resume_evaluations, 1);
    assert_eq!(cdc_counters.subscriber_continue_visible_bridge_count, 1);
    assert_eq!(cdc_counters.schema_normalized_descriptor_compositions, 1);
    assert_eq!(cdc_counters.replay_digest_parity_checks, 0);
    assert_eq!(cdc_counters.replay_deep_artifact_parity_checks, 0);

    runtime.performance_access().reset_counters();
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let plan_counters = runtime.performance_access().counters();

    assert!(plan_counters.replay_digest_parity_checks >= 1);
    assert_eq!(plan_counters.replay_deep_artifact_parity_checks, 0);
    assert_eq!(
        plan.compatibility.verification_outcome,
        crate::durability::data::RecoveryVerificationOutcome::VerifiedAtLayer(
            crate::replay::data::ReplayVerificationLayer::DigestParity
        )
    );

    let mut recovered = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(
            AspectSchemaFixture {
                schema_version_id: SchemaVersionId(2),
                ..AspectSchemaFixture::default()
            }
            .build_registry(),
        )
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: unique_test_store_path("forge-relational-m5-performance-closeout"),
            segment_commit_capacity: 2,
        })
        .build();
    let _ = recovered.durability_authority().recover(plan).unwrap();
    let recovered_counters = recovered.performance_access().counters();

    assert!(recovered_counters.replay_digest_parity_checks >= 1);
    assert_eq!(recovered_counters.replay_deep_artifact_parity_checks, 0);
    assert!(recovered
        .replay()
        .canonical_commit_envelope(transitioned.commit.commit_id)
        .is_some());
}

#[test]
fn complexity_budget_bulk_mutation_planning_reports_identity_scope_and_batch_evidence() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity_in_partition(&mut runtime, "source", PartitionId(7));
    let target = create_entity_in_partition(&mut runtime, "target", PartitionId(11));

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("entities").push(MutationIntent::Create(
            CreateIntent::BulkEntities(BulkEntityCreateIntent {
                partition_id: PartitionId(7),
                kind_id: KindId(1),
                client_keys: vec![
                    InternedString::Raw("alpha".to_string()),
                    InternedString::Raw("beta".to_string()),
                ],
                payloads: vec![
                    RecordPayload::StructuredJson(json!({"name":"alpha"})),
                    RecordPayload::StructuredJson(json!({"name":"beta"})),
                ],
            }),
        )),
    );
    txn.push_batch(
        WorkerIntentBatch::new("relations").push(MutationIntent::Create(
            CreateIntent::BulkRelations(BulkRelationCreateIntent {
                partition_id: PartitionId(13),
                kind_id: KindId(2),
                client_keys: vec![InternedString::Raw("edge".to_string())],
                endpoints: vec![(source, target)],
                payloads: vec![Some(RecordPayload::StructuredJson(json!({"label":"edge"})))],
            }),
        )),
    );

    let plan = txn.plan_bulk_mutation_batch().expect("planned batch");
    let counters = runtime.performance_access().counters();

    assert_eq!(plan.locality.entity_target_count, 2);
    assert_eq!(plan.locality.relation_target_count, 1);
    assert_eq!(plan.locality.cross_partition_relation_count, 1);
    assert_eq!(plan.naming.normalized_client_keys.len(), 3);
    assert_eq!(plan.lineage.transitions.len(), 3);
    assert_eq!(plan.provenance.worker_batch_names.len(), 2);
    assert_eq!(counters.bulk_mutation_batch_count, 0);
    assert_eq!(counters.bulk_mutation_naming_normalization_count, 0);
    assert_eq!(counters.bulk_mutation_lineage_transition_count, 0);
    assert_eq!(counters.bulk_mutation_provenance_record_count, 0);
}

#[test]
fn complexity_budget_bulk_mutation_admission_remains_side_effect_free_until_commit() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("relation-batch").push(MutationIntent::Create(
            CreateIntent::BulkRelations(BulkRelationCreateIntent {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_keys: vec![InternedString::Raw("edge".to_string())],
                endpoints: vec![(source, target)],
                payloads: vec![Some(RecordPayload::StructuredJson(json!({"label":"edge"})))],
            }),
        )),
    );

    let admitted = txn
        .admit_provenance_complete_bulk_mutation_batch()
        .expect("admission should succeed");
    let preflight_counters = runtime.performance_access().counters();

    assert!(admitted.is_some());
    assert_eq!(preflight_counters.bulk_mutation_batch_count, 0);
    assert_eq!(
        preflight_counters.bulk_mutation_naming_normalization_count,
        0
    );
    assert_eq!(preflight_counters.bulk_mutation_lineage_transition_count, 0);
    assert_eq!(preflight_counters.bulk_mutation_provenance_record_count, 0);

    let mut commit_txn = runtime.begin_transaction(TransactionOptions::default());
    commit_txn.push_batch(
        WorkerIntentBatch::new("relation-batch").push(MutationIntent::Create(
            CreateIntent::BulkRelations(BulkRelationCreateIntent {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_keys: vec![InternedString::Raw("edge-commit".to_string())],
                endpoints: vec![(source, target)],
                payloads: vec![Some(RecordPayload::StructuredJson(json!({"label":"edge"})))],
            }),
        )),
    );
    let _ = commit_txn.commit().expect("commit should succeed");
    let committed_counters = runtime.performance_access().counters();

    assert_eq!(committed_counters.bulk_mutation_batch_count, 1);
    assert_eq!(committed_counters.bulk_mutation_relation_target_count, 1);
    assert_eq!(
        committed_counters.bulk_mutation_naming_normalization_count,
        1
    );
    assert_eq!(committed_counters.bulk_mutation_lineage_transition_count, 1);
    assert_eq!(committed_counters.bulk_mutation_provenance_record_count, 1);
}
