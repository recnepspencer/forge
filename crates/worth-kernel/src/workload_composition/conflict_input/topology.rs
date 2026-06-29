use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_participant_identity,
    admit_conflict_routing_contract, ConflictAspectClass, ConflictOverlapIdentityInput,
    ConflictParticipantIdentity, ConflictParticipantIdentityInput, ConflictPriorProofIdentity,
    ConflictPriorProofInput, ConflictRoutingContract, ConflictRoutingPosture,
};
use topology::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;
use topology::facade::TopologyTouchedAspect;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::worth_workload::AdmittedBooleanSplitReplayUndoBoundary;
use crate::workload_composition::WorkloadCompositionError;

use super::{ConflictInputAdmissionError, ConflictInputAdmissionErrorKind};

#[derive(Clone, Copy)]
enum TopologyConflictRoute<'a> {
    Unset,
    AspectLocality(TopologyTouchedAspect),
    ReplayBoundary(&'a AdmittedBooleanSplitReplayUndoBoundary),
}

#[derive(Clone, Copy)]
pub enum AdmittedTopologyConflictRoute<'a> {
    AspectLocality(TopologyTouchedAspect),
    ReplayBoundary(&'a AdmittedBooleanSplitReplayUndoBoundary),
}

#[derive(Clone, Copy)]
pub struct TopologyConflictInputRequest<'a> {
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    route: TopologyConflictRoute<'a>,
}

#[derive(Clone)]
pub struct AdmittedTopologyConflictInput<'a> {
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    route: AdmittedTopologyConflictRoute<'a>,
    routing_contract: ConflictRoutingContract,
    admission_digest: String,
}

impl<'a> TopologyConflictInputRequest<'a> {
    pub fn new(touched_closure: &'a DerivedInvalidationTouchedClosure) -> Self {
        Self {
            touched_closure,
            route: TopologyConflictRoute::Unset,
        }
    }

    pub fn with_touched_aspect(self, aspect: TopologyTouchedAspect) -> Self {
        Self {
            touched_closure: self.touched_closure,
            route: TopologyConflictRoute::AspectLocality(aspect),
        }
    }

    pub fn with_replay_boundary(
        self,
        boundary: &'a AdmittedBooleanSplitReplayUndoBoundary,
    ) -> Self {
        Self {
            touched_closure: self.touched_closure,
            route: TopologyConflictRoute::ReplayBoundary(boundary),
        }
    }
}

impl<'a> AdmittedTopologyConflictInput<'a> {
    pub const fn touched_closure(&self) -> &'a DerivedInvalidationTouchedClosure {
        self.touched_closure
    }

    pub const fn route(&self) -> AdmittedTopologyConflictRoute<'a> {
        self.route
    }

    pub const fn routing_contract(&self) -> &ConflictRoutingContract {
        &self.routing_contract
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }
}

pub fn admit_topology_conflict_input<'a>(
    request: TopologyConflictInputRequest<'a>,
) -> Result<AdmittedTopologyConflictInput<'a>, WorkloadCompositionError> {
    let locality = request
        .touched_closure
        .conflict_locality_identity()
        .map_err(conflict_vocabulary_error)?;
    let (route, routing_contract, route_digest_part) = match request.route {
        TopologyConflictRoute::Unset => {
            return Err(WorkloadCompositionError::ConflictInput(
                ConflictInputAdmissionError::new(
                    ConflictInputAdmissionErrorKind::MissingTopologyConflictRoute,
                    "topology conflict input requires an explicit touched aspect or typed replay boundary proof",
                ),
            ));
        }
        TopologyConflictRoute::AspectLocality(aspect) => {
            if !request.touched_closure.basis().aspects().contains(&aspect) {
                return Err(WorkloadCompositionError::ConflictInput(
                    ConflictInputAdmissionError::new(
                        ConflictInputAdmissionErrorKind::MissingTouchedAspect,
                        "topology conflict input requires a declared touched aspect present in the sealed touched closure",
                    ),
                ));
            }
            let participants = topology_participants(request.touched_closure)?;
            let overlap = admit_conflict_overlap_identity(ConflictOverlapIdentityInput::aspect(
                ConflictAspectClass::WorthTopologyTouched(aspect),
                locality,
                participants,
            ))
            .map_err(conflict_vocabulary_error)?;
            (
                AdmittedTopologyConflictRoute::AspectLocality(aspect),
                admit_conflict_routing_contract(
                    overlap,
                    ConflictPriorProofInput::none(),
                    ConflictRoutingPosture::RequiresFamilySelection,
                ),
                format!("aspect-locality:{aspect:?}"),
            )
        }
        TopologyConflictRoute::ReplayBoundary(boundary) => {
            let packet = boundary.transaction_boundary_packet();
            if packet.touched_digest() != request.touched_closure.closure_digest() {
                return Err(WorkloadCompositionError::ConflictInput(
                    ConflictInputAdmissionError::new(
                        ConflictInputAdmissionErrorKind::WrongAuthority,
                        "topology replay conflict input requires one matching touched closure and replay/undo boundary packet touched digest",
                    ),
                ));
            }
            let prior_proof_identities = vec![
                ConflictPriorProofIdentity::from(packet.invalidation_receipt_identity().clone()),
                ConflictPriorProofIdentity::from(packet.replay_scope_identity().clone()),
                ConflictPriorProofIdentity::from(packet.undo_scope_identity().clone()),
            ];
            let overlap = admit_conflict_overlap_identity(
                ConflictOverlapIdentityInput::replay_undo(locality, prior_proof_identities.clone()),
            )
            .map_err(conflict_vocabulary_error)?;
            (
                AdmittedTopologyConflictRoute::ReplayBoundary(boundary),
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
            "worth-kernel:topology-conflict-input:v1".to_string(),
            format!("route:{route_digest_part}"),
            format!(
                "touched-closure:{}",
                request.touched_closure.closure_digest()
            ),
            format!("routing-contract:{}", routing_contract.contract_digest()),
        ],
    );
    Ok(AdmittedTopologyConflictInput {
        touched_closure: request.touched_closure,
        route,
        routing_contract,
        admission_digest,
    })
}

fn topology_participants(
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> Result<Vec<ConflictParticipantIdentity>, WorkloadCompositionError> {
    let participants = touched_closure
        .basis()
        .entities()
        .iter()
        .map(|entity| {
            admit_conflict_participant_identity(ConflictParticipantIdentityInput::entity(
                entity.entity_id(),
            ))
        })
        .chain(touched_closure.basis().relations().iter().map(|relation| {
            admit_conflict_participant_identity(ConflictParticipantIdentityInput::relation(
                relation.relation_id(),
            ))
        }))
        .collect::<Result<Vec<_>, _>>()
        .map_err(conflict_vocabulary_error)?;
    if participants.is_empty() {
        Err(WorkloadCompositionError::ConflictInput(
            ConflictInputAdmissionError::new(
                ConflictInputAdmissionErrorKind::MissingTouchedParticipants,
                "topology aspect-aware conflict input requires at least one touched entity or relation participant",
            ),
        ))
    } else {
        Ok(participants)
    }
}

fn conflict_vocabulary_error(
    error: schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingVocabularyError,
) -> WorkloadCompositionError {
    WorkloadCompositionError::ConflictInput(ConflictInputAdmissionError::new(
        ConflictInputAdmissionErrorKind::WrongAuthority,
        error.human_reason(),
    ))
}
