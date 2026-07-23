use worth_runtime_bridge::facade::{
    BridgeMutationAuthorityBundle, BridgeMutationSubjectKind, BridgeMutationSubjectTouch,
};

use super::{
    WorthQueryCommitIdentity, WorthQueryMutationDelta, WorthQueryMutationKind,
    WorthQuerySnapshotIdentity,
};

pub(super) fn bridge_authority_admits_receipt(
    commit_identity: &WorthQueryCommitIdentity,
    snapshot_identity: &WorthQuerySnapshotIdentity,
    deltas: &[WorthQueryMutationDelta],
    authority: &BridgeMutationAuthorityBundle,
) -> bool {
    let (Some(commit), Some(snapshot), [delta]) = (
        commit_identity.bridge_identity(),
        snapshot_identity.bridge_identity(),
        deltas,
    ) else {
        return false;
    };
    authority
        .causality()
        .retains_truth_handoff(commit, snapshot)
        && bridge_authority_admits_delta(delta, authority)
}

fn bridge_authority_admits_delta(
    delta: &WorthQueryMutationDelta,
    authority: &BridgeMutationAuthorityBundle,
) -> bool {
    let Some(target_record) = delta.entity_identity.relational_record_parts() else {
        return false;
    };
    let touches = delta
        .touched_aspects
        .iter()
        .map(|touch| match touch.native_field_path() {
            Some(field_path) => BridgeMutationSubjectTouch::aspect_field_path(
                touch.native_aspect_key().clone(),
                field_path.clone(),
            ),
            None => BridgeMutationSubjectTouch::whole_aspect(touch.native_aspect_key().clone()),
        })
        .collect::<Vec<_>>();
    authority.causality().retains_mutation_subject(
        delta.collection_identity.as_str(),
        target_record,
        bridge_mutation_subject_kind(&delta.kind),
        &touches,
    )
}

fn bridge_mutation_subject_kind(kind: &WorthQueryMutationKind) -> BridgeMutationSubjectKind {
    match kind {
        WorthQueryMutationKind::Created => BridgeMutationSubjectKind::Created,
        WorthQueryMutationKind::Updated => BridgeMutationSubjectKind::Updated,
        WorthQueryMutationKind::Deleted => BridgeMutationSubjectKind::Deleted,
    }
}
