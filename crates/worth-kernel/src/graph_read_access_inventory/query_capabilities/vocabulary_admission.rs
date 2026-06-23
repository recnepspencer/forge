use super::capability_row::QueryGraphReadAccessCapabilityKind;
use super::capability_snapshot::current_query_graph_read_access_capabilities;
use super::local_vocabulary_denial::{
    QueryGraphReadAccessLabelAdmission, WorthLocalGraphReadAccessVocabularyDenial,
};

pub fn reject_worth_local_graph_read_access_label(
    label: &str,
) -> Result<(), WorthLocalGraphReadAccessVocabularyDenial> {
    for admissible_kind in INVENTORY_ACCESS_VOCABULARY_KINDS {
        if admit_query_graph_read_access_label_for_kind(label, admissible_kind).is_ok() {
            return Ok(());
        }
    }

    Err(WorthLocalGraphReadAccessVocabularyDenial::unknown(label))
}

pub fn admit_query_graph_read_requirement_label(
    label: &str,
) -> Result<QueryGraphReadAccessLabelAdmission, WorthLocalGraphReadAccessVocabularyDenial> {
    admit_query_graph_read_access_label_for_kind(
        label,
        QueryGraphReadAccessCapabilityKind::RequirementKind,
    )
}

pub fn admit_query_graph_read_admission_posture_label(
    label: &str,
) -> Result<QueryGraphReadAccessLabelAdmission, WorthLocalGraphReadAccessVocabularyDenial> {
    admit_query_graph_read_access_label_for_kind(
        label,
        QueryGraphReadAccessCapabilityKind::AdmissionPosture,
    )
}

pub fn admit_query_graph_read_denial_kind_label(
    label: &str,
) -> Result<QueryGraphReadAccessLabelAdmission, WorthLocalGraphReadAccessVocabularyDenial> {
    admit_query_graph_read_access_label_for_kind(
        label,
        QueryGraphReadAccessCapabilityKind::DenialKind,
    )
}

pub fn admit_query_graph_read_receipt_field_label(
    label: &str,
) -> Result<QueryGraphReadAccessLabelAdmission, WorthLocalGraphReadAccessVocabularyDenial> {
    admit_query_graph_read_access_label_for_kind(
        label,
        QueryGraphReadAccessCapabilityKind::ReceiptField,
    )
}

pub fn admit_query_graph_read_cost_counter_label(
    label: &str,
) -> Result<QueryGraphReadAccessLabelAdmission, WorthLocalGraphReadAccessVocabularyDenial> {
    admit_query_graph_read_access_label_for_kind(
        label,
        QueryGraphReadAccessCapabilityKind::CostCounter,
    )
}

pub fn reject_graph_touch_obligation_vocabulary_as_graph_read_access(
    label: &str,
) -> Result<(), WorthLocalGraphReadAccessVocabularyDenial> {
    if GRAPH_TOUCH_OBLIGATION_VOCABULARY.contains(&label) {
        return Err(WorthLocalGraphReadAccessVocabularyDenial::wrong_authority_family(label));
    }

    reject_worth_local_graph_read_access_label(label)
}

fn admit_query_graph_read_access_label_for_kind(
    label: &str,
    expected: QueryGraphReadAccessCapabilityKind,
) -> Result<QueryGraphReadAccessLabelAdmission, WorthLocalGraphReadAccessVocabularyDenial> {
    let capabilities = current_query_graph_read_access_capabilities();
    let Some(row) = capabilities
        .rows()
        .iter()
        .find(|row| row.query_label() == label)
    else {
        return Err(WorthLocalGraphReadAccessVocabularyDenial::unknown(label));
    };

    if row.kind() != expected {
        return Err(
            WorthLocalGraphReadAccessVocabularyDenial::wrong_capability_kind(
                label,
                expected,
                row.kind(),
            ),
        );
    }

    Ok(QueryGraphReadAccessLabelAdmission::new(
        row.query_label(),
        row.kind(),
    ))
}

const GRAPH_TOUCH_OBLIGATION_VOCABULARY: [&str; 5] = [
    "ForgeQueryGraphObligationSelection",
    "ForgeQueryGraphObligationSupportMatrixRow",
    "ForgeQueryGraphTouchDescriptor",
    "selected_query_graph_obligation",
    "graph_touch_obligation_adoption_proof",
];

const INVENTORY_ACCESS_VOCABULARY_KINDS: [QueryGraphReadAccessCapabilityKind; 5] = [
    QueryGraphReadAccessCapabilityKind::AdmissionPosture,
    QueryGraphReadAccessCapabilityKind::DenialKind,
    QueryGraphReadAccessCapabilityKind::RequirementKind,
    QueryGraphReadAccessCapabilityKind::ReceiptField,
    QueryGraphReadAccessCapabilityKind::CostCounter,
];
