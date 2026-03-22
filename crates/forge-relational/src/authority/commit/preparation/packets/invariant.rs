use std::sync::Arc;

use crate::authority::commit::preparation::planning::context::PreparationPlanningContext;
use crate::authority::commit::preparation::proofs::kinds::PreparationProofKind;
use crate::authority::commit::preparation::proofs::locality::PreparationLocalityProof;
use crate::authority::commit::preparation::proofs::validity::PreparationProofValidity;
use crate::authority::commit::preparation::reduction::keys::ValidationReductionKey;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::InvariantRegistration;
use crate::validation::engine::{InvariantObservation, PreparedRelationIntegrityScopes};

#[derive(Clone)]
pub(crate) struct InvariantWorkPacket<'runtime> {
    pub(crate) packet_index: usize,
    pub(crate) registration: InvariantRegistration,
    pub(crate) reduction_key: ValidationReductionKey,
    pub(crate) proof_kind: PreparationProofKind,
    pub(crate) locality: PreparationLocalityProof,
    pub(crate) validity: PreparationProofValidity,
    pub(crate) planning_context: Arc<PreparationPlanningContext>,
    pub(crate) observation: &'runtime InvariantObservation<'runtime>,
    pub(crate) version_id: crate::identity::data::VersionId,
    pub(crate) merged_plan: Option<&'runtime MergedCommitPlan>,
    pub(crate) relation_integrity_scopes: Option<PreparedRelationIntegrityScopes>,
}
