use crate::binding_pipeline::WorthQueryBindingLinkedArtifacts;
use crate::continuation_pipeline::{
    WorthQueryPreparedContinuationChecked, WorthQueryPreparedContinuationOutcome,
};
use crate::recovery_boundary::{
    worth_query_recovery_brief_from_prepared_continuation_checked,
    worth_query_recovery_brief_from_signal_compatibility_checked, WorthQueryRecoveryAction,
    WorthQueryRecoveryAspectPosture, WorthQueryRecoveryAuthoritySurface,
    WorthQueryRecoveryBasisPosture, WorthQueryRecoverySourceFamily, WorthQueryRecoveryStopKind,
};
use crate::signal_compatibility_orchestration::{
    WorthQuerySignalCompatibilityOrchestrationChecked,
    WorthQuerySignalCompatibilityOrchestrationOutcome,
};

use super::support::{RecoveryDomain, RecoveryInput, RequiredIntentRouteFamily};

fn continuation_basis_mismatch_checked(
) -> WorthQueryPreparedContinuationChecked<RecoveryDomain, RecoveryInput<RequiredIntentRouteFamily>>
{
    WorthQueryPreparedContinuationChecked::new(
        WorthQueryPreparedContinuationOutcome::BasisMismatch(
            "continuation basis drifted".to_string(),
        ),
        "prepared-basis".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-continuation"),
    )
}

fn signal_basis_mismatch_checked() -> WorthQuerySignalCompatibilityOrchestrationChecked<
    RecoveryDomain,
    RecoveryInput<RequiredIntentRouteFamily>,
> {
    WorthQuerySignalCompatibilityOrchestrationChecked::new(
        WorthQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(
            "signal basis drifted".to_string(),
        ),
        "signal-basis".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-signal"),
    )
}

#[test]
fn signal_missing_required_aspect_stays_aspect_native() {
    let checked = WorthQuerySignalCompatibilityOrchestrationChecked::new(
        WorthQuerySignalCompatibilityOrchestrationOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::MissingRequiredAspect("required signal aspect is missing".to_string()),
        "signal-aspect".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-signal-aspect"),
    );

    let brief = worth_query_recovery_brief_from_signal_compatibility_checked(checked)
        .expect("signal aspect denial should recover");

    assert_eq!(
        brief.stop_kind(),
        WorthQueryRecoveryStopKind::MissingRequiredAspect
    );
    assert_eq!(
        brief.source_family(),
        WorthQueryRecoverySourceFamily::SignalCompatibility
    );
    assert_eq!(
        brief.aspect_posture(),
        WorthQueryRecoveryAspectPosture::RequiredContract
    );
    assert_eq!(
        brief.authority_surface(),
        WorthQueryRecoveryAuthoritySurface::DeclarationMeaning
    );
    assert_eq!(
        brief.recommended_action(),
        WorthQueryRecoveryAction::RepairDeclarationMeaning
    );
}

#[test]
fn signal_and_continuation_basis_mismatch_preserve_source_specific_recovery() {
    let continuation = worth_query_recovery_brief_from_prepared_continuation_checked(
        continuation_basis_mismatch_checked(),
    )
    .expect("continuation basis mismatch should recover");
    let signal = worth_query_recovery_brief_from_signal_compatibility_checked(
        signal_basis_mismatch_checked(),
    )
    .expect("signal basis mismatch should recover");

    assert_eq!(
        continuation.basis_posture(),
        WorthQueryRecoveryBasisPosture::BasisMismatch
    );
    assert_eq!(
        signal.basis_posture(),
        WorthQueryRecoveryBasisPosture::BasisMismatch
    );
    assert_eq!(
        continuation.source_family(),
        WorthQueryRecoverySourceFamily::Continuation
    );
    assert_eq!(
        signal.source_family(),
        WorthQueryRecoverySourceFamily::SignalCompatibility
    );
    assert_eq!(
        continuation.authority_surface(),
        WorthQueryRecoveryAuthoritySurface::TruthContinuationContext
    );
    assert_eq!(
        signal.authority_surface(),
        WorthQueryRecoveryAuthoritySurface::SignalCompatibility
    );
}
