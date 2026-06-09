use forge_foundational::facade::DiagnosticRichnessProfile;

use super::{
    ForgeServerAuthenticatedPrincipal, ForgeServerBranchTarget, ForgeServerWorkspaceTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContext {
    authenticated_principal: ForgeServerAuthenticatedPrincipal,
    workspace_target: ForgeServerWorkspaceTarget,
    branch_target: ForgeServerBranchTarget,
    diagnostics_profile: DiagnosticRichnessProfile,
}

impl ForgeServerRequestContext {
    pub(crate) fn new(
        authenticated_principal: ForgeServerAuthenticatedPrincipal,
        workspace_target: ForgeServerWorkspaceTarget,
        branch_target: ForgeServerBranchTarget,
        diagnostics_profile: DiagnosticRichnessProfile,
    ) -> Self {
        Self {
            authenticated_principal,
            workspace_target,
            branch_target,
            diagnostics_profile,
        }
    }

    pub fn authenticated_principal(&self) -> &ForgeServerAuthenticatedPrincipal {
        &self.authenticated_principal
    }

    pub fn workspace_target(&self) -> &ForgeServerWorkspaceTarget {
        &self.workspace_target
    }

    pub fn branch_target(&self) -> &ForgeServerBranchTarget {
        &self.branch_target
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }
}
