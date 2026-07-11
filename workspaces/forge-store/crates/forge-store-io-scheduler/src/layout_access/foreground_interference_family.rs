use forge_store_budgets::CounterEvidenceStrength;
use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase26_foreground_interference_rule, AdmittedForegroundInterferenceLayoutRule,
};

use crate::{
    BackgroundResourceBudget, InterferenceCounterName, InterferenceCounterRow,
    LatencyEnvelopeAssessment, LatencyEnvelopeAssessmentStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ForegroundInterferenceLayoutFamilyHome;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ForegroundInterferenceLayoutAdmission {
    _rule: AdmittedForegroundInterferenceLayoutRule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AdmittedForegroundInterferenceLayoutFamily {
    _admission: ForegroundInterferenceLayoutAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForegroundInterferencePosture {
    Held,
    ExecutionViolated,
    BackendContradictedWitness,
    EnvelopeExceeded,
    PolicyDebtIncurred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForegroundInterferenceAccessBudget {
    requested_budget: BackgroundResourceBudget,
    max_interference_events: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForegroundInterferenceLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    interference_posture: ForegroundInterferencePosture,
    declared_budget: ForegroundInterferenceAccessBudget,
    exact_rows: Vec<InterferenceCounterRow>,
}

impl ForegroundInterferenceLayoutFamilyHome {
    const fn s8() -> Self {
        Self
    }

    fn admit(
        self,
        rule: AdmittedForegroundInterferenceLayoutRule,
    ) -> ForegroundInterferenceLayoutAdmission {
        let _ = self;
        ForegroundInterferenceLayoutAdmission { _rule: rule }
    }
}

fn foreground_interference_layout() -> AdmittedForegroundInterferenceLayoutFamily {
    AdmittedForegroundInterferenceLayoutFamily {
        _admission: ForegroundInterferenceLayoutFamilyHome::s8().admit(
            phase26_foreground_interference_rule()
                .expect("phase 26 foreground interference rule must stay admitted"),
        ),
    }
}

impl AdmittedForegroundInterferenceLayoutFamily {
    fn admit_foreground_interference(
        &self,
        assessment: &LatencyEnvelopeAssessment,
    ) -> ForegroundInterferenceLayoutReport {
        let _ = self;
        ForegroundInterferenceLayoutReport {
            family_id: DurableArtifactFamilyId::ForegroundInterferenceRecord,
            access_shape: S8AccessShape::PointLookup,
            rebuild_posture: DurableArtifactRebuildPosture::NoRebuild,
            interference_posture: posture_for(assessment.status()),
            declared_budget: ForegroundInterferenceAccessBudget {
                requested_budget: assessment.replay_identity().requested_budget(),
                max_interference_events: assessment.max_interference_events(),
            },
            exact_rows: exact_rows(assessment),
        }
    }
}

impl ForegroundInterferenceLayoutReport {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> S8AccessShape {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn interference_posture(&self) -> ForegroundInterferencePosture {
        self.interference_posture
    }

    pub const fn declared_budget(&self) -> ForegroundInterferenceAccessBudget {
        self.declared_budget
    }

    pub fn exact_rows(&self) -> &[InterferenceCounterRow] {
        &self.exact_rows
    }

    pub fn exact_counter(&self, name: InterferenceCounterName) -> Option<InterferenceCounterRow> {
        self.exact_rows
            .iter()
            .copied()
            .find(|row| row.name() == name)
    }
}

impl LatencyEnvelopeAssessment {
    pub fn admit_foreground_interference_layout(&self) -> ForegroundInterferenceLayoutReport {
        foreground_interference_layout().admit_foreground_interference(self)
    }
}

impl ForegroundInterferenceAccessBudget {
    pub const fn requested_budget(&self) -> BackgroundResourceBudget {
        self.requested_budget
    }

    pub const fn max_interference_events(&self) -> Option<u64> {
        self.max_interference_events
    }
}

fn posture_for(status: LatencyEnvelopeAssessmentStatus) -> ForegroundInterferencePosture {
    match status {
        LatencyEnvelopeAssessmentStatus::Held => ForegroundInterferencePosture::Held,
        LatencyEnvelopeAssessmentStatus::ExecutionViolated => {
            ForegroundInterferencePosture::ExecutionViolated
        }
        LatencyEnvelopeAssessmentStatus::BackendContradictedWitness => {
            ForegroundInterferencePosture::BackendContradictedWitness
        }
        LatencyEnvelopeAssessmentStatus::EnvelopeExceeded => {
            ForegroundInterferencePosture::EnvelopeExceeded
        }
        LatencyEnvelopeAssessmentStatus::PolicyDebtIncurred => {
            ForegroundInterferencePosture::PolicyDebtIncurred
        }
    }
}

fn exact_rows(assessment: &LatencyEnvelopeAssessment) -> Vec<InterferenceCounterRow> {
    assessment
        .counter_rows()
        .iter()
        .copied()
        .filter(|row| row.strength() == CounterEvidenceStrength::Exact)
        .collect()
}
