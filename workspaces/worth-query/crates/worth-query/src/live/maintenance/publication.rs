use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryInvalidationPublishedPhase,
    WorthQueryOperationPhaseProof, WorthQueryOperationResultState,
    WorthQuerySettledDomainProjection,
};

use super::{
    WorthQueryMaintenanceScope, WorthQueryMaintenanceStrategy, WorthQueryPerformedMaintenance,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLivePublicationDenial {
    ForeignOrStaleMaintenance,
    ExecutionReceiptMismatch,
}

pub struct WorthQueryPublishedLiveDelivery {
    _phase: WorthQueryOperationPhaseProof<WorthQueryInvalidationPublishedPhase>,
    strategies: Vec<WorthQueryMaintenanceStrategy>,
    scope: WorthQueryMaintenanceScope,
    result_state: WorthQueryOperationResultState,
    roles: Vec<crate::domain_installation::WorthQuerySemanticDependencyRole>,
    delivery_identity: String,
    consumer_binding_identity: String,
    consumer_operation_identity: String,
    effect: Arc<super::WorthQueryPerformedMaintenanceEffect>,
}

impl WorthQueryPublishedLiveDelivery {
    pub fn strategy(&self) -> WorthQueryMaintenanceStrategy {
        self.strategies[0]
    }

    pub fn strategies(&self) -> &[WorthQueryMaintenanceStrategy] {
        &self.strategies
    }

    pub const fn scope(&self) -> &WorthQueryMaintenanceScope {
        &self.scope
    }

    pub const fn result_state(&self) -> WorthQueryOperationResultState {
        self.result_state
    }

    pub fn roles(&self) -> &[crate::domain_installation::WorthQuerySemanticDependencyRole] {
        &self.roles
    }

    pub fn delivery_identity(&self) -> &str {
        &self.delivery_identity
    }

    pub fn consumer_binding_identity(&self) -> &str {
        &self.consumer_binding_identity
    }

    pub fn consumer_operation_identity(&self) -> &str {
        &self.consumer_operation_identity
    }

    pub fn effect(&self) -> &super::WorthQueryPerformedMaintenanceEffect {
        &self.effect
    }
}

pub(crate) fn publish_invalidation_maintenance<D, O, F, L: BasisOperationLane>(
    maintenance: WorthQueryPerformedMaintenance,
    current: &WorthQuerySettledDomainProjection<D, O, F, L>,
    performed: &crate::domain_installation::WorthQueryLiveProjectionRefresh,
) -> Result<WorthQueryPublishedLiveDelivery, WorthQueryLivePublicationDenial> {
    let current_closure = current.semantic_aspect_dependency_closure();
    if operation_phase_basis(&maintenance.phase).operation_identity
        != current_closure.invalidation_manifest().operation_identity()
        || operation_phase_basis(&maintenance.phase).installation_generation
            != current_closure
                .invalidation_manifest()
                .installation_generation()
    {
        return Err(WorthQueryLivePublicationDenial::ForeignOrStaleMaintenance);
    }
    if performed.authority().receipt().receipt_digest() != maintenance.execution_identity
        || current.publication_receipt().identity() != maintenance.publication_identity
    {
        return Err(WorthQueryLivePublicationDenial::ExecutionReceiptMismatch);
    }
    let strategy_identity = maintenance
        .strategies
        .iter()
        .map(|strategy| (*strategy as u8).to_string())
        .collect::<Vec<_>>()
        .join(".");
    let role_identity = maintenance
        .roles
        .iter()
        .map(|role| role.canonical_name())
        .collect::<Vec<_>>()
        .join(".");
    let scope_identity = match &maintenance.scope {
        WorthQueryMaintenanceScope::ExactSourceRecord {
            partition_id,
            local_slot,
            generation,
        } => format!("record:{partition_id}:{local_slot}:{generation}"),
        WorthQueryMaintenanceScope::SourcePartition(partition) => {
            format!("partition:{partition}")
        }
        WorthQueryMaintenanceScope::WholeLogicalGraph => "whole-graph".to_owned(),
    };
    let delivery_identity = format!(
        "invalidation-delivery:{}:{strategy_identity}:{role_identity}:{scope_identity}:{}",
        maintenance.publication_identity,
        maintenance.effect.identity(),
    );
    let phase = mint_operation_phase_proof(
        delivery_identity.clone(),
        Some(maintenance.phase.payload().identity()),
        operation_phase_basis(&maintenance.phase).clone(),
    );
    Ok(WorthQueryPublishedLiveDelivery {
        _phase: phase,
        strategies: maintenance.strategies,
        scope: maintenance.scope,
        result_state: maintenance.result_state,
        roles: maintenance.roles,
        delivery_identity,
        consumer_binding_identity: current.consumer_contract().binding_identity().to_owned(),
        consumer_operation_identity: current
            .consumer_contract()
            .canonical_operation_identity()
            .to_owned(),
        effect: maintenance.effect,
    })
}
