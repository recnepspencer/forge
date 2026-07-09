use worth_query::facade::consumer_kit::{
    WorthQueryGraphObligationAdoptionProof, WorthQueryGraphObligationExecutionProof,
    WorthQueryGraphObligationExecutionBackedAdoptionProof,
    WorthQueryGraphObligationInMemoryProof, WorthQueryGraphObligationLocalCeremonyAudit,
    WorthQueryGraphObligationResidueManifest, WorthQueryGraphObligationSupportPin,
};

fn require_execution_proof(_: WorthQueryGraphObligationExecutionProof) {}

fn require_execution_backed_adoption(_: WorthQueryGraphObligationExecutionBackedAdoptionProof) {}

fn adoption_posture_is_not_execution_proof(proof: WorthQueryGraphObligationAdoptionProof) {
    require_execution_proof(proof);
}

fn selection_only_proof_is_not_execution_proof(proof: WorthQueryGraphObligationInMemoryProof) {
    require_execution_proof(proof);
}

fn support_pin_is_not_execution_proof(pin: WorthQueryGraphObligationSupportPin) {
    require_execution_proof(pin);
}

fn local_ceremony_audit_is_not_execution_proof(audit: WorthQueryGraphObligationLocalCeremonyAudit) {
    require_execution_proof(audit);
}

fn residue_manifest_is_not_execution_proof(manifest: WorthQueryGraphObligationResidueManifest) {
    require_execution_proof(manifest);
}

fn selection_only_proof_is_not_execution_backed_adoption(
    proof: WorthQueryGraphObligationInMemoryProof,
) {
    require_execution_backed_adoption(proof);
}

fn adoption_posture_is_not_execution_backed_adoption(
    proof: WorthQueryGraphObligationAdoptionProof,
) {
    require_execution_backed_adoption(proof);
}

fn main() {}
