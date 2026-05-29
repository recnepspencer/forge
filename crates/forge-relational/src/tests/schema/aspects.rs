use forge_foundational::{AspectShape, ReferenceAspectType, ScalarAspectType};

use crate::capabilities::AspectPlanSource;
use crate::facade::merge::{
    AspectMergePolicyDeclaration, AspectMergePolicyKind, IdentityBasisDeclaration,
    IdentityBasisKind, IdentityBasisScope,
};
use crate::facade::schema::{
    AspectBinding, AspectKey, DeclaredAspect, EntityKindRegistration, KindAspectDeclarations,
    RelationKindRegistration, RelationalSchemaRegistry, SchemaId, SchemaRegistryErrorClass,
    SchemaVersionId,
};
use crate::tests::support::*;

#[test]
fn schema_aspect_declarations_are_canonicalized_before_revision_is_derived() {
    let first = KindAspectDeclarations::new(vec![
        lifecycle_aspect_named("zeta"),
        entity_field_aspect(
            crate::tests::support::aspect_key("alpha"),
            crate::tests::support::field_key("name"),
        ),
    ]);
    let second = KindAspectDeclarations::new(vec![
        entity_field_aspect(
            crate::tests::support::aspect_key("alpha"),
            crate::tests::support::field_key("name"),
        ),
        lifecycle_aspect_named("zeta"),
    ]);

    let first_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: first,
        })
        .unwrap();
    let second_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: second,
        })
        .unwrap();

    let left = first_registry.entity_kinds.get(&KindId(1)).unwrap();
    let right = second_registry.entity_kinds.get(&KindId(1)).unwrap();

    assert_eq!(
        left.aspect_declarations.plan_revision,
        right.aspect_declarations.plan_revision
    );
    assert_eq!(
        left.aspect_declarations
            .aspects
            .iter()
            .map(DeclaredAspect::aspect_key)
            .collect::<Vec<_>>(),
        right
            .aspect_declarations
            .aspects
            .iter()
            .map(DeclaredAspect::aspect_key)
            .collect::<Vec<_>>()
    );
}

#[test]
fn duplicate_aspect_keys_are_rejected() {
    let error = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![
                entity_field_aspect(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                ),
                lifecycle_aspect_named("name"),
            ]),
        })
        .unwrap_err();

    assert!(matches!(
        error.class,
        SchemaRegistryErrorClass::DuplicateAspectKey {
            kind_id: KindId(1),
            ..
        }
    ));
}

#[test]
fn declared_aspect_contract_keys_round_trip_through_registration() {
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![entity_field_aspect(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            )]),
        })
        .expect("registry");

    let trace = registry.entity_aspect_declaration_trace(KindId(1)).unwrap();
    assert_eq!(
        trace.declarations[0].aspect_key,
        AspectKey::new("name").unwrap()
    );
    assert_eq!(trace.declarations[0].contract_revision, 1);
}

#[test]
fn schema_identity_declarations_are_defaulted_and_visible_in_trace() {
    let registry = RelationalSchemaRegistry::new()
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
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
            })
        })
        .unwrap();

    let entity_trace = registry.entity_aspect_declaration_trace(KindId(1)).unwrap();
    let relation_trace = registry
        .relation_aspect_declaration_trace(KindId(2))
        .unwrap();

    assert_eq!(
        entity_trace.identity_declarations,
        vec![
            IdentityBasisDeclaration {
                scope: IdentityBasisScope::EntityKind(KindId(1)),
                basis: IdentityBasisKind::StorageIdentity,
            },
            IdentityBasisDeclaration {
                scope: IdentityBasisScope::EntityKind(KindId(1)),
                basis: IdentityBasisKind::LineageIdentity,
            },
        ]
    );
    assert_eq!(
        relation_trace.identity_declarations,
        vec![IdentityBasisDeclaration {
            scope: IdentityBasisScope::RelationKind(KindId(2)),
            basis: IdentityBasisKind::StorageIdentity,
        }]
    );
}

#[test]
fn relation_schema_rejects_lineage_identity_declarations() {
    let error = RelationalSchemaRegistry::new()
        .register_relation_kind(RelationKindRegistration {
            kind_id: KindId(2),
            kind_name: "test.relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_declarations: KindAspectDeclarations::default().with_identity_declarations(
                vec![IdentityBasisDeclaration {
                    scope: IdentityBasisScope::RelationKind(KindId(2)),
                    basis: IdentityBasisKind::LineageIdentity,
                }],
            ),
            relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
        })
        .unwrap_err();

    assert!(matches!(
        error.class,
        SchemaRegistryErrorClass::InvalidAspectDeclaration {
            kind_id: KindId(2),
            ..
        }
    ));
}

#[test]
fn schema_rejects_unsupported_structural_fingerprint_identity_declarations() {
    for basis in [
        IdentityBasisKind::StructuralFingerprint,
        IdentityBasisKind::Custom(crate::facade::merge::CustomIdentityBasisIdentity {
            name: "custom.identity".into(),
            semantic_version: 1,
        }),
    ] {
        let error = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::default().with_identity_declarations(
                    vec![IdentityBasisDeclaration {
                        scope: IdentityBasisScope::EntityKind(KindId(1)),
                        basis,
                    }],
                ),
            })
            .unwrap_err();

        assert!(matches!(
            error.class,
            SchemaRegistryErrorClass::InvalidAspectDeclaration {
                kind_id: KindId(1),
                ..
            }
        ));
    }
}

#[test]
fn schema_accepts_runtime_owned_record_field_merge_policies_and_rejects_unsupported_policies() {
    let name_key = AspectKey::new("name").unwrap();
    let counter_key = AspectKey::new("counter").unwrap();
    let accepted = [
        (
            name_key.clone(),
            entity_field_aspect(name_key.clone(), crate::tests::support::field_key("name")),
            AspectMergePolicyKind::LastWriterWins,
        ),
        (
            counter_key.clone(),
            entity_i64_field_aspect(
                counter_key.clone(),
                crate::tests::support::field_key("counter"),
            ),
            AspectMergePolicyKind::MonotonicCounter,
        ),
    ];

    for (aspect_key, declared_aspect, policy) in accepted {
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::new(vec![declared_aspect])
                    .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                        aspect_key: aspect_key.clone(),
                        policy: policy.clone(),
                    }]),
            })
            .expect("runtime-owned record field policy should register");

        assert_eq!(
            registry
                .entity_merge_policy_declarations(KindId(1))
                .unwrap(),
            &[AspectMergePolicyDeclaration { aspect_key, policy }]
        );
    }

    for rejected_policy in [
        AspectMergePolicyKind::AdditiveSet,
        AspectMergePolicyKind::Custom(crate::facade::merge::CustomMergePolicyIdentity {
            name: "custom.merge".into(),
            semantic_version: 1,
        }),
    ] {
        let error =
            RelationalSchemaRegistry::new()
                .register_entity_kind(EntityKindRegistration {
                    kind_id: KindId(1),
                    kind_name: "test.entity".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    aspect_declarations: KindAspectDeclarations::new(vec![entity_field_aspect(
                        name_key.clone(),
                        crate::tests::support::field_key("name"),
                    )])
                    .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                        aspect_key: name_key.clone(),
                        policy: rejected_policy,
                    }]),
                })
                .unwrap_err();

        assert!(matches!(
            error.class,
            SchemaRegistryErrorClass::InvalidAspectDeclaration {
                kind_id: KindId(1),
                ..
            }
        ));
    }
}

#[test]
fn schema_rejects_monotonic_counter_on_non_integer_foundational_contract() {
    let name_key = AspectKey::new("name").unwrap();
    let error = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![entity_field_aspect(
                name_key.clone(),
                crate::tests::support::field_key("name"),
            )])
            .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                aspect_key: name_key,
                policy: AspectMergePolicyKind::MonotonicCounter,
            }]),
        })
        .unwrap_err();

    assert!(matches!(
        error.class,
        SchemaRegistryErrorClass::InvalidAspectDeclaration {
            kind_id: KindId(1),
            ..
        }
    ));
}

#[test]
fn schema_rejects_runtime_owned_merge_policies_on_non_record_field_aspects() {
    let lifecycle_key = AspectKey::new("lifecycle").unwrap();
    for policy in [
        AspectMergePolicyKind::Custom(crate::facade::merge::CustomMergePolicyIdentity {
            name: "custom.merge".into(),
            semantic_version: 1,
        }),
        AspectMergePolicyKind::LastWriterWins,
        AspectMergePolicyKind::MonotonicCounter,
        AspectMergePolicyKind::AdditiveSet,
    ] {
        let error = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::new(vec![lifecycle_aspect()])
                    .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                        aspect_key: lifecycle_key.clone(),
                        policy,
                    }]),
            })
            .unwrap_err();

        assert!(matches!(
            error.class,
            SchemaRegistryErrorClass::InvalidAspectDeclaration {
                kind_id: KindId(1),
                ..
            }
        ));
    }
}

#[test]
fn schema_merge_policy_declarations_are_traced_and_available_through_registry() {
    let name_key = AspectKey::new("name").unwrap();
    let lifecycle_key = AspectKey::new("lifecycle").unwrap();
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![
                entity_field_aspect(name_key.clone(), crate::tests::support::field_key("name")),
                lifecycle_aspect(),
            ])
            .with_merge_policy_declarations(vec![
                AspectMergePolicyDeclaration {
                    aspect_key: name_key.clone(),
                    policy: AspectMergePolicyKind::PreferRicher,
                },
                AspectMergePolicyDeclaration {
                    aspect_key: lifecycle_key.clone(),
                    policy: AspectMergePolicyKind::FailOnConflict,
                },
            ]),
        })
        .unwrap();

    let trace = registry.entity_aspect_declaration_trace(KindId(1)).unwrap();
    let policies = registry
        .entity_merge_policy_declarations(KindId(1))
        .unwrap();
    assert_eq!(trace.merge_policy_declarations, policies);
    assert_eq!(
        policies,
        &[
            AspectMergePolicyDeclaration {
                aspect_key: lifecycle_key,
                policy: AspectMergePolicyKind::FailOnConflict,
            },
            AspectMergePolicyDeclaration {
                aspect_key: name_key,
                policy: AspectMergePolicyKind::PreferRicher,
            },
        ]
    );
}

#[test]
fn schema_plan_revision_changes_when_identity_or_merge_policy_semantics_change() {
    let name_key = AspectKey::new("name").unwrap();
    let base = KindAspectDeclarations::new(vec![entity_field_aspect(
        name_key.clone(),
        crate::tests::support::field_key("name"),
    )]);
    let identity_variant = base.clone().with_identity_declarations(vec![
        IdentityBasisDeclaration {
            scope: IdentityBasisScope::EntityKind(KindId(1)),
            basis: IdentityBasisKind::StorageIdentity,
        },
        IdentityBasisDeclaration {
            scope: IdentityBasisScope::AspectKey(name_key.clone()),
            basis: IdentityBasisKind::DeclaredKeySet(vec![name_key.clone()].into()),
        },
    ]);
    let policy_variant =
        base.clone()
            .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                aspect_key: name_key.clone(),
                policy: AspectMergePolicyKind::PreferRicher,
            }]);

    let base_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: base,
        })
        .unwrap();
    let identity_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: identity_variant,
        })
        .unwrap();
    let policy_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: policy_variant,
        })
        .unwrap();

    let base_revision = base_registry
        .entity_registration(KindId(1))
        .unwrap()
        .aspect_declarations
        .plan_revision;
    let identity_revision = identity_registry
        .entity_registration(KindId(1))
        .unwrap()
        .aspect_declarations
        .plan_revision;
    let policy_revision = policy_registry
        .entity_registration(KindId(1))
        .unwrap()
        .aspect_declarations
        .plan_revision;

    assert_ne!(base_revision, identity_revision);
    assert_ne!(base_revision, policy_revision);
    assert_ne!(identity_revision, policy_revision);
}

#[test]
fn schema_rejects_merge_policy_for_undeclared_aspect_key() {
    let error = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::default().with_merge_policy_declarations(
                vec![AspectMergePolicyDeclaration {
                    aspect_key: AspectKey::new("missing").unwrap(),
                    policy: AspectMergePolicyKind::FailOnConflict,
                }],
            ),
        })
        .unwrap_err();

    assert!(matches!(
        error.class,
        SchemaRegistryErrorClass::InvalidAspectDeclaration {
            kind_id: KindId(1),
            ..
        }
    ));
}

#[test]
fn relation_field_aspects_are_governed_by_foundational_contract_shape() {
    let registry = RelationalSchemaRegistry::new()
        .register_relation_kind(RelationKindRegistration {
            kind_id: KindId(2),
            kind_name: "test.relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_declarations: KindAspectDeclarations::new(vec![relation_field_aspect(
                crate::tests::support::aspect_key("payload"),
                crate::tests::support::field_key("label"),
            )]),
            relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
        })
        .expect("relation field aspect with scalar contract registers");

    let relation_trace = registry
        .relation_aspect_declaration_trace(KindId(2))
        .expect("relation declaration trace");
    assert_eq!(
        relation_trace.declarations[0].aspect_key,
        aspect_key("payload")
    );

    let error = RelationalSchemaRegistry::new()
        .register_relation_kind(RelationKindRegistration {
            kind_id: KindId(3),
            kind_name: "test.invalid_relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_declarations: KindAspectDeclarations::new(vec![
                relation_field_with_entity_reference_contract("target_as_field", "label"),
            ]),
            relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
        })
        .unwrap_err();

    assert!(matches!(
        error.class,
        SchemaRegistryErrorClass::InvalidAspectDeclaration {
            kind_id: KindId(3),
            ..
        }
    ));
}

#[test]
fn runtime_build_lowers_schema_aspect_plans() {
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![
                entity_field_aspect(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                ),
                lifecycle_aspect(),
            ]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
            })
        })
        .unwrap();

    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(registry)
        .build();
    let plan = runtime.entity_aspect_plan(KindId(1)).unwrap();

    assert_eq!(plan.kind_id, KindId(1));
    assert_eq!(plan.executable_bindings.len(), 2);
    assert_eq!(
        plan.executable_bindings[0].aspect_key,
        AspectKey::new("lifecycle").unwrap()
    );
    assert_eq!(
        plan.executable_bindings[1].aspect_key,
        AspectKey::new("name").unwrap()
    );
    assert_eq!(
        plan.executable_bindings[0].contract.shape(),
        &AspectShape::Scalar(ScalarAspectType::String)
    );
    assert_eq!(
        plan.executable_bindings[1].contract.shape(),
        &AspectShape::Scalar(ScalarAspectType::String)
    );
}

#[test]
fn schema_and_runtime_expose_consistent_aspect_declaration_and_lowering_traces() {
    let registry = declared_aspect_schema_registry(CascadeDeletePolicy::CascadeDeleteRelations);
    let entity_declaration_trace = registry.entity_aspect_declaration_trace(KindId(1)).unwrap();
    let relation_declaration_trace = registry
        .relation_aspect_declaration_trace(KindId(2))
        .unwrap();
    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(registry.clone())
        .build();
    let entity_lowering_trace = runtime.entity_aspect_plan_trace(KindId(1)).unwrap();
    let relation_lowering_trace = runtime.relation_aspect_plan_trace(KindId(2)).unwrap();

    assert_eq!(entity_declaration_trace.kind_id, KindId(1));
    assert_eq!(
        entity_declaration_trace.plan_revision,
        entity_lowering_trace.plan_revision
    );
    assert_eq!(
        entity_declaration_trace
            .declarations
            .iter()
            .map(|row| row.aspect_key.clone())
            .collect::<Vec<_>>(),
        entity_lowering_trace
            .bindings
            .iter()
            .map(|row| row.aspect_key.clone())
            .collect::<Vec<_>>()
    );
    assert_eq!(entity_declaration_trace.declarations.len(), 2);
    assert_eq!(relation_declaration_trace.kind_id, KindId(2));
    assert_eq!(
        relation_declaration_trace.plan_revision,
        relation_lowering_trace.plan_revision
    );
    assert_eq!(relation_declaration_trace.declarations.len(), 4);
    assert_eq!(relation_lowering_trace.bindings.len(), 4);
    assert_eq!(
        relation_lowering_trace.bindings[0].aspect_key,
        AspectKey::new("label").unwrap()
    );
    assert_eq!(
        relation_lowering_trace.bindings[1].aspect_key,
        AspectKey::new("lifecycle").unwrap()
    );
    assert_eq!(
        relation_lowering_trace.bindings[2].aspect_key,
        AspectKey::new("source").unwrap()
    );
    assert_eq!(
        relation_lowering_trace.bindings[3].aspect_key,
        AspectKey::new("target").unwrap()
    );
}

#[test]
fn schema_traces_emit_diagnostic_artifacts_without_reinterpreting_semantics() {
    let registry = declared_aspect_schema_registry(CascadeDeletePolicy::CascadeDeleteRelations);
    let declaration_trace = registry.entity_aspect_declaration_trace(KindId(1)).unwrap();
    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(registry)
        .build();
    let lowering_trace = runtime.entity_aspect_plan_trace(KindId(1)).unwrap();
    let declaration_artifact = declaration_trace.diagnostic_artifact();
    let lowering_artifact = lowering_trace.diagnostic_artifact();

    assert_eq!(declaration_artifact.scope, DiagnosticsScope::Schema);
    assert_eq!(
        declaration_artifact.kind,
        DiagnosticsArtifactKind::DetailedTrace
    );
    assert_eq!(
        declaration_artifact.entries[0].code,
        DiagnosticCode::AspectDeclarationTraced
    );
    assert_eq!(lowering_artifact.scope, DiagnosticsScope::Schema);
    assert_eq!(
        lowering_artifact.kind,
        DiagnosticsArtifactKind::DetailedTrace
    );
    assert_eq!(
        lowering_artifact.entries[0].code,
        DiagnosticCode::AspectLoweringTraced
    );
}

#[test]
fn aspect_schema_fixture_builds_runtime_with_lowered_plans_for_customized_aspects() {
    let fixture = AspectSchemaFixture {
        entity_aspects: vec![
            entity_field_aspect(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            entity_field_aspect(
                crate::tests::support::aspect_key("status"),
                crate::tests::support::field_key("status"),
            ),
            lifecycle_aspect(),
        ],
        relation_aspects: vec![relation_source_aspect(), relation_target_aspect()],
        ..AspectSchemaFixture::default()
    };
    let runtime = fixture.build_runtime();
    let entity_plan = runtime.entity_aspect_plan(KindId(1)).unwrap();
    let relation_plan = runtime.relation_aspect_plan(KindId(2)).unwrap();

    assert_eq!(entity_plan.executable_bindings.len(), 3);
    assert_eq!(relation_plan.executable_bindings.len(), 2);
    assert_eq!(
        entity_plan
            .executable_bindings
            .iter()
            .map(|binding| binding.aspect_key.clone())
            .collect::<Vec<_>>(),
        vec![
            aspect_key("lifecycle"),
            aspect_key("name"),
            aspect_key("status")
        ]
    );
    assert_eq!(
        relation_plan
            .executable_bindings
            .iter()
            .map(|binding| binding.aspect_key.clone())
            .collect::<Vec<_>>(),
        vec![aspect_key("source"), aspect_key("target")]
    );
    assert_eq!(
        relation_plan.executable_bindings[0].contract.shape(),
        &AspectShape::Reference(ReferenceAspectType::Entity)
    );
    assert_eq!(
        relation_plan.executable_bindings[1].contract.shape(),
        &AspectShape::Reference(ReferenceAspectType::Entity)
    );
}

fn lifecycle_aspect_named(name: &str) -> DeclaredAspect {
    DeclaredAspect {
        binding: AspectBinding::LifecycleTransition,
        contract: test_scalar_contract(name),
    }
}

fn test_scalar_contract(name: &str) -> forge_foundational::AspectContract {
    forge_foundational::aspects()
        .contract()
        .for_key(
            forge_foundational::aspects()
                .vocabulary()
                .key(name)
                .expect("valid foundational aspect key"),
        )
        .identified_by(forge_foundational::AspectIdentity(41))
        .at_revision(forge_foundational::aspects().vocabulary().revision(1))
        .scalar(forge_foundational::ScalarAspectType::String)
}

fn relation_field_with_entity_reference_contract(name: &str, field: &str) -> DeclaredAspect {
    DeclaredAspect {
        binding: AspectBinding::RelationField {
            field: forge_foundational::FieldKey::new(field).expect("valid field key"),
        },
        contract: forge_foundational::aspects()
            .contract()
            .for_key(
                forge_foundational::aspects()
                    .vocabulary()
                    .key(name)
                    .expect("valid foundational aspect key"),
            )
            .identified_by(forge_foundational::AspectIdentity(42))
            .at_revision(forge_foundational::aspects().vocabulary().revision(1))
            .reference_entity(),
    }
}
