use forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionBackedAdoptionProof;
use forge_query::facade::ForgeQueryGraphObligationExecutionResultEnvelope;

#[derive(Clone, Debug)]
pub struct WorthTopologySelectedGraphObligationExecutionInput {
    query_execution_envelope: ForgeQueryGraphObligationExecutionResultEnvelope,
    execution_backed_adoption_proof: ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    input_digest: String,
}

impl WorthTopologySelectedGraphObligationExecutionInput {
    pub fn from_query_authority(
        query_execution_envelope: ForgeQueryGraphObligationExecutionResultEnvelope,
        execution_backed_adoption_proof: ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    ) -> Self {
        let input_digest = [
            "worth-topo-selected-graph-obligation-execution-input-v1",
            query_execution_envelope.envelope_digest(),
            execution_backed_adoption_proof.manifest().manifest_digest(),
            execution_backed_adoption_proof
                .execution_proof()
                .proof_digest(),
        ]
        .join("|");
        Self {
            query_execution_envelope,
            execution_backed_adoption_proof,
            input_digest,
        }
    }

    pub const fn query_execution_envelope(
        &self,
    ) -> &ForgeQueryGraphObligationExecutionResultEnvelope {
        &self.query_execution_envelope
    }

    pub const fn execution_backed_adoption_proof(
        &self,
    ) -> &ForgeQueryGraphObligationExecutionBackedAdoptionProof {
        &self.execution_backed_adoption_proof
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }
}
