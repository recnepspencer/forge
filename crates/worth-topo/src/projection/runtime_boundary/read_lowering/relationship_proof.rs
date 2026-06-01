use forge_query::facade::{
    admit_policy_tenant_context, admit_relationship_proofs, BranchAccessGrant,
    CanonicalQueryArtifact, ExecutionBasisIntent, PolicyEpoch, PolicyExecutionModeRequest,
    PolicyRuleSnapshot, RelationshipProofBudget, RelationshipProofDescriptor,
    RelationshipProofDescriptorSet, RelationshipProofTopologyClass, SchemaVariantSnapshot,
    SnapshotLineageClass, TenantBasisEpoch, TenantBindingSnapshot,
};

use super::TopologyReadRelationshipProofPosture;
use crate::projection::read_views::domain::error::TopologyReadError;
use crate::projection::read_views::domain::request::TopologyReadRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RelationshipProofLowering {
    pub(super) posture: TopologyReadRelationshipProofPosture,
    pub(super) admission_identity: Option<String>,
    pub(super) topology_classes: Vec<RelationshipProofTopologyClass>,
    pub(super) admission_count: usize,
    pub(super) topology_width: usize,
    pub(super) support_profile_digest: String,
}

pub(super) fn admit_topology_read_relationship_proofs(
    request: &TopologyReadRequest,
    canonical_query: &CanonicalQueryArtifact,
) -> Result<RelationshipProofLowering, TopologyReadError> {
    let relationship_proof_support_profile =
        forge_query::facade::runtime_backed_relationship_proof_support_profile();
    let policy = PolicyRuleSnapshot::synthetic_authority(
        "topology-read",
        "topology-read-rules",
        PolicyEpoch::Synthetic(1),
    );
    let admitted_policy_context = admit_policy_tenant_context(
        canonical_query,
        policy.clone(),
        TenantBindingSnapshot::synthetic_direct(
            "tenant-a",
            "branch-a",
            "schema-a",
            TenantBasisEpoch::Synthetic(1),
        ),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "compatible"),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .map_err(|error| TopologyReadError::canonical_lowering_resolution(format!("{error:?}")))?;
    let descriptor_set = relationship_proof_descriptor_set(
        request,
        admitted_policy_context.bundle().policy_digest(),
    );
    let (relationship_proof_admission, relationship_proof_counters) =
        admit_relationship_proofs(canonical_query, &admitted_policy_context, &descriptor_set)
            .map_err(|error| {
                TopologyReadError::canonical_lowering_resolution(format!("{error:?}"))
            })?;
    Ok(RelationshipProofLowering {
        posture: TopologyReadRelationshipProofPosture::Admitted,
        admission_identity: Some(relationship_proof_admission.identity().as_str().to_string()),
        topology_classes: relationship_proof_admission.topology_classes().to_vec(),
        admission_count: relationship_proof_counters.relationship_proof_admission_count(),
        topology_width: relationship_proof_counters.relationship_proof_topology_width(),
        support_profile_digest: relationship_proof_support_profile
            .profile_digest()
            .to_string(),
    })
}

pub(super) fn runtime_basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        forge_query::facade::BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

fn relationship_proof_descriptor_set(
    request: &TopologyReadRequest,
    policy_digest: &str,
) -> RelationshipProofDescriptorSet {
    let descriptors = request
        .traversal_steps()
        .into_iter()
        .map(|step| {
            if step.depth() == 1 {
                RelationshipProofDescriptor::direct_edge_relation_name(
                    step.relation_name(),
                    policy_digest.to_string(),
                )
            } else {
                RelationshipProofDescriptor::bounded_ancestor_relation_name(
                    step.relation_name(),
                    step.depth(),
                    policy_digest.to_string(),
                )
                .expect("validated traversal steps must admit bounded-ancestor descriptors")
            }
        })
        .collect::<Vec<_>>();
    let topology_width = descriptors
        .iter()
        .map(|descriptor| match descriptor {
            RelationshipProofDescriptor::DirectEdge { .. } => 1,
            RelationshipProofDescriptor::BoundedAncestor { max_depth, .. }
            | RelationshipProofDescriptor::BoundedDescendant { max_depth, .. } => {
                usize::from(*max_depth)
            }
            RelationshipProofDescriptor::TenantMembership { .. }
            | RelationshipProofDescriptor::QueryShapeMismatch { .. }
            | RelationshipProofDescriptor::UnboundedRecursiveWalk { .. }
            | RelationshipProofDescriptor::HostCallbackForbidden { .. } => 0,
        })
        .sum();
    RelationshipProofDescriptorSet::new(
        descriptors.clone(),
        RelationshipProofBudget::bounded(descriptors.len(), topology_width),
    )
}
