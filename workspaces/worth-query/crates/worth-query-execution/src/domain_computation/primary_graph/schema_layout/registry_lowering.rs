use std::collections::{BTreeMap, BTreeSet};

use worth_query_installation::facade::{
    ApplicationSchemaMember, ErasedApplicationSchemaDeclaration,
    WorthQueryInstalledApplicationSchemaContractCatalog,
};
use worth_relational::facade::config::{CascadeDeletePolicy, CrossContextPolicy};
use worth_relational::facade::identity::KindId;
use worth_relational::facade::schema::{
    DeclaredAspectContractBinding, EntityKindRegistration, KindAspectContractDeclarations,
    RelationIntegrityDeclarations, RelationKindRegistration, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};

use super::{
    kind_space_exhausted, relational_schema_denial, WorthQueryPrimaryGraphInstallationDenial,
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

pub(super) fn next_provider_kind_id(
    registry: &RelationalSchemaRegistry,
    entity_kinds: impl Iterator<Item = KindId>,
    relation_kinds: impl Iterator<Item = KindId>,
) -> Result<KindId, WorthQueryPrimaryGraphInstallationDenial> {
    let next_application_kind = entity_kinds
        .chain(relation_kinds)
        .map(|kind| kind.0)
        .max()
        .map(|kind| kind.checked_add(1).ok_or_else(kind_space_exhausted))
        .transpose()?;
    Ok(KindId(
        next_application_kind.unwrap_or(next_kind_id(registry)?),
    ))
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
                    SchemaId(format!(
                        "application-schema:{}:{}:{}:{}",
                        schema.owner(),
                        schema.name(),
                        schema.major(),
                        schema.minor(),
                    )),
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

pub(super) struct LoweredApplicationContractBindings {
    pub by_entity: BTreeMap<String, Vec<DeclaredAspectContractBinding>>,
}

pub(super) fn lower_application_contract_bindings(
    catalog: &WorthQueryInstalledApplicationSchemaContractCatalog,
) -> LoweredApplicationContractBindings {
    let mut by_entity = BTreeMap::<String, Vec<DeclaredAspectContractBinding>>::new();
    for installed in catalog.contracts() {
        by_entity
            .entry(installed.locus().entity().to_string())
            .or_default()
            .push(DeclaredAspectContractBinding {
                binding: installed.binding().clone(),
                contract: installed.contract().clone(),
            });
    }
    LoweredApplicationContractBindings { by_entity }
}
