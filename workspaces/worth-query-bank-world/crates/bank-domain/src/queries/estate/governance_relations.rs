use worth_query_decl::facade::application_query::{
    ApplicationQueryResultRelationRef, ExactlyOneResult, ForwardResultTraversal, ManyResults,
    OptionalOneResult, ReverseResultTraversal,
};

use crate::schema::{
    AssignmentPrincipal, BankSchema, CapabilityEstate, CapabilityGrant, CapabilityGrantee,
    CapabilityGrantor, EmergencyAccess, EmergencyApprover, EmergencyGrant, EmergencyRequester,
    EmployeeAssignment, EstateAssignment, EstateBeneficiary, EstateCase, Principal,
};

use super::governance::EstateGovernanceQuery;

pub(super) struct BeneficiariesSlot;
pub(super) struct AssignmentsSlot;
pub(super) struct AssignmentPrincipalSlot;
pub(super) struct CapabilitiesSlot;
pub(super) struct CapabilityGranteeSlot;
pub(super) struct CapabilityGrantorSlot;
pub(super) struct EmergenciesSlot;
pub(super) struct EmergencyRequesterSlot;
pub(super) struct EmergencyApproverSlot;

macro_rules! reverse_many {
    ($name:ident, $slot:ty, $relation:ty, $from:ty, $to:ty, $alias:literal) => {
        pub(super) fn $name() -> ApplicationQueryResultRelationRef<
            EstateGovernanceQuery,
            $slot,
            BankSchema,
            $relation,
            $from,
            $to,
            ReverseResultTraversal,
            ManyResults,
        > {
            ApplicationQueryResultRelationRef::reverse_many($alias, <$relation>::reference())
        }
    };
}

macro_rules! reverse_one {
    ($name:ident, $slot:ty, $relation:ty, $from:ty, $to:ty, $cardinality:ty, $method:ident, $alias:literal) => {
        pub(super) fn $name() -> ApplicationQueryResultRelationRef<
            EstateGovernanceQuery,
            $slot,
            BankSchema,
            $relation,
            $from,
            $to,
            ReverseResultTraversal,
            $cardinality,
        > {
            ApplicationQueryResultRelationRef::$method($alias, <$relation>::reference())
        }
    };
}

reverse_many!(
    estate_beneficiaries,
    BeneficiariesSlot,
    EstateBeneficiary,
    Principal,
    EstateCase,
    "beneficiaries"
);
reverse_many!(
    estate_assignments,
    AssignmentsSlot,
    EstateAssignment,
    EmployeeAssignment,
    EstateCase,
    "assignments"
);
reverse_many!(
    estate_capabilities,
    CapabilitiesSlot,
    CapabilityEstate,
    CapabilityGrant,
    EstateCase,
    "capabilities"
);
reverse_many!(
    capability_emergencies,
    EmergenciesSlot,
    EmergencyGrant,
    EmergencyAccess,
    CapabilityGrant,
    "emergencies"
);
reverse_one!(
    capability_grantee,
    CapabilityGranteeSlot,
    CapabilityGrantee,
    Principal,
    CapabilityGrant,
    ExactlyOneResult,
    reverse_one,
    "grantee"
);
reverse_one!(
    capability_grantor,
    CapabilityGrantorSlot,
    CapabilityGrantor,
    Principal,
    CapabilityGrant,
    ExactlyOneResult,
    reverse_one,
    "grantor"
);
reverse_one!(
    emergency_requester,
    EmergencyRequesterSlot,
    EmergencyRequester,
    Principal,
    EmergencyAccess,
    ExactlyOneResult,
    reverse_one,
    "requester"
);
reverse_one!(
    emergency_approver,
    EmergencyApproverSlot,
    EmergencyApprover,
    Principal,
    EmergencyAccess,
    OptionalOneResult,
    reverse_optional,
    "approver"
);

pub(super) fn assignment_principal() -> ApplicationQueryResultRelationRef<
    EstateGovernanceQuery,
    AssignmentPrincipalSlot,
    BankSchema,
    AssignmentPrincipal,
    EmployeeAssignment,
    Principal,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("principal", AssignmentPrincipal::reference())
}
