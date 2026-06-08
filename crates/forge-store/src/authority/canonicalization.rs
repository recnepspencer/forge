use crate::{
    authority::{CanonicalizedCommitEnvelope, RawRuntimeCommitEnvelope},
    evidence::CanonicalizationMetrics,
    failure::{StoreError, StoreErrorKind},
};
use forge_relational::facade::replay::CanonicalCommitEnvelope;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub const CURRENT_CANONICALIZATION_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDigest(String);

impl CanonicalDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub(crate) fn digest_from_string(value: String) -> CanonicalDigest {
    CanonicalDigest(value)
}

#[derive(Debug, Serialize)]
struct DigestEnvelope<'a> {
    canonicalization_version: u32,
    envelope: CanonicalCommitExport<'a>,
}

#[derive(Debug, Serialize)]
struct CanonicalCommitExport<'a> {
    commit: &'a forge_relational::facade::history::CommitReference,
    branch_context: &'a forge_relational::facade::history::BranchId,
    authority_kind: &'a forge_relational::facade::replay::CanonicalCommitAuthorityKind,
    strategy_artifacts:
        &'a Option<forge_relational::facade::commit_strategies::StrategyCommitArtifactBundle>,
    merge_execution_authority:
        &'a Option<forge_relational::facade::transactions::PublishedMergeExecutionAuthority>,
    merge_parent_branches: &'a [forge_relational::facade::history::BranchId],
    merge_base_commits: &'a [forge_relational::facade::history::CommitId],
    schema_version: &'a forge_relational::facade::schema::SchemaVersionId,
    schema_authority: &'a forge_relational::facade::schema::SchemaAuthoritySnapshot,
    merged_plan: &'a forge_relational::facade::transactions::MergedCommitPlan,
    patch: &'a forge_relational::facade::publication::PublishedAuthoritativePatchEnvelope,
    diagnostics_summary: &'a forge_relational::facade::diagnostics::RelationalDiagnosticArtifact,
    lineage_event_ids: &'a [u64],
    lineage_events: &'a [forge_relational::facade::lineage::LineageEventRecord],
    lineage_digest_basis: &'a forge_relational::facade::lineage::LineageDigestBasis,
    event_batch_digest_basis: &'a forge_relational::facade::lineage::LineageEventBatchDigestBasis,
    decision_log_digest_basis: &'a forge_relational::facade::lineage::LineageDecisionLogDigestBasis,
    lineage_artifact_counters: forge_relational::facade::lineage::LineageArtifactCounters,
    derived_index_artifacts: &'a forge_relational::facade::indexes::DerivedIndexArtifacts,
    schema_transition: &'a Option<forge_relational::facade::schema::SchemaTransitionArtifact>,
    schema_continuation_descriptor:
        &'a Option<forge_relational::facade::schema::SchemaContinuationDescriptor>,
    schema_reconciliation_descriptor:
        &'a Option<forge_relational::facade::schema::SchemaReconciliationDescriptor>,
    descriptor_semantics_version: &'a forge_relational::facade::schema::DescriptorSemanticsVersion,
}

pub fn canonicalize(
    raw: RawRuntimeCommitEnvelope,
    canonicalization_version: u32,
) -> Result<CanonicalizedCommitEnvelope, StoreError> {
    if canonicalization_version != CURRENT_CANONICALIZATION_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::UnsupportedCanonicalizationVersion,
            format!(
                "forge-store only supports canonicalization version {}",
                CURRENT_CANONICALIZATION_VERSION
            ),
        ));
    }

    let envelope = raw.into_inner();
    if envelope.branch_context != envelope.commit.branch_id {
        return Err(StoreError::new(
            StoreErrorKind::NonCanonicalEnvelope,
            "branch_context must match commit.branch_id",
        ));
    }

    let mut deduped = envelope.commit.parents.clone();
    deduped.sort();
    deduped.dedup();
    let duplicate_collapse_count =
        envelope.commit.parents.len().saturating_sub(deduped.len()) as u64;
    if deduped.len() != envelope.commit.parents.len() {
        return Err(StoreError::new(
            StoreErrorKind::NonCanonicalEnvelope,
            "authoritative ordered parent list may not contain duplicates",
        ));
    }

    let digest = stable_digest(&DigestEnvelope {
        canonicalization_version,
        envelope: canonical_export(&envelope)?,
    })?;

    Ok(CanonicalizedCommitEnvelope::new(
        envelope,
        CanonicalDigest(digest),
        canonicalization_version,
        CanonicalizationMetrics {
            canonicalization_item_count: deduped.len() as u64,
            canonicalization_duplicate_collapse_count: duplicate_collapse_count,
        },
    ))
}

pub fn digest_envelope(
    envelope: &CanonicalCommitEnvelope,
    canonicalization_version: u32,
) -> Result<CanonicalDigest, StoreError> {
    Ok(CanonicalDigest(stable_digest(&DigestEnvelope {
        canonicalization_version,
        envelope: canonical_export(envelope)?,
    })?))
}

fn canonical_export(
    envelope: &CanonicalCommitEnvelope,
) -> Result<CanonicalCommitExport<'_>, StoreError> {
    Ok(CanonicalCommitExport {
        commit: &envelope.commit,
        branch_context: &envelope.branch_context,
        authority_kind: &envelope.authority_kind,
        strategy_artifacts: &envelope.strategy_artifacts,
        merge_execution_authority: &envelope.merge_execution_authority,
        merge_parent_branches: &envelope.merge_parent_branches,
        merge_base_commits: &envelope.merge_base_commits,
        schema_version: &envelope.schema_version,
        schema_authority: &envelope.schema_authority,
        merged_plan: &envelope.merged_plan,
        patch: &envelope.patch,
        diagnostics_summary: &envelope.diagnostics_summary,
        lineage_event_ids: envelope.lineage_event_ids(),
        lineage_events: envelope.lineage_events(),
        lineage_digest_basis: envelope.lineage_digest_basis(),
        event_batch_digest_basis: envelope.event_batch_digest_basis(),
        decision_log_digest_basis: envelope.decision_log_digest_basis(),
        lineage_artifact_counters: envelope.lineage_artifact_counters(),
        derived_index_artifacts: envelope.derived_index_artifacts(),
        schema_transition: &envelope.schema_transition,
        schema_continuation_descriptor: &envelope.schema_continuation_descriptor,
        schema_reconciliation_descriptor: &envelope.schema_reconciliation_descriptor,
        descriptor_semantics_version: &envelope.descriptor_semantics_version,
    })
}

fn stable_digest<T: Serialize>(value: &T) -> Result<String, StoreError> {
    let mut hasher = Sha256::new();
    serde_json::to_writer(&mut HashWriter(&mut hasher), value)?;
    Ok(format!("{:x}", hasher.finalize()))
}

struct HashWriter<'a, D>(&'a mut D);

impl<D: Digest> std::io::Write for HashWriter<'_, D> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
