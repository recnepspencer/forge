use worth_query_decl::facade::application_capability::{
    ApplicationCapabilityEntitySelector, ApplicationCapabilityRequest,
    ApplicationCapabilityRequestContext, ApplicationCapabilityRequestProjection,
    ApplicationCapabilityRequestProjectionDenial,
};

use crate::{
    estate::EstateAction,
    schema::{
        BankSchema, EstateActionContext, EstateCase, EstateCaseIdentityField,
        ViewEstateAdministrationCapability, ViewEstateEmergencyProtectionCapability,
        ViewEstateIdentityVerificationCapability, ViewEstateLegalComplianceCapability,
        ViewEstateMandatoryReviewCapability,
    },
};

macro_rules! view_request {
    ($capability:ty) => {
        impl ApplicationCapabilityRequest<BankSchema, $capability> for EstateAction {
            type Scope = EstateCase;
            type Context = EstateActionContext;

            fn capability_request(
                &self,
            ) -> Result<
                ApplicationCapabilityRequestProjection<
                    BankSchema,
                    Self::Scope,
                    Self::Context,
                >,
                ApplicationCapabilityRequestProjectionDenial,
            > {
                let EstateAction::ViewRestrictedEstate {
                    estate,
                    field,
                    purpose,
                } = *self
                else {
                    return Err(ApplicationCapabilityRequestProjectionDenial::input_variant(
                        "ViewRestrictedEstateOperation",
                    ));
                };
                Ok(ApplicationCapabilityRequestProjection::new(
                    ApplicationCapabilityEntitySelector::new(
                        EstateCaseIdentityField::reference(),
                        estate,
                    ),
                    self.operation(),
                    purpose,
                    ApplicationCapabilityRequestContext::new(EstateActionContext::reference()),
                )
                .field(field))
            }
        }
    };
}

view_request!(ViewEstateAdministrationCapability);
view_request!(ViewEstateIdentityVerificationCapability);
view_request!(ViewEstateLegalComplianceCapability);
view_request!(ViewEstateEmergencyProtectionCapability);
view_request!(ViewEstateMandatoryReviewCapability);
