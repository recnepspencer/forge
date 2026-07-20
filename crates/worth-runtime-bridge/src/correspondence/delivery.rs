use worth_proof::TransitionOutcome;
use worth_signal::facade::SignalGraph;

use crate::facade::RuntimeBridge;
use crate::input::envelope::{BridgeCommittedPatchEnvelope, BridgeProducerAuthorityKind};

use super::delivery_preflight::{admit_envelope_source, preflight};
use super::semantic_delivery_match::match_envelope;
use super::{
    BridgeCorrespondenceAdmissionFailure, BridgeCorrespondenceDeferred,
    BridgeCorrespondenceDeliveryDenial, BridgeCorrespondenceDenialKind,
    BridgeCorrespondenceRebindRequired, BridgeCorrespondenceStale,
    BridgeInstalledSemanticCorrespondence, CorrespondenceDeliveryCounters,
};

pub type CorrespondenceDeliveryOutcome = TransitionOutcome<
    CorrespondenceDeliveryCounters,
    BridgeCorrespondenceDeliveryDenial,
    BridgeCorrespondenceDeferred,
    BridgeCorrespondenceStale,
    BridgeCorrespondenceRebindRequired,
    BridgeCorrespondenceAdmissionFailure,
>;

impl RuntimeBridge {
    pub(crate) fn deliver_installed_correspondence(
        &self,
        correspondence: &BridgeInstalledSemanticCorrespondence,
        graph: &mut SignalGraph,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> CorrespondenceDeliveryOutcome {
        if let Some(outcome) = preflight(self, correspondence, graph) {
            return outcome;
        }
        let requested_commit = request.commit_identity().clone();
        let envelope = match self.committed_patch_source.load_committed_patch(request) {
            Ok(envelope) => envelope,
            Err(_) => {
                return TransitionOutcome::Failed(
                    BridgeCorrespondenceAdmissionFailure::SourceLoadFailed,
                )
            }
        };
        if envelope.commit_identity() != &requested_commit {
            let mut counters = CorrespondenceDeliveryCounters::zero();
            counters.source_load_attempts = 1;
            counters.source_envelopes_loaded = 1;
            counters.failed_deliveries = 1;
            return TransitionOutcome::Denied(BridgeCorrespondenceDeliveryDenial::new(
                BridgeCorrespondenceDenialKind::CommittedPatchRequestMismatch,
                counters,
            ));
        }
        if envelope.producer_metadata().authority_kind()
            != BridgeProducerAuthorityKind::RegisteredAuthoritativeSource
        {
            let mut counters = CorrespondenceDeliveryCounters::zero();
            counters.source_load_attempts = 1;
            counters.source_envelopes_loaded = 1;
            counters.failed_deliveries = 1;
            return TransitionOutcome::Denied(BridgeCorrespondenceDeliveryDenial::new(
                BridgeCorrespondenceDenialKind::AuthoritativeSourceMismatch,
                counters,
            ));
        }
        let mut counters = CorrespondenceDeliveryCounters::zero();
        counters.source_load_attempts = 1;
        counters.source_envelopes_loaded = 1;
        self.deliver_installed_correspondence_envelope_with_counters(
            correspondence,
            graph,
            &envelope,
            counters,
        )
    }

    #[cfg(test)]
    pub(crate) fn deliver_installed_correspondence_envelope(
        &self,
        correspondence: &BridgeInstalledSemanticCorrespondence,
        graph: &mut SignalGraph,
        envelope: &BridgeCommittedPatchEnvelope,
    ) -> CorrespondenceDeliveryOutcome {
        self.deliver_installed_correspondence_envelope_with_counters(
            correspondence,
            graph,
            envelope,
            CorrespondenceDeliveryCounters::zero(),
        )
    }

    fn deliver_installed_correspondence_envelope_with_counters(
        &self,
        correspondence: &BridgeInstalledSemanticCorrespondence,
        graph: &mut SignalGraph,
        envelope: &BridgeCommittedPatchEnvelope,
        mut counters: CorrespondenceDeliveryCounters,
    ) -> CorrespondenceDeliveryOutcome {
        if let Err(denial) = admit_envelope_source(correspondence, envelope, counters) {
            return TransitionOutcome::Denied(denial);
        }
        if let Some(outcome) = preflight(self, correspondence, graph) {
            return outcome;
        }
        let basis = correspondence.basis();
        let targets = correspondence.targets.as_slice();
        counters.allocation_registry_lock_attempts += 1;
        let allocation_registry = match self.correspondence_allocations.try_read() {
            Ok(registry) => registry,
            Err(std::sync::TryLockError::WouldBlock) => {
                return TransitionOutcome::Deferred(
                    BridgeCorrespondenceDeferred::GraphMutationInProgress,
                )
            }
            Err(std::sync::TryLockError::Poisoned(_)) => {
                return TransitionOutcome::Failed(
                    BridgeCorrespondenceAdmissionFailure::LockPoisoned,
                )
            }
        };
        if targets
            .iter()
            .any(|target| !allocation_registry.admits_source_set(target))
        {
            return TransitionOutcome::RebindRequired(
                BridgeCorrespondenceRebindRequired::AllocationSourceSet,
            );
        }
        drop(allocation_registry);
        if targets.iter().any(|target| {
            target.signal_graph_instance_id != basis.signal_graph_instance_id
                || !basis.signal_partitions.contains(&target.partition)
        }) {
            return TransitionOutcome::RebindRequired(
                BridgeCorrespondenceRebindRequired::SignalGraphGeneration,
            );
        }

        let mut counters =
            match match_envelope(correspondence.ready.payload(), targets, envelope, counters) {
                Ok(counters) => counters,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        if counters.truth_targets_admitted == 0 {
            return TransitionOutcome::Success(counters);
        }

        let mut signal_capabilities = Vec::with_capacity(targets.len());
        for target in targets {
            let capability = match graph.admit_installed_aspect(target.node, target.aspect) {
                TransitionOutcome::Success(capability) => capability,
                _ => {
                    return TransitionOutcome::RebindRequired(
                        BridgeCorrespondenceRebindRequired::SignalGraphGeneration,
                    )
                }
            };
            if capability.graph_instance_id() != target.signal_graph_instance_id {
                return TransitionOutcome::RebindRequired(
                    BridgeCorrespondenceRebindRequired::SignalGraphGeneration,
                );
            }
            signal_capabilities.push(capability);
            counters.signal_capability_admissions += 1;
        }
        if worth_signal::facade::apply_installed_aspect_changes(graph, signal_capabilities).is_err()
        {
            return TransitionOutcome::Failed(
                BridgeCorrespondenceAdmissionFailure::SignalMutationFailed,
            );
        }
        counters.signal_seeds_emitted = targets.len();
        counters.node_fan_out = targets
            .iter()
            .map(|target| target.node)
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        counters.slots_touched = targets.len();
        TransitionOutcome::Success(counters)
    }
}
