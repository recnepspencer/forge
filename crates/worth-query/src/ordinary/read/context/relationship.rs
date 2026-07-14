use std::num::NonZeroU8;

use crate::authoring::RelationName;
use crate::relationship_proof::{
    RelationshipProofBudget, RelationshipProofDescriptor, RelationshipProofDescriptorSet,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryReadRelationshipDepth(NonZeroU8);

impl WorthQueryReadRelationshipDepth {
    pub fn new(max_depth: u8) -> Result<Self, WorthQueryReadRelationshipProofDeclarationError> {
        NonZeroU8::new(max_depth)
            .map(Self)
            .ok_or(WorthQueryReadRelationshipProofDeclarationError::ZeroDepth)
    }

    pub fn get(self) -> u8 {
        self.0.get()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadRelationshipProof {
    DirectEdge {
        relation: RelationName,
    },
    BoundedAncestor {
        relation: RelationName,
        max_depth: WorthQueryReadRelationshipDepth,
    },
    BoundedDescendant {
        relation: RelationName,
        max_depth: WorthQueryReadRelationshipDepth,
    },
    TenantMembership,
}

impl WorthQueryReadRelationshipProof {
    pub fn direct_edge(relation: RelationName) -> Self {
        Self::DirectEdge { relation }
    }

    pub fn bounded_ancestor(
        relation: RelationName,
        max_depth: WorthQueryReadRelationshipDepth,
    ) -> Self {
        Self::BoundedAncestor {
            relation,
            max_depth,
        }
    }

    pub fn bounded_descendant(
        relation: RelationName,
        max_depth: WorthQueryReadRelationshipDepth,
    ) -> Self {
        Self::BoundedDescendant {
            relation,
            max_depth,
        }
    }

    pub fn tenant_membership() -> Self {
        Self::TenantMembership
    }

    fn topology_width(&self) -> usize {
        match self {
            Self::DirectEdge { .. } | Self::TenantMembership => 1,
            Self::BoundedAncestor { max_depth, .. } | Self::BoundedDescendant { max_depth, .. } => {
                usize::from(max_depth.get())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadRelationshipProofDeclarationError {
    ZeroDepth,
    EmptyDeclaration,
    DescriptorBudgetExceeded { declared: usize, admitted: usize },
    TopologyBudgetExceeded { declared: usize, admitted: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadRelationshipProofs {
    proofs: Vec<WorthQueryReadRelationshipProof>,
    max_descriptors: usize,
    max_topology_width: usize,
}

impl WorthQueryReadRelationshipProofs {
    pub fn bounded<I>(
        proofs: I,
        max_descriptors: usize,
        max_topology_width: usize,
    ) -> Result<Self, WorthQueryReadRelationshipProofDeclarationError>
    where
        I: IntoIterator<Item = WorthQueryReadRelationshipProof>,
        I::IntoIter: ExactSizeIterator,
    {
        let proofs = proofs.into_iter();
        let declared_descriptor_count = proofs.len();
        if declared_descriptor_count == 0 {
            return Err(WorthQueryReadRelationshipProofDeclarationError::EmptyDeclaration);
        }
        if declared_descriptor_count > max_descriptors {
            return Err(
                WorthQueryReadRelationshipProofDeclarationError::DescriptorBudgetExceeded {
                    declared: declared_descriptor_count,
                    admitted: max_descriptors,
                },
            );
        }
        let proofs = proofs.collect::<Vec<_>>();
        let topology_width = proofs
            .iter()
            .map(WorthQueryReadRelationshipProof::topology_width)
            .fold(0usize, usize::saturating_add);
        if topology_width > max_topology_width {
            return Err(
                WorthQueryReadRelationshipProofDeclarationError::TopologyBudgetExceeded {
                    declared: topology_width,
                    admitted: max_topology_width,
                },
            );
        }
        Ok(Self {
            proofs,
            max_descriptors,
            max_topology_width,
        })
    }

    pub fn proofs(&self) -> &[WorthQueryReadRelationshipProof] {
        &self.proofs
    }

    pub(crate) fn lower(
        self,
        policy_digest: &str,
        tenant_schema_basis_digest: &str,
    ) -> RelationshipProofDescriptorSet {
        let mut descriptors = self
            .proofs
            .into_iter()
            .map(|proof| match proof {
                WorthQueryReadRelationshipProof::DirectEdge { relation } => {
                    RelationshipProofDescriptor::direct_edge_relation_name(relation, policy_digest)
                }
                WorthQueryReadRelationshipProof::BoundedAncestor {
                    relation,
                    max_depth,
                } => RelationshipProofDescriptor::bounded_ancestor_relation_name(
                    relation,
                    max_depth.get(),
                    policy_digest,
                )
                .expect("non-zero relationship depth is established by the declaration type"),
                WorthQueryReadRelationshipProof::BoundedDescendant {
                    relation,
                    max_depth,
                } => RelationshipProofDescriptor::bounded_descendant_relation_name(
                    relation,
                    max_depth.get(),
                    policy_digest,
                )
                .expect("non-zero relationship depth is established by the declaration type"),
                WorthQueryReadRelationshipProof::TenantMembership => {
                    RelationshipProofDescriptor::tenant_membership(tenant_schema_basis_digest)
                }
            })
            .collect::<Vec<_>>();
        descriptors.sort_by_key(RelationshipProofDescriptor::digest_part);
        RelationshipProofDescriptorSet::new(
            descriptors,
            RelationshipProofBudget::bounded(self.max_descriptors, self.max_topology_width),
        )
    }
}
