use super::selection_error::QueryObligationSelectionError;

pub fn deny_copied_query_obligation_selection_parts(
    surface: &str,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::copied_selection_parts_denied(surface)
}

pub fn deny_local_query_obligation_selector_authority(
    surface: &str,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::local_selector_authority_denied(surface)
}

pub fn deny_broad_collection_query_obligation_selector_authority(
    surface: &str,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::broad_collection_selector_authority_denied(surface)
}

pub fn deny_lifecycle_only_query_obligation_selector_authority(
    surface: &str,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::lifecycle_only_selector_authority_denied(surface)
}

pub fn deny_local_support_row_query_obligation_authority(
    surface: &str,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::local_support_row_authority_denied(surface)
}

pub fn deny_in_memory_query_obligation_selection_authority(
    surface: &str,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::in_memory_selection_authority_denied(surface)
}

pub fn deny_raw_descriptor_query_obligation_selection_authority(
    surface: &str,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::raw_descriptor_authority_denied(surface)
}

pub fn deny_topology_spatial_substitution_query_obligation_authority(
    surface: &str,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::topology_spatial_substitution_authority_denied(surface)
}

pub fn deny_source_grep_query_obligation_audit_authority(
    surface: &str,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::source_grep_audit_authority_denied(surface)
}
