use crate::{
    LaneFamilyExtension, ObservedPhysicalTrace, PhysicalProofOracleVerdict,
    PhysicalScenarioDefinition, PhysicalScenarioExecution, PhysicalScenarioPlan,
    PhysicalScenarioPlanDenial, PhysicalStoryTranscript, RoadmapLaneFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioQualityHarness {
    lane_family_extensions: Vec<LaneFamilyExtension>,
}

impl PhysicalScenarioQualityHarness {
    pub fn cross_cutting_scenario() -> Self {
        Self {
            lane_family_extensions: Vec::new(),
        }
    }

    pub fn with_lane_family_extension(
        mut self,
        extension: LaneFamilyExtension,
    ) -> Result<Self, PhysicalScenarioHarnessDenial> {
        if extension.family() == RoadmapLaneFamily::PhysicalSubstrate {
            return Err(PhysicalScenarioHarnessDenial::PhysicalSubstrateIsBuiltIn);
        }
        if !RoadmapLaneFamily::reserved_follow_on().contains(&extension.family()) {
            return Err(PhysicalScenarioHarnessDenial::UnreservedLaneFamily);
        }
        self.lane_family_extensions.push(extension);
        Ok(self)
    }

    pub fn lane_family_extensions(&self) -> &[LaneFamilyExtension] {
        &self.lane_family_extensions
    }

    pub fn lower(
        &self,
        definition: PhysicalScenarioDefinition,
    ) -> Result<PhysicalScenarioPlan, PhysicalScenarioPlanDenial> {
        let extensions = self.extensions_for_definition(&definition);
        PhysicalScenarioPlan::from_definition(definition, &extensions)
    }

    pub fn execute(&self, plan: PhysicalScenarioPlan) -> PhysicalScenarioExecution {
        PhysicalScenarioExecution::from_plan(plan)
    }

    pub fn observe(&self, execution: PhysicalScenarioExecution) -> ObservedPhysicalTrace {
        ObservedPhysicalTrace::from_execution(execution)
    }

    pub fn judge(&self, observed: ObservedPhysicalTrace) -> PhysicalProofOracleVerdict {
        PhysicalProofOracleVerdict::from_trace(observed)
    }

    pub fn transcribe(&self, verdict: PhysicalProofOracleVerdict) -> PhysicalStoryTranscript {
        PhysicalStoryTranscript::from_verdict(verdict)
    }
}

impl PhysicalScenarioQualityHarness {
    fn extensions_for_definition(
        &self,
        definition: &PhysicalScenarioDefinition,
    ) -> Vec<LaneFamilyExtension> {
        let family = definition.lane().family();
        self.lane_family_extensions
            .iter()
            .copied()
            .filter(|extension| extension.family() == family)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalScenarioHarnessDenial {
    PhysicalSubstrateIsBuiltIn,
    UnreservedLaneFamily,
}
