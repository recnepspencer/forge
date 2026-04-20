#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::{MaintenanceDeclaration, MaintenanceDeclarationId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MaintenanceWorkClass {
    RetentionAudit,
    CompactionMaintenance,
    DerivedArtifactReclaim,
    AuthoritativeReclaim,
    RetainedRangeRebuild,
    SnapshotRefresh,
    DerivedFamilyRebuild,
    ReplicationPreparation,
    MaintenanceAudit,
    TierPlacementProposal,
    TierMoveExecution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceExecutionPosture {
    ForegroundBlocking,
    ForegroundAware,
    FullyDeferrable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MaintenanceDebtFamily {
    CompactionDebt,
    RebuildDebt,
    SnapshotDebt,
    ReplicationPreparationDebt,
    TierPlacementDebt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ForegroundReservationFamily {
    Write,
    Read,
    Continuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BackgroundReservationFamily {
    Maintenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MaintenanceReservationFamily {
    Foreground(ForegroundReservationFamily),
    Background(BackgroundReservationFamily),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceEscalationDecision {
    StayBackground,
    PaceUpWithinBackgroundBudget,
    EscalateWithForegroundImpact,
    DeferWithOperatorSignal,
    RejectNewDerivedWork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TierWorkContainerClass {
    TierPlacementProposal,
    TierMoveExecution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceFailureKind {
    ReservationViolation,
    FreshnessFailure,
    EquivalenceConflict,
    RestartAdmissionFailure,
    Deferred,
    Cancelled,
    ExecutionFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenancePlanFamily {
    ForegroundReserved,
    BackgroundPaced,
    Escalated,
    Deferred,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MaintenanceLocalityScope {
    BranchLocalityScope { branch_label: String },
    ArtifactFamilyLocalityScope { family_label: String },
    TenantLocalityScope { tenant_label: String },
    StoreGlobalLocalityScope,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LocalityScopeToken(String);

impl LocalityScopeToken {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaintenanceWorkIdentity(String);

impl MaintenanceWorkIdentity {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaintenanceEquivalenceKey(String);

impl MaintenanceEquivalenceKey {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IoBudgetUnits(u64);

impl IoBudgetUnits {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CpuBudgetUnits(u64);

impl CpuBudgetUnits {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MemoryBudgetUnits(u64);

impl MemoryBudgetUnits {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PublicationSlotBudget(u64);

impl PublicationSlotBudget {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ForegroundLatencyGuard(u64);

impl ForegroundLatencyGuard {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceDescriptorDemand {
    predicted_io: IoBudgetUnits,
    predicted_cpu: CpuBudgetUnits,
    predicted_memory: MemoryBudgetUnits,
    predicted_publication: PublicationSlotBudget,
    foreground_latency_guard: ForegroundLatencyGuard,
}

impl MaintenanceDescriptorDemand {
    pub(crate) fn new(
        predicted_io: IoBudgetUnits,
        predicted_cpu: CpuBudgetUnits,
        predicted_memory: MemoryBudgetUnits,
        predicted_publication: PublicationSlotBudget,
        foreground_latency_guard: ForegroundLatencyGuard,
    ) -> Self {
        Self {
            predicted_io,
            predicted_cpu,
            predicted_memory,
            predicted_publication,
            foreground_latency_guard,
        }
    }

    pub fn predicted_io(&self) -> IoBudgetUnits {
        self.predicted_io
    }

    pub fn predicted_cpu(&self) -> CpuBudgetUnits {
        self.predicted_cpu
    }

    pub fn predicted_memory(&self) -> MemoryBudgetUnits {
        self.predicted_memory
    }

    pub fn predicted_publication(&self) -> PublicationSlotBudget {
        self.predicted_publication
    }

    pub fn foreground_latency_guard(&self) -> ForegroundLatencyGuard {
        self.foreground_latency_guard
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaintenanceQuantum(u64);

impl MaintenanceQuantum {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PacingWindow(u64);

impl PacingWindow {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlanGeneration(u64);

impl PlanGeneration {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SupersessionEpoch(u64);

impl SupersessionEpoch {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FreshnessWindow(u64);

impl FreshnessWindow {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

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

impl MaintenanceWorkDescriptor {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
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
    ) -> Self {
        Self {
            declaration_id,
            work_class,
            execution_posture,
            locality_scope,
            locality_scope_token,
            demand,
            reservation_family,
            work_identity,
            equivalence_key,
            plan_generation,
            supersession_epoch,
            freshness_window,
            debt_family,
            escalation_decision,
            tier_work_container_class,
            recovered_from_restart,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiscoveredMaintenanceWork {
    declaration: MaintenanceDeclaration,
    descriptor: MaintenanceWorkDescriptor,
}

impl DiscoveredMaintenanceWork {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        descriptor: MaintenanceWorkDescriptor,
    ) -> Self {
        Self {
            declaration,
            descriptor,
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdmittedMaintenanceWork {
    declaration: MaintenanceDeclaration,
    descriptor: MaintenanceWorkDescriptor,
}

impl AdmittedMaintenanceWork {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        descriptor: MaintenanceWorkDescriptor,
    ) -> Self {
        Self {
            declaration,
            descriptor,
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuantumBudgetReceipt {
    maintenance_quantum: MaintenanceQuantum,
    pacing_window: PacingWindow,
}

impl QuantumBudgetReceipt {
    pub(crate) fn new(maintenance_quantum: MaintenanceQuantum, pacing_window: PacingWindow) -> Self {
        Self {
            maintenance_quantum,
            pacing_window,
        }
    }

    pub fn maintenance_quantum(&self) -> MaintenanceQuantum {
        self.maintenance_quantum
    }

    pub fn pacing_window(&self) -> PacingWindow {
        self.pacing_window
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForegroundReservationWitness {
    family: ForegroundReservationFamily,
    locality_scope: MaintenanceLocalityScope,
}

impl ForegroundReservationWitness {
    pub(crate) fn new(
        family: ForegroundReservationFamily,
        locality_scope: MaintenanceLocalityScope,
    ) -> Self {
        Self {
            family,
            locality_scope,
        }
    }

    pub fn family(&self) -> ForegroundReservationFamily {
        self.family
    }

    pub fn locality_scope(&self) -> &MaintenanceLocalityScope {
        &self.locality_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackgroundPacedMaintenancePlan {
    descriptor: MaintenanceWorkDescriptor,
    quantum_budget_receipt: QuantumBudgetReceipt,
}

impl BackgroundPacedMaintenancePlan {
    pub(crate) fn new(
        descriptor: MaintenanceWorkDescriptor,
        quantum_budget_receipt: QuantumBudgetReceipt,
    ) -> Self {
        Self {
            descriptor,
            quantum_budget_receipt,
        }
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }

    pub fn quantum_budget_receipt(&self) -> &QuantumBudgetReceipt {
        &self.quantum_budget_receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForegroundReservedMaintenancePlan {
    descriptor: MaintenanceWorkDescriptor,
    reservation_witness: ForegroundReservationWitness,
    quantum_budget_receipt: QuantumBudgetReceipt,
}

impl ForegroundReservedMaintenancePlan {
    pub(crate) fn new(
        descriptor: MaintenanceWorkDescriptor,
        reservation_witness: ForegroundReservationWitness,
        quantum_budget_receipt: QuantumBudgetReceipt,
    ) -> Self {
        Self {
            descriptor,
            reservation_witness,
            quantum_budget_receipt,
        }
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }

    pub fn reservation_witness(&self) -> &ForegroundReservationWitness {
        &self.reservation_witness
    }

    pub fn quantum_budget_receipt(&self) -> &QuantumBudgetReceipt {
        &self.quantum_budget_receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EscalatedMaintenancePlan {
    descriptor: MaintenanceWorkDescriptor,
    escalation_decision: MaintenanceEscalationDecision,
    foreground_reservation_witness: Option<ForegroundReservationWitness>,
    quantum_budget_receipt: QuantumBudgetReceipt,
}

impl EscalatedMaintenancePlan {
    pub(crate) fn new(
        descriptor: MaintenanceWorkDescriptor,
        escalation_decision: MaintenanceEscalationDecision,
        foreground_reservation_witness: Option<ForegroundReservationWitness>,
        quantum_budget_receipt: QuantumBudgetReceipt,
    ) -> Self {
        Self {
            descriptor,
            escalation_decision,
            foreground_reservation_witness,
            quantum_budget_receipt,
        }
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }

    pub fn escalation_decision(&self) -> MaintenanceEscalationDecision {
        self.escalation_decision
    }

    pub fn foreground_reservation_witness(&self) -> Option<&ForegroundReservationWitness> {
        self.foreground_reservation_witness.as_ref()
    }

    pub fn quantum_budget_receipt(&self) -> &QuantumBudgetReceipt {
        &self.quantum_budget_receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeferredMaintenancePlan {
    descriptor: MaintenanceWorkDescriptor,
    reason: String,
}

impl DeferredMaintenancePlan {
    pub(crate) fn new(descriptor: MaintenanceWorkDescriptor, reason: impl Into<String>) -> Self {
        Self {
            descriptor,
            reason: reason.into(),
        }
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReservedMaintenanceWork {
    admitted_work: AdmittedMaintenanceWork,
    quantum_budget_receipt: QuantumBudgetReceipt,
    escalation_decision: MaintenanceEscalationDecision,
}

impl ReservedMaintenanceWork {
    pub(crate) fn new(
        admitted_work: AdmittedMaintenanceWork,
        quantum_budget_receipt: QuantumBudgetReceipt,
        escalation_decision: MaintenanceEscalationDecision,
    ) -> Self {
        Self {
            admitted_work,
            quantum_budget_receipt,
            escalation_decision,
        }
    }

    pub fn admitted_work(&self) -> &AdmittedMaintenanceWork {
        &self.admitted_work
    }

    pub fn quantum_budget_receipt(&self) -> &QuantumBudgetReceipt {
        &self.quantum_budget_receipt
    }

    pub fn escalation_decision(&self) -> MaintenanceEscalationDecision {
        self.escalation_decision
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExecutingMaintenanceWork {
    reserved_work: ReservedMaintenanceWork,
}

impl ExecutingMaintenanceWork {
    pub(crate) fn new(reserved_work: ReservedMaintenanceWork) -> Self {
        Self { reserved_work }
    }

    pub fn reserved_work(&self) -> &ReservedMaintenanceWork {
        &self.reserved_work
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CancelledMaintenanceWork {
    declaration_id: MaintenanceDeclarationId,
    descriptor: MaintenanceWorkDescriptor,
    reason: String,
}

impl CancelledMaintenanceWork {
    pub(crate) fn new(
        declaration_id: MaintenanceDeclarationId,
        descriptor: MaintenanceWorkDescriptor,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            declaration_id,
            descriptor,
            reason: reason.into(),
        }
    }

    pub fn declaration_id(&self) -> &MaintenanceDeclarationId {
        &self.declaration_id
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupersededMaintenanceWitness {
    superseded_identity: MaintenanceWorkIdentity,
    admitted_identity: MaintenanceWorkIdentity,
    reason: String,
}

impl SupersededMaintenanceWitness {
    pub(crate) fn new(
        superseded_identity: MaintenanceWorkIdentity,
        admitted_identity: MaintenanceWorkIdentity,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            superseded_identity,
            admitted_identity,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecoveredMaintenanceDescriptor {
    descriptor: MaintenanceWorkDescriptor,
}

impl RecoveredMaintenanceDescriptor {
    pub(crate) fn new(descriptor: MaintenanceWorkDescriptor) -> Self {
        Self { descriptor }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestartMaintenanceAdmission {
    recovered_descriptor: RecoveredMaintenanceDescriptor,
}

impl RestartMaintenanceAdmission {
    pub(crate) fn new(recovered_descriptor: RecoveredMaintenanceDescriptor) -> Self {
        Self { recovered_descriptor }
    }
}

impl MaintenanceDeclaration {
    pub fn work_descriptor(&self) -> MaintenanceWorkDescriptor {
        let work_class = self.work_class();
        let locality_scope = self.locality_scope();
        let locality_scope_token = LocalityScopeToken::new(locality_scope_token_string(&locality_scope));
        let work_identity = MaintenanceWorkIdentity::new(self.id().as_str().to_string());
        let equivalence_key = MaintenanceEquivalenceKey::new(format!(
            "{:?}:{}:{}",
            work_class,
            locality_scope_token.as_str(),
            self.id().as_str()
        ));
        MaintenanceWorkDescriptor::new(
            self.id().clone(),
            work_class,
            self.execution_posture(),
            locality_scope,
            locality_scope_token,
            self.predicted_demand(),
            self.reservation_family(),
            work_identity,
            equivalence_key,
            PlanGeneration::new(0),
            SupersessionEpoch::new(0),
            FreshnessWindow::new(1),
            self.debt_family(),
            MaintenanceEscalationDecision::StayBackground,
            self.tier_work_container_class(),
            false,
        )
    }

    pub fn work_class(&self) -> MaintenanceWorkClass {
        match self {
            Self::Retention { .. } => MaintenanceWorkClass::RetentionAudit,
            Self::Compaction { .. } => MaintenanceWorkClass::CompactionMaintenance,
            Self::Reclaim { .. } => MaintenanceWorkClass::DerivedArtifactReclaim,
            Self::AuthoritativeReclaim { .. } => MaintenanceWorkClass::AuthoritativeReclaim,
            Self::Rebuild { .. } => MaintenanceWorkClass::RetainedRangeRebuild,
        }
    }

    pub fn execution_posture(&self) -> MaintenanceExecutionPosture {
        match self.work_class() {
            MaintenanceWorkClass::RetentionAudit
            | MaintenanceWorkClass::CompactionMaintenance
            | MaintenanceWorkClass::DerivedArtifactReclaim
            | MaintenanceWorkClass::AuthoritativeReclaim
            | MaintenanceWorkClass::SnapshotRefresh
            | MaintenanceWorkClass::MaintenanceAudit => MaintenanceExecutionPosture::ForegroundAware,
            MaintenanceWorkClass::RetainedRangeRebuild
            | MaintenanceWorkClass::DerivedFamilyRebuild
            | MaintenanceWorkClass::ReplicationPreparation
            | MaintenanceWorkClass::TierPlacementProposal
            | MaintenanceWorkClass::TierMoveExecution => {
                MaintenanceExecutionPosture::FullyDeferrable
            }
        }
    }

    pub fn locality_scope(&self) -> MaintenanceLocalityScope {
        match self {
            Self::Retention { .. } => MaintenanceLocalityScope::StoreGlobalLocalityScope,
            Self::Compaction { declaration, .. } => {
                let family_label = declaration
                    .family_labels()
                    .first()
                    .cloned()
                    .unwrap_or_else(|| declaration.retained_basis_label().to_string());
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope { family_label }
            }
            Self::Reclaim { declaration, .. } => MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                family_label: declaration.artifact_family().to_string(),
            },
            Self::AuthoritativeReclaim { declaration, .. } => MaintenanceLocalityScope::BranchLocalityScope {
                branch_label: declaration.branch_id().0.clone(),
            },
            Self::Rebuild { declaration, .. } => MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                family_label: declaration.family_label().to_string(),
            },
        }
    }

    pub fn reservation_family(&self) -> MaintenanceReservationFamily {
        MaintenanceReservationFamily::Background(BackgroundReservationFamily::Maintenance)
    }

    pub fn predicted_demand(&self) -> MaintenanceDescriptorDemand {
        match self {
            Self::Retention { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(1),
                CpuBudgetUnits::new(1),
                MemoryBudgetUnits::new(1),
                PublicationSlotBudget::new(0),
                ForegroundLatencyGuard::new(1),
            ),
            Self::Compaction { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(4),
                CpuBudgetUnits::new(3),
                MemoryBudgetUnits::new(2),
                PublicationSlotBudget::new(1),
                ForegroundLatencyGuard::new(2),
            ),
            Self::Reclaim { .. } | Self::AuthoritativeReclaim { .. } => {
                MaintenanceDescriptorDemand::new(
                    IoBudgetUnits::new(2),
                    CpuBudgetUnits::new(1),
                    MemoryBudgetUnits::new(1),
                    PublicationSlotBudget::new(0),
                    ForegroundLatencyGuard::new(1),
                )
            }
            Self::Rebuild { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(3),
                CpuBudgetUnits::new(2),
                MemoryBudgetUnits::new(2),
                PublicationSlotBudget::new(1),
                ForegroundLatencyGuard::new(1),
            ),
        }
    }

    pub fn debt_family(&self) -> Option<MaintenanceDebtFamily> {
        match self {
            Self::Compaction { .. } => Some(MaintenanceDebtFamily::CompactionDebt),
            Self::Rebuild { .. } => Some(MaintenanceDebtFamily::RebuildDebt),
            _ => None,
        }
    }

    pub fn tier_work_container_class(&self) -> Option<TierWorkContainerClass> {
        None
    }
}

fn locality_scope_token_string(scope: &MaintenanceLocalityScope) -> String {
    match scope {
        MaintenanceLocalityScope::BranchLocalityScope { branch_label } => {
            format!("branch:{branch_label}")
        }
        MaintenanceLocalityScope::ArtifactFamilyLocalityScope { family_label } => {
            format!("family:{family_label}")
        }
        MaintenanceLocalityScope::TenantLocalityScope { tenant_label } => {
            format!("tenant:{tenant_label}")
        }
        MaintenanceLocalityScope::StoreGlobalLocalityScope => "store:global".to_string(),
    }
}
