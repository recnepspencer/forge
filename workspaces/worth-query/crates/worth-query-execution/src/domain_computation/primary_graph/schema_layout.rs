use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{AspectContract, AspectFieldLocator, AspectKey, FieldKey};
use worth_query_installation::facade::{
    ApplicationSchemaMember, ErasedApplicationSchemaDeclaration,
};
use worth_relational::facade::identity::KindId;
use worth_relational::facade::indexes::DerivedIndexId;
use worth_relational::facade::schema::RelationalSchemaRegistry;

use super::WorthQueryPrimaryGraphInstallationDenial;

mod capability_grant_join;
mod continuation_ordering;
mod installation_primitives;
mod principal_binding;
mod provider_aftermath_causality;
mod provider_dispatch_outbox;
mod provider_idempotency;
mod registry_lowering;

use crate::domain_computation::application_aftermath::WorthQueryDispatchOutboxLayout;
use capability_grant_join::{lower_capability_grant_joins, WorthQueryCapabilityGrantJoinLayout};
use continuation_ordering::{
    lower_continuation_orderings, WorthQueryPrimaryContinuationOrderingLayout,
};
pub(super) use installation_primitives::{
    contract_space_exhausted, invalid_member, kind_space_exhausted, planned_field_locator,
    relational_schema_denial, required_kind, valid_aspect_key, valid_field_key,
};
use principal_binding::lower_principal_bindings;
pub(in crate::domain_computation) use principal_binding::WorthQueryPrimaryPrincipalBindingLayout;
pub(super) use provider_aftermath_causality::{
    lower_provider_aftermath_causality, WorthQueryAftermathCausalityLayout,
};
use provider_dispatch_outbox::lower_provider_dispatch_outbox;
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
    aspect_contracts: BTreeMap<(String, AspectKey), AspectContract>,
    equality_field_keys: BTreeMap<AspectKey, BTreeSet<FieldKey>>,
    projection_field_keys: BTreeMap<AspectKey, BTreeSet<FieldKey>>,
    continuation_orderings: Vec<WorthQueryPrimaryContinuationOrderingLayout>,
    capability_grant_joins: BTreeMap<(String, String), WorthQueryCapabilityGrantJoinLayout>,
    provider_idempotency: WorthQueryProviderIdempotencyLayout,
    provider_dispatch_outbox: WorthQueryDispatchOutboxLayout,
    provider_aftermath_causality: WorthQueryAftermathCausalityLayout,
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
        let mut aspect_contracts = BTreeMap::new();

        for (entity, kind_id) in &entity_kinds {
            let aspects = lower_entity_aspects(schema, entity, &mut contract_ordinal)?;
            for binding in &aspects {
                aspect_contracts.insert(
                    (entity.clone(), binding.aspect_key()),
                    binding.contract.clone(),
                );
            }
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
        let dispatch_outbox_kind = KindId(
            provider_kind
                .0
                .checked_add(1)
                .ok_or_else(kind_space_exhausted)?,
        );
        let (registry, provider_dispatch_outbox) = lower_provider_dispatch_outbox(
            registry,
            &schema_id,
            schema_version_id,
            dispatch_outbox_kind,
            &mut contract_ordinal,
        )?;
        let aftermath_causality_kind = KindId(
            dispatch_outbox_kind
                .0
                .checked_add(1)
                .ok_or_else(kind_space_exhausted)?,
        );
        let (registry, provider_aftermath_causality) = lower_provider_aftermath_causality(
            registry,
            &schema_id,
            schema_version_id,
            aftermath_causality_kind,
            &mut contract_ordinal,
        )?;
        let principal_bindings = lower_principal_bindings(schema, &entity_kinds, &relation_kinds)?;
        let relation_layouts = lower_relation_layouts(schema, &entity_kinds, &relation_kinds)?;
        let continuation_orderings =
            lower_continuation_orderings(schema, &entity_kinds, &relation_layouts)?;
        let capability_grant_joins =
            lower_capability_grant_joins(schema, &entity_kinds, &relation_layouts)?;
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
                aspect_contracts,
                equality_field_keys,
                projection_field_keys,
                continuation_orderings,
                capability_grant_joins,
                provider_idempotency,
                provider_dispatch_outbox,
                provider_aftermath_causality,
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

    pub(in crate::domain_computation) fn aspect_contract(
        &self,
        entity: &str,
        aspect: &AspectKey,
    ) -> Option<&AspectContract> {
        self.aspect_contracts
            .get(&(entity.to_string(), aspect.clone()))
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

    pub(super) const fn provider_dispatch_outbox(&self) -> &WorthQueryDispatchOutboxLayout {
        &self.provider_dispatch_outbox
    }

    pub(super) fn provider_dispatch_outbox_mut(&mut self) -> &mut WorthQueryDispatchOutboxLayout {
        &mut self.provider_dispatch_outbox
    }

    pub(super) const fn provider_aftermath_causality(&self) -> &WorthQueryAftermathCausalityLayout {
        &self.provider_aftermath_causality
    }

    pub(super) fn provider_aftermath_causality_mut(
        &mut self,
    ) -> &mut WorthQueryAftermathCausalityLayout {
        &mut self.provider_aftermath_causality
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
