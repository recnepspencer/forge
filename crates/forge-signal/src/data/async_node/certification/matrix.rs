use serde::{Deserialize, Serialize};

use crate::data::async_node::{
    AsyncKeyedNodeCapabilityEquivalenceReport, AsyncKeyedNodeHistoricalParityReport,
    AsyncNodeAdmissionClass, AsyncNodeCapabilityEquivalenceReport, AsyncNodeConditionBlockClass,
    AsyncNodeGateStateReport, AsyncNodeHierarchyCancellationReport,
    AsyncNodeHierarchyHistoricalParityReport, AsyncNodeHierarchyReplaySummary,
    AsyncNodeHistoricalParityReport, AsyncNodeRequestAdmissionReport,
};
use crate::data::error::SignalError;
use crate::data::resource::{ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope};

use super::{
    canonical_digest, AsyncNodeCompileTimeBoundaryProof, AsyncNodeMilestoneDScenarioId,
    ASYNC_NODE_MILESTONE_D_SCENARIO_MATRIX_SCHEMA_VERSION,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AsyncNodeMilestoneDScenarioEvidenceKind {
    DirectBlocking,
    CombinedSuite,
    CompileTimeBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncNodeMilestoneDScenarioRow {
    scenario_id: AsyncNodeMilestoneDScenarioId,
    evidence_kind: AsyncNodeMilestoneDScenarioEvidenceKind,
    boundary: ResourceBoundaryKind,
    performance: ResourceBoundaryPerformanceEnvelope,
    scenario_digest: String,
}

impl AsyncNodeMilestoneDScenarioRow {
    pub fn scenario_id(&self) -> AsyncNodeMilestoneDScenarioId {
        self.scenario_id
    }

    pub fn evidence_kind(&self) -> AsyncNodeMilestoneDScenarioEvidenceKind {
        self.evidence_kind
    }

    pub fn boundary(&self) -> ResourceBoundaryKind {
        self.boundary
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn scenario_digest(&self) -> &str {
        &self.scenario_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncNodeMilestoneDScenarioMatrixSummary {
    required_scenario_count: u32,
    direct_blocking_count: u32,
    combined_suite_count: u32,
    compile_time_fixture_count: u32,
}

impl AsyncNodeMilestoneDScenarioMatrixSummary {
    pub fn required_scenario_count(&self) -> u32 {
        self.required_scenario_count
    }

    pub fn direct_blocking_count(&self) -> u32 {
        self.direct_blocking_count
    }

    pub fn combined_suite_count(&self) -> u32 {
        self.combined_suite_count
    }

    pub fn compile_time_fixture_count(&self) -> u32 {
        self.compile_time_fixture_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncNodeMilestoneDScenarioMatrix {
    rows: Vec<AsyncNodeMilestoneDScenarioRow>,
    compile_time_boundary: AsyncNodeCompileTimeBoundaryProof,
    summary: AsyncNodeMilestoneDScenarioMatrixSummary,
    matrix_digest: String,
}

impl AsyncNodeMilestoneDScenarioMatrix {
    pub fn rows(&self) -> &[AsyncNodeMilestoneDScenarioRow] {
        &self.rows
    }

    pub fn compile_time_boundary(&self) -> &AsyncNodeCompileTimeBoundaryProof {
        &self.compile_time_boundary
    }

    pub fn summary(&self) -> &AsyncNodeMilestoneDScenarioMatrixSummary {
        &self.summary
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }
}

pub struct AsyncNodeMilestoneDScenarioInputs<'a> {
    pub attachment_equivalence: &'a AsyncNodeCapabilityEquivalenceReport,
    pub condition_blocked_request: &'a AsyncNodeRequestAdmissionReport,
    pub aspect_keyed_historical: &'a AsyncKeyedNodeHistoricalParityReport,
    pub aspect_keyed_equivalence: &'a AsyncKeyedNodeCapabilityEquivalenceReport,
    pub previous_value_blocked_request: &'a AsyncNodeRequestAdmissionReport,
    pub temporal_blocked_request: &'a AsyncNodeRequestAdmissionReport,
    pub gate_state: &'a AsyncNodeGateStateReport,
    pub gate_historical_parity: &'a AsyncNodeHistoricalParityReport,
    pub hierarchy_replay: &'a AsyncNodeHierarchyReplaySummary,
    pub hierarchy_cancellation: &'a AsyncNodeHierarchyCancellationReport,
    pub hierarchy_historical_parity: &'a AsyncNodeHierarchyHistoricalParityReport,
    pub compile_time_boundary: &'a AsyncNodeCompileTimeBoundaryProof,
}

pub fn async_node_milestone_d_scenario_matrix(
    inputs: AsyncNodeMilestoneDScenarioInputs<'_>,
) -> Result<AsyncNodeMilestoneDScenarioMatrix, SignalError> {
    validate_condition_row(inputs.condition_blocked_request)?;
    validate_previous_temporal_row(
        inputs.previous_value_blocked_request,
        inputs.temporal_blocked_request,
    )?;
    validate_gate_row(inputs.gate_state, inputs.gate_historical_parity)?;
    validate_hierarchy_row(
        inputs.hierarchy_replay,
        inputs.hierarchy_cancellation,
        inputs.hierarchy_historical_parity,
    )?;
    validate_keyed_row(
        inputs.aspect_keyed_historical,
        inputs.aspect_keyed_equivalence,
    )?;

    let rows = vec![
        row(
            AsyncNodeMilestoneDScenarioId::AsyncCapabilityAttachmentEquivalence,
            AsyncNodeMilestoneDScenarioEvidenceKind::DirectBlocking,
            inputs.attachment_equivalence.performance().boundary(),
            inputs.attachment_equivalence.performance(),
            canonical_digest(inputs.attachment_equivalence),
        ),
        row(
            AsyncNodeMilestoneDScenarioId::ConditionGatedAsyncAdmissionParity,
            AsyncNodeMilestoneDScenarioEvidenceKind::CombinedSuite,
            inputs
                .condition_blocked_request
                .classification()
                .performance()
                .boundary(),
            inputs
                .condition_blocked_request
                .classification()
                .performance(),
            canonical_digest(&(
                inputs.condition_blocked_request.classification(),
                inputs.condition_blocked_request.resource_admission(),
            )),
        ),
        row(
            AsyncNodeMilestoneDScenarioId::AspectScopedAsyncCapability,
            AsyncNodeMilestoneDScenarioEvidenceKind::CombinedSuite,
            inputs.aspect_keyed_historical.performance().boundary(),
            inputs.aspect_keyed_historical.performance(),
            canonical_digest(&(
                inputs.aspect_keyed_historical,
                inputs.aspect_keyed_equivalence,
            )),
        ),
        row(
            AsyncNodeMilestoneDScenarioId::PreviousValueAndTemporalAsyncCapabilityParity,
            AsyncNodeMilestoneDScenarioEvidenceKind::CombinedSuite,
            inputs
                .previous_value_blocked_request
                .classification()
                .performance()
                .boundary(),
            inputs
                .previous_value_blocked_request
                .classification()
                .performance(),
            canonical_digest(&(
                inputs.previous_value_blocked_request.classification(),
                inputs.temporal_blocked_request.classification(),
            )),
        ),
        row(
            AsyncNodeMilestoneDScenarioId::InteriorAsyncNodeGateEquivalence,
            AsyncNodeMilestoneDScenarioEvidenceKind::DirectBlocking,
            inputs.gate_state.performance().boundary(),
            inputs.gate_state.performance(),
            canonical_digest(&(inputs.gate_state, inputs.gate_historical_parity)),
        ),
        row(
            AsyncNodeMilestoneDScenarioId::HierarchicalAsyncCapabilityReplayAndCancellation,
            AsyncNodeMilestoneDScenarioEvidenceKind::DirectBlocking,
            inputs.hierarchy_historical_parity.performance().boundary(),
            inputs.hierarchy_historical_parity.performance(),
            canonical_digest(&(
                inputs.hierarchy_replay,
                inputs.hierarchy_cancellation,
                inputs.hierarchy_historical_parity,
            )),
        ),
        row(
            AsyncNodeMilestoneDScenarioId::LegacyResourceAliasCompatibility,
            AsyncNodeMilestoneDScenarioEvidenceKind::CombinedSuite,
            inputs.attachment_equivalence.performance().boundary(),
            inputs.attachment_equivalence.performance(),
            canonical_digest(&(
                inputs.attachment_equivalence.alias_lowering_proof(),
                inputs.attachment_equivalence.equivalence_digest(),
            )),
        ),
        row(
            AsyncNodeMilestoneDScenarioId::AsyncCapabilityCompileTimeBoundary,
            AsyncNodeMilestoneDScenarioEvidenceKind::CompileTimeBoundary,
            ResourceBoundaryKind::DeclarationLowering,
            ResourceBoundaryPerformanceEnvelope::declaration_lowering(
                inputs.compile_time_boundary.fixture_labels().len() as u32,
            ),
            inputs.compile_time_boundary.proof_digest().to_owned(),
        ),
    ];
    let summary = AsyncNodeMilestoneDScenarioMatrixSummary {
        required_scenario_count: 8,
        direct_blocking_count: 3,
        combined_suite_count: 4,
        compile_time_fixture_count: inputs.compile_time_boundary.fixture_labels().len() as u32,
    };
    Ok(AsyncNodeMilestoneDScenarioMatrix {
        matrix_digest: canonical_digest(&(
            ASYNC_NODE_MILESTONE_D_SCENARIO_MATRIX_SCHEMA_VERSION,
            &rows,
            &summary,
            inputs.compile_time_boundary,
        )),
        rows,
        compile_time_boundary: inputs.compile_time_boundary.clone(),
        summary,
    })
}

fn row(
    scenario_id: AsyncNodeMilestoneDScenarioId,
    evidence_kind: AsyncNodeMilestoneDScenarioEvidenceKind,
    boundary: ResourceBoundaryKind,
    performance: ResourceBoundaryPerformanceEnvelope,
    scenario_digest: String,
) -> AsyncNodeMilestoneDScenarioRow {
    AsyncNodeMilestoneDScenarioRow {
        scenario_id,
        evidence_kind,
        boundary,
        performance,
        scenario_digest,
    }
}

fn validate_condition_row(report: &AsyncNodeRequestAdmissionReport) -> Result<(), SignalError> {
    if report.classification().class() != AsyncNodeAdmissionClass::BlockedByCondition {
        return Err(SignalError::invalid_input(
            "condition-gated scenario requires blocked async admission".to_owned(),
        ));
    }
    Ok(())
}

fn validate_previous_temporal_row(
    previous_value: &AsyncNodeRequestAdmissionReport,
    temporal: &AsyncNodeRequestAdmissionReport,
) -> Result<(), SignalError> {
    if previous_value.classification().condition_block_class()
        != Some(AsyncNodeConditionBlockClass::PreviousValueReferenceDrifted)
        || temporal.classification().condition_block_class()
            != Some(AsyncNodeConditionBlockClass::TemporalConditionNotReady)
    {
        return Err(SignalError::invalid_input(
            "previous-value/temporal scenario requires both drifted and temporal-not-ready reports"
                .to_owned(),
        ));
    }
    Ok(())
}

fn validate_gate_row(
    gate: &AsyncNodeGateStateReport,
    parity: &AsyncNodeHistoricalParityReport,
) -> Result<(), SignalError> {
    if gate.node() != parity.node() {
        return Err(SignalError::invalid_input(
            "gate scenario requires matching gate and historical parity nodes".to_owned(),
        ));
    }
    Ok(())
}

fn validate_hierarchy_row(
    replay: &AsyncNodeHierarchyReplaySummary,
    cancellation: &AsyncNodeHierarchyCancellationReport,
    parity: &AsyncNodeHierarchyHistoricalParityReport,
) -> Result<(), SignalError> {
    if replay.root_node() != cancellation.root_node()
        || replay.root_node() != parity.root_node()
        || replay.replay_digest() != cancellation.replay_digest()
        || replay.replay_digest() != parity.hierarchy_replay_summary().replay_digest()
    {
        return Err(SignalError::invalid_input(
            "hierarchy scenario requires replay/cancellation/parity root and digest agreement"
                .to_owned(),
        ));
    }
    Ok(())
}

fn validate_keyed_row(
    historical: &AsyncKeyedNodeHistoricalParityReport,
    equivalence: &AsyncKeyedNodeCapabilityEquivalenceReport,
) -> Result<(), SignalError> {
    if historical.node() != equivalence.node()
        || historical.family() != equivalence.family()
        || historical.key() != equivalence.key()
    {
        return Err(SignalError::invalid_input(
            "aspect-scoped keyed scenario requires matching family/key/node lineage".to_owned(),
        ));
    }
    Ok(())
}
