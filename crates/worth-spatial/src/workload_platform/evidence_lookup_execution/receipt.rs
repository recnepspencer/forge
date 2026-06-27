use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_lookup_index_product::{
    EvidenceLookupIndexDisposalPosture, EvidenceLookupIndexLifecyclePosture,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanTopologyPostureState, EvidenceLookupSelectedPlan,
};
use crate::workload_platform::evidence_lookup_query_surface_contract::{
    EvidenceLookupProductQuerySurfaceContractRow, EvidenceLookupQuerySurfaceContract,
};

use super::counters::EvidenceLookupExecutionCounters;
use super::outcome::EvidenceLookupExecutionOutcome;
use super::product_output::EvidenceLookupProductOutput;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupExecutionTopologySupportState {
    NotRequired,
    NotEvaluatedForUnaffectedFamily,
    Satisfied,
    RequiredButMissing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupExecutionReceipt {
    execution_receipt_digest: String,
    selected_plan_digest: String,
    index_product_digest: String,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    evidence_ledger_basis_digest: String,
    topology_support_digest: String,
    topology_support_state: EvidenceLookupExecutionTopologySupportState,
    query_support_digest: String,
    query_surface_contract_rows: Vec<EvidenceLookupProductQuerySurfaceContractRow>,
    index_lifecycle_posture: EvidenceLookupIndexLifecyclePosture,
    index_disposal_posture: EvidenceLookupIndexDisposalPosture,
    outcome: EvidenceLookupExecutionOutcome,
    counters: EvidenceLookupExecutionCounters,
    counter_digest: String,
    product_output: EvidenceLookupProductOutput,
}

pub(crate) struct EvidenceLookupExecutionReceiptParts {
    pub(crate) selected_plan_digest: String,
    pub(crate) index_product_digest: String,
    pub(crate) spatial_touch_digest: String,
    pub(crate) stage_receipt_digest: String,
    pub(crate) evidence_ledger_basis_digest: String,
    pub(crate) topology_support_digest: String,
    pub(crate) topology_support_state: EvidenceLookupExecutionTopologySupportState,
    pub(crate) query_support_digest: String,
    pub(crate) query_surface_contract_rows: Vec<EvidenceLookupProductQuerySurfaceContractRow>,
    pub(crate) index_lifecycle_posture: EvidenceLookupIndexLifecyclePosture,
    pub(crate) index_disposal_posture: EvidenceLookupIndexDisposalPosture,
    pub(crate) outcome: EvidenceLookupExecutionOutcome,
    pub(crate) counters: EvidenceLookupExecutionCounters,
    pub(crate) evidence_receipt_digests: Vec<String>,
}

impl EvidenceLookupExecutionReceipt {
    pub(crate) fn from_parts(parts: EvidenceLookupExecutionReceiptParts) -> Self {
        let counter_digest = parts.counters.digest();
        let provisional_receipt_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-execution-receipt-provisional:v1".to_string(),
                format!("selected-plan:{}", parts.selected_plan_digest),
                format!("index-product:{}", parts.index_product_digest),
                format!("spatial-touch:{}", parts.spatial_touch_digest),
                format!("stage-receipt:{}", parts.stage_receipt_digest),
                format!("ledger-basis:{}", parts.evidence_ledger_basis_digest),
                format!("topology-support:{}", parts.topology_support_digest),
                format!("topology-state:{:?}", parts.topology_support_state),
                format!("query-support:{}", parts.query_support_digest),
                format!(
                    "query-surface-contract-rows:{}",
                    parts.query_surface_contract_rows.len()
                ),
                format!("index-lifecycle:{:?}", parts.index_lifecycle_posture.kind()),
                format!("index-disposal:{:?}", parts.index_disposal_posture.kind()),
                format!("outcome:{:?}", parts.outcome),
                format!("counters:{counter_digest}"),
            ],
        );
        let product_output = EvidenceLookupProductOutput::new(
            provisional_receipt_digest.clone(),
            parts.evidence_receipt_digests,
        );
        let execution_receipt_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-execution-receipt:v1".to_string(),
                format!("selected-plan:{}", parts.selected_plan_digest),
                format!("index-product:{}", parts.index_product_digest),
                format!("spatial-touch:{}", parts.spatial_touch_digest),
                format!("stage-receipt:{}", parts.stage_receipt_digest),
                format!("ledger-basis:{}", parts.evidence_ledger_basis_digest),
                format!("topology-support:{}", parts.topology_support_digest),
                format!("topology-state:{:?}", parts.topology_support_state),
                format!("query-support:{}", parts.query_support_digest),
                format!(
                    "query-surface-contract-rows:{}",
                    parts.query_surface_contract_rows.len()
                ),
                format!("index-lifecycle:{:?}", parts.index_lifecycle_posture.kind()),
                format!("index-disposal:{:?}", parts.index_disposal_posture.kind()),
                format!("outcome:{:?}", parts.outcome),
                format!("counters:{counter_digest}"),
                format!("product-output:{}", product_output.output_digest()),
            ],
        );
        let product_output = EvidenceLookupProductOutput::new(
            execution_receipt_digest.clone(),
            product_output.evidence_receipt_digests().to_vec(),
        );
        Self {
            execution_receipt_digest,
            selected_plan_digest: parts.selected_plan_digest,
            index_product_digest: parts.index_product_digest,
            spatial_touch_digest: parts.spatial_touch_digest,
            stage_receipt_digest: parts.stage_receipt_digest,
            evidence_ledger_basis_digest: parts.evidence_ledger_basis_digest,
            topology_support_digest: parts.topology_support_digest,
            topology_support_state: parts.topology_support_state,
            query_support_digest: parts.query_support_digest,
            query_surface_contract_rows: parts.query_surface_contract_rows,
            index_lifecycle_posture: parts.index_lifecycle_posture,
            index_disposal_posture: parts.index_disposal_posture,
            outcome: parts.outcome,
            counters: parts.counters,
            counter_digest,
            product_output,
        }
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn index_product_digest(&self) -> &str {
        &self.index_product_digest
    }

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub fn evidence_ledger_basis_digest(&self) -> &str {
        &self.evidence_ledger_basis_digest
    }

    pub fn topology_support_digest(&self) -> &str {
        &self.topology_support_digest
    }

    pub const fn topology_support_state(&self) -> EvidenceLookupExecutionTopologySupportState {
        self.topology_support_state
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn query_surface_contract_rows(&self) -> &[EvidenceLookupProductQuerySurfaceContractRow] {
        &self.query_surface_contract_rows
    }

    pub fn query_surface_contract_for_family(
        &self,
        family_identity: &str,
    ) -> Option<&EvidenceLookupQuerySurfaceContract> {
        self.query_surface_contract_rows
            .iter()
            .find(|row| row.family_identity() == family_identity)
            .map(EvidenceLookupProductQuerySurfaceContractRow::contract)
    }

    pub const fn index_lifecycle_posture(&self) -> EvidenceLookupIndexLifecyclePosture {
        self.index_lifecycle_posture
    }

    pub const fn index_disposal_posture(&self) -> EvidenceLookupIndexDisposalPosture {
        self.index_disposal_posture
    }

    pub const fn outcome(&self) -> EvidenceLookupExecutionOutcome {
        self.outcome
    }

    pub const fn counters(&self) -> &EvidenceLookupExecutionCounters {
        &self.counters
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }

    pub fn lookup_product_output(&self) -> &EvidenceLookupProductOutput {
        &self.product_output
    }

    pub fn lookup_product_output_digest(&self) -> &str {
        self.product_output.output_digest()
    }

    pub const fn claims_query_descriptor_authority(&self) -> bool {
        false
    }

    #[cfg(test)]
    pub(crate) fn with_selected_plan_digest_for_tests(
        &self,
        selected_plan_digest: impl Into<String>,
    ) -> Self {
        self.rebuild_for_tests(Some(selected_plan_digest.into()), None)
    }

    #[cfg(test)]
    pub(crate) fn with_outcome_for_tests(&self, outcome: EvidenceLookupExecutionOutcome) -> Self {
        self.rebuild_for_tests(None, Some(outcome))
    }

    #[cfg(test)]
    fn rebuild_for_tests(
        &self,
        selected_plan_digest: Option<String>,
        outcome: Option<EvidenceLookupExecutionOutcome>,
    ) -> Self {
        Self::from_parts(EvidenceLookupExecutionReceiptParts {
            selected_plan_digest: selected_plan_digest
                .unwrap_or_else(|| self.selected_plan_digest.clone()),
            index_product_digest: self.index_product_digest.clone(),
            spatial_touch_digest: self.spatial_touch_digest.clone(),
            stage_receipt_digest: self.stage_receipt_digest.clone(),
            evidence_ledger_basis_digest: self.evidence_ledger_basis_digest.clone(),
            topology_support_digest: self.topology_support_digest.clone(),
            topology_support_state: self.topology_support_state,
            query_support_digest: self.query_support_digest.clone(),
            query_surface_contract_rows: self.query_surface_contract_rows.clone(),
            index_lifecycle_posture: self.index_lifecycle_posture,
            index_disposal_posture: self.index_disposal_posture,
            outcome: outcome.unwrap_or(self.outcome),
            counters: self.counters,
            evidence_receipt_digests: self.product_output.evidence_receipt_digests().to_vec(),
        })
    }
}

impl EvidenceLookupExecutionTopologySupportState {
    pub(crate) fn from_selected_plan(selected_plan: &EvidenceLookupSelectedPlan) -> Self {
        let mut saw_satisfied = false;
        let mut saw_not_evaluated = false;
        for row in selected_plan.rows() {
            match row.topology_posture().state() {
                EvidenceLookupPlanTopologyPostureState::RequiredButMissing { .. } => {
                    return Self::RequiredButMissing;
                }
                EvidenceLookupPlanTopologyPostureState::Satisfied { .. } => {
                    saw_satisfied = true;
                }
                EvidenceLookupPlanTopologyPostureState::NotEvaluatedForUnaffectedFamily => {
                    saw_not_evaluated = true;
                }
                EvidenceLookupPlanTopologyPostureState::NotRequired => {}
            }
        }
        if saw_satisfied {
            return Self::Satisfied;
        }
        if saw_not_evaluated {
            return Self::NotEvaluatedForUnaffectedFamily;
        }
        Self::NotRequired
    }
}
