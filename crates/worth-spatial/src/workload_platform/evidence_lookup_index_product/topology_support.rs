use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanTopologyPosture, EvidenceLookupPlanTopologyPostureState,
};

pub(crate) fn selected_topology_support_digests(
    rows: &[crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlanRow],
) -> Vec<String> {
    let mut digests = rows
        .iter()
        .filter_map(|row| topology_support_digest(row.topology_posture()))
        .collect::<Vec<_>>();
    digests.sort();
    digests.dedup();
    digests
}

pub(crate) fn selected_topology_support_digest(
    rows: &[crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlanRow],
) -> String {
    let digests = selected_topology_support_digests(rows);
    if digests.is_empty() {
        return "not-required".to_string();
    }
    digests.join("|")
}

pub(crate) fn topology_receipt_ref_count(
    rows: &[crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlanRow],
) -> usize {
    rows.iter()
        .filter(|row| topology_support_digest(row.topology_posture()).is_some())
        .count()
}

fn topology_support_digest(posture: &EvidenceLookupPlanTopologyPosture) -> Option<String> {
    match posture.state() {
        EvidenceLookupPlanTopologyPostureState::Satisfied {
            seed_digest,
            receipt_ref_digest,
            family_identity,
        } => Some(format!(
            "{seed_digest}:{receipt_ref_digest}:{family_identity}"
        )),
        _ => None,
    }
}
