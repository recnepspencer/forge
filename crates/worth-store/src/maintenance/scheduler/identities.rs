use serde::{Deserialize, Serialize};

use super::classes::{MaintenanceReservationFamily, MaintenanceWorkClass};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaintenanceLaneKey {
    work_class: MaintenanceWorkClass,
    locality_scope: MaintenanceLocalityScope,
    reservation_family: MaintenanceReservationFamily,
}

impl MaintenanceLaneKey {
    pub(crate) fn new(
        work_class: MaintenanceWorkClass,
        locality_scope: MaintenanceLocalityScope,
        reservation_family: MaintenanceReservationFamily,
    ) -> Self {
        Self {
            work_class,
            locality_scope,
            reservation_family,
        }
    }

    pub fn work_class(&self) -> MaintenanceWorkClass {
        self.work_class
    }

    pub fn locality_scope(&self) -> &MaintenanceLocalityScope {
        &self.locality_scope
    }

    pub fn reservation_family(&self) -> MaintenanceReservationFamily {
        self.reservation_family
    }

    pub fn artifact_id(&self) -> String {
        format!(
            "{:?}:{}:{:?}",
            self.work_class,
            locality_scope_token_string(&self.locality_scope),
            self.reservation_family
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceCoalescingDecision {
    NotCoalesced,
    CoalescedWithEquivalentLaneMember,
    CancelledAsSuperseded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceStarvationStatus {
    NotStarved,
    DeferredLanePressure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceDebtPressureClass {
    None,
    Active,
    Elevated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceEscalationVerdict {
    NoEscalation,
    DeferredForBudgetPressure,
    EscalatedForDebtPressure,
    RejectedIllegalLocalityPromotion,
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

pub(super) fn locality_scope_token_string(scope: &MaintenanceLocalityScope) -> String {
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
