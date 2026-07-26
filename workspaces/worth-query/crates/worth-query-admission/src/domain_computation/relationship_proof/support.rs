use crate::admission_digest::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RelationshipProofSurface {
    DescriptorAdmission,
    DirectEdgeTopology,
    BoundedAncestorTopology,
    BoundedDescendantTopology,
    TenantMembershipTopology,
    RuntimeProofEvaluation,
    HostCallbackProofs,
}

impl RelationshipProofSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DescriptorAdmission => "descriptor_admission",
            Self::DirectEdgeTopology => "direct_edge_topology",
            Self::BoundedAncestorTopology => "bounded_ancestor_topology",
            Self::BoundedDescendantTopology => "bounded_descendant_topology",
            Self::TenantMembershipTopology => "tenant_membership_topology",
            Self::RuntimeProofEvaluation => "runtime_proof_evaluation",
            Self::HostCallbackProofs => "host_callback_proofs",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RelationshipProofSupportStatus {
    Verified,
    Deferred,
    Forbidden,
}

impl RelationshipProofSupportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Deferred => "deferred",
            Self::Forbidden => "forbidden",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipProofSupportProfile {
    surfaces: Vec<(RelationshipProofSurface, RelationshipProofSupportStatus)>,
    profile_digest: String,
}

impl RelationshipProofSupportProfile {
    pub fn new(surfaces: Vec<(RelationshipProofSurface, RelationshipProofSupportStatus)>) -> Self {
        let profile_digest = hash_parts(
            &surfaces
                .iter()
                .map(|(surface, status)| format!("{}:{}", surface.as_str(), status.as_str()))
                .collect::<Vec<_>>(),
        );
        Self {
            surfaces,
            profile_digest,
        }
    }

    pub fn surfaces(&self) -> &[(RelationshipProofSurface, RelationshipProofSupportStatus)] {
        &self.surfaces
    }

    pub fn profile_digest(&self) -> &str {
        &self.profile_digest
    }
}

pub fn runtime_backed_relationship_proof_support_profile() -> RelationshipProofSupportProfile {
    RelationshipProofSupportProfile::new(vec![
        (
            RelationshipProofSurface::DescriptorAdmission,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::DirectEdgeTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::BoundedAncestorTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::BoundedDescendantTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::TenantMembershipTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::RuntimeProofEvaluation,
            RelationshipProofSupportStatus::Deferred,
        ),
        (
            RelationshipProofSurface::HostCallbackProofs,
            RelationshipProofSupportStatus::Forbidden,
        ),
    ])
}
