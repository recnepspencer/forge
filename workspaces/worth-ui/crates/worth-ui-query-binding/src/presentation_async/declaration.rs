use super::WorthUiPresentationRequestBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationAsyncDeclaration {
    basis: WorthUiPresentationRequestBasis,
    request_identity: worth_query::facade::foundation::WorthQueryAsyncResourceRequestIdentity,
    clause: worth_query::facade::foundation::WorthQueryAsyncDeclarationClause,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiPresentationAsyncDeclarationDenial {
    Identity(worth_query::facade::foundation::WorthQueryAsyncResourceRequestIdentityError),
}

impl WorthUiPresentationAsyncDeclaration {
    pub fn declare(
        basis: &WorthUiPresentationRequestBasis,
    ) -> Result<Self, WorthUiPresentationAsyncDeclarationDenial> {
        use worth_query::facade::foundation::{
            WorthQueryAsyncDeclarationClause, WorthQueryAsyncFailurePosture,
            WorthQueryAsyncLoadingPosture, WorthQueryAsyncResourceRequestIdentity,
            WorthQueryAsyncSourceFamily,
        };
        let request_identity = WorthQueryAsyncResourceRequestIdentity::declare(
            WorthQueryAsyncSourceFamily::HostResource,
            WorthQueryAsyncLoadingPosture::Blocking,
            WorthQueryAsyncFailurePosture::RetainStaleValue,
            basis.identity_parts(),
        )
        .map_err(WorthUiPresentationAsyncDeclarationDenial::Identity)?;
        let clause = WorthQueryAsyncDeclarationClause::resource_request(
            WorthQueryAsyncSourceFamily::HostResource,
            WorthQueryAsyncLoadingPosture::Blocking,
            WorthQueryAsyncFailurePosture::RetainStaleValue,
            request_identity.request_identity().to_vec(),
        );
        Ok(Self {
            basis: basis.clone(),
            request_identity,
            clause,
        })
    }

    pub fn request_identity(
        &self,
    ) -> &worth_query::facade::foundation::WorthQueryAsyncResourceRequestIdentity {
        &self.request_identity
    }

    pub fn basis(&self) -> &WorthUiPresentationRequestBasis {
        &self.basis
    }

    pub fn clause(&self) -> &worth_query::facade::foundation::WorthQueryAsyncDeclarationClause {
        &self.clause
    }
}
