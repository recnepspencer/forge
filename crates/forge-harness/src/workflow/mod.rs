mod adapter;
mod capability;
mod record;
mod runner;

pub use adapter::{WorkflowCertificationAdapter, WorkflowCertificationError};
pub use capability::{
    DifferentialMatrixCapability, ProfileConditionalGuarantee, UnsupportedWorkflowComparison,
    WorkflowArtifactSurfaceCapability, WorkflowCertificationCapabilities,
};
pub use record::{
    ArtifactBundle, ArtifactClass, ArtifactSurface, CheckpointSemantics, DifferentialComparison,
    DifferentialOutcome, FailureBundle, FailureBundleVersion, FailureInjectionPoint,
    InvariantCheck, InvariantReport, RegressionTarget, RegressionTargetKind,
    ReproductionMetadata, WorkflowCaptureRequest, WorkflowCertificationReport,
    WorkflowCheckpoint, WorkflowCheckpointTraceEntry, WorkflowFailureContext, WorkflowPlan,
    WorkflowRuntimeProfile, WorkflowSession, WorkflowState, WorkflowStep, WorkflowStepOutcome,
    WorkflowStepTraceEntry,
};
pub use runner::WorkflowCertificationRunner;

#[cfg(test)]
mod tests;
