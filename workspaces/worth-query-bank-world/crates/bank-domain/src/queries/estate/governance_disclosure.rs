use worth_query_decl::facade::application_query::{
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
};

use crate::{estate::RestrictedBankField, schema::ViewEstateAdministrationCapability};

use super::{governance_fields::*, governance_relations::*};

pub(super) fn governance_disclosure() -> ApplicationQueryDisclosureContract {
    let field = RestrictedBankField::GovernanceMetadata;
    let influence = ApplicationQueryInfluenceContract::forbid_all();
    ApplicationQueryDisclosureContract::governed_by(
        "estate-governance-context",
        ViewEstateAdministrationCapability::reference(),
    )
    .disclose_field_by(estate_id(), field, influence.clone())
    .disclose_field_by(estate_stage(), field, influence.clone())
    .disclose_relation_by(estate_beneficiaries(), field, influence.clone())
    .disclose_field_by(beneficiary(), field, influence.clone())
    .disclose_relation_by(estate_assignments(), field, influence.clone())
    .disclose_field_by(assignment_id(), field, influence.clone())
    .disclose_field_by(assignment_role(), field, influence.clone())
    .disclose_relation_by(assignment_principal(), field, influence.clone())
    .disclose_field_by(assignment_principal_identity(), field, influence.clone())
    .disclose_relation_by(estate_capabilities(), field, influence.clone())
    .disclose_field_by(capability_id(), field, influence.clone())
    .disclose_field_by(capability_operation(), field, influence.clone())
    .disclose_field_by(capability_purpose(), field, influence.clone())
    .disclose_field_by(capability_valid_from(), field, influence.clone())
    .disclose_field_by(capability_valid_through(), field, influence.clone())
    .disclose_field_by(capability_delegation(), field, influence.clone())
    .disclose_field_by(capability_workflow(), field, influence.clone())
    .disclose_field_by(capability_status(), field, influence.clone())
    .disclose_relation_by(capability_grantee(), field, influence.clone())
    .disclose_field_by(capability_grantee_identity(), field, influence.clone())
    .disclose_relation_by(capability_grantor(), field, influence.clone())
    .disclose_field_by(capability_grantor_identity(), field, influence.clone())
    .disclose_relation_by(capability_emergencies(), field, influence.clone())
    .disclose_field_by(emergency_id(), field, influence.clone())
    .disclose_field_by(emergency_reason(), field, influence.clone())
    .disclose_field_by(emergency_status(), field, influence.clone())
    .disclose_relation_by(emergency_requester(), field, influence.clone())
    .disclose_field_by(emergency_requester_identity(), field, influence.clone())
    .disclose_relation_by(emergency_approver(), field, influence.clone())
    .disclose_field_by(emergency_approver_identity(), field, influence)
}
