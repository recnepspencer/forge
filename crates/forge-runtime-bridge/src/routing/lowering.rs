use std::sync::Arc;

use crate::input::envelope::{BridgeProducerMetadata, TruthCommitIdentity, TruthPatchIdentity};
use crate::mapping::SubscriptionSliceKind;
use crate::routing::canonicalization::{
    digest_string, invalidation_digest_basis, lowering_provenance_digest_basis,
    lowering_summary_digest_basis, subscription_slice_digest_basis,
};
use crate::routing::counters::BridgeRoutingCounters;
use crate::routing::matching::FineGrainedMatchStatus;
use crate::identity::{BridgeIdentity, InvalidationIdentityTag, SubscriptionSliceIdentityTag};
use crate::routing::planning::{
    BridgeExecutionCounts, BridgePlanningProvenance, BridgeRouteIdentity, BridgeRouteSourceSummary,
};
use crate::routing::proof::BridgeRouteContractProof;
use crate::error::{BridgeRouteError, BridgeRouteErrorKind};
use crate::snapshot::{BridgeSnapshotToken, TruthSnapshotIdentity};

pub type BridgeInvalidationIdentity = BridgeIdentity<InvalidationIdentityTag>;
pub type BridgeSubscriptionSliceIdentity = BridgeIdentity<SubscriptionSliceIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BridgeInvalidationTarget {
    signal_scope: Arc<str>,
    routing_mode: crate::mapping::CoarseRoutingMode,
}

impl BridgeInvalidationTarget {
    pub(crate) fn new(
        signal_scope: Arc<str>,
        routing_mode: crate::mapping::CoarseRoutingMode,
    ) -> Self {
        Self {
            signal_scope,
            routing_mode,
        }
    }

    pub fn signal_scope(&self) -> &str {
        self.signal_scope.as_ref()
    }

    pub fn routing_mode(&self) -> crate::mapping::CoarseRoutingMode {
        self.routing_mode
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalInvalidationTargets {
    targets: Arc<[BridgeInvalidationTarget]>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BridgeSubscriptionSlice {
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    surface_label: Arc<str>,
    slice_kind: SubscriptionSliceKind,
    match_status: FineGrainedMatchStatus,
}

impl BridgeSubscriptionSlice {
    pub(crate) fn new(
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
        slice_kind: SubscriptionSliceKind,
        match_status: FineGrainedMatchStatus,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: surface_label.into(),
            slice_kind,
            match_status,
        }
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label.as_ref()
    }

    pub fn surface_label(&self) -> &str {
        self.surface_label.as_ref()
    }

    pub fn slice_kind(&self) -> &SubscriptionSliceKind {
        &self.slice_kind
    }

    pub fn match_status(&self) -> FineGrainedMatchStatus {
        self.match_status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalSubscriptionSlices {
    slices: Arc<[BridgeSubscriptionSlice]>,
}

impl CanonicalSubscriptionSlices {
    pub(crate) fn new(slices: Vec<BridgeSubscriptionSlice>) -> Self {
        Self {
            slices: Arc::from(slices),
        }
    }

    pub fn slices(&self) -> &[BridgeSubscriptionSlice] {
        &self.slices
    }

    pub(crate) fn shared(&self) -> &Arc<[BridgeSubscriptionSlice]> {
        &self.slices
    }

    pub fn len(&self) -> usize {
        self.slices.len()
    }
}

impl CanonicalInvalidationTargets {
    pub(crate) fn new(targets: Vec<BridgeInvalidationTarget>) -> Self {
        Self {
            targets: Arc::from(targets),
        }
    }

    pub fn targets(&self) -> &[BridgeInvalidationTarget] {
        &self.targets
    }

    pub(crate) fn shared(&self) -> &Arc<[BridgeInvalidationTarget]> {
        &self.targets
    }

    pub fn len(&self) -> usize {
        self.targets.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLoweringPlanSummary {
    route_identity: BridgeRouteIdentity,
    source: BridgeRouteSourceSummary,
    execution_counts: BridgeExecutionCounts,
    subscription_slice_identity: BridgeSubscriptionSliceIdentity,
}

impl BridgeLoweringPlanSummary {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        source: BridgeRouteSourceSummary,
        execution_counts: BridgeExecutionCounts,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    ) -> Self {
        Self {
            route_identity,
            source,
            execution_counts,
            subscription_slice_identity,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.source.source_commit()
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        self.source.source_patch()
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        self.source.source_snapshot()
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.execution_counts.invalidation_target_count()
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.execution_counts.subscription_slice_count()
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.subscription_slice_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLoweringProvenance {
    route_identity: BridgeRouteIdentity,
    planning_provenance: BridgePlanningProvenance,
    digest: Arc<str>,
}

impl BridgeLoweringProvenance {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        planning_provenance: BridgePlanningProvenance,
        digest: Arc<str>,
    ) -> Self {
        Self {
            route_identity,
            planning_provenance,
            digest,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn planning_provenance(&self) -> &BridgePlanningProvenance {
        &self.planning_provenance
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLoweringSummary {
    route_identity: BridgeRouteIdentity,
    execution_counts: BridgeExecutionCounts,
    digest: Arc<str>,
}

impl BridgeLoweringSummary {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        execution_counts: BridgeExecutionCounts,
        digest: Arc<str>,
    ) -> Self {
        Self {
            route_identity,
            execution_counts,
            digest,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.execution_counts.invalidation_target_count()
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.execution_counts.subscription_slice_count()
    }

    pub fn planned_read_count(&self) -> usize {
        self.execution_counts.snapshot_read_count()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BridgeLoweringPlan {
    route_identity: BridgeRouteIdentity,
    source_commit: TruthCommitIdentity,
    source_patch: TruthPatchIdentity,
    source_snapshot: TruthSnapshotIdentity,
    invalidation_targets: CanonicalInvalidationTargets,
    subscription_slices: CanonicalSubscriptionSlices,
    subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    planned_read_count: usize,
    provenance: BridgeLoweringProvenance,
    lowering_summary: BridgeLoweringSummary,
    summary: BridgeLoweringPlanSummary,
    digest_computation_count: usize,
    digest_input_bytes: usize,
}

impl BridgeLoweringPlan {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        source_commit: TruthCommitIdentity,
        source_patch: TruthPatchIdentity,
        source_snapshot: TruthSnapshotIdentity,
        invalidation_targets: Vec<(Arc<str>, crate::mapping::CoarseRoutingMode)>,
        subscription_slices: Vec<BridgeSubscriptionSlice>,
        planned_read_count: usize,
        provenance: BridgeLoweringProvenance,
    ) -> Self {
        let invalidation_targets = CanonicalInvalidationTargets::new(
            invalidation_targets
                .into_iter()
                .map(|(signal_scope, routing_mode)| {
                    BridgeInvalidationTarget::new(signal_scope, routing_mode)
                })
                .collect(),
        );
        let subscription_slices = CanonicalSubscriptionSlices::new(subscription_slices);
        let subscription_slice_basis =
            subscription_slice_digest_basis(source_snapshot.as_str(), subscription_slices.slices());
        let subscription_slice_identity =
            BridgeSubscriptionSliceIdentity::new(digest_string("subscription-slices", &subscription_slice_basis));
        let execution_counts = BridgeExecutionCounts::new(
            invalidation_targets.len(),
            subscription_slices.len(),
            planned_read_count,
        );
        let lowering_summary_basis = lowering_summary_digest_basis(
            &route_identity,
            invalidation_targets.targets(),
            subscription_slices.slices(),
            planned_read_count,
        );
        let lowering_summary = BridgeLoweringSummary::new(
            route_identity.clone(),
            execution_counts.clone(),
            digest_string(
                "lowering-summary",
                &lowering_summary_basis,
            ),
        );
        let summary = BridgeLoweringPlanSummary::new(
            route_identity.clone(),
            BridgeRouteSourceSummary::new(
                source_commit.clone(),
                source_patch.clone(),
                source_snapshot.clone(),
            ),
            execution_counts,
            subscription_slice_identity.clone(),
        );

        Self {
            route_identity,
            source_commit,
            source_patch,
            source_snapshot,
            invalidation_targets,
            subscription_slices,
            subscription_slice_identity,
            planned_read_count,
            provenance,
            lowering_summary,
            summary,
            digest_computation_count: 2,
            digest_input_bytes: subscription_slice_basis.len() + lowering_summary_basis.len(),
        }
    }

    pub(crate) fn summary(&self) -> &BridgeLoweringPlanSummary {
        &self.summary
    }

    pub(crate) fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub(crate) fn source_commit(&self) -> &TruthCommitIdentity {
        &self.source_commit
    }

    pub(crate) fn source_patch(&self) -> &TruthPatchIdentity {
        &self.source_patch
    }

    pub(crate) fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        &self.source_snapshot
    }

    pub(crate) fn invalidation_targets(&self) -> &CanonicalInvalidationTargets {
        &self.invalidation_targets
    }

    pub(crate) fn subscription_slices(&self) -> &CanonicalSubscriptionSlices {
        &self.subscription_slices
    }

    pub(crate) fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.subscription_slice_identity
    }

    pub(crate) fn provenance(&self) -> &BridgeLoweringProvenance {
        &self.provenance
    }

    pub(crate) fn lowering_summary(&self) -> &BridgeLoweringSummary {
        &self.lowering_summary
    }

    pub(crate) fn digest_computation_count(&self) -> usize {
        self.digest_computation_count
    }

    pub(crate) fn digest_input_bytes(&self) -> usize {
        self.digest_input_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ValidatedBridgeLoweringPlan {
    plan: BridgeLoweringPlan,
}

impl ValidatedBridgeLoweringPlan {
    pub(crate) fn from_plan(plan: &BridgeLoweringPlan) -> Result<Self, BridgeRouteError> {
        if plan.provenance.route_identity() != plan.route_identity() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::InvalidLoweringContract,
                "Bridge lowering provenance did not agree with the planned route identity.",
            ));
        }
        if plan.provenance.planning_provenance().route_identity() != plan.route_identity() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::InvalidLoweringContract,
                "Bridge lowering provenance carried planning provenance for a different route identity.",
            ));
        }
        if plan.lowering_summary.route_identity() != plan.route_identity() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::InvalidLoweringContract,
                "Bridge lowering summary did not agree with the planned route identity.",
            ));
        }
        if plan.summary.subscription_slice_identity() != plan.subscription_slice_identity() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::InvalidLoweringContract,
                "Bridge lowering plan summary did not agree with the canonical subscription slice identity.",
            ));
        }
        if plan.summary.invalidation_target_count() != plan.invalidation_targets().len() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::InvalidLoweringContract,
                "Bridge lowering plan summary invalidation-target count did not match the canonical lowering targets.",
            ));
        }
        if plan.summary.subscription_slice_count() != plan.subscription_slices().len() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::InvalidLoweringContract,
                "Bridge lowering plan summary subscription-slice count did not match the canonical lowering slices.",
            ));
        }
        if plan.lowering_summary.planned_read_count() != plan.planned_read_count {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::InvalidLoweringContract,
                "Bridge lowering summary planned-read count did not match the canonical read packet breadth.",
            ));
        }
        let expected_provenance_digest = digest_string(
            "lowering-provenance",
            &lowering_provenance_digest_basis(
                plan.route_identity(),
                plan.provenance.planning_provenance().digest(),
                plan.source_commit().as_str(),
                plan.source_patch().as_str(),
                plan.source_snapshot().as_str(),
            ),
        );
        if plan.provenance.digest() != expected_provenance_digest.as_ref() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::InvalidLoweringContract,
                "Bridge lowering provenance digest did not match the canonical lowering provenance basis.",
            ));
        }
        let expected_summary_digest = digest_string(
            "lowering-summary",
            &lowering_summary_digest_basis(
                plan.route_identity(),
                plan.invalidation_targets().targets(),
                plan.subscription_slices().slices(),
                plan.planned_read_count,
            ),
        );
        if plan.lowering_summary.digest() != expected_summary_digest.as_ref() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::InvalidLoweringContract,
                "Bridge lowering summary digest did not match the canonical lowering summary basis.",
            ));
        }

        Ok(Self { plan: plan.clone() })
    }

    pub(crate) fn plan(&self) -> &BridgeLoweringPlan {
        &self.plan
    }

    pub(crate) fn provenance(&self) -> &BridgeLoweringProvenance {
        self.plan.provenance()
    }

    pub(crate) fn summary(&self) -> &BridgeLoweringSummary {
        self.plan.lowering_summary()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeInvalidationArtifact {
    lowering_plan: ValidatedBridgeLoweringPlan,
    invalidation_identity: BridgeInvalidationIdentity,
    snapshot_token: BridgeSnapshotToken,
    counters: BridgeRoutingCounters,
}

impl BridgeInvalidationArtifact {
    pub(crate) fn new(
        lowering_plan: ValidatedBridgeLoweringPlan,
        invalidation_identity: BridgeInvalidationIdentity,
        snapshot_token: BridgeSnapshotToken,
        counters: BridgeRoutingCounters,
    ) -> Self {
        Self {
            lowering_plan,
            invalidation_identity,
            snapshot_token,
            counters,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        self.lowering_plan.plan().route_identity()
    }

    pub fn invalidation_identity(&self) -> &BridgeInvalidationIdentity {
        &self.invalidation_identity
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.lowering_plan.plan().source_commit()
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        self.lowering_plan.plan().source_patch()
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        self.lowering_plan.plan().source_snapshot()
    }

    pub fn invalidation_targets(&self) -> &CanonicalInvalidationTargets {
        self.lowering_plan.plan().invalidation_targets()
    }

    pub fn subscription_slices(&self) -> &CanonicalSubscriptionSlices {
        self.lowering_plan.plan().subscription_slices()
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        self.lowering_plan.plan().subscription_slice_identity()
    }

    pub fn snapshot_token(&self) -> &BridgeSnapshotToken {
        &self.snapshot_token
    }

    pub fn counters(&self) -> &BridgeRoutingCounters {
        &self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSignalInvalidationDelivery {
    route_identity: BridgeRouteIdentity,
    invalidation_identity: BridgeInvalidationIdentity,
    source_snapshot: TruthSnapshotIdentity,
    contract_proof: BridgeRouteContractProof,
    invalidation_targets: CanonicalInvalidationTargets,
    subscription_slices: CanonicalSubscriptionSlices,
    subscription_slice_identity: BridgeSubscriptionSliceIdentity,
}

impl BridgeSignalInvalidationDelivery {
    pub(crate) fn new(artifact: &BridgeInvalidationArtifact, contract_proof: &BridgeRouteContractProof) -> Self {
        Self {
            route_identity: artifact.route_identity().clone(),
            invalidation_identity: artifact.invalidation_identity().clone(),
            source_snapshot: artifact.source_snapshot().clone(),
            contract_proof: contract_proof.clone(),
            invalidation_targets: artifact.invalidation_targets().clone(),
            subscription_slices: artifact.subscription_slices().clone(),
            subscription_slice_identity: artifact.subscription_slice_identity().clone(),
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn invalidation_identity(&self) -> &BridgeInvalidationIdentity {
        &self.invalidation_identity
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        &self.source_snapshot
    }

    pub fn producer_metadata(&self) -> &BridgeProducerMetadata {
        self.contract_proof.producer_metadata()
    }

    pub fn mapping_context_digest(&self) -> &str {
        self.contract_proof.mapping_context_digest()
    }

    pub fn planning_provenance_digest(&self) -> &str {
        self.contract_proof.planning_provenance_digest()
    }

    pub fn planning_summary_digest(&self) -> &str {
        self.contract_proof.planning_summary_digest()
    }

    pub fn lowering_provenance_digest(&self) -> &str {
        self.contract_proof.lowering_provenance_digest()
    }

    pub fn lowering_summary_digest(&self) -> &str {
        self.contract_proof.lowering_summary_digest()
    }

    pub fn contract_proof(&self) -> &BridgeRouteContractProof {
        &self.contract_proof
    }

    pub fn invalidation_targets(&self) -> &CanonicalInvalidationTargets {
        &self.invalidation_targets
    }

    pub fn subscription_slices(&self) -> &CanonicalSubscriptionSlices {
        &self.subscription_slices
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.subscription_slice_identity
    }
}

pub(crate) fn lower_validated_route(
    validated: ValidatedBridgeLoweringPlan,
    counters: BridgeRoutingCounters,
) -> BridgeInvalidationArtifact {
    let invalidation_basis = invalidation_digest_basis(
        validated.plan().route_identity(),
        validated.plan().source_commit().as_str(),
        validated.plan().source_patch().as_str(),
        validated.plan().source_snapshot().as_str(),
        validated.plan().invalidation_targets().targets(),
    );
    let snapshot_basis = format!(
        "snapshot-token|route={}|commit={}|patch={}|snapshot={}",
        validated.plan().route_identity().as_str(),
        validated.plan().source_commit().as_str(),
        validated.plan().source_patch().as_str(),
        validated.plan().source_snapshot().as_str()
    );
    let counters = counters
        .with_digest_computations(2)
        .with_digest_input_bytes(invalidation_basis.len() + snapshot_basis.len());
    let plan = validated.plan();
    let invalidation_identity =
        BridgeInvalidationIdentity::new(digest_string("invalidation", &invalidation_basis));
    let snapshot_token = BridgeSnapshotToken::issued(
        plan.source_snapshot().clone(),
        digest_string("snapshot-token", &snapshot_basis),
    );

    BridgeInvalidationArtifact::new(
        validated,
        invalidation_identity,
        snapshot_token,
        counters,
    )
}
