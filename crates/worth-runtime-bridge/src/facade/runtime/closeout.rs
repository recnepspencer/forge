use super::*;

impl RuntimeBridge {
    pub fn public_authoritative_mutation_evidence_support(
    ) -> BridgeAuthoritativeMutationEvidenceSupport {
        BridgeAuthoritativeMutationEvidenceSupport::standard()
    }

    pub fn public_authoritative_mutation_evidence_closeout(
    ) -> BridgeAuthoritativeMutationEvidenceCloseout {
        BridgeAuthoritativeMutationEvidenceCloseout::derive(
            &Self::public_authoritative_mutation_evidence_support(),
        )
    }
}
