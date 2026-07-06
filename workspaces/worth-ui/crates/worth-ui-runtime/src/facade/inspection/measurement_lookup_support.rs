use worth_ui_inspection::{
    UiEvidenceFamily, UiInspectionMeasurementDenialPosture, UiInspectionMeasurementFailureSource,
    UiInspectionMeasurementGenerationCompatibility, UiInspectionMeasurementQueryFactFamily,
    UiInspectionMeasurementQueryUnsupportedReason, UiInspectionQuery, UiInspectionScope,
    UiInspectionSupportReport, UiRelevanceFamily,
};

use crate::admission::{
    UiAdmissionTarget, UiAdmissionWorld, UiQueryMeasurementBasisAuthority,
    UiQueryMeasurementEligibility, UiQueryMeasurementEligibilityPosture,
};
use crate::declaration::UiDeclarationArtifact;
use crate::evidence::{
    MeasurementEvidenceInput, UiEvidenceRef, UiInspectionCostMetrics, UiMeasurementBasis,
    UiProjectionFactReceipt,
};
use crate::facade::measurement_inspection_evidence::{
    UiMeasurementInspectionEvidenceBundle, UiMeasurementInspectionEvidenceSnapshot,
};
use crate::graph::{UiGraphNodeIdentity, UiGraphSnapshot};
use crate::obligations::touch::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchDescriptor, UiGraphTouchTiming,
};

pub(super) enum MeasurementInspectionOutcome {
    Basis(UiMeasurementBasis),
    Denial(
        UiInspectionMeasurementDenialPosture,
        Option<UiInspectionMeasurementFailureSource>,
    ),
    Compatibility(UiInspectionMeasurementGenerationCompatibility),
}

pub(super) struct MeasurementInspectionTarget<'a> {
    pub(super) artifact: &'a UiDeclarationArtifact,
    pub(super) graph_node_identity: Option<UiGraphNodeIdentity>,
    pub(super) graph_node_count: usize,
    pub(super) refs: &'a [UiEvidenceRef],
    pub(super) cost_metrics: UiInspectionCostMetrics,
}

impl<'a> MeasurementInspectionTarget<'a> {
    pub(super) fn new(
        artifact: &'a UiDeclarationArtifact,
        graph_node_identity: Option<UiGraphNodeIdentity>,
        graph_node_count: usize,
        refs: &'a [UiEvidenceRef],
        cost_metrics: UiInspectionCostMetrics,
    ) -> Self {
        Self {
            artifact,
            graph_node_identity,
            graph_node_count,
            refs,
            cost_metrics,
        }
    }
}

pub(super) fn declaration_instance_resolution(
    graph_snapshot: &UiGraphSnapshot,
    declaration_identity: &crate::declaration::UiDeclarationIdentity,
) -> (Option<UiGraphNodeIdentity>, usize) {
    let instances = graph_snapshot
        .lookup()
        .declaration_instances(declaration_identity);
    match instances.value() {
        [graph_node_identity] => (Some(*graph_node_identity), 1),
        values => (None, values.len()),
    }
}

pub(super) fn measurement_policy_for_artifact(
    artifact: &UiDeclarationArtifact,
) -> Option<&crate::declaration::UiDeclaredMeasurementPolicyPosture> {
    let snapshot = artifact.support_snapshot().ok()?;
    let row =
        snapshot.row(crate::declaration::UiDeclarationSupportRowSchemaKind::MeasurementPolicy)?;
    row.declared_measurement_policy_posture()
}

pub(super) fn measurement_touch_for_target(
    app: &crate::facade::WorthUiApp,
    artifact: &UiDeclarationArtifact,
    graph_node_identity: UiGraphNodeIdentity,
) -> Option<UiGraphTouchDescriptor> {
    let origin = app
        .graph()
        .touches()
        .declaration_change_receipt(artifact)
        .ok()?;
    app.graph()
        .touches()
        .from_node(
            origin,
            UiGraphTouchTiming::PostMutation,
            graph_node_identity,
            UiGraphTouchAspects::new().measurement(UiGraphTouchAspectPosture::Invalidated),
        )
        .ok()
}

pub(super) fn measurement_host_inputs(
    bundle: Option<&UiMeasurementInspectionEvidenceBundle>,
) -> Vec<MeasurementEvidenceInput> {
    let mut inputs = Vec::new();
    if let Some(bundle) = bundle {
        if let Some(report) = bundle.host_capability_report() {
            inputs.push(MeasurementEvidenceInput::host_capability_report(report));
        }
        inputs.extend(
            bundle
                .host_measurement_results()
                .iter()
                .map(MeasurementEvidenceInput::host_measurement_result),
        );
    }
    inputs
}

pub(super) fn admission_target_for_touch(
    touch: &UiGraphTouchDescriptor,
    bundle: Option<&UiMeasurementInspectionEvidenceBundle>,
) -> UiAdmissionTarget {
    let mut target = UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    );
    if let Some(report) =
        bundle.and_then(UiMeasurementInspectionEvidenceBundle::host_capability_report)
    {
        target = target.with_host_capability_report(report.clone());
    }
    if let Some(consumption) =
        bundle.and_then(UiMeasurementInspectionEvidenceBundle::query_projection_consumption)
    {
        if let Ok(bound_target) = target
            .clone()
            .with_query_prerequisites_from_projection_consumption(consumption)
        {
            target = bound_target;
        }
    }
    target
}

pub(super) fn query_measurement_outcome_for_bundle(
    app: &crate::facade::WorthUiApp,
    selected: &crate::obligations::selection::UiSelectedObligationSet,
    measurement_admission: Option<&crate::admission::UiMeasurementAdmission>,
    bundle: Option<&UiMeasurementInspectionEvidenceBundle>,
) -> Option<QueryMeasurementInspectionOutcome> {
    let eligibility =
        query_measurement_eligibility_for_bundle(app, selected, measurement_admission, bundle)?;
    match eligibility.posture() {
        UiQueryMeasurementEligibilityPosture::Eligible { .. } => eligibility
            .projection_fact_receipt()
            .cloned()
            .map(QueryMeasurementInspectionOutcome::Receipt),
        UiQueryMeasurementEligibilityPosture::StaleBasisGeneration {
            expected, observed, ..
        } => Some(QueryMeasurementInspectionOutcome::Compatibility(
            project_query_basis_compatibility(expected, observed),
        )),
        UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture { reason, .. } => {
            Some(QueryMeasurementInspectionOutcome::Denial(
                UiInspectionMeasurementDenialPosture::UnsupportedQueryPosture {
                    reason: project_query_unsupported_reason(*reason),
                },
                Some(UiInspectionMeasurementFailureSource::QueryFacts),
            ))
        }
        UiQueryMeasurementEligibilityPosture::UnavailableFactFamilies {
            available_families,
            missing_families,
            ..
        } => Some(QueryMeasurementInspectionOutcome::Denial(
            UiInspectionMeasurementDenialPosture::UnavailableFactFamilies {
                available_families: available_families
                    .iter()
                    .copied()
                    .map(project_query_fact_family)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                missing_families: missing_families
                    .iter()
                    .copied()
                    .map(project_query_fact_family)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            },
            Some(UiInspectionMeasurementFailureSource::QueryFacts),
        )),
    }
}

pub(super) fn support_report_for_artifact(
    artifact: &UiDeclarationArtifact,
    scope: UiInspectionScope,
) -> Option<UiInspectionSupportReport> {
    let snapshot = artifact.support_snapshot().ok()?;
    let rows = snapshot.inspection_rows(scope);
    (!rows.is_empty()).then(|| UiInspectionSupportReport::from_scope_rows(scope, rows.as_ref()))
}

pub(super) fn filter_refs_for_query(
    refs: &[UiEvidenceRef],
    query: &UiInspectionQuery,
) -> Box<[UiEvidenceRef]> {
    match query.relevance().filter().family_filter() {
        Some(family) => refs
            .iter()
            .copied()
            .filter(|evidence_ref| family_matches(evidence_ref.family(), family))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        None => refs.to_vec().into_boxed_slice(),
    }
}

fn query_measurement_eligibility_for_bundle(
    app: &crate::facade::WorthUiApp,
    selected: &crate::obligations::selection::UiSelectedObligationSet,
    measurement_admission: Option<&crate::admission::UiMeasurementAdmission>,
    bundle: Option<&UiMeasurementInspectionEvidenceBundle>,
) -> Option<UiQueryMeasurementEligibility> {
    let consumption = bundle?.query_projection_consumption()?;
    let admission = measurement_admission?;
    app.admission()
        .admit_query_measurement_eligibility_from_projection_consumption(
            selected,
            admission,
            consumption,
        )
}

pub(super) enum QueryMeasurementInspectionOutcome {
    Receipt(UiProjectionFactReceipt),
    Denial(
        UiInspectionMeasurementDenialPosture,
        Option<UiInspectionMeasurementFailureSource>,
    ),
    Compatibility(UiInspectionMeasurementGenerationCompatibility),
}

fn project_query_basis_compatibility(
    expected: &UiQueryMeasurementBasisAuthority,
    observed: &UiQueryMeasurementBasisAuthority,
) -> UiInspectionMeasurementGenerationCompatibility {
    UiInspectionMeasurementGenerationCompatibility::IncompatibleWorld {
        expected_query_basis_digest: query_basis_digest_from_authority(expected),
        observed_world_basis_digest: Some(query_basis_digest_from_authority(observed)),
    }
}

fn query_basis_digest_from_authority(authority: &UiQueryMeasurementBasisAuthority) -> Box<str> {
    match authority {
        UiQueryMeasurementBasisAuthority::AdmittedPrerequisites { basis_digest, .. } => {
            basis_digest.as_str().into()
        }
        UiQueryMeasurementBasisAuthority::ProjectionConsumption { basis_digest, .. } => {
            basis_digest.clone()
        }
    }
}

fn project_query_unsupported_reason(
    reason: crate::admission::UiQueryMeasurementUnsupportedQueryReason,
) -> UiInspectionMeasurementQueryUnsupportedReason {
    match reason {
        crate::admission::UiQueryMeasurementUnsupportedQueryReason::MissingQueryPrerequisites => {
            UiInspectionMeasurementQueryUnsupportedReason::MissingQueryPrerequisites
        }
        crate::admission::UiQueryMeasurementUnsupportedQueryReason::WrongWorldProjection => {
            UiInspectionMeasurementQueryUnsupportedReason::WrongWorldProjection
        }
        crate::admission::UiQueryMeasurementUnsupportedQueryReason::RebindRequired => {
            UiInspectionMeasurementQueryUnsupportedReason::RebindRequired
        }
        crate::admission::UiQueryMeasurementUnsupportedQueryReason::AmbiguousSources => {
            UiInspectionMeasurementQueryUnsupportedReason::AmbiguousSources
        }
        crate::admission::UiQueryMeasurementUnsupportedQueryReason::ProjectionConsumptionUnavailable => {
            UiInspectionMeasurementQueryUnsupportedReason::ProjectionConsumptionUnavailable
        }
    }
}

fn project_query_fact_family(
    family: worth_ui_query_binding::WorthUiQueryMeasurementFactFamily,
) -> UiInspectionMeasurementQueryFactFamily {
    match family {
        worth_ui_query_binding::WorthUiQueryMeasurementFactFamily::ScrollContentExtent => {
            UiInspectionMeasurementQueryFactFamily::ScrollContentExtent
        }
    }
}

fn family_matches(evidence_family: UiEvidenceFamily, relevance_family: UiRelevanceFamily) -> bool {
    matches!(
        (evidence_family, relevance_family),
        (
            UiEvidenceFamily::Declaration,
            UiRelevanceFamily::Declaration
        ) | (UiEvidenceFamily::Admission, UiRelevanceFamily::Admission)
            | (UiEvidenceFamily::Graph, UiRelevanceFamily::Graph)
            | (UiEvidenceFamily::Planning, UiRelevanceFamily::Planning)
    )
}

pub(super) fn measurement_bundle_for_artifact<'a>(
    snapshot: &'a UiMeasurementInspectionEvidenceSnapshot,
    artifact: &UiDeclarationArtifact,
) -> Option<&'a UiMeasurementInspectionEvidenceBundle> {
    snapshot.bundle_for_artifact(artifact)
}
