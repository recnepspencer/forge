use worth_store_formal_models::CompactionVisibilityMappedOwnerCase;

pub(in crate::courtroom::protocol_models) fn omit_one_mapping(
    mappings: impl IntoIterator<Item = CompactionVisibilityMappedOwnerCase>,
) -> Vec<CompactionVisibilityMappedOwnerCase> {
    let mut mappings = mappings.into_iter().collect::<Vec<_>>();
    mappings.pop();
    mappings
}
