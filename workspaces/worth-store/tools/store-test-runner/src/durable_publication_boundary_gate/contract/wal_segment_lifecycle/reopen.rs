use super::super::wal_source_syntax::ParsedRustSource;

pub(super) fn inspect(source: &str) -> Result<(), String> {
    let source = ParsedRustSource::parse(source, "bounded WAL reopen owner")?;
    let reopen = source.function("reopen_wal_inventory")?;
    reopen.deny("method:list_file_names")?;
    reopen.require_exact("method:list_file_names_bounded", 1)?;
    reopen.require_in_order(&[
        "method:list_file_names_bounded",
        "call:parse",
        "method:file_length",
        "method:try_reserve_exact",
        "method:read_exact_at",
        "call:inspect",
        "call:from_segment_scan",
        "call:from_reopened",
        "call:require_checkpoint_cutoff_within_retained_wal",
        "method:retains_canonical_wal_origin",
    ])
}
