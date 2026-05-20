use crate::performance::primitives::{
    FoundationalPerformanceAccessPatternDefinition, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceAllocationDefinition, FoundationalPerformanceAllocationPosture,
    FoundationalPerformanceLayoutIntent, FoundationalPerformanceLayoutIntentDefinition,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalLayoutIntentClaim {
    layout_intent: FoundationalPerformanceLayoutIntent,
    access_pattern: FoundationalPerformanceAccessPatternPosture,
    allocation_posture: FoundationalPerformanceAllocationPosture,
}

impl FoundationalLayoutIntentClaim {
    pub const fn new(
        layout_intent: FoundationalPerformanceLayoutIntent,
        access_pattern: FoundationalPerformanceAccessPatternPosture,
        allocation_posture: FoundationalPerformanceAllocationPosture,
    ) -> Self {
        Self {
            layout_intent,
            access_pattern,
            allocation_posture,
        }
    }

    pub const fn layout_intent(&self) -> FoundationalPerformanceLayoutIntent {
        self.layout_intent
    }

    pub const fn access_pattern(&self) -> FoundationalPerformanceAccessPatternPosture {
        self.access_pattern
    }

    pub const fn allocation_posture(&self) -> FoundationalPerformanceAllocationPosture {
        self.allocation_posture
    }

    pub fn layout_definition(&self) -> FoundationalPerformanceLayoutIntentDefinition {
        crate::performance::foundational_performance_layout_intent_definitions()
            .into_iter()
            .find(|definition| definition.family() == self.layout_intent)
            .expect("all layout intents have foundational definitions")
    }

    pub fn access_definition(&self) -> FoundationalPerformanceAccessPatternDefinition {
        crate::performance::foundational_performance_access_pattern_definitions()
            .into_iter()
            .find(|definition| definition.family() == self.access_pattern)
            .expect("all access postures have foundational definitions")
    }

    pub fn allocation_definition(&self) -> FoundationalPerformanceAllocationDefinition {
        crate::performance::foundational_performance_allocation_definitions()
            .into_iter()
            .find(|definition| definition.family() == self.allocation_posture)
            .expect("all allocation postures have foundational definitions")
    }
}
