use crate::runtime::host_observation::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryLiveRebindPlanDenial,
    WorthUiReloadCheckedStopPosture,
};

pub(crate) fn diagnostic_for_query_live_rebind(
    denial: &WorthUiQueryLiveRebindPlanDenial,
) -> WorthUiRuntimeDiagnostic {
    query_diagnostic(
        WorthUiRuntimeDiagnosticCode::QueryLiveRebindDenied,
        WorthUiReloadCheckedStopPosture::query_support_denied(),
        query_live_rebind_digest(denial),
    )
}

pub(crate) fn diagnostic_for_query_recovery(
    denial: &WorthUiQueryBindingDriftDenial,
) -> WorthUiRuntimeDiagnostic {
    query_diagnostic(
        WorthUiRuntimeDiagnosticCode::QueryRecoveryPreserved,
        WorthUiReloadCheckedStopPosture::query_recovery_preserved(),
        query_recovery_digest(denial),
    )
}

fn query_diagnostic(
    code: WorthUiRuntimeDiagnosticCode,
    checked_stop_posture: WorthUiReloadCheckedStopPosture,
    evidence_digest: u64,
) -> WorthUiRuntimeDiagnostic {
    WorthUiRuntimeDiagnostic::new(
        WorthUiRuntimeDiagnosticFamily::QueryLiveRebind,
        code,
        WorthUiDiagnosticSource::QueryStop {
            checked_stop_posture,
            evidence_digest,
        },
        Some(evidence_digest),
    )
}

fn query_live_rebind_digest(denial: &WorthUiQueryLiveRebindPlanDenial) -> u64 {
    match denial {
        WorthUiQueryLiveRebindPlanDenial::AmbiguousNodeReplacementPlan => 0xB1_00_00_01,
        WorthUiQueryLiveRebindPlanDenial::ComparisonDigestMismatch {
            comparison_active_artifact_digest,
            plan_active_artifact_digest,
            comparison_candidate_artifact_digest,
            plan_candidate_artifact_digest,
        } => fold_all(
            0xB1_00_00_02,
            [
                *comparison_active_artifact_digest,
                *plan_active_artifact_digest,
                *comparison_candidate_artifact_digest,
                *plan_candidate_artifact_digest,
            ],
        ),
        WorthUiQueryLiveRebindPlanDenial::NarrowingDigestMismatch {
            comparison_active_artifact_digest,
            narrowing_active_artifact_digest,
            comparison_candidate_artifact_digest,
            narrowing_candidate_artifact_digest,
        } => fold_all(
            0xB1_00_00_03,
            [
                *comparison_active_artifact_digest,
                *narrowing_active_artifact_digest,
                *comparison_candidate_artifact_digest,
                *narrowing_candidate_artifact_digest,
            ],
        ),
        WorthUiQueryLiveRebindPlanDenial::AdmittedCandidateDigestMismatch {
            comparison_candidate_artifact_digest,
            admitted_candidate_artifact_digest,
        } => fold_all(
            0xB1_00_00_04,
            [
                *comparison_candidate_artifact_digest,
                *admitted_candidate_artifact_digest,
            ],
        ),
        WorthUiQueryLiveRebindPlanDenial::AdmittedQuerySupportContractChanged {
            admitted_contract_identity,
            current_contract_identity,
        } => fold_all(
            0xB1_00_00_05,
            [
                admitted_contract_identity.as_u64(),
                current_contract_identity.as_u64(),
            ],
        ),
    }
}

fn query_recovery_digest(denial: &WorthUiQueryBindingDriftDenial) -> u64 {
    let identity = denial.identity();
    let mut digest = fold_all(
        0xB2_00_00_01,
        [
            identity.canonical_identity(),
            query_recovery_reason_digest(denial.reason()),
        ],
    );
    for family in denial.drift_families() {
        digest = fold(digest, query_drift_family_digest(*family));
    }
    digest
}

fn query_recovery_reason_digest(reason: crate::runtime::WorthUiQueryBindingDriftDenialKind) -> u64 {
    match reason {
        crate::runtime::WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery => 1,
        crate::runtime::WorthUiQueryBindingDriftDenialKind::QuerySupportPostureNotAdmitted => 2,
        crate::runtime::WorthUiQueryBindingDriftDenialKind::MissingCandidatePostureForRebind => 3,
        crate::runtime::WorthUiQueryBindingDriftDenialKind::MissingActivePostureForRetirement => 4,
    }
}

fn query_drift_family_digest(family: crate::runtime::WorthUiQueryBindingPostureDriftFamily) -> u64 {
    match family {
        crate::runtime::WorthUiQueryBindingPostureDriftFamily::SupportAdmission => 1,
        crate::runtime::WorthUiQueryBindingPostureDriftFamily::BasisCapability => 2,
        crate::runtime::WorthUiQueryBindingPostureDriftFamily::LiveCompatibility => 3,
        crate::runtime::WorthUiQueryBindingPostureDriftFamily::AsyncResultState => 4,
        crate::runtime::WorthUiQueryBindingPostureDriftFamily::Recovery => 5,
        crate::runtime::WorthUiQueryBindingPostureDriftFamily::Inspection => 6,
        crate::runtime::WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption => 7,
        crate::runtime::WorthUiQueryBindingPostureDriftFamily::DenialPresentation => 8,
    }
}

fn fold_all<const N: usize>(seed: u64, values: [u64; N]) -> u64 {
    values.into_iter().fold(seed, fold)
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
