use crate::ForgeServerAdmittedDirectDeclaration;
use forge_query::facade::ForgeQueryRuntimeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectLeaseDeclaration {
    principal_id: String,
    tenant_id: String,
    workspace_id: String,
    branch_label: String,
    workspace_name: String,
    resume_basis_digest: String,
    declaration_digest: String,
    declaration_binding_label: String,
    declaration_canonical_label: String,
    support_digest: String,
    canonical_digest: String,
}

impl ForgeServerDirectLeaseDeclaration {
    pub(crate) fn from_admitted_declaration(
        declaration: &ForgeServerAdmittedDirectDeclaration,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        let request_context = declaration.resolved_request_context().request_context();
        let principal_id = request_context
            .authenticated_principal()
            .principal_id()
            .to_string();
        let tenant_id = request_context.workspace_target().tenant_id().to_string();
        let workspace_id = request_context
            .workspace_target()
            .workspace_id()
            .to_string();
        let branch_label = request_context.branch_target().canonical_label();
        let workspace_name = declaration.workspace_name().to_string();
        let resume_basis_digest = declaration.subscription_basis_digest()?;
        let declaration_digest = declaration.declaration_digest().to_string();
        let declaration_binding_label = declaration
            .declaration()
            .source()
            .binding_label()
            .to_string();
        let declaration_canonical_label = declaration.declaration().source().canonical_label();
        let support_digest = declaration
            .support_snapshot()
            .support_posture_digest()
            .to_string();
        let canonical_digest = format!(
            "forge-server-direct-lease-v1|principal:{principal_id}|tenant:{tenant_id}|workspace:{workspace_id}|branch:{branch_label}|bound:{workspace_name}|basis:{resume_basis_digest}|declaration:{declaration_digest}|binding:{declaration_binding_label}|support:{support_digest}"
        );
        Ok(Self {
            principal_id,
            tenant_id,
            workspace_id,
            branch_label,
            workspace_name,
            resume_basis_digest,
            declaration_digest,
            declaration_binding_label,
            declaration_canonical_label,
            support_digest,
            canonical_digest,
        })
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn principal_id(&self) -> &str {
        &self.principal_id
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    pub fn branch_label(&self) -> &str {
        &self.branch_label
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn resume_basis_digest(&self) -> &str {
        &self.resume_basis_digest
    }

    pub fn declaration_binding_label(&self) -> &str {
        &self.declaration_binding_label
    }

    pub fn declaration_canonical_label(&self) -> &str {
        &self.declaration_canonical_label
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn lease_digest(&self) -> &str {
        self.canonical_digest()
    }
}
