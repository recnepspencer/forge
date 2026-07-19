use crate::runtime::{
    WorthQueryGraphObligationBudgetExceededPolicy, WorthQueryGraphObligationExecutionBudget,
    WorthQueryGraphObligationExecutionScope, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationOperatingWorldDescriptor, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportMatrix, WorthQueryGraphObligationSupportStatus,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchReadVerb, WorthQueryGraphTouchSelector,
};
use crate::{
    graph_obligation_consumer_kit, WorthQueryGraphObligationAdoptionProof,
    WorthQueryGraphObligationConsumerKitErrorKind,
    WorthQueryGraphObligationConsumerRegistrationDeclaration,
    WorthQueryGraphObligationLocalCeremonyAudit, WorthQueryGraphObligationResidueManifest,
    WorthQueryGraphObligationSelectorCoverageDeclaration, WorthQueryGraphObligationSupportPin,
};

use super::adoption_fixture::{
    adoption_attempt_with_pin, evaluated_clean_audit, reference_registration,
    support_matrix_with_status,
};

#[test]
fn consumer_kit_builds_manifest_from_query_surfaces_only() {
    let registration = reference_registration();
    let touch = WorthQueryGraphTouchDescriptor::read_family(
        "worth_faces",
        [WorthQueryGraphTouchReadVerb::ObservesCollection],
    )
    .unwrap();
    let proof: WorthQueryGraphObligationAdoptionProof =
        graph_obligation_consumer_kit("worth-kernel")
            .register_obligations(
                WorthQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                    "worth-kernel-validity",
                    [registration],
                )
                .unwrap(),
            )
            .declare_selector_coverage(
                WorthQueryGraphObligationSelectorCoverageDeclaration::required([(
                    "active face read coverage",
                    WorthQueryGraphTouchSelector::collection("worth_faces").unwrap(),
                )]),
            )
            .pin_support(WorthQueryGraphObligationSupportPin::supported_with_budget(
                [(
                    WorthQueryGraphObligationKind::BlockingInvariant,
                    WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
                    WorthQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
                )],
            ))
            .audit_local_ceremony(evaluated_clean_audit("worth-kernel"))
            .account_for_residue(WorthQueryGraphObligationResidueManifest::empty())
            .prove_in_memory_selection(
                &touch,
                &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
            )
            .unwrap()
            .prove_adoption()
            .unwrap();

    assert_eq!(proof.manifest().consumer_name(), "worth-kernel");
    let _manifest = proof.manifest();
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 1);
    assert_eq!(proof.in_memory_proof().selected_obligations().len(), 1);
    assert_eq!(proof.support_pin().row_count(), 1);
    assert!(proof.local_ceremony_audit().is_clean());
    assert_eq!(proof.local_ceremony_audit().evaluated_source_count(), 1);
    assert!(proof
        .manifest()
        .manifest_digest()
        .contains("adoption-manifest"));
}

#[test]
fn support_pin_drift_breaks_adoption_until_manifest_is_updated() {
    for drifted_status in [
        WorthQueryGraphObligationSupportStatus::DiagnosticOnly,
        WorthQueryGraphObligationSupportStatus::Unsupported,
        WorthQueryGraphObligationSupportStatus::NotApplicable,
        WorthQueryGraphObligationSupportStatus::DeferredToBackstop,
    ] {
        let stale_error = adoption_attempt_with_pin(
            WorthQueryGraphObligationSupportPin::supported([(
                WorthQueryGraphObligationKind::BlockingInvariant,
                WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
            )]),
            support_matrix_with_status(drifted_status),
        )
        .unwrap_err();

        assert_eq!(
            stale_error.kind(),
            WorthQueryGraphObligationConsumerKitErrorKind::SupportPinDrift
        );

        let updated_proof = adoption_attempt_with_pin(
            WorthQueryGraphObligationSupportPin::new([(
                WorthQueryGraphObligationKind::BlockingInvariant,
                WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
                drifted_status,
            )]),
            support_matrix_with_status(drifted_status),
        )
        .unwrap();
        assert!(updated_proof
            .support_pin()
            .findings(&support_matrix_with_status(drifted_status))
            .is_empty());
    }
}

#[test]
fn support_pin_budget_drift_breaks_registration_adoption() {
    let expected_budget =
        WorthQueryGraphObligationExecutionBudget::selection_only_deferred_execution();
    let observed_budget = WorthQueryGraphObligationExecutionBudget::bounded_sparse(
        WorthQueryGraphObligationExecutionScope::TouchedCollection,
        WorthQueryGraphObligationBudgetExceededPolicy::FailClosed,
    );
    let registration = reference_registration().with_execution_budget(observed_budget.clone());
    let pin = WorthQueryGraphObligationSupportPin::supported_with_budget([(
        WorthQueryGraphObligationKind::BlockingInvariant,
        WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
        expected_budget,
    )]);
    let matrix = WorthQueryGraphObligationSupportMatrix::assembly_selection_foundation();

    let error = pin
        .evaluate_for_registrations(&matrix, std::slice::from_ref(&registration))
        .unwrap_err();
    let findings = pin.findings_for_registrations(&matrix, &[registration]);

    assert_eq!(
        error.kind(),
        WorthQueryGraphObligationConsumerKitErrorKind::SupportPinDrift
    );
    assert_eq!(
        findings[0].observed_budget_digest(),
        Some(observed_budget.budget_digest())
    );
}

#[test]
fn adoption_rejects_selector_coverage_that_does_not_cover_registration() {
    let proof_attempt = base_adoption_with_coverage(
        WorthQueryGraphTouchSelector::collection("other_collection").unwrap(),
        evaluated_clean_audit("worth-kernel"),
        "worth_faces",
    )
    .unwrap_err();

    assert_eq!(
        proof_attempt.kind(),
        WorthQueryGraphObligationConsumerKitErrorKind::SelectorCoverageMismatch
    );
}

#[test]
fn adoption_rejects_empty_in_memory_proof() {
    let proof_attempt = base_adoption_with_coverage(
        WorthQueryGraphTouchSelector::collection("worth_faces").unwrap(),
        evaluated_clean_audit("worth-kernel"),
        "other_collection",
    )
    .unwrap_err();

    assert_eq!(
        proof_attempt.kind(),
        WorthQueryGraphObligationConsumerKitErrorKind::EmptyInMemoryProof
    );
}

#[test]
fn adoption_rejects_synthetic_clean_audit() {
    let proof_attempt = base_adoption_with_coverage(
        WorthQueryGraphTouchSelector::collection("worth_faces").unwrap(),
        WorthQueryGraphObligationLocalCeremonyAudit::clean(),
        "worth_faces",
    )
    .unwrap_err();

    assert_eq!(
        proof_attempt.kind(),
        WorthQueryGraphObligationConsumerKitErrorKind::UnevaluatedLocalCeremonyAudit
    );
}

fn base_adoption_with_coverage(
    coverage: WorthQueryGraphTouchSelector,
    audit: WorthQueryGraphObligationLocalCeremonyAudit,
    touched_collection: &str,
) -> Result<
    crate::WorthQueryGraphObligationAdoptionProof,
    crate::WorthQueryGraphObligationConsumerKitError,
> {
    graph_obligation_consumer_kit("worth-kernel")
        .register_obligations(
            WorthQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-kernel-validity",
                [reference_registration()],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            WorthQueryGraphObligationSelectorCoverageDeclaration::required([(
                "active face read coverage",
                coverage,
            )]),
        )
        .pin_support(WorthQueryGraphObligationSupportPin::supported([(
            WorthQueryGraphObligationKind::BlockingInvariant,
            WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
        )]))
        .audit_local_ceremony(audit)
        .prove_in_memory_selection(
            &WorthQueryGraphTouchDescriptor::read_family(
                touched_collection,
                [WorthQueryGraphTouchReadVerb::ObservesCollection],
            )
            .unwrap(),
            &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )?
        .prove_adoption()
}
