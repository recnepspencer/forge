use crate::application::{
    ForgeQueryDeclarationSignalCompatibilityChecked, ForgeQueryDeclarationSignalCompatibilityInput,
    ForgeQueryDeclarationSignalCompatibilitySupportRow,
};
use crate::continuation_pipeline::{
    ForgeQueryContinuationBasisPosture, ForgeQueryContinuationRuntimeContract,
    ForgeQueryContinuationTruthContext, ForgeQueryContinuationWorkspaceContract,
    ForgeQueryPreparedContinuationBasisKind, ForgeQueryPreparedContinuationFamily,
    ForgeQueryPreparedContinuationOutcome,
};
use crate::ForgeQueryEvidenceScope;

use super::support::{
    admitted_handle, context_request, envelope, historical_truth_view_request,
    preview_session_request, runtime_route_request, target_request, HistoricalFamily,
    PreviewFamily, RuntimeFamily,
};

#[test]
fn runtime_family_signal_compatibility_is_compatible_before_preparation() {
    let handle = admitted_handle("main");
    let support = handle.signal_compatibility_support::<super::support::Input<RuntimeFamily>>();
    let support_summary = support
        .rows()
        .iter()
        .map(|row: &ForgeQueryDeclarationSignalCompatibilitySupportRow| {
            format!(
                "{}:{}:{}",
                row.execution_family().as_str(),
                row.basis_family().as_str(),
                row.status().as_str()
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    match handle.signal_compatibility_checked(
        ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope::<RuntimeFamily>(
            &handle, "face-a",
        )),
    ) {
        ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(_) => {}
        ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(value) => panic!(
            "runtime continuation family unexpectedly deferred before preparation: {}; support rows = {}",
            value.reason(),
            support_summary
        ),
        ForgeQueryDeclarationSignalCompatibilityChecked::Denied(value) => panic!(
            "runtime continuation family unexpectedly denied before preparation: {}; support rows = {}",
            value.reason(),
            support_summary
        ),
        ForgeQueryDeclarationSignalCompatibilityChecked::Failed(value) => panic!(
            "runtime continuation family unexpectedly failed before preparation: {}; support rows = {}",
            value.reason(),
            support_summary
        ),
    }
}

#[test]
fn prepare_runtime_continuation_from_target_keeps_execution_explicit() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected prepared continuation"),
    };
    assert_eq!(
        prepared.family(),
        ForgeQueryPreparedContinuationFamily::BridgeRuntimeRoute
    );
    assert_eq!(
        prepared.truth_context(),
        ForgeQueryContinuationTruthContext::Current
    );
    assert_eq!(
        prepared.runtime_contract(),
        ForgeQueryContinuationRuntimeContract::RuntimeRoute
    );
    assert_eq!(
        prepared.workspace_contract(),
        ForgeQueryContinuationWorkspaceContract::RuntimeWorkspace
    );
    assert!(!prepared.prepared_digest().is_empty());
}

#[test]
fn target_and_context_preparation_converge_for_equivalent_runtime_meaning() {
    let handle = admitted_handle("main");
    let from_target =
        handle.prepare_continuation_from_target_checked(target_request::<RuntimeFamily>(
            &handle,
            "face-a",
            runtime_route_request(),
        ));
    let from_context =
        handle.prepare_continuation_from_context_checked(context_request(&handle, "face-a"));

    let target_prepared = match from_target.outcome() {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected target preparation"),
    };
    let context_prepared = match from_context.outcome() {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected context preparation"),
    };

    assert_eq!(
        target_prepared.prepared_digest(),
        context_prepared.prepared_digest()
    );
    assert_eq!(
        from_target.linked_artifacts(),
        from_context.linked_artifacts()
    );
}

#[test]
fn continuation_prepare_proof_exposes_target_specific_witnesses() {
    let handle = admitted_handle("main");
    let proof = handle.prepare_continuation_from_target_proof(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    ));

    assert_eq!(proof.request().request_kind(), "prepared_continuation");
    assert!(matches!(
        proof.outcome(),
        ForgeQueryPreparedContinuationOutcome::Prepared(_)
    ));
    assert_eq!(proof.witness_checks().len(), 3);
    assert_eq!(proof.witness_checks()[0].name(), "continuation_binding");
    assert!(proof.witness_checks()[0].did_pass());
    assert_eq!(proof.witness_checks()[1].name(), "signal_compatibility");
    assert_eq!(proof.witness_checks()[2].name(), "bridge_routing");
    assert!(proof.witness_checks()[2].did_pass());
    assert!(proof.linked_artifacts().declaration_digest().is_some());
    assert!(proof.linked_artifacts().route_plan_digest().is_some());
    assert!(proof.linked_artifacts().receipt_digest().is_some());
    assert!(proof.linked_artifacts().envelope_digest().is_some());
}

#[test]
fn preparation_keeps_current_historical_and_preview_truth_distinct() {
    let handle = admitted_handle("main");

    let current = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected current preparation"),
    };
    let historical = match handle.prepare_continuation_from_target(
        target_request::<HistoricalFamily>(&handle, "face-a", historical_truth_view_request()),
    ) {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected historical preparation"),
    };
    let preview = match handle.prepare_continuation_from_target(target_request::<PreviewFamily>(
        &handle,
        "face-a",
        preview_session_request(),
    )) {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected preview preparation"),
    };

    assert_eq!(
        current.truth_context(),
        ForgeQueryContinuationTruthContext::Current
    );
    assert_eq!(
        historical.truth_context(),
        ForgeQueryContinuationTruthContext::Historical
    );
    assert_eq!(
        preview.truth_context(),
        ForgeQueryContinuationTruthContext::Preview
    );
    assert_eq!(
        current.basis_posture(),
        ForgeQueryContinuationBasisPosture::CurrentHead
    );
    assert_eq!(
        historical.basis_posture(),
        ForgeQueryContinuationBasisPosture::HistoricalSnapshot
    );
    assert_eq!(
        preview.basis_posture(),
        ForgeQueryContinuationBasisPosture::PreviewDerived
    );
    assert_eq!(
        current.execution_readmission().basis_witness().kind(),
        ForgeQueryPreparedContinuationBasisKind::Current
    );
    assert_eq!(
        historical.execution_readmission().basis_witness().kind(),
        ForgeQueryPreparedContinuationBasisKind::Historical
    );
    assert_eq!(
        preview.execution_readmission().basis_witness().kind(),
        ForgeQueryPreparedContinuationBasisKind::PreviewDerived
    );
    assert_ne!(
        current
            .execution_readmission()
            .basis_witness()
            .basis_identity_digest(),
        historical
            .execution_readmission()
            .basis_witness()
            .basis_identity_digest()
    );
    assert_ne!(
        current
            .execution_readmission()
            .basis_witness()
            .basis_identity_digest(),
        preview
            .execution_readmission()
            .basis_witness()
            .basis_identity_digest()
    );
    let preview_witness = preview.execution_readmission().basis_witness();
    let preview_source_basis = preview_witness
        .source_basis_identity()
        .expect("preview continuation should retain source basis identity");
    let preview_lower_runtime_binding = preview_witness
        .expected_lower_runtime_binding_identity()
        .expect("preview continuation should retain lower-runtime binding identity");
    assert_eq!(
        preview_source_basis.scope(),
        ForgeQueryEvidenceScope::ContinuationReadmissionSourceBasis
    );
    assert_eq!(
        preview_lower_runtime_binding.scope(),
        ForgeQueryEvidenceScope::ContinuationReadmissionLowerRuntimeBinding
    );
    assert_ne!(
        preview_source_basis.as_str(),
        preview_lower_runtime_binding.as_str(),
        "source-basis identity and lower-runtime binding identity must stay typed as distinct roles"
    );
    assert_ne!(current.prepared_digest(), historical.prepared_digest());
    assert_ne!(current.prepared_digest(), preview.prepared_digest());
    assert_ne!(historical.prepared_digest(), preview.prepared_digest());
}
