use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectFit,
};
use crate::target_binding::{WorthQueryBindingTarget, WorthQueryBindingTargetKind};

use super::{WorthQueryBindingOutcome, WorthQueryBindingSourceKind, WorthQueryBindingSpecificity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingRequestDescriptor {
    family_key: &'static str,
    request_kind: &'static str,
    required_aspect_contract: WorthQueryDeclarationAspectContract,
}

impl WorthQueryBindingRequestDescriptor {
    pub(crate) fn new(
        family_key: &'static str,
        request_kind: &'static str,
        required_aspect_contract: WorthQueryDeclarationAspectContract,
    ) -> Self {
        Self {
            family_key,
            request_kind,
            required_aspect_contract,
        }
    }

    pub fn family_key(&self) -> &'static str {
        self.family_key
    }

    pub fn request_kind(&self) -> &'static str {
        self.request_kind
    }

    pub fn required_aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.required_aspect_contract
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingCandidateRecord {
    label: String,
    source_kind: WorthQueryBindingSourceKind,
    specificity: WorthQueryBindingSpecificity,
    target_kind: Option<WorthQueryBindingTargetKind>,
    target_digest: Option<String>,
}

impl WorthQueryBindingCandidateRecord {
    pub(crate) fn new(
        label: String,
        source_kind: WorthQueryBindingSourceKind,
        specificity: WorthQueryBindingSpecificity,
        target: Option<&WorthQueryBindingTarget>,
    ) -> Self {
        Self {
            label,
            source_kind,
            specificity,
            target_kind: target.map(|target: &WorthQueryBindingTarget| target.kind()),
            target_digest: target
                .map(|target: &WorthQueryBindingTarget| target.target_digest().to_string()),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn source_kind(&self) -> WorthQueryBindingSourceKind {
        self.source_kind
    }

    pub fn specificity(&self) -> WorthQueryBindingSpecificity {
        self.specificity
    }

    pub fn target_kind(&self) -> Option<WorthQueryBindingTargetKind> {
        self.target_kind
    }

    pub fn target_digest(&self) -> Option<&str> {
        self.target_digest.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingWitnessCheck {
    name: &'static str,
    passed: bool,
    reason: Option<String>,
}

impl WorthQueryBindingWitnessCheck {
    pub(crate) fn passed(name: &'static str) -> Self {
        Self {
            name,
            passed: true,
            reason: None,
        }
    }

    pub(crate) fn failed(name: &'static str, reason: impl Into<String>) -> Self {
        Self {
            name,
            passed: false,
            reason: Some(reason.into()),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn did_pass(&self) -> bool {
        self.passed
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingAspectFitReport {
    fit: WorthQueryDeclarationAspectFit,
    contract: WorthQueryDeclarationAspectContract,
    coverage: WorthQueryDeclarationAspectCoverage,
}

impl WorthQueryBindingAspectFitReport {
    pub(crate) fn new(
        fit: WorthQueryDeclarationAspectFit,
        contract: WorthQueryDeclarationAspectContract,
        coverage: WorthQueryDeclarationAspectCoverage,
    ) -> Self {
        Self {
            fit,
            contract,
            coverage,
        }
    }

    pub fn fit(&self) -> WorthQueryDeclarationAspectFit {
        self.fit
    }

    pub fn contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.contract
    }

    pub fn coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.coverage
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingNarrowingDecision {
    reason: String,
}

impl WorthQueryBindingNarrowingDecision {
    pub(crate) fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingLinkedArtifacts {
    declaration_digest: Option<String>,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: Option<String>,
    envelope_digest: Option<String>,
    orchestration_digest: Option<String>,
}

impl WorthQueryBindingLinkedArtifacts {
    pub(crate) fn new() -> Self {
        Self {
            declaration_digest: None,
            progression_digest: None,
            route_plan_digest: None,
            receipt_digest: None,
            envelope_digest: None,
            orchestration_digest: None,
        }
    }

    pub(crate) fn with_declaration_digest(mut self, value: impl Into<String>) -> Self {
        self.declaration_digest = Some(value.into());
        self
    }

    pub(crate) fn with_progression_digest(mut self, value: impl Into<String>) -> Self {
        self.progression_digest = Some(value.into());
        self
    }

    pub(crate) fn with_route_plan_digest(mut self, value: impl Into<String>) -> Self {
        self.route_plan_digest = Some(value.into());
        self
    }

    pub(crate) fn with_receipt_digest(mut self, value: impl Into<String>) -> Self {
        self.receipt_digest = Some(value.into());
        self
    }

    pub(crate) fn with_envelope_digest(mut self, value: impl Into<String>) -> Self {
        self.envelope_digest = Some(value.into());
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_orchestration_digest(mut self, value: impl Into<String>) -> Self {
        self.orchestration_digest = Some(value.into());
        self
    }

    pub fn declaration_digest(&self) -> Option<&str> {
        self.declaration_digest.as_deref()
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.progression_digest.as_deref()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.route_plan_digest.as_deref()
    }

    pub fn receipt_digest(&self) -> Option<&str> {
        self.receipt_digest.as_deref()
    }

    pub fn envelope_digest(&self) -> Option<&str> {
        self.envelope_digest.as_deref()
    }

    pub fn orchestration_digest(&self) -> Option<&str> {
        self.orchestration_digest.as_deref()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryBindingTranscript<T> {
    request: WorthQueryBindingRequestDescriptor,
    outcome: WorthQueryBindingOutcome<T>,
    candidates: Vec<WorthQueryBindingCandidateRecord>,
    witness_checks: Vec<WorthQueryBindingWitnessCheck>,
    aspect_fit_report: Option<WorthQueryBindingAspectFitReport>,
    narrowing_decisions: Vec<WorthQueryBindingNarrowingDecision>,
    resolved_target: Option<WorthQueryBindingTarget>,
    binding_digest: String,
    linked_artifacts: WorthQueryBindingLinkedArtifacts,
}

impl<T> WorthQueryBindingTranscript<T> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        request: WorthQueryBindingRequestDescriptor,
        outcome: WorthQueryBindingOutcome<T>,
        candidates: Vec<WorthQueryBindingCandidateRecord>,
        witness_checks: Vec<WorthQueryBindingWitnessCheck>,
        aspect_fit_report: Option<WorthQueryBindingAspectFitReport>,
        narrowing_decisions: Vec<WorthQueryBindingNarrowingDecision>,
        resolved_target: Option<WorthQueryBindingTarget>,
        binding_digest: String,
        linked_artifacts: WorthQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            request,
            outcome,
            candidates,
            witness_checks,
            aspect_fit_report,
            narrowing_decisions,
            resolved_target,
            binding_digest,
            linked_artifacts,
        }
    }

    pub fn request(&self) -> &WorthQueryBindingRequestDescriptor {
        &self.request
    }

    pub fn outcome(&self) -> &WorthQueryBindingOutcome<T> {
        &self.outcome
    }

    pub fn candidates(&self) -> &[WorthQueryBindingCandidateRecord] {
        &self.candidates
    }

    pub fn witness_checks(&self) -> &[WorthQueryBindingWitnessCheck] {
        &self.witness_checks
    }

    pub fn aspect_fit_report(&self) -> Option<&WorthQueryBindingAspectFitReport> {
        self.aspect_fit_report.as_ref()
    }

    pub fn narrowing_decisions(&self) -> &[WorthQueryBindingNarrowingDecision] {
        &self.narrowing_decisions
    }

    pub fn resolved_target(&self) -> Option<&WorthQueryBindingTarget> {
        self.resolved_target.as_ref()
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn linked_artifacts(&self) -> &WorthQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub fn into_outcome(self) -> WorthQueryBindingOutcome<T> {
        self.outcome
    }

    pub fn into_checked(self) -> crate::binding_pipeline::WorthQueryBindingChecked<T> {
        crate::binding_pipeline::WorthQueryBindingChecked::new(
            self.outcome,
            self.binding_digest,
            self.linked_artifacts,
        )
    }
}
