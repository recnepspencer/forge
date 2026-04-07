use std::sync::Arc;

use crate::error::{BridgeRouteError, BridgeRouteErrorKind};
use crate::facade::{BridgeDiagnosticsTier, RuntimeBridge};
use crate::mapping::{CoarseRoutingMode, FrozenMappingRegistry, MappingSelector};
use crate::routing::canonicalization::digest_string;
use crate::routing::planning::BridgePlannedRoute;

use super::types::*;

pub(crate) fn plan_bulk_workload(
    runtime: &RuntimeBridge,
    request: BridgeBulkWorkloadRequest,
) -> Result<BridgeBulkWorkloadPlan, BridgeRouteError> {
    if request.segments().is_empty() {
        return Err(BridgeRouteError::new(
            BridgeRouteErrorKind::EmptyBulkWorkloadRequest,
            "Bulk bridge planning requires at least one workload segment.",
        ));
    }

    let mut planned_routes: Vec<BridgePlannedRoute> = request
        .segments()
        .iter()
        .map(|segment| {
            runtime.plan_committed_patch_with_mapping_context(
                segment.request().clone(),
                segment.mapping_context().clone(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    planned_routes.sort_by(|left, right| left.route_identity().cmp(right.route_identity()));

    let workload_identity = BridgeWorkloadIdentity::new(digest_string(
        "bulk-workload",
        &bulk_workload_digest_basis(&planned_routes),
    ));
    let canonical_request =
        canonical_workload_request(workload_identity.clone(), request.segments(), &planned_routes);
    let normalized_summary = normalized_workload_summary(&canonical_request, &planned_routes);
    let canonical_planning_identity = BridgeCanonicalPlanningIdentity::new(digest_string(
        "bulk-planning-identity",
        &canonical_planning_digest_basis(runtime, &workload_identity, &planned_routes),
    ));
    let admission_profile_identity = BridgeAdmissionProfileIdentity::new(digest_string(
        "bulk-admission-profile",
        &admission_profile_digest_basis(runtime),
    ));
    let summary = BridgeBulkPlanningSummary::new(
        workload_identity.clone(),
        planned_routes.len(),
        planned_routes
            .iter()
            .map(|route| route.routing_summary().invalidation_target_count())
            .sum(),
        planned_routes
            .iter()
            .map(|route| route.planning_summary().subscription_slice_count())
            .sum(),
        planned_routes
            .iter()
            .map(|route| route.planning_summary().snapshot_read_count())
            .sum(),
    );
    let packet_set = plan_packet_set(workload_identity.clone(), &planned_routes);
    let execution_plan = admitted_execution_plan(
        workload_identity.clone(),
        canonical_planning_identity.clone(),
        admission_profile_identity.clone(),
        packet_set.clone(),
    );

    Ok(BridgeBulkWorkloadPlan::new(
        request,
        workload_identity,
        canonical_request,
        normalized_summary,
        canonical_planning_identity,
        admission_profile_identity,
        packet_set,
        execution_plan,
        planned_routes,
        summary,
    ))
}

fn canonical_workload_request(
    workload_identity: BridgeWorkloadIdentity,
    segments: &[BridgeBulkWorkloadSegment],
    planned_routes: &[BridgePlannedRoute],
) -> CanonicalBridgeWorkloadRequest {
    let route_members = planned_routes
        .iter()
        .map(|route| Arc::<str>::from(route.route_identity().as_str().to_owned()))
        .collect::<Vec<_>>();
    let mut subscription_slice_members = planned_routes
        .iter()
        .map(|route| Arc::<str>::from(route.lowering_summary().subscription_slice_identity().as_str().to_owned()))
        .collect::<Vec<_>>();
    subscription_slice_members.sort();
    subscription_slice_members.dedup();
    let mut continuity_members = planned_routes
        .iter()
        .filter_map(|route| {
            route.mapping_context().lineage_context().map(|lineage_context| {
                Arc::<str>::from(lineage_context.authority_basis().digest().to_owned())
            })
        })
        .collect::<Vec<_>>();
    continuity_members.sort();
    continuity_members.dedup();
    let mut truth_view_members = planned_routes
        .iter()
        .map(|route| {
            Arc::<str>::from(format!(
                "{}:{}:{}",
                route.source_branch().as_str(),
                route.source_snapshot().as_str(),
                route.source_commit().as_str()
            ))
        })
        .collect::<Vec<_>>();
    truth_view_members.sort();
    truth_view_members.dedup();
    let mut commit_members = planned_routes
        .iter()
        .map(|route| Arc::<str>::from(route.source_commit().as_str().to_owned()))
        .collect::<Vec<_>>();
    commit_members.sort();
    commit_members.dedup();
    let mut snapshot_members = planned_routes
        .iter()
        .map(|route| Arc::<str>::from(route.source_snapshot().as_str().to_owned()))
        .collect::<Vec<_>>();
    snapshot_members.sort();
    snapshot_members.dedup();
    let mut branch_members = planned_routes
        .iter()
        .map(|route| Arc::<str>::from(route.source_branch().as_str().to_owned()))
        .collect::<Vec<_>>();
    branch_members.sort();
    branch_members.dedup();
    let mut workload_segment_digests = segments
        .iter()
        .map(|segment| {
            Arc::<str>::from(format!(
                "segment|commit={}|mapping-context={}",
                segment.request().commit_identity(),
                segment.mapping_context().digest()
            ))
        })
        .collect::<Vec<_>>();
    workload_segment_digests.sort();

    CanonicalBridgeWorkloadRequest::new(
        workload_identity,
        route_members,
        subscription_slice_members,
        continuity_members,
        truth_view_members,
        commit_members,
        snapshot_members,
        branch_members,
        workload_segment_digests,
    )
}

fn normalized_workload_summary(
    canonical_request: &CanonicalBridgeWorkloadRequest,
    planned_routes: &[BridgePlannedRoute],
) -> NormalizedBridgeWorkloadSummary {
    let mut branch_scopes = std::collections::BTreeSet::<Arc<str>>::new();
    let mut snapshot_scopes = std::collections::BTreeSet::<Arc<str>>::new();
    for route in planned_routes {
        branch_scopes.insert(Arc::<str>::from(route.source_branch().as_str().to_owned()));
        snapshot_scopes.insert(Arc::<str>::from(route.source_snapshot().as_str().to_owned()));
    }
    let counters = BridgeBulkPlanningCounters::new(
        planned_routes.len(),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        BridgeParallelLegalityClass::SerialOnly,
        BridgeParallelProfitabilityClass::NotApplicable,
        BridgeParallelAdmissionClass::SerialRequired,
    );
    NormalizedBridgeWorkloadSummary::new(
        canonical_request.workload_identity().clone(),
        planned_routes.len(),
        planned_routes
            .iter()
            .map(|route| route.routing_summary().invalidation_target_count())
            .sum(),
        canonical_request.subscription_slice_members().len(),
        planned_routes
            .iter()
            .map(|route| route.planning_summary().snapshot_read_count())
            .sum(),
        canonical_request.truth_view_members().len(),
        canonical_request.continuity_members().len(),
        branch_scopes.len().max(1),
        snapshot_scopes.len().max(1),
        counters,
    )
}

fn admitted_execution_plan(
    workload_identity: BridgeWorkloadIdentity,
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    admission_profile_identity: BridgeAdmissionProfileIdentity,
    packet_set: PlannedBridgePacketSet,
) -> AdmittedBridgeExecutionPlan {
    let locality_footprint = locality_footprint(&packet_set);
    let legality_decision = classify_parallel_legality(&packet_set);
    let profitability_decision =
        classify_parallel_profitability(&packet_set, &legality_decision, &locality_footprint);
    let (selected_mode, class, reason) =
        classify_parallel_admission(&legality_decision, &profitability_decision);
    let counters = BridgeBulkPlanningCounters::new(
        packet_set.routing_packets().len(),
        packet_set.routing_packets().len()
            + packet_set.truth_view_packets().len()
            + packet_set.continuity_packets().len()
            + packet_set.fallback_packets().len()
            + packet_set.reduction_packets().len(),
        packet_set.routing_packets().len()
            + packet_set.truth_view_packets().len()
            + packet_set.continuity_packets().len()
            + packet_set.fallback_packets().len(),
        packet_set.routing_packets().len()
            + packet_set.truth_view_packets().len()
            + packet_set.continuity_packets().len(),
        packet_set.reduction_packets().len()
            + packet_set.truth_view_packets().len()
            + packet_set.continuity_packets().len(),
        packet_set.reduction_packets().len()
            + packet_set.truth_view_packets().len()
            + packet_set.continuity_packets().len(),
        packet_set.fallback_packets().len(),
        0,
        legality_decision.class(),
        profitability_decision.class(),
        class,
    );
    let reduced_artifact = reduce_packet_set(workload_identity.clone(), &packet_set, counters.clone());
    let route_regions = route_region_keys(&packet_set);
    let disjoint_packet_regions = if matches!(
        class,
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted
    ) {
        DisjointPacketRegionSet::new(route_regions.clone())
    } else {
        DisjointPacketRegionSet::new(Vec::new())
    };
    let admitted_partitions = if matches!(
        class,
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted
    ) {
        AdmittedPreparationPartitionSet::new(route_regions)
    } else {
        AdmittedPreparationPartitionSet::new(Vec::new())
    };
    let legality_proof = ParallelPreparationLegalityProof::new(
        canonical_planning_identity.clone(),
        disjoint_packet_regions,
        admitted_partitions,
    );
    let parallel_admission = BridgeParallelAdmission::new(class, reason);
    let decision_log =
        decision_log(&legality_decision, &profitability_decision, &parallel_admission);
    let planning_failures =
        planning_failures(&legality_decision, &profitability_decision, &packet_set);

    AdmittedBridgeExecutionPlan::new(
        workload_identity,
        canonical_planning_identity,
        admission_profile_identity,
        reduced_artifact,
        counters,
        locality_footprint,
        selected_mode,
        legality_decision,
        profitability_decision,
        parallel_admission,
        legality_proof,
        decision_log,
        planning_failures,
    )
}

fn plan_packet_set(
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
                    route.lowering_summary()
                        .subscription_slice_identity()
                        .as_str()
                        .to_owned(),
                ),
                route.routing_summary().invalidation_target_count(),
                packet_index,
            )
        })
        .collect::<Vec<_>>();
    let mut truth_view_groups = std::collections::BTreeMap::<
        (Arc<str>, Arc<str>, Arc<str>),
        (usize, usize),
    >::new();
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
            |(packet_index, ((source_branch, source_commit, source_snapshot), (planned_route_count, snapshot_read_count)))| {
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
                Arc::<str>::from(lineage_context.authority_basis().branch_identity().as_str().to_owned()),
                Arc::<str>::from(lineage_context.authority_basis().snapshot_identity().as_str().to_owned()),
                prior_slice_count,
            ))
        })
        .enumerate()
        .map(
            |(packet_index, (route_identity, continuity_authority_digest, branch_identity, snapshot_identity, prior_slice_count))| {
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
        .map(|(packet_index, (route_identity, fallback_class, bounded_scope_identity))| {
            FallbackAggregationPacket::new(
                workload_identity.clone(),
                route_identity,
                fallback_class,
                bounded_scope_identity,
                packet_index,
            )
        })
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
            let reduced_target_identity = ReducedPublicationIdentity::new(digest_string(
                "reduced-publication",
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

    let provisional_packet_set = PlannedBridgePacketSet::new(
        workload_identity.clone(),
        routing_packets,
        truth_view_packets,
        continuity_packets,
        fallback_packets,
        reduction_packets,
        BridgeBulkPlanningCounters::new(
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            BridgeParallelLegalityClass::SerialOnly,
            BridgeParallelProfitabilityClass::NotApplicable,
            BridgeParallelAdmissionClass::SerialRequired,
        ),
    );
    let locality_footprint = locality_footprint(&provisional_packet_set);
    let legality_decision = classify_parallel_legality(&provisional_packet_set);
    let profitability_decision =
        classify_parallel_profitability(&provisional_packet_set, &legality_decision, &locality_footprint);
    let (_, admission_class, _) =
        classify_parallel_admission(&legality_decision, &profitability_decision);
    let counters = BridgeBulkPlanningCounters::new(
        provisional_packet_set.routing_packets().len(),
        provisional_packet_set.routing_packets().len()
            + provisional_packet_set.truth_view_packets().len()
            + provisional_packet_set.continuity_packets().len()
            + provisional_packet_set.fallback_packets().len()
            + provisional_packet_set.reduction_packets().len(),
        provisional_packet_set.routing_packets().len()
            + provisional_packet_set.truth_view_packets().len()
            + provisional_packet_set.continuity_packets().len()
            + provisional_packet_set.fallback_packets().len(),
        provisional_packet_set.routing_packets().len()
            + provisional_packet_set.truth_view_packets().len()
            + provisional_packet_set.continuity_packets().len(),
        provisional_packet_set.reduction_packets().len()
            + provisional_packet_set.truth_view_packets().len()
            + provisional_packet_set.continuity_packets().len(),
        provisional_packet_set.reduction_packets().len()
            + provisional_packet_set.truth_view_packets().len()
            + provisional_packet_set.continuity_packets().len(),
        provisional_packet_set.fallback_packets().len(),
        0,
        legality_decision.class(),
        profitability_decision.class(),
        admission_class,
    );

    PlannedBridgePacketSet::new(
        provisional_packet_set.workload_identity().clone(),
        provisional_packet_set.routing_packets().to_vec(),
        provisional_packet_set.truth_view_packets().to_vec(),
        provisional_packet_set.continuity_packets().to_vec(),
        provisional_packet_set.fallback_packets().to_vec(),
        provisional_packet_set.reduction_packets().to_vec(),
        counters,
    )
}

fn reduce_packet_set(
    workload_identity: BridgeWorkloadIdentity,
    packet_set: &PlannedBridgePacketSet,
    counters: BridgeBulkPlanningCounters,
) -> ReducedBridgeWorkloadArtifact {
    let mut continuity_groups = std::collections::BTreeMap::<
        (Arc<str>, Arc<str>, Arc<str>),
        (usize, usize),
    >::new();
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
            |((continuity_authority_digest, branch_identity, snapshot_identity), (reduced_route_count, prior_slice_count))| {
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
            ReducedBridgePublication::new(
                packet.reduced_target_identity().clone(),
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
        reduced_publications,
        counters,
    )
}

fn bulk_workload_digest_basis(planned_routes: &[BridgePlannedRoute]) -> String {
    let mut basis = format!("bulk-workload|route-count={}", planned_routes.len());
    for route in planned_routes {
        basis.push_str("|route=");
        basis.push_str(route.route_identity().as_str());
        basis.push_str("|source-digest=");
        basis.push_str(route.source_digest().as_str());
        basis.push_str("|mapping-context=");
        basis.push_str(route.mapping_context().digest());
    }
    basis
}

fn canonical_planning_digest_basis(
    runtime: &RuntimeBridge,
    workload_identity: &BridgeWorkloadIdentity,
    planned_routes: &[BridgePlannedRoute],
) -> String {
    let mut basis = format!(
        "bulk-planning-identity|workload={}|mapping-registry={}",
        workload_identity.as_str(),
        mapping_registry_digest(&runtime.mapping_registry),
    );
    for route in planned_routes {
        basis.push_str("|route=");
        basis.push_str(route.route_identity().as_str());
        basis.push_str("|planning=");
        basis.push_str(route.planning_provenance().digest());
        basis.push_str("|lowering=");
        basis.push_str(route.lowering_provenance().digest());
        basis.push_str("|truth-view=");
        basis.push_str(route.source_branch().as_str());
        basis.push(':');
        basis.push_str(route.source_snapshot().as_str());
        basis.push(':');
        basis.push_str(route.source_commit().as_str());
        if let Some(lineage_context) = route.mapping_context().lineage_context() {
            basis.push_str("|continuity=");
            basis.push_str(lineage_context.authority_basis().digest());
        }
    }
    basis.push_str("|packetization-semantics=v1|reduction-semantics=v1");
    basis
}

fn admission_profile_digest_basis(runtime: &RuntimeBridge) -> String {
    format!(
        "bulk-admission-profile|diagnostics-tier={}|route-record-limit={}|failure-record-limit={}|allow-replay={}|has-branch-head-source={}|has-lineage-source={}|has-reader-pool={}",
        diagnostics_tier_label(runtime.policy.diagnostics_tier()),
        runtime.policy.retention_budget().route_record_limit(),
        runtime.policy.retention_budget().failure_record_limit(),
        runtime.policy.allow_replay_artifacts(),
        runtime.truth_branch_head_source.is_some(),
        runtime.continuity_lineage_source.is_some(),
        runtime.snapshot_reader_pool.is_some(),
    )
}

fn mapping_registry_digest(registry: &FrozenMappingRegistry) -> Arc<str> {
    let mut basis = format!("mapping-registry|registration-count={}", registry.registrations().len());
    for registration in registry.registrations() {
        basis.push_str("|registration=");
        basis.push_str(registration.mapping_id().as_str());
        basis.push(':');
        basis.push_str(selector_label(registration.truth_scope().entity_selector()).as_ref());
        basis.push(':');
        basis.push_str(selector_label(registration.truth_scope().aspect_selector()).as_ref());
        basis.push(':');
        basis.push_str(selector_label(registration.truth_scope().surface_selector()).as_ref());
        basis.push(':');
        basis.push_str(registration.signal_scope().as_str());
        basis.push(':');
        basis.push_str(routing_mode_label(registration.routing_mode()));
    }
    digest_string("mapping-registry", &basis)
}

fn selector_label(selector: &MappingSelector) -> Arc<str> {
    match selector {
        MappingSelector::Any => Arc::from("*"),
        MappingSelector::Exact(value) => Arc::clone(value),
    }
}

fn routing_mode_label(mode: CoarseRoutingMode) -> &'static str {
    match mode {
        CoarseRoutingMode::Direct => "direct",
    }
}

fn diagnostics_tier_label(tier: BridgeDiagnosticsTier) -> &'static str {
    match tier {
        BridgeDiagnosticsTier::Minimal => "minimal",
        BridgeDiagnosticsTier::Standard => "standard",
        BridgeDiagnosticsTier::Exhaustive => "exhaustive",
    }
}

fn route_region_keys(packet_set: &PlannedBridgePacketSet) -> Vec<Arc<str>> {
    let mut regions = packet_set
        .routing_packets()
        .iter()
        .map(|route| {
            Arc::<str>::from(format!(
                "route-partition:{}:{}:{}",
                route.route_identity(),
                route.source_snapshot(),
                route.subscription_slice_identity(),
            ))
        })
        .collect::<Vec<_>>();
    regions.extend(packet_set.truth_view_packets().iter().map(|packet| {
        Arc::<str>::from(format!(
            "truth-view-partition:{}:{}:{}",
            packet.source_branch(),
            packet.source_snapshot(),
            packet.source_commit(),
        ))
    }));
    regions.extend(packet_set.continuity_packets().iter().map(|packet| {
        Arc::<str>::from(format!(
            "continuity-partition:{}:{}:{}",
            packet.continuity_authority_digest(),
            packet.branch_identity(),
            packet.snapshot_identity(),
        ))
    }));
    regions
}

fn classify_parallel_admission(
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

fn classify_parallel_legality(
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
    } else {
        (
            BridgeParallelLegalityClass::ParallelPreparationLegal,
            BridgeParallelLegalityReason::DisjointPacketRegionsCertified,
        )
    };
    BridgeParallelLegalityDecision::new(class, reason)
}

fn classify_parallel_profitability(
    packet_set: &PlannedBridgePacketSet,
    legality_decision: &BridgeParallelLegalityDecision,
    locality_footprint: &BridgeLocalityFootprint,
) -> BridgeParallelProfitabilityDecision {
    let (class, reason) = match legality_decision.class() {
        BridgeParallelLegalityClass::SerialOnly | BridgeParallelLegalityClass::ParallelPreparationIllegal => (
            BridgeParallelProfitabilityClass::NotApplicable,
            BridgeParallelProfitabilityReason::SerialOnlyWorkload,
        ),
        BridgeParallelLegalityClass::ParallelPreparationLegal => {
            if packet_set.reduction_packets().len() != packet_set.routing_packets().len()
                || locality_footprint.publication_scope_count() != packet_set.routing_packets().len()
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
                BridgeParallelLegalityReason::BelowMinWorkloadWidth
                | BridgeParallelLegalityReason::DisjointPacketRegionsCertified => {
                    BridgeParallelAdmissionReason::SerialExecutor
                }
            };
            (BridgeParallelAdmissionClass::ParallelPreparationRejected, reason)
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

fn decision_log(
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

fn planning_failures(
    legality_decision: &BridgeParallelLegalityDecision,
    profitability_decision: &BridgeParallelProfitabilityDecision,
    packet_set: &PlannedBridgePacketSet,
) -> Vec<BridgeBulkPlanningFailure> {
    let mut failures = Vec::new();
    if packet_set.routing_packets().is_empty() {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::WorkloadSummaryConstructionFailure,
            Arc::from("normalized-workload-summary"),
            Arc::from("bulk workload produced zero routed items"),
        ));
    }
    if matches!(
        profitability_decision.class(),
        BridgeParallelProfitabilityClass::Unprofitable
    ) {
        failures.push(BridgeBulkPlanningFailure::new(
            BridgeBulkPlanningFailureKind::LegalButUnprofitableParallelFallback,
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
            BridgeBulkPlanningFailureKind::InvalidLegalityBasis,
            Arc::from("parallel-legality"),
            Arc::from(parallel_legality_reason_label(legality_decision.reason())),
        ));
    }
    failures
}

fn locality_footprint(packet_set: &PlannedBridgePacketSet) -> BridgeLocalityFootprint {
    let mut branch_scopes = std::collections::BTreeSet::<Arc<str>>::new();
    let mut snapshot_scopes = std::collections::BTreeSet::<Arc<str>>::new();
    let mut publication_scopes = std::collections::BTreeSet::<Arc<str>>::new();
    for packet in packet_set.routing_packets() {
        branch_scopes.insert(Arc::<str>::from(packet.source_branch().to_owned()));
        snapshot_scopes.insert(Arc::<str>::from(packet.source_snapshot().to_owned()));
        publication_scopes.insert(Arc::<str>::from(packet.subscription_slice_identity().to_owned()));
    }
    for packet in packet_set.truth_view_packets() {
        branch_scopes.insert(Arc::<str>::from(packet.source_branch().to_owned()));
        snapshot_scopes.insert(Arc::<str>::from(packet.source_snapshot().to_owned()));
    }
    for packet in packet_set.continuity_packets() {
        branch_scopes.insert(Arc::<str>::from(packet.branch_identity().to_owned()));
        snapshot_scopes.insert(Arc::<str>::from(packet.snapshot_identity().to_owned()));
    }
    BridgeLocalityFootprint::new(
        branch_scopes.len().max(1),
        snapshot_scopes.len().max(1),
        publication_scopes.len(),
    )
}

fn reduced_publication_packet_digest_basis(
    workload_identity: &BridgeWorkloadIdentity,
    subscription_slice_identity: &str,
    packets: &[&TruthDeltaRoutingPacket],
) -> String {
    let mut basis = format!(
        "reduced-publication|workload={}|subscription-slice={}|packet-count={}",
        workload_identity.as_str(),
        subscription_slice_identity,
        packets.len(),
    );
    for packet in packets {
        basis.push_str("|packet=");
        basis.push_str(packet.packet_identity().as_str());
    }
    basis
}

pub(super) fn parallel_legality_class_label(class: BridgeParallelLegalityClass) -> &'static str {
    match class {
        BridgeParallelLegalityClass::SerialOnly => "serial-only",
        BridgeParallelLegalityClass::ParallelPreparationLegal => "parallel-preparation-legal",
        BridgeParallelLegalityClass::ParallelPreparationIllegal => "parallel-preparation-illegal",
    }
}

pub(super) fn parallel_legality_reason_label(reason: BridgeParallelLegalityReason) -> &'static str {
    match reason {
        BridgeParallelLegalityReason::BelowMinWorkloadWidth => "below-min-workload-width",
        BridgeParallelLegalityReason::SharedTruthViewMaterializationTarget => {
            "shared-truth-view-materialization-target"
        }
        BridgeParallelLegalityReason::ContinuityRemapRequiresSerialPreparation => {
            "continuity-remap-requires-serial-preparation"
        }
        BridgeParallelLegalityReason::DisjointPacketRegionsCertified => {
            "disjoint-packet-regions-certified"
        }
    }
}

pub(super) fn parallel_profitability_class_label(class: BridgeParallelProfitabilityClass) -> &'static str {
    match class {
        BridgeParallelProfitabilityClass::NotApplicable => "not-applicable",
        BridgeParallelProfitabilityClass::Profitable => "profitable",
        BridgeParallelProfitabilityClass::Unprofitable => "unprofitable",
    }
}

pub(super) fn parallel_profitability_reason_label(reason: BridgeParallelProfitabilityReason) -> &'static str {
    match reason {
        BridgeParallelProfitabilityReason::SerialOnlyWorkload => "serial-only-workload",
        BridgeParallelProfitabilityReason::SharedPublicationReductionTarget => {
            "shared-publication-reduction-target"
        }
        BridgeParallelProfitabilityReason::AdmittedOperational => "admitted-operational",
    }
}

pub(super) fn parallel_admission_class_label(class: BridgeParallelAdmissionClass) -> &'static str {
    match class {
        BridgeParallelAdmissionClass::SerialRequired => "serial-required",
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted => {
            "parallel-preparation-admitted"
        }
        BridgeParallelAdmissionClass::ParallelPreparationRejected => {
            "parallel-preparation-rejected"
        }
    }
}

pub(super) fn parallel_admission_reason_label(reason: BridgeParallelAdmissionReason) -> &'static str {
    match reason {
        BridgeParallelAdmissionReason::SerialExecutor => "serial-executor",
        BridgeParallelAdmissionReason::BelowMinWorkloadWidth => "below-min-workload-width",
        BridgeParallelAdmissionReason::SharedPublicationReductionTarget => {
            "shared-publication-reduction-target"
        }
        BridgeParallelAdmissionReason::SharedTruthViewMaterializationTarget => {
            "shared-truth-view-materialization-target"
        }
        BridgeParallelAdmissionReason::ContinuityRemapRequiresSerialPreparation => {
            "continuity-remap-requires-serial-preparation"
        }
        BridgeParallelAdmissionReason::AdmittedOperational => "admitted-operational",
    }
}

pub(super) fn bulk_decision_kind_label(kind: BridgeBulkDecisionRecordKind) -> &'static str {
    match kind {
        BridgeBulkDecisionRecordKind::ParallelLegality => "parallel-legality",
        BridgeBulkDecisionRecordKind::ParallelProfitability => "parallel-profitability",
        BridgeBulkDecisionRecordKind::ParallelAdmission => "parallel-admission",
    }
}

pub(super) fn planning_failure_kind_label(kind: BridgeBulkPlanningFailureKind) -> &'static str {
    match kind {
        BridgeBulkPlanningFailureKind::WorkloadSummaryConstructionFailure => {
            "workload-summary-construction-failure"
        }
        BridgeBulkPlanningFailureKind::UnsupportedPacketClass => "unsupported-packet-class",
        BridgeBulkPlanningFailureKind::InvalidLegalityBasis => "invalid-legality-basis",
        BridgeBulkPlanningFailureKind::LegalButUnprofitableParallelFallback => {
            "legal-but-unprofitable-parallel-fallback"
        }
    }
}

pub(super) fn preparation_mode_label(mode: BridgePreparationMode) -> &'static str {
    match mode {
        BridgePreparationMode::Serial => "serial",
        BridgePreparationMode::ParallelPreparation => "parallel-preparation",
    }
}

