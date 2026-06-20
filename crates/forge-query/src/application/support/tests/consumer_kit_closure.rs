use crate::application::{
    milestone_nine_eight_consumer_kit_closure, ForgeQueryApplicationFacade,
    ForgeQueryConsumerKitClosure, ForgeQueryConsumerKitDocsAgreement,
    ForgeQueryConsumerKitFamilyClosureRow, ForgeQueryConsumerKitReferenceResidue,
    ForgeQueryMilestoneClosureStatus,
};
use crate::consumer_kit::forge_query_consumer_residue_certification_evidence;
use std::collections::BTreeSet;

#[test]
fn support_report_publishes_closed_consumer_kit_closure() {
    let report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let closure = report.consumer_kit_closure();

    assert_eq!(closure.status(), ForgeQueryMilestoneClosureStatus::Closed);
    assert_eq!(
        closure.kit_families().len(),
        ForgeQueryConsumerKitClosure::required_families().len()
    );
    assert!(closure.docs_agree_with_support_profile());
    assert_eq!(
        closure
            .reference_consumer_residue()
            .query_owned_residue_count(),
        0
    );
    assert_eq!(
        closure.hostile_certification().status(),
        ForgeQueryMilestoneClosureStatus::Closed
    );
    assert!(closure
        .hostile_certification()
        .missing_case_ids()
        .is_empty());
    assert!(closure
        .hostile_certification()
        .case_rows()
        .iter()
        .all(|case| case.satisfied() && !case.case_digest().is_empty()));
    assert!(!closure
        .hostile_certification()
        .certification_digest()
        .is_empty());
    assert!(!closure.closure_digest().is_empty());
    assert_eq!(
        closure.closure_identity().as_str(),
        closure.closure_digest()
    );
    assert!(report
        .report_digest()
        .contains("forge.query.evidence-identity.v1"));
}

#[test]
fn consumer_kit_closure_reopens_when_any_required_family_proof_is_missing() {
    for target_family in ForgeQueryConsumerKitClosure::required_families() {
        let sabotaged_rows = milestone_nine_eight_consumer_kit_closure()
            .kit_families()
            .iter()
            .map(|row| {
                if row.family_name() == *target_family {
                    ForgeQueryConsumerKitFamilyClosureRow::new(
                        row.family_name(),
                        ForgeQueryMilestoneClosureStatus::Open,
                        row.evidence_label(),
                        "",
                        [],
                    )
                } else {
                    row.clone()
                }
            })
            .collect::<Vec<_>>();
        let closure = ForgeQueryConsumerKitClosure::derive_from_parts(
            sabotaged_rows,
            ForgeQueryConsumerKitDocsAgreement::current(),
            ForgeQueryConsumerKitReferenceResidue::current(),
            ["durable persisted kit archives remain Milestone 10/11 scope"],
        );

        assert_ne!(
            closure.status(),
            ForgeQueryMilestoneClosureStatus::Closed,
            "missing {:?} must reopen milestone 9.8 closure",
            target_family
        );
    }
}

#[test]
fn consumer_kit_closure_reopens_when_docs_or_reference_residue_disagree() {
    let families = ForgeQueryConsumerKitClosure::required_families().to_vec();
    let docs_missing_ordinary_path =
        ForgeQueryConsumerKitDocsAgreement::new(families.clone(), families.clone(), false);
    let docs_sabotaged = ForgeQueryConsumerKitClosure::derive_from_parts(
        milestone_nine_eight_consumer_kit_closure()
            .kit_families()
            .to_vec(),
        docs_missing_ordinary_path,
        ForgeQueryConsumerKitReferenceResidue::current(),
        ["durable persisted kit archives remain Milestone 10/11 scope"],
    );
    assert_ne!(
        docs_sabotaged.status(),
        ForgeQueryMilestoneClosureStatus::Closed
    );

    let residue_sabotaged = ForgeQueryConsumerKitClosure::derive_from_parts(
        milestone_nine_eight_consumer_kit_closure()
            .kit_families()
            .to_vec(),
        ForgeQueryConsumerKitDocsAgreement::new(families.clone(), families, true),
        ForgeQueryConsumerKitReferenceResidue::new(
            1,
            1,
            "query-owned residue sabotage for Phase 9 certification",
        ),
        ["durable persisted kit archives remain Milestone 10/11 scope"],
    );
    assert_ne!(
        residue_sabotaged.status(),
        ForgeQueryMilestoneClosureStatus::Closed
    );
}

#[test]
fn consumer_kit_dx_target_is_one_support_report_call() {
    let support = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let closure = support.consumer_kit_closure();

    assert_eq!(closure.status(), ForgeQueryMilestoneClosureStatus::Closed);
    assert!(closure.docs_agree_with_support_profile());
    assert_eq!(
        closure
            .reference_consumer_residue()
            .query_owned_residue_count(),
        0
    );
    assert!(closure.kit_families().iter().all(|family| {
        !family.family_name().as_str().is_empty()
            && family.status() == ForgeQueryMilestoneClosureStatus::Closed
            && !family.evidence_digest().is_empty()
            && !family.evidence_source_paths().is_empty()
    }));
    assert!(closure
        .docs_agreement()
        .family_rows()
        .iter()
        .all(|row| row.agrees() && !row.row_digest().is_empty()));
}

#[test]
fn docs_and_test_requirements_publish_phase_nine_consumer_kit_closure() {
    let docs_root = workspace_docs_root();
    let test_requirements = std::fs::read_to_string(docs_root.join("test-requirements.md"))
        .expect("test requirements should be readable");
    let ai_readme =
        std::fs::read_to_string(workspace_root().join("crates/forge-query/docs/AI_README.md"))
            .expect("AI_README should be readable");

    assert!(test_requirements.contains("Milestone 9.8 Phase 9 Required Suite"));
    assert!(test_requirements.contains("Milestone 9.8 Consumer Kit Hostile Certification Matrix"));
    assert!(ai_readme.contains("Consumer Kit"));
    assert!(ai_readme.contains("ordinary downstream path"));

    for family in ForgeQueryConsumerKitClosure::required_families() {
        assert!(
            test_requirements.contains(family.as_str()),
            "test requirements must name consumer kit family {}",
            family.as_str()
        );
        assert!(
            ai_readme.contains(family.as_str()),
            "AI_README must name consumer kit family {}",
            family.as_str()
        );
    }
}

#[test]
fn docs_agreement_uses_the_required_family_set_without_string_drift() {
    let agreement = milestone_nine_eight_consumer_kit_closure()
        .docs_agreement()
        .clone();

    assert_eq!(
        agreement.support_families(),
        ForgeQueryConsumerKitClosure::required_families()
    );
    assert_eq!(
        agreement.documented_families(),
        ForgeQueryConsumerKitClosure::required_families()
    );
    assert!(agreement.ordinary_path_language_present());
    assert!(agreement.agrees());
    assert_eq!(
        agreement.family_rows().len(),
        ForgeQueryConsumerKitClosure::required_families().len()
    );
    for row in agreement.family_rows() {
        assert!(row.ai_readme_present());
        assert!(row.test_requirements_present());
        assert!(row.closeout_present());
        assert!(row.ordinary_path_present());
        assert!(row.family_obligation_present());
        assert!(row.agrees());
    }
    assert!(!agreement.agreement_digest().is_empty());
}

#[test]
fn reference_consumer_residue_publication_names_backend_applicability() {
    let residue = milestone_nine_eight_consumer_kit_closure()
        .reference_consumer_residue()
        .clone();

    assert_eq!(residue.query_owned_residue_count(), 0);
    assert_eq!(residue.defended_residue_count(), 3);
    assert_eq!(residue.breakdown().report_digest_residue_count(), 0);
    assert_eq!(residue.breakdown().prohibition_audit_residue_count(), 0);
    assert_eq!(residue.breakdown().support_pinning_residue_count(), 0);
    assert_eq!(residue.breakdown().test_backend_residue_count(), 0);
    assert_eq!(residue.breakdown().defended_worth_domain_residue_count(), 3);
    assert!(!residue.breakdown().breakdown_digest().is_empty());
    assert!(residue.backend_applicability_certified());
    assert!(residue
        .backend_applicability()
        .contains("zero hand-implemented Query runtime adapters"));
    assert!(!residue.residue_source_digest().is_empty());
    assert!(!residue.residue_digest().is_empty());
}

#[test]
fn family_closure_rows_are_backed_by_real_certification_sources() {
    let closure = milestone_nine_eight_consumer_kit_closure();
    let certified_families = closure
        .hostile_certification()
        .case_rows()
        .iter()
        .map(|case| case.family())
        .collect::<BTreeSet<_>>();

    for row in closure.kit_families() {
        assert_eq!(row.status(), ForgeQueryMilestoneClosureStatus::Closed);
        assert!(
            certified_families.contains(&row.family_name()),
            "{} must have at least one typed hostile certification case",
            row.family_name().as_str()
        );
        assert!(
            row.evidence_source_paths().len() >= 2,
            "{} must cite concrete certification sources",
            row.family_name().as_str()
        );
        assert!(row
            .evidence_source_paths()
            .iter()
            .all(|path| path.starts_with("crates/")));
    }
}

#[test]
fn hostile_certification_names_required_case_coverage() {
    let closure = milestone_nine_eight_consumer_kit_closure();
    let certification = closure.hostile_certification();
    let case_ids = certification
        .case_rows()
        .iter()
        .map(|case| case.case_id())
        .collect::<BTreeSet<_>>();

    assert!(case_ids.contains("evidence-report-compile-fail-boundary"));
    assert!(case_ids.contains("hard-prohibition-compile-fail-boundary"));
    assert!(case_ids.contains("boundary-audit-seeded-bypass-detection"));
    assert!(case_ids.contains("support-snapshot-live-matrix-equivalence"));
    assert!(case_ids.contains("support-pinning-drift-localization"));
    assert!(case_ids.contains("in-memory-test-backend-equivalence"));
    assert!(case_ids.contains("consumer-residue-proof-folklore-authority"));
    assert!(case_ids.contains("consumer-residue-false-positive-honesty"));
    assert!(case_ids.contains("reference-consumer-enforcement-adoption"));
    assert!(certification.missing_case_ids().is_empty());
    assert!(certification.case_rows().iter().all(|case| {
        case.satisfied()
            && !case.requirement().is_empty()
            && !case.required_signal().is_empty()
            && !case.evidence_source_paths().is_empty()
    }));
}

#[test]
fn consumer_residue_certification_cases_are_backed_by_typed_evidence() {
    let evidence = forge_query_consumer_residue_certification_evidence();
    let evidence_case_ids = evidence
        .iter()
        .map(|row| row.case_id())
        .collect::<BTreeSet<_>>();
    let closure = milestone_nine_eight_consumer_kit_closure();

    assert!(evidence_case_ids.contains("consumer-residue-proof-folklore-authority"));
    assert!(evidence_case_ids.contains("consumer-residue-false-positive-honesty"));
    assert!(evidence
        .iter()
        .all(|row| row.satisfied() && !row.case_digest().is_empty()));
    assert!(closure
        .hostile_certification()
        .case_rows()
        .iter()
        .any(|row| {
            row.case_id() == "consumer-residue-proof-folklore-authority"
                && row.required_signal() == "typed-consumer-residue-certification-evidence"
                && row.satisfied()
        }));
    assert!(closure
        .hostile_certification()
        .case_rows()
        .iter()
        .any(|row| {
            row.case_id() == "consumer-residue-false-positive-honesty"
                && row.required_signal() == "typed-consumer-residue-certification-evidence"
                && row.satisfied()
        }));
}

#[test]
fn milestone_9_8_consumer_kit_hostile_certification_matrix_closes_all_required_families() {
    let closure = milestone_nine_eight_consumer_kit_closure();
    let certified_families = closure
        .hostile_certification()
        .case_rows()
        .iter()
        .map(|case| case.family())
        .collect::<BTreeSet<_>>();
    let support_families = closure
        .kit_families()
        .iter()
        .map(|row| row.family_name())
        .collect::<BTreeSet<_>>();

    assert_eq!(closure.status(), ForgeQueryMilestoneClosureStatus::Closed);
    assert_eq!(
        support_families.len(),
        ForgeQueryConsumerKitClosure::required_families().len()
    );
    assert_eq!(support_families, certified_families);
    assert!(closure
        .kit_families()
        .iter()
        .all(|row| row.status() == ForgeQueryMilestoneClosureStatus::Closed));
    assert!(closure
        .hostile_certification()
        .case_rows()
        .iter()
        .any(|case| case.case_id() == "consumer-residue-proof-folklore-authority"));
    assert!(closure
        .hostile_certification()
        .case_rows()
        .iter()
        .any(|case| case.case_id() == "consumer-residue-false-positive-honesty"));
    assert_eq!(
        closure
            .reference_consumer_residue()
            .query_owned_residue_count(),
        0
    );
    assert!(closure.docs_agree_with_support_profile());
}

fn workspace_docs_root() -> std::path::PathBuf {
    workspace_root().join("_docs/forge-query")
}

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("forge-query crate should live under workspace crates directory")
        .to_path_buf()
}
