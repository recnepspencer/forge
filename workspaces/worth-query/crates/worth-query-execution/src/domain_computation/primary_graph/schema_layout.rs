use std::collections::BTreeMap;

use worth_foundational::facade::{
    AspectFieldLocator, AspectKey, CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use worth_query_installation::facade::{
    ApplicationSchemaMember, ErasedApplicationSchemaDeclaration,
};
use worth_relational::facade::identity::KindId;
use worth_relational::facade::indexes::DerivedIndexId;
use worth_relational::facade::schema::RelationalSchemaRegistry;

use super::{
    WorthQueryPrimaryGraphInstallationDenial, WorthQueryPrimaryGraphInstallationDenialKind,
};

mod principal_binding;
mod registry_lowering;

use principal_binding::lower_principal_bindings;
pub(super) use principal_binding::WorthQueryPrimaryPrincipalBindingLayout;
use registry_lowering::{
    lower_entity_aspects, lower_kind_ids, register_entity, register_relation,
    relational_schema_basis,
};

#[derive(Debug)]
pub(super) struct WorthQueryPrimaryGraphLayout {
    principal_bindings: BTreeMap<String, WorthQueryPrimaryPrincipalBindingLayout>,
    entity_kinds: BTreeMap<String, KindId>,
    relation_kinds: BTreeMap<String, WorthQueryPrimaryRelationLayout>,
    fields: BTreeMap<(String, String, String), WorthQueryPrimaryFieldLayout>,
}

#[derive(Clone, Debug)]
pub(super) struct WorthQueryPrimaryRelationLayout {
    pub(super) kind: KindId,
    pub(super) from: KindId,
    pub(super) to: KindId,
}

#[derive(Clone, Debug)]
pub(super) struct WorthQueryPrimaryFieldLayout {
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
        let principal_bindings = lower_principal_bindings(schema, &entity_kinds, &relation_kinds)?;
        let relation_layouts = lower_relation_layouts(schema, &entity_kinds, &relation_kinds)?;
        let fields = lower_fields(schema, &entity_kinds)?;

        Ok((
            Self {
                principal_bindings,
                entity_kinds,
                relation_kinds: relation_layouts,
                fields,
            },
            registry,
        ))
    }

    pub(super) fn principal_binding(
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

    pub(super) fn entity_kind(&self, entity: &str) -> Option<KindId> {
        self.entity_kinds.get(entity).copied()
    }

    pub(super) fn relation(&self, relation: &str) -> Option<&WorthQueryPrimaryRelationLayout> {
        self.relation_kinds.get(relation)
    }

    pub(super) fn field_locator(
        &self,
        entity: &str,
        aspect: &str,
        field: &str,
    ) -> Option<&AspectFieldLocator> {
        self.fields
            .get(&(entity.to_string(), aspect.to_string(), field.to_string()))
            .map(|layout| &layout.locator)
    }

    pub(super) fn equality_field(
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
