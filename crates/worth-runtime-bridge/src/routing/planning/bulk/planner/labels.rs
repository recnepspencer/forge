pub(crate) fn parallel_legality_class_label(class: BridgeParallelLegalityClass) -> &'static str {
    match class {
        BridgeParallelLegalityClass::SerialOnly => "serial-only",
        BridgeParallelLegalityClass::ParallelPreparationLegal => "parallel-preparation-legal",
        BridgeParallelLegalityClass::ParallelPreparationIllegal => "parallel-preparation-illegal",
    }
}

pub(crate) fn parallel_legality_reason_label(reason: BridgeParallelLegalityReason) -> &'static str {
    match reason {
        BridgeParallelLegalityReason::BelowMinWorkloadWidth => "below-min-workload-width",
        BridgeParallelLegalityReason::SharedTruthViewMaterializationTarget => {
            "shared-truth-view-materialization-target"
        }
        BridgeParallelLegalityReason::ContinuityRemapRequiresSerialPreparation => {
            "continuity-remap-requires-serial-preparation"
        }
        BridgeParallelLegalityReason::PacketRegionOverlapDetected => {
            "packet-region-overlap-detected"
        }
        BridgeParallelLegalityReason::DisjointPacketRegionsCertified => {
            "disjoint-packet-regions-certified"
        }
    }
}

pub(crate) fn parallel_profitability_class_label(
    class: BridgeParallelProfitabilityClass,
) -> &'static str {
    match class {
        BridgeParallelProfitabilityClass::NotApplicable => "not-applicable",
        BridgeParallelProfitabilityClass::Profitable => "profitable",
        BridgeParallelProfitabilityClass::Unprofitable => "unprofitable",
    }
}

pub(crate) fn parallel_profitability_reason_label(
    reason: BridgeParallelProfitabilityReason,
) -> &'static str {
    match reason {
        BridgeParallelProfitabilityReason::SerialOnlyWorkload => "serial-only-workload",
        BridgeParallelProfitabilityReason::SharedPublicationReductionTarget => {
            "shared-publication-reduction-target"
        }
        BridgeParallelProfitabilityReason::AdmittedOperational => "admitted-operational",
    }
}

pub(crate) fn parallel_admission_class_label(class: BridgeParallelAdmissionClass) -> &'static str {
    match class {
        BridgeParallelAdmissionClass::SerialRequired => "serial-required",
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted => {
            "parallel-preparation-admitted"
        }
        BridgeParallelAdmissionClass::ParallelPreparationRejected => {
            "parallel-preparation-rejected"
        }
    }
}

pub(crate) fn parallel_admission_reason_label(
    reason: BridgeParallelAdmissionReason,
) -> &'static str {
    match reason {
        BridgeParallelAdmissionReason::SerialExecutor => "serial-executor",
        BridgeParallelAdmissionReason::BelowMinWorkloadWidth => "below-min-workload-width",
        BridgeParallelAdmissionReason::SharedPublicationReductionTarget => {
            "shared-publication-reduction-target"
        }
        BridgeParallelAdmissionReason::SharedTruthViewMaterializationTarget => {
            "shared-truth-view-materialization-target"
        }
        BridgeParallelAdmissionReason::ContinuityRemapRequiresSerialPreparation => {
            "continuity-remap-requires-serial-preparation"
        }
        BridgeParallelAdmissionReason::PacketRegionOverlapDetected => {
            "packet-region-overlap-detected"
        }
        BridgeParallelAdmissionReason::AdmittedOperational => "admitted-operational",
    }
}

pub(crate) fn bulk_decision_kind_label(kind: BridgeBulkDecisionRecordKind) -> &'static str {
    match kind {
        BridgeBulkDecisionRecordKind::ParallelLegality => "parallel-legality",
        BridgeBulkDecisionRecordKind::ParallelProfitability => "parallel-profitability",
        BridgeBulkDecisionRecordKind::ParallelAdmission => "parallel-admission",
    }
}

pub(crate) fn planning_failure_kind_label(kind: BridgeBulkPlanningFailureKind) -> &'static str {
    match kind {
        BridgeBulkPlanningFailureKind::WorkloadSummaryConstructionFailure => {
            "workload-summary-construction-failure"
        }
        BridgeBulkPlanningFailureKind::ZeroRoutedItemWorkload => "zero-routed-item-workload",
        BridgeBulkPlanningFailureKind::UnsupportedPacketClass => "unsupported-packet-class",
        BridgeBulkPlanningFailureKind::InvalidReductionBasis => "invalid-reduction-basis",
        BridgeBulkPlanningFailureKind::InvalidParallelAdmissionBasis => {
            "invalid-parallel-admission-basis"
        }
        BridgeBulkPlanningFailureKind::PacketOverlapDetected => "packet-overlap-detected",
        BridgeBulkPlanningFailureKind::ReductionIdentityConflict => "reduction-identity-conflict",
        BridgeBulkPlanningFailureKind::ParallelPreparationNotProfitable => {
            "parallel-preparation-not-profitable"
        }
        BridgeBulkPlanningFailureKind::ReducerBufferCeilingExceeded => {
            "reducer-buffer-ceiling-exceeded"
        }
        BridgeBulkPlanningFailureKind::DiagnosticsFragmentCeilingExceeded => {
            "diagnostics-fragment-ceiling-exceeded"
        }
    }
}

pub(crate) fn preparation_mode_label(mode: BridgePreparationMode) -> &'static str {
    match mode {
        BridgePreparationMode::Serial => "serial",
        BridgePreparationMode::ParallelPreparation => "parallel-preparation",
    }
}

use super::*;
