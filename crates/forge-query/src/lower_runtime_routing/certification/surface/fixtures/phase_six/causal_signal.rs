use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::frontier_signal_adapter::SignalFrontierSurfaceEvidence;
use crate::identity::hash_parts;
use crate::intent_admission::{certification_bridge, certification_runtime};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{
    CausalInspection, ForgeQueryAspectMutationBuilder, ForgeQueryReadFamily, ForgeQueryReadResult,
    ForgeQueryWorkspace, QueryObservationReceipt,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use forge_signal::facade::adapters::{
    DedupedNodeBatch, FrontierEntryClassification, FrontierExecutionCounters,
    FrontierExecutionSummary, FrontierInclusionBasis, FrontierPlan, FrontierPredictedCounters,
    FrontierSeedCause, FrontierWaveEntryPlan, FrontierWaveEntrySummary, FrontierWavePlan,
    FrontierWaveSummary, InvalidationSeed, InvalidationSeedBatch, PartitionScopeSet,
    SortedSourceBatch, TouchedScopeSummary,
};
use forge_signal::facade::{Aspect, NodeId, PartitionSubscription};

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};

pub(crate) fn representative_causal_bridge_materialization_row() -> RepresentativeArtifacts {
    let read_result = certification_read_result();
    let observation = QueryObservationReceipt::from_read_receipt(read_result.receipt());
    let plan = CausalInspection::for_observation(observation)
        .why_replayed()
        .materialized_detail()
        .include_all_retained_evidence()
        .plan()
        .expect("causal bridge fixture should plan inspection");
    let artifact = plan
        .materialize_with_bridge(&certification_bridge())
        .expect("causal bridge fixture should materialize inspection");
    let retained = hash_parts(&[
        artifact.artifact_digest().to_string(),
        artifact
            .bridge_envelope_digest()
            .unwrap_or("none")
            .to_string(),
    ]);
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::CausalBridgeMaterialization,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        "Causal bridge materialization",
        &[
            "causal_bridge_materialization_subject_v1".to_string(),
            format!("artifact:{}", artifact.artifact_digest()),
            format!(
                "bridge-envelope:{}",
                artifact.bridge_envelope_digest().unwrap_or("none")
            ),
        ],
        "causal-bridge-materialization",
        retained,
    )
}

pub(crate) fn representative_frontier_evidence_row() -> RepresentativeArtifacts {
    let plan = FrontierPlan::new(
        InvalidationSeedBatch::new([InvalidationSeed::new(
            NodeId::new(1, 0),
            Aspect::new(0),
            PartitionScopeSet::new([PartitionSubscription::whole_partition("tasks")]),
            FrontierSeedCause::DirtySource,
        )]),
        vec![FrontierWavePlan::new(
            0,
            Aspect::new(0),
            [FrontierWaveEntryPlan::new(
                NodeId::new(2, 0),
                FrontierEntryClassification::DirectDirty,
                FrontierInclusionBasis::DirectSubscriptionMatch,
                PartitionScopeSet::new([PartitionSubscription::whole_partition("tasks")]),
                [0],
            )],
        )],
        Vec::new(),
        TouchedScopeSummary::new_invalidation(
            PartitionScopeSet::new([PartitionSubscription::whole_partition("tasks")]),
            PartitionScopeSet::new([PartitionSubscription::whole_partition("tasks")]),
            PartitionScopeSet::default(),
            PartitionScopeSet::new([PartitionSubscription::whole_partition("tasks")]),
            PartitionScopeSet::default(),
            DedupedNodeBatch::new([NodeId::new(1, 0), NodeId::new(2, 0)]),
            SortedSourceBatch::default(),
        ),
        FrontierPredictedCounters {
            seed_count: 1,
            direct_wave_count: 1,
            direct_dirty_count: 1,
            partition_match_count: 1,
            ..FrontierPredictedCounters::default()
        },
    );
    let summary = FrontierExecutionSummary::new(
        1,
        vec![FrontierWaveSummary::new(
            0,
            Aspect::new(0),
            [FrontierWaveEntrySummary::new(
                NodeId::new(2, 0),
                FrontierEntryClassification::DirectDirty,
                FrontierInclusionBasis::DirectSubscriptionMatch,
                PartitionScopeSet::new([PartitionSubscription::whole_partition("tasks")]),
            )],
        )],
        Vec::new(),
        TouchedScopeSummary::new_invalidation(
            PartitionScopeSet::new([PartitionSubscription::whole_partition("tasks")]),
            PartitionScopeSet::new([PartitionSubscription::whole_partition("tasks")]),
            PartitionScopeSet::default(),
            PartitionScopeSet::new([PartitionSubscription::whole_partition("tasks")]),
            PartitionScopeSet::default(),
            DedupedNodeBatch::new([NodeId::new(1, 0), NodeId::new(2, 0)]),
            SortedSourceBatch::default(),
        ),
        FrontierExecutionCounters {
            frontier_seed_count: 1,
            frontier_direct_wave_count: 1,
            frontier_direct_dirty_count: 1,
            frontier_partition_match_count: 1,
            ..FrontierExecutionCounters::default()
        },
    );
    let planned = SignalFrontierSurfaceEvidence::from_frontier_plan(&plan);
    let executed = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(&summary);
    let retained = hash_parts(&[
        planned.surface_digest().as_str().to_string(),
        executed.surface_digest().as_str().to_string(),
    ]);
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
        ForgeQueryLowerRuntimeAuthorityOwner::Signal,
        "Frontier evidence intake",
        &[
            "frontier_evidence_route_subject_v1".to_string(),
            format!("planned:{}", planned.surface_digest().as_str()),
            format!("executed:{}", executed.surface_digest().as_str()),
            format!("predicted_breadth:{}", planned.predicted_breadth()),
            format!(
                "realized_breadth:{}",
                executed.realized_breadth().unwrap_or_default()
            ),
        ],
        "frontier-signal-surface",
        retained,
    )
}

fn route_planned_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    owner: ForgeQueryLowerRuntimeAuthorityOwner,
    capability_label: &str,
    subject_parts: &[String],
    support_label: &str,
    retained_evidence_digest: String,
) -> RepresentativeArtifacts {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        owner,
        capability_label,
        hash_parts(subject_parts),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        retained_evidence_digest.clone(),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility.clone(), support_label);
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        retained_evidence_digest.clone(),
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam_key,
        &route_plan,
        &boundary_receipt,
        &retained_evidence_digest,
    );
    RepresentativeArtifacts {
        seam_key,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

fn certification_read_result() -> ForgeQueryReadResult {
    let mut workspace = certification_runtime()
        .workspace("lower-runtime-causal-bridge")
        .expect("causal bridge workspace should build");
    workspace
        .insert("Task", |task: ForgeQueryAspectMutationBuilder| {
            task.aspect("title.value", "Causal fixture")
        })
        .expect("causal bridge seed write should succeed");
    let family = certification_read_family(&mut workspace, "lower-runtime-causal-family");
    workspace
        .execute_read_family(&family)
        .expect("causal bridge read should execute")
}

fn certification_read_family(
    workspace: &mut ForgeQueryWorkspace,
    family_name: &str,
) -> ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.local_detail(
                "user",
                QuerySchemaView::new(
                    "lower-runtime-causal-read",
                    [
                        SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                        SchemaFieldView::new("title", "value", SchemaFieldKind::String),
                    ],
                    [],
                ),
                |query| {
                    query.project(
                        AspectFieldSelector::new("identity", "id")
                            .expect("identity projection should build"),
                    )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "id")
                            .expect("identity result-shape field should build"),
                    )
                },
            )
        })
        .expect("causal bridge read family should define")
}
