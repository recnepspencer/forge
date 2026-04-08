#[derive(Debug, Clone)]
struct BridgeBulkPlanningBasis {
    packet_set: PlannedBridgePacketSet,
    reduced_artifact: ReducedBridgeWorkloadArtifact,
    counters: BridgeBulkPlanningCounters,
    locality_footprint: BridgeLocalityFootprint,
    legality_decision: BridgeParallelLegalityDecision,
    profitability_decision: BridgeParallelProfitabilityDecision,
    parallel_admission: BridgeParallelAdmission,
    selected_mode: BridgePreparationMode,
    planning_failures: Vec<BridgeBulkPlanningFailure>,
}

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
    let planning_basis =
        derive_planning_basis(workload_identity.clone(), &normalized_summary, packet_set);
    let execution_plan = admitted_execution_plan(
        workload_identity.clone(),
        canonical_planning_identity.clone(),
        admission_profile_identity.clone(),
        planning_basis.clone(),
    );

    Ok(BridgeBulkWorkloadPlan::new(
        request,
        workload_identity,
        canonical_request,
        normalized_summary,
        canonical_planning_identity,
        admission_profile_identity,
        planning_basis.packet_set,
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
        .map(|route| {
            Arc::<str>::from(
                route.lowering_summary()
                    .subscription_slice_identity()
                    .as_str()
                    .to_owned(),
            )
        })
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
        BridgeBulkPlanningCounters::zero(),
    )
}

fn derive_planning_basis(
    workload_identity: BridgeWorkloadIdentity,
    normalized_summary: &NormalizedBridgeWorkloadSummary,
    packet_set: PlannedBridgePacketSet,
) -> BridgeBulkPlanningBasis {
    let locality_footprint = locality_footprint(&packet_set);
    let legality_decision = classify_parallel_legality(&packet_set);
    let profitability_decision =
        classify_parallel_profitability(&packet_set, &legality_decision, &locality_footprint);
    let (selected_mode, admission_class, admission_reason) =
        classify_parallel_admission(&legality_decision, &profitability_decision);
    let counters = planning_counters(
        normalized_summary,
        &packet_set,
        legality_decision.class(),
        profitability_decision.class(),
        admission_class,
    );
    let packet_set = packet_set.with_counters(counters.clone());
    let reduced_artifact = reduce_packet_set(workload_identity, &packet_set, counters.clone());
    let parallel_admission = BridgeParallelAdmission::new(admission_class, admission_reason);
    let planning_failures = planning_failures(
        &legality_decision,
        &profitability_decision,
        &parallel_admission,
        &packet_set,
        &reduced_artifact,
        &locality_footprint,
        &counters,
    );

    BridgeBulkPlanningBasis {
        packet_set,
        reduced_artifact,
        counters,
        locality_footprint,
        legality_decision,
        profitability_decision,
        parallel_admission,
        selected_mode,
        planning_failures,
    }
}

fn admitted_execution_plan(
    workload_identity: BridgeWorkloadIdentity,
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    admission_profile_identity: BridgeAdmissionProfileIdentity,
    planning_basis: BridgeBulkPlanningBasis,
) -> AdmittedBridgeExecutionPlan {
    let route_regions = route_region_keys(&planning_basis.packet_set);
    let disjoint_packet_regions = if matches!(
        planning_basis.parallel_admission.class(),
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted
    ) {
        DisjointPacketRegionSet::new(route_regions.clone())
    } else {
        DisjointPacketRegionSet::new(Vec::new())
    };
    let admitted_partitions = if matches!(
        planning_basis.parallel_admission.class(),
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
    let decision_log = decision_log(
        &planning_basis.legality_decision,
        &planning_basis.profitability_decision,
        &planning_basis.parallel_admission,
    );

    AdmittedBridgeExecutionPlan::new(
        workload_identity,
        canonical_planning_identity,
        admission_profile_identity,
        planning_basis.reduced_artifact,
        planning_basis.counters,
        planning_basis.locality_footprint,
        planning_basis.selected_mode,
        planning_basis.legality_decision,
        planning_basis.profitability_decision,
        planning_basis.parallel_admission,
        legality_proof,
        decision_log,
        planning_basis.planning_failures,
    )
}
use super::{admission::*, packet_reduction::*, support::*, *};
