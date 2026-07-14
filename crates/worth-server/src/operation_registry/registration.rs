use crate::{
    WorthServerOperationAuthorityDeclaration, WorthServerOperationAuthorizationPolicy,
    WorthServerOperationFamily, WorthServerSharedReadBasisKind, WorthServerSurfaceFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationRegistration {
    family: WorthServerOperationFamily,
    enabled: bool,
    exposed_surfaces: Vec<WorthServerSurfaceFamily>,
    admitted_operation_names: Vec<String>,
    authority_declaration: Option<WorthServerOperationAuthorityDeclaration>,
    authorization_policy: Option<WorthServerOperationAuthorizationPolicy>,
}

impl WorthServerOperationRegistration {
    pub fn enabled(family: WorthServerOperationFamily) -> Self {
        Self {
            family,
            enabled: true,
            exposed_surfaces: Vec::new(),
            admitted_operation_names: Vec::new(),
            authority_declaration: None,
            authorization_policy: None,
        }
    }

    pub fn disabled(family: WorthServerOperationFamily) -> Self {
        Self {
            family,
            enabled: false,
            exposed_surfaces: Vec::new(),
            admitted_operation_names: Vec::new(),
            authority_declaration: None,
            authorization_policy: None,
        }
    }

    pub fn exposed_on(
        mut self,
        surfaces: impl IntoIterator<Item = WorthServerSurfaceFamily>,
    ) -> Self {
        self.exposed_surfaces = surfaces.into_iter().collect();
        self.exposed_surfaces.sort();
        self.exposed_surfaces.dedup();
        self
    }

    pub fn admit_operation_names(
        mut self,
        operation_names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.admitted_operation_names = operation_names
            .into_iter()
            .map(|value| value.into().trim().to_ascii_lowercase())
            .collect();
        self.admitted_operation_names.sort();
        self.admitted_operation_names.dedup();
        self
    }

    pub fn with_authority_declaration(
        mut self,
        declaration: WorthServerOperationAuthorityDeclaration,
    ) -> Self {
        self.authority_declaration = Some(declaration);
        self
    }

    pub fn with_authorization_policy(
        mut self,
        policy: WorthServerOperationAuthorizationPolicy,
    ) -> Self {
        self.authorization_policy = Some(policy);
        self
    }

    pub fn family(&self) -> WorthServerOperationFamily {
        self.family
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn exposed_surfaces(&self) -> &[WorthServerSurfaceFamily] {
        &self.exposed_surfaces
    }

    pub fn admitted_operation_names(&self) -> &[String] {
        &self.admitted_operation_names
    }

    pub fn authority_declaration(&self) -> Option<&WorthServerOperationAuthorityDeclaration> {
        self.authority_declaration.as_ref()
    }

    pub fn authorization_policy(&self) -> Option<&WorthServerOperationAuthorizationPolicy> {
        self.authorization_policy.as_ref()
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if let Some(declaration) = &self.authority_declaration {
            declaration.validate_for_family(self.family)?;
        }
        if let Some(policy) = &self.authorization_policy {
            policy.validate()?;
        }
        Ok(())
    }

    pub fn phase_two_defaults() -> Vec<Self> {
        use crate::WorthServerOperationFamily::{
            BinaryTransfer, ProductApplicationMutation, ProductApplicationRead,
            ProductSessionCoordination, QueryDirectProjection, QueryDirectRead,
            QueryDirectSubmission, SyncLease,
        };
        use crate::WorthServerSurfaceFamily::{CompatHttp, Sync, WorthNative};

        vec![
            Self::enabled(QueryDirectRead)
                .exposed_on([WorthNative, CompatHttp])
                .with_authority_declaration(
                    WorthServerOperationAuthorityDeclaration::query_shared_read(),
                )
                .with_authorization_policy(
                    WorthServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(QueryDirectSubmission)
                .exposed_on([WorthNative, CompatHttp])
                .with_authority_declaration(
                    WorthServerOperationAuthorityDeclaration::deterministic_submission(
                        "query-write",
                        "query-write-review",
                        "derive-from-request",
                        "derive-from-request",
                    ),
                )
                .with_authorization_policy(
                    WorthServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(QueryDirectProjection)
                .exposed_on([WorthNative])
                .with_authority_declaration(
                    WorthServerOperationAuthorityDeclaration::query_shared_read(),
                )
                .with_authorization_policy(
                    WorthServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(ProductApplicationRead)
                .exposed_on([WorthNative, CompatHttp])
                .with_authority_declaration(
                    WorthServerOperationAuthorityDeclaration::product_shared_read(
                        WorthServerSharedReadBasisKind::QueryDerived,
                    ),
                )
                .with_authorization_policy(
                    WorthServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(ProductApplicationMutation)
                .exposed_on([WorthNative, CompatHttp])
                .with_authority_declaration(
                    WorthServerOperationAuthorityDeclaration::product_draft_mutation(
                        "product-draft",
                        "derive-from-request",
                        "derive-from-request",
                    ),
                )
                .with_authorization_policy(
                    WorthServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(ProductSessionCoordination)
                .exposed_on([WorthNative, CompatHttp])
                .with_authority_declaration(
                    WorthServerOperationAuthorityDeclaration::product_session_coordination(
                        "product-session",
                    ),
                )
                .with_authorization_policy(
                    WorthServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(BinaryTransfer)
                .exposed_on([CompatHttp])
                .with_authority_declaration(
                    WorthServerOperationAuthorityDeclaration::binary_streaming(
                        "binary-transfer",
                        "preflight-required",
                        "declared-size",
                        "cancellable",
                        "partial-failure-surfaced",
                    ),
                )
                .with_authorization_policy(
                    WorthServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(SyncLease)
                .exposed_on([WorthNative, Sync])
                .with_authority_declaration(
                    WorthServerOperationAuthorityDeclaration::lease_coordination(
                        "query-downstream-delivery",
                    ),
                )
                .with_authorization_policy(
                    WorthServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
        ]
    }
}
