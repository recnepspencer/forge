use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryAdmittedInvalidationImpact,
    WorthQueryInvalidationMaintenancePhase, WorthQueryOperationPhaseProof,
    WorthQueryOperationResultState, WorthQuerySemanticDependencyRole,
    WorthQuerySettledDomainProjection,
};

use super::{
    WorthQueryCoalescedMaintenancePlan, WorthQueryMaintenanceScope, WorthQueryMaintenanceStrategy,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryMaintenanceDenial {
    ForeignOrStaleOperation,
    UnsupportedEscalation,
    PerformedEffectUnavailable,
}

pub struct WorthQueryPerformedMaintenance {
    pub(super) phase: WorthQueryOperationPhaseProof<WorthQueryInvalidationMaintenancePhase>,
    pub(super) strategies: Vec<WorthQueryMaintenanceStrategy>,
    pub(super) scope: WorthQueryMaintenanceScope,
    pub(super) roles: Vec<WorthQuerySemanticDependencyRole>,
    pub(super) result_state: WorthQueryOperationResultState,
    pub(super) execution_identity: String,
    pub(super) publication_identity: String,
    pub(super) effect: Arc<super::WorthQueryPerformedMaintenanceEffect>,
}

impl WorthQueryPerformedMaintenance {
    pub fn strategy(&self) -> WorthQueryMaintenanceStrategy {
        self.strategies[0]
    }

    pub fn strategies(&self) -> &[WorthQueryMaintenanceStrategy] {
        &self.strategies
    }

    pub const fn scope(&self) -> &WorthQueryMaintenanceScope {
        &self.scope
    }

    pub fn roles(&self) -> &[WorthQuerySemanticDependencyRole] {
        &self.roles
    }

    pub const fn result_state(&self) -> WorthQueryOperationResultState {
        self.result_state
    }

    pub fn effect(&self) -> &super::WorthQueryPerformedMaintenanceEffect {
        &self.effect
    }
}

pub(crate) fn bind_performed_invalidation_maintenance<D, O, F, L: BasisOperationLane>(
    admitted: Vec<WorthQueryAdmittedInvalidationImpact>,
    plan: &WorthQueryCoalescedMaintenancePlan,
    current: &WorthQuerySettledDomainProjection<D, O, F, L>,
    performed: &crate::domain_installation::WorthQueryLiveProjectionRefresh,
    effect: Arc<super::WorthQueryPerformedMaintenanceEffect>,
) -> Result<WorthQueryPerformedMaintenance, WorthQueryMaintenanceDenial> {
    let closure = current.semantic_aspect_dependency_closure();
    let first = admitted
        .first()
        .ok_or(WorthQueryMaintenanceDenial::ForeignOrStaleOperation)?;
    if admitted.iter().any(|impact| {
        operation_phase_basis(&impact.phase) != &impact.affinity
            || impact.affinity != first.affinity
    }) || closure.invalidation_manifest().operation_identity()
        != first.affinity.operation_identity
        || closure.invalidation_manifest().installation_generation()
            != first.affinity.installation_generation
        || performed.work().authority_checks() != 1
        || performed.work().drain_calls() != 1
        || performed.work().read_calls() != 1
        || performed.work().projection_calls() != 1
    {
        return Err(WorthQueryMaintenanceDenial::ForeignOrStaleOperation);
    }
    let execution_identity = performed.authority().receipt().receipt_digest().to_owned();
    let publication_identity = current.publication_receipt().identity().to_owned();
    let parent_identity = crate::identity::hash_parts(
        &std::iter::once("worth_query_coalesced_invalidation_parent_v1".to_owned())
            .chain(
                admitted
                    .iter()
                    .map(|impact| impact.phase.payload().identity().to_owned()),
            )
            .collect::<Vec<_>>(),
    );
    let phase = mint_operation_phase_proof(
        format!("invalidation-maintenance:{execution_identity}"),
        Some(&parent_identity),
        first.affinity.clone(),
    );
    Ok(WorthQueryPerformedMaintenance {
        phase,
        strategies: plan.strategies().to_vec(),
        scope: plan.scope().clone(),
        roles: plan.roles().to_vec(),
        result_state: current.result_state(),
        execution_identity,
        publication_identity,
        effect,
    })
}
