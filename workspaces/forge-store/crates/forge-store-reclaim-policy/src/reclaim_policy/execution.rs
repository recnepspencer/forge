use super::{
    AdmittedReclaimPolicy, ReclaimPolicyExecutionObservation, ReclaimPolicyExecutionReceipt,
    ReclaimPolicyViolation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreOwnedReclaimPolicyExecution {
    _private: (),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReclaimPolicyExecutionRequest {
    policy: AdmittedReclaimPolicy,
}

pub trait PhysicalStoreReclaimPolicyExecutor {
    type Error;

    fn execute_reclaim_policy(
        &mut self,
        request: ReclaimPolicyExecutionRequest,
    ) -> Result<ReclaimPolicyExecutionObservation, Self::Error>;
}

pub struct ReclaimPolicyExecutionSession<'backend, Backend> {
    backend: &'backend mut Backend,
    authority: StoreOwnedReclaimPolicyExecution,
}

impl StoreOwnedReclaimPolicyExecution {
    #[allow(dead_code)]
    pub(crate) const fn store_owned() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn for_certification_test_authority() -> Self {
        Self { _private: () }
    }

    pub(crate) fn complete(
        self,
        policy: AdmittedReclaimPolicy,
        observation: ReclaimPolicyExecutionObservation,
    ) -> Result<ReclaimPolicyExecutionReceipt, ReclaimPolicyViolation> {
        policy.complete_execution_with_store_authority(observation)
    }
}

impl ReclaimPolicyExecutionRequest {
    pub(crate) const fn from_policy(policy: AdmittedReclaimPolicy) -> Self {
        Self { policy }
    }

    pub const fn policy(&self) -> &AdmittedReclaimPolicy {
        &self.policy
    }
}

impl<'backend, Backend> ReclaimPolicyExecutionSession<'backend, Backend> {
    pub fn for_store_backend(
        backend: &'backend mut Backend,
        authority: StoreOwnedReclaimPolicyExecution,
    ) -> Self {
        Self { backend, authority }
    }

    #[allow(dead_code)]
    pub(crate) fn for_owned_backend(backend: &'backend mut Backend) -> Self {
        Self::for_store_backend(backend, StoreOwnedReclaimPolicyExecution::store_owned())
    }

    pub fn execute(
        &mut self,
        policy: AdmittedReclaimPolicy,
    ) -> Result<Result<ReclaimPolicyExecutionReceipt, ReclaimPolicyViolation>, Backend::Error>
    where
        Backend: PhysicalStoreReclaimPolicyExecutor,
    {
        let request = ReclaimPolicyExecutionRequest::from_policy(policy.clone());
        let observation = self.backend.execute_reclaim_policy(request)?;
        Ok(self.authority.complete(policy, observation))
    }
}
