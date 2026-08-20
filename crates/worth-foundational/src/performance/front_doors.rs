use crate::performance::claims::FoundationalPerformanceClaimAuthoringFrontDoor;
use crate::performance::layouts::{
    FoundationalLayoutAnnotatedClaim, FoundationalLayoutAnnotatedClaimConstructionDenial,
    FoundationalLayoutIntentClaim,
};
use crate::performance::legality::{
    evaluate_performance_primitive_legality, FoundationalPerformancePrimitiveLegalityDenial,
};
use crate::performance::policy::FoundationalPolicyAdmissionReceiptBuilder;
use crate::performance::primitives::{
    foundational_performance_access_pattern_definitions,
    foundational_performance_allocation_definitions, foundational_performance_boundary_definitions,
    foundational_performance_breadth_locality_definitions,
    foundational_performance_evidence_strength_definitions,
    foundational_performance_execution_temperature_definitions,
    foundational_performance_fallback_debt_definitions,
    foundational_performance_freshness_retention_definitions,
    foundational_performance_layout_intent_definitions,
    foundational_performance_work_class_definitions, FoundationalPerformanceBoundary,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceLayoutIntent, FoundationalPerformanceWorkClass,
};
use crate::performance::{
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceAllocationPosture,
    FoundationalPerformanceBudgetDefinition, FoundationalPerformanceClaimSurface,
    FoundationalPolicyAdmissionPerformanceClaim,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct FoundationalPerformanceFrontDoor;

pub fn performance() -> FoundationalPerformanceFrontDoor {
    FoundationalPerformanceFrontDoor
}

impl FoundationalPerformanceFrontDoor {
    pub fn claim(&self) -> FoundationalPerformanceClaimAuthoringFrontDoor {
        FoundationalPerformanceClaimAuthoringFrontDoor
    }

    pub fn policy_admission_receipt(
        &self,
        claim: FoundationalPolicyAdmissionPerformanceClaim,
    ) -> FoundationalPolicyAdmissionReceiptBuilder {
        crate::performance::policy_admission_receipt(claim)
    }

    pub const fn define_layout_intent(
        &self,
        layout_intent: FoundationalPerformanceLayoutIntent,
        access_pattern: FoundationalPerformanceAccessPatternPosture,
        allocation_posture: FoundationalPerformanceAllocationPosture,
    ) -> FoundationalLayoutIntentClaim {
        FoundationalLayoutIntentClaim::new(layout_intent, access_pattern, allocation_posture)
    }

    pub fn attach_layout_intent<Claim>(
        &self,
        claim: Claim,
        layout_intent_claim: FoundationalLayoutIntentClaim,
    ) -> Result<
        FoundationalLayoutAnnotatedClaim<Claim>,
        FoundationalLayoutAnnotatedClaimConstructionDenial,
    >
    where
        Claim: FoundationalPerformanceClaimSurface,
    {
        FoundationalLayoutAnnotatedClaim::new(claim, layout_intent_claim)
    }

    pub fn layout_intent_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceLayoutIntentDefinition; 6] {
        foundational_performance_layout_intent_definitions()
    }

    pub fn boundary_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceBoundaryDefinition; 10] {
        foundational_performance_boundary_definitions()
    }

    pub fn evidence_strength_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceEvidenceStrengthDefinition; 5] {
        foundational_performance_evidence_strength_definitions()
    }

    pub fn breadth_locality_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceBreadthLocalityDefinition; 9] {
        foundational_performance_breadth_locality_definitions()
    }

    pub fn allocation_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceAllocationDefinition; 6] {
        foundational_performance_allocation_definitions()
    }

    pub fn access_pattern_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceAccessPatternDefinition; 6] {
        foundational_performance_access_pattern_definitions()
    }

    pub fn execution_temperature_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceExecutionTemperatureDefinition; 5] {
        foundational_performance_execution_temperature_definitions()
    }

    pub fn freshness_retention_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceFreshnessRetentionDefinition; 6] {
        foundational_performance_freshness_retention_definitions()
    }

    pub fn fallback_debt_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceFallbackDebtDefinition; 6] {
        foundational_performance_fallback_debt_definitions()
    }

    pub fn budget_definitions(&self) -> [FoundationalPerformanceBudgetDefinition; 4] {
        crate::performance::foundational_performance_budget_definitions()
    }

    pub fn work_class_definitions(
        &self,
    ) -> [crate::performance::FoundationalPerformanceWorkClassDefinition; 13] {
        foundational_performance_work_class_definitions()
    }

    pub fn evaluate_primitive_legality(
        &self,
        boundary: FoundationalPerformanceBoundary,
        evidence_strength: FoundationalPerformanceEvidenceStrength,
        execution_temperature: FoundationalPerformanceExecutionTemperature,
        freshness: FoundationalPerformanceFreshnessRetentionPosture,
        fallback: FoundationalPerformanceFallbackDebtPosture,
        included_work: &[FoundationalPerformanceWorkClass],
        excluded_work: &[FoundationalPerformanceWorkClass],
    ) -> Result<(), FoundationalPerformancePrimitiveLegalityDenial> {
        evaluate_performance_primitive_legality(
            boundary,
            evidence_strength,
            execution_temperature,
            freshness,
            fallback,
            included_work,
            excluded_work,
        )
    }
}
