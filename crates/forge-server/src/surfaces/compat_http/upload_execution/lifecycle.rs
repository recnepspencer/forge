use crate::{ForgeServerCompatibilityFacade, ForgeServerQueryHandoffDenial};

use super::{
    cleanup::{ForgeServerUploadCleanupReason, ForgeServerUploadCleanupReceipt},
    session::ForgeServerBinaryIngressSession,
};

pub(crate) fn stage_binary_ingress(
    facade: &ForgeServerCompatibilityFacade,
    session: &ForgeServerBinaryIngressSession,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    let mut store = facade
        .binary_ingress_store
        .lock()
        .expect("compatibility binary ingress store mutex should not be poisoned");
    if store.contains_key(session.session_digest()) {
        return Err(upload_request_invalid(
            session
                .prepared_request()
                .admission()
                .request_context()
                .diagnostics_profile(),
            format!(
                "compatibility upload session `{}` is already staged and cannot be started twice",
                session.session_digest()
            ),
        ));
    }
    store.insert(
        session.session_digest().to_string(),
        ForgeServerStoredBinaryIngress::new(
            session.session_digest().to_string(),
            session.tenant_id().to_string(),
            session.workspace_digest().to_string(),
            session.branch_digest().to_string(),
            session
                .upload()
                .parts()
                .iter()
                .map(|part| part.authoritative_len())
                .sum(),
        ),
    );
    Ok(())
}

pub(crate) fn require_active_session(
    facade: &ForgeServerCompatibilityFacade,
    session: &ForgeServerBinaryIngressSession,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    let store = facade
        .binary_ingress_store
        .lock()
        .expect("compatibility binary ingress store mutex should not be poisoned");
    let Some(stored) = store.get(session.session_digest()) else {
        return Err(upload_request_invalid(
            session
                .prepared_request()
                .admission()
                .request_context()
                .diagnostics_profile(),
            format!(
                "compatibility upload session `{}` is no longer active",
                session.session_digest()
            ),
        ));
    };
    if !stored.is_active() {
        return Err(upload_request_invalid(
            session
                .prepared_request()
                .admission()
                .request_context()
                .diagnostics_profile(),
            format!(
                "compatibility upload session `{}` was already finalized or cleaned up",
                session.session_digest()
            ),
        ));
    }
    Ok(())
}

pub(crate) fn remove_active_session(
    facade: &ForgeServerCompatibilityFacade,
    session: &ForgeServerBinaryIngressSession,
) {
    let mut store = facade
        .binary_ingress_store
        .lock()
        .expect("compatibility binary ingress store mutex should not be poisoned");
    if let Some(stored) = store.get_mut(session.session_digest()) {
        stored.deactivate();
    }
    store.remove(session.session_digest());
}

pub(crate) fn cleanup_active_session(
    facade: &ForgeServerCompatibilityFacade,
    session: &ForgeServerBinaryIngressSession,
    reason: ForgeServerUploadCleanupReason,
) -> Result<ForgeServerUploadCleanupReceipt, ForgeServerQueryHandoffDenial> {
    let mut store = facade
        .binary_ingress_store
        .lock()
        .expect("compatibility binary ingress store mutex should not be poisoned");
    let Some(mut stored) = store.remove(session.session_digest()) else {
        return Err(upload_request_invalid(
            session
                .prepared_request()
                .admission()
                .request_context()
                .diagnostics_profile(),
            format!(
                "compatibility upload session `{}` is no longer active",
                session.session_digest()
            ),
        ));
    };
    if !stored.is_active() {
        return Err(upload_request_invalid(
            session
                .prepared_request()
                .admission()
                .request_context()
                .diagnostics_profile(),
            format!(
                "compatibility upload session `{}` was already finalized or cleaned up",
                session.session_digest()
            ),
        ));
    }
    stored.deactivate();
    Ok(ForgeServerUploadCleanupReceipt::new(&stored, reason))
}

pub(crate) fn upload_request_invalid(
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
    detail: impl Into<String>,
) -> ForgeServerQueryHandoffDenial {
    ForgeServerQueryHandoffDenial::new(
        crate::ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        diagnostics_profile,
        detail,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeServerStoredBinaryIngress {
    session_digest: String,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    staged_authoritative_bytes: u64,
    active: bool,
}

impl ForgeServerStoredBinaryIngress {
    pub(crate) fn new(
        session_digest: String,
        tenant_id: String,
        workspace_digest: String,
        branch_digest: String,
        staged_authoritative_bytes: u64,
    ) -> Self {
        Self {
            session_digest,
            tenant_id,
            workspace_digest,
            branch_digest,
            staged_authoritative_bytes,
            active: true,
        }
    }

    pub(crate) fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub(crate) fn workspace_digest(&self) -> &str {
        &self.workspace_digest
    }

    pub(crate) fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub(crate) fn branch_digest(&self) -> &str {
        &self.branch_digest
    }

    pub(crate) fn staged_authoritative_bytes(&self) -> u64 {
        self.staged_authoritative_bytes
    }

    pub(crate) fn is_active(&self) -> bool {
        self.active
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
    }
}
