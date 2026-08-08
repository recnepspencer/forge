//! Read-only exact-field footprint of one invariant-validated mutation.

use worth_foundational::facade::{
    AspectFieldLocator, AspectKey, CanonicalFieldPath, PortableAspectPatchOperation,
};

use crate::transactions::data::{
    planned_aspect_field_locator, EntityMutationIntent, MutationIntent, RecordRef,
    RelationMutationIntent,
};

use super::ValidatedRelationalMutation;

/// How an owner-validated mutation changes fields on one existing record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ValidatedFieldMutationTarget {
    ExactField {
        record: RecordRef,
        locator: AspectFieldLocator,
    },
    WholeAspect {
        record: RecordRef,
        aspect: AspectKey,
    },
    WholeRecord {
        record: RecordRef,
    },
}

/// Relational-owner description of field changes in the exact validated plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedMutationFootprint {
    targets: Vec<ValidatedFieldMutationTarget>,
}

impl ValidatedMutationFootprint {
    /// True only when the validated plan changes this exact prior field.
    pub fn mutates_field(&self, record: &RecordRef, locator: &AspectFieldLocator) -> bool {
        self.targets.iter().any(|target| match target {
            ValidatedFieldMutationTarget::ExactField {
                record: target_record,
                locator: target_locator,
            } => target_record == record && target_locator == locator,
            ValidatedFieldMutationTarget::WholeAspect {
                record: target_record,
                aspect,
            } => target_record == record && aspect == locator.aspect().aspect_key(),
            ValidatedFieldMutationTarget::WholeRecord {
                record: target_record,
            } => target_record == record,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }
}

impl ValidatedRelationalMutation {
    /// Projects field mutation truth from Relational's merged validated plan.
    pub fn mutation_footprint(&self) -> ValidatedMutationFootprint {
        ValidatedMutationFootprint {
            targets: self
                .prepared
                .merged_plan
                .merged_intents
                .iter()
                .flat_map(targets_for_intent)
                .collect(),
        }
    }
}

fn targets_for_intent(intent: &MutationIntent) -> Vec<ValidatedFieldMutationTarget> {
    match intent {
        MutationIntent::Create(_) => Vec::new(),
        MutationIntent::Entity(intent) => entity_targets(intent),
        MutationIntent::Relation(intent) => relation_targets(intent),
    }
}

fn entity_targets(intent: &EntityMutationIntent) -> Vec<ValidatedFieldMutationTarget> {
    match intent {
        EntityMutationIntent::UpdateFields(update) => update
            .fields
            .locators()
            .cloned()
            .map(|locator| ValidatedFieldMutationTarget::ExactField {
                record: RecordRef::Entity(update.entity_id),
                locator,
            })
            .collect(),
        EntityMutationIntent::ApplyAspectPatch(update) => {
            patch_targets(RecordRef::Entity(update.entity_id), &update.aspect_patch)
        }
        EntityMutationIntent::Replace(replace) => vec![ValidatedFieldMutationTarget::WholeRecord {
            record: RecordRef::Entity(replace.entity_id),
        }],
        EntityMutationIntent::Delete(delete) => vec![ValidatedFieldMutationTarget::WholeRecord {
            record: RecordRef::Entity(delete.entity_id),
        }],
    }
}

fn relation_targets(intent: &RelationMutationIntent) -> Vec<ValidatedFieldMutationTarget> {
    match intent {
        RelationMutationIntent::ApplyAspectPatch(update) => patch_targets(
            RecordRef::Relation(update.relation_id),
            &update.aspect_patch,
        ),
        RelationMutationIntent::UpdateEndpoints(_) => Vec::new(),
        RelationMutationIntent::Delete(delete) => vec![ValidatedFieldMutationTarget::WholeRecord {
            record: RecordRef::Relation(delete.relation_id),
        }],
    }
}

fn patch_targets(
    record: RecordRef,
    patch: &worth_foundational::facade::PortableRecordAspectPatch,
) -> Vec<ValidatedFieldMutationTarget> {
    patch
        .operations()
        .iter()
        .flat_map(|operation| match operation {
            PortableAspectPatchOperation::SetWhole { basis, .. }
            | PortableAspectPatchOperation::ClearWhole { basis } => {
                vec![ValidatedFieldMutationTarget::WholeAspect {
                    record: record.clone(),
                    aspect: basis.key().clone(),
                }]
            }
            PortableAspectPatchOperation::PatchFields {
                basis,
                selected_fields,
                ..
            } => selected_fields
                .iter()
                .cloned()
                .map(|field| ValidatedFieldMutationTarget::ExactField {
                    record: record.clone(),
                    locator: planned_aspect_field_locator(
                        basis.key().clone(),
                        CanonicalFieldPath::single(field),
                    ),
                })
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use worth_foundational::facade::{
        AspectContractRevision, AspectIdentity, AspectValue, FieldKey, PortableAspectContractBasis,
        PortableAspectFieldSet, PortableRecordAspectPatch,
    };

    use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
    use crate::transactions::data::{
        ApplyEntityAspectPatchIntent, AspectFieldPatch, DeleteEntityIntent,
        UpdateEntityFieldsIntent, UpdateRelationEndpointsIntent,
    };

    use super::*;

    #[test]
    fn exact_field_target_keeps_record_aspect_and_field_distinct() {
        let record = entity(1);
        let changed = locator("estate", "frozen");
        let intent = MutationIntent::Entity(EntityMutationIntent::UpdateFields(
            UpdateEntityFieldsIntent {
                entity_id: record,
                fields: AspectFieldPatch::from_locator(changed.clone(), AspectValue::Bool(true)),
            },
        ));
        let footprint = footprint(intent);
        assert!(footprint.mutates_field(&RecordRef::Entity(record), &changed));
        assert!(!footprint.mutates_field(&RecordRef::Entity(record), &locator("estate", "note")));
        assert!(
            !footprint.mutates_field(&RecordRef::Entity(record), &locator("accounting", "frozen"))
        );
        assert!(!footprint.mutates_field(&RecordRef::Entity(entity(2)), &changed));
    }

    #[test]
    fn whole_aspect_and_whole_record_have_explicit_containment() {
        let record = entity(1);
        let whole_aspect = MutationIntent::Entity(EntityMutationIntent::ApplyAspectPatch(
            ApplyEntityAspectPatchIntent {
                entity_id: record,
                aspect_patch: PortableRecordAspectPatch::new([
                    PortableAspectPatchOperation::ClearWhole {
                        basis: basis("estate"),
                    },
                ]),
            },
        ));
        let whole_aspect_footprint = footprint(whole_aspect);
        assert!(whole_aspect_footprint
            .mutates_field(&RecordRef::Entity(record), &locator("estate", "frozen")));
        assert!(!whole_aspect_footprint
            .mutates_field(&RecordRef::Entity(record), &locator("accounting", "frozen")));

        let deleted = footprint(MutationIntent::Entity(EntityMutationIntent::Delete(
            DeleteEntityIntent { entity_id: record },
        )));
        assert!(deleted.mutates_field(
            &RecordRef::Entity(record),
            &locator("accounting", "anything")
        ));
    }

    #[test]
    fn field_patch_uses_the_validated_selected_field_set() {
        let record = entity(1);
        let intent = MutationIntent::Entity(EntityMutationIntent::ApplyAspectPatch(
            ApplyEntityAspectPatchIntent {
                entity_id: record,
                aspect_patch: PortableRecordAspectPatch::new([
                    PortableAspectPatchOperation::PatchFields {
                        basis: basis("estate"),
                        selected_fields: vec![FieldKey::new("note").unwrap()],
                        field_sets: vec![PortableAspectFieldSet::new(
                            FieldKey::new("note").unwrap(),
                            AspectValue::String("changed".into()),
                        )],
                        field_clears: Vec::new(),
                    },
                ]),
            },
        ));
        let footprint = footprint(intent);
        assert!(footprint.mutates_field(&RecordRef::Entity(record), &locator("estate", "note")));
        assert!(!footprint.mutates_field(&RecordRef::Entity(record), &locator("estate", "frozen")));
    }

    #[test]
    fn endpoint_only_mutation_changes_no_retained_field() {
        let relation = RelationId::new(PartitionId::main(), 4, 1);
        let intent = MutationIntent::Relation(RelationMutationIntent::UpdateEndpoints(
            UpdateRelationEndpointsIntent {
                relation_id: relation,
                kind_id: KindId(9),
                source: crate::transactions::data::EntityReference::Existing(entity(1)),
                target: crate::transactions::data::EntityReference::Existing(entity(2)),
            },
        ));
        assert!(footprint(intent).is_empty());
    }

    fn footprint(intent: MutationIntent) -> ValidatedMutationFootprint {
        ValidatedMutationFootprint {
            targets: targets_for_intent(&intent),
        }
    }

    fn entity(slot: u64) -> EntityId {
        EntityId::new(PartitionId::main(), slot, 1)
    }

    fn locator(aspect: &str, field: &str) -> AspectFieldLocator {
        planned_aspect_field_locator(
            AspectKey::new(aspect).unwrap(),
            CanonicalFieldPath::single(FieldKey::new(field).unwrap()),
        )
    }

    fn basis(aspect: &str) -> PortableAspectContractBasis {
        PortableAspectContractBasis::new(
            AspectKey::new(aspect).unwrap(),
            AspectIdentity(1),
            AspectContractRevision(1),
        )
    }
}
