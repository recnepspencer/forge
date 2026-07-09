use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::data::error::SignalError;

use super::{
    canonical_digest, matrix::AsyncNodeMilestoneDScenarioMatrix,
    performance::AsyncNodeMilestoneDPerformanceCloseout, AsyncNodeMilestoneDScenarioEvidenceKind,
    AsyncNodeMilestoneDScenarioId, ASYNC_NODE_COMPILE_TIME_BOUNDARY_PROOF_SCHEMA_VERSION,
    ASYNC_NODE_MILESTONE_D_CERTIFICATION_RUN_SCHEMA_VERSION,
    ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
    ASYNC_NODE_MILESTONE_D_SCENARIO_MATRIX_SCHEMA_VERSION,
    REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES, REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS,
    REQUIRED_ASYNC_NODE_MILESTONE_D_SCENARIOS,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncNodeMilestoneDCertificationRunSummary {
    direct_blocking_count: u32,
    combined_suite_count: u32,
    compile_time_fixture_count: u32,
}

impl AsyncNodeMilestoneDCertificationRunSummary {
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
pub struct AsyncNodeMilestoneDCertificationRun {
    scenario_matrix: AsyncNodeMilestoneDScenarioMatrix,
    performance_closeout: AsyncNodeMilestoneDPerformanceCloseout,
    summary: AsyncNodeMilestoneDCertificationRunSummary,
    run_digest: String,
}

impl AsyncNodeMilestoneDCertificationRun {
    pub fn scenario_matrix(&self) -> &AsyncNodeMilestoneDScenarioMatrix {
        &self.scenario_matrix
    }

    pub fn performance_closeout(&self) -> &AsyncNodeMilestoneDPerformanceCloseout {
        &self.performance_closeout
    }

    pub fn summary(&self) -> &AsyncNodeMilestoneDCertificationRunSummary {
        &self.summary
    }

    pub fn run_digest(&self) -> &str {
        &self.run_digest
    }
}

pub fn async_node_milestone_d_certification_run(
    scenario_matrix: AsyncNodeMilestoneDScenarioMatrix,
    performance_closeout: AsyncNodeMilestoneDPerformanceCloseout,
) -> Result<AsyncNodeMilestoneDCertificationRun, SignalError> {
    validate_matrix(&scenario_matrix)?;
    validate_performance_closeout(&scenario_matrix, &performance_closeout)?;
    if scenario_matrix.rows().len() != REQUIRED_ASYNC_NODE_MILESTONE_D_SCENARIOS.len() {
        return Err(SignalError::invalid_input(
            "milestone D certification run requires complete scenario coverage".to_owned(),
        ));
    }
    if performance_closeout.rows().len() != REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS.len()
    {
        return Err(SignalError::invalid_input(
            "milestone D certification run requires complete performance closeout".to_owned(),
        ));
    }
    let summary = AsyncNodeMilestoneDCertificationRunSummary {
        direct_blocking_count: scenario_matrix.summary().direct_blocking_count(),
        combined_suite_count: scenario_matrix.summary().combined_suite_count(),
        compile_time_fixture_count: scenario_matrix.summary().compile_time_fixture_count(),
    };
    Ok(AsyncNodeMilestoneDCertificationRun {
        run_digest: canonical_digest(&(
            ASYNC_NODE_MILESTONE_D_CERTIFICATION_RUN_SCHEMA_VERSION,
            scenario_matrix.matrix_digest(),
            performance_closeout.closeout_digest(),
            &summary,
        )),
        scenario_matrix,
        performance_closeout,
        summary,
    })
}

fn validate_matrix(matrix: &AsyncNodeMilestoneDScenarioMatrix) -> Result<(), SignalError> {
    let scenario_ids = matrix
        .rows()
        .iter()
        .map(|row| row.scenario_id())
        .collect::<BTreeSet<_>>();
    let required_scenarios = REQUIRED_ASYNC_NODE_MILESTONE_D_SCENARIOS
        .into_iter()
        .collect::<BTreeSet<_>>();
    if scenario_ids != required_scenarios {
        return Err(SignalError::invalid_input(
            "milestone D scenario matrix requires exact required scenario coverage".to_owned(),
        ));
    }

    let direct_blocking_count = matrix
        .rows()
        .iter()
        .filter(|row| {
            row.evidence_kind() == AsyncNodeMilestoneDScenarioEvidenceKind::DirectBlocking
        })
        .count() as u32;
    let combined_suite_count = matrix
        .rows()
        .iter()
        .filter(|row| row.evidence_kind() == AsyncNodeMilestoneDScenarioEvidenceKind::CombinedSuite)
        .count() as u32;
    let compile_time_count = matrix
        .rows()
        .iter()
        .filter(|row| {
            row.evidence_kind() == AsyncNodeMilestoneDScenarioEvidenceKind::CompileTimeBoundary
        })
        .count();
    if direct_blocking_count != matrix.summary().direct_blocking_count()
        || combined_suite_count != matrix.summary().combined_suite_count()
        || matrix.rows().len() as u32 != matrix.summary().required_scenario_count()
        || compile_time_count != 1
    {
        return Err(SignalError::invalid_input(
            "milestone D scenario matrix summary must match actual evidence lanes".to_owned(),
        ));
    }

    let compile_time = matrix.compile_time_boundary();
    let fixture_labels = compile_time
        .fixture_labels()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let required_fixtures = REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES
        .iter()
        .map(|label| (*label).to_owned())
        .collect::<BTreeSet<_>>();
    if fixture_labels != required_fixtures {
        return Err(SignalError::invalid_input(
            "milestone D compile-time boundary proof requires exact required fixtures".to_owned(),
        ));
    }
    let expected_compile_time_digest = canonical_digest(&(
        ASYNC_NODE_COMPILE_TIME_BOUNDARY_PROOF_SCHEMA_VERSION,
        compile_time.fixture_labels(),
    ));
    if compile_time.proof_digest() != expected_compile_time_digest {
        return Err(SignalError::invalid_input(
            "milestone D compile-time boundary proof digest drifted from its fixture set"
                .to_owned(),
        ));
    }
    let compile_time_row = matrix
        .rows()
        .iter()
        .find(|row| {
            row.scenario_id() == AsyncNodeMilestoneDScenarioId::AsyncCapabilityCompileTimeBoundary
        })
        .ok_or_else(|| {
            SignalError::invalid_input(
                "milestone D scenario matrix is missing the compile-time boundary row".to_owned(),
            )
        })?;
    if compile_time_row.scenario_digest() != compile_time.proof_digest() {
        return Err(SignalError::invalid_input(
            "milestone D compile-time scenario row must bind the compile-time proof digest"
                .to_owned(),
        ));
    }

    let expected_matrix_digest = canonical_digest(&(
        ASYNC_NODE_MILESTONE_D_SCENARIO_MATRIX_SCHEMA_VERSION,
        matrix.rows(),
        matrix.summary(),
        compile_time,
    ));
    if matrix.matrix_digest() != expected_matrix_digest {
        return Err(SignalError::invalid_input(
            "milestone D scenario matrix digest drifted from its report contents".to_owned(),
        ));
    }
    Ok(())
}

fn validate_performance_closeout(
    matrix: &AsyncNodeMilestoneDScenarioMatrix,
    closeout: &AsyncNodeMilestoneDPerformanceCloseout,
) -> Result<(), SignalError> {
    let claim_ids = closeout
        .rows()
        .iter()
        .map(|row| row.claim_id())
        .collect::<BTreeSet<_>>();
    let required_claims = REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS
        .into_iter()
        .collect::<BTreeSet<_>>();
    if claim_ids != required_claims {
        return Err(SignalError::invalid_input(
            "milestone D performance closeout requires exact required claim coverage".to_owned(),
        ));
    }
    if closeout.rows().len() as u32 != closeout.summary().claim_count()
        || REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS.len() as u32
            != closeout.summary().required_claim_count()
    {
        return Err(SignalError::invalid_input(
            "milestone D performance closeout summary must match actual claim coverage".to_owned(),
        ));
    }
    for row in closeout.rows() {
        let matrix_row = matrix
            .rows()
            .iter()
            .find(|candidate| candidate.scenario_id() == row.scenario_id())
            .ok_or_else(|| {
                SignalError::invalid_input(
                    "milestone D performance closeout references an unknown scenario row"
                        .to_owned(),
                )
            })?;
        if row.boundary() != matrix_row.boundary() || row.performance() != matrix_row.performance()
        {
            return Err(SignalError::invalid_input(
                "milestone D performance closeout must preserve the scenario boundary envelope"
                    .to_owned(),
            ));
        }
        let expected_claim_digest = canonical_digest(&(
            row.claim_id(),
            row.scenario_id(),
            row.performance(),
            matrix_row.scenario_digest(),
        ));
        if row.claim_digest() != expected_claim_digest {
            return Err(SignalError::invalid_input(
                "milestone D performance closeout digest drifted from its scenario boundary evidence"
                    .to_owned(),
            ));
        }
    }
    let expected_closeout_digest = canonical_digest(&(
        ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
        closeout.rows(),
        closeout.summary(),
        matrix.matrix_digest(),
    ));
    if closeout.closeout_digest() != expected_closeout_digest {
        return Err(SignalError::invalid_input(
            "milestone D performance closeout digest drifted from its report contents".to_owned(),
        ));
    }
    Ok(())
}
