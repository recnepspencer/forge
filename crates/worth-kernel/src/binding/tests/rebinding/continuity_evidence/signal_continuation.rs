use forge_query::facade::{
    ForgeQueryContinuationExecutionOutcome, ForgeQueryDeclarationBridgeRoutingSupportStatus,
    ForgeQueryDeclarationSignalCompatibilitySupportStatus, ForgeQueryOrdinaryOutcome,
    ForgeQueryPreparedContinuationOutcome, ForgeQuerySignalCompatibilityOrchestrationOutcome,
};

use crate::binding::tests::support::{admitted_rebinding_handle, face_surface_rebinding_fixture};
use worth_spatial::facade::continuation::{
    primitive_rebinding_continuation_target, primitive_rebinding_signal_workflow,
};

const SIGNAL_BASIS_MISMATCH_REASON: &str =
    "the retained envelope truth does not currently satisfy the required basis-sensitive signal continuation posture";

#[test]
fn rebinding_signal_compatibility_and_continuation_keep_checked_runtime_lanes() {
    let fixture = face_surface_rebinding_fixture();
    let handle = admitted_rebinding_handle("kernel-rebinding-signal");
    let bridge_support = handle
        .bridge_continuation_support::<
            worth_spatial::facade::bindings::PrimitiveRebindingDeclarationEntry,
        >();
    let bridge_row = bridge_support
        .rows()
        .first()
        .expect("rebinding bridge continuation support row");
    assert_eq!(
        bridge_row.status(),
        ForgeQueryDeclarationBridgeRoutingSupportStatus::Admitted,
        "bridge status={:?} reason={} aspect_fit={:?} aspect_mismatch={:?} mapping_fit={:?}",
        bridge_row.status(),
        bridge_row.reason(),
        bridge_row.aspect_fit(),
        bridge_row.aspect_mismatch(),
        bridge_row.mapping_fit(),
    );

    let signal_envelope =
        match handle.orchestrate_declaration_entry_outcome(fixture.declaration.clone()) {
            ForgeQueryOrdinaryOutcome::Bound(envelope) => envelope,
            _ => panic!("expected bound rebinding envelope for signal workflow"),
        };
    let signal_support = handle
        .signal_compatibility_support::<
            worth_spatial::facade::bindings::PrimitiveRebindingDeclarationEntry,
        >();
    let signal_row = signal_support
        .rows()
        .first()
        .expect("rebinding signal compatibility support row");
    assert_eq!(
        signal_row.status(),
        ForgeQueryDeclarationSignalCompatibilitySupportStatus::Admitted
    );

    let signal_checked = handle.orchestrate_signal_compatibility_checked(
        primitive_rebinding_signal_workflow(signal_envelope),
    );
    assert!(!signal_checked.orchestration_digest().is_empty());
    assert!(signal_checked
        .linked_artifacts()
        .declaration_digest()
        .is_some());
    assert!(signal_checked
        .linked_artifacts()
        .envelope_digest()
        .is_some());
    match signal_checked.outcome() {
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(artifact) => {
            assert_eq!(
                signal_checked.linked_artifacts().declaration_digest(),
                Some(artifact.declaration_digest())
            );
            assert!(signal_checked
                .linked_artifacts()
                .envelope_digest()
                .is_some());
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(_) => {
            assert!(signal_checked
                .linked_artifacts()
                .declaration_digest()
                .is_some());
            assert!(signal_checked
                .linked_artifacts()
                .envelope_digest()
                .is_some());
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Ambiguous(_) => {
            panic!("signal: ambiguous")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Unavailable(_) => {
            panic!("signal: unavailable")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(_) => {
            panic!("signal: wrong-world")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(_) => {
            panic!("signal: wrong-handle")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Stale(_) => panic!("signal: stale"),
        ForgeQuerySignalCompatibilityOrchestrationOutcome::RebindRequired(_) => {
            panic!("signal: rebind-required")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::MissingRequiredAspect(_) => {
            panic!("signal: missing-required-aspect")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::AspectConflict(_) => {
            panic!("signal: aspect-conflict")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::AuthorityMismatch(_) => {
            panic!("signal: authority-mismatch")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Deferred(_) => {
            panic!("signal: deferred")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Denied(_) => panic!("signal: denied"),
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Unsupported(_) => {
            panic!("signal: unsupported")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Failed(_) => panic!("signal: failed"),
    }

    let continuation_envelope =
        match handle.orchestrate_declaration_entry_outcome(fixture.declaration.clone()) {
            ForgeQueryOrdinaryOutcome::Bound(envelope) => envelope,
            _ => panic!("expected bound rebinding envelope for continuation workflow"),
        };
    let prepared_checked = handle.prepare_continuation_from_target_checked(
        primitive_rebinding_continuation_target(continuation_envelope),
    );
    assert!(!prepared_checked.prepared_digest().is_empty());
    assert!(prepared_checked
        .linked_artifacts()
        .envelope_digest()
        .is_some());
    match prepared_checked.outcome() {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => {
            assert_eq!(
                prepared_checked.linked_artifacts().declaration_digest(),
                Some(prepared.declaration_digest())
            );
            assert!(prepared_checked
                .linked_artifacts()
                .envelope_digest()
                .is_some());
        }
        ForgeQueryPreparedContinuationOutcome::Ambiguous(_) => panic!("continuation: ambiguous"),
        ForgeQueryPreparedContinuationOutcome::Unavailable(_) => {
            panic!("continuation: unavailable")
        }
        ForgeQueryPreparedContinuationOutcome::WrongWorld(_) => panic!("continuation: wrong-world"),
        ForgeQueryPreparedContinuationOutcome::WrongHandle(_) => {
            panic!("continuation: wrong-handle")
        }
        ForgeQueryPreparedContinuationOutcome::Stale(_) => panic!("continuation: stale"),
        ForgeQueryPreparedContinuationOutcome::RebindRequired(_) => {
            panic!("continuation: rebind-required")
        }
        ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(_) => {
            panic!("continuation: authority-mismatch")
        }
        ForgeQueryPreparedContinuationOutcome::BasisMismatch(_) => {
            panic!("continuation: basis-mismatch")
        }
        ForgeQueryPreparedContinuationOutcome::Unsupported(_) => {
            panic!("continuation: unsupported")
        }
        ForgeQueryPreparedContinuationOutcome::Deferred(_) => panic!("continuation: deferred"),
        ForgeQueryPreparedContinuationOutcome::Denied(reason) => {
            assert_eq!(reason, SIGNAL_BASIS_MISMATCH_REASON);
            assert!(prepared_checked
                .linked_artifacts()
                .declaration_digest()
                .is_some());
            assert!(prepared_checked
                .linked_artifacts()
                .envelope_digest()
                .is_some());
        }
        ForgeQueryPreparedContinuationOutcome::Failed(reason) => {
            panic!("continuation: failed: {reason}")
        }
    };

    let execution_envelope = match handle.orchestrate_declaration_entry_outcome(fixture.declaration)
    {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => envelope,
        _ => panic!("expected bound rebinding envelope for continuation execution"),
    };
    let owned_prepared = match handle.prepare_continuation_from_target(
        primitive_rebinding_continuation_target(execution_envelope),
    ) {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        ForgeQueryPreparedContinuationOutcome::Ambiguous(_) => {
            panic!("owned continuation: ambiguous")
        }
        ForgeQueryPreparedContinuationOutcome::Unavailable(_) => {
            panic!("owned continuation: unavailable")
        }
        ForgeQueryPreparedContinuationOutcome::WrongWorld(_) => {
            panic!("owned continuation: wrong-world")
        }
        ForgeQueryPreparedContinuationOutcome::WrongHandle(_) => {
            panic!("owned continuation: wrong-handle")
        }
        ForgeQueryPreparedContinuationOutcome::Stale(_) => panic!("owned continuation: stale"),
        ForgeQueryPreparedContinuationOutcome::RebindRequired(_) => {
            panic!("owned continuation: rebind-required")
        }
        ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(_) => {
            panic!("owned continuation: authority-mismatch")
        }
        ForgeQueryPreparedContinuationOutcome::BasisMismatch(_) => {
            panic!("owned continuation: basis-mismatch")
        }
        ForgeQueryPreparedContinuationOutcome::Unsupported(_) => {
            panic!("owned continuation: unsupported")
        }
        ForgeQueryPreparedContinuationOutcome::Deferred(_) => {
            panic!("owned continuation: deferred")
        }
        ForgeQueryPreparedContinuationOutcome::Denied(reason) => {
            assert_eq!(reason, SIGNAL_BASIS_MISMATCH_REASON);
            return;
        }
        ForgeQueryPreparedContinuationOutcome::Failed(reason) => {
            panic!("owned continuation: failed: {reason}")
        }
    };

    let execution_checked = handle.execute_prepared_continuation_checked(owned_prepared);
    assert!(!execution_checked.execution_digest().is_empty());
    assert!(execution_checked
        .linked_artifacts()
        .envelope_digest()
        .is_some());
    match execution_checked.outcome() {
        ForgeQueryContinuationExecutionOutcome::Executed(executed) => {
            assert_eq!(
                execution_checked.linked_artifacts().declaration_digest(),
                Some(executed.prepared().declaration_digest())
            );
            assert!(execution_checked
                .linked_artifacts()
                .envelope_digest()
                .is_some());
        }
        _ => panic!("expected executed rebinding continuation"),
    }
}
