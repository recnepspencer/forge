use crate::facade::{
    primitive_birth_contract_matches_counts, primitive_birth_contract_matches_support_planes,
    PrimitiveConstructionBirthContractCounts, PrimitiveConstructionBirthScaffoldInput,
    SpatialConstructionBirthError,
};

pub(super) fn validate_primitive_construction_birth_input(
    input: &PrimitiveConstructionBirthScaffoldInput,
) -> Result<(), SpatialConstructionBirthError> {
    if input.vertex_positions().len() != input.expected_vertex_count() {
        return Err(
            SpatialConstructionBirthError::InvalidPrimitiveBirthScaffold(
                "vertex positions must match the declared construction vertex count",
            ),
        );
    }
    if input
        .vertex_positions()
        .iter()
        .any(|position| position.iter().any(|value| !value.is_finite()))
    {
        return Err(
            SpatialConstructionBirthError::InvalidPrimitiveBirthScaffold(
                "construction birth positions must stay finite",
            ),
        );
    }
    let counts = PrimitiveConstructionBirthContractCounts::from_input(input);
    let valid_shape = primitive_birth_contract_matches_counts(input.family(), counts)
        && primitive_birth_contract_matches_support_planes(
            input.family(),
            input.support_planes().len(),
            counts,
        );
    if !valid_shape {
        return Err(
            SpatialConstructionBirthError::InvalidPrimitiveBirthScaffold(
                "construction birth counts must match the admitted primitive family contract",
            ),
        );
    }
    Ok(())
}
