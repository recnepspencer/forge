use super::super::super::super::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionPhaseError,
};
use super::super::birth_scaffold::{
    lower_family_birth_scaffold_plan, PrimitiveConstructionBirthScaffoldPlan,
};
use super::super::error_mapping::map_realization_geometry;
use super::super::geometry::pyramid_vertices;
use super::super::scalar_admission::{
    admit_polygon_edge_count, decode_non_negative_scalar, decode_positive_scalar,
};
use super::super::topology_counts::PrimitiveConstructionTopologyCounts;
use super::super::PrimitiveConstructionAdmittedBirthInput;
use worth_geom::facade::realize_pyramid_support;
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::placement::SpatialPlacementSpec;

struct AdmittedRegularPyramidBirthParameters {
    sides: u32,
    radius: f64,
    height: f64,
}

pub(in super::super) fn build_regular_pyramid_birth_input(
    placement_spec: SpatialPlacementSpec,
    intent_digest: &str,
    sides: u32,
    radius_bits: u64,
    height_bits: u64,
) -> Result<PrimitiveConstructionAdmittedBirthInput, PrimitiveConstructionPhaseError> {
    let admitted = admit_regular_pyramid_birth_parameters(sides, radius_bits, height_bits)?;
    let realization = realize_pyramid_support(
        [0.0, 0.0, 0.0],
        admitted.sides,
        admitted.radius,
        admitted.height,
    )
    .map_err(map_realization_geometry)?;
    let birth_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::RegularPyramid {
            side_count: admitted.sides,
        },
    );
    lower_family_birth_scaffold_plan(
        intent_digest,
        placement_spec,
        PrimitiveConstructionBirthScaffoldPlan::from_realized_support_facts(
            PrimitiveConstructionFamily::RegularPyramid,
            birth_contract,
            realization.planes().to_vec(),
            pyramid_vertices(admitted.sides, admitted.radius, admitted.height),
            realization.report().clone(),
            PrimitiveConstructionTopologyCounts::from_contract(birth_contract.topology_contract()),
        ),
    )
}

fn admit_regular_pyramid_birth_parameters(
    sides: u32,
    radius_bits: u64,
    height_bits: u64,
) -> Result<AdmittedRegularPyramidBirthParameters, PrimitiveConstructionPhaseError> {
    let family = PrimitiveConstructionFamily::RegularPyramid;
    Ok(AdmittedRegularPyramidBirthParameters {
        sides: admit_polygon_edge_count(family, sides)?,
        radius: decode_non_negative_scalar(
            family,
            radius_bits,
            "radius must stay finite and non-negative",
        )?,
        height: decode_positive_scalar(
            family,
            height_bits,
            "height must stay finite and positive",
        )?,
    })
}
