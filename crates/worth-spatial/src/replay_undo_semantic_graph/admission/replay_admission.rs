use schema::facade::platform::authority::replay_undo_semantic_graph_internal::{
    admit_replay_undo_stage_index_identity, admit_spatial_evidence_lookup_prior_proof_identity,
};

use super::admission_error::SpatialReplaySemanticGraphAdmissionError;
use super::admitted_input::{
    SpatialReplaySemanticGraphAdmittedInput, SpatialUndoSemanticGraphAdmittedInput,
};
use super::preparation::prepare_spatial_replay_semantic_graph_request;
use super::replay_request::{
    SpatialReplaySemanticGraphAdmissionRequest, SpatialReplaySemanticGraphPreparedRequest,
};
use super::undo_request::SpatialUndoSemanticGraphAdmissionRequest;
use crate::replay_family_catalog::{
    SpatialReplayFamilyCatalog, SpatialReplayFamilyDeclaration,
    SpatialReplayFamilyWorkloadDependencyPosture,
};
use crate::undo_family_catalog::{
    current_spatial_undo_family_catalog, SpatialUndoFamilyWorkloadDependencyPosture,
};
use crate::workload_platform::evidence_ledger::{
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStage, WorkloadEvidenceStageIndexProduct,
};
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

pub fn admit_spatial_replay_semantic_graph_input<'a>(
    catalog: &SpatialReplayFamilyCatalog,
    request: SpatialReplaySemanticGraphAdmissionRequest<'a>,
) -> Result<SpatialReplaySemanticGraphAdmittedInput<'a>, SpatialReplaySemanticGraphAdmissionError> {
    let prepared_request = prepare_spatial_replay_semantic_graph_request(request)?;
    admit_prepared_spatial_replay_semantic_graph_input(catalog, &prepared_request)
}

pub fn admit_prepared_spatial_replay_semantic_graph_input<'a>(
    catalog: &SpatialReplayFamilyCatalog,
    prepared_request: &SpatialReplaySemanticGraphPreparedRequest<'a>,
) -> Result<SpatialReplaySemanticGraphAdmittedInput<'a>, SpatialReplaySemanticGraphAdmissionError> {
    let family_declaration = catalog
        .require_family(prepared_request.family_identity())
        .ok_or(
            SpatialReplaySemanticGraphAdmissionError::MissingCoveredFamily {
                family_identity: prepared_request.family_identity(),
            },
        )?;
    require_covered_family(prepared_request, family_declaration)?;
    let retained_replay_receipt = require_family_workload_dependency(
        prepared_request,
        family_declaration.workload_dependency_posture(),
    )?;

    Ok(SpatialReplaySemanticGraphAdmittedInput::new(
        family_declaration.identity(),
        prepared_request.spatial_touch_authority(),
        prepared_request.prior_proof_identity().clone(),
        prepared_request.stage_index_identity().clone(),
        prepared_request.lookup_consumed_workload_handoff(),
        retained_replay_receipt,
    ))
}

pub fn admit_spatial_undo_semantic_graph_input<'a>(
    request: SpatialUndoSemanticGraphAdmissionRequest<'a>,
) -> Result<SpatialUndoSemanticGraphAdmittedInput<'a>, SpatialReplaySemanticGraphAdmissionError> {
    let catalog = current_spatial_undo_family_catalog();
    let declaration = catalog.require_family(request.family_identity()).ok_or(
        SpatialReplaySemanticGraphAdmissionError::MissingUndoFamily {
            family_identity: request.family_identity(),
        },
    )?;
    require_matching_undo_stage_index_identity(
        request.spatial_touch_authority(),
        request.stage_index_product(),
    )?;
    let lookup_consumed_workload_handoff = match declaration.workload_dependency_posture() {
        SpatialUndoFamilyWorkloadDependencyPosture::LookupReceiptOnly => {
            if request.lookup_consumed_workload_handoff().is_some() {
                return Err(
                    SpatialReplaySemanticGraphAdmissionError::UnexpectedLookupConsumedWorkload {
                        family_identity: declaration.identity(),
                    },
                );
            }
            None
        }
        SpatialUndoFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkload => {
            let handoff = request.lookup_consumed_workload_handoff().ok_or(
                SpatialReplaySemanticGraphAdmissionError::MissingRequiredLookupConsumedWorkload {
                    family_identity: declaration.identity(),
                },
            )?;
            require_matching_undo_stage_receipt_identity(
                request.spatial_touch_authority(),
                handoff.stage_receipt_identity(),
            )?;
            require_matching_undo_lookup_execution_receipt_identity(
                request.evidence_lookup_receipt(),
                handoff.lookup_execution_receipt_digest(),
            )?;
            Some(handoff)
        }
    };
    Ok(SpatialUndoSemanticGraphAdmittedInput::new(
        declaration.identity(),
        request.spatial_touch_authority(),
        admit_spatial_evidence_lookup_prior_proof_identity(
            request.evidence_lookup_receipt().execution_receipt_digest(),
        ),
        admit_replay_undo_stage_index_identity(request.stage_index_product().index_identity()),
        lookup_consumed_workload_handoff,
    ))
}

fn require_covered_family(
    prepared_request: &SpatialReplaySemanticGraphPreparedRequest<'_>,
    family_declaration: &SpatialReplayFamilyDeclaration,
) -> Result<(), SpatialReplaySemanticGraphAdmissionError> {
    let required_covered_family_identity = family_declaration.covered_lookup_identity().as_str();
    let family_is_covered = prepared_request
        .lookup_consumed_workload_handoff()
        .covered_family_identities()
        .iter()
        .any(|identity| identity == required_covered_family_identity);
    if family_is_covered {
        return Ok(());
    }
    Err(
        SpatialReplaySemanticGraphAdmissionError::MissingCoveredFamily {
            family_identity: prepared_request.family_identity(),
        },
    )
}

fn require_family_workload_dependency<'a>(
    prepared_request: &SpatialReplaySemanticGraphPreparedRequest<'a>,
    workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture,
) -> Result<Option<&'a RetainedReplayWorkloadReceipt>, SpatialReplaySemanticGraphAdmissionError> {
    match workload_dependency_posture {
        SpatialReplayFamilyWorkloadDependencyPosture::LookupReceiptOnly => {
            if prepared_request.retained_replay_receipt().is_some() {
                return Err(
                    SpatialReplaySemanticGraphAdmissionError::UnexpectedRetainedReplayReceipt {
                        family_identity: prepared_request.family_identity(),
                    },
                );
            }
            Ok(None)
        }
        SpatialReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay => {
            let retained_replay_receipt =
                prepared_request.retained_replay_receipt().ok_or(
                    SpatialReplaySemanticGraphAdmissionError::MissingRequiredRetainedReplayReceipt {
                        family_identity: prepared_request.family_identity(),
                    },
                )?;
            require_matching_retained_replay_receipt_identity(
                prepared_request.spatial_touch_authority(),
                retained_replay_receipt,
            )?;
            Ok(Some(retained_replay_receipt))
        }
    }
}

fn require_matching_retained_replay_receipt_identity(
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    retained_replay_receipt: &RetainedReplayWorkloadReceipt,
) -> Result<(), SpatialReplaySemanticGraphAdmissionError> {
    let authority_retained_replay_identity = spatial_touch_authority
        .authority_rows()
        .iter()
        .find(|row| row.stage() == WorkloadEvidenceStage::RetainedReplay)
        .map(|row| row.evidence_identity())
        .unwrap_or_default();
    let retained_replay_receipt_identity = retained_replay_receipt.identity().receipt_identity();
    if authority_retained_replay_identity == retained_replay_receipt_identity {
        return Ok(());
    }
    Err(
        SpatialReplaySemanticGraphAdmissionError::RetainedReplayReceiptMismatch {
            authority_retained_replay_identity: authority_retained_replay_identity.to_string(),
            retained_replay_receipt_identity,
        },
    )
}

fn require_matching_undo_stage_index_identity(
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    stage_index_product: &WorkloadEvidenceStageIndexProduct,
) -> Result<(), SpatialReplaySemanticGraphAdmissionError> {
    if spatial_touch_authority.stage_index_identity() == stage_index_product.index_identity() {
        return Ok(());
    }
    Err(
        SpatialReplaySemanticGraphAdmissionError::StageIndexIdentityMismatch {
            authority_stage_index_identity: spatial_touch_authority
                .stage_index_identity()
                .to_string(),
            product_stage_index_identity: stage_index_product.index_identity().to_string(),
        },
    )
}

fn require_matching_undo_stage_receipt_identity(
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    handoff_stage_receipt_identity: &str,
) -> Result<(), SpatialReplaySemanticGraphAdmissionError> {
    let authority_stage_receipt_identity = spatial_touch_authority.evidence_identity();
    if authority_stage_receipt_identity == handoff_stage_receipt_identity {
        return Ok(());
    }
    Err(
        SpatialReplaySemanticGraphAdmissionError::StageReceiptIdentityMismatch {
            authority_stage_receipt_identity: authority_stage_receipt_identity.to_string(),
            handoff_stage_receipt_identity: handoff_stage_receipt_identity.to_string(),
        },
    )
}

fn require_matching_undo_lookup_execution_receipt_identity(
    evidence_lookup_receipt: &EvidenceLookupExecutionReceipt,
    handoff_execution_digest: &str,
) -> Result<(), SpatialReplaySemanticGraphAdmissionError> {
    if evidence_lookup_receipt.execution_receipt_digest() == handoff_execution_digest {
        return Ok(());
    }
    Err(
        SpatialReplaySemanticGraphAdmissionError::LookupExecutionReceiptMismatch {
            receipt_execution_digest: evidence_lookup_receipt
                .execution_receipt_digest()
                .to_string(),
            handoff_execution_digest: handoff_execution_digest.to_string(),
        },
    )
}
