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
use crate::schema_view::{QuerySchemaView, ScalarAspectType, SchemaFieldView};
use worth_signal::facade::adapters::SignalInvalidationExecutionReceipt;
use worth_signal::facade::specialist::EvaluationOutput;
use worth_signal::facade::{
    mark_dirty, Aspect, AspectVersion, DependencyEdge, SignalError, SignalGraph, SignalRuntime,
};

use super::super::{
    title_value_touch, RepresentativeArtifacts, WorthQueryLowerRuntimeRepresentativeEvidenceSource,
};

pub(crate) fn representative_causal_bridge_materialization_row() -> RepresentativeArtifacts {
    let read_result = certification_read_result();
    let inspection_basis = crate::basis_lifecycle::basis_lifecycle()
        .historical_snapshot("certification-read", true)
        .inspect()
        .expect("causal bridge inspection basis should admit");
    let observation =
        QueryObservationReceipt::from_read_receipt(read_result.receipt(), inspection_basis.clone());
    let plan = CausalInspection::for_observation(observation, inspection_basis)
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
    let (planned, execution_receipt) = performed_signal_frontier_evidence();
    let planned = SignalFrontierSurfaceEvidence::from_invalidation_planning_estimate(&planned);
    let executed =
        SignalFrontierSurfaceEvidence::from_invalidation_execution_receipt(&execution_receipt);
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

fn performed_signal_frontier_evidence() -> (
    worth_signal::facade::adapters::InvalidationPlanningEstimate,
    SignalInvalidationExecutionReceipt,
) {
    let aspect = Aspect::new(0);
    let mut graph = SignalGraph::new();
    let source = graph.node().produces_aspects([aspect]).build();
    let dependent = graph.node().reads_aspects([aspect]).build();
    graph
        .set_dependencies(dependent, [DependencyEdge::new(source, aspect)])
        .expect("representative Signal dependency should install");
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    runtime
        .evaluate_dirty(&(), &|_| {
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                AspectVersion::from_updates([(aspect, 1)]),
            ))
        })
        .expect("representative Signal graph should initialize");
    let (_, receipt) = runtime
        .observe_invalidation_execution(|runtime| {
            mark_dirty(runtime.graph_mut(), source, aspect)?;
            runtime.evaluate_dirty(&(), &|_| {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                    AspectVersion::from_updates([(aspect, 2)]),
                ))
            })
        })
        .expect("representative Signal invalidation should execute");
    let estimate = runtime
        .graph()
        .observe()
        .latest_invalidation_planning_estimate()
        .cloned()
        .expect("performed invalidation should retain its public planning estimate");
    (estimate, receipt)
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
                            ScalarAspectType::String,
                        ),
                        SchemaFieldView::new(
                            crate::authoring::AspectName::new("title")
                                .expect("schema aspect literal must be valid"),
                            crate::authoring::FieldName::new("value")
                                .expect("schema field literal must be valid"),
                            ScalarAspectType::String,
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
