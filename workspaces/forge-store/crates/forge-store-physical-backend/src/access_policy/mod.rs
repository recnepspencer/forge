mod admission;
mod alignment;
mod coherence;
mod counters;
mod denial;
mod execution;
mod fault_posture;
mod mode;
mod page_cache;
mod proof_authority;
mod receipt;
mod request;
mod security_scope;
mod violation;

pub use admission::AccessPolicyAdmission;
pub use alignment::DirectIoAlignmentRequirement;
pub use coherence::{MixedAccessCoherenceBasis, MixedAccessTransition};
pub use counters::{AccessPolicyCounterSnapshot, AccessPolicyCounterStrength};
pub use denial::{AccessPolicyDenial, AccessPolicyDenialKind};
pub use execution::{
    AccessPolicyExecutionRequest, AccessPolicyExecutionSession, PhysicalStoreAccessPolicyExecutor,
    StoreOwnedAccessPolicyExecution,
};
pub use fault_posture::{
    MmapFaultHandling, MmapFaultPosture, MmapPunchHolePosture, MmapTruncatePosture,
    MmapVisibilityPosture, MmapWritebackPosture,
};
pub use forge_store_buffer_pool::{AccessPolicyBufferLifecycle, AccessPolicyBufferLifecycleKind};
pub use mode::{StoreAccessMode, StoreAccessOperation};
pub use page_cache::{PageCachePolicyKind, PageCachePolicyProof};
pub use proof_authority::StoreAccessPolicyProofAuthority;
pub use receipt::{AccessPolicyExecutionReceipt, AdmittedAccessPolicy};
pub use request::AccessPolicyRequest;
pub use security_scope::AccessPolicySecurityScope;
pub use violation::{
    AccessPolicyExecutionObservation, AccessPolicyViolation, AccessPolicyViolationKind,
};

#[cfg(test)]
mod dirty_mmap_lifecycle_tests;
#[cfg(test)]
mod execution_observation_tests;
#[cfg(test)]
mod policy_parity_tests;
#[cfg(test)]
mod proof_authority_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
