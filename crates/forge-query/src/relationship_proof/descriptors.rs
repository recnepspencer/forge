use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RelationshipProofTopologyClass {
    DirectEdge,
    BoundedAncestor,
    TenantMembership,
}

impl RelationshipProofTopologyClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectEdge => "direct_edge",
            Self::BoundedAncestor => "bounded_ancestor",
            Self::TenantMembership => "tenant_membership",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RelationshipProofBudget {
    max_descriptors: usize,
    max_topology_width: usize,
}

impl RelationshipProofBudget {
    pub fn bounded(max_descriptors: usize, max_topology_width: usize) -> Self {
        Self {
            max_descriptors,
            max_topology_width,
        }
    }

    pub fn max_descriptors(&self) -> usize {
        self.max_descriptors
    }

    pub fn max_topology_width(&self) -> usize {
        self.max_topology_width
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "relationship_budget:{}:{}",
            self.max_descriptors, self.max_topology_width
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationshipProofDescriptor {
    DirectEdge {
        relation: String,
        policy_digest: String,
    },
    BoundedAncestor {
        relation: String,
        max_depth: u8,
        policy_digest: String,
    },
    TenantMembership {
        tenant_schema_basis_digest: String,
    },
    QueryShapeMismatch {
        expected_query_digest: String,
    },
    UnboundedRecursiveWalk {
        relation: String,
    },
    HostCallbackForbidden {
        callback_label: String,
    },
}

impl RelationshipProofDescriptor {
    pub fn direct_edge(relation: impl Into<String>, policy_digest: impl Into<String>) -> Self {
        Self::DirectEdge {
            relation: relation.into(),
            policy_digest: policy_digest.into(),
        }
    }

    pub fn bounded_ancestor(
        relation: impl Into<String>,
        max_depth: u8,
        policy_digest: impl Into<String>,
    ) -> Self {
        Self::BoundedAncestor {
            relation: relation.into(),
            max_depth,
            policy_digest: policy_digest.into(),
        }
    }

    pub fn tenant_membership(tenant_schema_basis_digest: impl Into<String>) -> Self {
        Self::TenantMembership {
            tenant_schema_basis_digest: tenant_schema_basis_digest.into(),
        }
    }

    pub fn query_shape_mismatch_for_test(expected_query_digest: impl Into<String>) -> Self {
        Self::QueryShapeMismatch {
            expected_query_digest: expected_query_digest.into(),
        }
    }

    pub fn unbounded_recursive_walk_for_test(relation: impl Into<String>) -> Self {
        Self::UnboundedRecursiveWalk {
            relation: relation.into(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn host_callback_for_test(callback_label: impl Into<String>) -> Self {
        Self::HostCallbackForbidden {
            callback_label: callback_label.into(),
        }
    }

    pub(crate) fn topology_width(&self) -> Option<usize> {
        match self {
            Self::DirectEdge { .. } | Self::TenantMembership { .. } => Some(1),
            Self::BoundedAncestor { max_depth, .. } => Some(usize::from(*max_depth)),
            Self::QueryShapeMismatch { .. }
            | Self::UnboundedRecursiveWalk { .. }
            | Self::HostCallbackForbidden { .. } => None,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        match self {
            Self::DirectEdge {
                relation,
                policy_digest,
            } => format!("direct:{relation}:{policy_digest}"),
            Self::BoundedAncestor {
                relation,
                max_depth,
                policy_digest,
            } => format!("ancestor:{relation}:{max_depth}:{policy_digest}"),
            Self::TenantMembership {
                tenant_schema_basis_digest,
            } => format!("tenant_membership:{tenant_schema_basis_digest}"),
            Self::QueryShapeMismatch {
                expected_query_digest,
            } => format!("query_shape_mismatch:{expected_query_digest}"),
            Self::UnboundedRecursiveWalk { relation } => format!("unbounded:{relation}"),
            Self::HostCallbackForbidden { callback_label } => {
                format!("host_callback:{callback_label}")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipProofDescriptorSet {
    descriptors: Vec<RelationshipProofDescriptor>,
    budget: RelationshipProofBudget,
}

impl RelationshipProofDescriptorSet {
    pub fn none() -> Self {
        Self {
            descriptors: Vec::new(),
            budget: RelationshipProofBudget::bounded(0, 0),
        }
    }

    pub fn new(
        descriptors: Vec<RelationshipProofDescriptor>,
        budget: RelationshipProofBudget,
    ) -> Self {
        Self {
            descriptors,
            budget,
        }
    }

    pub fn descriptors(&self) -> &[RelationshipProofDescriptor] {
        &self.descriptors
    }

    pub fn budget(&self) -> RelationshipProofBudget {
        self.budget
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipProofAdmissionIdentity(String);

impl RelationshipProofAdmissionIdentity {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipProofAdmission {
    identity: RelationshipProofAdmissionIdentity,
    topology_classes: Vec<RelationshipProofTopologyClass>,
    budget: RelationshipProofBudget,
    descriptor_count: usize,
}

impl RelationshipProofAdmission {
    pub(crate) fn new(
        query_digest: &str,
        policy_digest: &str,
        tenant_schema_basis_digest: &str,
        descriptors: &[RelationshipProofDescriptor],
        topology_classes: Vec<RelationshipProofTopologyClass>,
        budget: RelationshipProofBudget,
    ) -> Self {
        let mut parts = vec![
            format!("query:{query_digest}"),
            format!("policy:{policy_digest}"),
            format!("tenant_schema:{tenant_schema_basis_digest}"),
            budget.digest_part(),
        ];
        parts.extend(
            descriptors
                .iter()
                .map(RelationshipProofDescriptor::digest_part),
        );
        parts.extend(
            topology_classes
                .iter()
                .map(|topology| format!("topology:{}", topology.as_str())),
        );
        Self {
            identity: RelationshipProofAdmissionIdentity::new(hash_parts(&parts)),
            topology_classes,
            budget,
            descriptor_count: descriptors.len(),
        }
    }

    pub fn identity(&self) -> &RelationshipProofAdmissionIdentity {
        &self.identity
    }

    pub fn topology_classes(&self) -> &[RelationshipProofTopologyClass] {
        &self.topology_classes
    }

    pub fn budget(&self) -> RelationshipProofBudget {
        self.budget
    }

    pub fn descriptor_count(&self) -> usize {
        self.descriptor_count
    }
}
