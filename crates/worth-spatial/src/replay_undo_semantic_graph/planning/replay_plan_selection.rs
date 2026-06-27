use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{SpatialReplayPlanError, SpatialReplaySelectedPlan};
use crate::replay_family_catalog::{
    current_spatial_replay_family_catalog, SpatialReplayFamilyLocalityPosture,
    SpatialReplayFamilyPriorProofPosture, SpatialReplayFamilyScopeProductPosture,
    SpatialReplayFamilyStageIndexPosture, SpatialReplayFamilyWorkloadDependencyPosture,
};
use crate::replay_undo_semantic_graph::SpatialReplaySemanticGraphAdmittedInput;

pub fn select_spatial_replay_plan<'a>(
    admitted_input: &'a SpatialReplaySemanticGraphAdmittedInput<'a>,
) -> Result<SpatialReplaySelectedPlan<'a>, SpatialReplayPlanError> {
    let catalog = current_spatial_replay_family_catalog();
    let declaration = catalog
        .require_family(admitted_input.family_identity())
        .expect("admitted replay input must come from a declared replay family");
    if declaration.locality_posture()
        != SpatialReplayFamilyLocalityPosture::RequiresSpatialTouchAuthority
    {
        return Err(SpatialReplayPlanError::UnsupportedLocalityPosture {
            family_identity: declaration.identity(),
            locality_posture: declaration.locality_posture(),
        });
    }
    if declaration.prior_proof_posture()
        != SpatialReplayFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt
    {
        return Err(SpatialReplayPlanError::UnsupportedPriorProofPosture {
            family_identity: declaration.identity(),
            prior_proof_posture: declaration.prior_proof_posture(),
        });
    }
    if declaration.stage_index_posture()
        != SpatialReplayFamilyStageIndexPosture::RequiresStageIndexIdentity
    {
        return Err(SpatialReplayPlanError::UnsupportedStageIndexPosture {
            family_identity: declaration.identity(),
            stage_index_posture: declaration.stage_index_posture(),
        });
    }
    if declaration.scope_product_posture()
        != SpatialReplayFamilyScopeProductPosture::RequiresSpatialReplayScopeProduct
    {
        return Err(SpatialReplayPlanError::UnsupportedScopeProductPosture {
            family_identity: declaration.identity(),
            scope_product_posture: declaration.scope_product_posture(),
        });
    }

    let admitted_input_semantic_graph_identity = admitted_input.semantic_graph_identity();
    let lookup_consumed_workload_handoff_identity =
        lower_lookup_consumed_workload_handoff_identity(admitted_input);
    let retained_replay_receipt_identity = admitted_input
        .retained_replay_receipt()
        .map(|receipt| receipt.identity().receipt_identity());
    let selected_plan_identity = lower_selected_plan_identity(
        admitted_input,
        &admitted_input_semantic_graph_identity,
        &lookup_consumed_workload_handoff_identity,
        retained_replay_receipt_identity.as_deref(),
        declaration.workload_dependency_posture(),
    );

    Ok(SpatialReplaySelectedPlan::new(
        declaration.identity(),
        admitted_input,
        admitted_input_semantic_graph_identity,
        lookup_consumed_workload_handoff_identity,
        retained_replay_receipt_identity,
        declaration.covered_lookup_identity(),
        declaration.workload_dependency_posture(),
        declaration.scope_product_posture(),
        selected_plan_identity,
    ))
}

fn lower_lookup_consumed_workload_handoff_identity(
    admitted_input: &SpatialReplaySemanticGraphAdmittedInput<'_>,
) -> String {
    admitted_input
        .lookup_consumed_workload_handoff()
        .semantic_graph_identity()
}

fn lower_selected_plan_identity(
    admitted_input: &SpatialReplaySemanticGraphAdmittedInput<'_>,
    admitted_input_semantic_graph_identity: &str,
    lookup_consumed_workload_handoff_identity: &str,
    retained_replay_receipt_identity: Option<&str>,
    workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:replay-undo-semantic-graph:selected-plan:v1".to_string(),
            format!("family:{}", admitted_input.family_identity().as_str()),
            format!("admitted:{}", admitted_input_semantic_graph_identity),
            format!(
                "spatial-touch-digest:{}",
                admitted_input.spatial_touch_authority().digest().as_str()
            ),
            format!(
                "prior-proof:{}",
                admitted_input.prior_proof_identity().digest()
            ),
            format!(
                "stage-index:{}",
                admitted_input.stage_index_identity().digest()
            ),
            format!(
                "lookup-handoff:{}",
                lookup_consumed_workload_handoff_identity
            ),
            format!(
                "retained-replay:{}",
                retained_replay_receipt_identity.unwrap_or("not-required")
            ),
            format!(
                "workload-dependency:{}",
                workload_dependency_posture_as_str(workload_dependency_posture)
            ),
        ],
    )
}

fn workload_dependency_posture_as_str(
    posture: SpatialReplayFamilyWorkloadDependencyPosture,
) -> &'static str {
    match posture {
        SpatialReplayFamilyWorkloadDependencyPosture::LookupReceiptOnly => "lookup-receipt-only",
        SpatialReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay => {
            "lookup-consumed-workload-and-retained-replay"
        }
    }
}
