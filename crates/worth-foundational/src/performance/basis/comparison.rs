use crate::performance::claims::FoundationalPerformanceClaimSurface;

use super::attachments::{
    FoundationalPerformanceContractName, FoundationalPerformanceCounterSpec,
    FoundationalPerformanceSupportingEvidenceRow,
};
use super::bundle::FoundationalPerformanceBundle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalPerformanceMismatch {
    Boundary {
        left: crate::performance::FoundationalPerformanceBoundary,
        right: crate::performance::FoundationalPerformanceBoundary,
    },
    EvidenceStrength {
        left: crate::performance::FoundationalPerformanceEvidenceStrength,
        right: crate::performance::FoundationalPerformanceEvidenceStrength,
    },
    BreadthLocality {
        left: crate::performance::FoundationalPerformanceBreadthLocalityPosture,
        right: crate::performance::FoundationalPerformanceBreadthLocalityPosture,
    },
    AccessPattern {
        left: crate::performance::FoundationalPerformanceAccessPatternPosture,
        right: crate::performance::FoundationalPerformanceAccessPatternPosture,
    },
    ExecutionTemperature {
        left: crate::performance::FoundationalPerformanceExecutionTemperature,
        right: crate::performance::FoundationalPerformanceExecutionTemperature,
    },
    FreshnessRetention {
        left: crate::performance::FoundationalPerformanceFreshnessRetentionPosture,
        right: crate::performance::FoundationalPerformanceFreshnessRetentionPosture,
    },
    FallbackDebt {
        left: crate::performance::FoundationalPerformanceFallbackDebtPosture,
        right: crate::performance::FoundationalPerformanceFallbackDebtPosture,
    },
    IncludedWorkDisclosure {
        left: Vec<crate::performance::FoundationalPerformanceWorkClass>,
        right: Vec<crate::performance::FoundationalPerformanceWorkClass>,
    },
    ExcludedWorkDisclosure {
        left: Vec<crate::performance::FoundationalPerformanceWorkClass>,
        right: Vec<crate::performance::FoundationalPerformanceWorkClass>,
    },
    ObservationContext {
        left: Option<crate::performance::FoundationalPerformanceObservationContext>,
        right: Option<crate::performance::FoundationalPerformanceObservationContext>,
    },
    LayoutIntentPresence {
        left_has_layout: bool,
        right_has_layout: bool,
    },
    LayoutIntent {
        left: crate::performance::FoundationalPerformanceLayoutIntent,
        right: crate::performance::FoundationalPerformanceLayoutIntent,
    },
    AllocationPosture {
        left: crate::performance::FoundationalPerformanceAllocationPosture,
        right: crate::performance::FoundationalPerformanceAllocationPosture,
    },
    ContractNames {
        left: Vec<FoundationalPerformanceContractName>,
        right: Vec<FoundationalPerformanceContractName>,
    },
    CounterSpecs {
        left: Vec<FoundationalPerformanceCounterSpec>,
        right: Vec<FoundationalPerformanceCounterSpec>,
    },
    SupportingEvidenceRows {
        left: Vec<FoundationalPerformanceSupportingEvidenceRow>,
        right: Vec<FoundationalPerformanceSupportingEvidenceRow>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPerformanceComparison {
    mismatches: Vec<FoundationalPerformanceMismatch>,
}

impl FoundationalPerformanceComparison {
    pub fn mismatches(&self) -> &[FoundationalPerformanceMismatch] {
        &self.mismatches
    }

    pub fn is_equivalent(&self) -> bool {
        self.mismatches.is_empty()
    }
}

pub fn compare_performance_bundles<LeftClaim, RightClaim>(
    left: &FoundationalPerformanceBundle<LeftClaim>,
    right: &FoundationalPerformanceBundle<RightClaim>,
) -> FoundationalPerformanceComparison
where
    LeftClaim: FoundationalPerformanceClaimSurface,
    RightClaim: FoundationalPerformanceClaimSurface,
{
    let mut mismatches = Vec::new();

    if left.claim().boundary() != right.claim().boundary() {
        mismatches.push(FoundationalPerformanceMismatch::Boundary {
            left: left.claim().boundary(),
            right: right.claim().boundary(),
        });
    }
    if left.claim().evidence_strength() != right.claim().evidence_strength() {
        mismatches.push(FoundationalPerformanceMismatch::EvidenceStrength {
            left: left.claim().evidence_strength(),
            right: right.claim().evidence_strength(),
        });
    }
    if left.claim().breadth_locality() != right.claim().breadth_locality() {
        mismatches.push(FoundationalPerformanceMismatch::BreadthLocality {
            left: left.claim().breadth_locality(),
            right: right.claim().breadth_locality(),
        });
    }
    if left.claim().access_pattern() != right.claim().access_pattern() {
        mismatches.push(FoundationalPerformanceMismatch::AccessPattern {
            left: left.claim().access_pattern(),
            right: right.claim().access_pattern(),
        });
    }
    if left.claim().execution_temperature() != right.claim().execution_temperature() {
        mismatches.push(FoundationalPerformanceMismatch::ExecutionTemperature {
            left: left.claim().execution_temperature(),
            right: right.claim().execution_temperature(),
        });
    }
    if left.claim().freshness_retention() != right.claim().freshness_retention() {
        mismatches.push(FoundationalPerformanceMismatch::FreshnessRetention {
            left: left.claim().freshness_retention(),
            right: right.claim().freshness_retention(),
        });
    }
    if left.claim().fallback_debt() != right.claim().fallback_debt() {
        mismatches.push(FoundationalPerformanceMismatch::FallbackDebt {
            left: left.claim().fallback_debt(),
            right: right.claim().fallback_debt(),
        });
    }
    if left.claim().included_work() != right.claim().included_work() {
        mismatches.push(FoundationalPerformanceMismatch::IncludedWorkDisclosure {
            left: left.claim().included_work().to_vec(),
            right: right.claim().included_work().to_vec(),
        });
    }
    if left.claim().excluded_work() != right.claim().excluded_work() {
        mismatches.push(FoundationalPerformanceMismatch::ExcludedWorkDisclosure {
            left: left.claim().excluded_work().to_vec(),
            right: right.claim().excluded_work().to_vec(),
        });
    }
    if left.claim().observation_context() != right.claim().observation_context() {
        mismatches.push(FoundationalPerformanceMismatch::ObservationContext {
            left: left.claim().observation_context().cloned(),
            right: right.claim().observation_context().cloned(),
        });
    }

    match (left.layout_intent_claim(), right.layout_intent_claim()) {
        (Some(left_layout), Some(right_layout)) => {
            if left_layout.layout_intent() != right_layout.layout_intent() {
                mismatches.push(FoundationalPerformanceMismatch::LayoutIntent {
                    left: left_layout.layout_intent(),
                    right: right_layout.layout_intent(),
                });
            }
            if left_layout.allocation_posture() != right_layout.allocation_posture() {
                mismatches.push(FoundationalPerformanceMismatch::AllocationPosture {
                    left: left_layout.allocation_posture(),
                    right: right_layout.allocation_posture(),
                });
            }
        }
        (left_layout, right_layout) if left_layout.is_some() != right_layout.is_some() => {
            mismatches.push(FoundationalPerformanceMismatch::LayoutIntentPresence {
                left_has_layout: left_layout.is_some(),
                right_has_layout: right_layout.is_some(),
            });
        }
        _ => {}
    }

    if left.contract_names() != right.contract_names() {
        mismatches.push(FoundationalPerformanceMismatch::ContractNames {
            left: left.contract_names().to_vec(),
            right: right.contract_names().to_vec(),
        });
    }
    if left.counter_specs() != right.counter_specs() {
        mismatches.push(FoundationalPerformanceMismatch::CounterSpecs {
            left: left.counter_specs().to_vec(),
            right: right.counter_specs().to_vec(),
        });
    }
    if left.supporting_evidence_rows() != right.supporting_evidence_rows() {
        mismatches.push(FoundationalPerformanceMismatch::SupportingEvidenceRows {
            left: left.supporting_evidence_rows().to_vec(),
            right: right.supporting_evidence_rows().to_vec(),
        });
    }

    FoundationalPerformanceComparison { mismatches }
}
