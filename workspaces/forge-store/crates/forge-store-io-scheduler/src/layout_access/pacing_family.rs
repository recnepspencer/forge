use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase26_background_pacing_rule, AdmittedBackgroundPacingLayoutRule,
};

use crate::{
    BackgroundIoPressureClass, BackgroundPacingCounterSnapshot, BackgroundPacingOutcome,
    BackgroundPacingStaleRebindKind, BackgroundResourceBudget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BackgroundPacingLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BackgroundPacingLayoutAdmission {
    _rule: AdmittedBackgroundPacingLayoutRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmittedBackgroundPacingLayoutFamily {
    _admission: BackgroundPacingLayoutAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundPacingInterferencePosture {
    Yield,
    Deferred,
    Denied,
    Stale,
    RebindRequired,
    Throttled,
    AdmittedWithDebt,
    Violation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundPacingLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    pressure_class: BackgroundIoPressureClass,
    requested_budget: BackgroundResourceBudget,
    admitted_budget: BackgroundResourceBudget,
    interference_posture: BackgroundPacingInterferencePosture,
    counters: BackgroundPacingCounterSnapshot,
}

impl BackgroundPacingLayoutFamilyHome {
    const fn s8() -> Self {
        Self
    }

    fn admit(self, rule: AdmittedBackgroundPacingLayoutRule) -> BackgroundPacingLayoutAdmission {
        let _ = self;
        BackgroundPacingLayoutAdmission { _rule: rule }
    }
}

fn background_pacing_layout() -> AdmittedBackgroundPacingLayoutFamily {
    AdmittedBackgroundPacingLayoutFamily {
        _admission: BackgroundPacingLayoutFamilyHome::s8().admit(
            phase26_background_pacing_rule()
                .expect("phase 26 background pacing rule must stay admitted"),
        ),
    }
}

impl AdmittedBackgroundPacingLayoutFamily {
    fn admit_background_pacing(
        &self,
        outcome: BackgroundPacingOutcome,
    ) -> BackgroundPacingLayoutReport {
        let _ = self;
        let (interference_posture, admitted_budget, counters) = pacing_basis(outcome);
        BackgroundPacingLayoutReport {
            family_id: DurableArtifactFamilyId::BackgroundPacingRecord,
            access_shape: S8AccessShape::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::NoRebuild,
            pressure_class: outcome.class(),
            requested_budget: counters.requested(),
            admitted_budget,
            interference_posture,
            counters,
        }
    }
}

impl BackgroundPacingLayoutReport {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> S8AccessShape {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn pressure_class(&self) -> BackgroundIoPressureClass {
        self.pressure_class
    }

    pub const fn requested_budget(&self) -> BackgroundResourceBudget {
        self.requested_budget
    }

    pub const fn admitted_budget(&self) -> BackgroundResourceBudget {
        self.admitted_budget
    }

    pub const fn interference_posture(&self) -> BackgroundPacingInterferencePosture {
        self.interference_posture
    }

    pub const fn exact_counters(&self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl BackgroundPacingOutcome {
    pub fn admit_background_pacing_layout(&self) -> BackgroundPacingLayoutReport {
        background_pacing_layout().admit_background_pacing(*self)
    }
}

fn pacing_basis(
    outcome: BackgroundPacingOutcome,
) -> (
    BackgroundPacingInterferencePosture,
    BackgroundResourceBudget,
    BackgroundPacingCounterSnapshot,
) {
    match outcome {
        BackgroundPacingOutcome::Yield(outcome) => (
            BackgroundPacingInterferencePosture::Yield,
            BackgroundResourceBudget::new(),
            outcome.counters(),
        ),
        BackgroundPacingOutcome::Deferred(outcome) => (
            BackgroundPacingInterferencePosture::Deferred,
            BackgroundResourceBudget::new(),
            outcome.counters(),
        ),
        BackgroundPacingOutcome::Denied(outcome) => (
            BackgroundPacingInterferencePosture::Denied,
            BackgroundResourceBudget::new(),
            outcome.counters(),
        ),
        BackgroundPacingOutcome::StaleRebindRequired(outcome) => (
            match outcome.kind() {
                BackgroundPacingStaleRebindKind::Stale => {
                    BackgroundPacingInterferencePosture::Stale
                }
                BackgroundPacingStaleRebindKind::RebindRequired => {
                    BackgroundPacingInterferencePosture::RebindRequired
                }
            },
            BackgroundResourceBudget::new(),
            outcome.counters(),
        ),
        BackgroundPacingOutcome::Throttled(outcome) => (
            BackgroundPacingInterferencePosture::Throttled,
            outcome.admitted_budget(),
            outcome.counters(),
        ),
        BackgroundPacingOutcome::AdmittedWithDebt(outcome) => (
            BackgroundPacingInterferencePosture::AdmittedWithDebt,
            outcome.lease().admitted_budget(),
            outcome.counters(),
        ),
        BackgroundPacingOutcome::Violation(outcome) => (
            BackgroundPacingInterferencePosture::Violation,
            outcome.counters().admitted_budget(),
            outcome.counters(),
        ),
    }
}
