use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_routing_contract, ConflictOverlapIdentityInput,
    ConflictPriorProofIdentity, ConflictPriorProofInput, ConflictRoutingContract,
    ConflictRoutingPosture,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

use crate::workload_composition::worth_workload::AdmittedBooleanSplitReplayUndoBoundary;
use crate::workload_composition::WorkloadCompositionError;

use super::handoff_guards::require_honest_lookup_handoff;
use super::{ConflictInputAdmissionError, ConflictInputAdmissionErrorKind};

#[derive(Clone, Copy)]
enum SpatialConflictRoute<'a> {
    Unset,
    EvidenceLookup {
        handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
        execution_receipt: &'a EvidenceLookupExecutionReceipt,
    },
    ReplayBoundary(&'a AdmittedBooleanSplitReplayUndoBoundary),
}

#[derive(Clone, Copy)]
pub enum AdmittedSpatialConflictRoute<'a> {
    EvidenceLookup {
        handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
        execution_receipt: &'a EvidenceLookupExecutionReceipt,
    },
    ReplayBoundary(&'a AdmittedBooleanSplitReplayUndoBoundary),
}

#[derive(Clone, Copy)]
pub struct SpatialConflictInputRequest<'a> {
    authority: &'a SpatialGeometryEvidenceTouchAuthority,
    route: SpatialConflictRoute<'a>,
}

#[derive(Clone)]
pub struct AdmittedSpatialConflictInput<'a> {
    authority: &'a SpatialGeometryEvidenceTouchAuthority,
    route: AdmittedSpatialConflictRoute<'a>,
    routing_contract: ConflictRoutingContract,
    admission_digest: String,
}

impl<'a> SpatialConflictInputRequest<'a> {
    pub fn new(authority: &'a SpatialGeometryEvidenceTouchAuthority) -> Self {
        Self {
            authority,
            route: SpatialConflictRoute::Unset,
        }
    }

    pub fn with_evidence_lookup(
        self,
        handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
        execution_receipt: &'a EvidenceLookupExecutionReceipt,
    ) -> Self {
        Self {
            authority: self.authority,
            route: SpatialConflictRoute::EvidenceLookup {
                handoff,
                execution_receipt,
            },
        }
    }

    pub fn with_replay_boundary(
        self,
        boundary: &'a AdmittedBooleanSplitReplayUndoBoundary,
    ) -> Self {
        Self {
            authority: self.authority,
            route: SpatialConflictRoute::ReplayBoundary(boundary),
        }
    }
}

impl<'a> AdmittedSpatialConflictInput<'a> {
    pub const fn authority(&self) -> &'a SpatialGeometryEvidenceTouchAuthority {
        self.authority
    }

    pub const fn route(&self) -> AdmittedSpatialConflictRoute<'a> {
        self.route
    }

    pub const fn routing_contract(&self) -> &ConflictRoutingContract {
        &self.routing_contract
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }
}

pub fn admit_spatial_conflict_input<'a>(
    request: SpatialConflictInputRequest<'a>,
) -> Result<AdmittedSpatialConflictInput<'a>, WorkloadCompositionError> {
    let locality = request
        .authority
        .conflict_locality_identity()
        .map_err(conflict_vocabulary_error)?;
    let (route, routing_contract, route_digest_part) = match request.route {
        SpatialConflictRoute::Unset => {
            return Err(WorkloadCompositionError::ConflictInput(
                ConflictInputAdmissionError::new(
                    ConflictInputAdmissionErrorKind::MissingSpatialConflictRoute,
                    "spatial conflict input requires either evidence lookup proof or replay boundary proof",
                ),
            ));
        }
        SpatialConflictRoute::EvidenceLookup {
            handoff,
            execution_receipt,
        } => {
            require_honest_lookup_handoff(request.authority, handoff)
                .map_err(WorkloadCompositionError::ConflictInput)?;
            if execution_receipt.execution_receipt_digest()
                != handoff.lookup_execution_receipt_digest()
            {
                return Err(WorkloadCompositionError::ConflictInput(
                    ConflictInputAdmissionError::new(
                        ConflictInputAdmissionErrorKind::WrongReceiptFamily,
                        "spatial conflict input requires one matching lookup execution receipt for the admitted lookup handoff",
                    ),
                ));
            }
            if execution_receipt.selected_plan_digest() != handoff.selected_lookup_plan_digest() {
                return Err(WorkloadCompositionError::ConflictInput(
                    ConflictInputAdmissionError::new(
                        ConflictInputAdmissionErrorKind::WrongReceiptFamily,
                        "spatial conflict input requires lookup execution proof whose selected family plan agrees with the admitted lookup handoff",
                    ),
                ));
            }
            let authority_participant = request
                .authority
                .conflict_participant_identity()
                .map_err(conflict_vocabulary_error)?;
            let overlap = admit_conflict_overlap_identity(ConflictOverlapIdentityInput::evidence(
                locality,
                vec![authority_participant],
            ))
            .map_err(conflict_vocabulary_error)?;
            (
                AdmittedSpatialConflictRoute::EvidenceLookup {
                    handoff,
                    execution_receipt,
                },
                admit_conflict_routing_contract(
                    overlap,
                    ConflictPriorProofInput::none(),
                    ConflictRoutingPosture::RequiresFamilySelection,
                ),
                format!(
                    "evidence-lookup:{}:{}",
                    handoff.semantic_graph_identity(),
                    execution_receipt.execution_receipt_digest()
                ),
            )
        }
        SpatialConflictRoute::ReplayBoundary(boundary) => {
            let packet = boundary.transaction_boundary_packet();
            let handoff = boundary
                .completed_split_handoff()
                .lookup_consumed_workload_handoff();
            require_honest_lookup_handoff(request.authority, handoff)
                .map_err(WorkloadCompositionError::ConflictInput)?;
            if packet.stage_index_identity().digest() != request.authority.stage_index_identity() {
                return Err(WorkloadCompositionError::ConflictInput(
                    ConflictInputAdmissionError::new(
                        ConflictInputAdmissionErrorKind::StageIndexMismatch,
                        "spatial replay conflict input requires matching spatial touch authority and replay boundary packet stage-index identity",
                    ),
                ));
            }
            if packet.evidence_lookup_receipt_identity().digest()
                != handoff.lookup_execution_receipt_digest()
            {
                return Err(WorkloadCompositionError::ConflictInput(
                    ConflictInputAdmissionError::new(
                        ConflictInputAdmissionErrorKind::WrongAuthority,
                        "spatial replay conflict input requires one matching spatial touch authority lookup execution identity",
                    ),
                ));
            }
            let prior_proof_identities = vec![
                ConflictPriorProofIdentity::from(packet.evidence_lookup_receipt_identity().clone()),
                ConflictPriorProofIdentity::from(packet.replay_scope_identity().clone()),
                ConflictPriorProofIdentity::from(packet.undo_scope_identity().clone()),
            ];
            let overlap = admit_conflict_overlap_identity(
                ConflictOverlapIdentityInput::replay_undo(locality, prior_proof_identities.clone()),
            )
            .map_err(conflict_vocabulary_error)?;
            (
                AdmittedSpatialConflictRoute::ReplayBoundary(boundary),
                admit_conflict_routing_contract(
                    overlap,
                    ConflictPriorProofInput::from_identities(prior_proof_identities),
                    ConflictRoutingPosture::RequiresFamilySelection,
                ),
                format!("replay-boundary:{}", boundary.packet_identity()),
            )
        }
    };
    let admission_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:spatial-conflict-input:v1".to_string(),
            format!("route:{route_digest_part}"),
            format!("spatial-touch:{}", request.authority.digest().as_str()),
            format!("routing-contract:{}", routing_contract.contract_digest()),
        ],
    );
    Ok(AdmittedSpatialConflictInput {
        authority: request.authority,
        route,
        routing_contract,
        admission_digest,
    })
}

fn conflict_vocabulary_error(
    error: schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingVocabularyError,
) -> WorkloadCompositionError {
    WorkloadCompositionError::ConflictInput(ConflictInputAdmissionError::new(
        ConflictInputAdmissionErrorKind::WrongAuthority,
        error.human_reason(),
    ))
}
