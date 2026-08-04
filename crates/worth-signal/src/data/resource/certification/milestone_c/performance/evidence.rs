use super::super::super::digest::resource_canonical_digest;
use super::super::catalog::{
    ResourceMilestoneCPolicyPerformanceClaimId, ResourceMilestoneCPolicyScenarioEvidenceKind,
    ResourceMilestoneCPolicyScenarioId,
};
use super::super::digest_basis::{
    ResourceMilestoneCPolicyPerformanceReplayCompatibilityBasis,
    ResourceMilestoneCPolicyPerformanceReplayPolicyProvenanceBasis,
    ResourceMilestoneCPolicyPerformanceScenarioEvidenceBasis,
};
use super::super::scenario::ResourceMilestoneCPolicyScenarioMatrix;
use super::contract::ResourceMilestoneCPolicyPerformanceCloseoutRow;
use super::validation::validate_milestone_c_policy_performance;
use crate::data::error::SignalError;
use crate::data::resource::ResourceBoundaryKind;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;

impl ResourceMilestoneCPolicyPerformanceCloseoutRow {
    pub(super) fn scenario_row(
        id: ResourceMilestoneCPolicyPerformanceClaimId,
        scenario: ResourceMilestoneCPolicyScenarioId,
        matrix: &ResourceMilestoneCPolicyScenarioMatrix,
    ) -> Result<Self, SignalError> {
        let Some(row) = matrix.rows().iter().find(|row| row.id() == scenario) else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone C policy performance claim {} is missing {} scenario evidence",
                id.label(),
                scenario.label()
            )));
        };
        if !row.passed() {
            return Err(SignalError::invalid_input(format!(
                "resource milestone C policy performance claim {} requires passing scenario evidence",
                id.label()
            )));
        }
        let policy_provenance_digest = row.policy_provenance_digest().ok_or_else(|| {
            SignalError::invalid_input(format!(
                "resource milestone C policy performance claim {} requires explicit policy provenance digest",
                id.label()
            ))
        })?;
        validate_milestone_c_policy_performance(id, row.performance())?;
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneCPolicyPerformanceScenarioEvidenceBasis {
                claim: id,
                scenario,
                scenario_evidence_digest: row.evidence_digest(),
                policy_provenance_digest,
                performance: row.performance(),
            });
        Ok(Self {
            id,
            evidence_digest,
            policy_provenance_digest: policy_provenance_digest.to_owned(),
            performance: row.performance(),
            passed: true,
        })
    }

    pub(super) fn replay_descriptor_bound(
        matrix: &ResourceMilestoneCPolicyScenarioMatrix,
    ) -> Result<Self, SignalError> {
        let id = ResourceMilestoneCPolicyPerformanceClaimId::ReplayCompatibilityDescriptorBounded;
        let scenarios = [
            ResourceMilestoneCPolicyScenarioId::CompatibleDescriptorRestoreAdmitted,
            ResourceMilestoneCPolicyScenarioId::IncompatibleDescriptorRestoreDenied,
            ResourceMilestoneCPolicyScenarioId::MissingDescriptorRestoreDenied,
        ];
        let mut row_digests = Vec::with_capacity(scenarios.len());
        let mut policy_provenance_rows = Vec::with_capacity(scenarios.len());
        let mut compared_width = 0_u32;
        let mut incompatible_width = 0_u32;
        for scenario in scenarios {
            let Some(row) = matrix.rows().iter().find(|row| row.id() == scenario) else {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} is missing {} scenario evidence",
                    id.label(),
                    scenario.label()
                )));
            };
            if !row.passed()
                || row.evidence_kind()
                    == ResourceMilestoneCPolicyScenarioEvidenceKind::RegistryFreeze
                || row.performance().boundary() != ResourceBoundaryKind::PolicyCompatibility
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires passing policy-compatibility replay rows",
                    id.label()
                )));
            }
            compared_width = compared_width.saturating_add(row.performance().input_width());
            incompatible_width =
                incompatible_width.saturating_add(row.performance().denied_count());
            row_digests.push((scenario, row.evidence_digest().to_owned()));
            policy_provenance_rows.push((
                scenario,
                row.policy_provenance_digest()
                    .ok_or_else(|| {
                        SignalError::invalid_input(format!(
                            "resource milestone C policy performance claim {} requires replay policy provenance for {}",
                            id.label(),
                            scenario.label()
                        ))
                    })?
                    .to_owned(),
            ));
        }
        let performance = ResourceBoundaryPerformanceEnvelope::policy_compatibility(
            compared_width,
            incompatible_width,
        );
        let policy_provenance_digest = resource_canonical_digest(
            &ResourceMilestoneCPolicyPerformanceReplayPolicyProvenanceBasis {
                claim: id,
                row_policy_provenance: &policy_provenance_rows,
            },
        );
        let evidence_digest = resource_canonical_digest(
            &ResourceMilestoneCPolicyPerformanceReplayCompatibilityBasis {
                claim: id,
                scenario_matrix_digest: matrix.matrix_digest(),
                row_digests: &row_digests,
                policy_provenance_digest: &policy_provenance_digest,
                performance,
            },
        );
        Ok(Self {
            id,
            evidence_digest,
            policy_provenance_digest,
            performance,
            passed: true,
        })
    }
}
