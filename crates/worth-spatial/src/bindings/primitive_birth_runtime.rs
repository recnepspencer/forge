use super::primitive_birth_placement::PrimitiveConstructionBirthPlacementFacts;
use worth_geom::facade::{
    build_direct_realization_report, Plane, PrimitiveConditioningWitness,
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationReport, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionBirthRealizationFacts {
    realization_report: PrimitiveRealizationReport,
    placement_facts: Option<PrimitiveConstructionBirthPlacementFacts>,
}

impl PrimitiveConstructionBirthRealizationFacts {
    pub fn from_realization_report(realization_report: PrimitiveRealizationReport) -> Self {
        Self {
            realization_report,
            placement_facts: None,
        }
    }

    pub fn from_direct_planar_support(
        label: &'static str,
        vertex_positions: &[[f64; 3]],
        support_planes: &[Plane],
    ) -> Self {
        Self::from_realization_report(build_direct_realization_report(
            label,
            vertex_positions,
            support_planes,
        ))
    }

    pub fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        self.realization_report.conditioning_witness()
    }

    pub fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.realization_report.strategy()
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.realization_report.attempted_strategies()
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.realization_report.stability_class()
    }

    pub(crate) fn with_placement_facts(
        mut self,
        placement_facts: PrimitiveConstructionBirthPlacementFacts,
    ) -> Self {
        self.placement_facts = Some(placement_facts);
        self
    }

    pub(crate) fn placement_facts(&self) -> Option<PrimitiveConstructionBirthPlacementFacts> {
        self.placement_facts
    }

    pub fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.conditioning_witness().feature_conditioning_class()
    }

    pub fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.conditioning_witness().support_normal_class()
    }

    pub fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.conditioning_witness().normalization_disposition()
    }

    pub fn realization_fact_digest(&self) -> &str {
        self.realization_report.report_digest()
    }

    pub fn realization_geometry_digest(&self) -> &str {
        self.realization_report.geometry_digest()
    }
}
