use super::super::digest::resource_canonical_digest;
use super::super::family::{
    ResourceCertificationBundle, ResourceCertificationFamily, ResourceCertificationRecord,
    REQUIRED_RESOURCE_CERTIFICATION_FAMILIES,
};
use super::catalog::{
    ResourceMilestoneBScenarioEvidenceKind, ResourceMilestoneBScenarioId,
    REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS,
};
use super::digest_basis::ResourceMilestoneBScenarioMatrixDigestBasis;
use super::hostile_evidence::{
    ResourceMilestoneBHostileScenarioEvidence, ResourceMilestoneBHostileScenarioEvidenceRow,
};
use crate::data::error::SignalError;
use crate::data::resource::CompletionDenialClass;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

pub const RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-b-scenario-matrix-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBScenarioRow {
    id: ResourceMilestoneBScenarioId,
    evidence_kind: ResourceMilestoneBScenarioEvidenceKind,
    certification_family: Option<ResourceCertificationFamily>,
    completion_denial_class: Option<CompletionDenialClass>,
    evidence_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
    passed: bool,
}

impl ResourceMilestoneBScenarioRow {
    fn from_record(
        id: ResourceMilestoneBScenarioId,
        record: &ResourceCertificationRecord,
    ) -> Result<Self, SignalError> {
        let Some(expected_family) = id.certification_family() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is not a certification-family scenario",
                id.label()
            )));
        };
        if record.family() != expected_family {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires {expected_family:?} evidence, got {:?}",
                id.label(),
                record.family()
            )));
        }
        if !record.passed() {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires passing certification evidence",
                id.label()
            )));
        }
        Ok(Self {
            id,
            evidence_kind: ResourceMilestoneBScenarioEvidenceKind::CertificationFamily,
            certification_family: Some(expected_family),
            completion_denial_class: None,
            evidence_digest: record.evidence_digest().to_owned(),
            performance: record.performance(),
            passed: true,
        })
    }

    fn from_hostile_completion_denial(
        id: ResourceMilestoneBScenarioId,
        evidence: &ResourceMilestoneBHostileScenarioEvidenceRow,
    ) -> Result<Self, SignalError> {
        let Some(expected_denial_class) = id.completion_denial_class() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is not a hostile completion denial scenario",
                id.label()
            )));
        };
        if evidence.id() != id {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} cannot use {:?} hostile evidence",
                id.label(),
                evidence.id()
            )));
        }
        if evidence.expected_denial_class() != expected_denial_class {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires {expected_denial_class:?} hostile evidence",
                id.label()
            )));
        }
        Ok(Self {
            id,
            evidence_kind: ResourceMilestoneBScenarioEvidenceKind::HostileCompletionDenial,
            certification_family: None,
            completion_denial_class: Some(expected_denial_class),
            evidence_digest: evidence.evidence_digest().to_owned(),
            performance: evidence.performance(),
            passed: true,
        })
    }

    pub fn id(&self) -> ResourceMilestoneBScenarioId {
        self.id
    }

    pub fn evidence_kind(&self) -> ResourceMilestoneBScenarioEvidenceKind {
        self.evidence_kind
    }

    pub fn certification_family(&self) -> Option<ResourceCertificationFamily> {
        self.certification_family
    }

    pub fn completion_denial_class(&self) -> Option<CompletionDenialClass> {
        self.completion_denial_class
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBScenarioMatrixSummary {
    required_scenario_count: u32,
    certified_scenario_count: u32,
    failed_scenario_count: u32,
    bundle_digest: String,
}

impl ResourceMilestoneBScenarioMatrixSummary {
    pub fn required_scenario_count(&self) -> u32 {
        self.required_scenario_count
    }

    pub fn certified_scenario_count(&self) -> u32 {
        self.certified_scenario_count
    }

    pub fn failed_scenario_count(&self) -> u32 {
        self.failed_scenario_count
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBScenarioMatrix {
    schema_version: String,
    bundle_digest: String,
    rows: Vec<ResourceMilestoneBScenarioRow>,
    summary: ResourceMilestoneBScenarioMatrixSummary,
    matrix_digest: String,
    passed: bool,
}

impl ResourceMilestoneBScenarioMatrix {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn rows(&self) -> &[ResourceMilestoneBScenarioRow] {
        &self.rows
    }

    pub fn summary(&self) -> &ResourceMilestoneBScenarioMatrixSummary {
        &self.summary
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

pub fn resource_milestone_b_scenario_matrix(
    bundle: &ResourceCertificationBundle,
    hostile_evidence: &ResourceMilestoneBHostileScenarioEvidence,
) -> Result<ResourceMilestoneBScenarioMatrix, SignalError> {
    bundle.ensure_passed()?;
    let bundle_summary = bundle.summary();
    let required_family_count = REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32;
    if bundle.records().len() != REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len()
        || bundle_summary.required_family_count() != required_family_count
        || bundle_summary.passed_family_count() != required_family_count
        || bundle_summary.failed_family_count() != 0
        || bundle_summary.missing_family_count() != 0
        || bundle_summary.duplicate_family_count() != 0
        || !bundle.failures().is_empty()
    {
        return Err(SignalError::invalid_input(
            "resource milestone B scenario matrix requires one passing record for every required family",
        ));
    }

    let records_by_family = bundle
        .records()
        .iter()
        .map(|record| (record.family(), record))
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut rows = Vec::with_capacity(REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len());
    for scenario in REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS {
        if !seen.insert(scenario) {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is duplicated",
                scenario.label()
            )));
        }
        if let Some(family) = scenario.certification_family() {
            let Some(record) = records_by_family.get(&family) else {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone B scenario {} is missing {family:?} evidence",
                    scenario.label()
                )));
            };
            rows.push(ResourceMilestoneBScenarioRow::from_record(
                scenario, record,
            )?);
            continue;
        }
        let Some(hostile_row) = hostile_evidence.row_for(scenario) else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is missing hostile completion evidence",
                scenario.label()
            )));
        };
        rows.push(
            ResourceMilestoneBScenarioRow::from_hostile_completion_denial(scenario, hostile_row)?,
        );
    }

    let summary = ResourceMilestoneBScenarioMatrixSummary {
        required_scenario_count: REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32,
        certified_scenario_count: rows.len() as u32,
        failed_scenario_count: rows.iter().filter(|row| !row.passed()).count() as u32,
        bundle_digest: bundle.bundle_digest().to_owned(),
    };
    if summary.certified_scenario_count != summary.required_scenario_count
        || summary.failed_scenario_count != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone B scenario matrix did not cover every required scenario",
        ));
    }
    let matrix_digest = resource_canonical_digest(&ResourceMilestoneBScenarioMatrixDigestBasis {
        schema_version: RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION,
        required_scenarios: &REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS,
        bundle_digest: bundle.bundle_digest(),
        summary: &summary,
        rows: &rows,
    });

    Ok(ResourceMilestoneBScenarioMatrix {
        schema_version: RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION.to_owned(),
        bundle_digest: bundle.bundle_digest().to_owned(),
        rows,
        summary,
        matrix_digest,
        passed: true,
    })
}
