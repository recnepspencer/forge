use crate::facade::storage::RecordLifecycleState;
use crate::tests::support::*;

fn source_max_one_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
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

fn publication_source_min_one_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
            vec![crate::schema::data::EndpointKindContractDeclaration {
                contract_id: "endpoint_domain".into(),
                allowed_source_kinds: vec![KindId(1)],
                allowed_target_kinds: vec![KindId(1)],
                self_edges_allowed: true,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
            }],
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "source_min_one".into(),
                source_max: None,
                source_min: Some(1),
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

fn publication_pair_min_two_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
            vec![crate::schema::data::EndpointKindContractDeclaration {
                contract_id: "endpoint_domain".into(),
                allowed_source_kinds: vec![KindId(1)],
                allowed_target_kinds: vec![KindId(1)],
                self_edges_allowed: true,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
            }],
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "pair_min_two".into(),
                source_max: None,
                source_min: None,
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

fn certification_authority_source_min_one_runtime() -> RelationalRuntime {
    publication_source_min_one_runtime()
}

#[test]
fn relation_integrity_commit_boundary_rejects_forbidden_self_edge() {
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
    let entity = create_entity(&mut runtime, "self");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("self".to_string()),
                source: entity,
                target: entity,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"self"}))),
            },
        ))),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationEndpointKindViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_rejects_source_cardinality_overflow() {
    let mut runtime = source_max_one_runtime();
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");

    create_relation(&mut runtime, source, target_a, "a");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("b".to_string()),
                source,
                target: target_b,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"b"}))),
            },
        ))),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationCardinalityViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_certification_boundary_rejects_zero_edge_entity_for_minimum_cardinality() {
    let mut runtime = publication_source_min_one_runtime();
    let _orphan = create_entity(&mut runtime, "orphan");

    let result = runtime.invariant_access().certification_state();
    let failure = result
        .summary()
        .publication_failure()
        .expect("certification minimum cardinality failure");

    assert_eq!(failure.code(), DiagnosticCode::RelationCardinalityViolation);
    let fields = failure.fields();
    assert_eq!(fields["contract_id"], json!("source_min_one"));
    assert_eq!(fields["relation_kind_id"], json!(2));
    assert_eq!(fields["boundary"], json!("source"));
    assert_eq!(fields["count"], json!(0));
    assert_eq!(fields["limit"], json!(1));
    assert_eq!(
        result.metadata().execution_point().diagnostic_label(),
        "certification_boundary"
    );
}

#[test]
fn relation_integrity_certification_boundary_rejects_observed_pair_below_parallel_minimum() {
    let mut runtime = publication_pair_min_two_runtime();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    create_relation(&mut runtime, source, target, "single");

    let result = runtime.invariant_access().certification_state();
    let failure = result
        .summary()
        .publication_failure()
        .expect("certification pair minimum failure");

    assert_eq!(failure.code(), DiagnosticCode::RelationCardinalityViolation);
    let fields = failure.fields();
    assert_eq!(fields["contract_id"], json!("pair_min_two"));
    assert_eq!(fields["relation_kind_id"], json!(2));
    assert_eq!(fields["source"], json!(source));
    assert_eq!(fields["target"], json!(target));
    assert_eq!(fields["count"], json!(1));
    assert_eq!(fields["limit"], json!(2));
}

#[test]
fn relation_integrity_certification_boundary_is_authority_owned_and_blocks_publication() {
    let mut runtime = certification_authority_source_min_one_runtime();
    let _orphan = create_entity(&mut runtime, "orphan");

    let error = runtime
        .certify_current_state()
        .expect_err("certification boundary should block incomplete topology");

    assert_eq!(error.stage, PublicationStage::InvariantCheck);
    assert!(error.detail.contains("source_min_one"));
}

#[test]
fn relation_integrity_rejected_branch_local_commit_does_not_advance_truth_or_leak_to_main() {
    let mut runtime = source_max_one_runtime();
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");
    let accepted = create_relation_outcome(&mut runtime, source, target_a, "accepted");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let main_head_before = runtime
        .history_access()
        .branch_head(&BranchId("main".to_string()))
        .cloned();
    let feature_head_before = runtime
        .history_access()
        .branch_head(&BranchId("feature".to_string()))
        .cloned();
    let main_digest_before = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("main".to_string()),
        changed_relations(&accepted)[0],
        None,
    );
    let feature_digest_before = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("feature".to_string()),
        changed_relations(&accepted)[0],
        None,
    );
    let latest_patch_before = runtime
        .publication_access()
        .latest_patch()
        .unwrap()
        .position;

    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(BranchId("feature".to_string())),
        ..TransactionOptions::default()
    });
    txn.push_batch(WorkerIntentBatch::new("illegal-feature-relation").push(
        MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("illegal-feature".to_string()),
                source,
                target: target_b,
                payload: Some(RecordPayload::StructuredJson(
                    json!({"label":"illegal-feature"}),
                )),
            },
        )),
    ));

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationCardinalityViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }

    assert_eq!(
        runtime
            .history_access()
            .branch_head(&BranchId("main".to_string())),
        main_head_before.as_ref()
    );
    assert_eq!(
        runtime
            .history_access()
            .branch_head(&BranchId("feature".to_string())),
        feature_head_before.as_ref()
    );
    assert_eq!(
        relation_aspect_history_digest_on_branch(
            &runtime,
            &BranchId("main".to_string()),
            changed_relations(&accepted)[0],
            None,
        ),
        main_digest_before
    );
    assert_eq!(
        relation_aspect_history_digest_on_branch(
            &runtime,
            &BranchId("feature".to_string()),
            changed_relations(&accepted)[0],
            None,
        ),
        feature_digest_before
    );
    assert_eq!(
        runtime
            .publication_access()
            .latest_patch()
            .unwrap()
            .position,
        latest_patch_before
    );
    assert_eq!(
        runtime.publication_access().latest_bundle().unwrap().commit,
        accepted.commit
    );
}

#[test]
fn relation_integrity_commit_boundary_rejects_duplicate_normalized_symmetric_edge() {
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
                    Vec::new(),
                    Vec::new(),
                    vec![crate::schema::data::UniquenessContractDeclaration {
                        contract_id: "uniq_norm".into(),
                        scope: crate::schema::data::UniquenessScope::NormalizedSymmetricEdge,
                    }],
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

    create_relation(&mut runtime, source, target, "forward");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-normalized").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("reverse".to_string()),
                source: target,
                target: source,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"reverse"}))),
            }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationUniquenessViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_requires_paired_inverse_edge() {
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
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![crate::schema::data::SymmetryContractDeclaration {
                        contract_id: "paired_inverse".into(),
                        mode: crate::schema::data::SymmetryMode::PairedInverseRequired,
                    }],
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

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("one-way").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("one-way".to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"one-way"}))),
            },
        ))),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationSymmetryViolation);
            let fields = error.fields().expect("symmetry localization fields");
            assert_eq!(fields["contract_id"], json!("paired_inverse"));
            assert_eq!(fields["relation_kind_id"], json!(2));
            assert_eq!(fields["source"], json!(source));
            assert_eq!(fields["target"], json!(target));
            assert_eq!(fields["mode"], json!("paired"));
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_requires_canonical_undirected_ordering() {
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
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![crate::schema::data::SymmetryContractDeclaration {
                        contract_id: "canonical_undirected".into(),
                        mode: crate::schema::data::SymmetryMode::CanonicalUndirected,
                    }],
                    Vec::new(),
                ),
            })
        })
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(schema)
        .build();
    let first = create_entity(&mut runtime, "first");
    let second = create_entity(&mut runtime, "second");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("reverse-undirected").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("reverse-undirected".to_string()),
                source: second,
                target: first,
                payload: Some(RecordPayload::StructuredJson(
                    json!({"label":"reverse-undirected"}),
                )),
            }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationSymmetryViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_prohibits_inverse_duplication() {
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
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![crate::schema::data::SymmetryContractDeclaration {
                        contract_id: "inverse_prohibited".into(),
                        mode: crate::schema::data::SymmetryMode::InverseProhibited,
                    }],
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

    create_relation(&mut runtime, source, target, "forward");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("inverse").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("inverse".to_string()),
                source: target,
                target: source,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"inverse"}))),
            },
        ))),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationSymmetryViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_requires_paired_twin_edge() {
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
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![crate::schema::data::SymmetryContractDeclaration {
                        contract_id: "paired_twin".into(),
                        mode: crate::schema::data::SymmetryMode::PairedTwinRequired,
                    }],
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
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationSymmetryViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_rejects_endpoint_delete_with_live_relations() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, _target, _relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete-source").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id: source }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(
                error.code(),
                DiagnosticCode::RelationEndpointDeletionIntegrityViolation
            );
            let fields = error
                .fields()
                .expect("endpoint deletion localization fields");
            assert_eq!(fields["contract_id"], json!("endpoint_delete"));
            assert_eq!(fields["relation_kind_id"], json!(2));
            assert_eq!(fields["entity_id"], json!(source));
            assert_eq!(fields["mode"], json!("reject_delete_with_live_relations"));
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_reports_contract_counters_on_success() {
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
                    vec![crate::schema::data::CardinalityContractDeclaration {
                        contract_id: "source_max_two".into(),
                        source_max: Some(2),
                        source_min: None,
                        target_max: None,
                        target_min: None,
                        pair_max: None,
                        pair_min: None,
                        pair_min_semantics: crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                        minimum_enforcement:
                            crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
                    }],
                    vec![crate::schema::data::UniquenessContractDeclaration {
                        contract_id: "uniq".into(),
                        scope: crate::schema::data::UniquenessScope::DirectedSemanticEdge,
                    }],
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

    let result = create_relation_outcome(&mut runtime, source, target, "guarded");

    assert!(
        result
            .complexity_delta()
            .relation_integrity_contracts_evaluated
            >= 3
    );
    assert!(result.complexity_delta().relation_endpoint_kind_checks >= 1);
    assert!(result.complexity_delta().relation_cardinality_checks >= 1);
    assert!(result.complexity_delta().relation_uniqueness_checks >= 1);
}

#[test]
fn relation_integrity_commit_boundary_rejects_replace_when_retained_relation_keeps_live_endpoint_dependency(
) {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, _target, _relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("replace-source").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: source,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: InternedString::Raw("source-replacement".to_string()),
                    payload: RecordPayload::StructuredJson(json!({"name":"source-replacement"})),
                },
            }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(
                error.code(),
                DiagnosticCode::RelationEndpointDeletionIntegrityViolation
            );
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_requires_relation_deletion_in_same_commit_under_retain_policy(
) {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, _target, _relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete-source").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id: source }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(
                error.code(),
                DiagnosticCode::RelationEndpointDeletionIntegrityViolation
            );
            assert!(error
                .detail()
                .contains("requires deleting dependent relations in the same commit"));
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_allows_relation_deletion_in_same_commit_under_cascade_policy()
{
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit,
        CascadeDeletePolicy::CascadeDeleteRelations,
    );
    let (source, _target, relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let deleted = delete_entity(&mut runtime, source);
    let read = runtime
        .visibility_reads()
        .read_snapshot(&deleted.snapshot)
        .unwrap();

    assert!(read.get_relation(relation).is_none());
}

#[test]
fn relation_integrity_commit_boundary_allows_relation_retirement_when_policy_retains_for_audit() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, _target, relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let deleted = delete_entity(&mut runtime, source);
    let read = runtime
        .visibility_reads()
        .read_snapshot(&deleted.snapshot)
        .unwrap();
    let relation = read.get_relation(relation).unwrap();

    assert_eq!(
        relation.lifecycle,
        RecordLifecycleState::RetainedDanglingForAudit
    );
}

#[test]
fn relation_integrity_commit_boundary_rejects_relation_retirement_under_cascade_policy() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
        CascadeDeletePolicy::CascadeDeleteRelations,
    );
    let (source, _target, _relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete-source").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id: source }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(
                error.code(),
                DiagnosticCode::RelationEndpointDeletionIntegrityViolation
            );
            assert!(error
                .detail()
                .contains("requires audit-retained relation retirement"));
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_allows_opposite_endpoint_delete_after_relation_retirement() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, target, relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    delete_entity(&mut runtime, source);
    let deleted_target = delete_entity(&mut runtime, target);
    let read = runtime
        .visibility_reads()
        .read_snapshot(&deleted_target.snapshot)
        .unwrap();
    let relation = read.get_relation(relation).unwrap();

    assert_eq!(
        relation.lifecycle,
        RecordLifecycleState::RetainedDanglingForAudit
    );
}

#[test]
fn relation_integrity_endpoint_deletion_history_stays_branch_local_under_divergence() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, target, relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();

    let main_delete = delete_entity(&mut runtime, source);
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        target,
        "feature-target",
        BranchId("feature".to_string()),
    );

    let main_digest = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let feature_digest = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("feature".to_string()),
        relation,
        None,
    );
    let main_head_version = runtime
        .history_access()
        .branch_head(&BranchId("main".to_string()))
        .unwrap()
        .version_id;
    let feature_head_version = runtime
        .history_access()
        .branch_head(&BranchId("feature".to_string()))
        .unwrap()
        .version_id;
    let main_inspection = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        main_head_version,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );
    let feature_inspection = runtime.inspection_access().inspect_historical_record(
        &BranchId("feature".to_string()),
        feature_head_version,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );

    assert_eq!(main_delete.version_id, main_head_version);
    assert_eq!(main_digest.entry_count, 2);
    assert!(feature_digest.entry_count < main_digest.entry_count);
    assert_eq!(
        main_inspection
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.query_result.trace.branch_id.clone()),
        Some(BranchId("main".to_string()))
    );
    assert_eq!(
        feature_inspection
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.query_result.trace.branch_id.clone()),
        Some(BranchId("feature".to_string()))
    );
    let main_read = runtime
        .visibility_reads()
        .read_snapshot(&main_delete.snapshot)
        .unwrap();
    assert_eq!(
        main_read.get_relation(relation).unwrap().lifecycle,
        RecordLifecycleState::RetainedDanglingForAudit
    );
}
