use forge_foundational::facade::{
    AspectFieldLocator, AspectValue, AuthoritativeRecordAspectState,
    ContractValidatedAspectValueView, FieldKey,
};

use super::EntityReadRecord;

pub fn entity_authoritative_aspect_field_value(
    record: &EntityReadRecord,
    field_locator: &AspectFieldLocator,
) -> Option<AspectValue> {
    authoritative_aspect_field_value(record.authoritative_aspect_state.as_ref(), field_locator)
}

fn authoritative_aspect_field_value(
    authoritative_state: Option<&AuthoritativeRecordAspectState>,
    field_locator: &AspectFieldLocator,
) -> Option<AspectValue> {
    let entry = authoritative_state?.get(field_locator.aspect().aspect_key())?;
    match entry.view() {
        ContractValidatedAspectValueView::Scalar(value) => Some(value.clone()),
        ContractValidatedAspectValueView::Struct(struct_value) => {
            let field = single_field_locator_key(field_locator)?;
            struct_value.get(field).cloned()
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
    use std::collections::BTreeMap;

    use forge_foundational::facade::{
        admit_authoritative_record_aspect_state, validate_aspect_value, AspectContract,
        AspectContractRevision, AspectFieldLocator, AspectIdentity, AspectKey, AspectValue,
        CanonicalFieldPath, ContractValidationInput, FieldKey, LocatorAuthority, ScalarAspectType,
    };
    use forge_proof::TransitionOutcome;

    use crate::identity::data::{EntityId, KindId, PartitionId, VersionId};
    use crate::schema::data::{KindResolution, SchemaId, SchemaVersionId};
    use crate::storage::data::{EntityReadRecord, RecordLifecycleState};

    use super::entity_authoritative_aspect_field_value;

    #[test]
    fn entity_field_value_preserves_authoritative_aspect_value_family() {
        let field = FieldKey::new("age").expect("field key");
        let aspect_key = AspectKey::new("profile.age").expect("aspect key");
        let contract = AspectContract::scalar(
            aspect_key.clone(),
            AspectIdentity(42),
            AspectContractRevision(1),
            ScalarAspectType::Int64,
        );
        let authoritative_state = authoritative_scalar_state(&contract, AspectValue::Int64(37));
        let record = EntityReadRecord {
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
            authoritative_aspect_state: Some(authoritative_state),
            authoritative_field_key_comparison_keys: BTreeMap::new(),
        };
        let locator = AspectFieldLocator::new(
            LocatorAuthority::Planned,
            aspect_key,
            CanonicalFieldPath::single(field),
        );

        assert_eq!(
            entity_authoritative_aspect_field_value(&record, &locator),
            Some(AspectValue::Int64(37))
        );
    }

    fn authoritative_scalar_state(
        contract: &AspectContract,
        value: AspectValue,
    ) -> forge_foundational::facade::AuthoritativeRecordAspectState {
        let TransitionOutcome::Success(validated) =
            validate_aspect_value(contract, ContractValidationInput::Scalar(value))
        else {
            panic!("test value should validate");
        };
        let TransitionOutcome::Success(state) =
            admit_authoritative_record_aspect_state([validated])
        else {
            panic!("validated aspect should admit");
        };
        let (state, _proofs, _basis) = state.into_parts().into_parts();
        state
    }
}
