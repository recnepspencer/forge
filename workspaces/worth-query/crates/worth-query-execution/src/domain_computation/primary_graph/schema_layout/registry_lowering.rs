use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{aspects, AspectIdentity};
use worth_query_installation::facade::{
    ApplicationFieldPresence, ApplicationSchemaMember, ErasedApplicationSchemaDeclaration,
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

pub(super) fn lower_entity_aspects(
    schema: &ErasedApplicationSchemaDeclaration,
    entity: &str,
    contract_ordinal: &mut u64,
) -> Result<Vec<DeclaredAspectContractBinding>, WorthQueryPrimaryGraphInstallationDenial> {
    let mut fields_by_aspect = BTreeMap::<
        String,
        Vec<(
            String,
            worth_foundational::facade::ScalarAspectType,
            ApplicationFieldPresence,
        )>,
    >::new();
    for member in schema.members() {
        match member {
            ApplicationSchemaMember::Field {
                entity: member_entity,
                aspect,
                field,
                presence,
                scalar_family,
                ..
            } if member_entity == entity => {
                fields_by_aspect.entry(aspect.clone()).or_default().push((
                    field.clone(),
                    *scalar_family,
                    *presence,
                ));
            }
            _ => {}
        }
    }
    let mut lowered = Vec::with_capacity(fields_by_aspect.len());
    for (aspect, fields) in fields_by_aspect {
        let mut fields = fields.into_iter();
        let (first_field, first_family, first_presence) =
            fields.next().ok_or_else(|| invalid_member(&aspect))?;
        let mut shape = match first_presence {
            ApplicationFieldPresence::Required => aspects()
                .struct_fields()
                .required(first_field, first_family),
            ApplicationFieldPresence::Optional => aspects()
                .struct_fields()
                .optional(first_field, first_family),
        };
        for (field, scalar_family, presence) in fields {
            shape = match presence {
                ApplicationFieldPresence::Required => shape.required(field, scalar_family),
                ApplicationFieldPresence::Optional => shape.optional(field, scalar_family),
            };
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

#[cfg(test)]
mod tests {
    use worth_foundational::facade::{
        aspects, validate_aspect_value, AspectShape, AspectValue, ContractValidationDenial,
        ContractValidationInput, FieldRequirement,
    };
    use worth_query_declaration::facade::application_schema::{
        ApplicationSchema, ApplicationSchemaDeclaration, ApplicationSchemaDeclarationBuilder,
        ApplicationSchemaDeclarationDenial,
    };
    use worth_query_declaration::{worth_query_aspect, worth_query_entity, worth_query_field};

    use super::lower_entity_aspects;

    struct PresenceSchema;

    impl ApplicationSchema for PresenceSchema {
        const OWNER: &'static str = "worth.test";
        const NAME: &'static str = "PresenceSchema";
        const MAJOR: u32 = 1;
        const MINOR: u32 = 0;

        fn declaration(
        ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial>
        {
            ApplicationSchemaDeclarationBuilder::<Self>::for_schema()
                .entity(Record::reference())
                .aspect(Record::reference(), Facts::reference())
                .field(Record::reference(), RequiredValue::reference())
                .field(Record::reference(), OptionalValue::reference())
                .build()
        }
    }

    worth_query_entity!(Record in PresenceSchema);
    worth_query_aspect!(Facts in PresenceSchema, Record);
    worth_query_field!(
        RequiredValue in PresenceSchema, Record, Facts: u64, read_only, equality
    );
    worth_query_field!(
        OptionalValue in PresenceSchema, Record, Facts: optional u64, read_only, equality
    );

    #[test]
    fn application_field_presence_lowers_exactly_into_foundational() {
        let contract = lowered_contract();
        let AspectShape::Struct(shape) = contract.shape() else {
            panic!("application fields must lower as a Foundational struct")
        };
        let requirements = shape
            .fields()
            .iter()
            .map(|field| (field.key().as_str(), field.requirement()))
            .collect::<std::collections::BTreeMap<_, _>>();
        assert_eq!(
            requirements.get("RequiredValue"),
            Some(&FieldRequirement::Required)
        );
        assert_eq!(
            requirements.get("OptionalValue"),
            Some(&FieldRequirement::Optional)
        );
    }

    #[test]
    fn lowered_presence_governs_absence_and_present_value_types() {
        let contract = lowered_contract();
        let optional_absent = aspects()
            .vocabulary()
            .struct_value()
            .with_field("RequiredValue", AspectValue::UInt64(7))
            .finish()
            .unwrap();
        assert!(
            validate_aspect_value(&contract, ContractValidationInput::Struct(optional_absent))
                .is_success()
        );

        let required_absent = aspects().vocabulary().struct_value().finish().unwrap();
        assert!(matches!(
            validate_aspect_value(&contract, ContractValidationInput::Struct(required_absent))
                .into_result(),
            Err(ContractValidationDenial::MissingRequiredField(_))
        ));

        let wrong_optional_type = aspects()
            .vocabulary()
            .struct_value()
            .with_field("RequiredValue", AspectValue::UInt64(7))
            .with_field("OptionalValue", AspectValue::Bool(true))
            .finish()
            .unwrap();
        assert!(matches!(
            validate_aspect_value(
                &contract,
                ContractValidationInput::Struct(wrong_optional_type)
            )
            .into_result(),
            Err(ContractValidationDenial::FieldTypeMismatch { .. })
        ));
    }

    fn lowered_contract() -> worth_foundational::facade::AspectContract {
        let declaration = PresenceSchema::declaration().unwrap();
        let mut next_contract = 1;
        let bindings =
            lower_entity_aspects(declaration.erased(), "Record", &mut next_contract).unwrap();
        let [binding] = bindings.as_slice() else {
            panic!("one declared aspect must lower to one Foundational contract")
        };
        binding.contract.clone()
    }
}
