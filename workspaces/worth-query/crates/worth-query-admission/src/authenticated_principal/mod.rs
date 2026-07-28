mod adapter;
mod admission;
mod candidate;
mod denial;
mod proof;
mod request_scope;
mod vocabulary;

pub use adapter::{WorthQueryAuthenticationAdapter, WorthQueryAuthenticationFuture};
pub use admission::{
    admit_authentication_adapter, WorthQueryAdmittedAuthenticationAdapter,
    WorthQueryAuthenticationAdapterAdmission, WorthQueryAuthenticationAdapterAdmissionDenial,
};
pub use candidate::{
    WorthQueryPrincipalAttribute, WorthQueryValidatedExternalPrincipal,
    WorthQueryValidatedExternalPrincipalDenial,
};
pub use denial::{
    WorthQueryAuthenticationAdapterFailure, WorthQueryAuthenticationAdapterFailureKind,
    WorthQueryAuthenticationDenial, WorthQueryAuthenticationDenialKind,
};
pub use proof::WorthQueryAuthenticatedExternalPrincipal;
pub use request_scope::{
    WorthQueryCancellationSource, WorthQueryCancellationToken, WorthQueryRequestInterruption,
    WorthQueryRequestScope,
};
pub use vocabulary::{
    WorthQueryAuthenticationAudience, WorthQueryAuthenticationMethod,
    WorthQueryAuthenticationVocabularyDenial,
};

#[cfg(test)]
mod tests;
