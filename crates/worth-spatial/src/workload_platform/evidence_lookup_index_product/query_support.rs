use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanQueryPosture, EvidenceLookupPlanQueryPostureState,
};

pub(crate) fn selected_query_support_digests(
    rows: &[crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlanRow],
) -> Vec<String> {
    let mut digests = rows
        .iter()
        .filter_map(|row| query_support_digest(row.query_posture()))
        .collect::<Vec<_>>();
    digests.sort();
    digests.dedup();
    digests
}

pub(crate) fn selected_query_support_digest(
    rows: &[crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlanRow],
) -> String {
    let digests = selected_query_support_digests(rows);
    if digests.is_empty() {
        return "none".to_string();
    }
    digests.join("|")
}

pub(crate) fn query_support_row_count(
    rows: &[crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlanRow],
) -> usize {
    rows.iter()
        .filter(|row| query_support_digest(row.query_posture()).is_some())
        .count()
}

fn query_support_digest(posture: &EvidenceLookupPlanQueryPosture) -> Option<String> {
    match posture.state() {
        EvidenceLookupPlanQueryPostureState::Satisfied { .. } => posture.satisfied_digest_summary(),
        _ => None,
    }
}
