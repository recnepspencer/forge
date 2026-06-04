use worth_geom::facade::{
    build_direct_realization_report, Plane, PrimitiveFeatureConditioningClass,
    PrimitiveNormalizationDisposition, PrimitiveRealizationReport, PrimitiveRealizationStrategy,
    PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};
use worth_primitives::{
    truth_digest_parts, PrimitiveConstructionBirthSynopsisContract,
    PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor, TruthDigestScope,
};

use crate::bindings::primitive_birth_validation::validate_primitive_construction_birth_input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionBirthFamily {
    SimplexSolid,
    Orthotope,
    RegularPrism,
    RegularPyramid,
    WireBody,
    ShellWithHole,
}

impl PrimitiveConstructionBirthFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SimplexSolid => "simplex_solid",
            Self::Orthotope => "orthotope",
            Self::RegularPrism => "regular_prism",
            Self::RegularPyramid => "regular_pyramid",
            Self::WireBody => "wire_body",
            Self::ShellWithHole => "shell_with_hole",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrimitiveConstructionBirthScaffoldInput {
    family: PrimitiveConstructionBirthFamily,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    topology_birth_class: &'static str,
    scaffold_digest: String,
    support_planes: Vec<Plane>,
    realization_report: PrimitiveRealizationReport,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family: PrimitiveConstructionBirthFamily,
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
        let realization_report =
            build_direct_realization_report(family.as_str(), &vertex_positions, &support_planes);
        Self::new_with_realization(
            family,
            birth_contract,
            topology_birth_class,
            scaffold_digest,
            support_planes,
            realization_report,
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
    pub fn new_with_realization_and_contract(
        family: PrimitiveConstructionBirthFamily,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        topology_birth_class: &'static str,
        scaffold_digest: String,
        support_planes: Vec<Plane>,
        realization_report: PrimitiveRealizationReport,
        vertex_positions: Vec<[f64; 3]>,
        expected_vertex_count: usize,
        expected_edge_count: usize,
        expected_loop_count: usize,
        expected_wire_count: usize,
        expected_face_count: usize,
        expected_shell_count: usize,
        expected_body_count: usize,
    ) -> Self {
        Self::new_with_realization(
            family,
            birth_contract,
            topology_birth_class,
            scaffold_digest,
            support_planes,
            realization_report,
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
    pub fn new_with_realization(
        family: PrimitiveConstructionBirthFamily,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        topology_birth_class: &'static str,
        scaffold_digest: String,
        support_planes: Vec<Plane>,
        realization_report: PrimitiveRealizationReport,
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
            realization_report,
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

    pub fn family(&self) -> PrimitiveConstructionBirthFamily {
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

    pub fn realization_report(&self) -> &PrimitiveRealizationReport {
        &self.realization_report
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConstructionBirthPlan {
    family: PrimitiveConstructionBirthFamily,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    scaffold_digest: String,
    topology_birth_class: &'static str,
    supported_vertex_count: usize,
    supported_edge_count: usize,
    supported_loop_count: usize,
    supported_wire_count: usize,
    supported_face_count: usize,
    supported_shell_count: usize,
    supported_body_count: usize,
    realization_strategy: PrimitiveRealizationStrategy,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: PrimitiveStabilityClass,
    feature_conditioning_class: PrimitiveFeatureConditioningClass,
    support_normal_class: PrimitiveSupportNormalClass,
    normalization_disposition: PrimitiveNormalizationDisposition,
    realization_report_digest: String,
    realization_geometry_digest: String,
    birth_digest: String,
}

impl SpatialConstructionBirthPlan {
    fn new(input: &PrimitiveConstructionBirthScaffoldInput) -> Self {
        let parts = [
            input.family().as_str().to_string(),
            input.scaffold_digest().to_string(),
            input.topology_birth_class().to_string(),
            input.expected_vertex_count().to_string(),
            input.expected_edge_count().to_string(),
            input.expected_loop_count().to_string(),
            input.expected_wire_count().to_string(),
            input.expected_face_count().to_string(),
            input.expected_shell_count().to_string(),
            input.expected_body_count().to_string(),
            input.realization_report().strategy().as_str().to_string(),
            input
                .realization_report()
                .attempted_strategies()
                .iter()
                .map(|strategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            input
                .realization_report()
                .stability_class()
                .as_str()
                .to_string(),
            input
                .realization_report()
                .conditioning_witness()
                .feature_conditioning_class()
                .as_str()
                .to_string(),
            input
                .realization_report()
                .conditioning_witness()
                .support_normal_class()
                .as_str()
                .to_string(),
            input
                .realization_report()
                .conditioning_witness()
                .normalization_disposition()
                .as_str()
                .to_string(),
            input.realization_report().geometry_digest().to_string(),
            input.realization_report().report_digest().to_string(),
        ];
        Self {
            family: input.family(),
            birth_contract: input.birth_contract(),
            scaffold_digest: input.scaffold_digest().to_string(),
            topology_birth_class: input.topology_birth_class(),
            supported_vertex_count: input.expected_vertex_count(),
            supported_edge_count: input.expected_edge_count(),
            supported_loop_count: input.expected_loop_count(),
            supported_wire_count: input.expected_wire_count(),
            supported_face_count: input.expected_face_count(),
            supported_shell_count: input.expected_shell_count(),
            supported_body_count: input.expected_body_count(),
            realization_strategy: input.realization_report().strategy(),
            attempted_realization_strategies: input
                .realization_report()
                .attempted_strategies()
                .to_vec(),
            stability_class: input.realization_report().stability_class(),
            feature_conditioning_class: input
                .realization_report()
                .conditioning_witness()
                .feature_conditioning_class(),
            support_normal_class: input
                .realization_report()
                .conditioning_witness()
                .support_normal_class(),
            normalization_disposition: input
                .realization_report()
                .conditioning_witness()
                .normalization_disposition(),
            realization_geometry_digest: input.realization_report().geometry_digest().to_string(),
            realization_report_digest: input.realization_report().report_digest().to_string(),
            birth_digest: digest_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionBirthFamily {
        self.family
    }

    pub fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.birth_contract
    }

    pub fn scaffold_digest(&self) -> &str {
        &self.scaffold_digest
    }

    pub fn topology_birth_class(&self) -> &str {
        self.topology_birth_class
    }

    pub fn supported_vertex_count(&self) -> usize {
        self.supported_vertex_count
    }

    pub fn supported_edge_count(&self) -> usize {
        self.supported_edge_count
    }

    pub fn supported_loop_count(&self) -> usize {
        self.supported_loop_count
    }

    pub fn supported_wire_count(&self) -> usize {
        self.supported_wire_count
    }

    pub fn supported_face_count(&self) -> usize {
        self.supported_face_count
    }

    pub fn supported_shell_count(&self) -> usize {
        self.supported_shell_count
    }

    pub fn supported_body_count(&self) -> usize {
        self.supported_body_count
    }

    pub fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.realization_strategy
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.feature_conditioning_class
    }

    pub fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.support_normal_class
    }

    pub fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.normalization_disposition
    }

    pub fn realization_report_digest(&self) -> &str {
        &self.realization_report_digest
    }

    pub fn realization_geometry_digest(&self) -> &str {
        &self.realization_geometry_digest
    }

    pub fn birth_digest(&self) -> &str {
        &self.birth_digest
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

pub fn plan_primitive_construction_birth(
    input: PrimitiveConstructionBirthScaffoldInput,
) -> Result<SpatialConstructionBirthPlan, SpatialConstructionBirthError> {
    validate_primitive_construction_birth_input(&input)?;
    Ok(SpatialConstructionBirthPlan::new(&input))
}

pub(super) fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

fn derive_birth_contract(
    family: PrimitiveConstructionBirthFamily,
    vertex_count: usize,
    edge_count: usize,
    loop_count: usize,
    _wire_count: usize,
    face_count: usize,
    _shell_count: usize,
    _body_count: usize,
) -> PrimitiveConstructionBirthSynopsisContract {
    let descriptor = match family {
        PrimitiveConstructionBirthFamily::SimplexSolid => PrimitiveWitnessDescriptor::SimplexSolid,
        PrimitiveConstructionBirthFamily::Orthotope => PrimitiveWitnessDescriptor::Orthotope,
        PrimitiveConstructionBirthFamily::RegularPrism => PrimitiveWitnessDescriptor::RegularPrism {
            side_count: (face_count - 2) as u32,
        },
        PrimitiveConstructionBirthFamily::RegularPyramid => {
            PrimitiveWitnessDescriptor::RegularPyramid {
                side_count: (vertex_count - 1) as u32,
            }
        }
        PrimitiveConstructionBirthFamily::WireBody => PrimitiveWitnessDescriptor::WireBody {
            edge_count: edge_count as u32,
        },
        PrimitiveConstructionBirthFamily::ShellWithHole => PrimitiveWitnessDescriptor::ShellWithHole {
            outer_loop_edge_count: (edge_count - (loop_count.saturating_sub(1) * 3)) as u32,
            hole_loop_edge_counts: vec![3; loop_count.saturating_sub(1)],
        },
    };
    PrimitiveConstructionFamilyContractRegistry::contract_for(&descriptor)
}

#[cfg(test)]
#[path = "primitive_birth_tests.rs"]
mod tests;
