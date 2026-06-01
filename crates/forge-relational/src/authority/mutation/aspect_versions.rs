use crate::authority::mutation::canonical_deltas::{
    CanonicalDeltaError, CanonicalRecordAspectDelta,
};
use crate::identity::data::VersionId;
use crate::storage::logic::state::{partition_of, slot_of, EntityRecordKind, RelationRecordKind};
use crate::storage::overlay::WorkingState;
use crate::symbols::data::StringInterner;
use crate::transactions::data::RecordRef;

pub(super) fn write_aspect_versions_for_delta(
    staged: &mut WorkingState,
    delta: &CanonicalRecordAspectDelta,
    version_id: VersionId,
    symbols: &mut StringInterner,
) -> Result<(), CanonicalDeltaError> {
    match delta.target {
        RecordRef::Entity(entity_id) => {
            let slot = slot_of::<EntityRecordKind>(&entity_id);
            let partition = staged.get_partition_mut(partition_of::<EntityRecordKind>(&entity_id));
            let versions = &mut partition.entity_arena.aspect_versions[slot];
            for aspect in delta.changed_aspects.iter() {
                versions.insert(symbols.intern(aspect.as_str()), version_id.0);
            }
        }
        RecordRef::Relation(relation_id) => {
            let slot = slot_of::<RelationRecordKind>(&relation_id);
            let partition =
                staged.get_partition_mut(partition_of::<RelationRecordKind>(&relation_id));
            let versions = &mut partition.relation_arena.aspect_versions[slot];
            for aspect in delta.changed_aspects.iter() {
                versions.insert(symbols.intern(aspect.as_str()), version_id.0);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::authority::mutation::canonical_deltas::CanonicalRecordAspectDelta;
    use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
    use crate::identity::data::{EntityId, KindId, PartitionId, VersionId};
    use crate::publication::patch::data::{ordered_aspect_keys, RecordStructuralChange};
    use crate::schema::data::AspectContractPlanRevision;
    use crate::storage::overlay::WorkingState;
    use crate::storage::substrate::{EntityRecordKind, SlotInit};
    use crate::symbols::data::StringInterner;
    use crate::transactions::data::RecordRef;
    use forge_foundational::facade::AspectKey;

    use super::write_aspect_versions_for_delta;
    #[test]
    fn foundational_aspect_keys_are_interned_for_version_storage() {
        let mut state = WorkingState::new(
            BTreeMap::new(),
            AdjacencyPolicy {
                backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                small_degree_inline_capacity: 4,
            },
        );
        {
            let partition = state.get_partition_mut(PartitionId(1));
            partition
                .entity_arena
                .push_slot(SlotInit::<EntityRecordKind> {
                    partition_id: PartitionId(1),
                    kind_id: KindId(1),
                    version_id: VersionId(1),
                    extra: Default::default(),
                });
        }
        let mut symbols = StringInterner::default();
        let delta = CanonicalRecordAspectDelta {
            target: RecordRef::Entity(EntityId::new(PartitionId(1), 0, 1)),
            kind_id: KindId(1),
            plan_revision: AspectContractPlanRevision(1),
            structural_change: RecordStructuralChange::Created,
            changed_aspects: ordered_aspect_keys([AspectKey::new("name").unwrap()]),
            evaluated_bindings: smallvec::SmallVec::new(),
            contains_opaque_aspect: false,
        };

        write_aspect_versions_for_delta(&mut state, &delta, VersionId(2), &mut symbols)
            .expect("foundational aspect keys should be interned for version storage");

        let recorded = state
            .get_partition_mut(PartitionId(1))
            .entity_arena
            .aspect_versions_at(0)
            .expect("entity slot versions");
        let name_symbol = symbols.intern("name");
        assert_eq!(recorded.get(&name_symbol), Some(&2));
    }
}
