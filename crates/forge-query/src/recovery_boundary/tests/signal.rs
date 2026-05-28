use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::continuation_pipeline::{
    ForgeQueryPreparedContinuationChecked, ForgeQueryPreparedContinuationOutcome,
};
use crate::recovery_boundary::{
    forge_query_recovery_brief_from_prepared_continuation_checked,
    forge_query_recovery_brief_from_signal_compatibility_checked, ForgeQueryRecoveryAction,
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryAuthoritySurface,
    ForgeQueryRecoveryBasisPosture, ForgeQueryRecoverySourceFamily, ForgeQueryRecoveryStopKind,
};
use crate::signal_compatibility_orchestration::{
    ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
};

use super::support::{RecoveryDomain, RecoveryInput, RequiredIntentRouteFamily};

fn continuation_basis_mismatch_checked(
) -> ForgeQueryPreparedContinuationChecked<RecoveryDomain, RecoveryInput<RequiredIntentRouteFamily>>
{
    ForgeQueryPreparedContinuationChecked::new(
        ForgeQueryPreparedContinuationOutcome::BasisMismatch(
            "continuation basis drifted".to_string(),
        ),
        "prepared-basis".to_string(),
        ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-continuation"),
    )
}

fn signal_basis_mismatch_checked() -> ForgeQuerySignalCompatibilityOrchestrationChecked<
    RecoveryDomain,
    RecoveryInput<RequiredIntentRouteFamily>,
> {
    ForgeQuerySignalCompatibilityOrchestrationChecked::new(
        ForgeQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(
            "signal basis drifted".to_string(),
        ),
        "signal-basis".to_string(),
        ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-signal"),
    )
}

#[test]
fn signal_missing_required_aspect_stays_aspect_native() {
    let checked = ForgeQuerySignalCompatibilityOrchestrationChecked::new(
        ForgeQuerySignalCompatibilityOrchestrationOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::MissingRequiredAspect("required signal aspect is missing".to_string()),
        "signal-aspect".to_string(),
        ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-signal-aspect"),
    );

    let brief = forge_query_recovery_brief_from_signal_compatibility_checked(checked)
        .expect("signal aspect denial should recover");

    assert_eq!(
        brief.stop_kind(),
        ForgeQueryRecoveryStopKind::MissingRequiredAspect
    );
    assert_eq!(
        brief.source_family(),
        ForgeQueryRecoverySourceFamily::SignalCompatibility
    );
    assert_eq!(
        brief.aspect_posture(),
        ForgeQueryRecoveryAspectPosture::RequiredContract
    );
    assert_eq!(
        brief.authority_surface(),
        ForgeQueryRecoveryAuthoritySurface::DeclarationMeaning
    );
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::RepairDeclarationMeaning
    );
}

#[test]
fn signal_and_continuation_basis_mismatch_preserve_source_specific_recovery() {
    let continuation = forge_query_recovery_brief_from_prepared_continuation_checked(
        continuation_basis_mismatch_checked(),
    )
    .expect("continuation basis mismatch should recover");
    let signal = forge_query_recovery_brief_from_signal_compatibility_checked(
        signal_basis_mismatch_checked(),
    )
    .expect("signal basis mismatch should recover");

    assert_eq!(
        continuation.basis_posture(),
        ForgeQueryRecoveryBasisPosture::BasisMismatch
    );
    assert_eq!(
        signal.basis_posture(),
        ForgeQueryRecoveryBasisPosture::BasisMismatch
    );
    assert_eq!(
        continuation.source_family(),
        ForgeQueryRecoverySourceFamily::Continuation
    );
    assert_eq!(
        signal.source_family(),
        ForgeQueryRecoverySourceFamily::SignalCompatibility
    );
    assert_eq!(
        continuation.authority_surface(),
        ForgeQueryRecoveryAuthoritySurface::TruthContinuationContext
    );
    assert_eq!(
        signal.authority_surface(),
        ForgeQueryRecoveryAuthoritySurface::SignalCompatibility
    );
}
