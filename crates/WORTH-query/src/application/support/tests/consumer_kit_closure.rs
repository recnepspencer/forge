use crate::application::{
    milestone_nine_eight_consumer_kit_closure, WorthQueryApplicationFacade,
    WorthQueryConsumerKitClosure, WorthQueryConsumerKitDocsAgreement,
    WorthQueryConsumerKitFamilyClosureRow, WorthQueryConsumerKitReferenceResidue,
    WorthQueryMilestoneClosureStatus,
};
use crate::consumer_kit::worth_query_consumer_residue_certification_evidence;
use std::collections::BTreeSet;

#[test]
fn support_report_publishes_closed_consumer_kit_closure() {
    let report = WorthQueryApplicationFacade::runtime_backed_default().support_report();
    let closure = report.consumer_kit_closure();

    assert_eq!(closure.status(), WorthQueryMilestoneClosureStatus::Closed);
    assert_eq!(
        closure.kit_families().len(),
        WorthQueryConsumerKitClosure::required_families().len()
    );
    assert!(closure.docs_agree_with_support_profile());
    assert_eq!(
        closure.hostile_certification().status(),
        WorthQueryMilestoneClosureStatus::Closed
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
        .contains("WORTH.query.evidence-identity.v1"));
}

#[test]
fn consumer_kit_closure_reopens_when_any_required_family_proof_is_missing() {
    for target_family in WorthQueryConsumerKitClosure::required_families() {
        let sabotaged_rows = milestone_nine_eight_consumer_kit_closure()
            .kit_families()
            .iter()
            .map(|row| {
                if row.family_name() == *target_family {
                    WorthQueryConsumerKitFamilyClosureRow::new(
                        row.family_name(),
                        WorthQueryMilestoneClosureStatus::Open,
                        row.evidence_label(),
                        "",
                        [],
                    )
                } else {
                    row.clone()
                }
            })
            .collect::<Vec<_>>();
        let closure = WorthQueryConsumerKitClosure::derive_from_parts(
            sabotaged_rows,
            WorthQueryConsumerKitDocsAgreement::current(),
            WorthQueryConsumerKitReferenceResidue::current(),
            ["durable persisted kit archives remain Milestone 10/11 scope"],
        );

        assert_ne!(
            closure.status(),
            WorthQueryMilestoneClosureStatus::Closed,
            "missing {:?} must reopen milestone 9.8 closure",
            target_family
        );
    }
}

#[test]
fn consumer_kit_closure_reopens_when_docs_or_reference_residue_disagree() {
    let families = WorthQueryConsumerKitClosure::required_families().to_vec();
    let docs_missing_ordinary_path =
        WorthQueryConsumerKitDocsAgreement::new(families.clone(), families.clone(), false);
    let docs_sabotaged = WorthQueryConsumerKitClosure::derive_from_parts(
        milestone_nine_eight_consumer_kit_closure()
            .kit_families()
            .to_vec(),
        docs_missing_ordinary_path,
        WorthQueryConsumerKitReferenceResidue::current(),
        ["durable persisted kit archives remain Milestone 10/11 scope"],
    );
    assert_ne!(
        docs_sabotaged.status(),
        WorthQueryMilestoneClosureStatus::Closed
    );

    let residue_sabotaged = WorthQueryConsumerKitClosure::derive_from_parts(
        milestone_nine_eight_consumer_kit_closure()
            .kit_families()
            .to_vec(),
        WorthQueryConsumerKitDocsAgreement::new(families.clone(), families, true),
        WorthQueryConsumerKitReferenceResidue::new(
            1,
            1,
            "query-owned residue sabotage for Phase 9 certification",
        ),
        ["durable persisted kit archives remain Milestone 10/11 scope"],
    );
    assert_ne!(
        residue_sabotaged.status(),
        WorthQueryMilestoneClosureStatus::Closed
    );
}

#[test]
fn consumer_kit_dx_target_is_one_support_report_call() {
    let support = WorthQueryApplicationFacade::runtime_backed_default().support_report();
    let closure = support.consumer_kit_closure();

    assert_eq!(closure.status(), WorthQueryMilestoneClosureStatus::Closed);
    assert!(closure.docs_agree_with_support_profile());
    assert!(closure.kit_families().iter().all(|family| {
        !family.family_name().as_str().is_empty()
            && family.status() == WorthQueryMilestoneClosureStatus::Closed
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
        std::fs::read_to_string(workspace_root().join("crates/worth-query/docs/AI_README.md"))
            .expect("AI_README should be readable");

    assert!(test_requirements.contains("Milestone 9.8 Phase 9 Required Suite"));
    assert!(test_requirements.contains("Milestone 9.8 Consumer Kit Hostile Certification Matrix"));
    assert!(ai_readme.contains("Consumer Kit"));
    assert!(ai_readme.contains("ordinary downstream path"));

    for family in WorthQueryConsumerKitClosure::required_families() {
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
        WorthQueryConsumerKitClosure::required_families()
    );
    assert_eq!(
        agreement.documented_families(),
        WorthQueryConsumerKitClosure::required_families()
    );
    assert!(agreement.ordinary_path_language_present());
    assert!(agreement.agrees());
    assert_eq!(
        agreement.family_rows().len(),
        WorthQueryConsumerKitClosure::required_families().len()
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
fn family_closure_rows_are_backed_by_real_certification_sources() {
    let closure = milestone_nine_eight_consumer_kit_closure();
    let certified_families = closure
        .hostile_certification()
        .case_rows()
        .iter()
        .map(|case| case.family())
        .collect::<BTreeSet<_>>();

    for row in closure.kit_families() {
        assert_eq!(row.status(), WorthQueryMilestoneClosureStatus::Closed);
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
    let evidence = worth_query_consumer_residue_certification_evidence();
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

    assert_eq!(closure.status(), WorthQueryMilestoneClosureStatus::Closed);
    assert_eq!(
        support_families.len(),
        WorthQueryConsumerKitClosure::required_families().len()
    );
    assert_eq!(support_families, certified_families);
    assert!(closure
        .kit_families()
        .iter()
        .all(|row| row.status() == WorthQueryMilestoneClosureStatus::Closed));
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
    assert!(closure.docs_agree_with_support_profile());
}

fn workspace_docs_root() -> std::path::PathBuf {
    workspace_root().join("_docs/worth-query")
}

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-query crate should live under workspace crates directory")
        .to_path_buf()
}
