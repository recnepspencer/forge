use super::{
    AccessPolicyExecutionObservation, AccessPolicyExecutionReceipt, AccessPolicyRequest,
    AccessPolicyViolation, AdmittedAccessPolicy,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreOwnedAccessPolicyExecution {
    _private: (),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessPolicyExecutionRequest {
    policy: AdmittedAccessPolicy,
}

pub trait PhysicalStoreAccessPolicyExecutor {
    type Error;

    fn execute_access_policy(
        &mut self,
        request: AccessPolicyExecutionRequest,
    ) -> Result<AccessPolicyExecutionObservation, Self::Error>;
}

pub struct AccessPolicyExecutionSession<'backend, Backend> {
    backend: &'backend mut Backend,
    authority: StoreOwnedAccessPolicyExecution,
}

impl StoreOwnedAccessPolicyExecution {
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
        policy: AdmittedAccessPolicy,
        observation: AccessPolicyExecutionObservation,
    ) -> Result<AccessPolicyExecutionReceipt, AccessPolicyViolation> {
        policy.complete_execution_with_store_authority(observation)
    }
}

impl AccessPolicyExecutionRequest {
    pub(crate) const fn from_policy(policy: AdmittedAccessPolicy) -> Self {
        Self { policy }
    }

    pub const fn policy(&self) -> AdmittedAccessPolicy {
        self.policy
    }

    pub const fn access_request(&self) -> AccessPolicyRequest {
        self.policy.request()
    }
}

impl<'backend, Backend> AccessPolicyExecutionSession<'backend, Backend> {
    pub fn for_store_backend(
        backend: &'backend mut Backend,
        authority: StoreOwnedAccessPolicyExecution,
    ) -> Self {
        Self { backend, authority }
    }

    #[allow(dead_code)]
    pub(crate) fn for_owned_backend(backend: &'backend mut Backend) -> Self {
        Self::for_store_backend(backend, StoreOwnedAccessPolicyExecution::store_owned())
    }

    pub fn execute(
        &mut self,
        policy: AdmittedAccessPolicy,
    ) -> Result<Result<AccessPolicyExecutionReceipt, AccessPolicyViolation>, Backend::Error>
    where
        Backend: PhysicalStoreAccessPolicyExecutor,
    {
        let request = AccessPolicyExecutionRequest::from_policy(policy);
        let observation = self.backend.execute_access_policy(request)?;
        Ok(self.authority.complete(policy, observation))
    }
}
