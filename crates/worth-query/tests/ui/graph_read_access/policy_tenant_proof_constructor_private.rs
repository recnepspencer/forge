use worth_query::facade::runtime::{
    WorthQueryGraphReadPolicyTenantPosture, WorthQueryGraphReadPolicyTenantProofBinding,
    WorthQueryGraphReadRelationshipProofBindingPosture,
};

fn main() {
    let _ = WorthQueryGraphReadPolicyTenantProofBinding {
        read_graph_digest: String::new(),
        policy_tenant_posture: WorthQueryGraphReadPolicyTenantPosture::SyntheticRuntimeCurrentRead,
        relationship_proof_posture: WorthQueryGraphReadRelationshipProofBindingPosture::NotRequired,
        relationship_proof_admission_digest: None,
    };
}
