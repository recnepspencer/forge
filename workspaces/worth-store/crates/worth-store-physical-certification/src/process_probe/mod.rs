mod artifact;
mod declaration;
mod execution;
mod identity;
#[cfg(test)]
mod tests;

pub use artifact::{
    ProcessArtifactDisposition, ProcessArtifactObservation, ProcessArtifactPath,
};
pub use declaration::{
    ProcessIsolationRequirement, ProcessProbeDeclaration, ProcessRole,
    ProcessTerminationRequirement, SealedProcessProbeInput,
};
pub use execution::{
    ProcessProbeEvidenceDenial, ProcessProbeExecution, ProcessTermination,
    PROCESS_PROBE_EVIDENCE_ROOT_ENV,
};
pub use identity::{
    admit_current_process_probe, AdmittedProcessProbe, ProcessEnvironmentBindingEvidence,
    ProcessIdentityEvidence,
};

pub(crate) use execution::{classify_exit, persist_execution};
pub(crate) use identity::{
    configure_process_probe, read_process_observation, write_current_process_observation,
};
