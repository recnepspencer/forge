use crate::{
    WorthServerCompatibilityFacade, WorthServerCompatibilityPreparedRequest,
    WorthServerQueryHandoffDenial,
};

use super::super::{
    cleanup::{ownership_matches, WorthServerUploadCleanupReason, WorthServerUploadCleanupReceipt},
    lifecycle::{cleanup_active_session, upload_request_invalid},
    session::WorthServerBinaryIngressSession,
};

impl WorthServerCompatibilityFacade {
    pub fn interrupt_binary_ingress(
        &self,
        session: &WorthServerBinaryIngressSession,
    ) -> Result<WorthServerUploadCleanupReceipt, WorthServerQueryHandoffDenial> {
        cleanup_active_session(self, session, WorthServerUploadCleanupReason::Interrupted)
    }

    pub fn expire_binary_ingress(
        &self,
        session: &WorthServerBinaryIngressSession,
    ) -> Result<WorthServerUploadCleanupReceipt, WorthServerQueryHandoffDenial> {
        cleanup_active_session(self, session, WorthServerUploadCleanupReason::Expired)
    }

    pub fn abandon_binary_ingress(
        &self,
        session: &WorthServerBinaryIngressSession,
    ) -> Result<WorthServerUploadCleanupReceipt, WorthServerQueryHandoffDenial> {
        cleanup_active_session(self, session, WorthServerUploadCleanupReason::Abandoned)
    }

    pub fn cleanup_mismatched_binary_ingress(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        session: &WorthServerBinaryIngressSession,
    ) -> Result<WorthServerUploadCleanupReceipt, WorthServerQueryHandoffDenial> {
        if ownership_matches(prepared_request, session) {
            return Err(upload_request_invalid(
                prepared_request
                    .admission()
                    .request_context()
                    .diagnostics_profile(),
                "compatibility upload mismatch cleanup requires a tenant or branch mismatch",
            ));
        }
        cleanup_active_session(
            self,
            session,
            WorthServerUploadCleanupReason::OwnershipMismatch,
        )
    }
}
