use topology::facade::TopologyConstructionQueryFactKind;

use super::{
    PlanarTopologyContractCompletenessBasis, PlanarTopologyContractCompletenessDenial,
    PlanarTopologyContractCompletenessDenialKind,
};

pub(crate) fn validate_planar_topology_contract_completeness_basis(
    basis: &PlanarTopologyContractCompletenessBasis,
) -> Result<(), PlanarTopologyContractCompletenessDenial> {
    require_declared_query_surface(basis)?;
    require_planar_neighborhood(basis)?;
    require_loop_basis(basis)?;
    require_shell_basis(basis)?;
    require_orientation_basis(basis)?;
    require_validation_surface(basis)?;
    reject_contradictory_topology_facts(basis)
}

fn require_declared_query_surface(
    basis: &PlanarTopologyContractCompletenessBasis,
) -> Result<(), PlanarTopologyContractCompletenessDenial> {
    if basis.declared_query_surface_identity().is_empty() {
        Err(denial(
            PlanarTopologyContractCompletenessDenialKind::MissingDeclaredQuerySurface,
            "topology completeness must consume a declared Query surface identity",
        ))
    } else {
        Ok(())
    }
}

fn require_planar_neighborhood(
    basis: &PlanarTopologyContractCompletenessBasis,
) -> Result<(), PlanarTopologyContractCompletenessDenial> {
    if basis.planar_neighborhood_identity().is_empty()
        || basis.fact_count(TopologyConstructionQueryFactKind::PlanarNeighborhoodBasis) == 0
    {
        Err(denial(
            PlanarTopologyContractCompletenessDenialKind::MissingNeighborhoodBasis,
            "topology completeness requires a planar-neighborhood basis fact",
        ))
    } else {
        Ok(())
    }
}

fn require_loop_basis(
    basis: &PlanarTopologyContractCompletenessBasis,
) -> Result<(), PlanarTopologyContractCompletenessDenial> {
    if basis.fact_count(TopologyConstructionQueryFactKind::LoopMembership) == 0
        || basis.fact_count(TopologyConstructionQueryFactKind::LoopClosure) == 0
    {
        Err(denial(
            PlanarTopologyContractCompletenessDenialKind::MissingLoopBasis,
            "topology completeness requires loop membership and loop closure facts",
        ))
    } else {
        Ok(())
    }
}

fn require_shell_basis(
    basis: &PlanarTopologyContractCompletenessBasis,
) -> Result<(), PlanarTopologyContractCompletenessDenial> {
    if basis.fact_count(TopologyConstructionQueryFactKind::ShellMembership) == 0
        || basis.fact_count(TopologyConstructionQueryFactKind::ShellClosure) == 0
    {
        Err(denial(
            PlanarTopologyContractCompletenessDenialKind::MissingShellBasis,
            "topology completeness requires shell membership and shell closure facts",
        ))
    } else {
        Ok(())
    }
}

fn require_orientation_basis(
    basis: &PlanarTopologyContractCompletenessBasis,
) -> Result<(), PlanarTopologyContractCompletenessDenial> {
    if basis.fact_count(TopologyConstructionQueryFactKind::FaceOrientation) == 0 {
        Err(denial(
            PlanarTopologyContractCompletenessDenialKind::MissingOrientationBasis,
            "topology completeness requires face-orientation facts",
        ))
    } else {
        Ok(())
    }
}

fn require_validation_surface(
    basis: &PlanarTopologyContractCompletenessBasis,
) -> Result<(), PlanarTopologyContractCompletenessDenial> {
    if basis.fact_count(TopologyConstructionQueryFactKind::ValidationSurface) == 0 {
        Err(denial(
            PlanarTopologyContractCompletenessDenialKind::MissingValidationSurface,
            "topology completeness requires a topology validation surface fact",
        ))
    } else {
        Ok(())
    }
}

fn reject_contradictory_topology_facts(
    basis: &PlanarTopologyContractCompletenessBasis,
) -> Result<(), PlanarTopologyContractCompletenessDenial> {
    let loops = basis.fact_count(TopologyConstructionQueryFactKind::LoopMembership);
    let faces = basis.fact_count(TopologyConstructionQueryFactKind::FaceMembership);
    let shells = basis.fact_count(TopologyConstructionQueryFactKind::ShellMembership);
    if loops > 0 && (faces == 0 || shells == 0) {
        Err(denial(
            PlanarTopologyContractCompletenessDenialKind::ContradictoryTopologyFacts,
            "loop facts cannot feed planar work without matching face and shell facts",
        ))
    } else {
        Ok(())
    }
}

fn denial(
    kind: PlanarTopologyContractCompletenessDenialKind,
    reason: &'static str,
) -> PlanarTopologyContractCompletenessDenial {
    PlanarTopologyContractCompletenessDenial::new(kind, reason)
}
