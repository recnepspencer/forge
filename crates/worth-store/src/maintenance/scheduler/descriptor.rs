use serde::Serialize;

use super::super::MaintenanceDeclarationId;

use super::budgets::{
    FreshnessWindow, MaintenanceDescriptorDemand, PlanGeneration, SupersessionEpoch,
};

use super::classes::{
    MaintenanceDebtFamily, MaintenanceEscalationDecision, MaintenanceExecutionPosture,
    MaintenanceReservationFamily, MaintenanceWorkClass, TierWorkContainerClass,
};

use super::identities::{
    locality_scope_token_string, LocalityScopeToken, MaintenanceEquivalenceKey, MaintenanceLaneKey,
    MaintenanceLocalityScope, MaintenanceWorkIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceWorkDescriptor {
    declaration_id: MaintenanceDeclarationId,
    work_class: MaintenanceWorkClass,
    execution_posture: MaintenanceExecutionPosture,
    locality_scope: MaintenanceLocalityScope,
    locality_scope_token: LocalityScopeToken,
    demand: MaintenanceDescriptorDemand,
    reservation_family: MaintenanceReservationFamily,
    work_identity: MaintenanceWorkIdentity,
    equivalence_key: MaintenanceEquivalenceKey,
    plan_generation: PlanGeneration,
    supersession_epoch: SupersessionEpoch,
    freshness_window: FreshnessWindow,
    debt_family: Option<MaintenanceDebtFamily>,
    escalation_decision: MaintenanceEscalationDecision,
    tier_work_container_class: Option<TierWorkContainerClass>,
    recovered_from_restart: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct MaintenanceWorkDescriptorBasis {
    pub(crate) declaration_id: MaintenanceDeclarationId,
    pub(crate) work_class: MaintenanceWorkClass,
    pub(crate) execution_posture: MaintenanceExecutionPosture,
    pub(crate) locality_scope: MaintenanceLocalityScope,
    pub(crate) locality_scope_token: LocalityScopeToken,
    pub(crate) demand: MaintenanceDescriptorDemand,
    pub(crate) reservation_family: MaintenanceReservationFamily,
    pub(crate) work_identity: MaintenanceWorkIdentity,
    pub(crate) equivalence_key: MaintenanceEquivalenceKey,
    pub(crate) plan_generation: PlanGeneration,
    pub(crate) supersession_epoch: SupersessionEpoch,
    pub(crate) freshness_window: FreshnessWindow,
    pub(crate) debt_family: Option<MaintenanceDebtFamily>,
    pub(crate) escalation_decision: MaintenanceEscalationDecision,
    pub(crate) tier_work_container_class: Option<TierWorkContainerClass>,
    pub(crate) recovered_from_restart: bool,
}

impl MaintenanceWorkDescriptor {
    pub(crate) fn new(basis: MaintenanceWorkDescriptorBasis) -> Self {
        Self {
            declaration_id: basis.declaration_id,
            work_class: basis.work_class,
            execution_posture: basis.execution_posture,
            locality_scope: basis.locality_scope,
            locality_scope_token: basis.locality_scope_token,
            demand: basis.demand,
            reservation_family: basis.reservation_family,
            work_identity: basis.work_identity,
            equivalence_key: basis.equivalence_key,
            plan_generation: basis.plan_generation,
            supersession_epoch: basis.supersession_epoch,
            freshness_window: basis.freshness_window,
            debt_family: basis.debt_family,
            escalation_decision: basis.escalation_decision,
            tier_work_container_class: basis.tier_work_container_class,
            recovered_from_restart: basis.recovered_from_restart,
        }
    }

    pub fn declaration_id(&self) -> &MaintenanceDeclarationId {
        &self.declaration_id
    }

    pub fn work_class(&self) -> MaintenanceWorkClass {
        self.work_class
    }

    pub fn execution_posture(&self) -> MaintenanceExecutionPosture {
        self.execution_posture
    }

    pub fn locality_scope(&self) -> &MaintenanceLocalityScope {
        &self.locality_scope
    }

    pub fn locality_scope_token(&self) -> &LocalityScopeToken {
        &self.locality_scope_token
    }

    pub fn demand(&self) -> &MaintenanceDescriptorDemand {
        &self.demand
    }

    pub fn reservation_family(&self) -> MaintenanceReservationFamily {
        self.reservation_family
    }

    pub fn work_identity(&self) -> &MaintenanceWorkIdentity {
        &self.work_identity
    }

    pub fn equivalence_key(&self) -> &MaintenanceEquivalenceKey {
        &self.equivalence_key
    }

    pub fn plan_generation(&self) -> PlanGeneration {
        self.plan_generation
    }

    pub fn supersession_epoch(&self) -> SupersessionEpoch {
        self.supersession_epoch
    }

    pub fn freshness_window(&self) -> FreshnessWindow {
        self.freshness_window
    }

    pub fn debt_family(&self) -> Option<MaintenanceDebtFamily> {
        self.debt_family
    }

    pub fn escalation_decision(&self) -> MaintenanceEscalationDecision {
        self.escalation_decision
    }

    pub fn tier_work_container_class(&self) -> Option<TierWorkContainerClass> {
        self.tier_work_container_class
    }

    pub fn recovered_from_restart(&self) -> bool {
        self.recovered_from_restart
    }

    pub fn lane_key(&self) -> MaintenanceLaneKey {
        MaintenanceLaneKey::new(
            self.work_class,
            self.locality_scope.clone(),
            self.reservation_family,
        )
    }

    pub(crate) fn with_escalation_decision(
        mut self,
        escalation_decision: MaintenanceEscalationDecision,
    ) -> Self {
        self.escalation_decision = escalation_decision;
        self
    }

    pub(crate) fn with_freshness_window(mut self, freshness_window: FreshnessWindow) -> Self {
        self.freshness_window = freshness_window;
        self
    }

    pub(crate) fn with_recovered_from_restart(mut self, recovered_from_restart: bool) -> Self {
        self.recovered_from_restart = recovered_from_restart;
        self
    }

    pub(crate) fn with_demand(mut self, demand: MaintenanceDescriptorDemand) -> Self {
        self.demand = demand;
        self
    }

    pub(crate) fn with_supersession_epoch(mut self, supersession_epoch: SupersessionEpoch) -> Self {
        self.supersession_epoch = supersession_epoch;
        self
    }

    pub(crate) fn with_plan_generation(mut self, plan_generation: PlanGeneration) -> Self {
        self.plan_generation = plan_generation;
        self
    }

    pub(crate) fn with_locality_scope(mut self, locality_scope: MaintenanceLocalityScope) -> Self {
        self.locality_scope_token =
            LocalityScopeToken::new(locality_scope_token_string(&locality_scope));
        self.locality_scope = locality_scope;
        self
    }
}
