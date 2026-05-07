use crate::canonicalization::CanonicalQueryArtifact;
use crate::policy_basis::AdmittedPolicyTenantContext;

use super::{
    RelationshipProofAdmission, RelationshipProofCounters, RelationshipProofDescriptor,
    RelationshipProofDescriptorSet, RelationshipProofError, RelationshipProofFailureClass,
    RelationshipProofTopologyClass,
};

pub fn admit_relationship_proofs(
    query: &CanonicalQueryArtifact,
    admitted: &AdmittedPolicyTenantContext,
    descriptor_set: &RelationshipProofDescriptorSet,
) -> Result<(RelationshipProofAdmission, RelationshipProofCounters), RelationshipProofError> {
    let mut counters = RelationshipProofCounters::default();
    let budget = descriptor_set.budget();
    if descriptor_set.descriptors().len() > budget.max_descriptors() {
        counters.deny();
        return Err(RelationshipProofError::new(
            RelationshipProofFailureClass::RelationshipProofBudgetExceeded,
            "relationship-proof descriptor count exceeds declared budget",
            counters,
        ));
    }

    let mut topology_width = 0usize;
    let mut topology_classes = Vec::new();
    for descriptor in descriptor_set.descriptors() {
        match descriptor {
            RelationshipProofDescriptor::DirectEdge { policy_digest, .. } => {
                if policy_digest != admitted.bundle().policy_digest() {
                    counters.deny();
                    return Err(RelationshipProofError::new(
                        RelationshipProofFailureClass::PolicyMismatch,
                        "relationship proof policy digest must match admitted policy basis",
                        counters,
                    ));
                }
                topology_width += 1;
                counters.admit(1);
                topology_classes.push(RelationshipProofTopologyClass::DirectEdge);
            }
            RelationshipProofDescriptor::BoundedAncestor {
                max_depth,
                policy_digest,
                ..
            } => {
                if *max_depth == 0 {
                    counters.deny_recursive_broadening();
                    return Err(RelationshipProofError::new(
                        RelationshipProofFailureClass::UnboundedProofTopology,
                        "bounded ancestor proof requires a non-zero explicit bound",
                        counters,
                    ));
                }
                if policy_digest != admitted.bundle().policy_digest() {
                    counters.deny();
                    return Err(RelationshipProofError::new(
                        RelationshipProofFailureClass::PolicyMismatch,
                        "relationship proof policy digest must match admitted policy basis",
                        counters,
                    ));
                }
                let width = usize::from(*max_depth);
                topology_width += width;
                counters.admit(width);
                topology_classes.push(RelationshipProofTopologyClass::BoundedAncestor);
            }
            RelationshipProofDescriptor::BoundedDescendant {
                max_depth,
                policy_digest,
                ..
            } => {
                if *max_depth == 0 {
                    counters.deny_recursive_broadening();
                    return Err(RelationshipProofError::new(
                        RelationshipProofFailureClass::UnboundedProofTopology,
                        "bounded descendant proof requires a non-zero explicit bound",
                        counters,
                    ));
                }
                if policy_digest != admitted.bundle().policy_digest() {
                    counters.deny();
                    return Err(RelationshipProofError::new(
                        RelationshipProofFailureClass::PolicyMismatch,
                        "relationship proof policy digest must match admitted policy basis",
                        counters,
                    ));
                }
                let width = usize::from(*max_depth);
                topology_width += width;
                counters.admit(width);
                topology_classes.push(RelationshipProofTopologyClass::BoundedDescendant);
            }
            RelationshipProofDescriptor::TenantMembership {
                tenant_schema_basis_digest,
            } => {
                if tenant_schema_basis_digest != admitted.bundle().tenant_schema_basis_digest() {
                    counters.deny();
                    return Err(RelationshipProofError::new(
                        RelationshipProofFailureClass::TenantSchemaMismatch,
                        "tenant membership proof must bind the admitted tenant schema basis",
                        counters,
                    ));
                }
                topology_width += 1;
                counters.admit(1);
                topology_classes.push(RelationshipProofTopologyClass::TenantMembership);
            }
            RelationshipProofDescriptor::QueryShapeMismatch {
                expected_query_digest,
            } => {
                if expected_query_digest != query.digest().as_str() {
                    counters.deny();
                    return Err(RelationshipProofError::new(
                        RelationshipProofFailureClass::QueryShapeMismatch,
                        "relationship proof descriptor must bind the canonical query shape",
                        counters,
                    ));
                }
            }
            RelationshipProofDescriptor::UnboundedRecursiveWalk { .. } => {
                counters.deny_recursive_broadening();
                return Err(RelationshipProofError::new(
                    RelationshipProofFailureClass::UnboundedRecursiveWalk,
                    "unbounded recursive relationship proofs are denied before truth touch",
                    counters,
                ));
            }
            RelationshipProofDescriptor::HostCallbackForbidden { .. } => {
                counters.deny_host_callback();
                return Err(RelationshipProofError::new(
                    RelationshipProofFailureClass::HostCallbackForbidden,
                    "relationship proofs must be typed query descriptors, not host callbacks",
                    counters,
                ));
            }
        }
    }

    if topology_width > budget.max_topology_width() {
        counters.deny();
        return Err(RelationshipProofError::new(
            RelationshipProofFailureClass::RelationshipProofBudgetExceeded,
            "relationship-proof topology width exceeds declared budget",
            counters,
        ));
    }

    Ok((
        RelationshipProofAdmission::new(
            query.digest().as_str(),
            admitted.bundle().policy_digest(),
            admitted.bundle().tenant_schema_basis_digest(),
            descriptor_set.descriptors(),
            topology_classes,
            budget,
        ),
        counters,
    ))
}
