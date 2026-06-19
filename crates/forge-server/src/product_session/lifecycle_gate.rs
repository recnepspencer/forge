use crate::{
    ForgeServerOperationFamily, ForgeServerOperationRequest, ForgeServerProductOperationBasisKind,
};

use super::{
    ForgeServerProductSession, ForgeServerProductSessionExpiryPosture,
    ForgeServerProductSessionLifecycle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerProductSessionDenialCode {
    CoordinationRequestDenied,
    CoordinationAdmissionDenied,
    CoordinationReadinessDenied,
    MissingProductSessionIdentity,
    UnknownProductSessionIdentity,
    ForeignProductSession,
    ExpiredProductSession,
    ClosedProductSession,
    PreviewSessionCannotMutate,
    SessionRebindRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductSessionDenial {
    code: ForgeServerProductSessionDenialCode,
    detail: String,
}

impl ForgeServerProductSessionDenial {
    pub(crate) fn new(
        code: ForgeServerProductSessionDenialCode,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerProductSessionDenialCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub(crate) fn admit_session_for_product_operation(
    session: Option<&ForgeServerProductSession>,
    request: &ForgeServerOperationRequest,
    basis_kind: ForgeServerProductOperationBasisKind,
) -> Result<(), ForgeServerProductSessionDenial> {
    let requires_session = request.identity().operation_family()
        == ForgeServerOperationFamily::ProductApplicationMutation
        || basis_kind == ForgeServerProductOperationBasisKind::ProductSessionDerived;
    if !requires_session {
        return Ok(());
    }
    let requested_identity = request
        .identity()
        .product_session_identity()
        .ok_or_else(|| {
            ForgeServerProductSessionDenial::new(
                ForgeServerProductSessionDenialCode::MissingProductSessionIdentity,
                "product operation requires an admitted product session identity",
            )
        })?;
    let session = session.ok_or_else(|| {
        ForgeServerProductSessionDenial::new(
            ForgeServerProductSessionDenialCode::UnknownProductSessionIdentity,
            format!("product session `{requested_identity}` was not found"),
        )
    })?;
    validate_session_scope(session, request)?;
    validate_session_branch_and_basis(session, request)?;
    validate_session_operation_binding(session, request)?;
    validate_session_lifecycle(session, request)?;
    Ok(())
}

fn validate_session_scope(
    session: &ForgeServerProductSession,
    request: &ForgeServerOperationRequest,
) -> Result<(), ForgeServerProductSessionDenial> {
    let workspace_target = request
        .resolved_request_context()
        .request_context()
        .workspace_target();
    if session.tenant_id() != workspace_target.tenant_id()
        || session.workspace_id() != workspace_target.workspace_id()
    {
        return Err(ForgeServerProductSessionDenial::new(
            ForgeServerProductSessionDenialCode::ForeignProductSession,
            format!(
                "product session `{}` does not belong to tenant `{}` workspace `{}`",
                session.identity().as_str(),
                workspace_target.tenant_id(),
                workspace_target.workspace_id()
            ),
        ));
    }
    Ok(())
}

fn validate_session_branch_and_basis(
    session: &ForgeServerProductSession,
    request: &ForgeServerOperationRequest,
) -> Result<(), ForgeServerProductSessionDenial> {
    let branch_label = request
        .resolved_request_context()
        .request_context()
        .branch_target()
        .canonical_label();
    if session.branch_label() != branch_label {
        return Err(ForgeServerProductSessionDenial::new(
            ForgeServerProductSessionDenialCode::SessionRebindRequired,
            format!(
                "product session `{}` is bound to branch `{}` and cannot be used on `{}` without explicit rebind",
                session.identity().as_str(),
                session.branch_label(),
                branch_label
            ),
        ));
    }
    Ok(())
}

fn validate_session_lifecycle(
    session: &ForgeServerProductSession,
    request: &ForgeServerOperationRequest,
) -> Result<(), ForgeServerProductSessionDenial> {
    match session.expiry_posture() {
        ForgeServerProductSessionExpiryPosture::Expired { .. } => {
            return Err(ForgeServerProductSessionDenial::new(
                ForgeServerProductSessionDenialCode::ExpiredProductSession,
                format!(
                    "product session `{}` has expired",
                    session.identity().as_str()
                ),
            ));
        }
        ForgeServerProductSessionExpiryPosture::Closed { .. } => {
            return Err(ForgeServerProductSessionDenial::new(
                ForgeServerProductSessionDenialCode::ClosedProductSession,
                format!(
                    "product session `{}` has been closed",
                    session.identity().as_str()
                ),
            ));
        }
        ForgeServerProductSessionExpiryPosture::Active { .. } => {}
    }
    if request.identity().operation_family()
        == ForgeServerOperationFamily::ProductApplicationMutation
        && session.lifecycle() == ForgeServerProductSessionLifecycle::ReadOnlyPreview
    {
        return Err(ForgeServerProductSessionDenial::new(
            ForgeServerProductSessionDenialCode::PreviewSessionCannotMutate,
            format!(
                "product session `{}` is preview-only and cannot authorize mutation work",
                session.identity().as_str()
            ),
        ));
    }
    Ok(())
}

fn validate_session_operation_binding(
    session: &ForgeServerProductSession,
    request: &ForgeServerOperationRequest,
) -> Result<(), ForgeServerProductSessionDenial> {
    if request.identity().operation_family()
        != ForgeServerOperationFamily::ProductApplicationMutation
    {
        return Ok(());
    }
    if session.operation_name() != request.identity().operation_name() {
        return Err(ForgeServerProductSessionDenial::new(
            ForgeServerProductSessionDenialCode::SessionRebindRequired,
            format!(
                "product session `{}` is bound to operation `{}` and cannot authorize `{}` without explicit rebind",
                session.identity().as_str(),
                session.operation_name(),
                request.identity().operation_name(),
            ),
        ));
    }
    Ok(())
}
