pub(crate) fn media_snapshot(root: &std::path::Path) -> Vec<(String, Vec<u8>)> {
    let mut rows = std::fs::read_dir(root)
        .expect("media directory")
        .map(|entry| {
            let entry = entry.expect("media entry");
            (
                entry.file_name().to_string_lossy().into_owned(),
                std::fs::read(entry.path()).expect("media bytes"),
            )
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    rows
}

pub(crate) fn verification_budget() -> worth_store_offline_verifier::BackupVerificationBudget {
    worth_store_offline_verifier::BackupVerificationBudget::from_inspection(
        worth_store_offline_verifier::OfflineInspectionBudget::bounded(4 * 1024, u64::MAX).unwrap(),
    )
}
