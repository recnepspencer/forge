use bank_domain::model::BankPrincipalId;
use bank_server::BankAuthenticatedPrincipal;
use worth_query_host::facade::declaration::authentication::WorthQueryExternalPrincipalIdentity;

#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) struct BankHttpAuthenticatedOwner {
    principal: BankPrincipalId,
    external: WorthQueryExternalPrincipalIdentity,
}

impl BankHttpAuthenticatedOwner {
    pub(super) fn from_principal(principal: &BankAuthenticatedPrincipal) -> Self {
        Self {
            principal: principal.principal_id(),
            external: principal.external_identity().clone(),
        }
    }
}
