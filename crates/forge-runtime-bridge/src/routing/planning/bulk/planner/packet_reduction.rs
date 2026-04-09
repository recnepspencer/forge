pub(super) fn plan_packet_set(
    workload_identity: BridgeWorkloadIdentity,
    planned_routes: &[BridgePlannedRoute],
) -> PlannedBridgePacketSet {
    let routing_packets = planned_routes
        .iter()
        .enumerate()
        .map(|(packet_index, route)| {
            TruthDeltaRoutingPacket::new(
                workload_identity.clone(),
                Arc::<str>::from(route.route_identity().as_str().to_owned()),
                Arc::<str>::from(route.source_branch().as_str().to_owned()),
                Arc::<str>::from(route.source_commit().as_str().to_owned()),
                Arc::<str>::from(route.source_snapshot().as_str().to_owned()),
                Arc::<str>::from(
                    route
                        .lowering_summary()
                        .subscription_slice_identity()
                        .as_str()
                        .to_owned(),
                ),
                route.routing_summary().invalidation_target_count(),
                packet_index,
            )
        })
        .collect::<Vec<_>>();
    let mut truth_view_groups =
        std::collections::BTreeMap::<(Arc<str>, Arc<str>, Arc<str>), (usize, usize)>::new();
    for route in planned_routes {
        let key = (
            Arc::<str>::from(route.source_branch().as_str().to_owned()),
            Arc::<str>::from(route.source_commit().as_str().to_owned()),
            Arc::<str>::from(route.source_snapshot().as_str().to_owned()),
        );
        let entry = truth_view_groups.entry(key).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += route.planning_summary().snapshot_read_count();
    }
    let truth_view_packets = truth_view_groups
        .into_iter()
        .enumerate()
        .map(
            |(
                packet_index,
                (
                    (source_branch, source_commit, source_snapshot),
                    (planned_route_count, snapshot_read_count),
                ),
            )| {
                TruthViewMaterializationPacket::new(
                    workload_identity.clone(),
                    source_branch,
                    source_commit,
                    source_snapshot,
                    planned_route_count,
                    snapshot_read_count,
                    packet_index,
                )
            },
        )
        .collect::<Vec<_>>();
    let continuity_packets = planned_routes
        .iter()
        .filter_map(|route| {
            let lineage_context = route.mapping_context().lineage_context()?;
            let prior_slice_count = route.routing_summary().invalidation_target_count();
            Some((
                Arc::<str>::from(route.route_identity().as_str().to_owned()),
                Arc::<str>::from(lineage_context.authority_basis().digest().to_owned()),
                Arc::<str>::from(
                    lineage_context
                        .authority_basis()
                        .branch_identity()
                        .as_str()
                        .to_owned(),
                ),
                Arc::<str>::from(
                    lineage_context
                        .authority_basis()
                        .snapshot_identity()
                        .as_str()
                        .to_owned(),
                ),
                prior_slice_count,
            ))
        })
        .enumerate()
        .map(
            |(
                packet_index,
                (
                    route_identity,
                    continuity_authority_digest,
                    branch_identity,
                    snapshot_identity,
                    prior_slice_count,
                ),
            )| {
                ContinuityRemapPacket::new(
                    workload_identity.clone(),
                    route_identity,
                    continuity_authority_digest,
                    branch_identity,
                    snapshot_identity,
                    prior_slice_count,
                    packet_index,
                )
            },
        )
        .collect::<Vec<_>>();
    let fallback_packets = planned_routes
        .iter()
        .flat_map(|route| {
            route
                .route_record_entries()
                .iter()
                .filter_map(|entry| {
                    entry.fallback_class().map(|fallback_class| {
                        (
                            Arc::<str>::from(route.route_identity().as_str().to_owned()),
                            Arc::<str>::from(format!("{fallback_class:?}").to_lowercase()),
                            Arc::<str>::from(entry.truth_surface_identity().to_owned()),
                        )
                    })
                })
                .collect::<Vec<_>>()
        })
        .enumerate()
        .map(
            |(packet_index, (route_identity, fallback_class, bounded_scope_identity))| {
                FallbackAggregationPacket::new(
                    workload_identity.clone(),
                    route_identity,
                    fallback_class,
                    bounded_scope_identity,
                    packet_index,
                )
            },
        )
        .collect::<Vec<_>>();

    let mut reduction_groups = Vec::<(Arc<str>, Vec<&TruthDeltaRoutingPacket>)>::new();
    for packet in &routing_packets {
        let scope = Arc::<str>::from(packet.subscription_slice_identity().to_owned());
        if let Some((_, packets)) = reduction_groups
            .iter_mut()
            .find(|(existing_scope, _)| existing_scope.as_ref() == scope.as_ref())
        {
            packets.push(packet);
        } else {
            reduction_groups.push((scope, vec![packet]));
        }
    }
    reduction_groups.sort_by(|left, right| left.0.cmp(&right.0));
    let reduction_packets = reduction_groups
        .into_iter()
        .enumerate()
        .map(|(packet_index, (reduced_target_scope, packets))| {
            let reduced_target_identity = ReducedRoutingTargetIdentity::new(digest_string(
                "reduced-routing-target",
                &reduced_publication_packet_digest_basis(
                    &workload_identity,
                    reduced_target_scope.as_ref(),
                    &packets,
                ),
            ));
            InvalidationReductionPacket::new(
                workload_identity.clone(),
                Arc::from("publication"),
                reduced_target_scope,
                reduced_target_identity,
                packet_index,
            )
        })
        .collect::<Vec<_>>();

    PlannedBridgePacketSet::new(
        workload_identity,
        routing_packets,
        truth_view_packets,
        continuity_packets,
        fallback_packets,
        reduction_packets,
        BridgeBulkPlanningCounters::zero(),
    )
}

pub(super) fn reduce_packet_set(
    workload_identity: BridgeWorkloadIdentity,
    packet_set: &PlannedBridgePacketSet,
    counters: BridgeBulkPlanningCounters,
) -> ReducedBridgeWorkloadArtifact {
    let mut continuity_groups =
        std::collections::BTreeMap::<(Arc<str>, Arc<str>, Arc<str>), (usize, usize)>::new();
    for packet in packet_set.continuity_packets() {
        let key = (
            Arc::<str>::from(packet.continuity_authority_digest().to_owned()),
            Arc::<str>::from(packet.branch_identity().to_owned()),
            Arc::<str>::from(packet.snapshot_identity().to_owned()),
        );
        let entry = continuity_groups.entry(key).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += packet.prior_slice_count();
    }
    let reduced_continuity_remaps = continuity_groups
        .into_iter()
        .map(
            |(
                (continuity_authority_digest, branch_identity, snapshot_identity),
                (reduced_route_count, prior_slice_count),
            )| {
                let continuity_identity = ReducedContinuityIdentity::new(digest_string(
                    "reduced-continuity",
                    &format!(
                        "reduced-continuity|workload={}|authority={}|branch={}|snapshot={}",
                        workload_identity.as_str(),
                        continuity_authority_digest,
                        branch_identity,
                        snapshot_identity,
                    ),
                ));
                ReducedContinuityRemap::new(
                    continuity_identity,
                    continuity_authority_digest,
                    branch_identity,
                    snapshot_identity,
                    reduced_route_count,
                    prior_slice_count,
                )
            },
        )
        .collect::<Vec<_>>();
    let reduced_truth_views = packet_set
        .truth_view_packets()
        .iter()
        .map(|packet| {
            let truth_view_identity = ReducedTruthViewIdentity::new(digest_string(
                "reduced-truth-view",
                &format!(
                    "reduced-truth-view|workload={}|branch={}|commit={}|snapshot={}",
                    workload_identity.as_str(),
                    packet.source_branch(),
                    packet.source_commit(),
                    packet.source_snapshot(),
                ),
            ));
            ReducedTruthViewMaterialization::new(
                truth_view_identity,
                Arc::<str>::from(packet.source_branch().to_owned()),
                Arc::<str>::from(packet.source_commit().to_owned()),
                Arc::<str>::from(packet.source_snapshot().to_owned()),
                packet.planned_route_count(),
                packet.snapshot_read_count(),
            )
        })
        .collect::<Vec<_>>();
    let mut fallback_groups =
        std::collections::BTreeMap::<(Arc<str>, Arc<str>), Vec<Arc<str>>>::new();
    for packet in packet_set.fallback_packets() {
        fallback_groups
            .entry((
                Arc::<str>::from(packet.fallback_class().to_owned()),
                Arc::<str>::from(packet.bounded_scope_identity().to_owned()),
            ))
            .or_default()
            .push(Arc::<str>::from(
                packet.originating_route_identity().to_owned(),
            ));
    }
    let reduced_fallbacks = fallback_groups
        .into_iter()
        .map(
            |((fallback_class, bounded_scope_identity), mut reduced_route_identities)| {
                reduced_route_identities.sort();
                let fallback_identity = ReducedFallbackIdentity::new(digest_string(
                    "reduced-fallback",
                    &format!(
                        "reduced-fallback|workload={}|fallback-class={}|bounded-scope={}",
                        workload_identity.as_str(),
                        fallback_class,
                        bounded_scope_identity,
                    ),
                ));
                ReducedFallbackAggregation::new(
                    fallback_identity,
                    fallback_class,
                    bounded_scope_identity,
                    reduced_route_identities,
                )
            },
        )
        .collect::<Vec<_>>();
    let reduced_publications = packet_set
        .reduction_packets()
        .iter()
        .map(|packet| {
            let routes = packet_set
                .routing_packets()
                .iter()
                .filter(|route| route.subscription_slice_identity() == packet.reduced_target_scope())
                .collect::<Vec<_>>();
            let reduced_route_identities = routes
                .iter()
                .map(|route| Arc::<str>::from(route.route_identity().to_owned()))
                .collect::<Vec<_>>();
            let invalidation_target_count = routes
                .iter()
                .map(|route| route.invalidation_target_count())
                .sum();
            let publication_identity = ReducedPublicationIdentity::new(digest_string(
                "reduced-publication",
                &format!(
                    "reduced-publication|workload={}|routing-target={}|subscription-slice={}|route-count={}|invalidation-target-count={}",
                    workload_identity.as_str(),
                    packet.reduced_target_identity().as_str(),
                    packet.reduced_target_scope(),
                    reduced_route_identities.len(),
                    invalidation_target_count,
                ),
            ));
            ReducedBridgePublication::new(
                packet.reduced_target_identity().clone(),
                publication_identity,
                Arc::<str>::from(packet.reduced_target_scope().to_owned()),
                reduced_route_identities,
                invalidation_target_count,
            )
        })
        .collect::<Vec<_>>();

    ReducedBridgeWorkloadArtifact::new(
        workload_identity,
        reduced_continuity_remaps,
        reduced_truth_views,
        reduced_fallbacks,
        reduced_publications,
        counters,
    )
}
use super::{admission::*, *};
