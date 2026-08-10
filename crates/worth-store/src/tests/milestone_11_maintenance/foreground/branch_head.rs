use super::super::*;

pub(super) fn latest_branch_head(
    store: &WORTHStore,
) -> (
    worth_relational::facade::history::BranchId,
    worth_relational::facade::history::CommitId,
) {
    let export = store.export_authoritative_records().into_canonicalized();
    let envelope = export
        .commit_envelopes
        .last()
        .expect("foreground maintenance fixture requires a canonical commit");
    (
        envelope.envelope.branch_context.clone(),
        envelope.envelope.commit.commit_id,
    )
}
