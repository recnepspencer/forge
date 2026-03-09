use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::diagnostics::failure::{FailureSummary, RollbackDiagnostic};
use crate::diagnostics::flow::{ChangeInputSummary, FlowSummary, InvalidationSummary};
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::summary::ExecutionHistorySummary;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct DiagnosticsState {
    #[serde(default)]
    profile: DiagnosticsProfile,
    #[serde(default)]
    latest_flow: Option<FlowSummary>,
    #[serde(default)]
    latest_failure: Option<FailureSummary>,
    #[serde(default)]
    latest_rollback: Option<RollbackDiagnostic>,
    #[serde(default)]
    recent_history: VecDeque<ExecutionHistorySummary>,
    #[serde(default)]
    pending_input: Option<PendingFlowInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PendingFlowInput {
    changed_nodes: Vec<NodeId>,
    changed_aspects: Vec<Aspect>,
    changed_region_count: u32,
    causality_kind: Option<String>,
    invalidated_direct_subscribers: u32,
    maybe_stale_direct_subscribers: u32,
    partition_scoped_checks: u32,
}

impl DiagnosticsState {
    pub fn profile(&self) -> DiagnosticsProfile {
        self.profile
    }

    pub fn set_profile(&mut self, profile: DiagnosticsProfile) {
        self.profile = profile;
        self.trim_history();
    }

    pub fn latest_flow(&self) -> Option<&FlowSummary> {
        self.latest_flow.as_ref()
    }

    pub fn latest_failure(&self) -> Option<&FailureSummary> {
        self.latest_failure.as_ref()
    }

    pub fn latest_rollback(&self) -> Option<&RollbackDiagnostic> {
        self.latest_rollback.as_ref()
    }

    pub fn recent_history(&self) -> &VecDeque<ExecutionHistorySummary> {
        &self.recent_history
    }

    pub fn note_change_input(
        &mut self,
        node: NodeId,
        aspect: Aspect,
        changed_regions: &[ChangedRegion],
        causality_kind: Option<String>,
    ) {
        let pending = self.pending_input.get_or_insert_with(|| PendingFlowInput {
            changed_nodes: Vec::new(),
            changed_aspects: Vec::new(),
            changed_region_count: 0,
            causality_kind: None,
            invalidated_direct_subscribers: 0,
            maybe_stale_direct_subscribers: 0,
            partition_scoped_checks: 0,
        });
        if !pending.changed_nodes.contains(&node) {
            pending.changed_nodes.push(node);
            pending.changed_nodes.sort();
        }
        if !pending.changed_aspects.contains(&aspect) {
            pending.changed_aspects.push(aspect);
            pending
                .changed_aspects
                .sort_by_key(|changed_aspect| changed_aspect.index());
        }
        pending.changed_region_count += changed_regions.len() as u32;
        if pending.causality_kind.is_none() {
            pending.causality_kind = causality_kind;
        }
    }

    pub fn record_invalidation_result(
        &mut self,
        invalidated_direct_subscribers: u32,
        maybe_stale_direct_subscribers: u32,
        partition_scoped_checks: u32,
    ) {
        if let Some(pending) = &mut self.pending_input {
            pending.invalidated_direct_subscribers += invalidated_direct_subscribers;
            pending.maybe_stale_direct_subscribers += maybe_stale_direct_subscribers;
            pending.partition_scoped_checks += partition_scoped_checks;
        }
    }

    pub fn complete_flow(
        &mut self,
        flow: FlowSummary,
        history: ExecutionHistorySummary,
    ) {
        self.latest_flow = Some(flow);
        self.recent_history.push_back(history);
        self.trim_history();
        self.pending_input = None;
    }

    pub fn record_failure(&mut self, failure: FailureSummary) {
        self.latest_failure = Some(failure);
    }

    pub fn record_rollback(&mut self, rollback: RollbackDiagnostic) {
        self.latest_rollback = Some(rollback);
    }

    pub fn clear_pending_input(&mut self) {
        self.pending_input = None;
    }

    pub fn pending_change_summary(&self) -> Option<(ChangeInputSummary, InvalidationSummary)> {
        self.pending_input.as_ref().map(|pending| {
            (
                ChangeInputSummary::new(
                    pending.changed_nodes.clone(),
                    pending.changed_aspects.clone(),
                    pending.changed_region_count,
                    pending.causality_kind.clone(),
                ),
                InvalidationSummary::new(
                    pending.invalidated_direct_subscribers,
                    pending.maybe_stale_direct_subscribers,
                    pending.partition_scoped_checks,
                ),
            )
        })
    }

    fn trim_history(&mut self) {
        let limit = self.profile.history_limit();
        while self.recent_history.len() > limit {
            self.recent_history.pop_front();
        }
    }
}
