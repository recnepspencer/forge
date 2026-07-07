#[path = "public_api_planar_boolean_loop_reconstruction_public_contract_support/mod.rs"]
mod public_contract_support;
#[path = "public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod workload_evidence_support;

#[test]
fn loop_reconstruction_public_contract_surfaces_preserve_real_workload_backed_identities() {
    public_contract_support::support::assert_loop_public_contract_surfaces_preserve_real_workload_backed_identities();
}

#[test]
fn loop_reconstruction_public_contract_fences_reject_foreign_authority() {
    public_contract_support::support::assert_loop_public_contract_fences_reject_foreign_authority();
}
