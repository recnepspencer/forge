use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationAdoptionProof, ForgeQueryGraphObligationExecutionProof,
    ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    ForgeQueryGraphObligationInMemoryProof, ForgeQueryGraphObligationLocalCeremonyAudit,
    ForgeQueryGraphObligationResidueManifest, ForgeQueryGraphObligationSupportPin,
};

fn require_execution_proof(_: ForgeQueryGraphObligationExecutionProof) {}

fn require_execution_backed_adoption(_: ForgeQueryGraphObligationExecutionBackedAdoptionProof) {}

fn adoption_posture_is_not_execution_proof(proof: ForgeQueryGraphObligationAdoptionProof) {
    require_execution_proof(proof);
}

fn selection_only_proof_is_not_execution_proof(proof: ForgeQueryGraphObligationInMemoryProof) {
    require_execution_proof(proof);
}

fn support_pin_is_not_execution_proof(pin: ForgeQueryGraphObligationSupportPin) {
    require_execution_proof(pin);
}

fn local_ceremony_audit_is_not_execution_proof(audit: ForgeQueryGraphObligationLocalCeremonyAudit) {
    require_execution_proof(audit);
}

fn residue_manifest_is_not_execution_proof(manifest: ForgeQueryGraphObligationResidueManifest) {
    require_execution_proof(manifest);
}

fn selection_only_proof_is_not_execution_backed_adoption(
    proof: ForgeQueryGraphObligationInMemoryProof,
) {
    require_execution_backed_adoption(proof);
}

fn adoption_posture_is_not_execution_backed_adoption(
    proof: ForgeQueryGraphObligationAdoptionProof,
) {
    require_execution_backed_adoption(proof);
}

fn main() {}
