pub(super) fn classify_parallel_admission(
    legality_decision: &BridgeParallelLegalityDecision,
    profitability_decision: &BridgeParallelProfitabilityDecision,
) -> (
    BridgePreparationMode,
    BridgeParallelAdmissionClass,
    BridgeParallelAdmissionReason,
) {
    let (class, reason) =
        classify_parallel_admission_components(legality_decision, profitability_decision);
    let mode = match class {
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted => {
            BridgePreparationMode::ParallelPreparation
        }
        BridgeParallelAdmissionClass::SerialRequired
        | BridgeParallelAdmissionClass::ParallelPreparationRejected => {
            BridgePreparationMode::Serial
        }
    };
    (mode, class, reason)
}

pub(super) fn classify_parallel_legality(
    packet_set: &PlannedBridgePacketSet,
) -> BridgeParallelLegalityDecision {
    let (class, reason) = if packet_set.routing_packets().len() <= 1 {
        (
            BridgeParallelLegalityClass::SerialOnly,
            BridgeParallelLegalityReason::BelowMinWorkloadWidth,
        )
    } else if !packet_set.continuity_packets().is_empty() {
        (
            BridgeParallelLegalityClass::ParallelPreparationIllegal,
            BridgeParallelLegalityReason::ContinuityRemapRequiresSerialPreparation,
        )
    } else if packet_set
        .truth_view_packets()
        .iter()
        .any(|packet| packet.planned_route_count() > 1)
    {
        (
            BridgeParallelLegalityClass::ParallelPreparationIllegal,
            BridgeParallelLegalityReason::SharedTruthViewMaterializationTarget,
        )
    } else if !packet_region_identities_are_disjoint(packet_set) {
        (
            BridgeParallelLegalityClass::ParallelPreparationIllegal,
            BridgeParallelLegalityReason::PacketRegionOverlapDetected,
        )
    } else {
        (
            BridgeParallelLegalityClass::ParallelPreparationLegal,
            BridgeParallelLegalityReason::DisjointPacketRegionsCertified,
        )
    };
    BridgeParallelLegalityDecision::new(class, reason)
}

pub(super) fn classify_parallel_profitability(
    packet_set: &PlannedBridgePacketSet,
    legality_decision: &BridgeParallelLegalityDecision,
    locality_footprint: &BridgeLocalityFootprint,
) -> BridgeParallelProfitabilityDecision {
    let (class, reason) = match legality_decision.class() {
        BridgeParallelLegalityClass::SerialOnly
        | BridgeParallelLegalityClass::ParallelPreparationIllegal => (
            BridgeParallelProfitabilityClass::NotApplicable,
            BridgeParallelProfitabilityReason::SerialOnlyWorkload,
        ),
        BridgeParallelLegalityClass::ParallelPreparationLegal => {
            if packet_set.reduction_packets().len() != packet_set.routing_packets().len()
                || locality_footprint.publication_scope_count()
                    != packet_set.routing_packets().len()
            {
                (
                    BridgeParallelProfitabilityClass::Unprofitable,
                    BridgeParallelProfitabilityReason::SharedPublicationReductionTarget,
                )
            } else {
                (
                    BridgeParallelProfitabilityClass::Profitable,
                    BridgeParallelProfitabilityReason::AdmittedOperational,
                )
            }
        }
    };
    BridgeParallelProfitabilityDecision::new(class, reason)
}

fn classify_parallel_admission_components(
    legality_decision: &BridgeParallelLegalityDecision,
    profitability_decision: &BridgeParallelProfitabilityDecision,
) -> (BridgeParallelAdmissionClass, BridgeParallelAdmissionReason) {
    match legality_decision.class() {
        BridgeParallelLegalityClass::SerialOnly => (
            BridgeParallelAdmissionClass::SerialRequired,
            BridgeParallelAdmissionReason::BelowMinWorkloadWidth,
        ),
        BridgeParallelLegalityClass::ParallelPreparationIllegal => {
            let reason = match legality_decision.reason() {
                BridgeParallelLegalityReason::SharedTruthViewMaterializationTarget => {
                    BridgeParallelAdmissionReason::SharedTruthViewMaterializationTarget
                }
                BridgeParallelLegalityReason::ContinuityRemapRequiresSerialPreparation => {
                    BridgeParallelAdmissionReason::ContinuityRemapRequiresSerialPreparation
                }
                BridgeParallelLegalityReason::PacketRegionOverlapDetected => {
                    BridgeParallelAdmissionReason::PacketRegionOverlapDetected
                }
                BridgeParallelLegalityReason::BelowMinWorkloadWidth
                | BridgeParallelLegalityReason::DisjointPacketRegionsCertified => {
                    BridgeParallelAdmissionReason::SerialExecutor
                }
            };
            (
                BridgeParallelAdmissionClass::ParallelPreparationRejected,
                reason,
            )
        }
        BridgeParallelLegalityClass::ParallelPreparationLegal => {
            match profitability_decision.class() {
                BridgeParallelProfitabilityClass::Profitable => (
                    BridgeParallelAdmissionClass::ParallelPreparationAdmitted,
                    BridgeParallelAdmissionReason::AdmittedOperational,
                ),
                BridgeParallelProfitabilityClass::Unprofitable => (
                    BridgeParallelAdmissionClass::SerialRequired,
                    BridgeParallelAdmissionReason::SharedPublicationReductionTarget,
                ),
                BridgeParallelProfitabilityClass::NotApplicable => (
                    BridgeParallelAdmissionClass::SerialRequired,
                    BridgeParallelAdmissionReason::SerialExecutor,
                ),
            }
        }
    }
}

pub(super) fn decision_log(
    legality_decision: &BridgeParallelLegalityDecision,
    profitability_decision: &BridgeParallelProfitabilityDecision,
    parallel_admission: &BridgeParallelAdmission,
) -> BridgeBulkDecisionLog {
    BridgeBulkDecisionLog::new(vec![
        BridgeBulkDecisionRecord::new(
            BridgeBulkDecisionRecordKind::ParallelLegality,
            Arc::from(parallel_legality_class_label(legality_decision.class())),
            Arc::from(parallel_legality_reason_label(legality_decision.reason())),
        ),
        BridgeBulkDecisionRecord::new(
            BridgeBulkDecisionRecordKind::ParallelProfitability,
            Arc::from(parallel_profitability_class_label(
                profitability_decision.class(),
            )),
            Arc::from(parallel_profitability_reason_label(
                profitability_decision.reason(),
            )),
        ),
        BridgeBulkDecisionRecord::new(
            BridgeBulkDecisionRecordKind::ParallelAdmission,
            Arc::from(parallel_admission_class_label(parallel_admission.class())),
            Arc::from(parallel_admission_reason_label(parallel_admission.reason())),
        ),
    ])
}

pub(super) fn planning_failures(
    legality_decision: &BridgeParallelLegalityDecision,
    profitability_decision: &BridgeParallelProfitabilityDecision,
    parallel_admission: &BridgeParallelAdmission,
    packet_set: &PlannedBridgePacketSet,
    reduced_artifact: &ReducedBridgeWorkloadArtifact,
    locality_footprint: &BridgeLocalityFootprint,
    counters: &BridgeBulkPlanningCounters,
) -> Vec<BridgeBulkPlanningFailure> {
    let mut failures = Vec::new();
    if packet_set.routing_packets().is_empty() {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::ZeroRoutedItemWorkload,
            Arc::from("packet-planning"),
            Arc::from("bulk workload produced zero routed items"),
        ));
    }
    if matches!(
        profitability_decision.class(),
        BridgeParallelProfitabilityClass::Unprofitable
    ) {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::ParallelPreparationNotProfitable,
            Arc::from("parallel-profitability"),
            Arc::from(parallel_profitability_reason_label(
                profitability_decision.reason(),
            )),
        ));
    }
    if matches!(
        legality_decision.class(),
        BridgeParallelLegalityClass::ParallelPreparationIllegal
    ) && packet_set.routing_packets().len() > 1
    {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::InvalidParallelAdmissionBasis,
            Arc::from("parallel-legality"),
            Arc::from(parallel_legality_reason_label(legality_decision.reason())),
        ));
    }
    if matches!(
        parallel_admission.class(),
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted
    ) && locality_footprint.publication_scope_count() != packet_set.routing_packets().len()
    {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::PacketOverlapDetected,
            Arc::from("packet-overlap"),
            Arc::from("parallel-admitted packet regions collapsed onto shared publication scopes"),
        ));
    }
    if reduced_artifact.reduction_output_count() > reduced_artifact.reduction_input_count() {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::InvalidReductionBasis,
            Arc::from("reduction"),
            Arc::from(
                "bulk reduction emitted more outputs than the canonical reduction input basis admitted",
            ),
        ));
    }
    if reduction_identity_conflict_detected(reduced_artifact) {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::ReductionIdentityConflict,
            Arc::from("reduction-identity"),
            Arc::from("multiple reduced outputs claimed the same canonical reduction identity"),
        ));
    }
    if counters.bulk_reducer_input_buffer_peak() > bulk_reducer_input_buffer_ceiling() {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::ReducerBufferCeilingExceeded,
            Arc::from("reduction-buffer"),
            Arc::from("bulk reducer input buffer exceeded the bounded planning ceiling"),
        ));
    }
    if diagnostics_fragment_count(&failures) > bulk_diagnostics_fragment_ceiling() {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::DiagnosticsFragmentCeilingExceeded,
            Arc::from("diagnostics-fragments"),
            Arc::from("bulk diagnostics fragments exceeded the bounded planning ceiling"),
        ));
    }
    failures
}

fn reduction_identity_conflict_detected(reduced_artifact: &ReducedBridgeWorkloadArtifact) -> bool {
    has_duplicate_keys(
        reduced_artifact
            .reduced_publications()
            .iter()
            .map(|publication| publication.publication_identity().as_str()),
    ) || has_duplicate_keys(
        reduced_artifact
            .reduced_truth_views()
            .iter()
            .map(|truth_view| truth_view.truth_view_identity().as_str()),
    ) || has_duplicate_keys(
        reduced_artifact
            .reduced_continuity_remaps()
            .iter()
            .map(|continuity| continuity.continuity_identity().as_str()),
    ) || has_duplicate_keys(
        reduced_artifact
            .reduced_widenings()
            .iter()
            .map(|widening| widening.widening_identity().as_str()),
    )
}

fn has_duplicate_keys<'a>(keys: impl IntoIterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::<&'a str>::new();
    for key in keys {
        if !seen.insert(key) {
            return true;
        }
    }
    false
}

pub(super) fn locality_footprint(packet_set: &PlannedBridgePacketSet) -> BridgeLocalityFootprint {
    let mut branch_scopes =
        std::collections::BTreeSet::<crate::input::envelope::TruthBranchIdentity>::new();
    let mut snapshot_scopes =
        std::collections::BTreeSet::<crate::snapshot::TruthSnapshotIdentity>::new();
    let mut publication_scopes =
        std::collections::BTreeSet::<crate::routing::BridgeSubscriptionSliceIdentity>::new();
    for packet in packet_set.routing_packets() {
        branch_scopes.insert(packet.typed_source_branch().clone());
        snapshot_scopes.insert(packet.typed_source_snapshot().clone());
        publication_scopes.insert(packet.subscription_slice_identity().clone());
    }
    for packet in packet_set.truth_view_packets() {
        branch_scopes.insert(packet.typed_source_branch().clone());
        snapshot_scopes.insert(packet.typed_source_snapshot().clone());
    }
    for packet in packet_set.continuity_packets() {
        branch_scopes.insert(packet.typed_branch_identity().clone());
        snapshot_scopes.insert(packet.typed_snapshot_identity().clone());
    }
    BridgeLocalityFootprint::new(
        branch_scopes.len().max(1),
        snapshot_scopes.len().max(1),
        publication_scopes.len().max(1),
    )
}

pub(super) fn reduced_publication_packet_digest_basis(
    workload_identity: &BridgeWorkloadIdentity,
    subscription_slice_identity: &crate::routing::BridgeSubscriptionSliceIdentity,
    packets: &[&TruthDeltaRoutingPacket],
) -> String {
    let mut basis = format!(
        "reduced-publication|workload={}|subscription-slice={}|packet-count={}",
        workload_identity.as_str(),
        subscription_slice_identity.as_str(),
        packets.len(),
    );
    for packet in packets {
        basis.push_str("|packet=");
        basis.push_str(packet.packet_identity().as_str());
    }
    basis
}

use super::{labels::*, support::*, *};
