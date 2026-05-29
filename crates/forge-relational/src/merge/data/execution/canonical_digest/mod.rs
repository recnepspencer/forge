mod diagnostic_terms;
mod enum_terms;
mod plan_terms;
mod schema_terms;
mod value_terms;

use sha2::{Digest, Sha256};

use crate::aspect_wire;
use crate::history::data::{BranchId, CommitId};
use crate::merge::data::{
    BoundExecutableMergeRecordPlan, ExecutedMergeRecordDiagnosticRow,
    MergeSchemaSnapshotDigestBasis, VisibleMergeRecord,
};

pub(crate) fn compiled_executable_plan_digest(
    target_branch: &BranchId,
    source_branch: &BranchId,
    merge_intent: crate::merge::data::MergeIntent,
    parent_order: &[CommitId],
    record_plans: &[BoundExecutableMergeRecordPlan],
) -> String {
    let mut bytes = CanonicalDigestBytes::new("merge.executable-plan.v1");
    bytes.branch_id(target_branch);
    bytes.branch_id(source_branch);
    bytes.merge_intent(merge_intent);
    bytes.commit_ids(parent_order);
    bytes.executable_record_plans(record_plans);
    bytes.finish()
}

pub(crate) fn schema_snapshot_digest(schema_snapshot: &MergeSchemaSnapshotDigestBasis) -> String {
    let mut bytes = CanonicalDigestBytes::new("merge.schema-snapshot.v1");
    bytes.optional_schema_id(schema_snapshot.authoritative_schema_id.as_ref());
    bytes.optional_schema_version_id(schema_snapshot.authoritative_schema_version_id);
    bytes.str(&schema_snapshot.registry_digest);
    bytes.schema_kind_snapshots(&schema_snapshot.touched_kinds);
    bytes.finish()
}

pub(crate) fn equality_witness_digest(record: &VisibleMergeRecord) -> String {
    let mut bytes = CanonicalDigestBytes::new("merge.equality-witness.v1");
    bytes.record_ref(&record.record_ref);
    bytes.optional_entity_snapshot(record.source_entity.as_ref());
    bytes.optional_entity_snapshot(record.target_entity.as_ref());
    bytes.optional_relation_snapshot(record.source_relation.as_ref());
    bytes.optional_relation_snapshot(record.target_relation.as_ref());
    bytes.finish()
}

pub(crate) fn merge_execution_diagnostics_digest(
    executed_records: &[ExecutedMergeRecordDiagnosticRow],
) -> String {
    let mut bytes = CanonicalDigestBytes::new("merge.execution-diagnostics.v1");
    bytes.executed_record_rows(executed_records);
    bytes.finish()
}

struct CanonicalDigestBytes {
    bytes: Vec<u8>,
}

impl CanonicalDigestBytes {
    fn new(domain: &'static str) -> Self {
        let mut bytes = Self { bytes: Vec::new() };
        bytes.str(domain);
        bytes
    }

    fn finish(self) -> String {
        let digest = Sha256::digest(self.bytes);
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    pub(super) fn tag(&mut self, tag: u8) {
        self.bytes.push(tag);
    }

    pub(super) fn u32(&mut self, value: u32) {
        aspect_wire::encode_u32(&mut self.bytes, value);
    }

    pub(super) fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub(super) fn u128(&mut self, value: u128) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(super) fn usize(&mut self, value: usize) {
        self.u64(value as u64);
    }

    pub(super) fn str(&mut self, value: &str) {
        aspect_wire::encode_string(&mut self.bytes, value);
    }

    pub(super) fn extend_canonical_bytes(&mut self, value: &[u8]) {
        self.usize(value.len());
        self.bytes.extend_from_slice(value);
    }
}
