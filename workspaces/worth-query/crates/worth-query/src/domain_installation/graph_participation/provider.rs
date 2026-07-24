#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphProviderCallKind {
    Observe,
    Project,
    TouchEffect,
    CommitAdmission,
}

impl WorthQueryGraphProviderCallKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Project => "project",
            Self::TouchEffect => "touch-effect",
            Self::CommitAdmission => "commit-admission",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphProviderCall {
    call_identity: String,
    kind: WorthQueryGraphProviderCallKind,
    scope_identity: String,
    operation_identity: String,
    binding_identity: String,
    graph_role: String,
    canonical_query_digest: String,
    basis_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphCommitCall {
    call_identity: String,
    scope_identity: String,
    operation_identity: String,
    binding_identity: String,
    graph_roles: Vec<String>,
}

impl WorthQueryGraphCommitCall {
    pub(crate) fn new(
        scope_identity: String,
        operation_identity: String,
        binding_identity: String,
        mut graph_roles: Vec<String>,
    ) -> Self {
        graph_roles.sort();
        graph_roles.dedup();
        Self {
            call_identity: crate::identity::hash_parts(&[
                "worth_query_graph_commit_call_v1".into(),
                format!("operation:{operation_identity}"),
                format!("binding:{binding_identity}"),
                format!("roles:{}", graph_roles.join(",")),
                format!("scope:{scope_identity}"),
            ]),
            scope_identity,
            operation_identity,
            binding_identity,
            graph_roles,
        }
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn graph_roles(&self) -> &[String] {
        &self.graph_roles
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn completed(&self, provider_receipt: impl Into<String>) -> WorthQueryGraphProviderReceipt {
        WorthQueryGraphProviderReceipt::completed(self.call_identity.clone(), provider_receipt)
    }

    pub(crate) fn call_identity(&self) -> &str {
        &self.call_identity
    }
}

impl WorthQueryGraphProviderCall {
    pub(crate) fn new(
        scope_identity: String,
        kind: WorthQueryGraphProviderCallKind,
        operation_identity: String,
        binding_identity: String,
        graph_role: String,
        canonical_query_digest: String,
        basis_identity: String,
    ) -> Self {
        Self {
            call_identity: crate::identity::hash_parts(&[
                "worth_query_graph_provider_call_v1".into(),
                format!("operation:{operation_identity}"),
                format!("binding:{binding_identity}"),
                format!("role:{graph_role}"),
                format!("query:{canonical_query_digest}"),
                format!("basis:{basis_identity}"),
                format!("kind:{}", kind.as_str()),
                format!("scope:{scope_identity}"),
            ]),
            kind,
            scope_identity,
            operation_identity,
            binding_identity,
            graph_role,
            canonical_query_digest,
            basis_identity,
        }
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn kind(&self) -> WorthQueryGraphProviderCallKind {
        self.kind
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn graph_role(&self) -> &str {
        &self.graph_role
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn canonical_query_digest(&self) -> &str {
        &self.canonical_query_digest
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn completed(&self, provider_receipt: impl Into<String>) -> WorthQueryGraphProviderReceipt {
        WorthQueryGraphProviderReceipt::completed(self.call_identity.clone(), provider_receipt)
    }

    pub fn projected(
        &self,
        provider_receipt: impl Into<String>,
        projection: crate::runtime::WorthQueryReadResult,
    ) -> WorthQueryGraphProviderReceipt {
        WorthQueryGraphProviderReceipt::projected(
            self.call_identity.clone(),
            provider_receipt,
            projection,
        )
    }

    pub(crate) fn call_identity(&self) -> &str {
        &self.call_identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryGraphProviderReceipt {
    call_identity: String,
    provider_receipt: String,
    projection: Option<Box<crate::runtime::WorthQueryReadResult>>,
}

impl WorthQueryGraphProviderReceipt {
    fn completed(call_identity: String, provider_receipt: impl Into<String>) -> Self {
        Self {
            call_identity,
            provider_receipt: provider_receipt.into(),
            projection: None,
        }
    }

    fn projected(
        call_identity: String,
        provider_receipt: impl Into<String>,
        projection: crate::runtime::WorthQueryReadResult,
    ) -> Self {
        Self {
            call_identity,
            provider_receipt: provider_receipt.into(),
            projection: Some(Box::new(projection)),
        }
    }

    pub(crate) fn provider_receipt(&self) -> &str {
        &self.provider_receipt
    }

    pub(crate) fn into_projection(self) -> Option<Box<crate::runtime::WorthQueryReadResult>> {
        self.projection
    }

    pub(crate) fn binds_call(&self, call_identity: &str) -> bool {
        self.call_identity == call_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphProviderFailure {
    detail: String,
}

impl WorthQueryGraphProviderFailure {
    pub fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub trait WorthQueryGraphParticipationProvider<G>: Send + Sync + 'static {
    fn execution_resource_support(
        &self,
    ) -> crate::domain_installation::WorthQueryExecutionResourceSupport;

    fn observe(
        &self,
        call: &WorthQueryGraphProviderCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;

    fn project(
        &self,
        call: &WorthQueryGraphProviderCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;

    fn touch_effect(
        &self,
        call: &WorthQueryGraphProviderCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;
}

pub trait WorthQueryGraphCommitProvider<C>: Send + Sync + 'static {
    fn admit_commit(
        &self,
        call: &WorthQueryGraphCommitCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;
}
