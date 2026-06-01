use forge_foundational::facade::{AspectFieldLocator, ContractValidatedAspectValueView, FieldKey};

use crate::storage::data::{AuthoritativeFieldComparisonKey, EntityReadRecord, RelationReadRecord};

use super::{EntityProjectionRecord, ProjectionAspectScope, RelationProjectionRecord};

pub(crate) fn entity_query_locus_comparison_key(
    record: &EntityReadRecord,
    field_locator: &AspectFieldLocator,
) -> Option<AuthoritativeFieldComparisonKey> {
    let aspect_key = field_locator.aspect().aspect_key();
    match record
        .authoritative_aspect_state
        .as_ref()?
        .get(aspect_key)?
        .view()
    {
        ContractValidatedAspectValueView::Scalar(_) => {
            let projection_scope = ProjectionAspectScope::whole_aspects([aspect_key.clone()]);
            EntityProjectionRecord::new(record, &projection_scope)
                .aspect_value(aspect_key)
                .map(AuthoritativeFieldComparisonKey::from_aspect_value)
        }
        ContractValidatedAspectValueView::Struct(_) => {
            let field = single_field_locator_key(field_locator)?;
            let projection_scope =
                ProjectionAspectScope::fields(aspect_key.clone(), [field.clone()]);
            EntityProjectionRecord::new(record, &projection_scope)
                .aspect_field_value(aspect_key, field)
                .map(AuthoritativeFieldComparisonKey::from_aspect_value)
        }
    }
}

pub(crate) fn relation_query_locus_comparison_key(
    record: &RelationReadRecord,
    field_locator: &AspectFieldLocator,
) -> Option<AuthoritativeFieldComparisonKey> {
    let aspect_key = field_locator.aspect().aspect_key();
    match record
        .authoritative_aspect_state
        .as_ref()?
        .get(aspect_key)?
        .view()
    {
        ContractValidatedAspectValueView::Scalar(_) => {
            let projection_scope = ProjectionAspectScope::whole_aspects([aspect_key.clone()]);
            RelationProjectionRecord::new(record, &projection_scope)
                .aspect_value(aspect_key)
                .map(AuthoritativeFieldComparisonKey::from_aspect_value)
        }
        ContractValidatedAspectValueView::Struct(_) => {
            let field = single_field_locator_key(field_locator)?;
            let projection_scope =
                ProjectionAspectScope::fields(aspect_key.clone(), [field.clone()]);
            RelationProjectionRecord::new(record, &projection_scope)
                .aspect_field_value(aspect_key, field)
                .map(AuthoritativeFieldComparisonKey::from_aspect_value)
        }
    }
}

fn single_field_locator_key(field_locator: &AspectFieldLocator) -> Option<&FieldKey> {
    match field_locator.field_path().fields() {
        [field] => Some(field),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::{
        admit_authoritative_record_aspect_state, validate_aspect_value, AspectContract,
        AspectContractRevision, AspectFieldLocator, AspectIdentity, AspectKey, AspectValue,
        CanonicalFieldPath, ContractValidationInput, FieldKey, LocatorAuthority, ScalarAspectType,
        StructAspectValue,
    };
    use forge_proof::TransitionOutcome;

    use crate::identity::data::{EntityId, KindId, PartitionId, VersionId};
    use crate::schema::data::{KindResolution, SchemaId, SchemaVersionId};
    use crate::storage::data::{EntityReadRecord, RecordLifecycleState};

    use super::entity_query_locus_comparison_key;

    #[test]
    fn scalar_query_locus_uses_whole_aspect_projection_scope() {
        let field = FieldKey::new("age").expect("field key");
        let aspect_key = AspectKey::new("profile.age").expect("aspect key");
        let contract = AspectContract::scalar(
            aspect_key.clone(),
            AspectIdentity(42),
            AspectContractRevision(1),
            ScalarAspectType::Int64,
        );
        let record = entity_record_with_authoritative_state(
            &contract,
            ContractValidationInput::Scalar(AspectValue::Int64(37)),
        );
        let locator = AspectFieldLocator::new(
            LocatorAuthority::Planned,
            aspect_key,
            CanonicalFieldPath::single(field),
        );

        assert_eq!(
            entity_query_locus_comparison_key(&record, &locator)
                .map(|key| key.canonical_value_bytes().to_vec()),
            Some(crate::aspect_wire::encode_aspect_value(
                &AspectValue::Int64(37)
            ))
        );
    }

    #[test]
    fn struct_query_locus_uses_field_projection_scope() {
        let field = FieldKey::new("title").expect("field key");
        let aspect_key = AspectKey::new("summary").expect("aspect key");
        let shape = forge_foundational::aspects()
            .struct_fields()
            .required("title", ScalarAspectType::String)
            .finish()
            .expect("valid struct shape");
        let contract = AspectContract::struct_aspect(
            aspect_key.clone(),
            AspectIdentity(43),
            AspectContractRevision(1),
            shape,
        );
        let value =
            StructAspectValue::new([(field.clone(), AspectValue::String("projected".into()))])
                .expect("valid struct value");
        let record = entity_record_with_authoritative_state(
            &contract,
            ContractValidationInput::Struct(value),
        );
        let locator = AspectFieldLocator::new(
            LocatorAuthority::Planned,
            aspect_key,
            CanonicalFieldPath::single(field),
        );

        assert_eq!(
            entity_query_locus_comparison_key(&record, &locator)
                .map(|key| key.canonical_value_bytes().to_vec()),
            Some(crate::aspect_wire::encode_aspect_value(
                &AspectValue::String("projected".into())
            ))
        );
    }

    fn entity_record_with_authoritative_state(
        contract: &AspectContract,
        input: ContractValidationInput,
    ) -> EntityReadRecord {
        let TransitionOutcome::Success(validated) = validate_aspect_value(contract, input) else {
            panic!("test value should validate");
        };
        let TransitionOutcome::Success(state) =
            admit_authoritative_record_aspect_state([validated])
        else {
            panic!("validated aspect should admit");
        };
        let (state, _proofs, _basis) = state.into_parts().into_parts();
        EntityReadRecord {
            entity_id: EntityId::new(PartitionId::main(), 0, 1),
            lineage_id: None,
            kind: KindResolution {
                kind_id: KindId(1),
                kind_name: "person".to_string(),
                schema_id: SchemaId("validation".to_string()),
                schema_version_id: SchemaVersionId(1),
            },
            lifecycle: RecordLifecycleState::Live,
            created_at_version: VersionId::new(1),
            retired_at_version: None,
            authoritative_aspect_state: Some(state),
        }
    }
}
