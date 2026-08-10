use worth_proof::TransitionOutcome;

use crate::{WorthServerCompatibilityFacade, WorthServerQueryHandoffFailure};

use super::super::super::abuse_accounting::{
    denied_budget_receipt_for_prepared_request, WorthServerAbuseBudgetDenialClass,
    WorthServerTransferByteClass,
};
use super::super::{
    ingress_policy::admit_binary_ingress,
    lifecycle::{require_active_session, stage_binary_ingress},
    performance::WorthServerIngressPerformanceReceipt,
    session::{WorthServerBinaryIngressSession, WorthServerVerifiedBinaryIngress},
};
use super::contract::{WorthServerCompatibilityUploadOutcome, WorthServerPreparedMultipartUpload};

impl WorthServerCompatibilityFacade {
    pub fn begin_binary_ingress(
        &self,
        prepared: WorthServerPreparedMultipartUpload,
    ) -> WorthServerCompatibilityUploadOutcome<WorthServerBinaryIngressSession> {
        let (prepared_request, operation_name, upload, mutation_request, operation_admission) =
            prepared.into_parts();
        let diagnostics_profile = prepared_request
            .admission()
            .request_context()
            .diagnostics_profile();
        let metrics = match admit_binary_ingress(&upload, diagnostics_profile) {
            Ok(value) => value,
            Err(denial) => {
                let detail = denial.detail().to_string();
                return TransitionOutcome::Denied(denial.with_abuse_budget_receipt(
                    denied_budget_receipt_for_prepared_request(
                        &prepared_request,
                        WorthServerTransferByteClass::BinaryWire,
                        detail,
                        WorthServerAbuseBudgetDenialClass::SlowlorisCutoff,
                    ),
                ));
            }
        };
        let performance_receipt = match WorthServerIngressPerformanceReceipt::build(
            metrics,
            "compat_http.upload.ingress",
        ) {
            Ok(value) => value,
            Err(_) => {
                return TransitionOutcome::Failed(WorthServerQueryHandoffFailure::new(
                    "compatibility_upload_ingress_performance_receipt_failed",
                ));
            }
        };
        let session = WorthServerBinaryIngressSession::new(
            prepared_request,
            operation_name,
            upload,
            mutation_request,
            operation_admission,
            performance_receipt,
        );
        if let Err(denial) = stage_binary_ingress(self, &session) {
            return TransitionOutcome::Denied(denial);
        }
        TransitionOutcome::Success(session)
    }

    pub fn verify_binary_ingress(
        &self,
        session: WorthServerBinaryIngressSession,
    ) -> WorthServerCompatibilityUploadOutcome<WorthServerVerifiedBinaryIngress> {
        if let Err(denial) = require_active_session(self, &session) {
            return TransitionOutcome::Denied(denial);
        }
        let diagnostics_profile = session
            .prepared_request()
            .admission()
            .request_context()
            .diagnostics_profile();
        let integrity_digest =
            match super::super::integrity::WorthServerIngressIntegrityDigest::verify(
                session.upload(),
                diagnostics_profile,
            ) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        TransitionOutcome::Success(WorthServerVerifiedBinaryIngress::new(
            session,
            integrity_digest,
        ))
    }
}
