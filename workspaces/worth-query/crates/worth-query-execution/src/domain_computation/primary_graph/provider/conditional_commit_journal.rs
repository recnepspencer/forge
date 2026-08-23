use std::collections::{BTreeMap, BTreeSet};

use worth_relational::facade::{history::CommitId, transactions::RecordRef};

const MAX_RETAINED_CONDITIONAL_COMMITS: usize = 100_000;

#[derive(Default)]
struct WorthQueryConditionalCommitRoute {
    entries: BTreeMap<u64, CommitId>,
    dropped_through: Option<u64>,
}

impl WorthQueryConditionalCommitRoute {
    fn record(&mut self, sequence: u64, commit: CommitId) {
        self.entries.insert(sequence, commit);
        if self.entries.len() > MAX_RETAINED_CONDITIONAL_COMMITS {
            let dropped = *self
                .entries
                .first_key_value()
                .expect("over-capacity route has a first entry")
                .0;
            self.entries.remove(&dropped);
            self.dropped_through = Some(dropped);
        }
    }

    fn requires_reconstruction(&self, cursor: u64) -> bool {
        self.dropped_through.is_some_and(|dropped| cursor < dropped)
    }

    fn after(&self, cursor: u64, maximum: usize) -> impl Iterator<Item = (u64, CommitId)> + '_ {
        self.entries
            .range((
                std::ops::Bound::Excluded(cursor),
                std::ops::Bound::Unbounded,
            ))
            .take(maximum)
            .map(|(sequence, commit)| (*sequence, *commit))
    }
}

#[derive(Default)]
pub(super) struct WorthQueryConditionalCommitJournal {
    latest_sequence: u64,
    exact_routes: BTreeMap<RecordRef, WorthQueryConditionalCommitRoute>,
    whole_graph_route: Option<WorthQueryConditionalCommitRoute>,
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryConditionalCommitBatch {
    pub(in crate::domain_computation::primary_graph) commits: Vec<(u64, CommitId)>,
    pub(in crate::domain_computation::primary_graph) cursor: u64,
    pub(in crate::domain_computation::primary_graph) work_remaining: bool,
}

impl WorthQueryConditionalCommitJournal {
    pub(super) fn latest_sequence(&self) -> u64 {
        self.latest_sequence
    }

    pub(super) fn replace_routes(
        &mut self,
        exact_records: impl IntoIterator<Item = RecordRef>,
        include_whole_graph: bool,
    ) {
        let retained = exact_records.into_iter().collect::<BTreeSet<_>>();
        self.exact_routes
            .retain(|record, _| retained.contains(record));
        for record in retained {
            self.exact_routes.entry(record).or_default();
        }
        match (include_whole_graph, self.whole_graph_route.is_some()) {
            (true, false) => self.whole_graph_route = Some(Default::default()),
            (false, true) => self.whole_graph_route = None,
            _ => {}
        }
    }

    pub(super) fn record(
        &mut self,
        commit: CommitId,
        records: impl IntoIterator<Item = RecordRef>,
    ) {
        self.latest_sequence = self.latest_sequence.saturating_add(1);
        let sequence = self.latest_sequence;
        if let Some(route) = self.whole_graph_route.as_mut() {
            route.record(sequence, commit);
        }
        for record in records.into_iter().collect::<BTreeSet<_>>() {
            if let Some(route) = self.exact_routes.get_mut(&record) {
                route.record(sequence, commit);
            }
        }
    }

    pub(super) fn after_records(
        &self,
        sequence: u64,
        maximum: usize,
        watched_records: impl IntoIterator<Item = RecordRef>,
        include_whole_graph: bool,
    ) -> Result<WorthQueryConditionalCommitBatch, &'static str> {
        let watched = watched_records.into_iter().collect::<BTreeSet<_>>();
        let routes = watched
            .iter()
            .filter_map(|record| self.exact_routes.get(record))
            .chain(
                include_whole_graph
                    .then_some(self.whole_graph_route.as_ref())
                    .flatten(),
            );
        let mut relevant = BTreeMap::new();
        for route in routes {
            if route.requires_reconstruction(sequence) {
                return Err("conditional authoritative-change route was overrun");
            }
            relevant.extend(route.after(sequence, maximum.saturating_add(1)));
        }
        let work_remaining = relevant.len() > maximum;
        let commits = relevant.into_iter().take(maximum).collect::<Vec<_>>();
        let cursor = commits
            .last()
            .map(|(sequence, _)| *sequence)
            .unwrap_or(self.latest_sequence);
        Ok(WorthQueryConditionalCommitBatch {
            commits,
            cursor,
            work_remaining,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_relational::facade::identity::{EntityId, PartitionId};

    fn entity(slot: u32) -> RecordRef {
        RecordRef::Entity(EntityId::new(PartitionId(1), u64::from(slot), 1))
    }

    #[test]
    fn same_kind_unrelated_records_do_not_enter_exact_record_route() {
        let mut journal = WorthQueryConditionalCommitJournal::default();
        journal.replace_routes([entity(7)], false);
        journal.record(CommitId(1), [entity(8)]);
        journal.record(CommitId(2), [entity(9)]);

        let batch = journal.after_records(0, 8, [entity(7)], false).unwrap();
        assert!(batch.commits.is_empty());
        assert_eq!(batch.cursor, 2);
    }

    #[test]
    fn exact_record_route_retains_only_matching_commits() {
        let mut journal = WorthQueryConditionalCommitJournal::default();
        journal.replace_routes([entity(7)], false);
        journal.record(CommitId(1), [entity(7)]);
        journal.record(CommitId(2), [entity(8)]);

        assert_eq!(
            journal
                .after_records(0, 8, [entity(7)], false)
                .unwrap()
                .commits,
            vec![(1, CommitId(1))]
        );
    }

    #[test]
    fn whole_graph_route_admits_every_committed_record() {
        let mut journal = WorthQueryConditionalCommitJournal::default();
        journal.replace_routes(std::iter::empty(), true);
        journal.record(CommitId(1), [entity(8)]);
        journal.record(CommitId(2), [entity(9)]);

        assert_eq!(
            journal
                .after_records(0, 8, std::iter::empty(), true)
                .unwrap()
                .commits,
            vec![(1, CommitId(1)), (2, CommitId(2))]
        );
    }

    #[test]
    fn exact_route_reports_only_its_own_real_overrun() {
        let mut journal = WorthQueryConditionalCommitJournal::default();
        journal.replace_routes([entity(7)], false);
        for commit in 1..=(MAX_RETAINED_CONDITIONAL_COMMITS as u64 + 1) {
            journal.record(CommitId(commit), [entity(7)]);
        }
        assert!(journal.after_records(0, 1, [entity(7)], false).is_err());
    }

    #[test]
    fn unrelated_commits_beyond_capacity_neither_scan_nor_overrun_exact_route() {
        let mut journal = WorthQueryConditionalCommitJournal::default();
        journal.replace_routes([entity(7)], false);
        for commit in 1..=(MAX_RETAINED_CONDITIONAL_COMMITS as u64 + 1) {
            journal.record(CommitId(commit), [entity(8)]);
        }
        let batch = journal.after_records(0, 1, [entity(7)], false).unwrap();
        assert!(batch.commits.is_empty());
        assert_eq!(batch.cursor, MAX_RETAINED_CONDITIONAL_COMMITS as u64 + 1);
        assert!(journal.exact_routes[&entity(7)].entries.is_empty());
    }

    #[test]
    fn route_replacement_bounds_record_identity_inventory() {
        let mut journal = WorthQueryConditionalCommitJournal::default();
        journal.replace_routes([entity(7), entity(8)], false);
        journal.replace_routes([entity(9)], false);
        assert_eq!(
            journal.exact_routes.keys().cloned().collect::<Vec<_>>(),
            vec![entity(9)]
        );
    }
}
