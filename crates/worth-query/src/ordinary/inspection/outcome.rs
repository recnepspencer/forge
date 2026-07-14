use crate::runtime::{
    CausalInspectionArtifactKind, CausalInspectionPlan, CausalInspectionProofFlow,
    QueryCausalInspectionArtifact,
};
use crate::WorthQueryEvidenceIdentity;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryInspectionCounters {
    context_handoff_count: usize,
    planning_attempt_count: usize,
    materialization_attempt_count: usize,
    materialization_completed_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryInspectionCost {
    anchor_derivation_count: usize,
    evidence_reference_resolution_count: usize,
    admission_count: usize,
    bridge_envelope_assembly_count: usize,
    evidence_reference_count: usize,
}

impl WorthQueryInspectionCost {
    pub fn anchor_derivation_count(&self) -> usize {
        self.anchor_derivation_count
    }

    pub fn evidence_reference_resolution_count(&self) -> usize {
        self.evidence_reference_resolution_count
    }

    pub fn admission_count(&self) -> usize {
        self.admission_count
    }

    pub fn bridge_envelope_assembly_count(&self) -> usize {
        self.bridge_envelope_assembly_count
    }

    pub fn evidence_reference_count(&self) -> usize {
        self.evidence_reference_count
    }

    pub(crate) fn from_plan(plan: &CausalInspectionPlan, materialize: bool) -> Self {
        let estimated = plan.estimated_cost();
        Self {
            anchor_derivation_count: estimated.anchor_derivation_count(),
            evidence_reference_resolution_count: estimated.evidence_reference_resolution_count(),
            admission_count: estimated.admission_count(),
            bridge_envelope_assembly_count: usize::from(materialize),
            evidence_reference_count: estimated.evidence_reference_count(),
        }
    }
}

impl WorthQueryInspectionCounters {
    pub fn context_handoff_count(&self) -> usize {
        self.context_handoff_count
    }

    pub fn planning_attempt_count(&self) -> usize {
        self.planning_attempt_count
    }

    pub fn materialization_attempt_count(&self) -> usize {
        self.materialization_attempt_count
    }

    pub fn materialization_completed_count(&self) -> usize {
        self.materialization_completed_count
    }

    pub(crate) fn context_handed_off() -> Self {
        Self {
            context_handoff_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn planning_attempted(mut self) -> Self {
        self.planning_attempt_count = 1;
        self
    }

    pub(crate) fn materialization_attempted(mut self) -> Self {
        self.materialization_attempt_count = 1;
        self
    }

    pub(crate) fn materialization_completed(mut self) -> Self {
        self.materialization_completed_count = 1;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInspectionReceipt {
    identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryInspectionReceipt {
    pub fn identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }

    pub fn identity_for_reporting(&self) -> &str {
        self.identity.as_str()
    }

    pub(crate) fn from_plan(plan: &CausalInspectionPlan) -> Self {
        let identity = match plan.admission() {
            CausalInspectionProofFlow::Admitted(inspection) => inspection.receipt(),
            CausalInspectionProofFlow::Advisory(inspection) => inspection.receipt(),
            CausalInspectionProofFlow::Denied(inspection) => inspection.receipt(),
        }
        .evidence_identity()
        .clone();
        Self { identity }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInspectionMaterializationKind {
    Admitted,
    Advisory,
    Denied,
}

pub struct WorthQueryInspectionMaterialization {
    artifact: QueryCausalInspectionArtifact,
}

impl WorthQueryInspectionMaterialization {
    pub fn kind(&self) -> WorthQueryInspectionMaterializationKind {
        match self.artifact.primary_result() {
            CausalInspectionArtifactKind::Admitted => {
                WorthQueryInspectionMaterializationKind::Admitted
            }
            CausalInspectionArtifactKind::Advisory => {
                WorthQueryInspectionMaterializationKind::Advisory
            }
            CausalInspectionArtifactKind::Denied => WorthQueryInspectionMaterializationKind::Denied,
        }
    }

    pub fn warnings(&self) -> Vec<&str> {
        self.artifact.warnings()
    }

    pub fn artifact_identity_for_reporting(&self) -> &str {
        self.artifact.integrity().artifact_for_reporting()
    }

    pub fn causal_identity_for_reporting(&self) -> &str {
        self.artifact.integrity().causal_identity_for_reporting()
    }

    pub fn query_decision_for_reporting(&self) -> &str {
        self.artifact
            .decision_trace()
            .query_decision_for_reporting()
    }

    pub fn bridge_envelope_for_reporting(&self) -> Option<&str> {
        self.artifact
            .decision_trace()
            .bridge_envelope_for_reporting()
    }

    pub fn bridge_denial_for_reporting(&self) -> Option<&str> {
        self.artifact.decision_trace().bridge_denial_for_reporting()
    }

    pub(crate) fn new(artifact: QueryCausalInspectionArtifact) -> Self {
        Self { artifact }
    }
}

pub struct WorthQueryInspectionCompletion {
    receipt: WorthQueryInspectionReceipt,
    estimated_cost: WorthQueryInspectionCost,
    materialization: Option<WorthQueryInspectionMaterialization>,
    counters: WorthQueryInspectionCounters,
}

impl WorthQueryInspectionCompletion {
    pub fn receipt(&self) -> &WorthQueryInspectionReceipt {
        &self.receipt
    }

    pub fn estimated_cost(&self) -> &WorthQueryInspectionCost {
        &self.estimated_cost
    }

    pub fn materialization(&self) -> Option<&WorthQueryInspectionMaterialization> {
        self.materialization.as_ref()
    }

    pub fn counters(&self) -> &WorthQueryInspectionCounters {
        &self.counters
    }

    pub(crate) fn new(
        receipt: WorthQueryInspectionReceipt,
        estimated_cost: WorthQueryInspectionCost,
        materialization: Option<WorthQueryInspectionMaterialization>,
        counters: WorthQueryInspectionCounters,
    ) -> Self {
        Self {
            receipt,
            estimated_cost,
            materialization,
            counters,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInspectionStopSource {
    BasisMismatch,
    InvalidOutcomeEvidence,
    MissingEvidence,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInspectionUnavailableSource {
    Runtime,
    Materialization,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInspectionNextAction {
    RefreshInspectionBasis,
    InspectOutcomeEvidence,
    RequestAvailableEvidence,
    ReviseDeclaration,
    ConfigureRuntime,
    UseOperationalReceipt,
}

pub struct WorthQueryInspectionStop {
    source: WorthQueryInspectionStopSource,
    evidence_for_reporting: String,
    counters: WorthQueryInspectionCounters,
}

impl WorthQueryInspectionStop {
    pub fn source(&self) -> WorthQueryInspectionStopSource {
        self.source
    }

    pub fn evidence_for_reporting(&self) -> &str {
        &self.evidence_for_reporting
    }

    pub fn counters(&self) -> &WorthQueryInspectionCounters {
        &self.counters
    }

    pub fn next_action(&self) -> WorthQueryInspectionNextAction {
        match self.source {
            WorthQueryInspectionStopSource::BasisMismatch => {
                WorthQueryInspectionNextAction::RefreshInspectionBasis
            }
            WorthQueryInspectionStopSource::InvalidOutcomeEvidence => {
                WorthQueryInspectionNextAction::InspectOutcomeEvidence
            }
            WorthQueryInspectionStopSource::MissingEvidence => {
                WorthQueryInspectionNextAction::RequestAvailableEvidence
            }
            WorthQueryInspectionStopSource::InvalidRequest => {
                WorthQueryInspectionNextAction::ReviseDeclaration
            }
        }
    }

    pub(crate) fn new(
        source: WorthQueryInspectionStopSource,
        evidence_for_reporting: impl Into<String>,
        counters: WorthQueryInspectionCounters,
    ) -> Self {
        Self {
            source,
            evidence_for_reporting: evidence_for_reporting.into(),
            counters,
        }
    }
}

pub struct WorthQueryInspectionUnavailable {
    source: WorthQueryInspectionUnavailableSource,
    message: String,
    receipt: WorthQueryInspectionReceipt,
    estimated_cost: WorthQueryInspectionCost,
    counters: WorthQueryInspectionCounters,
}

impl WorthQueryInspectionUnavailable {
    pub fn source(&self) -> WorthQueryInspectionUnavailableSource {
        self.source
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn receipt(&self) -> &WorthQueryInspectionReceipt {
        &self.receipt
    }

    pub fn estimated_cost(&self) -> &WorthQueryInspectionCost {
        &self.estimated_cost
    }

    pub fn counters(&self) -> &WorthQueryInspectionCounters {
        &self.counters
    }

    pub fn next_action(&self) -> WorthQueryInspectionNextAction {
        match self.source {
            WorthQueryInspectionUnavailableSource::Runtime => {
                WorthQueryInspectionNextAction::ConfigureRuntime
            }
            WorthQueryInspectionUnavailableSource::Materialization => {
                WorthQueryInspectionNextAction::UseOperationalReceipt
            }
        }
    }

    pub(crate) fn new(
        source: WorthQueryInspectionUnavailableSource,
        message: impl Into<String>,
        receipt: WorthQueryInspectionReceipt,
        estimated_cost: WorthQueryInspectionCost,
        counters: WorthQueryInspectionCounters,
    ) -> Self {
        Self {
            source,
            message: message.into(),
            receipt,
            estimated_cost,
            counters,
        }
    }
}

pub enum WorthQueryInspectionOutcome {
    Completed(WorthQueryInspectionCompletion),
    Advisory(WorthQueryInspectionCompletion),
    Violation(WorthQueryInspectionCompletion),
    Unavailable(WorthQueryInspectionUnavailable),
    Stopped(WorthQueryInspectionStop),
}

impl WorthQueryInspectionOutcome {
    pub fn completion(&self) -> Option<&WorthQueryInspectionCompletion> {
        match self {
            Self::Completed(completion)
            | Self::Advisory(completion)
            | Self::Violation(completion) => Some(completion),
            Self::Unavailable(_) | Self::Stopped(_) => None,
        }
    }

    pub fn unavailable(&self) -> Option<&WorthQueryInspectionUnavailable> {
        match self {
            Self::Unavailable(unavailable) => Some(unavailable),
            _ => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryInspectionStop> {
        match self {
            Self::Stopped(stop) => Some(stop),
            _ => None,
        }
    }
}
