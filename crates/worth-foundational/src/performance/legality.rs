use crate::performance::primitives::{
    FoundationalPerformanceBoundary, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalPerformancePrimitiveLegalityDenial {
    SupportDerivedClaimsCannotUseHotPathTemperature,
    CounterBackedExecutionClaimsCannotUseSupportOnlyTemperature,
    DebtOrDeferredEvidenceCannotClaimVerifiedPosture,
    HotPathClaimsCannotUseReplayOrStaleFreshness,
    BoundaryMaterializationCannotClaimHotPathTemperature,
    HotPathIncludedWorkMustExcludeReplaySupportAndForensics,
    HotPathMustDiscloseExcludedColdSupportWork,
}

pub fn evaluate_performance_primitive_legality(
    boundary: FoundationalPerformanceBoundary,
    evidence_strength: FoundationalPerformanceEvidenceStrength,
    execution_temperature: FoundationalPerformanceExecutionTemperature,
    freshness: FoundationalPerformanceFreshnessRetentionPosture,
    fallback: FoundationalPerformanceFallbackDebtPosture,
    included_work: &[FoundationalPerformanceWorkClass],
    excluded_work: &[FoundationalPerformanceWorkClass],
) -> Result<(), FoundationalPerformancePrimitiveLegalityDenial> {
    if matches!(
        evidence_strength,
        FoundationalPerformanceEvidenceStrength::SupportDerivedPerformanceClaim
    ) && matches!(
        execution_temperature,
        FoundationalPerformanceExecutionTemperature::HotPath
    ) {
        return Err(
            FoundationalPerformancePrimitiveLegalityDenial::SupportDerivedClaimsCannotUseHotPathTemperature,
        );
    }

    if matches!(
        evidence_strength,
        FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt
    ) && matches!(
        execution_temperature,
        FoundationalPerformanceExecutionTemperature::SupportOnly
    ) {
        return Err(
            FoundationalPerformancePrimitiveLegalityDenial::CounterBackedExecutionClaimsCannotUseSupportOnlyTemperature,
        );
    }

    if matches!(
        evidence_strength,
        FoundationalPerformanceEvidenceStrength::ExplicitDebtDeferredClaim
    ) && matches!(
        fallback,
        FoundationalPerformanceFallbackDebtPosture::Verified
    ) {
        return Err(
            FoundationalPerformancePrimitiveLegalityDenial::DebtOrDeferredEvidenceCannotClaimVerifiedPosture,
        );
    }

    if matches!(
        boundary,
        FoundationalPerformanceBoundary::BoundaryMaterialization
    ) && matches!(
        execution_temperature,
        FoundationalPerformanceExecutionTemperature::HotPath
    ) {
        return Err(
            FoundationalPerformancePrimitiveLegalityDenial::BoundaryMaterializationCannotClaimHotPathTemperature,
        );
    }

    if matches!(
        execution_temperature,
        FoundationalPerformanceExecutionTemperature::HotPath
    ) && matches!(
        freshness,
        FoundationalPerformanceFreshnessRetentionPosture::ReplayDerived
            | FoundationalPerformanceFreshnessRetentionPosture::StaleSupport
            | FoundationalPerformanceFreshnessRetentionPosture::HistoricalRetained
            | FoundationalPerformanceFreshnessRetentionPosture::ReducedRetention
    ) {
        return Err(
            FoundationalPerformancePrimitiveLegalityDenial::HotPathClaimsCannotUseReplayOrStaleFreshness,
        );
    }

    if matches!(
        execution_temperature,
        FoundationalPerformanceExecutionTemperature::HotPath
    ) && included_work.iter().any(|work_class| {
        matches!(
            work_class,
            FoundationalPerformanceWorkClass::ReplayReconstruction
                | FoundationalPerformanceWorkClass::SupportReportAssembly
                | FoundationalPerformanceWorkClass::ForensicParity
        )
    }) {
        return Err(
            FoundationalPerformancePrimitiveLegalityDenial::HotPathIncludedWorkMustExcludeReplaySupportAndForensics,
        );
    }

    if matches!(
        execution_temperature,
        FoundationalPerformanceExecutionTemperature::HotPath
    ) && !excluded_work.iter().any(|work_class| {
        matches!(
            work_class,
            FoundationalPerformanceWorkClass::ReplayReconstruction
                | FoundationalPerformanceWorkClass::SupportReportAssembly
                | FoundationalPerformanceWorkClass::ForensicParity
        )
    }) {
        return Err(
            FoundationalPerformancePrimitiveLegalityDenial::HotPathMustDiscloseExcludedColdSupportWork,
        );
    }

    Ok(())
}
