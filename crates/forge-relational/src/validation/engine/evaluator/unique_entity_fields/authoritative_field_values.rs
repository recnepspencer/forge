use forge_foundational::facade::{
    AspectFieldLocator, AspectValue, ContractValidatedAspectValueView, FieldKey,
};

use crate::storage::data::{
    authoritative_aspect_value_field_comparison_key, AuthoritativeFieldComparisonKey,
    EntityReadRecord,
};

use super::super::super::context::InvariantExecutionContext;

pub(super) fn visible_entity_field_value_conflict(
    context: &InvariantExecutionContext<'_>,
    field_locator: &AspectFieldLocator,
    comparison_key: &AuthoritativeFieldComparisonKey,
    include_entity: impl Fn(crate::identity::data::EntityId) -> bool,
) -> bool {
    let state_view = context.state_view();
    for partition_id in state_view.state().partition_ids() {
        if state_view.state().get_partition(partition_id).is_none() {
            continue;
        }
        let Some(slot_count) = state_view.entity_slot_scan_count(partition_id) else {
            continue;
        };
        for slot in 0..slot_count {
            let Some(metadata) = state_view.entity_metadata_for_slot(partition_id, slot) else {
                continue;
            };
            if !include_entity(metadata.entity_id) {
                continue;
            }
            let Some(record) = context.visible_entity_record(metadata.entity_id) else {
                continue;
            };
            let Some(value) = record_authoritative_field_value(&record, field_locator) else {
                continue;
            };
            if &authoritative_aspect_value_field_comparison_key(&value) == comparison_key {
                return true;
            }
        }
    }
    false
}

pub(super) fn record_authoritative_field_value(
    record: &EntityReadRecord,
    field_locator: &AspectFieldLocator,
) -> Option<AspectValue> {
    let entry = record
        .authoritative_aspect_state
        .as_ref()?
        .get(field_locator.aspect().aspect_key())?;
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

    use super::record_authoritative_field_value;

    #[test]
    fn record_field_value_preserves_authoritative_aspect_value_family() {
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
            record_authoritative_field_value(&record, &locator),
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
