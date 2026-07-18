pub(super) fn controlled_defect_case(case_name: &str) -> bool {
    [
        "corrupt",
        "denial",
        "denies",
        "forbid",
        "hostile",
        "invalid",
        "missing",
        "mutant",
        "reject",
        "stale",
        "violation",
    ]
    .iter()
    .any(|marker| case_name.contains(marker))
}
