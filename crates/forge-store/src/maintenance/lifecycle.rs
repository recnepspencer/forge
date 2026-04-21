use serde::{Deserialize, Serialize};

use super::{
    FreshnessWindow, MaintenanceCoalescingDecision, MaintenanceDebtFamily, MaintenanceDeclaration,
    MaintenanceDeclarationClass, MaintenanceDeclarationId, MaintenanceEscalationDecision,
    MaintenanceEscalationVerdict, MaintenanceExecutionPosture, MaintenanceFailureKind,
    MaintenanceLaneKey, MaintenanceLocalityScope, MaintenancePlanFamily,
    MaintenanceReservationFamily, MaintenanceResourceBudgetGrant, MaintenanceStarvationStatus,
    MaintenanceWorkClass, MaintenanceWorkDescriptor, PlanGeneration, SupersessionEpoch,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceExecutionStatus {
    Declared,
    Admitted,
    Reserved,
    Deferred,
    Started,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceReadmissionStatus {
    PendingRecoveredReadmission,
    ReadmittedRecoveredWork,
    RejectedStaleRecoveredWork,
    RejectedSupersededRecoveredWork,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceForegroundImpact {
    borrowed_foreground_reservation: bool,
    foreground_wait_required: bool,
    cutover_dependency_required: bool,
}

impl MaintenanceForegroundImpact {
    pub const fn none() -> Self {
        Self {
            borrowed_foreground_reservation: false,
            foreground_wait_required: false,
            cutover_dependency_required: false,
        }
    }

    pub(crate) fn escalated() -> Self {
        Self {
            borrowed_foreground_reservation: true,
            foreground_wait_required: true,
            cutover_dependency_required: true,
        }
    }

    pub fn borrowed_foreground_reservation(&self) -> bool {
        self.borrowed_foreground_reservation
    }

    pub fn foreground_wait_required(&self) -> bool {
        self.foreground_wait_required
    }

    pub fn cutover_dependency_required(&self) -> bool {
        self.cutover_dependency_required
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundReservationClass {
    Read,
    Write,
    Continuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundInterferencePosture {
    StayedIsolated,
    ObservedMaintenanceNoWait,
    WaitedOnMaintenance,
    BroadenedByMaintenance,
    ReservationViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundWaitDependency {
    MaintenanceReservationRelease,
    MaintenancePublication,
    MaintenanceCutover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundBroadeningCause {
    MaintenanceBlockedIsolatedPath,
    GlobalDebtPromotion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundIsolationViolation {
    SharedReservationConflict,
    IllegalForegroundBorrow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForegroundIsolationOutcome {
    reservation_class: ForegroundReservationClass,
    posture: ForegroundInterferencePosture,
    wait_dependency: Option<ForegroundWaitDependency>,
    broadening_cause: Option<ForegroundBroadeningCause>,
    violation: Option<ForegroundIsolationViolation>,
}

impl ForegroundIsolationOutcome {
    pub fn stayed_isolated(reservation_class: ForegroundReservationClass) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::StayedIsolated,
            wait_dependency: None,
            broadening_cause: None,
            violation: None,
        }
    }

    pub fn observed_maintenance(reservation_class: ForegroundReservationClass) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::ObservedMaintenanceNoWait,
            wait_dependency: None,
            broadening_cause: None,
            violation: None,
        }
    }

    pub fn waited(
        reservation_class: ForegroundReservationClass,
        wait_dependency: ForegroundWaitDependency,
    ) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::WaitedOnMaintenance,
            wait_dependency: Some(wait_dependency),
            broadening_cause: None,
            violation: None,
        }
    }

    pub fn broadened(
        reservation_class: ForegroundReservationClass,
        broadening_cause: ForegroundBroadeningCause,
    ) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::BroadenedByMaintenance,
            wait_dependency: None,
            broadening_cause: Some(broadening_cause),
            violation: None,
        }
    }

    pub fn violated(
        reservation_class: ForegroundReservationClass,
        violation: ForegroundIsolationViolation,
    ) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::ReservationViolation,
            wait_dependency: None,
            broadening_cause: None,
            violation: Some(violation),
        }
    }

    pub fn reservation_class(&self) -> ForegroundReservationClass {
        self.reservation_class
    }

    pub fn posture(&self) -> ForegroundInterferencePosture {
        self.posture
    }

    pub fn wait_dependency(&self) -> Option<ForegroundWaitDependency> {
        self.wait_dependency
    }

    pub fn broadening_cause(&self) -> Option<ForegroundBroadeningCause> {
        self.broadening_cause
    }

    pub fn violation(&self) -> Option<ForegroundIsolationViolation> {
        self.violation
    }

    pub fn maintenance_interference(&self) -> bool {
        !matches!(self.posture, ForegroundInterferencePosture::StayedIsolated)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveredMaintenanceLaneIntake {
    lane_key: MaintenanceLaneKey,
    pending_recovered_count: u64,
    readmitted_recovered_count: u64,
    rejected_recovered_count: u64,
    stale_recovered_count: u64,
    coalesced_recovered_count: u64,
    debt_bearing: bool,
}

impl RecoveredMaintenanceLaneIntake {
    pub(crate) fn new(
        lane_key: MaintenanceLaneKey,
        pending_recovered_count: u64,
        readmitted_recovered_count: u64,
        rejected_recovered_count: u64,
        stale_recovered_count: u64,
        coalesced_recovered_count: u64,
        debt_bearing: bool,
    ) -> Self {
        Self {
            lane_key,
            pending_recovered_count,
            readmitted_recovered_count,
            rejected_recovered_count,
            stale_recovered_count,
            coalesced_recovered_count,
            debt_bearing,
        }
    }

    pub fn lane_key(&self) -> &MaintenanceLaneKey {
        &self.lane_key
    }

    pub fn pending_recovered_count(&self) -> u64 {
        self.pending_recovered_count
    }

    pub fn readmitted_recovered_count(&self) -> u64 {
        self.readmitted_recovered_count
    }

    pub fn rejected_recovered_count(&self) -> u64 {
        self.rejected_recovered_count
    }

    pub fn stale_recovered_count(&self) -> u64 {
        self.stale_recovered_count
    }

    pub fn coalesced_recovered_count(&self) -> u64 {
        self.coalesced_recovered_count
    }

    pub fn debt_bearing(&self) -> bool {
        self.debt_bearing
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveredMaintenanceIntakeReport {
    pending_recovered_count: u64,
    readmitted_recovered_count: u64,
    rejected_recovered_count: u64,
    stale_recovered_count: u64,
    coalesced_recovered_count: u64,
    lane_intake: Vec<RecoveredMaintenanceLaneIntake>,
}

impl RecoveredMaintenanceIntakeReport {
    pub(crate) fn new(
        pending_recovered_count: u64,
        readmitted_recovered_count: u64,
        rejected_recovered_count: u64,
        stale_recovered_count: u64,
        coalesced_recovered_count: u64,
        lane_intake: Vec<RecoveredMaintenanceLaneIntake>,
    ) -> Self {
        Self {
            pending_recovered_count,
            readmitted_recovered_count,
            rejected_recovered_count,
            stale_recovered_count,
            coalesced_recovered_count,
            lane_intake,
        }
    }

    pub fn pending_recovered_count(&self) -> u64 {
        self.pending_recovered_count
    }

    pub fn readmitted_recovered_count(&self) -> u64 {
        self.readmitted_recovered_count
    }

    pub fn rejected_recovered_count(&self) -> u64 {
        self.rejected_recovered_count
    }

    pub fn stale_recovered_count(&self) -> u64 {
        self.stale_recovered_count
    }

    pub fn coalesced_recovered_count(&self) -> u64 {
        self.coalesced_recovered_count
    }

    pub fn lane_intake(&self) -> &[RecoveredMaintenanceLaneIntake] {
        &self.lane_intake
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceColdStartBootReport {
    loaded_persisted_summaries: bool,
    used_legacy_summary_backfill: bool,
    recovered_backlog_count: u64,
    integrity_reject_count: u64,
}

impl MaintenanceColdStartBootReport {
    pub(crate) fn new(
        loaded_persisted_summaries: bool,
        used_legacy_summary_backfill: bool,
        recovered_backlog_count: u64,
        integrity_reject_count: u64,
    ) -> Self {
        Self {
            loaded_persisted_summaries,
            used_legacy_summary_backfill,
            recovered_backlog_count,
            integrity_reject_count,
        }
    }

    pub fn loaded_persisted_summaries(&self) -> bool {
        self.loaded_persisted_summaries
    }

    pub fn used_legacy_summary_backfill(&self) -> bool {
        self.used_legacy_summary_backfill
    }

    pub fn recovered_backlog_count(&self) -> u64 {
        self.recovered_backlog_count
    }

    pub fn integrity_reject_count(&self) -> u64 {
        self.integrity_reject_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceReservationTransition {
    plan_family: MaintenancePlanFamily,
    quantum_units: u64,
}

impl MaintenanceReservationTransition {
    pub(crate) fn new(plan_family: MaintenancePlanFamily, quantum_units: u64) -> Self {
        Self {
            plan_family,
            quantum_units,
        }
    }

    pub fn plan_family(&self) -> MaintenancePlanFamily {
        self.plan_family
    }

    pub fn quantum_units(&self) -> u64 {
        self.quantum_units
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceExecutionTransition {
    resumed_from_started: bool,
    quantum_units: Option<u64>,
}

impl MaintenanceExecutionTransition {
    pub(crate) fn new(resumed_from_started: bool, quantum_units: Option<u64>) -> Self {
        Self {
            resumed_from_started,
            quantum_units,
        }
    }

    pub fn resumed_from_started(&self) -> bool {
        self.resumed_from_started
    }

    pub fn quantum_units(&self) -> Option<u64> {
        self.quantum_units
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompletedMaintenance {
    declaration: MaintenanceDeclaration,
    descriptor: MaintenanceWorkDescriptor,
    last_completed_phase: String,
}

impl CompletedMaintenance {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        descriptor: MaintenanceWorkDescriptor,
        last_completed_phase: impl Into<String>,
    ) -> Self {
        Self {
            declaration,
            descriptor,
            last_completed_phase: last_completed_phase.into(),
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }

    pub fn last_completed_phase(&self) -> &str {
        &self.last_completed_phase
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FailedMaintenance {
    declaration: MaintenanceDeclaration,
    descriptor: Option<MaintenanceWorkDescriptor>,
    failure_kind: MaintenanceFailureKind,
    error_kind: String,
    message: String,
}

impl FailedMaintenance {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        descriptor: Option<MaintenanceWorkDescriptor>,
        failure_kind: MaintenanceFailureKind,
        error_kind: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            declaration,
            descriptor,
            failure_kind,
            error_kind: error_kind.into(),
            message: message.into(),
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn descriptor(&self) -> Option<&MaintenanceWorkDescriptor> {
        self.descriptor.as_ref()
    }

    pub fn failure_kind(&self) -> MaintenanceFailureKind {
        self.failure_kind
    }

    pub fn error_kind(&self) -> &str {
        &self.error_kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceStatusReport {
    declaration_id: MaintenanceDeclarationId,
    declaration_class: MaintenanceDeclarationClass,
    work_class: MaintenanceWorkClass,
    execution_posture: MaintenanceExecutionPosture,
    locality_scope: MaintenanceLocalityScope,
    lane_key: MaintenanceLaneKey,
    reservation_family: MaintenanceReservationFamily,
    plan_generation: PlanGeneration,
    supersession_epoch: SupersessionEpoch,
    freshness_window: FreshnessWindow,
    debt_family: Option<MaintenanceDebtFamily>,
    escalation_decision: MaintenanceEscalationDecision,
    tier_work_container_class: Option<crate::TierWorkContainerClass>,
    recovered_from_restart: bool,
    restart_readmission_status: Option<MaintenanceReadmissionStatus>,
    reservation_transition: Option<MaintenanceReservationTransition>,
    execution_transition: Option<MaintenanceExecutionTransition>,
    foreground_impact: MaintenanceForegroundImpact,
    coalescing_decision: Option<MaintenanceCoalescingDecision>,
    supersession_source: Option<String>,
    resource_budget_grant: Option<MaintenanceResourceBudgetGrant>,
    starvation_status: Option<MaintenanceStarvationStatus>,
    escalation_verdict: Option<MaintenanceEscalationVerdict>,
    explicit_global_scope_debt: bool,
    plan_family: Option<MaintenancePlanFamily>,
    pending_reason: Option<String>,
    execution_status: MaintenanceExecutionStatus,
    last_completed_phase: Option<String>,
    durable_error_kind: Option<String>,
    debt_link_artifact_id: Option<String>,
}

impl MaintenanceStatusReport {
    pub(crate) fn new(
        declaration_id: MaintenanceDeclarationId,
        declaration_class: MaintenanceDeclarationClass,
        work_class: MaintenanceWorkClass,
        execution_posture: MaintenanceExecutionPosture,
        locality_scope: MaintenanceLocalityScope,
        lane_key: MaintenanceLaneKey,
        reservation_family: MaintenanceReservationFamily,
        plan_generation: PlanGeneration,
        supersession_epoch: SupersessionEpoch,
        freshness_window: FreshnessWindow,
        debt_family: Option<MaintenanceDebtFamily>,
        escalation_decision: MaintenanceEscalationDecision,
        tier_work_container_class: Option<crate::TierWorkContainerClass>,
        recovered_from_restart: bool,
        restart_readmission_status: Option<MaintenanceReadmissionStatus>,
        reservation_transition: Option<MaintenanceReservationTransition>,
        execution_transition: Option<MaintenanceExecutionTransition>,
        foreground_impact: MaintenanceForegroundImpact,
        coalescing_decision: Option<MaintenanceCoalescingDecision>,
        supersession_source: Option<String>,
        resource_budget_grant: Option<MaintenanceResourceBudgetGrant>,
        starvation_status: Option<MaintenanceStarvationStatus>,
        escalation_verdict: Option<MaintenanceEscalationVerdict>,
        explicit_global_scope_debt: bool,
        plan_family: Option<MaintenancePlanFamily>,
        pending_reason: Option<String>,
        execution_status: MaintenanceExecutionStatus,
        last_completed_phase: Option<String>,
        durable_error_kind: Option<String>,
        debt_link_artifact_id: Option<String>,
    ) -> Self {
        Self {
            declaration_id,
            declaration_class,
            work_class,
            execution_posture,
            locality_scope,
            lane_key,
            reservation_family,
            plan_generation,
            supersession_epoch,
            freshness_window,
            debt_family,
            escalation_decision,
            tier_work_container_class,
            recovered_from_restart,
            restart_readmission_status,
            reservation_transition,
            execution_transition,
            foreground_impact,
            coalescing_decision,
            supersession_source,
            resource_budget_grant,
            starvation_status,
            escalation_verdict,
            explicit_global_scope_debt,
            plan_family,
            pending_reason,
            execution_status,
            last_completed_phase,
            durable_error_kind,
            debt_link_artifact_id,
        }
    }

    pub fn declaration_id(&self) -> &MaintenanceDeclarationId {
        &self.declaration_id
    }

    pub fn declaration_class(&self) -> MaintenanceDeclarationClass {
        self.declaration_class
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

    pub fn lane_key(&self) -> &MaintenanceLaneKey {
        &self.lane_key
    }

    pub fn reservation_family(&self) -> MaintenanceReservationFamily {
        self.reservation_family
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

    pub fn tier_work_container_class(&self) -> Option<crate::TierWorkContainerClass> {
        self.tier_work_container_class
    }

    pub fn recovered_from_restart(&self) -> bool {
        self.recovered_from_restart
    }

    pub fn restart_readmission_status(&self) -> Option<MaintenanceReadmissionStatus> {
        self.restart_readmission_status
    }

    pub fn reservation_transition(&self) -> Option<&MaintenanceReservationTransition> {
        self.reservation_transition.as_ref()
    }

    pub fn execution_transition(&self) -> Option<&MaintenanceExecutionTransition> {
        self.execution_transition.as_ref()
    }

    pub fn foreground_impact(&self) -> &MaintenanceForegroundImpact {
        &self.foreground_impact
    }

    pub fn coalescing_decision(&self) -> Option<MaintenanceCoalescingDecision> {
        self.coalescing_decision
    }

    pub fn supersession_source(&self) -> Option<&str> {
        self.supersession_source.as_deref()
    }

    pub fn resource_budget_grant(&self) -> Option<&MaintenanceResourceBudgetGrant> {
        self.resource_budget_grant.as_ref()
    }

    pub fn starvation_status(&self) -> Option<MaintenanceStarvationStatus> {
        self.starvation_status
    }

    pub fn escalation_verdict(&self) -> Option<MaintenanceEscalationVerdict> {
        self.escalation_verdict
    }

    pub fn explicit_global_scope_debt(&self) -> bool {
        self.explicit_global_scope_debt
    }

    pub fn plan_family(&self) -> Option<MaintenancePlanFamily> {
        self.plan_family
    }

    pub fn pending_reason(&self) -> Option<&str> {
        self.pending_reason.as_deref()
    }

    pub fn execution_status(&self) -> MaintenanceExecutionStatus {
        self.execution_status
    }

    pub fn last_completed_phase(&self) -> Option<&str> {
        self.last_completed_phase.as_deref()
    }

    pub fn durable_error_kind(&self) -> Option<&str> {
        self.durable_error_kind.as_deref()
    }

    pub fn debt_link_artifact_id(&self) -> Option<&str> {
        self.debt_link_artifact_id.as_deref()
    }
}
