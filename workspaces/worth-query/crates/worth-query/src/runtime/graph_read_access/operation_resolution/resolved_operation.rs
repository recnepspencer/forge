use super::{WorthQueryBuiltInGraphReadOperation, WorthQueryDomainRegisteredGraphReadOperation};
use crate::runtime::{
    WorthQueryAdmittedQuerySchemaReferences, WorthQueryGraphReadBasisBinding,
    WorthQueryGraphReadPolicyTenantProofBinding, WorthQueryReadBuiltInOperator,
    WorthQueryReadGraphFamily, WorthQueryReadScopeClass,
};
use worth_foundational::facade::CanonicalDigestId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadResolvedOperationFamily {
    BuiltIn,
    DomainRegistered,
    DeclaredTraversal,
}

impl WorthQueryGraphReadResolvedOperationFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BuiltIn => "built_in",
            Self::DomainRegistered => "domain_registered",
            Self::DeclaredTraversal => "declared_traversal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadResolvedOperationKind {
    BuiltIn(WorthQueryReadBuiltInOperator),
    DomainRegistered(WorthQueryDomainRegisteredGraphReadOperation),
    DeclarationTraversal,
}

impl WorthQueryGraphReadResolvedOperationKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BuiltIn(operator) => operator.as_str(),
            Self::DomainRegistered(operation) => operation.operation_name(),
            Self::DeclarationTraversal => "declaration_traversal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadResolvedOperation {
    family: WorthQueryGraphReadResolvedOperationFamily,
    kind: WorthQueryGraphReadResolvedOperationKind,
    built_in: Option<WorthQueryBuiltInGraphReadOperation>,
}

impl WorthQueryGraphReadResolvedOperation {
    pub fn family(&self) -> &WorthQueryGraphReadResolvedOperationFamily {
        &self.family
    }

    pub fn kind(&self) -> &WorthQueryGraphReadResolvedOperationKind {
        &self.kind
    }

    pub fn built_in_operation(&self) -> Option<&WorthQueryBuiltInGraphReadOperation> {
        self.built_in.as_ref()
    }

    pub fn built_in_operator(&self) -> Option<&WorthQueryReadBuiltInOperator> {
        match &self.kind {
            WorthQueryGraphReadResolvedOperationKind::BuiltIn(operator) => Some(operator),
            WorthQueryGraphReadResolvedOperationKind::DomainRegistered(_)
            | WorthQueryGraphReadResolvedOperationKind::DeclarationTraversal => None,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        let basis = format!("operation:{}:{}", self.family.as_str(), self.kind.as_str());
        match &self.kind {
            WorthQueryGraphReadResolvedOperationKind::BuiltIn(_) => self
                .built_in
                .as_ref()
                .map(WorthQueryBuiltInGraphReadOperation::digest_part)
                .unwrap_or(basis),
            WorthQueryGraphReadResolvedOperationKind::DomainRegistered(operation) => {
                format!("{basis}:{}", operation.digest_part())
            }
            WorthQueryGraphReadResolvedOperationKind::DeclarationTraversal => basis,
        }
    }

    pub(crate) fn built_in(operator: WorthQueryReadBuiltInOperator) -> Self {
        let built_in = WorthQueryBuiltInGraphReadOperation::admitted(operator.clone());
        Self {
            family: WorthQueryGraphReadResolvedOperationFamily::BuiltIn,
            kind: WorthQueryGraphReadResolvedOperationKind::BuiltIn(operator),
            built_in: Some(built_in),
        }
    }

    pub(crate) fn domain_registered(
        operation: WorthQueryDomainRegisteredGraphReadOperation,
    ) -> Self {
        Self {
            family: WorthQueryGraphReadResolvedOperationFamily::DomainRegistered,
            kind: WorthQueryGraphReadResolvedOperationKind::DomainRegistered(operation),
            built_in: None,
        }
    }

    pub(crate) fn declaration_traversal() -> Self {
        Self {
            family: WorthQueryGraphReadResolvedOperationFamily::DeclaredTraversal,
            kind: WorthQueryGraphReadResolvedOperationKind::DeclarationTraversal,
            built_in: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadOperationResolution {
    read_graph_digest: String,
    read_graph_canonical_digest: CanonicalDigestId,
    graph_family: WorthQueryReadGraphFamily,
    scope_class: WorthQueryReadScopeClass,
    admitted_reference_count: usize,
    basis_binding_digest_part: String,
    policy_tenant_proof_digest_part: String,
    references: WorthQueryAdmittedQuerySchemaReferences,
    basis_binding: WorthQueryGraphReadBasisBinding,
    policy_tenant_proof_binding: WorthQueryGraphReadPolicyTenantProofBinding,
    operations: Vec<WorthQueryGraphReadResolvedOperation>,
}

impl WorthQueryGraphReadOperationResolution {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub(crate) fn read_graph_canonical_digest(&self) -> &CanonicalDigestId {
        &self.read_graph_canonical_digest
    }

    pub fn admitted_reference_count(&self) -> usize {
        self.admitted_reference_count
    }

    pub fn graph_family(&self) -> &WorthQueryReadGraphFamily {
        &self.graph_family
    }

    pub fn scope_class(&self) -> &WorthQueryReadScopeClass {
        &self.scope_class
    }

    pub fn operations(&self) -> &[WorthQueryGraphReadResolvedOperation] {
        &self.operations
    }

    pub fn references(&self) -> &WorthQueryAdmittedQuerySchemaReferences {
        &self.references
    }

    pub fn basis_binding(&self) -> &WorthQueryGraphReadBasisBinding {
        &self.basis_binding
    }

    pub fn policy_tenant_proof_binding(&self) -> &WorthQueryGraphReadPolicyTenantProofBinding {
        &self.policy_tenant_proof_binding
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("read_graph:{}", self.read_graph_digest),
            format!("graph_family:{:?}", self.graph_family),
            format!("scope_class:{}", self.scope_class.as_str()),
            format!("admitted_reference_count:{}", self.admitted_reference_count),
            self.basis_binding_digest_part.clone(),
            self.policy_tenant_proof_digest_part.clone(),
        ];
        parts.extend(self.references.digest_parts());
        parts.extend(self.operations.iter().map(|row| row.digest_part()));
        parts
    }

    pub(crate) fn new(
        read_graph_digest: impl Into<String>,
        read_graph_canonical_digest: CanonicalDigestId,
        graph_family: WorthQueryReadGraphFamily,
        scope_class: WorthQueryReadScopeClass,
        admitted_reference_count: usize,
        references: WorthQueryAdmittedQuerySchemaReferences,
        basis_binding: WorthQueryGraphReadBasisBinding,
        policy_tenant_proof_binding: WorthQueryGraphReadPolicyTenantProofBinding,
        operations: Vec<WorthQueryGraphReadResolvedOperation>,
    ) -> Self {
        let basis_binding_digest_part = basis_binding.digest_part();
        let policy_tenant_proof_digest_part = policy_tenant_proof_binding.digest_part();
        Self {
            read_graph_digest: read_graph_digest.into(),
            read_graph_canonical_digest,
            graph_family,
            scope_class,
            admitted_reference_count,
            basis_binding_digest_part,
            policy_tenant_proof_digest_part,
            references,
            basis_binding,
            policy_tenant_proof_binding,
            operations,
        }
    }
}
