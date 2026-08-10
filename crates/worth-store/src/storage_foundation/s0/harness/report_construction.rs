use super::super::artifacts::{
    BackendCapabilityMatrix, S0ArtifactEnvelopeMetadata, S0NondeterministicMetadata,
};
use super::super::deferred::DeferredPhysicalGuaranteeMap;
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::super::terminology::{ReleaseClaimReport, TerminologyRiskReport};
use super::backend_tier_fence_row::backend_tier_fence_row;
use super::compile_time_fixture_row::compile_time_fixture_row;
use super::deferred_validation_row::deferred_validation_row;
use super::digest::{stable_digest, HarnessMaturityDigestBasis};
use super::fixtures::S1CompileTimeBoundaryFixture;
use super::maturity::{EvidenceBundleReadiness, HarnessMaturityLevel};
use super::milestone_completeness_row::milestone_completeness_row;
use super::report::HarnessMaturityReport;
use super::row::HarnessMaturityRow;
use super::stale_handoff_row::stale_handoff_row;
use super::terminology_claim_gate_row::terminology_claim_gate_row;
use super::validation::{
    ensure_required_harness_subsystems, reject_duplicate_rows, require_non_empty,
    S0HarnessMaturityBuildRejection,
};

impl HarnessMaturityReport {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<HarnessMaturityRow>,
        evidence_bundle_readiness: EvidenceBundleReadiness,
    ) -> Result<Self, S0HarnessMaturityBuildRejection> {
        let source_revision = require_non_empty(source_revision)
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?;
        let generated_by = require_non_empty(generated_by)
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?;
        rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        reject_duplicate_rows(&rows)?;
        ensure_required_harness_subsystems(&rows)?;
        let deterministic_digest = stable_digest(&HarnessMaturityDigestBasis {
            schema_version: super::super::artifacts::S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::HarnessMaturityReport,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            readiness: evidence_bundle_readiness,
            rows: &rows,
        })
        .map_err(|_| S0HarnessMaturityBuildRejection::InvalidDigest)?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::HarnessMaturityReport,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            rows,
            evidence_bundle_readiness,
        })
    }

    pub fn baseline_for_s1(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        backend_matrix: &BackendCapabilityMatrix,
        deferred_map: &DeferredPhysicalGuaranteeMap,
        terminology_report: &TerminologyRiskReport,
        release_claim_report: &ReleaseClaimReport,
        milestone_row_count: u64,
        required_milestone_row_count: u64,
        available_fixtures: &[S1CompileTimeBoundaryFixture],
    ) -> Result<Self, S0HarnessMaturityBuildRejection> {
        let rows = build_baseline_rows(BaselineRowInputs {
            backend_matrix,
            deferred_map,
            terminology_report,
            release_claim_report,
            milestone_row_count,
            required_milestone_row_count,
            available_fixtures,
        })?;
        let readiness = baseline_readiness(&rows);
        Self::new(
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            rows,
            readiness,
        )
    }
}

struct BaselineRowInputs<'a> {
    backend_matrix: &'a BackendCapabilityMatrix,
    deferred_map: &'a DeferredPhysicalGuaranteeMap,
    terminology_report: &'a TerminologyRiskReport,
    release_claim_report: &'a ReleaseClaimReport,
    milestone_row_count: u64,
    required_milestone_row_count: u64,
    available_fixtures: &'a [S1CompileTimeBoundaryFixture],
}

fn build_baseline_rows(
    inputs: BaselineRowInputs<'_>,
) -> Result<Vec<HarnessMaturityRow>, S0HarnessMaturityBuildRejection> {
    Ok(vec![
        terminology_claim_gate_row(inputs.terminology_report, inputs.release_claim_report)?,
        backend_tier_fence_row(inputs.backend_matrix)?,
        deferred_validation_row(inputs.deferred_map)?,
        milestone_completeness_row(
            inputs.milestone_row_count,
            inputs.required_milestone_row_count,
        )?,
        compile_time_fixture_row(inputs.available_fixtures)?,
        stale_handoff_row(
            inputs.backend_matrix,
            inputs.deferred_map,
            inputs.terminology_report,
        )?,
    ])
}

fn baseline_readiness(rows: &[HarnessMaturityRow]) -> EvidenceBundleReadiness {
    if rows
        .iter()
        .filter(|row| {
            row.required_for_sequences
                .iter()
                .any(|sequence| sequence.as_str() == "S1")
        })
        .all(|row| row.maturity_level >= HarnessMaturityLevel::Exists)
    {
        EvidenceBundleReadiness::ReadyForS1Planning
    } else {
        EvidenceBundleReadiness::Insufficient
    }
}
