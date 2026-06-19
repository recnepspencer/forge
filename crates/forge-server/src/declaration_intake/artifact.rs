use forge_query::facade::{
    ForgeQueryLiveReadResult, ForgeQueryRuntimeError, ForgeQueryRuntimePublicApiFamilyContract,
    ForgeQueryRuntimeStateSnapshot, ForgeQueryUnifiedInspectionResult, ForgeQueryWorkspace,
    ProjectionFactConsumptionAttempt, ProjectionFactConsumptionPathError,
};
use std::{fmt, sync::Mutex};

use crate::{ForgeServerAdmission, ForgeServerResolvedRequestContext};

use super::{
    ForgeServerDirectDeclaration, ForgeServerDirectDeclarationDenial,
    ForgeServerDirectDeclarationSourceSupportStatus, ForgeServerDirectSupportSnapshot,
};
use crate::ForgeServerDirectProjectionRequest;

pub(crate) enum ForgeServerNamedLiveProjectionExecutionError {
    Runtime(ForgeQueryRuntimeError),
    Consumption(ProjectionFactConsumptionPathError),
}

pub struct ForgeServerPreparedDirectDeclaration {
    admission: ForgeServerAdmission,
    declaration: ForgeServerDirectDeclaration,
    workspace_name: String,
    workspace: Mutex<ForgeQueryWorkspace>,
    declaration_digest: String,
    support_snapshot: ForgeServerDirectSupportSnapshot,
}

impl fmt::Debug for ForgeServerPreparedDirectDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForgeServerPreparedDirectDeclaration")
            .field("declaration", &self.declaration)
            .field("workspace_name", &self.workspace_name)
            .field("declaration_digest", &self.declaration_digest)
            .field("support_snapshot", &self.support_snapshot)
            .finish()
    }
}

impl ForgeServerPreparedDirectDeclaration {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        declaration: ForgeServerDirectDeclaration,
        workspace: ForgeQueryWorkspace,
        declaration_digest: String,
        support_snapshot: ForgeServerDirectSupportSnapshot,
    ) -> Self {
        let workspace_name = workspace.name().to_string();
        Self {
            admission,
            declaration,
            workspace_name,
            workspace: Mutex::new(workspace),
            declaration_digest,
            support_snapshot,
        }
    }

    pub fn declaration(&self) -> &ForgeServerDirectDeclaration {
        &self.declaration
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn support_snapshot(&self) -> &ForgeServerDirectSupportSnapshot {
        &self.support_snapshot
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        self.admission.resolved_request_context()
    }

    pub fn admission(&self) -> &ForgeServerAdmission {
        &self.admission
    }

    pub fn admit(
        self,
    ) -> Result<ForgeServerAdmittedDirectDeclaration, ForgeServerDirectDeclarationDenial> {
        if self.support_snapshot.source_support_status()
            != ForgeServerDirectDeclarationSourceSupportStatus::Supported
        {
            return Err(ForgeServerDirectDeclarationDenial::source_not_admitted(
                self.admission.request_context().diagnostics_profile(),
                self.support_snapshot.source_support_reason(),
                self.support_snapshot.clone(),
            ));
        }

        if self.support_snapshot.read_family_contract().is_none()
            || !self.support_snapshot.read_family_pin_satisfied()
        {
            return Err(
                ForgeServerDirectDeclarationDenial::query_facade_family_not_admitted(
                    self.admission.request_context().diagnostics_profile(),
                    "query workspace does not admit the read facade family for direct declaration intake",
                    self.support_snapshot.clone(),
                ),
            );
        }

        Ok(ForgeServerAdmittedDirectDeclaration {
            admission: self.admission,
            declaration: self.declaration,
            workspace_name: self.workspace_name,
            workspace: self.workspace,
            declaration_digest: self.declaration_digest,
            support_snapshot: self.support_snapshot,
        })
    }
}

pub struct ForgeServerAdmittedDirectDeclaration {
    admission: ForgeServerAdmission,
    declaration: ForgeServerDirectDeclaration,
    workspace_name: String,
    workspace: Mutex<ForgeQueryWorkspace>,
    declaration_digest: String,
    support_snapshot: ForgeServerDirectSupportSnapshot,
}

impl fmt::Debug for ForgeServerAdmittedDirectDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForgeServerAdmittedDirectDeclaration")
            .field("declaration", &self.declaration)
            .field("workspace_name", &self.workspace_name)
            .field("declaration_digest", &self.declaration_digest)
            .field("support_snapshot", &self.support_snapshot)
            .finish()
    }
}

impl ForgeServerAdmittedDirectDeclaration {
    pub fn declaration(&self) -> &ForgeServerDirectDeclaration {
        &self.declaration
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn support_snapshot(&self) -> &ForgeServerDirectSupportSnapshot {
        &self.support_snapshot
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        self.admission.resolved_request_context()
    }

    pub fn query_family_contract(&self) -> &ForgeQueryRuntimePublicApiFamilyContract {
        self.support_snapshot
            .read_family_contract()
            .expect("admitted direct declarations must retain an admitted read-family contract")
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub(crate) fn declaration_binding_label(&self) -> &str {
        self.declaration.source().binding_label()
    }

    pub(crate) fn declaration_canonical_label(&self) -> String {
        self.declaration.source().canonical_label()
    }

    pub(crate) fn execute_named_live_read(
        &self,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.with_workspace_mut(|workspace| {
            workspace.read_live_by_name(self.declaration_binding_label())
        })
    }

    pub(crate) fn snapshot_named_live_state(
        &self,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        self.with_workspace(|workspace| {
            workspace.state_live_by_name(self.declaration_binding_label())
        })
    }

    pub(crate) fn inspect_named_live_view(
        &self,
    ) -> Result<ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
        self.with_workspace(|workspace| {
            workspace.inspect_live_by_name(self.declaration_binding_label())
        })
    }

    pub(crate) fn subscription_basis_digest(&self) -> Result<String, ForgeQueryRuntimeError> {
        self.with_workspace(|workspace| {
            workspace.subscription_basis_digest_by_name(self.declaration_binding_label())
        })
    }

    pub(crate) fn consume_named_live_projection(
        &self,
        request: &ForgeServerDirectProjectionRequest,
    ) -> Result<ProjectionFactConsumptionAttempt, ForgeServerNamedLiveProjectionExecutionError>
    {
        let live_read = self
            .with_workspace_mut(|workspace| {
                workspace.read_live_by_name(self.declaration_binding_label())
            })
            .map_err(ForgeServerNamedLiveProjectionExecutionError::Runtime)?;
        live_read
            .consume_projection_facts_with_binding(
                request.binding_context(
                    live_read.receipt().query_digest(),
                    live_read.receipt().view_shape_digest(),
                ),
                request.requested_facts_owned(),
            )
            .map_err(ForgeServerNamedLiveProjectionExecutionError::Consumption)
    }

    fn with_workspace<T>(&self, operation: impl FnOnce(&ForgeQueryWorkspace) -> T) -> T {
        let workspace = self
            .workspace
            .lock()
            .expect("direct declaration workspace mutex should not be poisoned");
        operation(&workspace)
    }

    fn with_workspace_mut<T>(&self, operation: impl FnOnce(&mut ForgeQueryWorkspace) -> T) -> T {
        let mut workspace = self
            .workspace
            .lock()
            .expect("direct declaration workspace mutex should not be poisoned");
        operation(&mut workspace)
    }
}
