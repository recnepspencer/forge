use super::*;

pub(crate) fn test_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
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
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .unwrap()
}

pub(crate) fn aspect_key(name: &str) -> AspectKey {
    AspectKey(InternedString::Raw(name.to_string()))
}

pub(crate) fn entity_payload_aspect(name: &str, field: &str) -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key(name),
        binding: AspectBinding::EntityPayloadField {
            field: InternedString::Raw(field.to_string()),
        },
        comparator: AspectComparator::JsonScalarEquality,
        precision: AspectPrecision::Structured,
    }
}

pub(crate) fn relation_payload_aspect(name: &str, field: &str) -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key(name),
        binding: AspectBinding::RelationPayloadField {
            field: InternedString::Raw(field.to_string()),
        },
        comparator: AspectComparator::JsonScalarEquality,
        precision: AspectPrecision::Structured,
    }
}

pub(crate) fn lifecycle_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key("lifecycle"),
        binding: AspectBinding::LifecycleTransition,
        comparator: AspectComparator::LifecycleTransitionEquality,
        precision: AspectPrecision::Structured,
    }
}

pub(crate) fn relation_source_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key("source"),
        binding: AspectBinding::RelationSourceEndpoint,
        comparator: AspectComparator::EndpointIdentityEquality,
        precision: AspectPrecision::Structured,
    }
}

pub(crate) fn relation_target_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key("target"),
        binding: AspectBinding::RelationTargetEndpoint,
        comparator: AspectComparator::EndpointIdentityEquality,
        precision: AspectPrecision::Structured,
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AspectSchemaFixture {
    pub entity_kind_id: KindId,
    pub relation_kind_id: KindId,
    pub entity_kind_name: String,
    pub relation_kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub relation_payload_class: RelationPayloadClass,
    pub cross_context_policy: CrossContextPolicy,
    pub cascade_delete_policy: CascadeDeletePolicy,
    pub entity_aspects: Vec<DeclaredAspect>,
    pub relation_aspects: Vec<DeclaredAspect>,
}

impl Default for AspectSchemaFixture {
    fn default() -> Self {
        Self {
            entity_kind_id: KindId(1),
            relation_kind_id: KindId(2),
            entity_kind_name: "test.entity".to_string(),
            relation_kind_name: "test.relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            relation_payload_class: RelationPayloadClass::PayloadBearingRelation,
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            entity_aspects: Vec::new(),
            relation_aspects: Vec::new(),
        }
    }
}

impl AspectSchemaFixture {
    pub(crate) fn with_default_declared_aspects(
        cascade_delete_policy: CascadeDeletePolicy,
    ) -> Self {
        Self {
            cascade_delete_policy,
            entity_aspects: vec![entity_payload_aspect("name", "name"), lifecycle_aspect()],
            relation_aspects: vec![
                relation_payload_aspect("label", "label"),
                lifecycle_aspect(),
                relation_source_aspect(),
                relation_target_aspect(),
            ],
            ..Self::default()
        }
    }

    pub(crate) fn build_registry(&self) -> RelationalSchemaRegistry {
        RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: self.entity_kind_id,
                kind_name: self.entity_kind_name.clone(),
                schema_id: self.schema_id.clone(),
                schema_version_id: self.schema_version_id,
                aspect_declarations: KindAspectDeclarations::new(self.entity_aspects.clone()),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: self.relation_kind_id,
                    kind_name: self.relation_kind_name.clone(),
                    schema_id: self.schema_id.clone(),
                    schema_version_id: self.schema_version_id,
                    payload_class: self.relation_payload_class,
                    cross_context_policy: self.cross_context_policy,
                    cascade_delete_policy: self.cascade_delete_policy,
                    aspect_declarations: KindAspectDeclarations::new(self.relation_aspects.clone()),
                    relation_integrity: RelationIntegrityDeclarations::default(),
                })
            })
            .unwrap()
    }

    pub(crate) fn build_runtime(&self) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(self.build_registry())
            .build()
    }
}

pub(crate) fn runtime_with_declared_aspect_schema(
    cascade_delete_policy: CascadeDeletePolicy,
) -> RelationalRuntime {
    AspectSchemaFixture::with_default_declared_aspects(cascade_delete_policy).build_runtime()
}

pub(crate) fn declared_aspect_schema_registry(
    cascade_delete_policy: CascadeDeletePolicy,
) -> RelationalSchemaRegistry {
    AspectSchemaFixture::with_default_declared_aspects(cascade_delete_policy).build_registry()
}
