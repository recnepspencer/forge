use worth_query::facade::policy::{
    AdmittedBasisCapability, BasisAuthorityPosture, BasisEligibilityCounters,
    BasisOperationLaneRequest, BasisTenantSchemaPosture, NormalizedBasisFamily,
};

fn main() {
    let _ = AdmittedBasisCapability {
        normalized_basis_intent_digest: String::new(),
        family: NormalizedBasisFamily::CurrentHead,
        authority_posture: BasisAuthorityPosture::RuntimeBackedCurrentHead,
        operation_lane: BasisOperationLaneRequest::Observation,
        tenant_schema_posture: BasisTenantSchemaPosture::Unscoped,
        counters: BasisEligibilityCounters {
            consulted_row_count: 1,
            tenant_check_count: 0,
            policy_check_count: 0,
            lower_runtime_check_count: 0,
            denied_residue_count: 0,
        },
        capability_digest: String::new(),
    };
}
