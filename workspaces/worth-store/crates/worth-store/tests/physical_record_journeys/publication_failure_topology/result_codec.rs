pub(super) struct ReopenedResult {
    pub(super) generation: u64,
    pub(super) records: usize,
    pub(super) residue: bool,
    pub(super) fenced: bool,
    pub(super) recovery_count: usize,
}

pub(super) fn parse_reopener(stdout: &str) -> ReopenedResult {
    let fields = completion_fields(stdout, "C5_PUBLICATION_REOPEN ");
    assert_eq!(fields.len(), 5);
    ReopenedResult {
        generation: fields[0].parse().unwrap(),
        records: fields[1].parse().unwrap(),
        residue: fields[2].parse().unwrap(),
        fenced: fields[3].parse().unwrap(),
        recovery_count: fields[4].parse().unwrap(),
    }
}

pub(super) struct OfflineResult {
    pub(super) generation: u64,
    pub(super) records: usize,
}

pub(super) fn parse_offline(stdout: &str) -> OfflineResult {
    let fields = completion_fields(stdout, "C5_OFFLINE ");
    assert_eq!(fields.len(), 10);
    OfflineResult {
        generation: fields[1].parse().unwrap(),
        records: fields[2].parse().unwrap(),
    }
}

fn completion_fields<'output>(stdout: &'output str, prefix: &str) -> Vec<&'output str> {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("child output omitted `{prefix}` completion"))
        .split_whitespace()
        .collect()
}
