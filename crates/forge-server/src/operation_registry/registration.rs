use crate::{
    ForgeServerOperationAuthorityDeclaration, ForgeServerOperationAuthorizationPolicy,
    ForgeServerOperationFamily, ForgeServerSharedReadBasisKind, ForgeServerSurfaceFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationRegistration {
    family: ForgeServerOperationFamily,
    enabled: bool,
    exposed_surfaces: Vec<ForgeServerSurfaceFamily>,
    admitted_operation_names: Vec<String>,
    authority_declaration: Option<ForgeServerOperationAuthorityDeclaration>,
    authorization_policy: Option<ForgeServerOperationAuthorizationPolicy>,
}

impl ForgeServerOperationRegistration {
    pub fn enabled(family: ForgeServerOperationFamily) -> Self {
        Self {
            family,
            enabled: true,
            exposed_surfaces: Vec::new(),
            admitted_operation_names: Vec::new(),
            authority_declaration: None,
            authorization_policy: None,
        }
    }

    pub fn disabled(family: ForgeServerOperationFamily) -> Self {
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
        surfaces: impl IntoIterator<Item = ForgeServerSurfaceFamily>,
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
        declaration: ForgeServerOperationAuthorityDeclaration,
    ) -> Self {
        self.authority_declaration = Some(declaration);
        self
    }

    pub fn with_authorization_policy(
        mut self,
        policy: ForgeServerOperationAuthorizationPolicy,
    ) -> Self {
        self.authorization_policy = Some(policy);
        self
    }

    pub fn family(&self) -> ForgeServerOperationFamily {
        self.family
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn exposed_surfaces(&self) -> &[ForgeServerSurfaceFamily] {
        &self.exposed_surfaces
    }

    pub fn admitted_operation_names(&self) -> &[String] {
        &self.admitted_operation_names
    }

    pub fn authority_declaration(&self) -> Option<&ForgeServerOperationAuthorityDeclaration> {
        self.authority_declaration.as_ref()
    }

    pub fn authorization_policy(&self) -> Option<&ForgeServerOperationAuthorizationPolicy> {
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
        use crate::ForgeServerOperationFamily::{
            BinaryTransfer, ProductApplicationMutation, ProductApplicationRead,
            ProductSessionCoordination, QueryDirectProjection, QueryDirectRead,
            QueryDirectSubmission, SyncLease,
        };
        use crate::ForgeServerSurfaceFamily::{CompatHttp, ForgeNative, Sync};

        vec![
            Self::enabled(QueryDirectRead)
                .exposed_on([ForgeNative, CompatHttp])
                .with_authority_declaration(
                    ForgeServerOperationAuthorityDeclaration::query_shared_read(),
                )
                .with_authorization_policy(
                    ForgeServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(QueryDirectSubmission)
                .exposed_on([ForgeNative, CompatHttp])
                .with_authority_declaration(
                    ForgeServerOperationAuthorityDeclaration::deterministic_submission(
                        "query-write",
                        "query-write-review",
                        "derive-from-request",
                        "derive-from-request",
                    ),
                )
                .with_authorization_policy(
                    ForgeServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(QueryDirectProjection)
                .exposed_on([ForgeNative])
                .with_authority_declaration(
                    ForgeServerOperationAuthorityDeclaration::query_shared_read(),
                )
                .with_authorization_policy(
                    ForgeServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(ProductApplicationRead)
                .exposed_on([ForgeNative, CompatHttp])
                .with_authority_declaration(
                    ForgeServerOperationAuthorityDeclaration::product_shared_read(
                        ForgeServerSharedReadBasisKind::QueryDerived,
                    ),
                )
                .with_authorization_policy(
                    ForgeServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(ProductApplicationMutation)
                .exposed_on([ForgeNative, CompatHttp])
                .with_authority_declaration(
                    ForgeServerOperationAuthorityDeclaration::product_draft_mutation(
                        "product-draft",
                        "derive-from-request",
                        "derive-from-request",
                    ),
                )
                .with_authorization_policy(
                    ForgeServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(ProductSessionCoordination)
                .exposed_on([ForgeNative, CompatHttp])
                .with_authority_declaration(
                    ForgeServerOperationAuthorityDeclaration::product_session_coordination(
                        "product-session",
                    ),
                )
                .with_authorization_policy(
                    ForgeServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(BinaryTransfer)
                .exposed_on([CompatHttp])
                .with_authority_declaration(
                    ForgeServerOperationAuthorityDeclaration::binary_streaming(
                        "binary-transfer",
                        "preflight-required",
                        "declared-size",
                        "cancellable",
                        "partial-failure-surfaced",
                    ),
                )
                .with_authorization_policy(
                    ForgeServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
            Self::enabled(SyncLease)
                .exposed_on([ForgeNative, Sync])
                .with_authority_declaration(
                    ForgeServerOperationAuthorityDeclaration::lease_coordination(
                        "query-downstream-delivery",
                    ),
                )
                .with_authorization_policy(
                    ForgeServerOperationAuthorizationPolicy::allow_authenticated(),
                ),
        ]
    }
}
