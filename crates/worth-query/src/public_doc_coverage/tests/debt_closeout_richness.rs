use worth_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalProfileSet, FoundationalProfileSetInput,
    RetentionDeliveryProfile, SupportPostureProfile,
};

use crate::runtime::{
    WorthQueryRuntime, WorthQueryRuntimeDownstreamDeliveryContract, WorthQueryRuntimeFacadeFamily,
    WorthQueryRuntimeFamilySupportStatus, WorthQueryRuntimeFamilyTeachingPosture,
    WorthQueryRuntimePublicApiContract, WorthQueryRuntimePublicSupportMatrix,
    WorthQueryRuntimeSupportProfile,
};

const SUPPORT_MATRIX_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/foundations/support-matrix-and-admission.md"
));
const DOWNSTREAM_RUNTIME_INTEGRATION_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/foundations/downstream-runtime-integration.md"
));

#[test]
fn support_docs_match_runtime_backed_support_gated_and_deferred_truth() {
    let support_profile = WorthQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    );
    let contract = WorthQueryRuntimePublicApiContract::from_support_profile(&support_profile);
    let support_matrix = WorthQueryRuntimePublicSupportMatrix::from_public_api_contract(&contract);
    let downstream_contract =
        WorthQueryRuntimeDownstreamDeliveryContract::from_support_profile(&support_profile);
    let authority_evidence_closeout =
        WorthQueryRuntime::public_authoritative_mutation_evidence_closeout_for_support_profile(
            &support_profile,
        );

    for family in [
        WorthQueryRuntimeFacadeFamily::Temporal,
        WorthQueryRuntimeFacadeFamily::AsyncResource,
        WorthQueryRuntimeFacadeFamily::MixedCauseDelivery,
    ] {
        let contract_row = contract
            .family(family)
            .expect("runtime-backed support-gated family should exist");
        let matrix_row = support_matrix
            .row_for_family(family)
            .expect("support matrix should mirror support-gated family posture");
        support_profile
            .admit(family)
            .expect("runtime-backed support-gated family should admit");

        assert_eq!(
            contract_row.status(),
            WorthQueryRuntimeFamilySupportStatus::Supported
        );
        assert_eq!(
            contract_row.teaching_posture(),
            WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly
        );
        assert!(!contract_row.ordinary_downstream_dx());
        assert!(contract_row.admission_fail_closed());
        assert_eq!(contract_row.owner_closure(), "Milestone 9.4");
        assert_eq!(
            matrix_row.support_contract_digest(),
            Some(contract_row.contract_digest())
        );
    }

    for (family, expected_reason) in [
        (
            WorthQueryRuntimeFacadeFamily::StoreBackedExecution,
            "Milestone 10",
        ),
        (
            WorthQueryRuntimeFacadeFamily::DurableArtifacts,
            "Milestone 11",
        ),
    ] {
        let contract_row = contract
            .family(family)
            .expect("deferred family should stay visible in the contract");
        let matrix_row = support_matrix
            .row_for_family(family)
            .expect("support matrix should mirror deferred family posture");
        let denial = support_profile
            .admit(family)
            .expect_err("deferred family should deny before use");
        assert_eq!(denial.family(), family);
        assert!(denial.reason().contains(expected_reason));

        assert_eq!(
            contract_row.status(),
            WorthQueryRuntimeFamilySupportStatus::DeferredDebt
        );
        assert_eq!(
            contract_row.teaching_posture(),
            WorthQueryRuntimeFamilyTeachingPosture::VisibleButDeferred
        );
        assert!(!contract_row.ordinary_downstream_dx());
        assert!(contract_row.admission_fail_closed());
        assert_eq!(
            matrix_row.status(),
            WorthQueryRuntimeFamilySupportStatus::DeferredDebt
        );
    }

    assert!(downstream_contract.runtime_backed_resume_supported());
    assert!(downstream_contract.durable_resume_deferred());
    assert!(authority_evidence_closeout
        .must_not_assume_yet()
        .iter()
        .any(|line: &String| line.contains("durable restart")));
    assert!(authority_evidence_closeout
        .must_not_assume_yet()
        .iter()
        .any(|line: &String| line.contains("typed and fail-closed")));

    let downstream_matrix_row = support_matrix
        .row("downstream-delivery-contract")
        .expect("downstream delivery contract row must stay explicit");
    assert_eq!(
        downstream_matrix_row.support_contract_digest(),
        Some(downstream_contract.contract_identity().as_str())
    );

    let authority_matrix_row = support_matrix
        .row("authoritative-mutation-evidence-certification")
        .expect("authority evidence certification row must stay explicit");
    assert_eq!(
        authority_matrix_row.support_contract_digest(),
        Some(authority_evidence_closeout.query_support_digest())
    );

    let scaffold_contract = WorthQueryRuntimePublicApiContract::from_support_profile(
        &WorthQueryRuntimeSupportProfile::scaffold_backend_profile(),
    );
    assert_eq!(
        scaffold_contract
            .family(WorthQueryRuntimeFacadeFamily::Temporal)
            .expect("scaffold temporal family should exist")
            .status(),
        WorthQueryRuntimeFamilySupportStatus::Supported
    );

    assert!(SUPPORT_MATRIX_DOC.contains(
        "- `support-gate-only` means the row is shipped and machine-checkable, but it is"
    ));
    assert!(SUPPORT_MATRIX_DOC.contains(
        "- `visible-but-deferred` means the family name is published now, but admission"
    ));
    assert!(SUPPORT_MATRIX_DOC
        .contains("- `visible-vocabulary-only` means the public vocabulary exists, but normal"));
    assert!(DOWNSTREAM_RUNTIME_INTEGRATION_DOC
        .contains("- runtime-backed resume is supported now only when the basis digest matches"));
    assert!(DOWNSTREAM_RUNTIME_INTEGRATION_DOC.contains(
        "- durable replay/restart resume is still deferred debt and stays typed as debt"
    ));
}

#[test]
fn diagnostic_richness_profiles_produce_distinct_closeout_reporting_identities() {
    let profiles = [
        profile_with_richness(DiagnosticRichnessProfile::OperationalMinimal),
        profile_with_richness(DiagnosticRichnessProfile::Standard),
        profile_with_richness(DiagnosticRichnessProfile::Forensic),
    ];

    assert_eq!(
        profiles[0].diagnostic_richness(),
        DiagnosticRichnessProfile::OperationalMinimal
    );
    assert_eq!(
        profiles[1].diagnostic_richness(),
        DiagnosticRichnessProfile::Standard
    );
    assert_eq!(
        profiles[2].diagnostic_richness(),
        DiagnosticRichnessProfile::Forensic
    );
    assert_ne!(profiles[0], profiles[1]);
    assert_ne!(profiles[1], profiles[2]);
    assert_ne!(profiles[0], profiles[2]);
}

fn profile_with_richness(richness: DiagnosticRichnessProfile) -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: richness,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::Uncertified,
    })
    .expect("richness-only profile variation should stay legal")
}
