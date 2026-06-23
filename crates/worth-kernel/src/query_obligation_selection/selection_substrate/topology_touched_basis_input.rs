use forge_query::facade::runtime::ForgeQueryGraphObligationOperatingWorldDescriptor;
use topology::facade::{
    TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedOperatingWorld,
    TopologyTouchedOperatingWorldPosture,
};

use super::selection_error::QueryObligationSelectionError;
use super::selection_request::{
    QueryObligationSelectionAuthorityKind, QueryObligationSelectionInput,
};

impl QueryObligationSelectionInput {
    pub fn from_topology_touched_basis(
        touched_basis: &TopologyDeclaredTouchedGraphBasisProof,
    ) -> Result<Self, QueryObligationSelectionError> {
        Self::from_authority_parts(
            touched_basis.touch_descriptor().clone(),
            query_operating_world_from_topology_world(touched_basis.operating_world()),
            touched_basis.basis_digest(),
            QueryObligationSelectionAuthorityKind::TopologyTouchedBasis,
        )
    }
}

fn query_operating_world_from_topology_world(
    operating_world: &TopologyTouchedOperatingWorld,
) -> ForgeQueryGraphObligationOperatingWorldDescriptor {
    match operating_world.posture() {
        TopologyTouchedOperatingWorldPosture::Mainline => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority()
        }
        TopologyTouchedOperatingWorldPosture::Branch => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::branch()
        }
        TopologyTouchedOperatingWorldPosture::Preview => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::preview()
        }
        TopologyTouchedOperatingWorldPosture::ConfiguredDomainHandle => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle()
        }
    }
}
