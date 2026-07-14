use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::session_label::WorthQuerySessionLabel;

use super::{
    WorthQueryInspection, WorthQueryPreviewBasisAdmission, WorthQueryPreviewOutcome,
    WorthQueryRuntime, WorthQueryRuntimeError, WorthQueryWriteCommand,
};

pub(crate) struct WorthQueryLowerRuntimePreviewExecution {
    request_identity: WorthQueryEvidenceIdentity,
    receipt_identity: WorthQueryEvidenceIdentity,
    aftermath_identity: WorthQueryEvidenceIdentity,
    inspection_identity: Option<WorthQueryEvidenceIdentity>,
    outcome: WorthQueryPreviewOutcome,
}

impl WorthQueryLowerRuntimePreviewExecution {
    pub(crate) fn request_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.request_identity
    }

    pub(crate) fn receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.receipt_identity
    }

    pub(crate) fn aftermath_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.aftermath_identity
    }

    pub(crate) fn inspection_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.inspection_identity.as_ref()
    }

    pub(crate) fn outcome(&self) -> &WorthQueryPreviewOutcome {
        &self.outcome
    }

    pub(crate) fn into_outcome(self) -> WorthQueryPreviewOutcome {
        self.outcome
    }
}

impl WorthQueryRuntime {
    pub(crate) fn execute_ordinary_read_only_preview(
        &mut self,
        basis_admission: WorthQueryPreviewBasisAdmission,
        declaration_identity: &WorthQueryEvidenceIdentity,
        materialize_inspection: bool,
    ) -> Result<WorthQueryLowerRuntimePreviewExecution, WorthQueryRuntimeError> {
        let label = basis_admission.session_label().clone();
        let request_identity = preview_request_identity("read-only", &label, declaration_identity);
        let outcome = {
            let session = self.open_preview_with_admitted_basis(basis_admission)?;
            session.discard()
        };
        let receipt_identity = outcome.closeout_evidence().closeout_identity().clone();
        self.finish_ordinary_preview_execution(
            request_identity,
            receipt_identity,
            outcome,
            materialize_inspection,
        )
    }

    pub(crate) fn execute_ordinary_preview_promotion(
        &mut self,
        basis_admission: WorthQueryPreviewBasisAdmission,
        declaration_identity: &WorthQueryEvidenceIdentity,
        command: WorthQueryWriteCommand,
        materialize_inspection: bool,
    ) -> Result<WorthQueryLowerRuntimePreviewExecution, WorthQueryRuntimeError> {
        let label = basis_admission.session_label().clone();
        let request_identity = preview_request_identity("promotion", &label, declaration_identity);
        let (receipt_identity, outcome) = {
            let mut session = self.open_preview_with_admitted_basis(basis_admission)?;
            let preview_receipt = session.write(command)?;
            let receipt_identity = preview_receipt.commit_evidence_identity().clone();
            (receipt_identity, session.promote()?)
        };
        self.finish_ordinary_preview_execution(
            request_identity,
            receipt_identity,
            outcome,
            materialize_inspection,
        )
    }

    fn finish_ordinary_preview_execution(
        &self,
        request_identity: WorthQueryEvidenceIdentity,
        receipt_identity: WorthQueryEvidenceIdentity,
        outcome: WorthQueryPreviewOutcome,
        materialize_inspection: bool,
    ) -> Result<WorthQueryLowerRuntimePreviewExecution, WorthQueryRuntimeError> {
        let inspection_identity = if materialize_inspection {
            let inspection = match self.inspect(&outcome)? {
                WorthQueryInspection::PreviewOutcome(inspection) => inspection,
                other => panic!("expected preview outcome inspection, got {other:?}"),
            };
            Some(inspection.inspection_identity().clone())
        } else {
            None
        };
        Ok(WorthQueryLowerRuntimePreviewExecution {
            request_identity,
            receipt_identity,
            aftermath_identity: outcome.closeout_evidence().closeout_identity().clone(),
            inspection_identity,
            outcome,
        })
    }
}

fn preview_request_identity(
    family: &'static str,
    label: &WorthQuerySessionLabel,
    declaration_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "ordinary-preview-request",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), family)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("session_label"),
            label.identity_digest(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("declaration"),
            declaration_identity,
        )
        .seal()
}
