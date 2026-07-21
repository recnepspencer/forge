use crate::runtime::host_observation::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::{WorthUiQueryBindingDriftDenial, WorthUiQueryLiveRebindPlanDenial};

pub(crate) fn diagnostic_for_query_live_rebind(
    denial: &WorthUiQueryLiveRebindPlanDenial,
) -> WorthUiRuntimeDiagnostic {
    query_diagnostic(
        WorthUiRuntimeDiagnosticCode::QueryLiveRebindDenied,
        query_live_rebind_digest(denial),
    )
}

pub(crate) fn diagnostic_for_query_recovery(
    denial: &WorthUiQueryBindingDriftDenial,
) -> WorthUiRuntimeDiagnostic {
    query_diagnostic(
        WorthUiRuntimeDiagnosticCode::QueryRecoveryPreserved,
        query_recovery_digest(denial),
    )
}

fn query_diagnostic(
    code: WorthUiRuntimeDiagnosticCode,
    evidence_digest: u64,
) -> WorthUiRuntimeDiagnostic {
    WorthUiRuntimeDiagnostic::new(
        WorthUiRuntimeDiagnosticFamily::QueryLiveRebind,
        code,
        WorthUiDiagnosticSource::PhaseDenial { evidence_digest },
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
        crate::runtime::WorthUiQueryBindingDriftDenialKind::MissingCandidateUiRequirementsForRebind => 1,
        crate::runtime::WorthUiQueryBindingDriftDenialKind::MissingActiveUiRequirementsForRetirement => 2,
    }
}

fn query_drift_family_digest(
    family: crate::runtime::WorthUiQueryBindingUiRequirementsDriftFamily,
) -> u64 {
    match family {
        crate::runtime::WorthUiQueryBindingUiRequirementsDriftFamily::LifecycleDeclaration => 1,
        crate::runtime::WorthUiQueryBindingUiRequirementsDriftFamily::AsyncResultPresentation => 2,
        crate::runtime::WorthUiQueryBindingUiRequirementsDriftFamily::RecoveryPresentation => 3,
        crate::runtime::WorthUiQueryBindingUiRequirementsDriftFamily::InspectionRelevance => 4,
        crate::runtime::WorthUiQueryBindingUiRequirementsDriftFamily::ProjectionConsumption => 5,
        crate::runtime::WorthUiQueryBindingUiRequirementsDriftFamily::DenialPresentation => 6,
    }
}

fn fold_all<const N: usize>(seed: u64, values: [u64; N]) -> u64 {
    values.into_iter().fold(seed, fold)
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
