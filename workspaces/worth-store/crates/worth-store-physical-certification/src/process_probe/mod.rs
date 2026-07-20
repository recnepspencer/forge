mod artifact;
mod declaration;
mod environment;
mod execution;
mod identity;
#[cfg(test)]
mod tests;
mod wire_encoding;

pub use artifact::{ProcessArtifactDisposition, ProcessArtifactObservation, ProcessArtifactPath};
pub use declaration::{
    ProcessIsolationRequirement, ProcessProbeDeclaration, ProcessProbeIntent, ProcessRole,
    ProcessTerminationRequirement, SealedProcessProbeInput,
};
pub use environment::ProcessEnvironmentBindingEvidence;
pub use execution::{
    ProcessProbeEvidenceDenial, ProcessProbeExecution, ProcessTermination,
    PROCESS_PROBE_EVIDENCE_ROOT_ENV,
};
pub use identity::{admit_current_process_probe, AdmittedProcessProbe, ProcessIdentityEvidence};

pub(crate) use execution::{observe_graceful_exit, persist_execution, terminate_by_parent};
pub(crate) use identity::{
    configure_process_probe, read_process_observation, write_current_process_observation,
};
