use forge_proof::TransitionOutcome;

use crate::{
    ForgeServerAdmission, ForgeServerAdmittedDirectDeclaration,
    ForgeServerCompatibilityPreparedRequest, ForgeServerDirectDeclaration,
    ForgeServerDirectDeclarationDenial, ForgeServerQueryHandoffDeferred,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffFailure, ForgeServerQueryHandoffRebindRequired,
    ForgeServerQueryHandoffStale,
};

pub type ForgeServerCompatibilityExecutionOutcome<T> = TransitionOutcome<
    T,
    ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDeferred,
    ForgeServerQueryHandoffStale,
    ForgeServerQueryHandoffRebindRequired,
    ForgeServerQueryHandoffFailure,
>;

#[derive(Clone, Debug)]
pub struct ForgeServerCompatibilityExecutionInput {
    prepared_request: ForgeServerCompatibilityPreparedRequest,
    operation_name: String,
}

impl ForgeServerCompatibilityExecutionInput {
    pub fn new(
        prepared_request: ForgeServerCompatibilityPreparedRequest,
        operation_name: impl Into<String>,
    ) -> Self {
        Self {
            prepared_request,
            operation_name: operation_name.into().trim().to_string(),
        }
    }

    pub fn prepared_request(&self) -> &ForgeServerCompatibilityPreparedRequest {
        &self.prepared_request
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub(crate) fn into_parts(self) -> (ForgeServerCompatibilityPreparedRequest, String) {
        (self.prepared_request, self.operation_name)
    }
}

pub(crate) fn named_read_declaration(operation_name: &str) -> ForgeServerDirectDeclaration {
    ForgeServerDirectDeclaration::named_read(operation_name)
}

pub(crate) fn admit_declaration(
    intake: &crate::declaration_intake::ForgeServerDirectDeclarationIntakeFacade,
    admission: ForgeServerAdmission,
    declaration: ForgeServerDirectDeclaration,
) -> Result<ForgeServerAdmittedDirectDeclaration, ForgeServerQueryHandoffDenial> {
    let prepared = intake
        .prepare(admission.clone(), declaration)
        .map_err(|denial| map_declaration_denial(&admission, denial))?;
    prepared
        .admit()
        .map_err(|denial| map_declaration_denial(&admission, denial))
}

fn map_declaration_denial(
    admission: &ForgeServerAdmission,
    denial: ForgeServerDirectDeclarationDenial,
) -> ForgeServerQueryHandoffDenial {
    let diagnostics = admission.request_context().diagnostics_profile();
    match denial {
        ForgeServerDirectDeclarationDenial::InvalidDeclarationIdentity { detail, .. } => {
            ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
                diagnostics,
                detail,
            )
        }
        ForgeServerDirectDeclarationDenial::WorkspaceBindingFailed { detail, .. } => {
            ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::WorkspaceBindingFailed,
                diagnostics,
                detail,
            )
        }
        ForgeServerDirectDeclarationDenial::SourceNotAdmitted { detail, .. } => {
            ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::DirectDeclarationSourceNotAdmitted,
                diagnostics,
                detail,
            )
        }
        ForgeServerDirectDeclarationDenial::QueryFacadeFamilyNotAdmitted { detail, .. } => {
            ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily,
                diagnostics,
                detail,
            )
        }
    }
}
