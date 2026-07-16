use std::marker::PhantomData;

use worth_proof::prelude::*;
use worth_proof::{Admitted, AssumptionBasis, CurrentValidity, FreshnessScopedBasis, Lowered};

use crate::owner_plan_dag::OperationalPlanBinding;

use super::{
    AuthorizationDenial, AuthorizationProviderDecision, AuthorizationRevocationObservation,
    ExternalOperatorAssertion, OperationalAuthorizationPort, OperationalAuthorizationRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationReplayPolicy {
    SingleUse,
    ReplaySameOperationIdentity,
}

#[derive(Debug, Clone)]
pub(crate) struct LoweredOperationalPlan<K> {
    proof: Recipe<Lowered, OperationalPlanBinding, CurrentPlanBasis>,
    operation: PhantomData<K>,
}

#[derive(Debug, Clone)]
pub(crate) struct AuthorizedOperationalPlan<K> {
    proof: Recipe<Admitted, OperationalPlanBinding, CurrentPlanBasis>,
    authorization_identity: [u8; 32],
    assertion_identity: [u8; 32],
    issued_at: u64,
    expires_at: u64,
    replay_policy: AuthorizationReplayPolicy,
    operation: PhantomData<K>,
}

type CurrentPlanBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<()>>;

impl<K> LoweredOperationalPlan<K> {
    pub(crate) fn from_binding(binding: OperationalPlanBinding) -> Self {
        let lowered = recipe(binding)
            .resolve_with(private_resolution_authority(), ())
            .lower_with(private_lowering_capability());
        Self {
            proof: lowered,
            operation: PhantomData,
        }
    }

    pub(crate) fn binding(&self) -> &OperationalPlanBinding {
        self.proof.payload()
    }
}

pub(crate) fn authorize_lowered_plan<K>(
    lowered: LoweredOperationalPlan<K>,
    port: &impl OperationalAuthorizationPort,
    assertion: &ExternalOperatorAssertion,
    requested_at: u64,
    expires_at: u64,
    replay_policy: AuthorizationReplayPolicy,
    revocation: AuthorizationRevocationObservation,
) -> Result<AuthorizedOperationalPlan<K>, AuthorizationDenial> {
    validate_request_time(assertion, requested_at, expires_at, revocation)?;
    let request = OperationalAuthorizationRequest::new(
        lowered.binding(),
        requested_at,
        expires_at,
        replay_policy,
    );
    let decision = port
        .authorize(request, assertion)
        .map_err(AuthorizationDenial::Provider)?;
    let (authorization_identity, decision_issued_at, decision_expires_at) =
        validate_provider_decision(&lowered, assertion, requested_at, expires_at, decision)?;
    let admitted = lowered.proof.admit_with(private_admission_authority());
    Ok(AuthorizedOperationalPlan {
        proof: admitted,
        authorization_identity,
        assertion_identity: assertion.assertion_identity(),
        issued_at: decision_issued_at,
        expires_at: decision_expires_at,
        replay_policy,
        operation: PhantomData,
    })
}

impl<K> AuthorizedOperationalPlan<K> {
    pub(crate) fn binding(&self) -> &OperationalPlanBinding {
        self.proof.payload()
    }
    pub(crate) const fn authorization_identity(&self) -> [u8; 32] {
        self.authorization_identity
    }
    pub(crate) const fn assertion_identity(&self) -> [u8; 32] {
        self.assertion_identity
    }
    pub(crate) const fn issued_at(&self) -> u64 {
        self.issued_at
    }
    pub(crate) const fn expires_at(&self) -> u64 {
        self.expires_at
    }
    pub(crate) const fn replay_policy(&self) -> AuthorizationReplayPolicy {
        self.replay_policy
    }
}

fn validate_request_time(
    assertion: &ExternalOperatorAssertion,
    requested_at: u64,
    expires_at: u64,
    revocation: AuthorizationRevocationObservation,
) -> Result<(), AuthorizationDenial> {
    if requested_at >= expires_at || expires_at > assertion.expires_at() {
        return Err(AuthorizationDenial::InvalidValidityWindow);
    }
    if requested_at < assertion.issued_at() || requested_at > assertion.expires_at() {
        return Err(AuthorizationDenial::AssertionExpired);
    }
    match revocation {
        AuthorizationRevocationObservation::NotRevoked { observed_at }
            if observed_at >= requested_at =>
        {
            Ok(())
        }
        AuthorizationRevocationObservation::Revoked { .. } => {
            Err(AuthorizationDenial::AuthorizationRevoked)
        }
        _ => Err(AuthorizationDenial::Provider(
            super::AuthorizationProviderFailure::Unavailable,
        )),
    }
}

fn validate_provider_decision<K>(
    lowered: &LoweredOperationalPlan<K>,
    assertion: &ExternalOperatorAssertion,
    requested_at: u64,
    requested_expiry: u64,
    decision: AuthorizationProviderDecision,
) -> Result<([u8; 32], u64, u64), AuthorizationDenial> {
    match decision {
        AuthorizationProviderDecision::Denied { reason_code } => {
            Err(AuthorizationDenial::ProviderDenied { reason_code })
        }
        AuthorizationProviderDecision::Authorized {
            authorization_identity,
            plan_fingerprint,
            proof_of_possession_binding,
            issued_at,
            expires_at,
        } => {
            if authorization_identity == [0; 32]
                || issued_at > requested_at
                || expires_at < requested_expiry
                || issued_at >= expires_at
            {
                return Err(AuthorizationDenial::InvalidProviderDecision);
            }
            if plan_fingerprint != lowered.binding().fingerprint() {
                return Err(AuthorizationDenial::PlanBindingMismatch);
            }
            if proof_of_possession_binding != assertion.proof_of_possession_binding() {
                return Err(AuthorizationDenial::Provider(
                    super::AuthorizationProviderFailure::InvalidProofOfPossession,
                ));
            }
            Ok((authorization_identity, issued_at, expires_at))
        }
    }
}

#[derive(Clone, Copy)]
struct PrivateResolutionAuthority;
impl worth_proof::raw::AuthorityMarker for PrivateResolutionAuthority {}
#[derive(Clone, Copy)]
struct PrivateLoweringCapability;
impl worth_proof::raw::CapabilityMarker for PrivateLoweringCapability {}
#[derive(Clone, Copy)]
struct PrivateAdmissionAuthority;
impl worth_proof::raw::AuthorityMarker for PrivateAdmissionAuthority {}

fn private_resolution_authority() -> worth_proof::raw::AuthorityWitness<PrivateResolutionAuthority>
{
    AuthorityWitness::from_authority_marker(PrivateResolutionAuthority)
}
fn private_lowering_capability() -> worth_proof::raw::CapabilityWitness<PrivateLoweringCapability> {
    CapabilityWitness::from_capability_marker(PrivateLoweringCapability)
}
fn private_admission_authority() -> worth_proof::raw::AuthorityWitness<PrivateAdmissionAuthority> {
    AuthorityWitness::from_authority_marker(PrivateAdmissionAuthority)
}
