pub(super) fn file_facts(
    walked: &crate::StructurallyWalkedMedia,
) -> Vec<(
    worth_store_physical_format::OfflinePhysicalArtifactFamily,
    u64,
    [u8; 32],
)> {
    walked
        .files()
        .iter()
        .map(|file| (file.family(), file.length(), file.content_digest()))
        .collect()
}
