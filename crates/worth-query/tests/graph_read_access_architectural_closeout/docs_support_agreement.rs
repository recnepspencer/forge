use worth_query::facade::runtime::{
    plan_admitted_graph_read_access_for_family, worth_query_graph_index_inventory,
    WorthQueryGraphReadAccessAdmissionPosture, WorthQueryGraphReadAccessDenialKind,
    WorthQueryGraphReadAccessRequirementKind,
};

use crate::graph_read_access_cost_model_support::{simple_traversal_family, workspace};

const GRAPH_READ_ACCESS_DOC: &str =
    include_str!("../../docs/authoring/graph-read-access-planning.md");
const GRAPH_TOUCH_OBLIGATION_DOC: &str =
    include_str!("../../docs/authoring/graph-touch-obligation-authority.md");
const AI_README: &str = include_str!("../../docs/AI_README.md");

struct CloseoutAgreementRow {
    row_kind: &'static str,
    runtime_label: String,
    docs_present: bool,
    ai_present: bool,
}

#[test]
fn closeout_docs_ai_and_runtime_share_access_vocabulary() {
    for posture in WorthQueryGraphReadAccessAdmissionPosture::ALL {
        assert_doc_and_ai_contain(posture.as_str());
    }
    for denial in WorthQueryGraphReadAccessDenialKind::ALL {
        assert_doc_and_ai_contain(denial.as_str());
    }
    for requirement in WorthQueryGraphReadAccessRequirementKind::all() {
        assert_doc_and_ai_contain(requirement.as_str());
    }
}

#[test]
fn closeout_docs_support_and_receipts_agree_through_structured_rows() {
    let mut rows = Vec::new();
    rows.extend(
        WorthQueryGraphReadAccessAdmissionPosture::ALL
            .iter()
            .map(|posture| agreement_row("admission_posture", posture.as_str())),
    );
    rows.extend(
        WorthQueryGraphReadAccessDenialKind::ALL
            .iter()
            .map(|denial| agreement_row("denial_kind", denial.as_str())),
    );
    rows.extend(
        WorthQueryGraphReadAccessRequirementKind::all()
            .iter()
            .map(|requirement| agreement_row("requirement_kind", requirement.as_str())),
    );

    let inventory = worth_query_graph_index_inventory();
    assert_eq!(
        inventory.rows().len(),
        WorthQueryGraphReadAccessRequirementKind::all().len()
    );
    for support_row in inventory.rows() {
        rows.push(agreement_row(
            "support_inventory_case",
            support_row.requirement_kind().as_str(),
        ));
        assert!(!support_row.digest().is_empty());
    }

    let mut workspace = workspace("closeout-docs-support-case-registry");
    let family = simple_traversal_family(&mut workspace, "closeout-docs-case-registry");
    let plan = plan_admitted_graph_read_access_for_family(&family)
        .expect("case-registry closeout family should plan")
        .expect("case-registry closeout family should admit");
    let case_registry = plan.admission().case_registry();
    assert_eq!(
        case_registry.cases().len(),
        WorthQueryGraphReadAccessRequirementKind::all().len()
    );
    assert!(!case_registry.digest().is_empty());
    for access_case in case_registry.cases() {
        rows.push(agreement_row(
            "access_case_registry",
            access_case.requirement_kind().as_str(),
        ));
        assert_doc_and_ai_contain(access_case.inline_posture().as_str());
        assert_doc_and_ai_contain(access_case.required_capability_owner().as_str());
    }

    for receipt_field in [
        "graph_read_access_plan_consumption",
        "ephemeral_graph_index_receipt",
        "graph_read_streaming_receipt",
        "live_graph_read_access",
        "graph_read_access_summary",
    ] {
        rows.push(agreement_row("receipt_field", receipt_field));
    }

    for row in rows {
        assert!(
            row.docs_present,
            "{} `{}` missing from graph-read access docs",
            row.row_kind, row.runtime_label
        );
        assert!(
            row.ai_present,
            "{} `{}` missing from AI_README",
            row.row_kind, row.runtime_label
        );
    }
}

#[test]
fn closeout_docs_do_not_teach_magic_or_panic_flow() {
    for forbidden in [
        "automatic graph read access planning",
        "automatic indexing",
        "background index provisioning",
        "increase limit and retry",
        ".unwrap()",
    ] {
        assert!(
            !GRAPH_READ_ACCESS_DOC.contains(forbidden),
            "graph read access docs must not teach `{forbidden}`"
        );
        assert!(
            !GRAPH_TOUCH_OBLIGATION_DOC.contains(forbidden),
            "graph touch docs must not teach stale 9.10 scope `{forbidden}`"
        );
        assert!(
            !AI_README.contains(forbidden),
            "AI_README must not teach stale 9.10 scope `{forbidden}`"
        );
    }
    assert!(GRAPH_READ_ACCESS_DOC.contains("execute_read_family_with_access_plan"));
    assert!(GRAPH_READ_ACCESS_DOC.contains("graph_read_access_plan_consumption"));
    assert!(AI_README.contains("## Graph Read Access Planning"));
}

fn assert_doc_and_ai_contain(term: &str) {
    assert!(
        GRAPH_READ_ACCESS_DOC.contains(term),
        "graph read access docs should contain `{term}`"
    );
    assert!(
        AI_README.contains(term),
        "AI_README should contain `{term}`"
    );
}

fn agreement_row(row_kind: &'static str, label: &str) -> CloseoutAgreementRow {
    CloseoutAgreementRow {
        row_kind,
        runtime_label: label.to_string(),
        docs_present: GRAPH_READ_ACCESS_DOC.contains(label),
        ai_present: AI_README.contains(label),
    }
}
