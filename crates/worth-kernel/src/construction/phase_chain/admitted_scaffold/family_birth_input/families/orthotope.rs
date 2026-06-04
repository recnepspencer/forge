use super::super::birth_scaffold::{
    lower_family_birth_scaffold_plan, PrimitiveConstructionBirthScaffoldPlan,
};
use super::super::error_mapping::map_geometry;
use super::super::geometry::orthotope_vertices;
use super::super::scalar_admission::decode_positive_triplet;
use super::super::topology_counts::PrimitiveConstructionTopologyCounts;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use worth_geom::facade::realize_block_support;
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::PrimitiveConstructionBirthScaffoldInput;
use worth_spatial::facade::placement::AdmittedSpatialPlacement;

struct AdmittedOrthotopeBirthParameters {
    half_extents: [f64; 3],
}

pub(in super::super) fn build_orthotope_birth_input(
    placement: &AdmittedSpatialPlacement,
    intent_digest: &str,
    half_extents_bits: [u64; 3],
) -> Result<PrimitiveConstructionBirthScaffoldInput, PrimitiveConstructionPhaseError> {
    let admitted = admit_orthotope_birth_parameters(half_extents_bits)?;
    let realization =
        realize_block_support([0.0, 0.0, 0.0], admitted.half_extents).map_err(map_geometry)?;
    let birth_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    lower_family_birth_scaffold_plan(
        intent_digest,
        placement,
        PrimitiveConstructionBirthScaffoldPlan::from_realized_support(
            PrimitiveConstructionFamily::Orthotope,
            birth_contract,
            realization.planes().to_vec(),
            orthotope_vertices(admitted.half_extents),
            realization.report().clone(),
            PrimitiveConstructionTopologyCounts::from_contract(birth_contract.topology_contract()),
        ),
    )
}

fn admit_orthotope_birth_parameters(
    half_extents_bits: [u64; 3],
) -> Result<AdmittedOrthotopeBirthParameters, PrimitiveConstructionPhaseError> {
    Ok(AdmittedOrthotopeBirthParameters {
        half_extents: decode_positive_triplet(
            PrimitiveConstructionFamily::Orthotope,
            half_extents_bits,
            "orthotope half-extents must stay finite and positive",
        )?,
    })
}
