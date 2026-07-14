use serde::Serialize;

use crate::backend::records::StableBasisRecord;
use crate::failure::{StoreError, StoreErrorKind};
use crate::live_query::restart::StableBasisSurvival;
use crate::live_query::retention_descriptor::ContinuationRetentionDescriptor;

use super::{StableBasisHandle, StableBasisId, StableBasisReadRequest};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct StableBasisPublicationPlan {
    request: StableBasisReadRequest,
    stable_basis_id: StableBasisId,
    retention_descriptor: ContinuationRetentionDescriptor,
}

impl StableBasisPublicationPlan {
    pub(crate) fn new(
        request: StableBasisReadRequest,
        stable_basis_id: StableBasisId,
        retention_descriptor: ContinuationRetentionDescriptor,
    ) -> Self {
        Self {
            request,
            stable_basis_id,
            retention_descriptor,
        }
    }

    pub(crate) fn to_record(&self) -> StableBasisRecord {
        StableBasisRecord {
            artifact_id: self.stable_basis_id.as_str().to_string(),
            family_version: 1,
            request: self.request.clone(),
            minimum_retained_commit_id: self.retention_descriptor.minimum_retained_commit_id(),
            required_support_artifact_set: self
                .retention_descriptor
                .required_support_artifact_set()
                .to_vec(),
            schema_boundary_dependency: self
                .retention_descriptor
                .schema_boundary_dependency()
                .to_string(),
            authority_replay_fallback_class: self
                .retention_descriptor
                .authority_replay_fallback_class()
                .to_string(),
            snapshot_tail_fallback_class: self
                .retention_descriptor
                .snapshot_tail_fallback_class()
                .to_string(),
            descriptor_version: self.retention_descriptor.version(),
        }
    }

    pub(crate) fn into_handle(self) -> StableBasisHandle {
        StableBasisHandle::new(
            self.request,
            self.stable_basis_id,
            self.retention_descriptor,
        )
    }
}

pub(crate) fn stable_basis_handle_from_record_with_survival(
    record: &StableBasisRecord,
    survival: StableBasisSurvival,
) -> StableBasisHandle {
    let stable_basis_id = StableBasisId::from_string(record.artifact_id.clone());
    let descriptor = ContinuationRetentionDescriptor::new(
        stable_basis_id.clone(),
        record.minimum_retained_commit_id,
        record.required_support_artifact_set.clone(),
        record.schema_boundary_dependency.clone(),
        record.authority_replay_fallback_class.clone(),
        record.snapshot_tail_fallback_class.clone(),
        record.descriptor_version,
    );
    let mut request = record.request.clone();
    request.set_retention_status(survival.to_retention_status());
    StableBasisHandle::new(request, stable_basis_id, descriptor)
}

pub(crate) fn validate_stable_basis_request(
    request: &StableBasisReadRequest,
) -> Result<(), StoreError> {
    if request.read_scope().fingerprint().is_empty() || request.branch_id().0.is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::StableBasisShapeViolation,
            "stable-basis reads require a branch identity and a concrete scope fingerprint",
        ));
    }
    if request.support_context_digest().trim().is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::StableBasisSupportContextMismatch,
            "stable-basis reads require a non-empty support-context digest",
        ));
    }
    if request.schema_boundary_artifact_id().trim().is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::StableBasisSchemaMismatch,
            "stable-basis reads require a schema-boundary artifact id",
        ));
    }
    if request.authority_basis_digest().trim().is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::StableBasisShapeViolation,
            "stable-basis reads require an authority-basis digest",
        ));
    }
    if let StableBasisSurvival::Rejected { .. } = StableBasisSurvival::from_request(request) {
        return Err(StoreError::new(
            StoreErrorKind::StableBasisRetainedStateRejected,
            "rejected retention status cannot be used to plan stable-basis reads",
        ));
    }
    Ok(())
}

pub(crate) fn validate_stable_basis_handle(basis: &StableBasisHandle) -> Result<(), StoreError> {
    validate_stable_basis_request(basis.request())?;
    match StableBasisSurvival::from_handle(basis) {
        StableBasisSurvival::Retained | StableBasisSurvival::DegradedButRecoverable { .. } => {
            Ok(())
        }
        StableBasisSurvival::Rejected { .. } => Err(StoreError::new(
            StoreErrorKind::StableBasisRetainedStateRejected,
            "rejected stable-basis handle cannot be reused for continuation",
        )),
    }
}
