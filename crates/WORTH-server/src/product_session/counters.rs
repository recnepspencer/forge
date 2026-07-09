#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthServerProductSessionCounterSnapshot {
    pub sessions_created: u64,
    pub preview_sessions_created: u64,
    pub mutation_sessions_created: u64,
    pub lookups_attempted: u64,
    pub lookups_denied_missing: u64,
    pub lookups_denied_foreign: u64,
    pub lookups_denied_expired: u64,
    pub lookups_denied_closed: u64,
    pub lookups_denied_moved: u64,
    pub lookups_denied_preview_for_mutation: u64,
    pub closes_recorded: u64,
}
