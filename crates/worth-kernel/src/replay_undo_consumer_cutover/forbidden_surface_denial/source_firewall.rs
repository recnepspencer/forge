use super::denial_kind::ReplayUndoForbiddenConsumerSurfaceKind;
use crate::replay_undo_consumer_cutover::error::{
    forbidden_surface_firewall_violation, ReplayUndoConsumerCutoverError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoForbiddenConsumerSurfaceFirewallReport {
    rows: Vec<ReplayUndoForbiddenConsumerSurfaceFirewallRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoForbiddenConsumerSurfaceFirewallRow {
    kind: ReplayUndoForbiddenConsumerSurfaceKind,
    scanned_source: &'static str,
    forbidden_pattern: &'static str,
    ordinary_occurrence_count: usize,
    allowed_non_authority_occurrence_count: usize,
}

pub fn current_replay_undo_forbidden_surface_firewall_report(
) -> ReplayUndoForbiddenConsumerSurfaceFirewallReport {
    const BOOLEAN_CHAIN_HANDOFF_SOURCE: &str =
        include_str!("../../workload_composition/worth_workload/boolean_chain_handoff.rs");
    const BOOLEAN_CHAIN_HANDOFF_PATH: &str =
        "crates/worth-kernel/src/workload_composition/worth_workload/boolean_chain_handoff.rs";

    ReplayUndoForbiddenConsumerSurfaceFirewallReport::new(vec![
        ReplayUndoForbiddenConsumerSurfaceFirewallRow::new(
            ReplayUndoForbiddenConsumerSurfaceKind::OldReplayHelper,
            BOOLEAN_CHAIN_HANDOFF_PATH,
            ".complete_boolean_chain_integration_handoff(",
            count_occurrences(
                BOOLEAN_CHAIN_HANDOFF_SOURCE,
                ".complete_boolean_chain_integration_handoff(",
            ),
            count_occurrences(
                BOOLEAN_CHAIN_HANDOFF_SOURCE,
                "pub(crate) fn complete_boolean_chain_integration_handoff(",
            ),
        ),
        ReplayUndoForbiddenConsumerSurfaceFirewallRow::new(
            ReplayUndoForbiddenConsumerSurfaceKind::LocalRollbackShortcut,
            BOOLEAN_CHAIN_HANDOFF_PATH,
            "packetless_legacy_loop_handoff_witness",
            count_occurrences(
                BOOLEAN_CHAIN_HANDOFF_SOURCE,
                "packetless_legacy_loop_handoff_witness",
            ),
            0,
        ),
    ])
}

impl ReplayUndoForbiddenConsumerSurfaceFirewallReport {
    pub(crate) fn new(rows: Vec<ReplayUndoForbiddenConsumerSurfaceFirewallRow>) -> Self {
        Self { rows }
    }

    pub fn require_clean(&self) -> Result<(), ReplayUndoConsumerCutoverError> {
        for row in &self.rows {
            if row.ordinary_occurrence_count != 0 {
                return Err(forbidden_surface_firewall_violation(
                    row.kind,
                    row.scanned_source,
                    row.ordinary_occurrence_count,
                ));
            }
        }
        Ok(())
    }

    pub fn rows(&self) -> &[ReplayUndoForbiddenConsumerSurfaceFirewallRow] {
        &self.rows
    }

    pub fn row_for_kind(
        &self,
        kind: ReplayUndoForbiddenConsumerSurfaceKind,
    ) -> Option<&ReplayUndoForbiddenConsumerSurfaceFirewallRow> {
        self.rows.iter().find(|row| row.kind == kind)
    }
}

impl ReplayUndoForbiddenConsumerSurfaceFirewallRow {
    pub(crate) const fn new(
        kind: ReplayUndoForbiddenConsumerSurfaceKind,
        scanned_source: &'static str,
        forbidden_pattern: &'static str,
        ordinary_occurrence_count: usize,
        allowed_non_authority_occurrence_count: usize,
    ) -> Self {
        Self {
            kind,
            scanned_source,
            forbidden_pattern,
            ordinary_occurrence_count,
            allowed_non_authority_occurrence_count,
        }
    }

    pub const fn kind(&self) -> ReplayUndoForbiddenConsumerSurfaceKind {
        self.kind
    }

    pub const fn scanned_source(&self) -> &'static str {
        self.scanned_source
    }

    pub const fn forbidden_pattern(&self) -> &'static str {
        self.forbidden_pattern
    }

    pub const fn ordinary_occurrence_count(&self) -> usize {
        self.ordinary_occurrence_count
    }

    pub const fn allowed_non_authority_occurrence_count(&self) -> usize {
        self.allowed_non_authority_occurrence_count
    }
}

fn count_occurrences(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}
