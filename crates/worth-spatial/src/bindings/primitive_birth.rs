use worth_geom::facade::{
    Plane, PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};
use worth_primitives::{
    truth_digest_parts, PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyKey,
    TruthDigestScope,
};
#[cfg(test)]
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};

use super::primitive_birth_placement::PrimitiveConstructionBirthPlacementFacts;
use super::primitive_birth_runtime::PrimitiveConstructionBirthRealizationFacts;

#[derive(Clone, Debug)]
pub struct PrimitiveConstructionBirthScaffoldInput {
    family: PrimitiveConstructionFamilyKey,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    topology_birth_class: &'static str,
    scaffold_digest: String,
    support_planes: Vec<Plane>,
    realization: PrimitiveConstructionBirthRealizationFacts,
    vertex_positions: Vec<[f64; 3]>,
    expected_vertex_count: usize,
    expected_edge_count: usize,
    expected_loop_count: usize,
    expected_wire_count: usize,
    expected_face_count: usize,
    expected_shell_count: usize,
    expected_body_count: usize,
}

impl PrimitiveConstructionBirthScaffoldInput {
    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family: PrimitiveConstructionFamilyKey,
        topology_birth_class: &'static str,
        scaffold_digest: String,
        support_planes: Vec<Plane>,
        vertex_positions: Vec<[f64; 3]>,
        expected_vertex_count: usize,
        expected_edge_count: usize,
        expected_loop_count: usize,
        expected_wire_count: usize,
        expected_face_count: usize,
        expected_shell_count: usize,
        expected_body_count: usize,
    ) -> Self {
        let birth_contract = derive_birth_contract(
            family,
            expected_vertex_count,
            expected_edge_count,
            expected_loop_count,
            expected_wire_count,
            expected_face_count,
            expected_shell_count,
            expected_body_count,
        );
        let realization = PrimitiveConstructionBirthRealizationFacts::from_direct_planar_support(
            family.as_str(),
            &vertex_positions,
            &support_planes,
        );
        Self::new_with_realization_facts(
            family,
            birth_contract,
            topology_birth_class,
            scaffold_digest,
            support_planes,
            realization,
            vertex_positions,
            expected_vertex_count,
            expected_edge_count,
            expected_loop_count,
            expected_wire_count,
            expected_face_count,
            expected_shell_count,
            expected_body_count,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_realization_facts_and_contract(
        family: PrimitiveConstructionFamilyKey,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        topology_birth_class: &'static str,
        scaffold_digest: String,
        support_planes: Vec<Plane>,
        realization: PrimitiveConstructionBirthRealizationFacts,
        vertex_positions: Vec<[f64; 3]>,
        expected_vertex_count: usize,
        expected_edge_count: usize,
        expected_loop_count: usize,
        expected_wire_count: usize,
        expected_face_count: usize,
        expected_shell_count: usize,
        expected_body_count: usize,
    ) -> Self {
        Self {
            family,
            birth_contract,
            topology_birth_class,
            scaffold_digest,
            support_planes,
            realization,
            vertex_positions,
            expected_vertex_count,
            expected_edge_count,
            expected_loop_count,
            expected_wire_count,
            expected_face_count,
            expected_shell_count,
            expected_body_count,
        }
    }

    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_realization_facts(
        family: PrimitiveConstructionFamilyKey,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        topology_birth_class: &'static str,
        scaffold_digest: String,
        support_planes: Vec<Plane>,
        realization: PrimitiveConstructionBirthRealizationFacts,
        vertex_positions: Vec<[f64; 3]>,
        expected_vertex_count: usize,
        expected_edge_count: usize,
        expected_loop_count: usize,
        expected_wire_count: usize,
        expected_face_count: usize,
        expected_shell_count: usize,
        expected_body_count: usize,
    ) -> Self {
        Self {
            family,
            birth_contract,
            topology_birth_class,
            scaffold_digest,
            support_planes,
            realization,
            vertex_positions,
            expected_vertex_count,
            expected_edge_count,
            expected_loop_count,
            expected_wire_count,
            expected_face_count,
            expected_shell_count,
            expected_body_count,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamilyKey {
        self.family
    }

    pub fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.birth_contract
    }

    pub fn scaffold_digest(&self) -> &str {
        &self.scaffold_digest
    }

    pub fn topology_birth_class(&self) -> &'static str {
        self.topology_birth_class
    }

    pub fn support_planes(&self) -> &[Plane] {
        &self.support_planes
    }

    #[cfg(test)]
    pub fn conditioning_witness(&self) -> &worth_geom::facade::PrimitiveConditioningWitness {
        self.realization.conditioning_witness()
    }

    pub fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.realization.realization_strategy()
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.realization.attempted_realization_strategies()
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.realization.stability_class()
    }

    pub fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.realization.feature_conditioning_class()
    }

    pub fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.realization.support_normal_class()
    }

    pub fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.realization.normalization_disposition()
    }

    pub fn realization_fact_digest(&self) -> &str {
        self.realization.realization_fact_digest()
    }

    pub fn realization_geometry_digest(&self) -> &str {
        self.realization.realization_geometry_digest()
    }

    pub(crate) fn placement_facts(&self) -> PrimitiveConstructionBirthPlacementFacts {
        self.realization
            .placement_facts()
            .expect("birth scaffold input should retain placement facts after materialization")
    }

    pub fn vertex_positions(&self) -> &[[f64; 3]] {
        &self.vertex_positions
    }

    pub fn expected_vertex_count(&self) -> usize {
        self.expected_vertex_count
    }

    pub fn expected_edge_count(&self) -> usize {
        self.expected_edge_count
    }

    pub fn expected_loop_count(&self) -> usize {
        self.expected_loop_count
    }

    pub fn expected_wire_count(&self) -> usize {
        self.expected_wire_count
    }

    pub fn expected_face_count(&self) -> usize {
        self.expected_face_count
    }

    pub fn expected_shell_count(&self) -> usize {
        self.expected_shell_count
    }

    pub fn expected_body_count(&self) -> usize {
        self.expected_body_count
    }
}

#[derive(Debug)]
pub enum SpatialConstructionBirthError {
    InvalidPrimitiveBirthScaffold(&'static str),
}

impl std::fmt::Display for SpatialConstructionBirthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPrimitiveBirthScaffold(reason) => {
                write!(f, "invalid primitive construction birth scaffold: {reason}")
            }
        }
    }
}

impl std::error::Error for SpatialConstructionBirthError {}

pub(super) fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

#[cfg(test)]
fn derive_birth_contract(
    family: PrimitiveConstructionFamilyKey,
    vertex_count: usize,
    edge_count: usize,
    loop_count: usize,
    _wire_count: usize,
    face_count: usize,
    _shell_count: usize,
    _body_count: usize,
) -> PrimitiveConstructionBirthSynopsisContract {
    let descriptor = match family {
        PrimitiveConstructionFamilyKey::SimplexSolid => PrimitiveWitnessDescriptor::SimplexSolid,
        PrimitiveConstructionFamilyKey::Orthotope => PrimitiveWitnessDescriptor::Orthotope,
        PrimitiveConstructionFamilyKey::RegularPrism => PrimitiveWitnessDescriptor::RegularPrism {
            side_count: (face_count - 2) as u32,
        },
        PrimitiveConstructionFamilyKey::RegularPyramid => {
            PrimitiveWitnessDescriptor::RegularPyramid {
                side_count: (vertex_count - 1) as u32,
            }
        }
        PrimitiveConstructionFamilyKey::WireBody => PrimitiveWitnessDescriptor::WireBody {
            edge_count: edge_count as u32,
        },
        PrimitiveConstructionFamilyKey::ShellWithHole => {
            PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: (edge_count - (loop_count.saturating_sub(1) * 3)) as u32,
                hole_loop_edge_counts: vec![3; loop_count.saturating_sub(1)],
            }
        }
    };
    PrimitiveConstructionFamilyContractRegistry::contract_for(&descriptor)
}

#[cfg(test)]
#[path = "primitive_birth_tests.rs"]
mod tests;
