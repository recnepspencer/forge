use super::super::birth_scaffold::{
    lower_family_birth_scaffold_plan, PrimitiveConstructionBirthScaffoldPlan,
};
use super::super::error_mapping::map_realization_geometry;
use super::super::geometry::simplex_vertices;
use super::super::scalar_admission::{decode_non_negative_scalar, decode_positive_scalar};
use super::super::topology_counts::PrimitiveConstructionTopologyCounts;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use worth_geom::facade::realize_tetrahedron_support_with_altitude_component;
use worth_spatial::facade::bindings::PrimitiveConstructionBirthScaffoldInput;
use worth_spatial::facade::placement::AdmittedSpatialPlacement;

struct AdmittedSimplexSolidBirthParameters {
    scale: f64,
    auxiliary_altitude_component: f64,
}

pub(in super::super) fn build_simplex_solid_birth_input(
    placement: &AdmittedSpatialPlacement,
    intent_digest: &str,
    scale_bits: u64,
    auxiliary_altitude_component_bits: u64,
) -> Result<PrimitiveConstructionBirthScaffoldInput, PrimitiveConstructionPhaseError> {
    let admitted =
        admit_simplex_solid_birth_parameters(scale_bits, auxiliary_altitude_component_bits)?;
    let realization = realize_tetrahedron_support_with_altitude_component(
        [0.0, 0.0, 0.0],
        admitted.scale,
        admitted.auxiliary_altitude_component,
    )
    .map_err(map_realization_geometry)?;
    lower_family_birth_scaffold_plan(
        intent_digest,
        placement,
        PrimitiveConstructionBirthScaffoldPlan::from_realized_support(
            PrimitiveConstructionFamily::SimplexSolid,
            realization.planes().to_vec(),
            simplex_vertices(admitted.scale, admitted.auxiliary_altitude_component),
            realization.report().clone(),
            PrimitiveConstructionTopologyCounts::new(4, 6, 4, 0, 4, 1, 1),
        ),
    )
}

fn admit_simplex_solid_birth_parameters(
    scale_bits: u64,
    auxiliary_altitude_component_bits: u64,
) -> Result<AdmittedSimplexSolidBirthParameters, PrimitiveConstructionPhaseError> {
    let family = PrimitiveConstructionFamily::SimplexSolid;
    Ok(AdmittedSimplexSolidBirthParameters {
        scale: decode_positive_scalar(family, scale_bits, "scale must stay finite and positive")?,
        auxiliary_altitude_component: decode_non_negative_scalar(
            family,
            auxiliary_altitude_component_bits,
            "auxiliary altitude component must stay finite and non-negative",
        )?,
    })
}
