use crate::validator_invariant_catalog::selected_graph_obligation_enforcement::{
    WorthTopologySelectedGraphObligationEnforcementCounters,
    WorthTopologySelectedGraphObligationEnforcementReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologySelectedGraphObligationEnforcementPhaseSevenSeed {
    phase_six_seed_digest: String,
    selected_plan_digest: String,
    routing_closure_digest: String,
    query_execution_envelope_digest: String,
    execution_backed_adoption_manifest_digest: String,
    support_pin_digest: String,
    support_matrix_digest: String,
    residue_manifest_digest: String,
    local_ceremony_audit_digest: String,
    in_memory_proof_digest: String,
    execution_proof_digest: String,
    receipt_count: usize,
    counters_digest: String,
    source_firewall_digest: String,
    receipt_digests: Vec<String>,
    seed_digest: String,
}

impl WorthTopologySelectedGraphObligationEnforcementPhaseSevenSeed {
    pub(in crate::validator_invariant_catalog) fn from_parts(
        phase_six_seed_digest: &str,
        selected_plan_digest: &str,
        routing_closure_digest: &str,
        query_execution_envelope_digest: &str,
        execution_backed_adoption_manifest_digest: &str,
        support_pin_digest: &str,
        support_matrix_digest: &str,
        residue_manifest_digest: &str,
        local_ceremony_audit_digest: &str,
        in_memory_proof_digest: &str,
        execution_proof_digest: &str,
        counters: &WorthTopologySelectedGraphObligationEnforcementCounters,
        source_firewall_digest: &str,
        receipts: &[WorthTopologySelectedGraphObligationEnforcementReceipt],
    ) -> Self {
        let receipt_digests = receipts
            .iter()
            .map(|receipt| receipt.enforcement_receipt_digest().to_string())
            .collect::<Vec<_>>();
        let mut seed_parts = vec![
            "worth-topo-selected-graph-obligation-enforcement-phase-seven-seed-v1".to_string(),
            format!("phase-six-seed:{phase_six_seed_digest}"),
            format!("selected-plan:{selected_plan_digest}"),
            format!("routing-closure:{routing_closure_digest}"),
            format!("query-envelope:{query_execution_envelope_digest}"),
            format!("adoption-manifest:{execution_backed_adoption_manifest_digest}"),
            format!("support-pin:{support_pin_digest}"),
            format!("support-matrix:{support_matrix_digest}"),
            format!("residue-manifest:{residue_manifest_digest}"),
            format!("local-ceremony-audit:{local_ceremony_audit_digest}"),
            format!("in-memory-proof:{in_memory_proof_digest}"),
            format!("execution-proof:{execution_proof_digest}"),
            format!("receipt-count:{}", receipt_digests.len()),
            format!("counters:{}", counters.counters_digest()),
            format!("source-firewall:{source_firewall_digest}"),
        ];
        seed_parts.extend(
            receipt_digests
                .iter()
                .map(|digest| format!("receipt:{digest}")),
        );
        let seed_digest = seed_parts.join("|");
        Self {
            phase_six_seed_digest: phase_six_seed_digest.to_string(),
            selected_plan_digest: selected_plan_digest.to_string(),
            routing_closure_digest: routing_closure_digest.to_string(),
            query_execution_envelope_digest: query_execution_envelope_digest.to_string(),
            execution_backed_adoption_manifest_digest: execution_backed_adoption_manifest_digest
                .to_string(),
            support_pin_digest: support_pin_digest.to_string(),
            support_matrix_digest: support_matrix_digest.to_string(),
            residue_manifest_digest: residue_manifest_digest.to_string(),
            local_ceremony_audit_digest: local_ceremony_audit_digest.to_string(),
            in_memory_proof_digest: in_memory_proof_digest.to_string(),
            execution_proof_digest: execution_proof_digest.to_string(),
            receipt_count: receipt_digests.len(),
            counters_digest: counters.counters_digest().to_string(),
            source_firewall_digest: source_firewall_digest.to_string(),
            receipt_digests,
            seed_digest,
        }
    }

    pub fn phase_six_seed_digest(&self) -> &str {
        &self.phase_six_seed_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn routing_closure_digest(&self) -> &str {
        &self.routing_closure_digest
    }

    pub fn query_execution_envelope_digest(&self) -> &str {
        &self.query_execution_envelope_digest
    }

    pub fn execution_backed_adoption_manifest_digest(&self) -> &str {
        &self.execution_backed_adoption_manifest_digest
    }

    pub fn support_pin_digest(&self) -> &str {
        &self.support_pin_digest
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn residue_manifest_digest(&self) -> &str {
        &self.residue_manifest_digest
    }

    pub fn local_ceremony_audit_digest(&self) -> &str {
        &self.local_ceremony_audit_digest
    }

    pub fn in_memory_proof_digest(&self) -> &str {
        &self.in_memory_proof_digest
    }

    pub fn execution_proof_digest(&self) -> &str {
        &self.execution_proof_digest
    }

    pub const fn receipt_count(&self) -> usize {
        self.receipt_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn receipt_digests(&self) -> &[String] {
        &self.receipt_digests
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
