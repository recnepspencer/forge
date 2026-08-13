use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationIdempotencyResolutionDenial,
    WorthQueryApplicationIdempotencyResolutionDenialKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankEstateIdempotencyResolutionDenial {
    Authorization(crate::BankAuthorizationDenial),
    AuthorizationLineageUnavailable,
    ForeignAdmission,
    ProviderUnavailable,
}

pub(super) fn from_query(
    denial: WorthQueryApplicationIdempotencyResolutionDenial,
) -> BankEstateIdempotencyResolutionDenial {
    match denial.kind() {
        WorthQueryApplicationIdempotencyResolutionDenialKind::Authorization => denial
            .authorization()
            .cloned()
            .map(crate::BankAuthorizationDenial::from_query)
            .map(BankEstateIdempotencyResolutionDenial::Authorization)
            .unwrap_or(BankEstateIdempotencyResolutionDenial::AuthorizationLineageUnavailable),
        WorthQueryApplicationIdempotencyResolutionDenialKind::ForeignAdmission => {
            BankEstateIdempotencyResolutionDenial::ForeignAdmission
        }
        WorthQueryApplicationIdempotencyResolutionDenialKind::ProviderUnavailable => {
            BankEstateIdempotencyResolutionDenial::ProviderUnavailable
        }
    }
}
