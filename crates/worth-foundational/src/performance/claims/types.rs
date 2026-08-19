use super::context::FoundationalPerformanceObservationContext;
use crate::performance::primitives::{
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FoundationalPerformanceClaimPayload {
    pub(crate) boundary: FoundationalPerformanceBoundary,
    pub(crate) evidence_strength: FoundationalPerformanceEvidenceStrength,
    pub(crate) breadth_locality: FoundationalPerformanceBreadthLocalityPosture,
    pub(crate) access_pattern: FoundationalPerformanceAccessPatternPosture,
    pub(crate) execution_temperature: FoundationalPerformanceExecutionTemperature,
    pub(crate) freshness_retention: FoundationalPerformanceFreshnessRetentionPosture,
    pub(crate) fallback_debt: FoundationalPerformanceFallbackDebtPosture,
    pub(crate) included_work: Vec<FoundationalPerformanceWorkClass>,
    pub(crate) excluded_work: Vec<FoundationalPerformanceWorkClass>,
    pub(crate) observation_context: Option<FoundationalPerformanceObservationContext>,
}

pub trait FoundationalPerformanceClaimSurface: sealed::Sealed {
    fn boundary(&self) -> FoundationalPerformanceBoundary;
    fn evidence_strength(&self) -> FoundationalPerformanceEvidenceStrength;
    fn breadth_locality(&self) -> FoundationalPerformanceBreadthLocalityPosture;
    fn access_pattern(&self) -> FoundationalPerformanceAccessPatternPosture;
    fn execution_temperature(&self) -> FoundationalPerformanceExecutionTemperature;
    fn freshness_retention(&self) -> FoundationalPerformanceFreshnessRetentionPosture;
    fn fallback_debt(&self) -> FoundationalPerformanceFallbackDebtPosture;
    fn included_work(&self) -> &[FoundationalPerformanceWorkClass];
    fn excluded_work(&self) -> &[FoundationalPerformanceWorkClass];
    fn observation_context(&self) -> Option<&FoundationalPerformanceObservationContext>;
}

macro_rules! performance_claim_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name(pub(crate) FoundationalPerformanceClaimPayload);

        impl $name {
            pub fn boundary(&self) -> FoundationalPerformanceBoundary {
                self.0.boundary
            }

            pub fn evidence_strength(&self) -> FoundationalPerformanceEvidenceStrength {
                self.0.evidence_strength
            }

            pub fn breadth_locality(&self) -> FoundationalPerformanceBreadthLocalityPosture {
                self.0.breadth_locality
            }

            pub fn access_pattern(&self) -> FoundationalPerformanceAccessPatternPosture {
                self.0.access_pattern
            }

            pub fn execution_temperature(&self) -> FoundationalPerformanceExecutionTemperature {
                self.0.execution_temperature
            }

            pub fn freshness_retention(&self) -> FoundationalPerformanceFreshnessRetentionPosture {
                self.0.freshness_retention
            }

            pub fn fallback_debt(&self) -> FoundationalPerformanceFallbackDebtPosture {
                self.0.fallback_debt
            }

            pub fn included_work(&self) -> &[FoundationalPerformanceWorkClass] {
                &self.0.included_work
            }

            pub fn excluded_work(&self) -> &[FoundationalPerformanceWorkClass] {
                &self.0.excluded_work
            }

            pub fn observation_context(
                &self,
            ) -> Option<&FoundationalPerformanceObservationContext> {
                self.0.observation_context.as_ref()
            }
        }

        impl FoundationalPerformanceClaimSurface for $name {
            fn boundary(&self) -> FoundationalPerformanceBoundary {
                self.boundary()
            }

            fn evidence_strength(&self) -> FoundationalPerformanceEvidenceStrength {
                self.evidence_strength()
            }

            fn breadth_locality(&self) -> FoundationalPerformanceBreadthLocalityPosture {
                self.breadth_locality()
            }

            fn access_pattern(&self) -> FoundationalPerformanceAccessPatternPosture {
                self.access_pattern()
            }

            fn execution_temperature(&self) -> FoundationalPerformanceExecutionTemperature {
                self.execution_temperature()
            }

            fn freshness_retention(&self) -> FoundationalPerformanceFreshnessRetentionPosture {
                self.freshness_retention()
            }

            fn fallback_debt(&self) -> FoundationalPerformanceFallbackDebtPosture {
                self.fallback_debt()
            }

            fn included_work(&self) -> &[FoundationalPerformanceWorkClass] {
                self.included_work()
            }

            fn excluded_work(&self) -> &[FoundationalPerformanceWorkClass] {
                self.excluded_work()
            }

            fn observation_context(&self) -> Option<&FoundationalPerformanceObservationContext> {
                self.observation_context()
            }
        }
    };
}

performance_claim_type!(FoundationalAuthoritativePerformanceClaim);
performance_claim_type!(FoundationalSupportDerivedPerformanceClaim);
performance_claim_type!(FoundationalReplayMaterializationPerformanceClaim);
performance_claim_type!(FoundationalPolicyAdmissionPerformanceClaim);

mod sealed {
    use super::{
        FoundationalAuthoritativePerformanceClaim, FoundationalPolicyAdmissionPerformanceClaim,
        FoundationalReplayMaterializationPerformanceClaim,
        FoundationalSupportDerivedPerformanceClaim,
    };

    pub trait Sealed {}

    impl Sealed for FoundationalAuthoritativePerformanceClaim {}
    impl Sealed for FoundationalSupportDerivedPerformanceClaim {}
    impl Sealed for FoundationalReplayMaterializationPerformanceClaim {}
    impl Sealed for FoundationalPolicyAdmissionPerformanceClaim {}
}
