use super::bootstrap::WorthUiFacadeLifecycleBootstrap;
use super::declaration_freeze::lower_graph_handoffs;
use super::runtime_instance_expansion::expand_runtime_instance_handoffs;
use super::{WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationSource};
use crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use crate::facade::prepared_application_authority::{
    WorthUiPreparedApplicationAuthority, WorthUiPreparedApplicationAuthorityInput,
    WorthUiPreparedGenerationLineage,
};
use crate::facade::registry::snapshot::CapabilitySnapshot;
use crate::graph::{admit_graph_handoffs, UiGraphWorldProfile};
use std::rc::Rc;

pub(crate) struct WorthUiApplicationPreparationInput {
    pub(crate) capability_snapshot: CapabilitySnapshot,
    pub(crate) preparation_source: WorthUiApplicationPreparationSource,
    pub(crate) host_session_plan:
        crate::facade::prepared_application_authority::WorthUiHostSessionPlan,
    pub(crate) visual_inspection_policy: worth_ui_inspection::UiVisualInspectionPolicy,
    pub(crate) graph_world_profile: UiGraphWorldProfile,
    pub(crate) runtime_instance_basis_admissions:
        Box<[crate::graph::UiRuntimeInstanceBasisAdmission]>,
    pub(crate) measurement_inspection_evidence: Box<[UiMeasurementInspectionEvidenceBundle]>,
    pub(crate) query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
    pub(crate) intent_application_facts: crate::declaration::UiIntentApplicationFactPlan,
    pub(crate) change_profile: crate::runtime::rebind::UiChangeProfile,
}

pub(crate) fn prepare_application_authority(
    input: WorthUiApplicationPreparationInput,
) -> Result<WorthUiPreparedApplicationAuthority, WorthUiApplicationPreparationDenial> {
    let WorthUiApplicationPreparationInput {
        capability_snapshot,
        preparation_source,
        host_session_plan,
        visual_inspection_policy,
        graph_world_profile,
        runtime_instance_basis_admissions,
        measurement_inspection_evidence,
        query_binding_plan,
        intent_application_facts,
        change_profile,
    } = input;
    let capability_snapshot = Rc::new(capability_snapshot);
    let (
        canonical_artifact,
        authored_source_basis,
        declaration_source_identity,
        semantic_handoff,
        declaration_artifacts,
    ) = preparation_source.into_prepared_parts();
    let graph_handoffs = lower_graph_handoffs(&declaration_artifacts)
        .map_err(WorthUiApplicationPreparationDenial::GraphHandoff)?;
    let graph_handoffs =
        expand_runtime_instance_handoffs(&graph_handoffs, &runtime_instance_basis_admissions);
    let graph_snapshot = admit_graph_handoffs(&graph_handoffs, &runtime_instance_basis_admissions)
        .map_err(WorthUiApplicationPreparationDenial::GraphAdmission)?
        .commit_initial_generation(graph_world_profile)
        .map_err(WorthUiApplicationPreparationDenial::GraphCommit)?
        .into_committed_snapshot();
    let intent_catalog = crate::declaration::UiIntentCatalog::prepare(
        semantic_handoff.intent_material(),
        capability_snapshot.intent_definitions(),
        &graph_snapshot,
        &query_binding_plan,
        &intent_application_facts,
    )
    .map_err(WorthUiApplicationPreparationDenial::IntentCatalog)?;
    let retained_measurement_inspection_evidence = measurement_inspection_evidence.clone();
    let lifecycle = WorthUiFacadeLifecycleBootstrap::bootstrap_with_inspection_scope_inventory(
        &declaration_artifacts,
        measurement_inspection_evidence,
        worth_ui_inspection::RUNTIME_INSPECTION_SCOPE_INVENTORY,
    );
    Ok(WorthUiPreparedApplicationAuthority::seal(
        WorthUiPreparedApplicationAuthorityInput {
            capability_snapshot,
            canonical_artifact,
            authored_source_basis,
            generation_lineage: WorthUiPreparedGenerationLineage::initial(),
            declaration_source_identity,
            semantic_handoff,
            declaration_artifacts,
            graph_snapshot,
            intent_catalog,
            lifecycle,
            query_binding_plan,
            intent_application_facts,
            host_session_plan,
            visual_inspection_policy,
            runtime_instance_basis_admissions,
            measurement_inspection_evidence: retained_measurement_inspection_evidence,
            change_profile,
        },
    ))
}

pub(crate) fn prepare_successor_application_authority(
    current: &WorthUiPreparedApplicationAuthority,
    submission: crate::runtime::WorthUiWatchedCandidateSubmission,
) -> Result<
    (
        WorthUiPreparedApplicationAuthority,
        crate::runtime::WorthUiReplacementCandidate,
    ),
    WorthUiApplicationPreparationDenial,
> {
    let candidate_snapshot_digest = submission.candidate_snapshot_digest();
    let authored_source_basis = submission.authored_source_basis();
    if candidate_snapshot_digest != current.capabilities().digest().as_u64() {
        return Err(
            WorthUiApplicationPreparationDenial::CandidateSnapshotMismatch {
                candidate_snapshot_digest,
                prepared_snapshot_digest: current.capabilities().digest().as_u64(),
            },
        );
    }
    let (canonical_artifact, candidate, declaration_material, semantic_handoff) = submission
        .into_replacement_handoff()
        .into_replacement_parts();
    let (declaration_artifacts, declaration_source_identity) = declaration_material.into_parts();
    let graph_handoffs = lower_graph_handoffs(&declaration_artifacts)
        .map_err(WorthUiApplicationPreparationDenial::GraphHandoff)?;
    let admissions = current.runtime_instance_basis_admissions();
    let graph_handoffs = expand_runtime_instance_handoffs(&graph_handoffs, admissions);
    let graph_snapshot = admit_graph_handoffs(&graph_handoffs, admissions)
        .map_err(WorthUiApplicationPreparationDenial::GraphAdmission)?
        .commit_successor_generation(crate::graph::UiGraphAuthority::new(
            current.graph_snapshot(),
        ))
        .map_err(WorthUiApplicationPreparationDenial::GraphCommit)?
        .into_committed_snapshot();
    let intent_catalog = crate::declaration::UiIntentCatalog::prepare(
        semantic_handoff.intent_material(),
        current.capabilities().intent_definitions(),
        &graph_snapshot,
        current.query_binding_plan(),
        current.intent_application_fact_plan(),
    )
    .map_err(WorthUiApplicationPreparationDenial::IntentCatalog)?;
    let measurement_inspection_evidence = current
        .measurement_inspection_evidence()
        .to_vec()
        .into_boxed_slice();
    let lifecycle = WorthUiFacadeLifecycleBootstrap::bootstrap_with_inspection_scope_inventory(
        &declaration_artifacts,
        measurement_inspection_evidence.clone(),
        worth_ui_inspection::RUNTIME_INSPECTION_SCOPE_INVENTORY,
    );
    let authority =
        WorthUiPreparedApplicationAuthority::seal(WorthUiPreparedApplicationAuthorityInput {
            capability_snapshot: current.capability_authority(),
            canonical_artifact,
            generation_lineage: WorthUiPreparedGenerationLineage::authored_source_successor(
                authored_source_basis.clone(),
            ),
            authored_source_basis,
            declaration_source_identity,
            semantic_handoff,
            declaration_artifacts,
            graph_snapshot,
            intent_catalog,
            lifecycle,
            query_binding_plan: current.query_binding_plan().clone(),
            intent_application_facts: current.intent_application_fact_plan().clone(),
            host_session_plan: current.host_session_plan().clone(),
            visual_inspection_policy: current.visual_inspection_policy(),
            runtime_instance_basis_admissions: admissions.to_vec().into_boxed_slice(),
            measurement_inspection_evidence,
            change_profile: current.change_profile(),
        });
    Ok((authority, candidate))
}
