use crate::runtime::{
    ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationExecutionBudget,
    ForgeQueryGraphObligationExecutionScope, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportStatus,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector,
};
use crate::{
    graph_obligation_consumer_kit, ForgeQueryGraphObligationConsumerKitErrorKind,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationSelectorCoverageDeclaration, ForgeQueryGraphObligationSupportPin,
};

use super::adoption_fixture::{
    adoption_attempt_with_pin, evaluated_clean_audit, reference_registration,
    support_matrix_with_status,
};

#[test]
fn consumer_kit_builds_manifest_from_query_surfaces_only() {
    let registration = reference_registration();
    let touch = ForgeQueryGraphTouchDescriptor::read_family(
        "worth_faces",
        [ForgeQueryGraphTouchReadVerb::ObservesCollection],
    )
    .unwrap();
    let proof = graph_obligation_consumer_kit("worth-kernel")
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-kernel-validity",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
                "active face read coverage",
                ForgeQueryGraphTouchSelector::collection("worth_faces").unwrap(),
            )]),
        )
        .pin_support(ForgeQueryGraphObligationSupportPin::supported_with_budget(
            [(
                ForgeQueryGraphObligationKind::BlockingInvariant,
                ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
                ForgeQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
            )],
        ))
        .audit_local_ceremony(evaluated_clean_audit("worth-kernel"))
        .account_for_residue(ForgeQueryGraphObligationResidueManifest::empty())
        .prove_in_memory_selection(
            &touch,
            &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )
        .unwrap()
        .prove_adoption()
        .unwrap();

    assert_eq!(proof.manifest().consumer_name(), "worth-kernel");
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
        ForgeQueryGraphObligationSupportStatus::DiagnosticOnly,
        ForgeQueryGraphObligationSupportStatus::Unsupported,
        ForgeQueryGraphObligationSupportStatus::NotApplicable,
        ForgeQueryGraphObligationSupportStatus::DeferredToBackstop,
    ] {
        let stale_error = adoption_attempt_with_pin(
            ForgeQueryGraphObligationSupportPin::supported([(
                ForgeQueryGraphObligationKind::BlockingInvariant,
                ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
            )]),
            support_matrix_with_status(drifted_status),
        )
        .unwrap_err();

        assert_eq!(
            stale_error.kind(),
            ForgeQueryGraphObligationConsumerKitErrorKind::SupportPinDrift
        );

        let updated_proof = adoption_attempt_with_pin(
            ForgeQueryGraphObligationSupportPin::new([(
                ForgeQueryGraphObligationKind::BlockingInvariant,
                ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
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
        ForgeQueryGraphObligationExecutionBudget::selection_only_deferred_execution();
    let observed_budget = ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
        ForgeQueryGraphObligationExecutionScope::TouchedCollection,
        ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed,
    );
    let registration = reference_registration().with_execution_budget(observed_budget.clone());
    let pin = ForgeQueryGraphObligationSupportPin::supported_with_budget([(
        ForgeQueryGraphObligationKind::BlockingInvariant,
        ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
        expected_budget,
    )]);
    let matrix = ForgeQueryGraphObligationSupportMatrix::assembly_selection_foundation();

    let error = pin
        .evaluate_for_registrations(&matrix, std::slice::from_ref(&registration))
        .unwrap_err();
    let findings = pin.findings_for_registrations(&matrix, &[registration]);

    assert_eq!(
        error.kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::SupportPinDrift
    );
    assert_eq!(
        findings[0].observed_budget_digest(),
        Some(observed_budget.budget_digest())
    );
}

#[test]
fn adoption_rejects_selector_coverage_that_does_not_cover_registration() {
    let proof_attempt = base_adoption_with_coverage(
        ForgeQueryGraphTouchSelector::collection("other_collection").unwrap(),
        evaluated_clean_audit("worth-kernel"),
        "worth_faces",
    )
    .unwrap_err();

    assert_eq!(
        proof_attempt.kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::SelectorCoverageMismatch
    );
}

#[test]
fn adoption_rejects_empty_in_memory_proof() {
    let proof_attempt = base_adoption_with_coverage(
        ForgeQueryGraphTouchSelector::collection("worth_faces").unwrap(),
        evaluated_clean_audit("worth-kernel"),
        "other_collection",
    )
    .unwrap_err();

    assert_eq!(
        proof_attempt.kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::EmptyInMemoryProof
    );
}

#[test]
fn adoption_rejects_synthetic_clean_audit() {
    let proof_attempt = base_adoption_with_coverage(
        ForgeQueryGraphTouchSelector::collection("worth_faces").unwrap(),
        ForgeQueryGraphObligationLocalCeremonyAudit::clean(),
        "worth_faces",
    )
    .unwrap_err();

    assert_eq!(
        proof_attempt.kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::UnevaluatedLocalCeremonyAudit
    );
}

fn base_adoption_with_coverage(
    coverage: ForgeQueryGraphTouchSelector,
    audit: ForgeQueryGraphObligationLocalCeremonyAudit,
    touched_collection: &str,
) -> Result<
    crate::ForgeQueryGraphObligationAdoptionProof,
    crate::ForgeQueryGraphObligationConsumerKitError,
> {
    graph_obligation_consumer_kit("worth-kernel")
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-kernel-validity",
                [reference_registration()],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
                "active face read coverage",
                coverage,
            )]),
        )
        .pin_support(ForgeQueryGraphObligationSupportPin::supported([(
            ForgeQueryGraphObligationKind::BlockingInvariant,
            ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
        )]))
        .audit_local_ceremony(audit)
        .prove_in_memory_selection(
            &ForgeQueryGraphTouchDescriptor::read_family(
                touched_collection,
                [ForgeQueryGraphTouchReadVerb::ObservesCollection],
            )
            .unwrap(),
            &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )?
        .prove_adoption()
}
