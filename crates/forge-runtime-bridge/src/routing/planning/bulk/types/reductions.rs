#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedContinuityRemap {
    continuity_identity: ReducedContinuityIdentity,
    continuity_authority_digest: Arc<str>,
    branch_identity: Arc<str>,
    snapshot_identity: Arc<str>,
    reduced_route_count: usize,
    prior_slice_count: usize,
    digest: Arc<str>,
}

impl ReducedContinuityRemap {
    pub(crate) fn new(
        continuity_identity: ReducedContinuityIdentity,
        continuity_authority_digest: Arc<str>,
        branch_identity: Arc<str>,
        snapshot_identity: Arc<str>,
        reduced_route_count: usize,
        prior_slice_count: usize,
    ) -> Self {
        let basis = format!(
            "reduced-continuity-remap|identity={}|authority={}|branch={}|snapshot={}|reduced-route-count={}|prior-slice-count={}",
            continuity_identity.as_str(),
            continuity_authority_digest,
            branch_identity,
            snapshot_identity,
            reduced_route_count,
            prior_slice_count,
        );
        Self {
            continuity_identity,
            continuity_authority_digest,
            branch_identity,
            snapshot_identity,
            reduced_route_count,
            prior_slice_count,
            digest: digest_string("reduced-continuity-remap", &basis),
        }
    }

    pub fn continuity_identity(&self) -> &ReducedContinuityIdentity {
        &self.continuity_identity
    }
    pub fn continuity_authority_digest(&self) -> &str {
        self.continuity_authority_digest.as_ref()
    }
    pub fn branch_identity(&self) -> &str {
        self.branch_identity.as_ref()
    }
    pub fn snapshot_identity(&self) -> &str {
        self.snapshot_identity.as_ref()
    }
    pub fn reduced_route_count(&self) -> usize {
        self.reduced_route_count
    }
    pub fn prior_slice_count(&self) -> usize {
        self.prior_slice_count
    }
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedTruthViewMaterialization {
    truth_view_identity: ReducedTruthViewIdentity,
    source_branch: Arc<str>,
    source_commit: Arc<str>,
    source_snapshot: Arc<str>,
    planned_route_count: usize,
    snapshot_read_count: usize,
    digest: Arc<str>,
}

impl ReducedTruthViewMaterialization {
    pub(crate) fn new(
        truth_view_identity: ReducedTruthViewIdentity,
        source_branch: Arc<str>,
        source_commit: Arc<str>,
        source_snapshot: Arc<str>,
        planned_route_count: usize,
        snapshot_read_count: usize,
    ) -> Self {
        let basis = format!(
            "reduced-truth-view-materialization|identity={}|branch={}|commit={}|snapshot={}|planned-route-count={}|snapshot-read-count={}",
            truth_view_identity.as_str(),
            source_branch,
            source_commit,
            source_snapshot,
            planned_route_count,
            snapshot_read_count,
        );
        Self {
            truth_view_identity,
            source_branch,
            source_commit,
            source_snapshot,
            planned_route_count,
            snapshot_read_count,
            digest: digest_string("reduced-truth-view-materialization", &basis),
        }
    }

    pub fn truth_view_identity(&self) -> &ReducedTruthViewIdentity {
        &self.truth_view_identity
    }
    pub fn source_branch(&self) -> &str {
        self.source_branch.as_ref()
    }
    pub fn source_commit(&self) -> &str {
        self.source_commit.as_ref()
    }
    pub fn source_snapshot(&self) -> &str {
        self.source_snapshot.as_ref()
    }
    pub fn planned_route_count(&self) -> usize {
        self.planned_route_count
    }
    pub fn snapshot_read_count(&self) -> usize {
        self.snapshot_read_count
    }
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedBridgePublication {
    routing_target_identity: ReducedRoutingTargetIdentity,
    publication_identity: ReducedPublicationIdentity,
    subscription_slice_identity: Arc<str>,
    reduced_route_identities: Arc<[Arc<str>]>,
    invalidation_target_count: usize,
    digest: Arc<str>,
}

impl ReducedBridgePublication {
    pub(crate) fn new(
        routing_target_identity: ReducedRoutingTargetIdentity,
        publication_identity: ReducedPublicationIdentity,
        subscription_slice_identity: Arc<str>,
        reduced_route_identities: Vec<Arc<str>>,
        invalidation_target_count: usize,
    ) -> Self {
        let mut basis = format!(
            "reduced-bridge-publication|routing-target={}|publication={}|subscription-slice={}|route-count={}|invalidation-target-count={}",
            routing_target_identity.as_str(),
            publication_identity.as_str(),
            subscription_slice_identity,
            reduced_route_identities.len(),
            invalidation_target_count,
        );
        for route_identity in &reduced_route_identities {
            basis.push_str("|route=");
            basis.push_str(route_identity);
        }
        Self {
            routing_target_identity,
            publication_identity,
            subscription_slice_identity,
            reduced_route_identities: reduced_route_identities.into(),
            invalidation_target_count,
            digest: digest_string("reduced-bridge-publication", &basis),
        }
    }

    pub fn routing_target_identity(&self) -> &ReducedRoutingTargetIdentity {
        &self.routing_target_identity
    }

    pub fn publication_identity(&self) -> &ReducedPublicationIdentity {
        &self.publication_identity
    }

    pub fn subscription_slice_identity(&self) -> &str {
        self.subscription_slice_identity.as_ref()
    }

    pub fn reduced_route_identities(&self) -> &[Arc<str>] {
        &self.reduced_route_identities
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedFallbackAggregation {
    fallback_identity: ReducedFallbackIdentity,
    fallback_class: Arc<str>,
    bounded_scope_identity: Arc<str>,
    reduced_route_identities: Arc<[Arc<str>]>,
    digest: Arc<str>,
}

impl ReducedFallbackAggregation {
    pub(crate) fn new(
        fallback_identity: ReducedFallbackIdentity,
        fallback_class: Arc<str>,
        bounded_scope_identity: Arc<str>,
        reduced_route_identities: Vec<Arc<str>>,
    ) -> Self {
        let mut basis = format!(
            "reduced-fallback-aggregation|identity={}|fallback-class={}|bounded-scope={}|route-count={}",
            fallback_identity.as_str(),
            fallback_class,
            bounded_scope_identity,
            reduced_route_identities.len(),
        );
        for route_identity in &reduced_route_identities {
            basis.push_str("|route=");
            basis.push_str(route_identity);
        }
        Self {
            fallback_identity,
            fallback_class,
            bounded_scope_identity,
            reduced_route_identities: reduced_route_identities.into(),
            digest: digest_string("reduced-fallback-aggregation", &basis),
        }
    }

    pub fn fallback_identity(&self) -> &ReducedFallbackIdentity {
        &self.fallback_identity
    }

    pub fn fallback_class(&self) -> &str {
        self.fallback_class.as_ref()
    }

    pub fn bounded_scope_identity(&self) -> &str {
        self.bounded_scope_identity.as_ref()
    }

    pub fn reduced_route_identities(&self) -> &[Arc<str>] {
        &self.reduced_route_identities
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedBridgeWorkloadArtifact {
    workload_identity: BridgeWorkloadIdentity,
    reduced_continuity_remaps: Arc<[ReducedContinuityRemap]>,
    reduced_truth_views: Arc<[ReducedTruthViewMaterialization]>,
    reduced_fallbacks: Arc<[ReducedFallbackAggregation]>,
    reduced_publications: Arc<[ReducedBridgePublication]>,
    counters: BridgeBulkPlanningCounters,
    digest: Arc<str>,
}

impl ReducedBridgeWorkloadArtifact {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        reduced_continuity_remaps: Vec<ReducedContinuityRemap>,
        reduced_truth_views: Vec<ReducedTruthViewMaterialization>,
        reduced_fallbacks: Vec<ReducedFallbackAggregation>,
        reduced_publications: Vec<ReducedBridgePublication>,
        counters: BridgeBulkPlanningCounters,
    ) -> Self {
        let reduction_output_count = reduced_continuity_remaps.len()
            + reduced_truth_views.len()
            + reduced_fallbacks.len()
            + reduced_publications.len();
        let mut basis = format!(
            "reduced-bridge-workload-artifact|workload={}|reduction-input-count={}|reduction-output-count={}",
            workload_identity.as_str(),
            counters.bulk_reduction_input_count(),
            reduction_output_count,
        );
        for continuity in &reduced_continuity_remaps {
            basis.push_str("|continuity=");
            basis.push_str(continuity.digest());
        }
        for truth_view in &reduced_truth_views {
            basis.push_str("|truth-view=");
            basis.push_str(truth_view.digest());
        }
        for fallback in &reduced_fallbacks {
            basis.push_str("|fallback=");
            basis.push_str(fallback.digest());
        }
        for publication in &reduced_publications {
            basis.push_str("|publication=");
            basis.push_str(publication.digest());
        }
        Self {
            workload_identity,
            reduced_continuity_remaps: reduced_continuity_remaps.into(),
            reduced_truth_views: reduced_truth_views.into(),
            reduced_fallbacks: reduced_fallbacks.into(),
            reduced_publications: reduced_publications.into(),
            counters,
            digest: digest_string("reduced-bridge-workload-artifact", &basis),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn reduced_continuity_remaps(&self) -> &[ReducedContinuityRemap] {
        &self.reduced_continuity_remaps
    }

    pub fn reduced_truth_views(&self) -> &[ReducedTruthViewMaterialization] {
        &self.reduced_truth_views
    }

    pub fn reduced_fallbacks(&self) -> &[ReducedFallbackAggregation] {
        &self.reduced_fallbacks
    }

    pub fn reduced_publications(&self) -> &[ReducedBridgePublication] {
        &self.reduced_publications
    }

    pub fn reduction_input_count(&self) -> usize {
        self.counters.bulk_reduction_input_count()
    }

    pub fn reduction_output_count(&self) -> usize {
        self.counters.bulk_reduction_output_count()
    }

    pub fn counters(&self) -> &BridgeBulkPlanningCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

use super::*;
