use sha2::{Digest, Sha256};

use crate::owner_plan_dag::OperationalPlanBinding;

use super::{AuthorizationDenial, AuthorizationProviderFailure, AuthorizationReplayPolicy};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalOperatorAssertion {
    provider: String,
    assertion_type: String,
    assertion_identity: [u8; 32],
    proof_of_possession_binding: [u8; 32],
    issued_at: u64,
    expires_at: u64,
}

impl ExternalOperatorAssertion {
    pub fn admit(
        provider: impl Into<String>,
        assertion_type: impl Into<String>,
        assertion_bytes: &[u8],
        proof_of_possession_binding: [u8; 32],
        issued_at: u64,
        expires_at: u64,
    ) -> Result<Self, AuthorizationDenial> {
        let provider = provider.into();
        let assertion_type = assertion_type.into();
        if provider.trim().is_empty()
            || assertion_type.trim().is_empty()
            || assertion_bytes.is_empty()
            || proof_of_possession_binding == [0; 32]
            || issued_at >= expires_at
        {
            return Err(AuthorizationDenial::InvalidValidityWindow);
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-external-operator-assertion-v1");
        digest.update((provider.len() as u64).to_be_bytes());
        digest.update(provider.as_bytes());
        digest.update((assertion_type.len() as u64).to_be_bytes());
        digest.update(assertion_type.as_bytes());
        digest.update((assertion_bytes.len() as u64).to_be_bytes());
        digest.update(assertion_bytes);
        digest.update(proof_of_possession_binding);
        Ok(Self {
            provider,
            assertion_type,
            assertion_identity: digest.finalize().into(),
            proof_of_possession_binding,
            issued_at,
            expires_at,
        })
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }
    pub fn assertion_type(&self) -> &str {
        &self.assertion_type
    }
    pub const fn assertion_identity(&self) -> [u8; 32] {
        self.assertion_identity
    }
    pub const fn proof_of_possession_binding(&self) -> [u8; 32] {
        self.proof_of_possession_binding
    }
    pub const fn issued_at(&self) -> u64 {
        self.issued_at
    }
    pub const fn expires_at(&self) -> u64 {
        self.expires_at
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperationalAuthorizationRequest<'a> {
    plan: &'a OperationalPlanBinding,
    requested_at: u64,
    expires_at: u64,
    replay_policy: AuthorizationReplayPolicy,
}

impl<'a> OperationalAuthorizationRequest<'a> {
    pub(crate) const fn new(
        plan: &'a OperationalPlanBinding,
        requested_at: u64,
        expires_at: u64,
        replay_policy: AuthorizationReplayPolicy,
    ) -> Self {
        Self {
            plan,
            requested_at,
            expires_at,
            replay_policy,
        }
    }

    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan.fingerprint()
    }
    pub const fn authority_identity_fingerprint(self) -> [u8; 32] {
        self.plan.authority_identity().fingerprint()
    }
    pub const fn source_identity(self) -> [u8; 32] {
        self.plan.source_identity()
    }
    pub const fn target_identity(self) -> [u8; 32] {
        self.plan.target_identity()
    }
    pub const fn frontier_identity(self) -> [u8; 32] {
        self.plan.frontier_identity()
    }
    pub const fn requested_at(self) -> u64 {
        self.requested_at
    }
    pub const fn expires_at(self) -> u64 {
        self.expires_at
    }
    pub const fn replay_policy(self) -> AuthorizationReplayPolicy {
        self.replay_policy
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationProviderDecision {
    Authorized {
        authorization_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        proof_of_possession_binding: [u8; 32],
        issued_at: u64,
        expires_at: u64,
    },
    Denied {
        reason_code: String,
    },
}

impl AuthorizationProviderDecision {
    pub const fn authorized(
        authorization_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        proof_of_possession_binding: [u8; 32],
        issued_at: u64,
        expires_at: u64,
    ) -> Self {
        Self::Authorized {
            authorization_identity,
            plan_fingerprint,
            proof_of_possession_binding,
            issued_at,
            expires_at,
        }
    }

    pub fn denied(reason_code: impl Into<String>) -> Self {
        Self::Denied {
            reason_code: reason_code.into(),
        }
    }
}

pub trait OperationalAuthorizationPort {
    fn authorize(
        &self,
        request: OperationalAuthorizationRequest<'_>,
        assertion: &ExternalOperatorAssertion,
    ) -> Result<AuthorizationProviderDecision, AuthorizationProviderFailure>;
}
