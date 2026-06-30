use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayUndoSemanticGraphPriorProofIdentity, ReplayUndoSemanticGraphStageIndexIdentity,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity;

use super::admission_error::TopologyReplaySemanticGraphAdmissionError;
use super::admitted_input::TopologyReplaySemanticGraphAdmittedInput;
use super::preparation::{
    prepare_legacy_topology_replay_semantic_graph_request,
    TopologyReplaySemanticGraphPreparedRequest,
};
use super::replay_request::TopologyReplaySemanticGraphAdmissionRequest;
use super::selected_plan_identity::TopologyReplaySemanticGraphSelectedPlanIdentity;
use crate::derived_topology::invalidation_plan::catalog::catalog_digest;
use crate::replay_family_catalog::{
    current_topology_replay_family_catalog, TopologyReplayFamilyIdentity,
    TopologyReplayFamilyLocalityPosture, TopologyReplayFamilyPriorProofPosture,
    TopologyReplayFamilyStageIndexPosture,
};

pub fn admit_topology_replay_semantic_graph_input<'a>(
    request: TopologyReplaySemanticGraphAdmissionRequest<'a>,
) -> Result<TopologyReplaySemanticGraphAdmittedInput<'a>, TopologyReplaySemanticGraphAdmissionError>
{
    admit_prepared_topology_replay_semantic_graph_input(
        prepare_legacy_topology_replay_semantic_graph_request(request),
    )
}

pub fn admit_prepared_topology_replay_semantic_graph_input<'a>(
    request: TopologyReplaySemanticGraphPreparedRequest<'a>,
) -> Result<TopologyReplaySemanticGraphAdmittedInput<'a>, TopologyReplaySemanticGraphAdmissionError>
{
    let catalog = current_topology_replay_family_catalog();
    let declaration = catalog.require_family(request.family_identity()).ok_or(
        TopologyReplaySemanticGraphAdmissionError::MissingReplayFamilyDeclaration {
            family_identity: request.family_identity(),
        },
    )?;

    if declaration.locality_posture() != TopologyReplayFamilyLocalityPosture::RequiresTouchedClosure
    {
        return Err(
            TopologyReplaySemanticGraphAdmissionError::UnsupportedLocalityPosture {
                family_identity: declaration.identity(),
                locality_posture: declaration.locality_posture(),
            },
        );
    }

    if declaration.prior_proof_posture()
        != TopologyReplayFamilyPriorProofPosture::RequiresInvalidationSelectedPlanAndExecutionReceipt
    {
        return Err(
            TopologyReplaySemanticGraphAdmissionError::UnsupportedPriorProofPosture {
                family_identity: declaration.identity(),
                prior_proof_posture: declaration.prior_proof_posture(),
            },
        );
    }

    if request.touched_closure().closure_digest()
        != request.invalidation_receipt().touched_closure_digest()
    {
        return Err(
            TopologyReplaySemanticGraphAdmissionError::InvalidationReceiptTouchedClosureMismatch {
                touched_closure_digest: request.touched_closure().closure_digest().to_string(),
                receipt_touched_closure_digest: request
                    .invalidation_receipt()
                    .touched_closure_digest()
                    .to_string(),
            },
        );
    }

    let stage_identity = require_stage_identity(
        declaration.identity(),
        declaration.stage_index_posture(),
        &request,
    )?;

    Ok(TopologyReplaySemanticGraphAdmittedInput::new(
        declaration.identity(),
        request.touched_closure(),
        admit_topology_derived_invalidation_prior_proof_identity(
            request.invalidation_receipt().execution_receipt_digest(),
        ),
        TopologyReplaySemanticGraphSelectedPlanIdentity::from_invalidation_selected_plan_digest(
            request.invalidation_receipt().selected_plan_digest(),
        ),
        stage_identity,
    ))
}

fn require_stage_identity(
    family_identity: TopologyReplayFamilyIdentity,
    stage_index_posture: TopologyReplayFamilyStageIndexPosture,
    request: &TopologyReplaySemanticGraphPreparedRequest<'_>,
) -> Result<
    super::stage_identity::TopologyReplaySemanticGraphStageIdentity,
    TopologyReplaySemanticGraphAdmissionError,
> {
    match stage_index_posture {
        TopologyReplayFamilyStageIndexPosture::RequiresStageIndexIdentity => {
            let stage_authority = request.stage_authority().ok_or(
                TopologyReplaySemanticGraphAdmissionError::MissingRequiredStageReceiptAuthority {
                    family_identity,
                },
            )?;
            let declared_stage_identity = request.declared_stage_identity().ok_or(
                TopologyReplaySemanticGraphAdmissionError::MissingRequiredStageIdentity {
                    family_identity,
                },
            )?;
            require_matching_stage_receipt_family(
                family_identity,
                stage_authority.family_identity(),
            )?;
            require_matching_stage_receipt_selected_plan(
                request.invalidation_receipt().selected_plan_digest(),
                stage_authority.selected_plan_digest(),
            )?;
            require_matching_stage_receipt_touched_closure(
                request.touched_closure().closure_digest(),
                stage_authority.touched_closure_digest(),
            )?;
            require_matching_stage_identity(declared_stage_identity, stage_authority)?;
            Ok(declared_stage_identity.clone())
        }
    }
}

fn require_matching_stage_receipt_family(
    family_identity: TopologyReplayFamilyIdentity,
    stage_receipt_family_identity: TopologyReplayFamilyIdentity,
) -> Result<(), TopologyReplaySemanticGraphAdmissionError> {
    if family_identity == stage_receipt_family_identity {
        return Ok(());
    }
    Err(
        TopologyReplaySemanticGraphAdmissionError::StageReceiptFamilyMismatch {
            family_identity,
            stage_receipt_family_identity,
        },
    )
}

fn require_matching_stage_receipt_selected_plan(
    invalidation_selected_plan_digest: &str,
    stage_receipt_selected_plan_digest: &str,
) -> Result<(), TopologyReplaySemanticGraphAdmissionError> {
    if invalidation_selected_plan_digest == stage_receipt_selected_plan_digest {
        return Ok(());
    }
    Err(
        TopologyReplaySemanticGraphAdmissionError::StageReceiptSelectedPlanMismatch {
            invalidation_selected_plan_digest: invalidation_selected_plan_digest.to_string(),
            stage_receipt_selected_plan_digest: stage_receipt_selected_plan_digest.to_string(),
        },
    )
}

fn require_matching_stage_receipt_touched_closure(
    touched_closure_digest: &str,
    stage_receipt_touched_closure_digest: &str,
) -> Result<(), TopologyReplaySemanticGraphAdmissionError> {
    if touched_closure_digest == stage_receipt_touched_closure_digest {
        return Ok(());
    }
    Err(
        TopologyReplaySemanticGraphAdmissionError::StageReceiptTouchedClosureMismatch {
            touched_closure_digest: touched_closure_digest.to_string(),
            stage_receipt_touched_closure_digest: stage_receipt_touched_closure_digest.to_string(),
        },
    )
}

fn require_matching_stage_identity(
    declared_stage_identity: &super::stage_identity::TopologyReplaySemanticGraphStageIdentity,
    stage_authority: &super::stage_authority::TopologyReplaySemanticGraphPreparedStageAuthority,
) -> Result<(), TopologyReplaySemanticGraphAdmissionError> {
    if declared_stage_identity == stage_authority.stage_identity() {
        return Ok(());
    }
    Err(
        TopologyReplaySemanticGraphAdmissionError::StageIdentityMismatch {
            declared_stage_identity_digest: declared_stage_identity.digest().to_string(),
            stage_receipt_stage_identity_digest: stage_authority
                .stage_identity()
                .digest()
                .to_string(),
        },
    )
}

pub(crate) fn replay_admission_digest(
    family_identity: TopologyReplayFamilyIdentity,
    touched_closure: &crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure,
    prior_proof_identity: &ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: &ReplayUndoSemanticGraphStageIndexIdentity,
) -> String {
    catalog_digest([
        "worth-topo:replay-undo-semantic-graph:topology-replay-admitted-input:v1".to_string(),
        format!("family:{}", family_identity.as_str()),
        format!("touched-closure:{}", touched_closure.closure_digest()),
        format!("prior-proof:{}", prior_proof_identity.digest_part()),
        stage_index_identity.digest_part(),
    ])
}
