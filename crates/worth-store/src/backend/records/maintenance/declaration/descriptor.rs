use crate::{
    FreshnessWindow, LocalityScopeToken, MaintenanceDebtFamily, MaintenanceDeclarationId,
    MaintenanceDescriptorDemand, MaintenanceEquivalenceKey, MaintenanceEscalationDecision,
    MaintenanceExecutionPosture, MaintenanceLocalityScope, MaintenanceReservationFamily,
    MaintenanceWorkClass, MaintenanceWorkDescriptor, MaintenanceWorkDescriptorBasis,
    MaintenanceWorkIdentity, PlanGeneration, SupersessionEpoch, TierWorkContainerClass,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct PersistedMaintenanceWorkDescriptor {
    declaration_id: String,
    work_class: MaintenanceWorkClass,
    execution_posture: MaintenanceExecutionPosture,
    locality_scope: MaintenanceLocalityScope,
    locality_scope_token: LocalityScopeToken,
    demand: MaintenanceDescriptorDemand,
    reservation_family: MaintenanceReservationFamily,
    work_identity: String,
    equivalence_key: String,
    plan_generation: PlanGeneration,
    supersession_epoch: SupersessionEpoch,
    freshness_window: FreshnessWindow,
    debt_family: Option<MaintenanceDebtFamily>,
    escalation_decision: MaintenanceEscalationDecision,
    tier_work_container_class: Option<TierWorkContainerClass>,
    recovered_from_restart: bool,
}

impl From<&MaintenanceWorkDescriptor> for PersistedMaintenanceWorkDescriptor {
    fn from(descriptor: &MaintenanceWorkDescriptor) -> Self {
        Self {
            declaration_id: descriptor.declaration_id().as_str().to_string(),
            work_class: descriptor.work_class(),
            execution_posture: descriptor.execution_posture(),
            locality_scope: descriptor.locality_scope().clone(),
            locality_scope_token: descriptor.locality_scope_token().clone(),
            demand: descriptor.demand().clone(),
            reservation_family: descriptor.reservation_family(),
            work_identity: descriptor.work_identity().as_str().to_string(),
            equivalence_key: descriptor.equivalence_key().as_str().to_string(),
            plan_generation: descriptor.plan_generation(),
            supersession_epoch: descriptor.supersession_epoch(),
            freshness_window: descriptor.freshness_window(),
            debt_family: descriptor.debt_family(),
            escalation_decision: descriptor.escalation_decision(),
            tier_work_container_class: descriptor.tier_work_container_class(),
            recovered_from_restart: descriptor.recovered_from_restart(),
        }
    }
}

impl TryFrom<PersistedMaintenanceWorkDescriptor> for MaintenanceWorkDescriptor {
    type Error = String;

    fn try_from(descriptor: PersistedMaintenanceWorkDescriptor) -> Result<Self, Self::Error> {
        Ok(MaintenanceWorkDescriptor::new(
            MaintenanceWorkDescriptorBasis {
                declaration_id: MaintenanceDeclarationId::new(descriptor.declaration_id),
                work_class: descriptor.work_class,
                execution_posture: descriptor.execution_posture,
                locality_scope: descriptor.locality_scope,
                locality_scope_token: descriptor.locality_scope_token,
                demand: descriptor.demand,
                reservation_family: descriptor.reservation_family,
                work_identity: MaintenanceWorkIdentity::new(descriptor.work_identity),
                equivalence_key: MaintenanceEquivalenceKey::new(descriptor.equivalence_key),
                plan_generation: descriptor.plan_generation,
                supersession_epoch: descriptor.supersession_epoch,
                freshness_window: descriptor.freshness_window,
                debt_family: descriptor.debt_family,
                escalation_decision: descriptor.escalation_decision,
                tier_work_container_class: descriptor.tier_work_container_class,
                recovered_from_restart: descriptor.recovered_from_restart,
            },
        ))
    }
}
