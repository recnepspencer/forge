use worth_proof::TransitionOutcome;
use worth_signal::facade::SignalGraph;

use crate::facade::RuntimeBridge;
use crate::input::envelope::{BridgeCommittedPatchEnvelope, BridgeProducerAuthorityKind};

use super::delivery::CorrespondenceDeliveryOutcome;
use super::{
    BridgeCorrespondenceDeliveryDenial, BridgeCorrespondenceDenialKind,
    BridgeCorrespondenceRebindRequired, BridgeCorrespondenceStale,
    BridgeInstalledSemanticCorrespondence,
};

pub(crate) fn preflight(
    runtime: &RuntimeBridge,
    correspondence: &BridgeInstalledSemanticCorrespondence,
    graph: &SignalGraph,
) -> Option<CorrespondenceDeliveryOutcome> {
    let basis = correspondence.basis();
    if basis.bridge_runtime_key != runtime.signal_runtime_key {
        return Some(TransitionOutcome::Stale(
            BridgeCorrespondenceStale::BridgeRuntimeBasis,
        ));
    }
    let graph = graph.installed_graph_capability();
    if basis.signal_graph_instance_id != graph.graph_instance_id() {
        return Some(TransitionOutcome::RebindRequired(
            BridgeCorrespondenceRebindRequired::SignalGraphGeneration,
        ));
    }
    None
}

pub(crate) fn admit_envelope_source(
    correspondence: &BridgeInstalledSemanticCorrespondence,
    envelope: &BridgeCommittedPatchEnvelope,
    mut counters: super::CorrespondenceDeliveryCounters,
) -> Result<(), BridgeCorrespondenceDeliveryDenial> {
    match envelope.producer_metadata().authority_kind() {
        BridgeProducerAuthorityKind::RegisteredAuthoritativeSource => {
            let basis = correspondence.basis();
            let admitted = envelope
                .producer_metadata()
                .authoritative_source()
                .is_some_and(|source| {
                    source.graph_role() == basis.declared_graph_role.as_ref()
                        && basis
                            .authoritative_source_profile
                            .as_ref()
                            .is_some_and(|profile| source.matches_profile(profile))
                });
            if !admitted {
                counters.failed_deliveries += 1;
                return Err(BridgeCorrespondenceDeliveryDenial::new(
                    BridgeCorrespondenceDenialKind::AuthoritativeSourceMismatch,
                    counters,
                ));
            }
        }
        BridgeProducerAuthorityKind::BridgeHarnessFixture => {}
        BridgeProducerAuthorityKind::Unknown => {
            counters.failed_deliveries += 1;
            return Err(BridgeCorrespondenceDeliveryDenial::new(
                BridgeCorrespondenceDenialKind::AuthoritativeSourceMismatch,
                counters,
            ));
        }
    }
    Ok(())
}
