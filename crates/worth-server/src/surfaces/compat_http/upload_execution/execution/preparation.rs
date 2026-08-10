use worth_proof::TransitionOutcome;

use crate::{
    WorthServerCompatibilityFacade, WorthServerCompatibilityPreparedRequest,
    WorthServerMultipartUpload, WorthServerOperationAdmissionFacade, WorthServerOperationFamily,
    WorthServerOperationRequestFacade, WorthServerQueryHandoffDenial,
};

use super::super::admission::validate_upload_admission;
use super::contract::{
    parse_upload_mutation_request, WorthServerCompatibilityUploadExecutionInput,
    WorthServerCompatibilityUploadOutcome, WorthServerPreparedMultipartUpload,
};

impl WorthServerCompatibilityFacade {
    pub fn prepare_upload(
        &self,
        input: WorthServerCompatibilityUploadExecutionInput,
    ) -> WorthServerCompatibilityUploadOutcome<WorthServerPreparedMultipartUpload> {
        let (prepared_request, operation_name, upload) = input.into_parts();
        if let Err(denial) = self.admit_binary_family(&prepared_request) {
            return TransitionOutcome::Denied(denial);
        }
        if let Err(denial) = self.admit_query_direct_family(&prepared_request) {
            return TransitionOutcome::Denied(denial);
        }
        let mutation_request =
            match validate_and_parse_upload(&prepared_request, &operation_name, &upload) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        let operation_request =
            match self.build_upload_operation_request(&prepared_request, &operation_name) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        let operation_admission =
            match self.admit_declared_upload_operation(&prepared_request, &operation_request) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        TransitionOutcome::Success(WorthServerPreparedMultipartUpload::from_parts(
            prepared_request,
            operation_name,
            upload,
            mutation_request,
            operation_admission,
        ))
    }

    fn admit_binary_family(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
    ) -> Result<(), WorthServerQueryHandoffDenial> {
        self.admit_operation_family_for_query(
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            WorthServerOperationFamily::BinaryTransfer,
        )
    }

    fn admit_query_direct_family(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
    ) -> Result<(), WorthServerQueryHandoffDenial> {
        self.admit_operation_family_for_query(
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            WorthServerOperationFamily::QueryDirectSubmission,
        )
    }

    fn build_upload_operation_request(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        operation_name: &str,
    ) -> Result<crate::WorthServerOperationRequest, WorthServerQueryHandoffDenial> {
        match WorthServerOperationRequestFacade::new(self.operation_registry.clone())
            .admit_from_compat_http(
                prepared_request,
                WorthServerOperationFamily::BinaryTransfer,
                operation_name,
                None,
            ) {
            Ok(value) => Ok(value),
            Err(denial) => Err(WorthServerQueryHandoffDenial::new(
                crate::WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                denial.diagnostics_profile(),
                denial.detail(),
            )),
        }
    }

    fn admit_declared_upload_operation(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        operation_request: &crate::WorthServerOperationRequest,
    ) -> Result<crate::WorthServerOperationAdmissionPosture, WorthServerQueryHandoffDenial> {
        WorthServerOperationAdmissionFacade::with_operation_registry(
            self.operation_registry.clone(),
        )
        .admit_declared(prepared_request.admission(), operation_request)
        .map_err(crate::surfaces::compat_http::map_operation_admission_denial)
    }
}

fn validate_and_parse_upload(
    prepared_request: &WorthServerCompatibilityPreparedRequest,
    operation_name: &str,
    upload: &WorthServerMultipartUpload,
) -> Result<crate::WorthServerCompatibilityMutationRequest, WorthServerQueryHandoffDenial> {
    validate_upload_admission(prepared_request, upload, operation_name)?;
    parse_upload_mutation_request(
        upload,
        prepared_request
            .admission()
            .request_context()
            .diagnostics_profile(),
    )
}
