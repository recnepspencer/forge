use crate::authority::mutation::canonical_deltas::CanonicalRecordAspectDelta;
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
) {
    match delta.target {
        RecordRef::Entity(entity_id) => {
            let slot = slot_of::<EntityRecordKind>(&entity_id);
            let partition = staged.get_partition_mut(partition_of::<EntityRecordKind>(&entity_id));
            let versions = &mut partition.entity_arena.aspect_versions[slot];
            for aspect in delta.changed_aspects.iter() {
                if let crate::symbols::data::InternedString::Raw(raw) = &aspect.0 {
                    versions.insert(symbols.intern(raw), version_id.0);
                }
            }
        }
        RecordRef::Relation(relation_id) => {
            let slot = slot_of::<RelationRecordKind>(&relation_id);
            let partition =
                staged.get_partition_mut(partition_of::<RelationRecordKind>(&relation_id));
            let versions = &mut partition.relation_arena.aspect_versions[slot];
            for aspect in delta.changed_aspects.iter() {
                if let crate::symbols::data::InternedString::Raw(raw) = &aspect.0 {
                    versions.insert(symbols.intern(raw), version_id.0);
                }
            }
        }
    }
}
