use crate::construction::admission::AdmittedPrimitiveConstructionIntent;
use crate::construction::digest::digest_owned_parts;
use crate::construction::execution::PreparedPrimitiveConstructionExecution;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionRequest};
use crate::construction::scaffold::PrimitiveConstructionScaffold;
use topology::facade::{
    build_topology_construction_fact_report, TopologyConstructionCertificationPlan,
    TopologyConstructionFactReport, TopologyConstructionInspectionSurface,
    TopologyConstructionLoweringPlan, TopologyConstructionMutationSurface,
};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationReport, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};
use worth_spatial::facade::{
    certify_primitive_construction_birth_completeness, SpatialConstructionBirthCompletenessReport,
    SpatialConstructionBirthError, SpatialConstructionBirthPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalPrimitiveConstructionArtifact {
    family: PrimitiveConstructionFamily,
    topology_birth_class: String,
    realization_report: PrimitiveRealizationReport,
    request_digest: String,
    intent_digest: String,
    scaffold_digest: String,
    birth_digest: String,
    birth_completeness_digest: String,
    topology_fact_digest: String,
    lowering_digest: String,
    execution_digest: String,
    certification_digest: String,
    mutation_surface: TopologyConstructionMutationSurface,
    inspection_surface: TopologyConstructionInspectionSurface,
    supported_vertex_count: usize,
    supported_edge_count: usize,
    supported_loop_count: usize,
    supported_wire_count: usize,
    supported_face_count: usize,
    supported_shell_count: usize,
    supported_body_count: usize,
    artifact_digest: String,
}

impl CanonicalPrimitiveConstructionArtifact {
    fn new(
        request: &PrimitiveConstructionRequest,
        intent: &AdmittedPrimitiveConstructionIntent,
        scaffold: &PrimitiveConstructionScaffold,
        birth_plan: &SpatialConstructionBirthPlan,
        birth_completeness: &SpatialConstructionBirthCompletenessReport,
        fact_report: &TopologyConstructionFactReport,
        lowering_plan: &TopologyConstructionLoweringPlan,
        execution: &PreparedPrimitiveConstructionExecution,
        certification: &TopologyConstructionCertificationPlan,
    ) -> Self {
        let parts = [
            request.request_digest().to_string(),
            intent.intent_digest().to_string(),
            scaffold.scaffold_digest().to_string(),
            birth_plan.birth_digest().to_string(),
            birth_completeness.completeness_digest().to_string(),
            fact_report.report_digest().to_string(),
            lowering_plan.lowering_digest().to_string(),
            execution.execution_digest().to_string(),
            certification.certification_digest().to_string(),
            scaffold.realization_report().report_digest().to_string(),
            lowering_plan.mutation_surface().as_str().to_string(),
            certification.inspection_surface().as_str().to_string(),
        ];
        Self {
            family: request.family(),
            topology_birth_class: birth_plan.topology_birth_class().to_string(),
            realization_report: scaffold.realization_report().clone(),
            request_digest: request.request_digest().to_string(),
            intent_digest: intent.intent_digest().to_string(),
            scaffold_digest: scaffold.scaffold_digest().to_string(),
            birth_digest: birth_plan.birth_digest().to_string(),
            birth_completeness_digest: birth_completeness.completeness_digest().to_string(),
            topology_fact_digest: fact_report.report_digest().to_string(),
            lowering_digest: lowering_plan.lowering_digest().to_string(),
            execution_digest: execution.execution_digest().to_string(),
            certification_digest: certification.certification_digest().to_string(),
            mutation_surface: lowering_plan.mutation_surface(),
            inspection_surface: certification.inspection_surface(),
            supported_vertex_count: birth_completeness.supported_vertex_count(),
            supported_edge_count: birth_completeness.supported_edge_count(),
            supported_loop_count: birth_completeness.supported_loop_count(),
            supported_wire_count: birth_completeness.supported_wire_count(),
            supported_face_count: birth_completeness.supported_face_count(),
            supported_shell_count: birth_completeness.supported_shell_count(),
            supported_body_count: birth_completeness.supported_body_count(),
            artifact_digest: digest_owned_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn realization_report(&self) -> &PrimitiveRealizationReport {
        &self.realization_report
    }

    pub fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.realization_report.strategy()
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.realization_report.attempted_strategies()
    }

    pub fn attempted_realization_strategy_count(&self) -> usize {
        self.realization_report.attempted_strategies().len()
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.realization_report.stability_class()
    }

    pub fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.realization_report()
            .conditioning_witness()
            .feature_conditioning_class()
    }

    pub fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.realization_report()
            .conditioning_witness()
            .support_normal_class()
    }

    pub fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.realization_report()
            .conditioning_witness()
            .normalization_disposition()
    }

    pub fn birth_truth_digest(&self) -> &str {
        &self.birth_digest
    }

    pub fn birth_completeness_digest(&self) -> &str {
        &self.birth_completeness_digest
    }

    pub fn topology_fact_digest(&self) -> &str {
        &self.topology_fact_digest
    }

    pub fn mutation_surface(&self) -> TopologyConstructionMutationSurface {
        self.mutation_surface
    }

    pub fn inspection_surface(&self) -> TopologyConstructionInspectionSurface {
        self.inspection_surface
    }

    pub fn supported_loop_count(&self) -> usize {
        self.supported_loop_count
    }

    pub fn supported_body_count(&self) -> usize {
        self.supported_body_count
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionArtifactError {
    SpatialBirth(SpatialConstructionBirthError),
}

impl std::fmt::Display for PrimitiveConstructionArtifactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpatialBirth(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionArtifactError {}

pub(crate) fn build_canonical_primitive_construction_artifact_with_completeness(
    request: &PrimitiveConstructionRequest,
    intent: &AdmittedPrimitiveConstructionIntent,
    scaffold: &PrimitiveConstructionScaffold,
    birth_plan: &SpatialConstructionBirthPlan,
    birth_completeness: &SpatialConstructionBirthCompletenessReport,
    fact_report: &TopologyConstructionFactReport,
    lowering_plan: &TopologyConstructionLoweringPlan,
    execution: &PreparedPrimitiveConstructionExecution,
    certification: &TopologyConstructionCertificationPlan,
) -> Result<CanonicalPrimitiveConstructionArtifact, PrimitiveConstructionArtifactError> {
    Ok(CanonicalPrimitiveConstructionArtifact::new(
        request,
        intent,
        scaffold,
        birth_plan,
        birth_completeness,
        fact_report,
        lowering_plan,
        execution,
        certification,
    ))
}

pub fn build_canonical_primitive_construction_artifact(
    request: &PrimitiveConstructionRequest,
    intent: &AdmittedPrimitiveConstructionIntent,
    scaffold: &PrimitiveConstructionScaffold,
    birth_plan: &SpatialConstructionBirthPlan,
    lowering_plan: &TopologyConstructionLoweringPlan,
    execution: &PreparedPrimitiveConstructionExecution,
    certification: &TopologyConstructionCertificationPlan,
) -> Result<CanonicalPrimitiveConstructionArtifact, PrimitiveConstructionArtifactError> {
    let birth_completeness =
        certify_primitive_construction_birth_completeness(&scaffold.birth_input(), birth_plan)
            .map_err(PrimitiveConstructionArtifactError::SpatialBirth)?;
    let fact_report = build_topology_construction_fact_report(lowering_plan, certification);
    build_canonical_primitive_construction_artifact_with_completeness(
        request,
        intent,
        scaffold,
        birth_plan,
        &birth_completeness,
        &fact_report,
        lowering_plan,
        execution,
        certification,
    )
}

#[cfg(test)]
mod tests {
    use super::build_canonical_primitive_construction_artifact;
    use crate::construction::{
        lower_scaffold_to_topology, PreparedPrimitiveConstructionExecution,
        PrimitiveConstructionFamily, PrimitiveConstructionIntent, ShellWithHoleSpec,
    };
    use topology::facade::TopologyConstructionInspectionSurface;

    #[test]
    fn canonical_artifact_binds_shell_birth_and_topology_truth() {
        let intent = PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
            outer_loop_edge_count: 6,
            hole_loop_edge_counts: vec![3, 4],
        });
        let request = intent.clone().into_request();
        let admitted = request.clone().admit().expect("admitted intent");
        let scaffold = admitted.build_scaffold().expect("scaffold");
        let (birth_plan, lowering_plan) = lower_scaffold_to_topology(&scaffold).expect("lowering");
        let execution = PreparedPrimitiveConstructionExecution::from_phase_chain(
            &request,
            &admitted,
            &scaffold,
            &birth_plan,
            &lowering_plan,
        )
        .expect("execution");
        let certification = execution.plan_topology_certification();
        let artifact = build_canonical_primitive_construction_artifact(
            &request,
            &admitted,
            &scaffold,
            &birth_plan,
            &lowering_plan,
            &execution,
            &certification,
        )
        .expect("artifact");

        assert_eq!(
            artifact.family(),
            PrimitiveConstructionFamily::ShellWithHole
        );
        assert_eq!(
            artifact.topology_birth_class(),
            "planar_shell_with_hole_body"
        );
        assert_eq!(artifact.birth_truth_digest(), birth_plan.birth_digest());
        assert_eq!(artifact.supported_loop_count(), 3);
        assert_eq!(artifact.supported_body_count(), 1);
        assert!(!artifact.topology_fact_digest().is_empty());
        assert_eq!(
            artifact.inspection_surface(),
            TopologyConstructionInspectionSurface::InspectReceipt
        );
        assert!(!artifact.birth_completeness_digest().is_empty());
        assert!(!artifact.artifact_digest().is_empty());
    }
}
