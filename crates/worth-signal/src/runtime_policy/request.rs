use super::definition::SignalRuntimePolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalRuntimePolicyRequest {
    policy: SignalRuntimePolicy,
}

impl SignalRuntimePolicyRequest {
    pub const fn new(policy: SignalRuntimePolicy) -> Self {
        Self { policy }
    }

    pub const fn policy(&self) -> SignalRuntimePolicy {
        self.policy
    }
}
