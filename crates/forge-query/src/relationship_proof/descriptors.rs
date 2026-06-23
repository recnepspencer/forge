use crate::authoring::{AuthoringError, RelationName};
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RelationshipProofTopologyClass {
    DirectEdge,
    BoundedAncestor,
    BoundedDescendant,
    TenantMembership,
}

impl RelationshipProofTopologyClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectEdge => "direct_edge",
            Self::BoundedAncestor => "bounded_ancestor",
            Self::BoundedDescendant => "bounded_descendant",
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
    BoundedDescendant {
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
        Self::direct_edge_relation_name(
            RelationName::new(relation).expect("relationship proof relation must be non-empty"),
            policy_digest,
        )
    }

    pub fn direct_edge_relation_name(
        relation: RelationName,
        policy_digest: impl Into<String>,
    ) -> Self {
        Self::DirectEdge {
            relation: relation.as_str().to_string(),
            policy_digest: policy_digest.into(),
        }
    }

    pub fn bounded_ancestor(
        relation: impl Into<String>,
        max_depth: u8,
        policy_digest: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        Self::bounded_ancestor_relation_name(
            RelationName::new(relation).expect("relationship proof relation must be non-empty"),
            max_depth,
            policy_digest,
        )
    }

    pub fn bounded_ancestor_relation_name(
        relation: RelationName,
        max_depth: u8,
        policy_digest: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        if max_depth == 0 {
            return Err(AuthoringError::UnsupportedTraversalDepth { depth: 0 });
        }
        Ok(Self::BoundedAncestor {
            relation: relation.as_str().to_string(),
            max_depth,
            policy_digest: policy_digest.into(),
        })
    }

    pub fn bounded_descendant(
        relation: impl Into<String>,
        max_depth: u8,
        policy_digest: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        Self::bounded_descendant_relation_name(
            RelationName::new(relation).expect("relationship proof relation must be non-empty"),
            max_depth,
            policy_digest,
        )
    }

    pub fn bounded_descendant_relation_name(
        relation: RelationName,
        max_depth: u8,
        policy_digest: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        if max_depth == 0 {
            return Err(AuthoringError::UnsupportedTraversalDepth { depth: 0 });
        }
        Ok(Self::BoundedDescendant {
            relation: relation.as_str().to_string(),
            max_depth,
            policy_digest: policy_digest.into(),
        })
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
        Self::unbounded_recursive_walk_relation_name_for_test(
            RelationName::new(relation).expect("relationship proof relation must be non-empty"),
        )
    }

    pub fn unbounded_recursive_walk_relation_name_for_test(relation: RelationName) -> Self {
        Self::UnboundedRecursiveWalk {
            relation: relation.as_str().to_string(),
        }
    }

    #[cfg(test)]
    pub(crate) fn host_callback_for_test(callback_label: impl Into<String>) -> Self {
        Self::HostCallbackForbidden {
            callback_label: callback_label.into(),
        }
    }

    pub(crate) fn topology_width(&self) -> Option<usize> {
        match self {
            Self::DirectEdge { .. } | Self::TenantMembership { .. } => Some(1),
            Self::BoundedAncestor { max_depth, .. } | Self::BoundedDescendant { max_depth, .. } => {
                Some(usize::from(*max_depth))
            }
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
            Self::BoundedDescendant {
                relation,
                max_depth,
                policy_digest,
            } => format!("descendant:{relation}:{max_depth}:{policy_digest}"),
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
    policy_digest: String,
    tenant_schema_basis_digest: String,
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
            policy_digest: policy_digest.to_string(),
            tenant_schema_basis_digest: tenant_schema_basis_digest.to_string(),
            topology_classes,
            budget,
            descriptor_count: descriptors.len(),
        }
    }

    pub fn identity(&self) -> &RelationshipProofAdmissionIdentity {
        &self.identity
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_schema_basis_digest(&self) -> &str {
        &self.tenant_schema_basis_digest
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

#[cfg(test)]
mod tests {
    use super::RelationshipProofDescriptor;
    use crate::authoring::{AuthoringError, RelationName};

    #[test]
    fn relationship_proof_descriptors_can_reuse_validated_relation_names() {
        let relation = RelationName::new("worth.half_edge_next").expect("valid relation");
        let direct =
            RelationshipProofDescriptor::direct_edge_relation_name(relation.clone(), "policy-a");
        let ancestor = RelationshipProofDescriptor::bounded_ancestor_relation_name(
            relation.clone(),
            4,
            "policy-a",
        )
        .expect("validated bounded-ancestor descriptors should construct");
        let denied =
            RelationshipProofDescriptor::unbounded_recursive_walk_relation_name_for_test(relation);

        assert_eq!(direct.digest_part(), "direct:worth.half_edge_next:policy-a");
        assert_eq!(
            ancestor.digest_part(),
            "ancestor:worth.half_edge_next:4:policy-a"
        );
        assert_eq!(denied.digest_part(), "unbounded:worth.half_edge_next");
    }

    #[test]
    fn bounded_ancestor_relation_name_rejects_zero_depth() {
        let relation = RelationName::new("worth.half_edge_next").expect("valid relation");
        let error =
            RelationshipProofDescriptor::bounded_ancestor_relation_name(relation, 0, "policy-a")
                .expect_err("zero-depth bounded ancestors must fail at construction");

        assert!(matches!(
            error,
            AuthoringError::UnsupportedTraversalDepth { depth: 0 }
        ));
    }
}
