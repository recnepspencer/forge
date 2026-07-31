use worth_query_decl::facade::application_capability::{
    ApplicationCapabilityEntitySelector, ApplicationCapabilityRelatedEntitySelector,
    ApplicationCapabilityRequest, ApplicationCapabilityRequestContext,
    ApplicationCapabilityRequestProjection, ApplicationCapabilityRequestProjectionDenial,
};

use crate::{
    estate::EstateAction,
    schema::{
        AccountIdentity, BankSchema, CapabilityAccount, DisburseEstateCapability,
        EstateActionContext, EstateCase, EstateCaseIdentityField, EstateLegalAuthoritySlot,
        FreezeEstateAccountCapability, LegalAuthorityIdentityField,
        RecognizeEstateExecutorCapability, ViewEstateAdministrationCapability,
        ViewEstateEmergencyProtectionCapability, ViewEstateIdentityVerificationCapability,
        ViewEstateLegalComplianceCapability, ViewEstateMandatoryReviewCapability,
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
                ApplicationCapabilityRequestProjection<BankSchema, Self::Scope, Self::Context>,
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

impl ApplicationCapabilityRequest<BankSchema, FreezeEstateAccountCapability> for EstateAction {
    type Scope = EstateCase;
    type Context = EstateActionContext;

    fn capability_request(
        &self,
    ) -> Result<
        ApplicationCapabilityRequestProjection<BankSchema, Self::Scope, Self::Context>,
        ApplicationCapabilityRequestProjectionDenial,
    > {
        let EstateAction::FreezeAccount { estate, account } = *self else {
            return Err(ApplicationCapabilityRequestProjectionDenial::input_variant(
                "FreezeEstateAccountOperation",
            ));
        };
        Ok(estate_request(self, estate).related_entity(
            ApplicationCapabilityRelatedEntitySelector::new(
                CapabilityAccount::reference(),
                ApplicationCapabilityEntitySelector::new(AccountIdentity::reference(), account),
            ),
        ))
    }
}

impl ApplicationCapabilityRequest<BankSchema, DisburseEstateCapability> for EstateAction {
    type Scope = EstateCase;
    type Context = EstateActionContext;

    fn capability_request(
        &self,
    ) -> Result<
        ApplicationCapabilityRequestProjection<BankSchema, Self::Scope, Self::Context>,
        ApplicationCapabilityRequestProjectionDenial,
    > {
        let EstateAction::DisburseEstate(disbursement) = *self else {
            return Err(ApplicationCapabilityRequestProjectionDenial::input_variant(
                "DisburseEstateOperation",
            ));
        };
        Ok(estate_request(self, disbursement.estate)
            .related_entity(ApplicationCapabilityRelatedEntitySelector::new(
                CapabilityAccount::reference(),
                ApplicationCapabilityEntitySelector::new(
                    AccountIdentity::reference(),
                    disbursement.source_account,
                ),
            ))
            .amount(disbursement.amount))
    }
}

impl ApplicationCapabilityRequest<BankSchema, RecognizeEstateExecutorCapability> for EstateAction {
    type Scope = EstateCase;
    type Context = EstateActionContext;

    fn capability_request(
        &self,
    ) -> Result<
        ApplicationCapabilityRequestProjection<BankSchema, Self::Scope, Self::Context>,
        ApplicationCapabilityRequestProjectionDenial,
    > {
        let EstateAction::RecognizeExecutor {
            estate, authority, ..
        } = *self
        else {
            return Err(ApplicationCapabilityRequestProjectionDenial::input_variant(
                "RecognizeEstateExecutorOperation",
            ));
        };
        Ok(ApplicationCapabilityRequestProjection::new(
            ApplicationCapabilityEntitySelector::new(EstateCaseIdentityField::reference(), estate),
            self.operation(),
            self.purpose(),
            ApplicationCapabilityRequestContext::new(EstateActionContext::reference()).entity(
                EstateLegalAuthoritySlot::reference(),
                ApplicationCapabilityEntitySelector::new(
                    LegalAuthorityIdentityField::reference(),
                    authority,
                ),
            ),
        ))
    }
}

fn estate_request(
    action: &EstateAction,
    estate: crate::estate::EstateCaseId,
) -> ApplicationCapabilityRequestProjection<BankSchema, EstateCase, EstateActionContext> {
    ApplicationCapabilityRequestProjection::new(
        ApplicationCapabilityEntitySelector::new(EstateCaseIdentityField::reference(), estate),
        action.operation(),
        action.purpose(),
        ApplicationCapabilityRequestContext::new(EstateActionContext::reference()),
    )
}
