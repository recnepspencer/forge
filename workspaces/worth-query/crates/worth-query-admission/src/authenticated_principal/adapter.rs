use std::future::Future;
use std::pin::Pin;

use super::{
    WorthQueryAuthenticationAdapterFailure, WorthQueryRequestScope,
    WorthQueryValidatedExternalPrincipal,
};

pub type WorthQueryAuthenticationFuture<'a> = Pin<
    Box<
        dyn Future<
                Output = Result<
                    WorthQueryValidatedExternalPrincipal,
                    WorthQueryAuthenticationAdapterFailure,
                >,
            > + Send
            + 'a,
    >,
>;

pub trait WorthQueryAuthenticationAdapter: Send + Sync + 'static {
    type Credential: Send + 'static;

    fn configuration_identity(&self) -> &str;

    fn validate<'a>(
        &'a self,
        credential: Self::Credential,
        scope: &'a WorthQueryRequestScope,
    ) -> WorthQueryAuthenticationFuture<'a>;
}
