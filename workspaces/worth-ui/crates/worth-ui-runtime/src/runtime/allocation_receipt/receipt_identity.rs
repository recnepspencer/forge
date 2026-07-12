use crate::declaration::UiDeclarationIdentity;
use crate::evidence::{UiAllocationNeighborhoodScope, UiLayoutOperatorContractIdentity};
use crate::graph::UiGraphNodeIdentity;

/// Canonical per-neighborhood identity for committed allocation truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationReceiptIdentity {
    declaration_identity: UiDeclarationIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    neighborhood_scope: UiAllocationNeighborhoodScope,
    coordinate_ownership: UiLayoutOperatorContractIdentity,
    portal_anchor: Option<crate::runtime::UiPortalAnchorIdentity>,
}

impl UiAllocationReceiptIdentity {
    pub(crate) fn from_candidate(candidate: &super::UiAllocationCandidate) -> Self {
        let basis = candidate.measurement_basis();
        let neighborhood = candidate.allocation_neighborhood();
        Self {
            declaration_identity: basis.declaration_identity().clone(),
            graph_node_identity: basis.graph_node_identity(),
            neighborhood_scope: UiAllocationNeighborhoodScope::from_neighborhood(neighborhood),
            coordinate_ownership: neighborhood.identity().layout_operator_contract_identity(),
            portal_anchor: candidate
                .portal_allocation_input()
                .map(|basis| basis.observation().identity())
                .or_else(|| {
                    candidate
                        .measurement_basis()
                        .evidence_inputs()
                        .iter()
                        .find_map(|input| {
                            crate::runtime::UiPortalAnchorIdentity::from_measurement_result(
                                input.as_host_measurement_result()?,
                            )
                        })
                }),
        }
    }

    pub fn declaration_identity(&self) -> &UiDeclarationIdentity {
        &self.declaration_identity
    }
    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }
    pub fn neighborhood_scope(&self) -> &UiAllocationNeighborhoodScope {
        &self.neighborhood_scope
    }
    pub fn coordinate_ownership(&self) -> UiLayoutOperatorContractIdentity {
        self.coordinate_ownership
    }
    pub fn portal_anchor(&self) -> Option<crate::runtime::UiPortalAnchorIdentity> {
        self.portal_anchor
    }
    pub(crate) fn identity_digest(&self) -> u64 {
        self.declaration_identity.digest().raw()
            ^ self.graph_node_identity.digest().rotate_left(11)
            ^ self.neighborhood_scope.identity_digest().rotate_left(23)
            ^ self.coordinate_ownership.identity_digest().rotate_left(41)
            ^ self
                .portal_anchor
                .map_or(0, |identity| identity.identity_digest().rotate_left(53))
    }
}
