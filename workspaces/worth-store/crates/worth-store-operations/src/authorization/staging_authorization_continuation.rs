use worth_store_physical_backend::NonCurrentStagingBoundary;

use super::{
    AuthorizationConsumptionReceipt, AuthorizationProviderFailure,
    AuthorizationRevocationObservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StagingAuthorizationContinuationRequest {
    authorization_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    boundary: NonCurrentStagingBoundary,
}

impl StagingAuthorizationContinuationRequest {
    pub const fn authorization_identity(self) -> [u8; 32] {
        self.authorization_identity
    }
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn boundary(self) -> NonCurrentStagingBoundary {
        self.boundary
    }
}

pub trait StagingAuthorizationContinuationPort {
    fn observe_revocation(
        &self,
        request: StagingAuthorizationContinuationRequest,
    ) -> Result<AuthorizationRevocationObservation, AuthorizationProviderFailure>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagingAuthorizationContinuationDenial {
    Provider {
        boundary: NonCurrentStagingBoundary,
        failure: AuthorizationProviderFailure,
    },
    Expired {
        boundary: NonCurrentStagingBoundary,
    },
    Revoked {
        boundary: NonCurrentStagingBoundary,
    },
    ObservationRegressed {
        boundary: NonCurrentStagingBoundary,
    },
}

pub(crate) struct StagingAuthorizationContinuation<'a, P> {
    receipt: AuthorizationConsumptionReceipt,
    port: &'a P,
    last_observed_at: Option<u64>,
    denial: Option<StagingAuthorizationContinuationDenial>,
}

impl<'a, P: StagingAuthorizationContinuationPort> StagingAuthorizationContinuation<'a, P> {
    pub(crate) const fn new(receipt: AuthorizationConsumptionReceipt, port: &'a P) -> Self {
        Self {
            receipt,
            port,
            last_observed_at: None,
            denial: None,
        }
    }

    pub(crate) fn admit(&mut self, boundary: NonCurrentStagingBoundary) -> bool {
        if self.denial.is_some() {
            return false;
        }
        let request = StagingAuthorizationContinuationRequest {
            authorization_identity: self.receipt.authorization_identity(),
            plan_fingerprint: self.receipt.plan_fingerprint(),
            boundary,
        };
        let result = self
            .port
            .observe_revocation(request)
            .map_err(|failure| StagingAuthorizationContinuationDenial::Provider {
                boundary,
                failure,
            })
            .and_then(|observation| self.validate(boundary, observation));
        if let Err(denial) = result {
            self.denial = Some(denial);
            false
        } else {
            true
        }
    }

    pub(crate) const fn denial(&self) -> Option<StagingAuthorizationContinuationDenial> {
        self.denial
    }

    fn validate(
        &mut self,
        boundary: NonCurrentStagingBoundary,
        observation: AuthorizationRevocationObservation,
    ) -> Result<(), StagingAuthorizationContinuationDenial> {
        let observed_at = match observation {
            AuthorizationRevocationObservation::NotRevoked { observed_at } => observed_at,
            AuthorizationRevocationObservation::Revoked { .. } => {
                return Err(StagingAuthorizationContinuationDenial::Revoked { boundary })
            }
            AuthorizationRevocationObservation::Unavailable { .. } => {
                return Err(StagingAuthorizationContinuationDenial::Provider {
                    boundary,
                    failure: AuthorizationProviderFailure::Unavailable,
                })
            }
        };
        if observed_at > self.receipt.expires_at() {
            return Err(StagingAuthorizationContinuationDenial::Expired { boundary });
        }
        if self
            .last_observed_at
            .is_some_and(|previous| observed_at < previous)
        {
            return Err(StagingAuthorizationContinuationDenial::ObservationRegressed { boundary });
        }
        self.last_observed_at = Some(observed_at);
        Ok(())
    }
}
