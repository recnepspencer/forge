use super::super::super::digest::resource_canonical_digest;
use super::super::catalog::{
    ResourceMilestoneBPerformanceClaimId, ResourceMilestoneBScenarioEvidenceKind,
    ResourceMilestoneBScenarioId, REQUIRED_RESOURCE_MILESTONE_B_SCALAR_HOSTILE_SCENARIOS,
};
use super::super::digest_basis::{
    ResourceMilestoneBPerformanceDiagnosticsDenialBasis,
    ResourceMilestoneBPerformanceHostileDenialBasis,
    ResourceMilestoneBPerformanceScenarioEvidenceBasis,
    ResourceMilestoneBPerformanceSummaryReadEvidenceBasis,
};
use super::super::scenario_matrix::ResourceMilestoneBScenarioMatrix;
use super::contract::ResourceMilestoneBPerformanceCloseoutRow;
use super::validation::{require_performance, validate_certification_family_performance};
use crate::data::error::SignalError;
use crate::data::resource::ResourceBoundaryKind;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceCostPosture;
use crate::data::resource::ResourceDiagnosticsExpansionDenial;
use crate::data::resource::ResourceDiagnosticsSummary;
use crate::data::resource::ResourceRuntimeSummaryReadReport;

impl ResourceMilestoneBPerformanceCloseoutRow {
    pub(super) fn scenario_family(
        id: ResourceMilestoneBPerformanceClaimId,
        scenario: ResourceMilestoneBScenarioId,
        scenario_matrix: &ResourceMilestoneBScenarioMatrix,
    ) -> Result<Self, SignalError> {
        let Some(row) = scenario_matrix
            .rows()
            .iter()
            .find(|row| row.id() == scenario)
        else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} is missing {} scenario evidence",
                id.label(),
                scenario.label()
            )));
        };
        if row.evidence_kind() != ResourceMilestoneBScenarioEvidenceKind::CertificationFamily
            || !row.passed()
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} requires passing certification-family scenario evidence",
                id.label()
            )));
        }
        validate_certification_family_performance(id, scenario, row.performance())?;
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBPerformanceScenarioEvidenceBasis {
                claim: id,
                scenario,
                scenario_evidence_digest: row.evidence_digest(),
                performance: row.performance(),
            });
        Ok(Self {
            id,
            evidence_digest,
            performance: row.performance(),
            passed: true,
        })
    }

    pub(super) fn summary_read(
        report: ResourceRuntimeSummaryReadReport,
    ) -> Result<Self, SignalError> {
        let performance = report.performance();
        let id = ResourceMilestoneBPerformanceClaimId::RuntimeSummaryReadZeroColdReconstruction;
        require_performance(
            id,
            performance,
            ResourceBoundaryKind::SummaryRead,
            ResourceCostPosture::Verified,
        )?;
        if performance.input_width() != 1
            || performance.admitted_count() != 1
            || performance.denied_count() != 0
            || performance.lifecycle_transition_count() != 0
            || performance.operational_allocation_count() != 0
            || performance.retained_history_allocation_count() != 0
            || performance.diagnostics_allocation_count() != 0
            || performance.facade_report_allocation_count() != 1
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} requires zero-cold summary read evidence",
                id.label()
            )));
        }
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBPerformanceSummaryReadEvidenceBasis {
                summary: report.summary(),
                performance,
            });
        Ok(Self {
            id,
            evidence_digest,
            performance,
            passed: true,
        })
    }

    pub(super) fn diagnostics_summary(
        summary: &ResourceDiagnosticsSummary,
    ) -> Result<Self, SignalError> {
        let performance = summary.performance();
        let id =
            ResourceMilestoneBPerformanceClaimId::DiagnosticsExpansionBudgetedColdReconstruction;
        require_performance(
            id,
            performance,
            ResourceBoundaryKind::DiagnosticsExpansion,
            ResourceCostPosture::Debt,
        )?;
        let replay_width = summary.replay_reconstruction().performance().input_width();
        if !summary.expansion_budget().admits_replay_width(replay_width)
            || summary.replay_reconstruction().performance().boundary()
                != ResourceBoundaryKind::ReplayReconstruction
            || summary.replay_reconstruction().performance().cost_posture()
                != ResourceCostPosture::Debt
            || performance.diagnostics_allocation_count() != replay_width
            || performance.denied_count() != 0
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} requires budgeted diagnostics expansion evidence",
                id.label()
            )));
        }
        Ok(Self {
            id,
            evidence_digest: summary.provenance_digest().to_owned(),
            performance,
            passed: true,
        })
    }

    pub(super) fn diagnostics_denial(
        denial: ResourceDiagnosticsExpansionDenial,
    ) -> Result<Self, SignalError> {
        let performance = denial.performance();
        let id = ResourceMilestoneBPerformanceClaimId::DiagnosticsExpansionBudgetDenial;
        require_performance(
            id,
            performance,
            ResourceBoundaryKind::DiagnosticsExpansion,
            ResourceCostPosture::DeniedFallback,
        )?;
        if denial.budget().denial_class(
            denial.replay_reconstruction_width(),
            denial.forensic_reconstruction_width(),
        ) != Some(denial.class())
            || performance.admitted_count() != 0
            || performance.denied_count() != 1
            || performance.diagnostics_allocation_count() != 0
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} requires budget denial evidence",
                id.label()
            )));
        }
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBPerformanceDiagnosticsDenialBasis {
                class: denial.class(),
                budget: denial.budget(),
                replay_reconstruction_width: denial.replay_reconstruction_width(),
                performance,
            });
        Ok(Self {
            id,
            evidence_digest,
            performance,
            passed: true,
        })
    }

    pub(super) fn hostile_completion_denials(
        scenario_matrix: &ResourceMilestoneBScenarioMatrix,
    ) -> Result<Self, SignalError> {
        let id = ResourceMilestoneBPerformanceClaimId::HostileCompletionDenialsScalarBounded;
        let mut hostile_digests =
            Vec::with_capacity(REQUIRED_RESOURCE_MILESTONE_B_SCALAR_HOSTILE_SCENARIOS.len());
        let mut total_denied = 0_u32;
        for scenario in REQUIRED_RESOURCE_MILESTONE_B_SCALAR_HOSTILE_SCENARIOS {
            let Some(row) = scenario_matrix
                .rows()
                .iter()
                .find(|row| row.id() == scenario)
            else {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone B performance claim {} is missing {} evidence",
                    id.label(),
                    scenario.label()
                )));
            };
            let performance = row.performance();
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::CompletionAdmission,
                ResourceCostPosture::Verified,
            )?;
            if row.evidence_kind()
                != ResourceMilestoneBScenarioEvidenceKind::HostileCompletionDenial
                || !row.passed()
                || performance.input_width() != 1
                || performance.admitted_count() != 0
                || performance.denied_count() != 1
                || performance.lifecycle_transition_count() != 0
                || performance.operational_allocation_count() != 0
                || performance.diagnostics_allocation_count() != 0
                || performance.facade_report_allocation_count() != 1
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone B performance claim {} requires scalar hostile completion denial evidence",
                    id.label()
                )));
            }
            total_denied = total_denied.saturating_add(performance.denied_count());
            hostile_digests.push((scenario, row.evidence_digest().to_owned()));
        }
        let performance =
            ResourceBoundaryPerformanceEnvelope::completion_admission(0, total_denied, 0);
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBPerformanceHostileDenialBasis {
                scenario_matrix_digest: scenario_matrix.matrix_digest(),
                hostile_digests: &hostile_digests,
                performance,
            });
        Ok(Self {
            id,
            evidence_digest,
            performance,
            passed: true,
        })
    }
}
