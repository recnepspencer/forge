use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{
    AspectFieldLocator, AspectKey, CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use worth_query_installation::facade::{
    ApplicationSchemaMember, ErasedApplicationSchemaDeclaration,
};
use worth_relational::facade::identity::KindId;
use worth_relational::facade::indexes::{DerivedIndexDefinition, DerivedIndexId};
use worth_relational::facade::schema::RelationalSchemaRegistry;

use super::{
    WorthQueryPrimaryGraphInstallationDenial, WorthQueryPrimaryGraphInstallationDenialKind,
};

mod continuation_ordering;
mod principal_binding;
mod provider_idempotency;
mod registry_lowering;

use continuation_ordering::{
    lower_continuation_orderings, WorthQueryPrimaryContinuationOrderingLayout,
};
use principal_binding::lower_principal_bindings;
pub(in crate::domain_computation) use principal_binding::WorthQueryPrimaryPrincipalBindingLayout;
use provider_idempotency::lower_provider_idempotency;
pub(super) use provider_idempotency::WorthQueryProviderIdempotencyLayout;
use registry_lowering::{
    lower_entity_aspects, lower_kind_ids, next_provider_kind_id, register_entity,
    register_relation, relational_schema_basis,
};

#[derive(Debug)]
pub(in crate::domain_computation) struct WorthQueryPrimaryGraphLayout {
    principal_bindings: BTreeMap<String, WorthQueryPrimaryPrincipalBindingLayout>,
    entity_kinds: BTreeMap<String, KindId>,
    relation_kinds: BTreeMap<String, WorthQueryPrimaryRelationLayout>,
    fields: BTreeMap<(String, String, String), WorthQueryPrimaryFieldLayout>,
    equality_field_keys: BTreeMap<AspectKey, BTreeSet<FieldKey>>,
    projection_field_keys: BTreeMap<AspectKey, BTreeSet<FieldKey>>,
    continuation_orderings: Vec<WorthQueryPrimaryContinuationOrderingLayout>,
    provider_idempotency: WorthQueryProviderIdempotencyLayout,
}

#[derive(Clone, Debug)]
pub(in crate::domain_computation) struct WorthQueryPrimaryRelationLayout {
    pub(in crate::domain_computation) kind: KindId,
    pub(in crate::domain_computation) from: KindId,
    pub(in crate::domain_computation) to: KindId,
}

#[derive(Clone, Debug)]
pub(in crate::domain_computation) struct WorthQueryPrimaryFieldLayout {
    pub(super) entity_kind: KindId,
    pub(super) locator: AspectFieldLocator,
    pub(super) equality_index_id: Option<DerivedIndexId>,
}

impl WorthQueryPrimaryGraphLayout {
    pub(super) fn lower(
        schema: &ErasedApplicationSchemaDeclaration,
        existing_registry: &RelationalSchemaRegistry,
    ) -> Result<(Self, RelationalSchemaRegistry), WorthQueryPrimaryGraphInstallationDenial> {
        let (entity_kinds, relation_kinds) = lower_kind_ids(schema, existing_registry)?;
        let (schema_id, schema_version_id) = relational_schema_basis(schema, existing_registry)?;
        let mut registry = RelationalSchemaRegistry::new();
        let mut contract_ordinal = 1_u64;

        for (entity, kind_id) in &entity_kinds {
            let aspects = lower_entity_aspects(schema, entity, &mut contract_ordinal)?;
            registry = register_entity(
                registry,
                &schema_id,
                schema_version_id,
                entity,
                *kind_id,
                aspects,
            )?;
        }
        for (relation, kind_id) in &relation_kinds {
            registry =
                register_relation(registry, &schema_id, schema_version_id, relation, *kind_id)?;
        }
        let provider_kind = next_provider_kind_id(
            existing_registry,
            entity_kinds.values().copied(),
            relation_kinds.values().copied(),
        )?;
        let (registry, provider_idempotency) = lower_provider_idempotency(
            registry,
            &schema_id,
            schema_version_id,
            provider_kind,
            &mut contract_ordinal,
        )?;
        let principal_bindings = lower_principal_bindings(schema, &entity_kinds, &relation_kinds)?;
        let relation_layouts = lower_relation_layouts(schema, &entity_kinds, &relation_kinds)?;
        let continuation_orderings =
            lower_continuation_orderings(schema, &entity_kinds, &relation_layouts)?;
        let fields = lower_fields(schema, &entity_kinds)?;
        let equality_field_keys = field_capability_keys(
            fields
                .values()
                .filter(|layout| layout.equality_index_id.is_some()),
        );
        let projection_field_keys = field_capability_keys(fields.values());

        Ok((
            Self {
                principal_bindings,
                entity_kinds,
                relation_kinds: relation_layouts,
                fields,
                equality_field_keys,
                projection_field_keys,
                continuation_orderings,
                provider_idempotency,
            },
            registry,
        ))
    }

    pub(in crate::domain_computation) fn principal_binding(
        &self,
        name: &str,
    ) -> Option<&WorthQueryPrimaryPrincipalBindingLayout> {
        self.principal_bindings.get(name)
    }

    pub(super) fn principal_bindings(
        &self,
    ) -> impl Iterator<Item = (&str, &WorthQueryPrimaryPrincipalBindingLayout)> {
        self.principal_bindings
            .iter()
            .map(|(name, layout)| (name.as_str(), layout))
    }

    pub(super) fn principal_bindings_mut(
        &mut self,
    ) -> impl Iterator<Item = (&str, &mut WorthQueryPrimaryPrincipalBindingLayout)> {
        self.principal_bindings
            .iter_mut()
            .map(|(name, layout)| (name.as_str(), layout))
    }

    pub(in crate::domain_computation) fn entity_kind(&self, entity: &str) -> Option<KindId> {
        self.entity_kinds.get(entity).copied()
    }

    pub(in crate::domain_computation) fn relation(
        &self,
        relation: &str,
    ) -> Option<&WorthQueryPrimaryRelationLayout> {
        self.relation_kinds.get(relation)
    }

    pub(in crate::domain_computation) fn field_locator(
        &self,
        entity: &str,
        aspect: &str,
        field: &str,
    ) -> Option<&AspectFieldLocator> {
        self.fields
            .get(&(entity.to_string(), aspect.to_string(), field.to_string()))
            .map(|layout| &layout.locator)
    }

    pub(in crate::domain_computation) fn equality_field(
        &self,
        entity: &str,
        aspect: &str,
        field: &str,
    ) -> Option<&WorthQueryPrimaryFieldLayout> {
        self.fields
            .get(&(entity.to_string(), aspect.to_string(), field.to_string()))
            .filter(|layout| layout.equality_index_id.is_some())
    }

    pub(super) fn equality_fields_mut(
        &mut self,
    ) -> impl Iterator<Item = (&(String, String, String), &mut WorthQueryPrimaryFieldLayout)> {
        self.fields
            .iter_mut()
            .filter(|(_, layout)| layout.equality_index_id.is_some())
    }

    pub(super) fn equality_index_ids(&self) -> impl Iterator<Item = DerivedIndexId> + '_ {
        self.fields
            .values()
            .filter_map(|field| field.equality_index_id)
    }

    pub(super) fn register_continuation_orderings(
        &mut self,
        mut register: impl FnMut(DerivedIndexDefinition) -> DerivedIndexId,
    ) {
        for (ordinal, continuation) in self.continuation_orderings.iter_mut().enumerate() {
            let index_id = register(continuation.index_definition(ordinal));
            continuation.bind_index(index_id);
        }
    }

    pub(super) fn supports_continuation_ordering(
        &self,
        contract: &worth_query_installation::facade::WorthQueryInstalledApplicationContinuationContract,
    ) -> bool {
        self.continuation_orderings
            .iter()
            .any(|layout| layout.matches(contract))
    }

    pub(super) fn continuation_ordering_index_id(
        &self,
        contract: &worth_query_installation::facade::WorthQueryInstalledApplicationContinuationContract,
    ) -> Option<DerivedIndexId> {
        self.continuation_orderings
            .iter()
            .find(|layout| layout.matches(contract))
            .map(WorthQueryPrimaryContinuationOrderingLayout::index_id)
    }

    pub(super) fn continuation_ordering_index_ids(
        &self,
    ) -> impl Iterator<Item = DerivedIndexId> + '_ {
        self.continuation_orderings
            .iter()
            .map(WorthQueryPrimaryContinuationOrderingLayout::index_id)
    }

    pub(super) fn supports_equality_field(&self, aspect: &AspectKey, field: &FieldKey) -> bool {
        self.equality_field_keys
            .get(aspect)
            .is_some_and(|fields| fields.contains(field))
    }

    pub(super) fn supports_projection_field(&self, aspect: &AspectKey, field: &FieldKey) -> bool {
        self.projection_field_keys
            .get(aspect)
            .is_some_and(|fields| fields.contains(field))
    }

    pub(super) const fn provider_idempotency(&self) -> &WorthQueryProviderIdempotencyLayout {
        &self.provider_idempotency
    }

    pub(super) fn provider_idempotency_mut(&mut self) -> &mut WorthQueryProviderIdempotencyLayout {
        &mut self.provider_idempotency
    }
}

fn field_capability_keys<'a>(
    fields: impl IntoIterator<Item = &'a WorthQueryPrimaryFieldLayout>,
) -> BTreeMap<AspectKey, BTreeSet<FieldKey>> {
    let mut keys = BTreeMap::<AspectKey, BTreeSet<FieldKey>>::new();
    for field in fields {
        let aspect = field.locator.aspect().aspect_key().clone();
        let field_key = field
            .locator
            .field_path()
            .fields()
            .first()
            .expect("primary application fields always have a single canonical field")
            .clone();
        keys.entry(aspect).or_default().insert(field_key);
    }
    keys
}

fn lower_relation_layouts(
    schema: &ErasedApplicationSchemaDeclaration,
    entity_kinds: &BTreeMap<String, KindId>,
    relation_kinds: &BTreeMap<String, KindId>,
) -> Result<
    BTreeMap<String, WorthQueryPrimaryRelationLayout>,
    WorthQueryPrimaryGraphInstallationDenial,
> {
    schema
        .members()
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Relation { relation, from, to } => Some((relation, from, to)),
            _ => None,
        })
        .map(|(relation, from, to)| {
            Ok((
                relation.clone(),
                WorthQueryPrimaryRelationLayout {
                    kind: required_kind(relation_kinds, relation)?,
                    from: required_kind(entity_kinds, from)?,
                    to: required_kind(entity_kinds, to)?,
                },
            ))
        })
        .collect()
}

fn lower_fields(
    schema: &ErasedApplicationSchemaDeclaration,
    entity_kinds: &BTreeMap<String, KindId>,
) -> Result<
    BTreeMap<(String, String, String), WorthQueryPrimaryFieldLayout>,
    WorthQueryPrimaryGraphInstallationDenial,
> {
    schema
        .members()
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Field {
                entity,
                aspect,
                field,
                equality_queryable,
                ..
            } => Some((entity, aspect, field, equality_queryable)),
            _ => None,
        })
        .map(|(entity, aspect, field, equality_queryable)| {
            Ok((
                (entity.clone(), aspect.clone(), field.clone()),
                WorthQueryPrimaryFieldLayout {
                    entity_kind: required_kind(entity_kinds, entity)?,
                    locator: planned_field_locator(aspect, field)?,
                    equality_index_id: equality_queryable.then_some(DerivedIndexId(0)),
                },
            ))
        })
        .collect()
}

pub(super) fn required_kind(
    kinds: &BTreeMap<String, KindId>,
    name: &str,
) -> Result<KindId, WorthQueryPrimaryGraphInstallationDenial> {
    kinds.get(name).copied().ok_or_else(|| invalid_member(name))
}

pub(super) fn planned_field_locator(
    aspect: &str,
    field: &str,
) -> Result<AspectFieldLocator, WorthQueryPrimaryGraphInstallationDenial> {
    Ok(AspectFieldLocator::new(
        LocatorAuthority::Planned,
        valid_aspect_key(aspect)?,
        CanonicalFieldPath::single(valid_field_key(field)?),
    ))
}

pub(super) fn valid_aspect_key(
    value: &str,
) -> Result<AspectKey, WorthQueryPrimaryGraphInstallationDenial> {
    AspectKey::new(value).ok_or_else(|| invalid_member(value))
}

pub(super) fn valid_field_key(
    value: &str,
) -> Result<FieldKey, WorthQueryPrimaryGraphInstallationDenial> {
    FieldKey::new(value).ok_or_else(|| invalid_member(value))
}

pub(super) fn invalid_member(subject: &str) -> WorthQueryPrimaryGraphInstallationDenial {
    WorthQueryPrimaryGraphInstallationDenial::new(
        WorthQueryPrimaryGraphInstallationDenialKind::InvalidSchemaMember,
        subject,
    )
}

pub(super) fn kind_space_exhausted() -> WorthQueryPrimaryGraphInstallationDenial {
    invalid_member("application schema exhausts Relational kind identity space")
}

pub(super) fn contract_space_exhausted() -> WorthQueryPrimaryGraphInstallationDenial {
    invalid_member("application schema exhausts Relational aspect-contract identity space")
}

pub(super) fn relational_schema_denial(
    denial: worth_relational::facade::schema::SchemaRegistryError,
) -> WorthQueryPrimaryGraphInstallationDenial {
    WorthQueryPrimaryGraphInstallationDenial::new(
        WorthQueryPrimaryGraphInstallationDenialKind::RelationalSchemaRejected,
        format!("{denial:?}"),
    )
}
