use super::{
    admit_query_graph_read_admission_posture_label, admit_query_graph_read_cost_counter_label,
    admit_query_graph_read_denial_kind_label, admit_query_graph_read_receipt_field_label,
    admit_query_graph_read_requirement_label, current_query_graph_read_access_capabilities,
    reject_graph_touch_obligation_vocabulary_as_graph_read_access,
    reject_worth_local_graph_read_access_label, QueryGraphReadAccessCapabilityAuthority,
    QueryGraphReadAccessCapabilityKind, QueryGraphReadCostCounterField, QueryGraphReadReceiptField,
    WorthLocalGraphReadAccessVocabularyDenialKind,
};
use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
    ForgeQueryGraphReadAccessRequirementKind,
};

#[test]
fn graph_read_capability_refresh_tracks_query_runtime_facade() {
    let capabilities = current_query_graph_read_access_capabilities();

    for label in [
        "derive_graph_read_access_requirements",
        "try_derive_graph_read_access_requirements",
        "admit_graph_read_access_for_family",
        "plan_admitted_graph_read_access_for_family",
        "ForgeQueryAdmittedGraphReadAccessPlan",
        "ForgeQueryGraphReadAccessRequirementRow",
        "ForgeQueryGraphReadAccessPlanConsumption",
        "ForgeQueryGraphReadAccessComplexityCounters",
    ] {
        assert!(
            capabilities.contains_query_label(label),
            "missing Query graph-read capability label: {label}"
        );
    }

    for kind in [
        QueryGraphReadAccessCapabilityKind::Function,
        QueryGraphReadAccessCapabilityKind::Type,
        QueryGraphReadAccessCapabilityKind::AdmissionPosture,
        QueryGraphReadAccessCapabilityKind::DenialKind,
        QueryGraphReadAccessCapabilityKind::RequirementKind,
        QueryGraphReadAccessCapabilityKind::ReceiptField,
        QueryGraphReadAccessCapabilityKind::CostCounter,
        QueryGraphReadAccessCapabilityKind::CapabilityGapPressure,
    ] {
        assert!(capabilities.contains_kind(kind), "missing kind: {kind:?}");
    }
}

#[test]
fn admission_posture_capabilities_match_query_all_exactly() {
    let capabilities = current_query_graph_read_access_capabilities();

    assert_eq!(
        capabilities.labels_for_kind(QueryGraphReadAccessCapabilityKind::AdmissionPosture),
        ForgeQueryGraphReadAccessAdmissionPosture::ALL
            .iter()
            .map(ForgeQueryGraphReadAccessAdmissionPosture::as_str)
            .collect::<Vec<_>>()
    );
}

#[test]
fn denial_kind_capabilities_match_query_all_exactly() {
    let capabilities = current_query_graph_read_access_capabilities();

    assert_eq!(
        capabilities.labels_for_kind(QueryGraphReadAccessCapabilityKind::DenialKind),
        ForgeQueryGraphReadAccessDenialKind::ALL
            .iter()
            .map(ForgeQueryGraphReadAccessDenialKind::as_str)
            .collect::<Vec<_>>()
    );
}

#[test]
fn requirement_kind_capabilities_match_query_all_exactly() {
    let capabilities = current_query_graph_read_access_capabilities();

    assert_eq!(
        capabilities.labels_for_kind(QueryGraphReadAccessCapabilityKind::RequirementKind),
        ForgeQueryGraphReadAccessRequirementKind::all()
            .iter()
            .map(ForgeQueryGraphReadAccessRequirementKind::as_str)
            .collect::<Vec<_>>()
    );
}

#[test]
fn receipt_field_capabilities_match_declared_receipt_fields_exactly() {
    let capabilities = current_query_graph_read_access_capabilities();

    assert_eq!(
        capabilities.labels_for_kind(QueryGraphReadAccessCapabilityKind::ReceiptField),
        QueryGraphReadReceiptField::ALL
            .iter()
            .map(QueryGraphReadReceiptField::query_label)
            .collect::<Vec<_>>()
    );
}

#[test]
fn cost_counter_capabilities_match_declared_counter_fields_exactly() {
    let capabilities = current_query_graph_read_access_capabilities();

    assert_eq!(
        capabilities.labels_for_kind(QueryGraphReadAccessCapabilityKind::CostCounter),
        QueryGraphReadCostCounterField::ALL
            .iter()
            .map(QueryGraphReadCostCounterField::query_label)
            .collect::<Vec<_>>()
    );
}

#[test]
fn capability_catalog_contains_no_duplicate_kind_label_pairs() {
    let capabilities = current_query_graph_read_access_capabilities();

    assert!(!capabilities.has_duplicate_kind_label_pairs());
}

#[test]
fn worth_graph_read_inventory_rejects_local_access_vocabulary() {
    for label in local_graph_read_folklore_labels() {
        let denial = reject_worth_local_graph_read_access_label(label)
            .expect_err("Worth-local graph-read folklore must be rejected");
        assert_eq!(denial.rejected_label(), label);
        assert_eq!(
            denial.kind(),
            WorthLocalGraphReadAccessVocabularyDenialKind::UnknownQueryGraphReadAccessLabel
        );
        assert!(denial.requires_query_owned_vocabulary());
    }

    let admission = admit_query_graph_read_admission_posture_label("persistent_index_required")
        .expect("Query admission posture should be admitted");
    assert_eq!(admission.label(), "persistent_index_required");
    assert_eq!(
        admission.kind(),
        QueryGraphReadAccessCapabilityKind::AdmissionPosture
    );
}

#[test]
fn graph_touch_obligation_outputs_cannot_satisfy_read_access_capability() {
    for obligation_label in graph_touch_obligation_labels() {
        let denial =
            reject_graph_touch_obligation_vocabulary_as_graph_read_access(obligation_label)
                .expect_err("graph-touch obligation vocabulary is the wrong authority family");
        assert_eq!(denial.rejected_label(), obligation_label);
        assert_eq!(
            denial.kind(),
            WorthLocalGraphReadAccessVocabularyDenialKind::WrongAuthorityFamily
        );
    }
}

#[test]
fn type_and_function_labels_cannot_be_used_as_inventory_access_vocabulary() {
    for label in [
        "ForgeQueryAdmittedGraphReadAccessPlan",
        "derive_graph_read_access_requirements",
    ] {
        let denial = admit_query_graph_read_requirement_label(label)
            .expect_err("type/function labels are not inventory access vocabulary");
        assert_eq!(denial.rejected_label(), label);
        assert!(matches!(
            denial.kind(),
            WorthLocalGraphReadAccessVocabularyDenialKind::WrongCapabilityKind { .. }
        ));
    }
}

#[test]
fn category_specific_admission_rejects_cross_category_labels() {
    assert!(admit_query_graph_read_requirement_label("directional_adjacency").is_ok());
    assert!(admit_query_graph_read_admission_posture_label("persistent_index_required").is_ok());
    assert!(admit_query_graph_read_denial_kind_label("required_persistent_index").is_ok());
    assert!(
        admit_query_graph_read_receipt_field_label("graph_read_access_plan_consumption").is_ok()
    );
    assert!(
        admit_query_graph_read_cost_counter_label("access_execution_counters.edge_scan_count")
            .is_ok()
    );

    let denial = admit_query_graph_read_requirement_label("persistent_index_required")
        .expect_err("admission posture must not masquerade as requirement vocabulary");
    assert!(matches!(
        denial.kind(),
        WorthLocalGraphReadAccessVocabularyDenialKind::WrongCapabilityKind {
            expected: QueryGraphReadAccessCapabilityKind::RequirementKind,
            actual: QueryGraphReadAccessCapabilityKind::AdmissionPosture
        }
    ));
}

#[test]
fn phase_one_capability_rows_are_vocabulary_only() {
    let capabilities = current_query_graph_read_access_capabilities();

    assert!(!capabilities.claims_execution_authority());
    assert!(capabilities
        .rows()
        .iter()
        .all(|row| row.authority() == QueryGraphReadAccessCapabilityAuthority::VocabularyOnly));
}

fn local_graph_read_folklore_labels() -> [&'static str; 7] {
    [
        "safe-neighborhood",
        "manual-no-n-plus-one",
        "local-adjacency-cache",
        "helper-proof",
        "fabricated-receipt",
        "local-support-row",
        "new-worth-local-graph-read-shortcut",
    ]
}

fn graph_touch_obligation_labels() -> [&'static str; 5] {
    [
        "ForgeQueryGraphObligationSelection",
        "ForgeQueryGraphObligationSupportMatrixRow",
        "ForgeQueryGraphTouchDescriptor",
        "selected_query_graph_obligation",
        "graph_touch_obligation_adoption_proof",
    ]
}
