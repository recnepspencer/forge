use serde::{Deserialize, Serialize};

use crate::data::error::SignalError;
use crate::data::resource::ResourceBoundaryKind;

use super::{
    canonical_digest, matrix::AsyncNodeMilestoneDScenarioMatrix,
    AsyncNodeMilestoneDPerformanceClaimId, AsyncNodeMilestoneDScenarioId,
    ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
    REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncNodeMilestoneDPerformanceCloseoutRow {
    claim_id: AsyncNodeMilestoneDPerformanceClaimId,
    scenario_id: AsyncNodeMilestoneDScenarioId,
    boundary: ResourceBoundaryKind,
    performance: crate::data::resource::ResourceBoundaryPerformanceEnvelope,
    claim_digest: String,
}

impl AsyncNodeMilestoneDPerformanceCloseoutRow {
    pub fn claim_id(&self) -> AsyncNodeMilestoneDPerformanceClaimId {
        self.claim_id
    }

    pub fn scenario_id(&self) -> AsyncNodeMilestoneDScenarioId {
        self.scenario_id
    }

    pub fn boundary(&self) -> ResourceBoundaryKind {
        self.boundary
    }

    pub fn performance(&self) -> crate::data::resource::ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn claim_digest(&self) -> &str {
        &self.claim_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncNodeMilestoneDPerformanceCloseoutSummary {
    required_claim_count: u32,
    claim_count: u32,
}

impl AsyncNodeMilestoneDPerformanceCloseoutSummary {
    pub fn required_claim_count(&self) -> u32 {
        self.required_claim_count
    }

    pub fn claim_count(&self) -> u32 {
        self.claim_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncNodeMilestoneDPerformanceCloseout {
    rows: Vec<AsyncNodeMilestoneDPerformanceCloseoutRow>,
    summary: AsyncNodeMilestoneDPerformanceCloseoutSummary,
    closeout_digest: String,
}

impl AsyncNodeMilestoneDPerformanceCloseout {
    pub fn rows(&self) -> &[AsyncNodeMilestoneDPerformanceCloseoutRow] {
        &self.rows
    }

    pub fn summary(&self) -> &AsyncNodeMilestoneDPerformanceCloseoutSummary {
        &self.summary
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

pub fn async_node_milestone_d_performance_closeout(
    matrix: &AsyncNodeMilestoneDScenarioMatrix,
) -> Result<AsyncNodeMilestoneDPerformanceCloseout, SignalError> {
    let rows = vec![
        claim_row(
            matrix,
            AsyncNodeMilestoneDPerformanceClaimId::AttachmentEquivalenceBounded,
            AsyncNodeMilestoneDScenarioId::AsyncCapabilityAttachmentEquivalence,
        )?,
        claim_row(
            matrix,
            AsyncNodeMilestoneDPerformanceClaimId::ConditionAdmissionBoundaryBounded,
            AsyncNodeMilestoneDScenarioId::ConditionGatedAsyncAdmissionParity,
        )?,
        claim_row(
            matrix,
            AsyncNodeMilestoneDPerformanceClaimId::AspectScopedBreadthBounded,
            AsyncNodeMilestoneDScenarioId::AspectScopedAsyncCapability,
        )?,
        claim_row(
            matrix,
            AsyncNodeMilestoneDPerformanceClaimId::InteriorGateCoordinationBounded,
            AsyncNodeMilestoneDScenarioId::InteriorAsyncNodeGateEquivalence,
        )?,
        claim_row(
            matrix,
            AsyncNodeMilestoneDPerformanceClaimId::HierarchyReplayRestoreBounded,
            AsyncNodeMilestoneDScenarioId::HierarchicalAsyncCapabilityReplayAndCancellation,
        )?,
        claim_row(
            matrix,
            AsyncNodeMilestoneDPerformanceClaimId::LegacyAliasCompatibilityBounded,
            AsyncNodeMilestoneDScenarioId::LegacyResourceAliasCompatibility,
        )?,
    ];
    let summary = AsyncNodeMilestoneDPerformanceCloseoutSummary {
        required_claim_count: REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS.len() as u32,
        claim_count: rows.len() as u32,
    };
    Ok(AsyncNodeMilestoneDPerformanceCloseout {
        closeout_digest: canonical_digest(&(
            ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
            &rows,
            &summary,
            matrix.matrix_digest(),
        )),
        rows,
        summary,
    })
}

fn claim_row(
    matrix: &AsyncNodeMilestoneDScenarioMatrix,
    claim_id: AsyncNodeMilestoneDPerformanceClaimId,
    scenario_id: AsyncNodeMilestoneDScenarioId,
) -> Result<AsyncNodeMilestoneDPerformanceCloseoutRow, SignalError> {
    let row = matrix
        .rows()
        .iter()
        .find(|row| row.scenario_id() == scenario_id)
        .ok_or_else(|| {
            SignalError::invalid_input(format!("missing milestone D scenario {scenario_id:?}"))
        })?;
    Ok(AsyncNodeMilestoneDPerformanceCloseoutRow {
        claim_id,
        scenario_id,
        boundary: row.boundary(),
        performance: row.performance(),
        claim_digest: canonical_digest(&(
            claim_id,
            scenario_id,
            row.performance(),
            row.scenario_digest(),
        )),
    })
}
