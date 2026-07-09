use serde::{Deserialize, Serialize};

use super::denial::DeniedHostComputedReadSet;
use super::descriptor::HostComputedDescriptor;
use super::diagnostics::HostComputedDiagnosticsSummary;
use super::prepared::PreparedHostComputedEvaluation;
use super::request::HostComputedEvaluationRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostComputedFailureClass {
    HostAdapterRejected,
    RuntimeInvariantViolation,
}

impl HostComputedFailureClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HostAdapterRejected => "HostAdapterRejected",
            Self::RuntimeInvariantViolation => "RuntimeInvariantViolation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostComputedFailure {
    descriptor: HostComputedDescriptor,
    class: HostComputedFailureClass,
    message: String,
    diagnostics_summary: HostComputedDiagnosticsSummary,
}

impl HostComputedFailure {
    pub(crate) fn new(
        descriptor: HostComputedDescriptor,
        class: HostComputedFailureClass,
        message: impl Into<String>,
    ) -> Self {
        let diagnostics_summary =
            HostComputedDiagnosticsSummary::failed(&descriptor, class.as_str());
        Self {
            descriptor,
            class,
            message: message.into(),
            diagnostics_summary,
        }
    }

    pub fn descriptor(&self) -> &HostComputedDescriptor {
        &self.descriptor
    }

    pub fn class(&self) -> HostComputedFailureClass {
        self.class
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn diagnostics_summary(&self) -> &HostComputedDiagnosticsSummary {
        &self.diagnostics_summary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedHostComputedArtifact {
    prepared: PreparedHostComputedEvaluation,
}

impl StagedHostComputedArtifact {
    pub(crate) fn new(prepared: PreparedHostComputedEvaluation) -> Self {
        Self { prepared }
    }

    pub fn prepared(&self) -> &PreparedHostComputedEvaluation {
        &self.prepared
    }

    pub fn diagnostics_summary(&self) -> &HostComputedDiagnosticsSummary {
        self.prepared.diagnostics_summary()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommittedHostComputedArtifact {
    staged: StagedHostComputedArtifact,
}

impl CommittedHostComputedArtifact {
    pub(crate) fn new(staged: StagedHostComputedArtifact) -> Self {
        Self { staged }
    }

    pub fn staged(&self) -> &StagedHostComputedArtifact {
        &self.staged
    }

    pub fn diagnostics_summary(&self) -> HostComputedDiagnosticsSummary {
        self.staged
            .diagnostics_summary()
            .clone()
            .with_outcome(super::diagnostics::HostComputedOutcomeClass::Committed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedHostComputedEvaluation {
    request: HostComputedEvaluationRequest,
    denial: DeniedHostComputedReadSet,
    diagnostics_summary: HostComputedDiagnosticsSummary,
}

impl DeniedHostComputedEvaluation {
    pub(crate) fn new(
        request: HostComputedEvaluationRequest,
        denial: DeniedHostComputedReadSet,
    ) -> Self {
        let diagnostics_summary = HostComputedDiagnosticsSummary::denied(&request, denial.class());
        Self {
            request,
            denial,
            diagnostics_summary,
        }
    }

    pub fn request(&self) -> &HostComputedEvaluationRequest {
        &self.request
    }

    pub fn denial(&self) -> &DeniedHostComputedReadSet {
        &self.denial
    }

    pub fn diagnostics_summary(&self) -> &HostComputedDiagnosticsSummary {
        &self.diagnostics_summary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostComputedEvaluationOutcome {
    Committed(CommittedHostComputedArtifact),
    Denied(DeniedHostComputedEvaluation),
    Failed(HostComputedFailure),
}

impl HostComputedEvaluationOutcome {
    pub(crate) fn committed(prepared: PreparedHostComputedEvaluation) -> Self {
        let staged = StagedHostComputedArtifact::new(prepared);
        Self::Committed(CommittedHostComputedArtifact::new(staged))
    }

    pub(crate) fn denied(
        request: HostComputedEvaluationRequest,
        denial: DeniedHostComputedReadSet,
    ) -> Self {
        Self::Denied(DeniedHostComputedEvaluation::new(request, denial))
    }

    pub(crate) fn failed(
        descriptor: HostComputedDescriptor,
        class: HostComputedFailureClass,
        message: impl Into<String>,
    ) -> Self {
        Self::Failed(HostComputedFailure::new(descriptor, class, message))
    }

    pub fn diagnostics_summary(&self) -> HostComputedDiagnosticsSummary {
        match self {
            Self::Committed(artifact) => artifact.diagnostics_summary(),
            Self::Denied(denied) => denied.diagnostics_summary().clone(),
            Self::Failed(failure) => failure.diagnostics_summary().clone(),
        }
    }
}
