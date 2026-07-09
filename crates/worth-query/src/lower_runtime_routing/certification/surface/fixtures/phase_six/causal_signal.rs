use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::frontier_signal_adapter::SignalFrontierSurfaceEvidence;
use crate::intent_admission::{certification_bridge, certification_runtime};
use crate::lower_runtime_routing::{
    WorthQueryLowerRuntimeAuthorityOwner, WorthQueryLowerRuntimeBoundaryEnvelope,
    WorthQueryLowerRuntimeBoundaryExecutionReceipt, WorthQueryLowerRuntimeCapabilityEligibility,
    WorthQueryLowerRuntimeCapabilityRequest, WorthQueryLowerRuntimeRouteKind,
    WorthQueryLowerRuntimeRoutePlan, WorthQueryLowerRuntimeSeamKey,
};
use crate::runtime::{
    CausalInspection, QueryObservationReceipt, WorthQueryAspectMutationBuilder,
    WorthQueryReadFamily, WorthQueryReadResult, WorthQueryWorkspace,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use worth_signal::facade::adapters::{
    DedupedNodeBatch, FrontierEntryClassification, FrontierExecutionCounters,
    FrontierExecutionSummary, FrontierInclusionBasis, FrontierPlan, FrontierPredictedCounters,
    FrontierSeedCause, FrontierWaveEntryPlan, FrontierWaveEntrySummary, FrontierWavePlan,
    FrontierWaveSummary, InvalidationSeed, InvalidationSeedBatch, PartitionScopeSet,
    SortedSourceBatch, TouchedScopeSummary,
};
use worth_signal::facade::{Aspect, NodeId, PartitionSubscription};

use super::super::{
    title_value_touch, RepresentativeArtifacts, WorthQueryLowerRuntimeRepresentativeEvidenceSource,
};

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
    let evidence =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(
                WorthQueryEvidenceTag::new("artifact"),
                artifact.artifact_for_reporting(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("bridge_envelope"),
                artifact.bridge_envelope_for_reporting().unwrap_or("none"),
            )
            .seal();
    route_planned_row(
        WorthQueryLowerRuntimeSeamKey::CausalBridgeMaterialization,
        WorthQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        "Causal bridge materialization",
        evidence,
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
    let evidence =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(
                WorthQueryEvidenceTag::new("planned"),
                planned.surface_digest().as_str(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("executed"),
                executed.surface_digest().as_str(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("predicted_breadth"),
                planned.predicted_breadth().to_string(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("realized_breadth"),
                executed.realized_breadth().unwrap_or_default().to_string(),
            )
            .seal();
    route_planned_row(
        WorthQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
        WorthQueryLowerRuntimeAuthorityOwner::Signal,
        "Frontier evidence intake",
        evidence,
    )
}

fn route_planned_row(
    seam_key: WorthQueryLowerRuntimeSeamKey,
    owner: WorthQueryLowerRuntimeAuthorityOwner,
    capability_label: &str,
    evidence: WorthQueryEvidenceIdentity,
) -> RepresentativeArtifacts {
    let request = WorthQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        WorthQueryLowerRuntimeRouteKind::RoutePlanning,
        owner,
        capability_label,
        crate::lower_runtime_routing::WorthQueryLowerRuntimeSubjectIdentity::compose(
            "phase-six-causal-signal-route-subject",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), &evidence)
        .seal(),
    );
    let eligibility = WorthQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request.clone(),
        &evidence,
    );
    let route_plan = WorthQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        crate::lower_runtime_routing::WorthQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "phase-six-causal-signal-route",
            &evidence,
        ),
    );
    let retained_evidence_identity =
        crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
            "phase-six-causal-signal-route",
            &evidence,
        );
    let boundary_receipt = WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        &retained_evidence_identity,
    );
    let envelope = WorthQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam_key,
        &route_plan,
        &boundary_receipt,
        &retained_evidence_identity,
    );
    RepresentativeArtifacts {
        seam_key,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: WorthQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

fn certification_read_result() -> WorthQueryReadResult {
    let mut workspace = certification_runtime()
        .workspace("lower-runtime-causal-bridge")
        .expect("causal bridge workspace should build");
    workspace
        .insert("Task", |task: WorthQueryAspectMutationBuilder| {
            task.set_aspect(
                title_value_touch(),
                crate::runtime::WorthQueryAuthoredAspectValue::string("Causal fixture"),
            )
        })
        .expect("causal bridge seed write should succeed");
    let family = certification_read_family(&mut workspace, "lower-runtime-causal-family");
    workspace
        .execute_read_family(&family)
        .expect("causal bridge read should execute")
}

fn certification_read_family(
    workspace: &mut WorthQueryWorkspace,
    family_name: &str,
) -> WorthQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.local_detail(
                "user",
                QuerySchemaView::new(
                    "lower-runtime-causal-read",
                    [
                        SchemaFieldView::new(
                            crate::authoring::AspectName::new("identity")
                                .expect("schema aspect literal must be valid"),
                            crate::authoring::FieldName::new("id")
                                .expect("schema field literal must be valid"),
                            SchemaFieldKind::String,
                        ),
                        SchemaFieldView::new(
                            crate::authoring::AspectName::new("title")
                                .expect("schema aspect literal must be valid"),
                            crate::authoring::FieldName::new("value")
                                .expect("schema field literal must be valid"),
                            SchemaFieldKind::String,
                        ),
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
