use worth_query_decl::facade::{
    application_capability::{
        ApplicationCapabilityAmountDimension, ApplicationCapabilityCardinalityDimension,
        ApplicationCapabilityConstraintDefinition, ApplicationCapabilityContractBuilder,
        ApplicationCapabilityCurrentnessDefinition, ApplicationCapabilityDelegationDefinition,
        ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
        ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
        ApplicationCapabilityTargetDefinition, ApplicationCapabilityValidityDefinition,
        ApplicationCapabilityValidityTimeline, ApplicationCapabilityValueBinding,
        ApplicationCapabilityWorkflowDefinition,
    },
    application_schema::ApplicationSchemaDeclarationBuilder,
};

use super::*;
use crate::{
    estate::{CapabilityGrantStatus, EstateCapabilityOperation, EstateCapabilityPurpose},
    schema::BankSchema,
};

macro_rules! relation_dimension {
    (account_relation) => {
        ApplicationCapabilityRelationDimension::bound(CapabilityAccount::reference())
    };
    (no_relation) => {
        ApplicationCapabilityRelationDimension::not_applicable()
    };
}

macro_rules! field_dimension {
    (field) => {
        ApplicationCapabilityFieldDimension::bound(CapabilityDisclosureField::reference())
    };
    (no_field) => {
        ApplicationCapabilityFieldDimension::not_applicable()
    };
}

macro_rules! amount_dimension {
    (amount) => {
        ApplicationCapabilityAmountDimension::bound(CapabilityAmountCeilingField::reference())
    };
    (no_amount) => {
        ApplicationCapabilityAmountDimension::not_applicable()
    };
}

macro_rules! install_contract {
    (
        $schema:expr,
        $capability:ident,
        $operation:ident,
        $action:expr,
        $purpose:expr,
        $relation:ident,
        $field:ident,
        $amount:ident
    ) => {{
        let contract = ApplicationCapabilityContractBuilder::new(
            $capability::reference(),
            $operation::reference(),
            CapabilityGrant::reference(),
        )
        .target(target(
            $action,
            $purpose,
            relation_dimension!($relation),
            field_dimension!($field),
        ))
        .constraints(constraints(amount_dimension!($amount)))
        .delegation(delegation())
        .composition(super::capability_composition::composition(
            $action, $purpose,
        ))
        .build();
        $schema.capability(contract)
    }};
}

macro_rules! install_view_contract {
    ($schema:expr, $capability:ident, $purpose:expr) => {
        install_contract!(
            $schema,
            $capability,
            ViewRestrictedEstateOperation,
            EstateCapabilityOperation::ViewRestrictedEstate,
            $purpose,
            no_relation,
            field,
            no_amount
        )
    };
}

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    let schema = install_capability_dimensions(install_operations(schema));
    let schema = install_contract!(
        schema,
        NotifyDeathEstateCapability,
        NotifyDeathEstateOperation,
        EstateCapabilityOperation::NotifyDeath,
        EstateCapabilityPurpose::EstateAdministration,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        FreezeEstateAccountCapability,
        FreezeEstateAccountOperation,
        EstateCapabilityOperation::FreezeAccount,
        EstateCapabilityPurpose::EstateAdministration,
        account_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        OpenEstateCaseCapability,
        OpenEstateCaseOperation,
        EstateCapabilityOperation::OpenEstateCase,
        EstateCapabilityPurpose::EstateAdministration,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        RecognizeEstateExecutorCapability,
        RecognizeEstateExecutorOperation,
        EstateCapabilityOperation::RecognizeExecutor,
        EstateCapabilityPurpose::LegalCompliance,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        DelegateEstateCapability,
        DelegateEstateCapabilityOperation,
        EstateCapabilityOperation::DelegateCapability,
        EstateCapabilityPurpose::EstateAdministration,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        RevokeEstateCapability,
        RevokeEstateCapabilityOperation,
        EstateCapabilityOperation::RevokeCapability,
        EstateCapabilityPurpose::EstateAdministration,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        RequestEstateEmergencyAccessCapability,
        RequestEstateEmergencyAccessOperation,
        EstateCapabilityOperation::RequestEmergencyAccess,
        EstateCapabilityPurpose::EmergencyProtection,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        ApproveEstateEmergencyAccessCapability,
        ApproveEstateEmergencyAccessOperation,
        EstateCapabilityOperation::ApproveEmergencyAccess,
        EstateCapabilityPurpose::EmergencyProtection,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        RevokeEstateEmergencyAccessCapability,
        RevokeEstateEmergencyAccessOperation,
        EstateCapabilityOperation::RevokeEmergencyAccess,
        EstateCapabilityPurpose::EmergencyProtection,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        CompleteEstateMandatoryReviewCapability,
        CompleteEstateMandatoryReviewOperation,
        EstateCapabilityOperation::CompleteMandatoryReview,
        EstateCapabilityPurpose::MandatoryReview,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        ReleaseEstateCapability,
        ReleaseEstateOperation,
        EstateCapabilityOperation::ReleaseEstate,
        EstateCapabilityPurpose::EstateAdministration,
        no_relation,
        no_field,
        no_amount
    );
    let schema = install_contract!(
        schema,
        DisburseEstateCapability,
        DisburseEstateOperation,
        EstateCapabilityOperation::DisburseEstate,
        EstateCapabilityPurpose::EstateDisbursement,
        account_relation,
        no_field,
        amount
    );
    install_view_contracts(schema)
}

fn install_capability_dimensions(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema
        .capability_context(EstateActionContext::reference())
        .capability_context_entity_slot(EstateLegalAuthoritySlot::reference())
        .capability_context_entity_slot(EstateEmergencyAccessSlot::reference())
        .capability_context_entity_slot(EstateMandatoryReviewSlot::reference())
        .capability_provenance(EstateGrantChainProvenance::reference())
}

fn install_operations(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema
        .operation(NotifyDeathEstateOperation::reference())
        .operation(FreezeEstateAccountOperation::reference())
        .operation(OpenEstateCaseOperation::reference())
        .operation(RecognizeEstateExecutorOperation::reference())
        .operation(DelegateEstateCapabilityOperation::reference())
        .operation(RevokeEstateCapabilityOperation::reference())
        .operation(RequestEstateEmergencyAccessOperation::reference())
        .operation(ApproveEstateEmergencyAccessOperation::reference())
        .operation(RevokeEstateEmergencyAccessOperation::reference())
        .operation(CompleteEstateMandatoryReviewOperation::reference())
        .operation(ReleaseEstateOperation::reference())
        .operation(DisburseEstateOperation::reference())
        .operation(ViewRestrictedEstateOperation::reference())
}

fn install_view_contracts(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    let schema = install_view_contract!(
        schema,
        ViewEstateAdministrationCapability,
        EstateCapabilityPurpose::EstateAdministration
    );
    let schema = install_view_contract!(
        schema,
        ViewEstateIdentityVerificationCapability,
        EstateCapabilityPurpose::IdentityVerification
    );
    let schema = install_view_contract!(
        schema,
        ViewEstateLegalComplianceCapability,
        EstateCapabilityPurpose::LegalCompliance
    );
    let schema = install_view_contract!(
        schema,
        ViewEstateEmergencyProtectionCapability,
        EstateCapabilityPurpose::EmergencyProtection
    );
    install_view_contract!(
        schema,
        ViewEstateMandatoryReviewCapability,
        EstateCapabilityPurpose::MandatoryReview
    )
}

fn target(
    action: EstateCapabilityOperation,
    purpose: EstateCapabilityPurpose,
    relation: ApplicationCapabilityRelationDimension,
    field: ApplicationCapabilityFieldDimension,
) -> ApplicationCapabilityTargetDefinition {
    ApplicationCapabilityTargetDefinition::new(
        ApplicationCapabilityValueBinding::new(CapabilityOperationField::reference(), action),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityEstate::reference()),
        relation,
        field,
        ApplicationCapabilityValueBinding::new(CapabilityPurposeField::reference(), purpose),
    )
}

fn constraints(
    amount: ApplicationCapabilityAmountDimension,
) -> ApplicationCapabilityConstraintDefinition {
    ApplicationCapabilityConstraintDefinition::new(
        amount,
        ApplicationCapabilityCardinalityDimension::One,
        ApplicationCapabilityCurrentnessDefinition::new(
            ApplicationCapabilityValueBinding::new(
                CapabilityGrantStatusField::reference(),
                CapabilityGrantStatus::Active,
            ),
            ApplicationCapabilityWorkflowDefinition::new(
                ApplicationCapabilityFieldBinding::from_reference(
                    CapabilityWorkflowStageField::reference(),
                ),
                ApplicationCapabilityFieldBinding::from_reference(
                    EstateWorkflowStageField::reference(),
                ),
            ),
            ApplicationCapabilityValidityDefinition::new(
                ApplicationCapabilityValidityTimeline::UnixEpochSeconds,
                ApplicationCapabilityFieldBinding::from_reference(
                    CapabilityValidFromField::reference(),
                ),
                ApplicationCapabilityFieldBinding::from_reference(
                    CapabilityValidThroughField::reference(),
                ),
            ),
        ),
        EstateActionContext::reference(),
    )
}

fn delegation() -> ApplicationCapabilityDelegationDefinition {
    ApplicationCapabilityDelegationDefinition::new(
        ApplicationCapabilityRelationBinding::from_reference(CapabilityParent::reference()),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityGrantor::reference()),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityGrantee::reference()),
        ApplicationCapabilityFieldBinding::from_reference(
            CapabilityDelegationLimitField::reference(),
        ),
        EstateGrantChainProvenance::reference(),
    )
}
