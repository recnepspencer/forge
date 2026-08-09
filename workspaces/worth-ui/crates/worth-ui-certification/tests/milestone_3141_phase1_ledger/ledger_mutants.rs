pub(super) fn remove_first_data_row(ledger: &str) -> String {
    let mut lines = ledger.lines();
    let header = lines.next().unwrap();
    let _ = lines.next();
    std::iter::once(header)
        .chain(lines)
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn duplicate_first_data_row(ledger: &str) -> String {
    let mut lines = ledger.lines();
    let header = lines.next().unwrap();
    let first = lines.next().unwrap();
    std::iter::once(header)
        .chain([first, first])
        .chain(lines)
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn swap_first_data_rows(ledger: &str) -> String {
    let mut lines = ledger.lines();
    let header = lines.next().unwrap();
    let first = lines.next().unwrap();
    let second = lines.next().unwrap();
    std::iter::once(header)
        .chain([second, first])
        .chain(lines)
        .collect::<Vec<_>>()
        .join("\n")
}
