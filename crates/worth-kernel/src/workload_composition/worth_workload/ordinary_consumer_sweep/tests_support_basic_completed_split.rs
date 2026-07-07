use crate::workload_composition::CompletedBooleanSplitHandoff;

#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod replay_support;

pub(crate) fn ordinary_completed_split_handoff(
    label: &'static str,
) -> CompletedBooleanSplitHandoff {
    let subject = replay_support::MetabossEventExtractionSubject::certify(label);
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    replay_support::completed_split_handoff_for(&subject, &replay_subject)
}
