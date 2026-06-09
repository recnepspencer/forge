use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveFeatureConditioningClass,
    PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};
use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyKey,
};

use super::primitive_birth::{
    PrimitiveConstructionBirthScaffoldInput, SpatialConstructionBirthError,
};
use super::primitive_birth_consequence::{
    admit_primitive_construction_birth_consequence, primitive_construction_birth_digest,
};
use super::primitive_birth_scaffold_materialization::{
    materialize_primitive_construction_birth_scaffold_input,
    PrimitiveConstructionBirthScaffoldMaterializationInput,
    SpatialConstructionBirthScaffoldMaterializationError,
};

#[derive(Clone, Debug)]
pub(crate) struct AdmittedPrimitiveConstructionBirthAssessment {
    birth_input: PrimitiveConstructionBirthScaffoldInput,
    birth_digest: String,
    birth_mapping_digest: String,
    consequence_digest: String,
}

impl PartialEq for AdmittedPrimitiveConstructionBirthAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.family() == other.family()
            && self.birth_contract() == other.birth_contract()
            && self.scaffold_digest() == other.scaffold_digest()
            && self.topology_birth_class() == other.topology_birth_class()
            && self.supported_vertex_count() == other.supported_vertex_count()
            && self.supported_edge_count() == other.supported_edge_count()
            && self.supported_loop_count() == other.supported_loop_count()
            && self.supported_wire_count() == other.supported_wire_count()
            && self.supported_face_count() == other.supported_face_count()
            && self.supported_shell_count() == other.supported_shell_count()
            && self.supported_body_count() == other.supported_body_count()
            && self.birth_digest() == other.birth_digest()
            && self.realization_strategy() == other.realization_strategy()
            && self.attempted_realization_strategies() == other.attempted_realization_strategies()
            && self.stability_class() == other.stability_class()
            && self.feature_conditioning_class() == other.feature_conditioning_class()
            && self.conditioning_witness() == other.conditioning_witness()
            && self.placement_facts() == other.placement_facts()
            && self.support_normal_class() == other.support_normal_class()
            && self.normalization_disposition() == other.normalization_disposition()
            && self.realization_fact_digest() == other.realization_fact_digest()
            && self.realization_geometry_digest() == other.realization_geometry_digest()
            && self.birth_mapping_digest == other.birth_mapping_digest
            && self.consequence_digest == other.consequence_digest
    }
}

impl AdmittedPrimitiveConstructionBirthAssessment {
    fn new(
        birth_input: PrimitiveConstructionBirthScaffoldInput,
        birth_digest: String,
        birth_mapping_digest: String,
        consequence_digest: String,
    ) -> Self {
        Self {
            birth_input,
            birth_digest,
            birth_mapping_digest,
            consequence_digest,
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamilyKey {
        self.birth_input.family()
    }

    pub(crate) fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.birth_input.birth_contract()
    }

    pub(crate) fn scaffold_digest(&self) -> &str {
        self.birth_input.scaffold_digest()
    }

    pub(crate) fn topology_birth_class(&self) -> &str {
        self.birth_input.topology_birth_class()
    }

    pub(crate) fn supported_loop_count(&self) -> usize {
        self.birth_input.expected_loop_count()
    }

    pub(crate) fn supported_vertex_count(&self) -> usize {
        self.birth_input.expected_vertex_count()
    }

    pub(crate) fn supported_edge_count(&self) -> usize {
        self.birth_input.expected_edge_count()
    }

    pub(crate) fn supported_wire_count(&self) -> usize {
        self.birth_input.expected_wire_count()
    }

    pub(crate) fn supported_face_count(&self) -> usize {
        self.birth_input.expected_face_count()
    }

    pub(crate) fn supported_shell_count(&self) -> usize {
        self.birth_input.expected_shell_count()
    }

    pub(crate) fn supported_body_count(&self) -> usize {
        self.birth_input.expected_body_count()
    }

    pub(crate) fn birth_digest(&self) -> &str {
        &self.birth_digest
    }

    pub(crate) fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.birth_input.realization_strategy()
    }

    pub(crate) fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.birth_input.attempted_realization_strategies()
    }

    pub(crate) fn stability_class(&self) -> PrimitiveStabilityClass {
        self.birth_input.stability_class()
    }

    pub(crate) fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.birth_input.feature_conditioning_class()
    }

    pub(crate) fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        self.birth_input.conditioning_witness()
    }

    pub(crate) fn placement_facts(
        &self,
    ) -> super::primitive_birth_placement::PrimitiveConstructionBirthPlacementFacts {
        self.birth_input.placement_facts()
    }

    pub(crate) fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.birth_input.support_normal_class()
    }

    pub(crate) fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.birth_input.normalization_disposition()
    }

    pub(crate) fn realization_fact_digest(&self) -> &str {
        self.birth_input.realization_fact_digest()
    }

    pub(crate) fn realization_geometry_digest(&self) -> &str {
        self.birth_input.realization_geometry_digest()
    }

    pub(crate) fn consequence_digest(&self) -> &str {
        &self.consequence_digest
    }

    pub(crate) fn birth_mapping_digest(&self) -> String {
        self.birth_mapping_digest.clone()
    }
}

impl PrimitiveConstructionBirthScaffoldMaterializationInput {
    pub(crate) fn materialize_assessment(
        self,
    ) -> Result<AdmittedPrimitiveConstructionBirthAssessment, SpatialConstructionBirthAssessmentError>
    {
        let scaffold_input = materialize_primitive_construction_birth_scaffold_input(self)
            .map_err(SpatialConstructionBirthAssessmentError::Materialization)?;
        assess_primitive_construction_birth(scaffold_input)
            .map_err(SpatialConstructionBirthAssessmentError::InvalidBirth)
    }
}

#[derive(Debug)]
pub(crate) enum SpatialConstructionBirthAssessmentError {
    Materialization(SpatialConstructionBirthScaffoldMaterializationError),
    InvalidBirth(SpatialConstructionBirthError),
}

impl std::fmt::Display for SpatialConstructionBirthAssessmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Materialization(error) => write!(f, "{error}"),
            Self::InvalidBirth(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for SpatialConstructionBirthAssessmentError {}

pub(crate) fn assess_primitive_construction_birth(
    input: PrimitiveConstructionBirthScaffoldInput,
) -> Result<AdmittedPrimitiveConstructionBirthAssessment, SpatialConstructionBirthError> {
    let admitted = admit_primitive_construction_birth_consequence(&input)?;
    Ok(AdmittedPrimitiveConstructionBirthAssessment::new(
        input.clone(),
        primitive_construction_birth_digest(&input),
        admitted
            .rows()
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>()
            .join("|"),
        admitted.consequence_digest().to_string(),
    ))
}
