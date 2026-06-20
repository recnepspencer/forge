use forge_query::facade::runtime::{
    ForgeQueryGraphReadPolicyTenantPosture, ForgeQueryGraphReadPolicyTenantProofBinding,
    ForgeQueryGraphReadRelationshipProofBindingPosture,
};

fn main() {
    let _ = ForgeQueryGraphReadPolicyTenantProofBinding {
        read_graph_digest: String::new(),
        policy_tenant_posture: ForgeQueryGraphReadPolicyTenantPosture::SyntheticRuntimeCurrentRead,
        relationship_proof_posture: ForgeQueryGraphReadRelationshipProofBindingPosture::NotRequired,
        relationship_proof_admission_digest: None,
    };
}
