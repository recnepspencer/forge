use worth_proof::TransitionOutcome;
use worth_signal::facade::SignalGraph;

use super::admission::CorrespondenceAdmissionOutcome;
use super::{BridgeCorrespondenceDenialKind, BridgeInstalledSemanticCorrespondence};

pub(super) fn admit_signal_targets(
    mut allocated: super::target_allocation::AllocatedCorrespondence<'_>,
    graph: &SignalGraph,
) -> CorrespondenceAdmissionOutcome {
    let planned = &mut allocated.planned;
    let signal_targets = match graph.admit_installed_aspects(
        planned
            .targets
            .as_slice()
            .iter()
            .map(|target| (target.node, target.aspect)),
    ) {
        TransitionOutcome::Success(capability) => capability,
        _ => {
            return super::admission::denied(
                BridgeCorrespondenceDenialKind::MissingOrStaleSignalNode,
                planned.resolved.counters,
            )
        }
    };
    planned.resolved.counters.signal_node_admissions = signal_targets.aspects().len();
    planned.resolved.counters.targets_admitted = planned.targets.as_slice().len();
    planned.resolved.counters.authoritative_records_committed = planned.pending_records.len();
    for record in std::mem::take(&mut planned.pending_records) {
        allocated.registry.commit(record);
    }
    TransitionOutcome::Success(BridgeInstalledSemanticCorrespondence::admit_ready(
        allocated.planned.resolved.recipe,
        allocated.planned.targets,
        allocated.planned.resolved.counters,
        &allocated.planned.resolved.signal_graph,
        &signal_targets,
    ))
}
