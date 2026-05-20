mod families;
mod postures;
mod work;

pub use families::{
    foundational_performance_boundary_definitions,
    foundational_performance_evidence_strength_definitions,
    foundational_performance_layout_intent_definitions, FoundationalPerformanceBoundary,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceLayoutIntent,
};
pub use postures::{
    foundational_performance_access_pattern_definitions,
    foundational_performance_allocation_definitions,
    foundational_performance_breadth_locality_definitions,
    foundational_performance_execution_temperature_definitions,
    foundational_performance_fallback_debt_definitions,
    foundational_performance_freshness_retention_definitions,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceAllocationPosture,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
};
pub use work::{foundational_performance_work_class_definitions, FoundationalPerformanceWorkClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformancePrimitiveDefinition<Family> {
    family: Family,
    name: &'static str,
    intended_use: &'static str,
    must_not_mean: &'static str,
}

impl<Family: Copy> FoundationalPerformancePrimitiveDefinition<Family> {
    pub const fn new(
        family: Family,
        name: &'static str,
        intended_use: &'static str,
        must_not_mean: &'static str,
    ) -> Self {
        Self {
            family,
            name,
            intended_use,
            must_not_mean,
        }
    }

    pub const fn family(&self) -> Family {
        self.family
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn intended_use(&self) -> &'static str {
        self.intended_use
    }

    pub const fn must_not_mean(&self) -> &'static str {
        self.must_not_mean
    }
}

pub type FoundationalPerformanceLayoutIntentDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceLayoutIntent>;
pub type FoundationalPerformanceBoundaryDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceBoundary>;
pub type FoundationalPerformanceEvidenceStrengthDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceEvidenceStrength>;
pub type FoundationalPerformanceBreadthLocalityDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceBreadthLocalityPosture>;
pub type FoundationalPerformanceAllocationDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceAllocationPosture>;
pub type FoundationalPerformanceAccessPatternDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceAccessPatternPosture>;
pub type FoundationalPerformanceExecutionTemperatureDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceExecutionTemperature>;
pub type FoundationalPerformanceFreshnessRetentionDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceFreshnessRetentionPosture>;
pub type FoundationalPerformanceFallbackDebtDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceFallbackDebtPosture>;
pub type FoundationalPerformanceWorkClassDefinition =
    FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceWorkClass>;
