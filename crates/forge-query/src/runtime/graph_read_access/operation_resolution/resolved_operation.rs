use super::{ForgeQueryBuiltInGraphReadOperation, ForgeQueryDomainRegisteredGraphReadOperation};
use crate::runtime::{
    ForgeQueryAdmittedQuerySchemaReferences, ForgeQueryGraphReadBasisBinding,
    ForgeQueryGraphReadPolicyTenantProofBinding, ForgeQueryReadBuiltInOperator,
    ForgeQueryReadGraphFamily, ForgeQueryReadScopeClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadResolvedOperationFamily {
    BuiltIn,
    DomainRegistered,
    DeclaredTraversal,
}

impl ForgeQueryGraphReadResolvedOperationFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BuiltIn => "built_in",
            Self::DomainRegistered => "domain_registered",
            Self::DeclaredTraversal => "declared_traversal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadResolvedOperationKind {
    BuiltIn(ForgeQueryReadBuiltInOperator),
    DomainRegistered(ForgeQueryDomainRegisteredGraphReadOperation),
    DeclarationTraversal,
}

impl ForgeQueryGraphReadResolvedOperationKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BuiltIn(operator) => operator.as_str(),
            Self::DomainRegistered(operation) => operation.operation_name(),
            Self::DeclarationTraversal => "declaration_traversal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadResolvedOperation {
    family: ForgeQueryGraphReadResolvedOperationFamily,
    kind: ForgeQueryGraphReadResolvedOperationKind,
    built_in: Option<ForgeQueryBuiltInGraphReadOperation>,
}

impl ForgeQueryGraphReadResolvedOperation {
    pub fn family(&self) -> &ForgeQueryGraphReadResolvedOperationFamily {
        &self.family
    }

    pub fn kind(&self) -> &ForgeQueryGraphReadResolvedOperationKind {
        &self.kind
    }

    pub fn built_in_operation(&self) -> Option<&ForgeQueryBuiltInGraphReadOperation> {
        self.built_in.as_ref()
    }

    pub fn built_in_operator(&self) -> Option<&ForgeQueryReadBuiltInOperator> {
        match &self.kind {
            ForgeQueryGraphReadResolvedOperationKind::BuiltIn(operator) => Some(operator),
            ForgeQueryGraphReadResolvedOperationKind::DomainRegistered(_)
            | ForgeQueryGraphReadResolvedOperationKind::DeclarationTraversal => None,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        let basis = format!("operation:{}:{}", self.family.as_str(), self.kind.as_str());
        match &self.kind {
            ForgeQueryGraphReadResolvedOperationKind::BuiltIn(_) => self
                .built_in
                .as_ref()
                .map(ForgeQueryBuiltInGraphReadOperation::digest_part)
                .unwrap_or(basis),
            ForgeQueryGraphReadResolvedOperationKind::DomainRegistered(operation) => {
                format!("{basis}:{}", operation.digest_part())
            }
            ForgeQueryGraphReadResolvedOperationKind::DeclarationTraversal => basis,
        }
    }

    pub(crate) fn built_in(operator: ForgeQueryReadBuiltInOperator) -> Self {
        let built_in = ForgeQueryBuiltInGraphReadOperation::admitted(operator.clone());
        Self {
            family: ForgeQueryGraphReadResolvedOperationFamily::BuiltIn,
            kind: ForgeQueryGraphReadResolvedOperationKind::BuiltIn(operator),
            built_in: Some(built_in),
        }
    }

    pub(crate) fn domain_registered(
        operation: ForgeQueryDomainRegisteredGraphReadOperation,
    ) -> Self {
        Self {
            family: ForgeQueryGraphReadResolvedOperationFamily::DomainRegistered,
            kind: ForgeQueryGraphReadResolvedOperationKind::DomainRegistered(operation),
            built_in: None,
        }
    }

    pub(crate) fn declaration_traversal() -> Self {
        Self {
            family: ForgeQueryGraphReadResolvedOperationFamily::DeclaredTraversal,
            kind: ForgeQueryGraphReadResolvedOperationKind::DeclarationTraversal,
            built_in: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadOperationResolution {
    read_graph_digest: String,
    graph_family: ForgeQueryReadGraphFamily,
    scope_class: ForgeQueryReadScopeClass,
    admitted_reference_count: usize,
    basis_binding_digest_part: String,
    policy_tenant_proof_digest_part: String,
    references: ForgeQueryAdmittedQuerySchemaReferences,
    basis_binding: ForgeQueryGraphReadBasisBinding,
    policy_tenant_proof_binding: ForgeQueryGraphReadPolicyTenantProofBinding,
    operations: Vec<ForgeQueryGraphReadResolvedOperation>,
}

impl ForgeQueryGraphReadOperationResolution {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn admitted_reference_count(&self) -> usize {
        self.admitted_reference_count
    }

    pub fn graph_family(&self) -> &ForgeQueryReadGraphFamily {
        &self.graph_family
    }

    pub fn scope_class(&self) -> &ForgeQueryReadScopeClass {
        &self.scope_class
    }

    pub fn operations(&self) -> &[ForgeQueryGraphReadResolvedOperation] {
        &self.operations
    }

    pub fn references(&self) -> &ForgeQueryAdmittedQuerySchemaReferences {
        &self.references
    }

    pub fn basis_binding(&self) -> &ForgeQueryGraphReadBasisBinding {
        &self.basis_binding
    }

    pub fn policy_tenant_proof_binding(&self) -> &ForgeQueryGraphReadPolicyTenantProofBinding {
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
        graph_family: ForgeQueryReadGraphFamily,
        scope_class: ForgeQueryReadScopeClass,
        admitted_reference_count: usize,
        references: ForgeQueryAdmittedQuerySchemaReferences,
        basis_binding: ForgeQueryGraphReadBasisBinding,
        policy_tenant_proof_binding: ForgeQueryGraphReadPolicyTenantProofBinding,
        operations: Vec<ForgeQueryGraphReadResolvedOperation>,
    ) -> Self {
        let basis_binding_digest_part = basis_binding.digest_part();
        let policy_tenant_proof_digest_part = policy_tenant_proof_binding.digest_part();
        Self {
            read_graph_digest: read_graph_digest.into(),
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
