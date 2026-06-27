use crate::workload_platform::evidence_lookup_stage_cutover::{
    EvidenceLookupCoveredStageCutoverProof, EvidenceLookupTopologyDerivedReceiptState,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::counters::EvidenceLookupWorkloadCutoverCounters;
use super::error::{EvidenceLookupWorkloadCutoverError, EvidenceLookupWorkloadCutoverErrorKind};
use super::seed::EvidenceLookupMilestoneTwelveSeed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupConsumedWorkloadHandoff {
    stage_receipt_identity: String,
    workload_stage_index_identity: String,
    selected_lookup_plan_digest: String,
    lookup_execution_receipt_digest: String,
    lookup_product_output_digest: String,
    topology_derived_receipt_state: EvidenceLookupTopologyDerivedReceiptState,
    covered_family_identities: Vec<String>,
    counters: EvidenceLookupWorkloadCutoverCounters,
    milestone_twelve_seed: EvidenceLookupMilestoneTwelveSeed,
}

impl EvidenceLookupConsumedWorkloadHandoff {
    pub fn lower_from_stage_proof(
        proof: &EvidenceLookupCoveredStageCutoverProof,
    ) -> Result<Self, EvidenceLookupWorkloadCutoverError> {
        if proof.counters().raw_row_scan_count() != 0
            || proof.counters().broad_receipt_scan_count() != 0
        {
            return Err(EvidenceLookupWorkloadCutoverError::new(
                EvidenceLookupWorkloadCutoverErrorKind::RawEvidenceFallbackDenied,
                "lookup-consumed workload handoff cannot lower from raw evidence or broad receipt scans",
            ));
        }
        if proof.counters().caller_owned_scan_count() != 0 {
            return Err(EvidenceLookupWorkloadCutoverError::new(
                EvidenceLookupWorkloadCutoverErrorKind::ScopeExpansionDenied,
                "lookup-consumed workload handoff cannot lower from caller-owned lookup scans",
            ));
        }

        let counters = EvidenceLookupWorkloadCutoverCounters::new(
            proof.counters().covered_family_count(),
            proof.counters().indexed_lookup_count(),
            proof.counters().topology_receipt_ref_count(),
            proof.counters().raw_row_scan_count(),
            proof.counters().broad_receipt_scan_count(),
            proof.counters().caller_owned_scan_count(),
        );
        let milestone_twelve_seed = EvidenceLookupMilestoneTwelveSeed::new_stage_cutover(
            proof.selected_lookup_plan_digest().to_string(),
            proof.lookup_execution_receipt_digest().to_string(),
            proof.lookup_product_output_digest().to_string(),
            proof.covered_family_identities().to_vec(),
        );

        Ok(Self {
            stage_receipt_identity: proof.stage_receipt_identity().to_string(),
            workload_stage_index_identity: proof.workload_stage_index_identity().to_string(),
            selected_lookup_plan_digest: proof.selected_lookup_plan_digest().to_string(),
            lookup_execution_receipt_digest: proof.lookup_execution_receipt_digest().to_string(),
            lookup_product_output_digest: proof.lookup_product_output_digest().to_string(),
            topology_derived_receipt_state: proof.topology_derived_receipt_state().clone(),
            covered_family_identities: proof.covered_family_identities().to_vec(),
            counters,
            milestone_twelve_seed,
        })
    }

    pub fn stage_receipt_identity(&self) -> &str {
        &self.stage_receipt_identity
    }

    pub fn workload_stage_index_identity(&self) -> &str {
        &self.workload_stage_index_identity
    }

    pub fn selected_lookup_plan_digest(&self) -> &str {
        &self.selected_lookup_plan_digest
    }

    pub fn lookup_execution_receipt_digest(&self) -> &str {
        &self.lookup_execution_receipt_digest
    }

    pub fn lookup_product_output_digest(&self) -> &str {
        &self.lookup_product_output_digest
    }

    pub const fn topology_derived_receipt_state(
        &self,
    ) -> &EvidenceLookupTopologyDerivedReceiptState {
        &self.topology_derived_receipt_state
    }

    pub fn covered_family_identities(&self) -> &[String] {
        &self.covered_family_identities
    }

    pub const fn counters(&self) -> &EvidenceLookupWorkloadCutoverCounters {
        &self.counters
    }

    pub fn milestone_twelve_seed(&self) -> &EvidenceLookupMilestoneTwelveSeed {
        &self.milestone_twelve_seed
    }

    pub fn semantic_graph_identity(&self) -> String {
        let mut covered_family_identities = self.covered_family_identities.clone();
        covered_family_identities.sort();

        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-consumed-workload-handoff:v1".to_string(),
                format!("stage-receipt:{}", self.stage_receipt_identity),
                format!(
                    "workload-stage-index:{}",
                    self.workload_stage_index_identity
                ),
                format!("selected-plan:{}", self.selected_lookup_plan_digest),
                format!("lookup-execution:{}", self.lookup_execution_receipt_digest),
                format!("lookup-product:{}", self.lookup_product_output_digest),
                topology_derived_receipt_state_digest_part(&self.topology_derived_receipt_state),
                format!("covered-families:{}", covered_family_identities.join("|")),
                format!(
                    "covered-family-count:{}",
                    self.counters.covered_family_count()
                ),
                format!(
                    "indexed-lookup-count:{}",
                    self.counters.indexed_lookup_count()
                ),
                format!(
                    "topology-receipt-ref-count:{}",
                    self.counters.topology_receipt_ref_count()
                ),
                format!("raw-row-scan-count:{}", self.counters.raw_row_scan_count()),
                format!(
                    "broad-receipt-scan-count:{}",
                    self.counters.broad_receipt_scan_count()
                ),
                format!(
                    "caller-owned-scan-count:{}",
                    self.counters.caller_owned_scan_count()
                ),
                format!(
                    "milestone-twelve-selected-plan:{}",
                    self.milestone_twelve_seed.selected_lookup_plan_digest()
                ),
                format!(
                    "milestone-twelve-lookup-execution:{}",
                    self.milestone_twelve_seed.lookup_execution_receipt_digest()
                ),
                format!(
                    "milestone-twelve-lookup-product:{}",
                    self.milestone_twelve_seed.lookup_product_output_digest()
                ),
                format!(
                    "milestone-twelve-family-coverage:{}",
                    self.milestone_twelve_seed.family_coverage_digest()
                ),
                format!(
                    "milestone-twelve-source-firewall:{}",
                    self.milestone_twelve_seed.source_firewall_digest()
                ),
                format!(
                    "milestone-twelve-residue-audit:{}",
                    self.milestone_twelve_seed.residue_audit_digest()
                ),
                format!(
                    "milestone-twelve-replay-readiness:{:?}",
                    self.milestone_twelve_seed.replay_readiness_posture()
                ),
            ],
        )
    }

    #[cfg(test)]
    pub(crate) fn with_test_stage_receipt_identity(
        mut self,
        stage_receipt_identity: impl Into<String>,
    ) -> Self {
        self.stage_receipt_identity = stage_receipt_identity.into();
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_workload_stage_index_identity(
        mut self,
        workload_stage_index_identity: impl Into<String>,
    ) -> Self {
        self.workload_stage_index_identity = workload_stage_index_identity.into();
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_lookup_execution_receipt_digest(
        mut self,
        lookup_execution_receipt_digest: impl Into<String>,
    ) -> Self {
        self.lookup_execution_receipt_digest = lookup_execution_receipt_digest.into();
        self
    }

    #[cfg(any(test, feature = "test-support-lowering"))]
    pub(crate) fn with_test_covered_family_identities(
        mut self,
        covered_family_identities: Vec<String>,
    ) -> Self {
        self.covered_family_identities = covered_family_identities;
        self
    }
}

fn topology_derived_receipt_state_digest_part(
    state: &EvidenceLookupTopologyDerivedReceiptState,
) -> String {
    match state {
        EvidenceLookupTopologyDerivedReceiptState::NotRequired => {
            "topology-derived:not-required".to_string()
        }
        EvidenceLookupTopologyDerivedReceiptState::ReceiptRef(receipt_ref) => format!(
            "topology-derived:{}:{}:{}",
            receipt_ref.seed_digest(),
            receipt_ref.receipt_ref_digest(),
            receipt_ref.family_identity()
        ),
    }
}
