use crate::ForgeServerAdmission;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationAuthorizationPolicy {
    AllowAuthenticated,
    RequirePrincipal(String),
    RequireWorkspace {
        tenant_id: String,
        workspace_id: String,
    },
}

impl ForgeServerOperationAuthorizationPolicy {
    pub fn allow_authenticated() -> Self {
        Self::AllowAuthenticated
    }

    pub fn require_principal(principal_id: impl Into<String>) -> Self {
        Self::RequirePrincipal(principal_id.into())
    }

    pub fn require_workspace(
        tenant_id: impl Into<String>,
        workspace_id: impl Into<String>,
    ) -> Self {
        Self::RequireWorkspace {
            tenant_id: tenant_id.into(),
            workspace_id: workspace_id.into(),
        }
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        match self {
            Self::AllowAuthenticated => Ok(()),
            Self::RequirePrincipal(principal_id) if principal_id.trim().is_empty() => {
                Err("authorization policy principal may not be blank".to_string())
            }
            Self::RequireWorkspace {
                tenant_id,
                workspace_id,
            } if tenant_id.trim().is_empty() || workspace_id.trim().is_empty() => {
                Err("authorization policy workspace identifiers may not be blank".to_string())
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn authorize(
        &self,
        admission: &ForgeServerAdmission,
    ) -> Result<&'static str, String> {
        let request_context = admission.request_context();
        match self {
            Self::AllowAuthenticated => Ok("allow-authenticated"),
            Self::RequirePrincipal(principal_id) => {
                let actual = request_context.authenticated_principal().principal_id();
                if actual == principal_id {
                    Ok("require-principal")
                } else {
                    Err(format!(
                        "principal `{actual}` is not authorized; required principal `{principal_id}`"
                    ))
                }
            }
            Self::RequireWorkspace {
                tenant_id,
                workspace_id,
            } => {
                let actual_tenant = request_context.workspace_target().tenant_id();
                let actual_workspace = request_context.workspace_target().workspace_id();
                if actual_tenant == tenant_id && actual_workspace == workspace_id {
                    Ok("require-workspace")
                } else {
                    Err(format!(
                        "workspace `{actual_tenant}/{actual_workspace}` is not authorized; required workspace `{tenant_id}/{workspace_id}`"
                    ))
                }
            }
        }
    }
}
