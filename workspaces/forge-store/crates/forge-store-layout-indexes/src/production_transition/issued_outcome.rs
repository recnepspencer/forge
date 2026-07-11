use super::S8LayoutProductionTransition;
use forge_proof::{AuthorityMarker, AuthorityWitness};

#[derive(Debug, PartialEq, Eq)]
struct OwnerOutcomeIssuanceAuthority;
impl AuthorityMarker for OwnerOutcomeIssuanceAuthority {}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct S8OwnerIssuedCase<T> {
    _authority: AuthorityWitness<OwnerOutcomeIssuanceAuthority>,
    payload: T,
    transition: S8LayoutProductionTransition,
}

impl<T> S8OwnerIssuedCase<T> {
    pub(crate) fn issue(payload: T, transition: S8LayoutProductionTransition) -> Self {
        let issued = Self {
            _authority: AuthorityWitness::from_authority_marker(OwnerOutcomeIssuanceAuthority),
            payload,
            transition,
        };
        #[cfg(test)]
        super::observation::record(transition);
        issued
    }

    pub(crate) const fn payload(&self) -> &T {
        &self.payload
    }
    pub(crate) fn into_payload(self) -> T {
        self.payload
    }
    pub(crate) const fn transition(&self) -> S8LayoutProductionTransition {
        self.transition
    }
}

/// Internal core for an opaque, owner-issued `Result`-shaped outcome.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct S8OwnerIssuedResult<T, D> {
    _authority: AuthorityWitness<OwnerOutcomeIssuanceAuthority>,
    result: Result<T, D>,
    transition: S8LayoutProductionTransition,
}

impl<T, D> S8OwnerIssuedResult<T, D> {
    pub(crate) fn admitted(value: T, transition: S8LayoutProductionTransition) -> Self {
        Self {
            _authority: AuthorityWitness::from_authority_marker(OwnerOutcomeIssuanceAuthority),
            result: Ok(value),
            transition,
        }
    }
    pub(crate) fn denied(denial: D, transition: S8LayoutProductionTransition) -> Self {
        Self {
            _authority: AuthorityWitness::from_authority_marker(OwnerOutcomeIssuanceAuthority),
            result: Err(denial),
            transition,
        }
    }
    pub(crate) const fn transition(&self) -> S8LayoutProductionTransition {
        self.transition
    }
    pub(crate) const fn result(&self) -> Result<&T, &D> {
        self.result.as_ref()
    }
    pub(crate) fn into_result(self) -> Result<T, D> {
        self.result
    }
}
