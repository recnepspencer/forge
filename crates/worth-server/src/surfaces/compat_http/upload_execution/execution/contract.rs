use worth_proof::TransitionOutcome;

use crate::{
    WorthServerCompatibilityPreparedRequest, WorthServerMultipartUpload,
    WorthServerOperationAdmissionPosture, WorthServerQueryHandoffDeferred,
    WorthServerQueryHandoffDenial, WorthServerQueryHandoffFailure,
    WorthServerQueryHandoffRebindRequired, WorthServerQueryHandoffStale,
};

pub type WorthServerCompatibilityUploadOutcome<T> = TransitionOutcome<
    T,
    WorthServerQueryHandoffDenial,
    WorthServerQueryHandoffDeferred,
    WorthServerQueryHandoffStale,
    WorthServerQueryHandoffRebindRequired,
    WorthServerQueryHandoffFailure,
>;

#[derive(Clone, Debug)]
pub struct WorthServerCompatibilityUploadExecutionInput {
    prepared_request: WorthServerCompatibilityPreparedRequest,
    operation_name: String,
    upload: WorthServerMultipartUpload,
}

impl WorthServerCompatibilityUploadExecutionInput {
    pub fn new(
        prepared_request: WorthServerCompatibilityPreparedRequest,
        operation_name: impl Into<String>,
        upload: WorthServerMultipartUpload,
    ) -> Self {
        Self {
            prepared_request,
            operation_name: operation_name.into().trim().to_string(),
            upload,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthServerCompatibilityPreparedRequest,
        String,
        WorthServerMultipartUpload,
    ) {
        (self.prepared_request, self.operation_name, self.upload)
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerPreparedMultipartUpload {
    prepared_request: WorthServerCompatibilityPreparedRequest,
    operation_name: String,
    upload: WorthServerMultipartUpload,
    mutation_request: crate::WorthServerCompatibilityMutationRequest,
    operation_admission: WorthServerOperationAdmissionPosture,
}

impl WorthServerPreparedMultipartUpload {
    pub(super) fn from_parts(
        prepared_request: WorthServerCompatibilityPreparedRequest,
        operation_name: String,
        upload: WorthServerMultipartUpload,
        mutation_request: crate::WorthServerCompatibilityMutationRequest,
        operation_admission: WorthServerOperationAdmissionPosture,
    ) -> Self {
        Self {
            prepared_request,
            operation_name,
            upload,
            mutation_request,
            operation_admission,
        }
    }

    pub fn prepared_request(&self) -> &WorthServerCompatibilityPreparedRequest {
        &self.prepared_request
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn upload(&self) -> &WorthServerMultipartUpload {
        &self.upload
    }

    pub fn mutation_request(&self) -> &crate::WorthServerCompatibilityMutationRequest {
        &self.mutation_request
    }

    pub fn operation_admission(&self) -> &WorthServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn requires_early_admission(&self) -> bool {
        self.upload.expectation().requires_early_admission()
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthServerCompatibilityPreparedRequest,
        String,
        WorthServerMultipartUpload,
        crate::WorthServerCompatibilityMutationRequest,
        WorthServerOperationAdmissionPosture,
    ) {
        (
            self.prepared_request,
            self.operation_name,
            self.upload,
            self.mutation_request,
            self.operation_admission,
        )
    }
}

pub(super) fn parse_upload_mutation_request(
    upload: &WorthServerMultipartUpload,
    diagnostics_profile: worth_foundational::facade::DiagnosticRichnessProfile,
) -> Result<crate::WorthServerCompatibilityMutationRequest, WorthServerQueryHandoffDenial> {
    crate::surfaces::compat_http::mutation_execution::WorthServerCompatibilityMutationRequest::parse(
        upload.manifest().metadata_body().clone(),
        diagnostics_profile,
    )
}
