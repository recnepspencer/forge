use super::*;

pub fn force_publication_commit_id_conflict(
    path: &std::path::Path,
    replacement_commit_id: CommitId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let wal_sequence = state
        .wal_records
        .iter()
        .rev()
        .find_map(|(wal_sequence, record)| match &record.payload {
            WalRecordPayload::DurablePublicationProgress(progress)
                if progress.commit_id.is_some() =>
            {
                Some(*wal_sequence)
            }
            _ => None,
        })
        .expect("store should contain a publication progress wal record");
    let original = state
        .wal_records
        .get(&wal_sequence)
        .cloned()
        .expect("target wal record should exist");
    let replacement = match original.payload {
        WalRecordPayload::DurablePublicationProgress(progress) => {
            WalRecord::durable_publication_progress(
                original.wal_sequence,
                original.durable_mutation_id,
                original.runtime_session_id,
                progress.phase,
                Some(replacement_commit_id),
            )
            .expect("replacement wal record should encode")
        }
        _ => unreachable!("selected wal record should be publication progress"),
    };
    state.wal_records.insert(wal_sequence, replacement);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("conflicted store state should write");
}

pub fn force_branch_head_gap(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    state.branch_head_records.clear();
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("branch head gap state should write");
}
