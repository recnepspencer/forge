use worth_query_decl::facade::application_query::{
    ApplicationQueryResultShapeBuilder, TypedApplicationQueryResultShape,
};

use crate::{
    reads::EstateGovernanceContext,
    schema::{
        BankSchema, CapabilityGrant, EmergencyAccess, EmployeeAssignment, EstateCase, Principal,
    },
};

use super::{governance::EstateGovernanceQuery, governance_fields::*, governance_relations::*};

pub(super) fn governance_shape() -> TypedApplicationQueryResultShape<
    BankSchema,
    EstateGovernanceQuery,
    EstateCase,
    EstateGovernanceContext,
> {
    let beneficiary = principal_shape(beneficiary());
    let assignment_principal_shape = principal_shape(assignment_principal_identity());
    let grantee = principal_shape(capability_grantee_identity());
    let grantor = principal_shape(capability_grantor_identity());
    let requester = principal_shape(emergency_requester_identity());
    let approver = principal_shape(emergency_approver_identity());
    let assignment = ApplicationQueryResultShapeBuilder::<
        BankSchema,
        EstateGovernanceQuery,
        EmployeeAssignment,
        (),
    >::new(EmployeeAssignment::reference())
    .field(assignment_id())
    .field(assignment_role())
    .relation(assignment_principal(), assignment_principal_shape);
    let emergency = ApplicationQueryResultShapeBuilder::<
        BankSchema,
        EstateGovernanceQuery,
        EmergencyAccess,
        (),
    >::new(EmergencyAccess::reference())
    .field(emergency_id())
    .field(emergency_reason())
    .field(emergency_status())
    .relation(emergency_requester(), requester)
    .relation(emergency_approver(), approver);
    let capability = ApplicationQueryResultShapeBuilder::<
        BankSchema,
        EstateGovernanceQuery,
        CapabilityGrant,
        (),
    >::new(CapabilityGrant::reference())
    .field(capability_id())
    .field(capability_operation())
    .field(capability_purpose())
    .field(capability_valid_from())
    .field(capability_valid_through())
    .field(capability_delegation())
    .field(capability_workflow())
    .field(capability_status())
    .relation(capability_grantee(), grantee)
    .relation(capability_grantor(), grantor)
    .relation(capability_emergencies(), emergency);

    ApplicationQueryResultShapeBuilder::<
        BankSchema,
        EstateGovernanceQuery,
        EstateCase,
        EstateGovernanceContext,
    >::new(EstateCase::reference())
    .field(estate_id())
    .field(estate_stage())
    .relation(estate_beneficiaries(), beneficiary)
    .relation(estate_assignments(), assignment)
    .relation(estate_capabilities(), capability)
    .build()
}

fn principal_shape<Slot: 'static>(
    selector: worth_query_decl::facade::application_query::ApplicationQueryResultFieldRef<
        EstateGovernanceQuery,
        Slot,
        BankSchema,
        Principal,
        crate::schema::PrincipalIdentity,
        crate::schema::PrincipalIdentityField,
        crate::model::BankPrincipalId,
        worth_query_decl::facade::application_schema::ReadOnly,
        worth_query_decl::facade::application_schema::EqualityPredicate,
        worth_query_decl::facade::application_schema::NoApplicationCurrency,
    >,
) -> ApplicationQueryResultShapeBuilder<BankSchema, EstateGovernanceQuery, Principal, ()> {
    ApplicationQueryResultShapeBuilder::new(Principal::reference()).field(selector)
}
