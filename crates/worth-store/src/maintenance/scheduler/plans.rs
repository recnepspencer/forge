use serde::Serialize;

use super::super::MaintenanceDeclarationId;
use super::super::MaintenanceWorkDescriptor;

use super::admission::{AdmittedMaintenanceWork, QuantumBudgetReceipt};

use super::classes::{ForegroundReservationFamily, MaintenanceEscalationDecision};

use super::identities::{MaintenanceLocalityScope, MaintenanceWorkIdentity};

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
        Self {
            recovered_descriptor,
        }
    }
}
