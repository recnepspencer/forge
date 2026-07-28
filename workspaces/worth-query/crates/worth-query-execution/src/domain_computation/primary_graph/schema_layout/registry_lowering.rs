use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{aspects, AspectIdentity};
use worth_query_installation::facade::{
    ApplicationSchemaMember, ErasedApplicationSchemaDeclaration,
};
use worth_relational::facade::config::{CascadeDeletePolicy, CrossContextPolicy};
use worth_relational::facade::identity::KindId;
use worth_relational::facade::schema::{
    AspectBinding, DeclaredAspectContractBinding, EntityKindRegistration,
    KindAspectContractDeclarations, RelationIntegrityDeclarations, RelationKindRegistration,
    RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};

use super::{
    contract_space_exhausted, invalid_member, kind_space_exhausted, relational_schema_denial,
    valid_aspect_key, valid_field_key, WorthQueryPrimaryGraphInstallationDenial,
};

pub(super) fn lower_kind_ids(
    schema: &ErasedApplicationSchemaDeclaration,
    existing_registry: &RelationalSchemaRegistry,
) -> Result<
    (BTreeMap<String, KindId>, BTreeMap<String, KindId>),
    WorthQueryPrimaryGraphInstallationDenial,
> {
    let mut entity_names = BTreeSet::new();
    let mut relation_names = BTreeSet::new();
    for member in schema.members() {
        match member {
            ApplicationSchemaMember::Entity { entity } => {
                entity_names.insert(entity.clone());
            }
            ApplicationSchemaMember::Relation { relation, .. } => {
                relation_names.insert(relation.clone());
            }
            _ => {}
        }
    }
    let mut next_kind = next_kind_id(existing_registry)?;
    let mut entity_kinds = BTreeMap::new();
    let mut relation_kinds = BTreeMap::new();
    for name in entity_names {
        entity_kinds.insert(name, KindId(next_kind));
        next_kind = next_kind.checked_add(1).ok_or_else(kind_space_exhausted)?;
    }
    for name in relation_names {
        relation_kinds.insert(name, KindId(next_kind));
        next_kind = next_kind.checked_add(1).ok_or_else(kind_space_exhausted)?;
    }
    Ok((entity_kinds, relation_kinds))
}

fn next_kind_id(
    registry: &RelationalSchemaRegistry,
) -> Result<u32, WorthQueryPrimaryGraphInstallationDenial> {
    registry
        .entity_kinds
        .keys()
        .chain(registry.relation_kinds.keys())
        .map(|kind| kind.0)
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or_else(kind_space_exhausted)
}

pub(super) fn relational_schema_basis(
    schema: &ErasedApplicationSchemaDeclaration,
    existing_registry: &RelationalSchemaRegistry,
) -> Result<(SchemaId, SchemaVersionId), WorthQueryPrimaryGraphInstallationDenial> {
    existing_registry
        .authoritative_schema_basis()
        .map_err(relational_schema_denial)
        .map(|basis| {
            basis.unwrap_or_else(|| {
                (
                    SchemaId(schema.identity().as_str().to_string()),
                    SchemaVersionId(schema.major()),
                )
            })
        })
}

pub(super) fn register_entity(
    registry: RelationalSchemaRegistry,
    schema_id: &SchemaId,
    schema_version_id: SchemaVersionId,
    entity: &str,
    kind_id: KindId,
    aspects: Vec<DeclaredAspectContractBinding>,
) -> Result<RelationalSchemaRegistry, WorthQueryPrimaryGraphInstallationDenial> {
    registry
        .register_entity_kind(EntityKindRegistration {
            kind_id,
            kind_name: entity.to_string(),
            schema_id: schema_id.clone(),
            schema_version_id,
            aspect_contract_declarations: KindAspectContractDeclarations::new(aspects),
        })
        .map_err(relational_schema_denial)
}

pub(super) fn register_relation(
    registry: RelationalSchemaRegistry,
    schema_id: &SchemaId,
    schema_version_id: SchemaVersionId,
    relation: &str,
    kind_id: KindId,
) -> Result<RelationalSchemaRegistry, WorthQueryPrimaryGraphInstallationDenial> {
    registry
        .register_relation_kind(RelationKindRegistration {
            kind_id,
            kind_name: relation.to_string(),
            schema_id: schema_id.clone(),
            schema_version_id,
            cross_context_policy: CrossContextPolicy::Forbid,
            cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
            aspect_contract_declarations: KindAspectContractDeclarations::default(),
            relation_integrity: RelationIntegrityDeclarations::default(),
        })
        .map_err(relational_schema_denial)
}

pub(super) fn lower_entity_aspects(
    schema: &ErasedApplicationSchemaDeclaration,
    entity: &str,
    contract_ordinal: &mut u64,
) -> Result<Vec<DeclaredAspectContractBinding>, WorthQueryPrimaryGraphInstallationDenial> {
    let mut fields_by_aspect =
        BTreeMap::<String, Vec<(String, worth_foundational::facade::ScalarAspectType)>>::new();
    for member in schema.members() {
        match member {
            ApplicationSchemaMember::Field {
                entity: member_entity,
                aspect,
                field,
                scalar_family,
                ..
            } if member_entity == entity => {
                fields_by_aspect
                    .entry(aspect.clone())
                    .or_default()
                    .push((field.clone(), *scalar_family));
            }
            _ => {}
        }
    }
    let mut lowered = Vec::with_capacity(fields_by_aspect.len());
    for (aspect, fields) in fields_by_aspect {
        let mut fields = fields.into_iter();
        let (first_field, first_family) = fields.next().ok_or_else(|| invalid_member(&aspect))?;
        let mut shape = aspects()
            .struct_fields()
            .required(first_field, first_family);
        for (field, scalar_family) in fields {
            shape = shape.required(field, scalar_family);
        }
        let shape = shape.finish().map_err(|_| invalid_member(&aspect))?;
        let identity = AspectIdentity(*contract_ordinal);
        *contract_ordinal = contract_ordinal
            .checked_add(1)
            .ok_or_else(contract_space_exhausted)?;
        let contract = aspects()
            .contract()
            .for_key(valid_aspect_key(&aspect)?)
            .identified_by(identity)
            .at_revision(aspects().vocabulary().revision(1))
            .struct_aspect(shape);
        lowered.push(DeclaredAspectContractBinding {
            binding: AspectBinding::EntityField {
                field: valid_field_key(&aspect)?,
            },
            contract,
        });
    }
    Ok(lowered)
}
