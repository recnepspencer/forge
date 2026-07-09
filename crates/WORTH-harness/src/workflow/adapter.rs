use std::fmt;

use super::capability::WorkflowCertificationCapabilities;
use super::record::{
    ArtifactBundle, FailureInjectionPoint, InvariantCheck, InvariantReport, ReproductionMetadata,
    WorkflowCaptureRequest, WorkflowCheckpoint, WorkflowFailureContext, WorkflowPlan,
    WorkflowRuntimeProfile, WorkflowState, WorkflowStep, WorkflowStepOutcome,
};

pub trait WorkflowCertificationAdapter {
    type Session;
    type Step;
    type Error;

    fn adapter_name(&self) -> &'static str;
    fn capabilities(&self) -> WorkflowCertificationCapabilities;
    fn initialize_session(
        &self,
        plan: &WorkflowPlan<Self::Step>,
        profile: &WorkflowRuntimeProfile,
    ) -> Result<Self::Session, Self::Error>;
    fn execute_step(
        &self,
        session: &mut Self::Session,
        step: &WorkflowStep<Self::Step>,
        injection: Option<&FailureInjectionPoint>,
    ) -> Result<WorkflowStepOutcome, Self::Error>;
    fn create_checkpoint(
        &self,
        session: &mut Self::Session,
        checkpoint: &WorkflowCheckpoint,
    ) -> Result<(), Self::Error>;
    fn capture_artifacts(
        &self,
        session: &Self::Session,
        request: &WorkflowCaptureRequest,
    ) -> Result<Vec<ArtifactBundle>, Self::Error>;
    fn run_invariants(
        &self,
        session: &Self::Session,
        boundary: WorkflowState,
        checks: &[InvariantCheck],
    ) -> Result<Vec<InvariantReport>, Self::Error>;
    fn serialize_reproduction(
        &self,
        session: &Self::Session,
        failure: &WorkflowFailureContext,
    ) -> Result<ReproductionMetadata, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowCertificationError<AdapterError> {
    InvalidStateTransition {
        from: WorkflowState,
        to: WorkflowState,
    },
    Adapter(AdapterError),
}

impl<AdapterError: fmt::Display> fmt::Display for WorkflowCertificationError<AdapterError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStateTransition { from, to } => {
                write!(f, "invalid workflow transition: {from:?} -> {to:?}")
            }
            Self::Adapter(error) => write!(f, "{error}"),
        }
    }
}

impl<AdapterError: fmt::Debug + fmt::Display> std::error::Error
    for WorkflowCertificationError<AdapterError>
{
}
