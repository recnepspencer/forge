use forge_foundational::facade::{AspectValueLocator, LocatorAuthority};

use crate::aspect_wire;
use crate::history::data::{BranchId, CommitId};
use crate::merge::data::{
    MaterializedAspectValue, MaterializedAspectValueEvidence, MergeValueMaterialization,
    MergeValueSourceSide,
};
use crate::query::data::{
    query_unmasked_entity_record_digest, query_unmasked_relation_record_digest,
};
use crate::transactions::data::RecordRef;

impl super::CanonicalDigestBytes {
    pub(super) fn branch_id(&mut self, value: &BranchId) {
        self.str(&value.0);
    }

    pub(super) fn commit_id(&mut self, value: CommitId) {
        self.u64(value.0);
    }

    pub(super) fn commit_ids(&mut self, values: &[CommitId]) {
        self.usize(values.len());
        for value in values {
            self.commit_id(*value);
        }
    }

    pub(super) fn record_ref(&mut self, value: &RecordRef) {
        match value {
            RecordRef::Entity(id) => {
                self.tag(1);
                self.u32(id.partition_value());
                self.u64(id.local_slot_value());
                self.u32(id.generation_value());
            }
            RecordRef::Relation(id) => {
                self.tag(2);
                self.u32(id.partition_value());
                self.u64(id.local_slot_value());
                self.u32(id.generation_value());
            }
        }
    }

    pub(super) fn optional_record_ref(&mut self, value: Option<&RecordRef>) {
        match value {
            Some(value) => {
                self.tag(1);
                self.record_ref(value);
            }
            None => self.tag(0),
        }
    }

    pub(super) fn optional_entity_snapshot(
        &mut self,
        value: Option<&crate::storage::data::EntityReadRecord>,
    ) {
        match value {
            Some(value) => {
                self.tag(1);
                self.str(&query_unmasked_entity_record_digest(value));
            }
            None => self.tag(0),
        }
    }

    pub(super) fn optional_relation_snapshot(
        &mut self,
        value: Option<&crate::storage::data::RelationReadRecord>,
    ) {
        match value {
            Some(value) => {
                self.tag(1);
                self.str(&query_unmasked_relation_record_digest(value));
            }
            None => self.tag(0),
        }
    }

    pub(super) fn optional_materialized_value(&mut self, value: Option<&MaterializedAspectValue>) {
        match value {
            Some(value) => {
                self.tag(1);
                self.materialized_value(value);
            }
            None => self.tag(0),
        }
    }

    pub(super) fn materialized_value(&mut self, value: &MaterializedAspectValue) {
        self.merge_value_materialization(value.policy);
        match &value.evidence {
            MaterializedAspectValueEvidence::EqualityWitnessDigest(digest) => {
                self.tag(1);
                self.str(digest);
            }
            MaterializedAspectValueEvidence::PinnedVisibleAspect {
                side,
                record,
                locator,
            } => {
                self.tag(2);
                self.merge_value_source_side(*side);
                self.record_ref(record);
                self.aspect_value_locator(locator);
            }
            MaterializedAspectValueEvidence::InlineAspectValue(value) => {
                self.tag(3);
                self.aspect_value(value);
            }
        }
    }

    pub(super) fn aspect_value(&mut self, value: &forge_foundational::facade::AspectValue) {
        let bytes = aspect_wire::encode_aspect_value(value);
        self.extend_canonical_bytes(&bytes);
    }

    fn aspect_value_locator(&mut self, value: &AspectValueLocator) {
        match value {
            AspectValueLocator::WholeAspect(locator) => {
                self.tag(1);
                self.locator_authority(locator.authority());
                self.str(locator.aspect_key().as_str());
            }
            AspectValueLocator::StructField(locator) => {
                self.tag(2);
                self.locator_authority(locator.aspect().authority());
                self.str(locator.aspect().aspect_key().as_str());
                self.usize(locator.field_path().fields().len());
                for field in locator.field_path().fields() {
                    self.str(field.as_str());
                }
            }
        }
    }

    fn locator_authority(&mut self, value: LocatorAuthority) {
        match value {
            LocatorAuthority::Authoritative => self.tag(1),
            LocatorAuthority::Derived => self.tag(2),
            LocatorAuthority::Projected => self.tag(3),
            LocatorAuthority::SupportOnly => self.tag(4),
            LocatorAuthority::Planned => self.tag(5),
            LocatorAuthority::ReceiptBearing => self.tag(6),
        }
    }

    fn merge_value_materialization(&mut self, value: MergeValueMaterialization) {
        match value {
            MergeValueMaterialization::EqualityWitnessDigest => self.tag(1),
            MergeValueMaterialization::SnapshotPinnedRead => self.tag(2),
            MergeValueMaterialization::InternedCanonicalValueHandle => self.tag(3),
            MergeValueMaterialization::EagerInlineAspectValue => self.tag(4),
        }
    }

    fn merge_value_source_side(&mut self, value: MergeValueSourceSide) {
        match value {
            MergeValueSourceSide::Source => self.tag(1),
            MergeValueSourceSide::Target => self.tag(2),
            MergeValueSourceSide::Base => self.tag(3),
        }
    }
}
