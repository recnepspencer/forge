use super::construction::classify_loop_role_outcomes;
use super::counters::PlanarBooleanLoopRoleOutcomeBoundaryCounters;
use super::input::PlanarBooleanLoopRoleOutcomeBoundaryInput;
use super::row::{PlanarBooleanLoopContainmentEvidencePosture, PlanarBooleanLoopRoleOutcome};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopRoleOutcomeSet {
    set_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanLoopRoleOutcome>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopContainmentEvidencePostureSet {
    set_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanLoopContainmentEvidencePosture>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopRoleOutcomeBoundary {
    role_outcomes: PlanarBooleanLoopRoleOutcomeSet,
    containment_evidence_postures: PlanarBooleanLoopContainmentEvidencePostureSet,
    counters: PlanarBooleanLoopRoleOutcomeBoundaryCounters,
}

impl PlanarBooleanLoopRoleOutcomeBoundary {
    pub fn classify(input: PlanarBooleanLoopRoleOutcomeBoundaryInput<'_>) -> Self {
        classify_loop_role_outcomes(input)
    }

    pub(crate) fn new(
        role_outcomes: PlanarBooleanLoopRoleOutcomeSet,
        containment_evidence_postures: PlanarBooleanLoopContainmentEvidencePostureSet,
        counters: PlanarBooleanLoopRoleOutcomeBoundaryCounters,
    ) -> Self {
        Self {
            role_outcomes,
            containment_evidence_postures,
            counters,
        }
    }

    pub fn role_outcomes(&self) -> &PlanarBooleanLoopRoleOutcomeSet {
        &self.role_outcomes
    }

    pub fn containment_evidence_postures(&self) -> &PlanarBooleanLoopContainmentEvidencePostureSet {
        &self.containment_evidence_postures
    }

    pub fn counters(&self) -> PlanarBooleanLoopRoleOutcomeBoundaryCounters {
        self.counters
    }
}

impl PlanarBooleanLoopRoleOutcomeSet {
    pub(crate) fn new(
        set_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanLoopRoleOutcome>,
    ) -> Self {
        Self {
            set_identity,
            request_identity,
            rows,
        }
    }

    pub fn role_outcome_set_identity(&self) -> &str {
        &self.set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopRoleOutcome] {
        &self.rows
    }
}

impl PlanarBooleanLoopContainmentEvidencePostureSet {
    pub(crate) fn new(
        set_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanLoopContainmentEvidencePosture>,
    ) -> Self {
        Self {
            set_identity,
            request_identity,
            rows,
        }
    }

    pub fn containment_evidence_posture_set_identity(&self) -> &str {
        &self.set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopContainmentEvidencePosture] {
        &self.rows
    }
}
