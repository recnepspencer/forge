use forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionBackedAdoptionProof;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyGraphObligationExecutionProofProjection {
    adoption_manifest_digest: String,
    support_pin_digest: String,
    support_matrix_digest: String,
    residue_manifest_digest: String,
    local_ceremony_audit_digest: String,
    in_memory_proof_digest: String,
    execution_proof_digest: String,
    execution_envelope_digest: String,
    selected_obligation_count: usize,
    executor_row_count: usize,
    projection_digest: String,
}

impl WorthTopologyGraphObligationExecutionProofProjection {
    pub(in crate::validator_invariant_catalog) fn from_execution_backed_proof(
        proof: &ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    ) -> Self {
        let execution_proof = proof.execution_proof();
        let manifest = proof.manifest();
        let adoption_manifest_digest = manifest.manifest_digest().to_string();
        let support_pin_digest = manifest.support_pin_digest().to_string();
        let support_matrix_digest = manifest.support_matrix_digest().to_string();
        let residue_manifest_digest = manifest.residue_manifest_digest().to_string();
        let local_ceremony_audit_digest = manifest.local_ceremony_audit_digest().to_string();
        let in_memory_proof_digest = manifest.in_memory_proof_digest().to_string();
        let execution_proof_digest = execution_proof.proof_digest().to_string();
        let execution_envelope_digest = execution_proof.envelope_digest().to_string();
        let selected_obligation_count = execution_proof.selected_obligation_count();
        let executor_row_count = execution_proof.rows().len();
        let projection_digest = [
            "worth-topo-graph-obligation-execution-proof-projection-v1",
            adoption_manifest_digest.as_str(),
            support_pin_digest.as_str(),
            support_matrix_digest.as_str(),
            residue_manifest_digest.as_str(),
            local_ceremony_audit_digest.as_str(),
            in_memory_proof_digest.as_str(),
            execution_proof_digest.as_str(),
            execution_envelope_digest.as_str(),
            &selected_obligation_count.to_string(),
            &executor_row_count.to_string(),
        ]
        .join("|");
        Self {
            adoption_manifest_digest,
            support_pin_digest,
            support_matrix_digest,
            residue_manifest_digest,
            local_ceremony_audit_digest,
            in_memory_proof_digest,
            execution_proof_digest,
            execution_envelope_digest,
            selected_obligation_count,
            executor_row_count,
            projection_digest,
        }
    }

    pub fn adoption_manifest_digest(&self) -> &str {
        &self.adoption_manifest_digest
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

    pub fn execution_envelope_digest(&self) -> &str {
        &self.execution_envelope_digest
    }

    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub const fn executor_row_count(&self) -> usize {
        self.executor_row_count
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }
}
