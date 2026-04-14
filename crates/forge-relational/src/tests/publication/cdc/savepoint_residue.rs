use crate::facade::lineage::LineageDecisionKind;
use crate::facade::storage::RecordLifecycleState;
use crate::tests::support::*;

#[test]
fn savepoint_abandoned_work_never_appears_in_subscriber_cdc() {
    let mut runtime = runtime_with_test_schema();
    let anchor = create_entity_outcome(&mut runtime, "anchor");
    let anchor_entity = changed_entities(&anchor)[0];
    let checkpoint = checkpoint_for_schema_version(anchor.patch_position(), SchemaVersionId(1));

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("surviving"));
    let savepoint = txn.create_savepoint();
    txn.push_batch(batch_create("abandoned"));
    txn.push_batch(
        WorkerIntentBatch::new("abandoned-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor_entity,
                payload: RecordPayload::StructuredJson(json!({"name":"abandoned-anchor"})),
            }),
        )),
    );
    let rollback = txn.rollback_to_savepoint(savepoint).unwrap();
    txn.push_batch(
        WorkerIntentBatch::new("survived-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor_entity,
                payload: RecordPayload::StructuredJson(json!({"name":"survived-anchor"})),
            }),
        )),
    );
    let outcome = txn.commit().unwrap();

    assert!(rollback.summary().has_discarded_entity_creation());

    assert_subscriber_stream_omits_detail(&runtime, checkpoint, "abandoned");

    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let names = read
        .entities()
        .iter()
        .filter_map(|record| read_entity_name(record))
        .collect::<Vec<_>>();

    assert!(names.contains(&"surviving"));
    assert!(names.contains(&"survived-anchor"));
    assert!(!names.contains(&"abandoned"));
    assert!(!names.contains(&"abandoned-anchor"));
}

#[test]
fn nested_savepoint_abandoned_aspect_work_leaves_zero_patch_cdc_history_and_lineage_residue() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "anchor");
    let anchor = changed_entities(&created)[0];
    let target = create_entity(&mut runtime, "target");
    let start_lineage = runtime
        .lineage_access()
        .for_record(anchor)
        .unwrap()
        .lineage_id;
    let checkpoint = checkpoint_for_schema_version(
        runtime.publication().latest_patch().unwrap().position,
        SchemaVersionId(1),
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    let savepoint_a = txn.create_savepoint();
    txn.push_batch(batch_create("surviving-a"));
    txn.push_batch(
        WorkerIntentBatch::new("surviving-a-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor,
                payload: RecordPayload::StructuredJson(json!({"name":"surviving-a-anchor"})),
            }),
        )),
    );

    let savepoint_b = txn.create_savepoint();
    txn.push_batch(batch_create("abandoned-entity"));
    txn.push_batch(
        WorkerIntentBatch::new("abandoned-relation").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("abandoned-r".to_string()),
                source: crate::transactions::data::EntityReference::Existing(anchor),
                target: crate::transactions::data::EntityReference::Existing(target),
                payload: Some(RecordPayload::StructuredJson(
                    json!({"label":"abandoned-label"}),
                )),
            }),
        )),
    );
    txn.push_batch(
        WorkerIntentBatch::new("abandoned-replace").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: anchor,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: InternedString::Raw("abandoned-replacement".to_string()),
                    payload: RecordPayload::StructuredJson(json!({"name":"abandoned-replacement"})),
                },
            }),
        )),
    );
    let rollback_b = txn.rollback_to_savepoint(savepoint_b).unwrap();

    txn.push_batch(batch_create("surviving-b"));
    txn.push_batch(
        WorkerIntentBatch::new("surviving-b-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor,
                payload: RecordPayload::StructuredJson(json!({"name":"surviving-b-anchor"})),
            }),
        )),
    );
    let rollback_a = txn.rollback_to_savepoint(savepoint_a).unwrap();

    txn.push_batch(batch_create("surviving-final"));
    txn.push_batch(
        WorkerIntentBatch::new("surviving-final-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor,
                payload: RecordPayload::StructuredJson(json!({"name":"surviving-final-anchor"})),
            }),
        )),
    );
    txn.push_batch(WorkerIntentBatch::new("surviving-final-relation").push(
        MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("surviving-r".to_string()),
                source: crate::transactions::data::EntityReference::Existing(anchor),
                target: crate::transactions::data::EntityReference::Existing(target),
                payload: Some(RecordPayload::StructuredJson(
                    json!({"label":"surviving-label"}),
                )),
            },
        )),
    ));
    let outcome = txn.commit().unwrap();

    assert!(rollback_b.has_effects());
    assert!(rollback_a.has_effects());
    let _ = assert_patch_truth_invariants(&outcome);
    assert_patch_omits_detail(&outcome, "abandoned");

    assert_subscriber_stream_omits_detail(&runtime, checkpoint, "abandoned");

    let direct_history =
        runtime
            .history()
            .entity_aspect_history(&BranchId("main".to_string()), anchor, None);
    let direct_traced = runtime.history().entity_aspect_history_with_trace(
        &BranchId("main".to_string()),
        anchor,
        None,
    );
    assert_eq!(direct_history.len(), 2);
    assert_eq!(direct_traced.aspect_history_digest().entry_count, 2);
    assert_direct_history_origin_invariants(&direct_history, RecordRef::Entity(anchor));

    let lineage_traced = runtime.lineage_access().entity_aspect_history_with_trace(
        crate::facade::lineage::HistoricalResolutionRequest {
            branch_id: BranchId("main".to_string()),
            lineage_id: start_lineage,
            boundedness_basis:
                crate::facade::lineage::HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
        },
        None,
    );
    let lineage_history = lineage_traced
        .history
        .as_ref()
        .expect("lineage aspect history");
    assert_eq!(lineage_history.traversed_event_ids.len(), 0);
    assert_eq!(lineage_history.entries.len(), 2);
    assert_lineage_history_origin_invariants(&lineage_history.entries, start_lineage);
    assert_eq!(
        lineage_traced
            .lineage_aspect_resolution_digest()
            .traversed_lineage_events,
        0
    );

    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let entity_names = read
        .entities()
        .iter()
        .filter_map(|record| read_entity_name(record))
        .collect::<Vec<_>>();

    assert!(entity_names.contains(&"target"));
    assert!(entity_names.contains(&"surviving-final"));
    assert!(entity_names.contains(&"surviving-final-anchor"));
    assert!(!entity_names.iter().any(|name| name.contains("abandoned")));
    assert_eq!(read.relations().len(), 1);
    let replay = runtime.replay();
    let envelope = replay
        .canonical_commit_envelope(outcome.commit.commit_id)
        .unwrap();
    assert!(!envelope
        .lineage_decision_log()
        .iter()
        .any(|decision| decision.kind == LineageDecisionKind::ReplaceAccepted));
}

#[test]
fn rolled_back_illegal_relation_work_leaves_zero_cdc_and_diagnostic_residue() {
    let schema = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
                    vec![crate::schema::data::EndpointKindContractDeclaration {
                        contract_id: "no_self".into(),
                        allowed_source_kinds: vec![KindId(1)],
                        allowed_target_kinds: vec![KindId(1)],
                        self_edges_allowed: false,
                        cross_context_policy: CrossContextPolicy::AllowExplicit,
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            })
        })
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(schema)
        .build();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let checkpoint = checkpoint_for_schema_version(
        runtime.publication().latest_patch().unwrap().position,
        SchemaVersionId(1),
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    let savepoint = txn.create_savepoint();
    txn.push_batch(
        WorkerIntentBatch::new("illegal-self-edge").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("illegal".to_string()),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(source),
                payload: Some(RecordPayload::StructuredJson(json!({"label":"illegal"}))),
            }),
        )),
    );
    let rollback = txn.rollback_to_savepoint(savepoint).unwrap();
    txn.push_batch(
        WorkerIntentBatch::new("surviving-edge").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("surviving".to_string()),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target),
                payload: Some(RecordPayload::StructuredJson(json!({"label":"surviving"}))),
            }),
        )),
    );
    let outcome = txn.commit().unwrap();

    assert!(rollback.has_effects());
    assert_patch_omits_detail(&outcome, "illegal");
    assert_subscriber_stream_omits_detail(&runtime, checkpoint, "illegal");

    assert!(!runtime
        .publication()
        .diagnostics()
        .artifacts()
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::RelationEndpointKindViolation));
}

#[test]
fn rolled_back_endpoint_deletion_work_leaves_zero_cdc_and_diagnostic_residue() {
    let schema = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![crate::schema::data::EndpointDeletionIntegrityDeclaration {
                        contract_id: "require_retirement".into(),
                        mode: crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
                    }],
                ),
            })
        })
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(schema)
        .cascade_delete_policy(CascadeDeletePolicy::RetainDanglingForAudit)
        .build();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "live");
    let relation = changed_relations(&relation_outcome)[0];
    let checkpoint = checkpoint_for_schema_version(
        runtime.publication().latest_patch().unwrap().position,
        SchemaVersionId(1),
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    let savepoint = txn.create_savepoint();
    txn.push_batch(WorkerIntentBatch::new("rolled-back-delete-source").push(
        MutationIntent::Entity(EntityMutationIntent::Delete(DeleteEntityIntent {
            entity_id: source,
        })),
    ));
    let rollback = txn.rollback_to_savepoint(savepoint).unwrap();
    txn.push_batch(
        WorkerIntentBatch::new("surviving-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: target,
                payload: RecordPayload::StructuredJson(json!({"name":"target-survived"})),
            }),
        )),
    );
    let outcome = txn.commit().unwrap();

    assert!(rollback.has_effects());
    assert_patch_omits_detail(&outcome, "RetainedDanglingForAudit");
    assert_subscriber_stream_omits_detail(&runtime, checkpoint, "RetainedDanglingForAudit");

    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let relation = read.get_relation(relation).unwrap();
    assert_eq!(relation.lifecycle, RecordLifecycleState::Live);
    assert!(!runtime
        .publication()
        .diagnostics()
        .artifacts()
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::RelationEndpointDeletionIntegrityViolation));
}

