use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use worth_geom::facade::{
    build_direct_realization_report, Plane, PrimitiveFeatureConditioningClass,
    PrimitiveNormalizationDisposition, PrimitiveRealizationReport, PrimitiveRealizationStrategy,
    PrimitiveStabilityClass, PrimitiveSupportNormalClass,
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
        let realization_report =
            build_direct_realization_report(family.as_str(), &vertex_positions, &support_planes);
        Self::new_with_realization(
            family,
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
            input.realization_report().report_digest().to_string(),
        ];
        Self {
            family: input.family(),
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
            realization_report_digest: input.realization_report().report_digest().to_string(),
            birth_digest: digest_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionBirthFamily {
        self.family
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
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
#[path = "primitive_birth_tests.rs"]
mod tests;
