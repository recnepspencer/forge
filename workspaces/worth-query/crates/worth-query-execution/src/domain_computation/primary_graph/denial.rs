#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPrimaryGraphInstallationDenialKind {
    ForeignRuntime,
    StaleInstalledSchema,
    AlreadyInstalled,
    BindingNotInstalled,
    BindingSchemaMismatch,
    DuplicateExternalIdentity,
    DuplicatePrincipalIdentity,
    DuplicatePrincipalKey,
    EmptyBootstrap,
    InvalidSchemaMember,
    RelationalCommitRejected,
    IndexBuildRejected,
    RelationalSchemaRejected,
    RelationalRuntimeAlreadyPublished,
    AuthorizationPolicyRejected,
    RuntimeBridgeRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPrimaryGraphInstallationDenial {
    kind: WorthQueryPrimaryGraphInstallationDenialKind,
    subject: String,
}

impl WorthQueryPrimaryGraphInstallationDenial {
    pub(super) fn new(
        kind: WorthQueryPrimaryGraphInstallationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryPrimaryGraphInstallationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for WorthQueryPrimaryGraphInstallationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "primary graph installation denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryPrimaryGraphInstallationDenial {}
