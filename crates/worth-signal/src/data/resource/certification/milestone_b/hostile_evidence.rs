use super::super::digest::resource_canonical_digest;
use super::catalog::{
    ResourceMilestoneBScenarioId, REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS,
};
use super::digest_basis::{
    ResourceMilestoneBHostileScenarioEvidenceDigestBasis,
    ResourceMilestoneBHostileScenarioEvidenceRowDigestBasis,
};
use crate::data::error::SignalError;
use crate::data::resource::CompletionDenialClass;
use crate::data::resource::DeniedResourceCompletion;
use crate::data::resource::ResourceBoundaryKind;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceCompletionAdmissionReport;
use crate::data::resource::ResourceCompletionBatchAdmissionReport;
use crate::data::resource::ResourceDensityStrategy;
use serde::Serialize;

pub const RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-b-hostile-scenario-evidence-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBHostileScenarioEvidenceRow {
    id: ResourceMilestoneBScenarioId,
    expected_denial_class: CompletionDenialClass,
    denied_completion: DeniedResourceCompletion,
    performance: ResourceBoundaryPerformanceEnvelope,
    evidence_digest: String,
}

impl ResourceMilestoneBHostileScenarioEvidenceRow {
    fn from_completion_denial_report(
        id: ResourceMilestoneBScenarioId,
        report: ResourceCompletionAdmissionReport,
    ) -> Result<Self, SignalError> {
        let Some(expected_denial_class) = id.completion_denial_class() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is not a hostile completion denial scenario",
                id.label()
            )));
        };
        if report.admitted_completion().is_some() {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires denied completion evidence",
                id.label()
            )));
        }
        let Some(denied_completion) = report.denied_completion() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is missing denied completion evidence",
                id.label()
            )));
        };
        if denied_completion.class() != expected_denial_class {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires {expected_denial_class:?} denial evidence, got {:?}",
                id.label(),
                denied_completion.class()
            )));
        }
        let performance = report.performance();
        if performance.boundary() != ResourceBoundaryKind::CompletionAdmission {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires completion admission performance evidence",
                id.label()
            )));
        }
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBHostileScenarioEvidenceRowDigestBasis {
                id,
                expected_denial_class,
                denied_completion,
                performance,
            });
        Ok(Self {
            id,
            expected_denial_class,
            denied_completion,
            performance,
            evidence_digest,
        })
    }

    fn from_completion_batch_denial_report(
        id: ResourceMilestoneBScenarioId,
        report: &ResourceCompletionBatchAdmissionReport,
    ) -> Result<Self, SignalError> {
        let Some(expected_denial_class) = id.completion_denial_class() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is not a hostile completion denial scenario",
                id.label()
            )));
        };
        let mut matches = report
            .denied_completions()
            .iter()
            .copied()
            .filter(|denied| denied.class() == expected_denial_class);
        let Some(denied_completion) = matches.next() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is missing {:?} denial evidence in completion batch",
                id.label(),
                expected_denial_class
            )));
        };
        if matches.next().is_some() {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires exactly one {:?} denial entry in completion batch evidence",
                id.label(),
                expected_denial_class
            )));
        }
        let performance = report.performance();
        if performance.boundary() != ResourceBoundaryKind::CompletionBatchAdmission {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires completion batch admission performance evidence",
                id.label()
            )));
        }
        if performance.input_width() != 4
            || performance.admitted_count() != 1
            || performance.denied_count() != 3
            || performance.lifecycle_transition_count() != 1
            || performance.operational_allocation_count() != 3
            || performance.retained_history_allocation_count() != 0
            || performance.diagnostics_allocation_count() != 4
            || performance.facade_report_allocation_count() != 1
            || performance.density_strategy() != ResourceDensityStrategy::BurstySortedDeduplicated
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires hostile mixed batch denial evidence rather than an arbitrary completion batch",
                id.label()
            )));
        }
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBHostileScenarioEvidenceRowDigestBasis {
                id,
                expected_denial_class,
                denied_completion,
                performance,
            });
        Ok(Self {
            id,
            expected_denial_class,
            denied_completion,
            performance,
            evidence_digest,
        })
    }

    pub fn id(&self) -> ResourceMilestoneBScenarioId {
        self.id
    }

    pub fn expected_denial_class(&self) -> CompletionDenialClass {
        self.expected_denial_class
    }

    pub fn denied_completion(&self) -> DeniedResourceCompletion {
        self.denied_completion
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBHostileScenarioEvidence {
    schema_version: String,
    rows: Vec<ResourceMilestoneBHostileScenarioEvidenceRow>,
    evidence_digest: String,
}

impl ResourceMilestoneBHostileScenarioEvidence {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn rows(&self) -> &[ResourceMilestoneBHostileScenarioEvidenceRow] {
        &self.rows
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub(super) fn row_for(
        &self,
        id: ResourceMilestoneBScenarioId,
    ) -> Option<&ResourceMilestoneBHostileScenarioEvidenceRow> {
        self.rows.iter().find(|row| row.id() == id)
    }
}

pub fn resource_milestone_b_hostile_scenario_evidence(
    late_superseded_completion: ResourceCompletionAdmissionReport,
    late_cancelled_completion: ResourceCompletionAdmissionReport,
    late_timed_out_completion: ResourceCompletionAdmissionReport,
    malformed_completion: ResourceCompletionAdmissionReport,
    completion_pressure_batch: &ResourceCompletionBatchAdmissionReport,
) -> Result<ResourceMilestoneBHostileScenarioEvidence, SignalError> {
    let rows = vec![
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_denial_report(
            ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected,
            late_superseded_completion,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_denial_report(
            ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected,
            late_cancelled_completion,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_denial_report(
            ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected,
            late_timed_out_completion,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_denial_report(
            ResourceMilestoneBScenarioId::MalformedCompletionRejected,
            malformed_completion,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_batch_denial_report(
            ResourceMilestoneBScenarioId::DuplicateCompletionRejected,
            completion_pressure_batch,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_batch_denial_report(
            ResourceMilestoneBScenarioId::ContradictoryCompletionRejected,
            completion_pressure_batch,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_batch_denial_report(
            ResourceMilestoneBScenarioId::UnknownRequestCompletionRejected,
            completion_pressure_batch,
        )?,
    ];
    let evidence_digest =
        resource_canonical_digest(&ResourceMilestoneBHostileScenarioEvidenceDigestBasis {
            schema_version: RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION,
            required_scenarios: &REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS,
            rows: &rows,
        });
    Ok(ResourceMilestoneBHostileScenarioEvidence {
        schema_version: RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION.to_owned(),
        rows,
        evidence_digest,
    })
}
