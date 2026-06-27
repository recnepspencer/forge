use super::super::ReplayUndoTransactionBoundaryInput;
use super::super::ReplayUndoTransactionBoundaryPacketCounters;
use super::assembly_error::ReplayUndoTransactionBoundaryAssemblyError;
use super::packet_support_posture::{
    lower_replay_undo_transaction_boundary_support_posture,
    ReplayUndoTransactionBoundarySupportSource,
};
use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoTransactionScopeClaim;
use topology::facade::TopologyUndoScopeProduct;
use worth_spatial::facade::replay_undo_semantic_graph::{
    SpatialReplayScopeProduct, SpatialUndoScopeProduct,
};

pub struct ReplayUndoTransactionBoundaryAssemblyRequest<'a> {
    topology_undo_scope_product: &'a TopologyUndoScopeProduct<'a>,
    spatial_replay_scope_product: &'a SpatialReplayScopeProduct<'a>,
    spatial_undo_scope_product: &'a SpatialUndoScopeProduct<'a>,
    support_source: ReplayUndoTransactionBoundarySupportSource,
    mutation_claims: Vec<ReplayUndoTransactionScopeClaim>,
    counters: ReplayUndoTransactionBoundaryPacketCounters,
}

impl<'a> ReplayUndoTransactionBoundaryAssemblyRequest<'a> {
    pub fn new(
        topology_undo_scope_product: &'a TopologyUndoScopeProduct<'a>,
        spatial_replay_scope_product: &'a SpatialReplayScopeProduct<'a>,
        spatial_undo_scope_product: &'a SpatialUndoScopeProduct<'a>,
        support_source: ReplayUndoTransactionBoundarySupportSource,
    ) -> Self {
        Self {
            topology_undo_scope_product,
            spatial_replay_scope_product,
            spatial_undo_scope_product,
            support_source,
            mutation_claims: Vec::new(),
            counters: ReplayUndoTransactionBoundaryPacketCounters::empty(),
        }
    }

    pub fn with_mutation_claims(
        self,
        mutation_claims: Vec<ReplayUndoTransactionScopeClaim>,
        counters: ReplayUndoTransactionBoundaryPacketCounters,
    ) -> Self {
        Self {
            topology_undo_scope_product: self.topology_undo_scope_product,
            spatial_replay_scope_product: self.spatial_replay_scope_product,
            spatial_undo_scope_product: self.spatial_undo_scope_product,
            support_source: self.support_source,
            mutation_claims,
            counters,
        }
    }

    pub const fn topology_undo_scope_product(&self) -> &'a TopologyUndoScopeProduct<'a> {
        self.topology_undo_scope_product
    }

    pub const fn spatial_replay_scope_product(&self) -> &'a SpatialReplayScopeProduct<'a> {
        self.spatial_replay_scope_product
    }

    pub const fn spatial_undo_scope_product(&self) -> &'a SpatialUndoScopeProduct<'a> {
        self.spatial_undo_scope_product
    }

    pub const fn support_source(&self) -> &ReplayUndoTransactionBoundarySupportSource {
        &self.support_source
    }

    pub fn mutation_claims(&self) -> &[ReplayUndoTransactionScopeClaim] {
        &self.mutation_claims
    }

    pub const fn counters(&self) -> &ReplayUndoTransactionBoundaryPacketCounters {
        &self.counters
    }
}

pub fn assemble_replay_undo_transaction_boundary_input(
    request: ReplayUndoTransactionBoundaryAssemblyRequest<'_>,
) -> Result<ReplayUndoTransactionBoundaryInput, ReplayUndoTransactionBoundaryAssemblyError> {
    require_matching_touched_subjects(
        request.spatial_replay_scope_product(),
        request.spatial_undo_scope_product(),
    )?;
    require_matching_evidence_lookup_prior_proof(
        request.spatial_replay_scope_product(),
        request.spatial_undo_scope_product(),
    )?;
    require_matching_stage_index(
        request.spatial_replay_scope_product(),
        request.spatial_undo_scope_product(),
    )?;

    Ok(ReplayUndoTransactionBoundaryInput::new(
        request
            .topology_undo_scope_product()
            .touched_closure()
            .closure_digest(),
        request
            .spatial_replay_scope_product()
            .stage_index_identity()
            .clone(),
        request
            .topology_undo_scope_product()
            .prior_proof_identity()
            .clone(),
        request
            .spatial_replay_scope_product()
            .prior_proof_identity()
            .clone(),
        request
            .spatial_replay_scope_product()
            .scope_identity()
            .clone(),
        request
            .spatial_undo_scope_product()
            .scope_identity()
            .clone(),
        lower_replay_undo_transaction_boundary_support_posture(match request.support_source {
            ReplayUndoTransactionBoundarySupportSource::Ordinary => {
                ReplayUndoTransactionBoundarySupportSource::Ordinary
            }
            ReplayUndoTransactionBoundarySupportSource::QueryGap {
                owner,
                blocker,
                removal_trigger,
            } => ReplayUndoTransactionBoundarySupportSource::QueryGap {
                owner,
                blocker,
                removal_trigger,
            },
        }),
        request.mutation_claims().to_vec(),
        request.counters().clone(),
    ))
}

fn require_matching_touched_subjects(
    replay_scope_product: &SpatialReplayScopeProduct<'_>,
    undo_scope_product: &SpatialUndoScopeProduct<'_>,
) -> Result<(), ReplayUndoTransactionBoundaryAssemblyError> {
    if replay_scope_product.equivalence_basis().touched_subjects()
        == undo_scope_product.equivalence_basis().touched_subjects()
    {
        return Ok(());
    }
    Err(ReplayUndoTransactionBoundaryAssemblyError::ReplayUndoTouchedSubjectMismatch)
}

fn require_matching_evidence_lookup_prior_proof(
    replay_scope_product: &SpatialReplayScopeProduct<'_>,
    undo_scope_product: &SpatialUndoScopeProduct<'_>,
) -> Result<(), ReplayUndoTransactionBoundaryAssemblyError> {
    if replay_scope_product.prior_proof_identity().digest()
        == undo_scope_product.prior_proof_identity().digest()
    {
        return Ok(());
    }
    Err(
        ReplayUndoTransactionBoundaryAssemblyError::ReplayUndoEvidenceLookupPriorProofMismatch {
            replay_prior_proof_digest: replay_scope_product
                .prior_proof_identity()
                .digest()
                .to_string(),
            undo_prior_proof_digest: undo_scope_product
                .prior_proof_identity()
                .digest()
                .to_string(),
        },
    )
}

fn require_matching_stage_index(
    replay_scope_product: &SpatialReplayScopeProduct<'_>,
    undo_scope_product: &SpatialUndoScopeProduct<'_>,
) -> Result<(), ReplayUndoTransactionBoundaryAssemblyError> {
    if replay_scope_product.stage_index_identity().digest()
        == undo_scope_product.stage_index_identity().digest()
    {
        return Ok(());
    }
    Err(
        ReplayUndoTransactionBoundaryAssemblyError::ReplayUndoStageIndexMismatch {
            replay_stage_index_digest: replay_scope_product
                .stage_index_identity()
                .digest()
                .to_string(),
            undo_stage_index_digest: undo_scope_product
                .stage_index_identity()
                .digest()
                .to_string(),
        },
    )
}
