use super::bootstrap::WorthUiFacadeLifecycleBootstrap;
use super::declaration_freeze::{
    lower_declaration_artifacts, lower_graph_handoffs, lower_source_backed_declaration_artifacts,
};
use super::runtime_instance_expansion::expand_runtime_instance_handoffs;
use super::{
    WorthUiApplicationDeclarationSource, WorthUiApplicationPreparationDenial,
    WorthUiApplicationPreparationSource,
};
use crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use crate::facade::prepared_application_authority::{
    WorthUiPreparedApplicationAuthority, WorthUiPreparedApplicationAuthorityInput,
};
use crate::facade::registry::snapshot::CapabilitySnapshot;
use crate::graph::{admit_graph_handoffs, UiGraphWorldProfile};
use std::rc::Rc;

pub(crate) fn prepare_application_authority(
    capability_snapshot: CapabilitySnapshot,
    preparation_source: WorthUiApplicationPreparationSource,
    host_session_plan: crate::facade::prepared_application_authority::WorthUiHostSessionPlan,
    graph_world_profile: UiGraphWorldProfile,
    runtime_instance_basis_admissions: Box<[crate::graph::UiRuntimeInstanceBasisAdmission]>,
    measurement_inspection_evidence: Box<[UiMeasurementInspectionEvidenceBundle]>,
    query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
) -> Result<WorthUiPreparedApplicationAuthority, WorthUiApplicationPreparationDenial> {
    let capability_snapshot = Rc::new(capability_snapshot);
    let declaration_artifacts = match preparation_source.declaration_source() {
        WorthUiApplicationDeclarationSource::Declared(dsl_package) => {
            lower_declaration_artifacts(dsl_package)
        }
        WorthUiApplicationDeclarationSource::SourceBacked(declaration_source) => {
            lower_source_backed_declaration_artifacts(declaration_source)
        }
    };
    let graph_handoffs = lower_graph_handoffs(&declaration_artifacts)
        .map_err(WorthUiApplicationPreparationDenial::GraphHandoff)?;
    let graph_handoffs =
        expand_runtime_instance_handoffs(&graph_handoffs, &runtime_instance_basis_admissions);
    let graph_snapshot = admit_graph_handoffs(&graph_handoffs, &runtime_instance_basis_admissions)
        .map_err(WorthUiApplicationPreparationDenial::GraphAdmission)?
        .commit_initial_generation(graph_world_profile)
        .map_err(WorthUiApplicationPreparationDenial::GraphCommit)?
        .into_committed_snapshot();
    let retained_measurement_inspection_evidence = measurement_inspection_evidence.clone();
    let lifecycle = WorthUiFacadeLifecycleBootstrap::bootstrap_with_inspection_scope_inventory(
        &declaration_artifacts,
        measurement_inspection_evidence,
        worth_ui_inspection::RUNTIME_INSPECTION_SCOPE_INVENTORY,
    );
    let (canonical_artifact, declaration_source_identity) =
        preparation_source.into_prepared_parts();

    Ok(WorthUiPreparedApplicationAuthority::seal(
        WorthUiPreparedApplicationAuthorityInput {
            capability_snapshot,
            canonical_artifact,
            declaration_source_identity,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
            query_binding_plan,
            host_session_plan,
            runtime_instance_basis_admissions,
            measurement_inspection_evidence: retained_measurement_inspection_evidence,
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
    if candidate_snapshot_digest != current.capabilities().digest().as_u64() {
        return Err(
            WorthUiApplicationPreparationDenial::CandidateSnapshotMismatch {
                candidate_snapshot_digest,
                prepared_snapshot_digest: current.capabilities().digest().as_u64(),
            },
        );
    }
    let (canonical_artifact, candidate, declaration_source, declaration_source_identity) =
        submission
            .into_replacement_handoff()
            .into_replacement_parts();
    let declaration_artifacts = lower_source_backed_declaration_artifacts(&declaration_source);
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
            declaration_source_identity,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
            query_binding_plan: current.query_binding_plan().clone(),
            host_session_plan: current.host_session_plan().clone(),
            runtime_instance_basis_admissions: admissions.to_vec().into_boxed_slice(),
            measurement_inspection_evidence,
        });
    Ok((authority, candidate))
}
