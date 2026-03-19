use crate::capabilities::AspectPlanSource;
use crate::facade::schema::{
    AspectBinding, AspectComparator, AspectKey, AspectPrecision, DeclaredAspect,
    EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration,
    RelationalSchemaRegistry, SchemaId, SchemaRegistryErrorClass, SchemaVersionId,
};
use crate::schema::data::RelationPayloadClass;
use crate::symbols::data::InternedString;
use crate::tests::support::*;

#[test]
fn schema_aspect_declarations_are_canonicalized_before_revision_is_derived() {
    let first = KindAspectDeclarations::new(vec![
        DeclaredAspect {
            key: AspectKey(InternedString::Raw("zeta".to_string())),
            binding: AspectBinding::LifecycleTransition,
            comparator: AspectComparator::LifecycleTransitionEquality,
            precision: AspectPrecision::Structured,
        },
        DeclaredAspect {
            key: AspectKey(InternedString::Raw("alpha".to_string())),
            binding: AspectBinding::EntityPayloadField {
                field: InternedString::Raw("name".to_string()),
            },
            comparator: AspectComparator::JsonScalarEquality,
            precision: AspectPrecision::Structured,
        },
    ]);
    let second = KindAspectDeclarations::new(vec![
        DeclaredAspect {
            key: AspectKey(InternedString::Raw("alpha".to_string())),
            binding: AspectBinding::EntityPayloadField {
                field: InternedString::Raw("name".to_string()),
            },
            comparator: AspectComparator::JsonScalarEquality,
            precision: AspectPrecision::Structured,
        },
        DeclaredAspect {
            key: AspectKey(InternedString::Raw("zeta".to_string())),
            binding: AspectBinding::LifecycleTransition,
            comparator: AspectComparator::LifecycleTransitionEquality,
            precision: AspectPrecision::Structured,
        },
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
            .map(|aspect| &aspect.key)
            .collect::<Vec<_>>(),
        right
            .aspect_declarations
            .aspects
            .iter()
            .map(|aspect| &aspect.key)
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
                DeclaredAspect {
                    key: AspectKey(InternedString::Raw("name".to_string())),
                    binding: AspectBinding::EntityPayloadField {
                        field: InternedString::Raw("name".to_string()),
                    },
                    comparator: AspectComparator::JsonScalarEquality,
                    precision: AspectPrecision::Structured,
                },
                DeclaredAspect {
                    key: AspectKey(InternedString::Raw("name".to_string())),
                    binding: AspectBinding::LifecycleTransition,
                    comparator: AspectComparator::LifecycleTransitionEquality,
                    precision: AspectPrecision::Structured,
                },
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
fn topology_only_relations_reject_payload_field_aspects() {
    let error = RelationalSchemaRegistry::new()
        .register_relation_kind(RelationKindRegistration {
            kind_id: KindId(2),
            kind_name: "test.relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            payload_class: RelationPayloadClass::TopologyOnlyRelation,
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_declarations: KindAspectDeclarations::new(vec![DeclaredAspect {
                key: AspectKey(InternedString::Raw("payload".to_string())),
                binding: AspectBinding::RelationPayloadField {
                    field: InternedString::Raw("label".to_string()),
                },
                comparator: AspectComparator::JsonScalarEquality,
                precision: AspectPrecision::Structured,
            }]),
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
fn runtime_build_lowers_schema_aspect_plans() {
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![
                DeclaredAspect {
                    key: AspectKey(InternedString::Raw("name".to_string())),
                    binding: AspectBinding::EntityPayloadField {
                        field: InternedString::Raw("name".to_string()),
                    },
                    comparator: AspectComparator::JsonScalarEquality,
                    precision: AspectPrecision::Structured,
                },
                DeclaredAspect {
                    key: AspectKey(InternedString::Raw("lifecycle".to_string())),
                    binding: AspectBinding::LifecycleTransition,
                    comparator: AspectComparator::LifecycleTransitionEquality,
                    precision: AspectPrecision::Structured,
                },
            ]),
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
        AspectKey(InternedString::Raw("lifecycle".to_string()))
    );
    assert_eq!(
        plan.executable_bindings[1].aspect_key,
        AspectKey(InternedString::Raw("name".to_string()))
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
        AspectKey(InternedString::Raw("label".to_string()))
    );
    assert_eq!(
        relation_lowering_trace.bindings[1].aspect_key,
        AspectKey(InternedString::Raw("lifecycle".to_string()))
    );
    assert_eq!(
        relation_lowering_trace.bindings[2].aspect_key,
        AspectKey(InternedString::Raw("source".to_string()))
    );
    assert_eq!(
        relation_lowering_trace.bindings[3].aspect_key,
        AspectKey(InternedString::Raw("target".to_string()))
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
            entity_payload_aspect("name", "name"),
            entity_payload_aspect("status", "status"),
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
}
