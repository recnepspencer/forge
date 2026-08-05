use worth_query_decl::facade::application_query::{
    ApplicationQueryResultRelationRef, ExactlyOneResult, ForwardResultTraversal, ManyResults,
    OptionalOneResult, ReverseResultTraversal,
};

use crate::schema::{
    Account, AssignmentPrincipal, BankSchema, Branch, CapabilityAccount, CapabilityBranch,
    CapabilityEstate, CapabilityGrant, CapabilityGrantee, CapabilityGrantor, CapabilityInstitution,
    CapabilityParent, EmergencyAccess, EmergencyApprover, EmergencyGrant, EmergencyRequester,
    EmergencyReview, EmployeeAssignment, EstateAssignment, EstateBeneficiary, EstateCase,
    Institution, MandatoryReview, Principal, ReviewEstate, ReviewPrincipal,
};

use super::governance::EstateGovernanceQuery;

pub(super) struct BeneficiariesSlot;
pub(super) struct AssignmentsSlot;
pub(super) struct AssignmentPrincipalSlot;
pub(super) struct CapabilitiesSlot;
pub(super) struct CapabilityGranteeSlot;
pub(super) struct CapabilityGrantorSlot;
pub(super) struct CapabilityAccountSlot;
pub(super) struct CapabilityInstitutionSlot;
pub(super) struct CapabilityBranchSlot;
pub(super) struct CapabilityParentSlot;
pub(super) struct EmergenciesSlot;
pub(super) struct EmergencyRequesterSlot;
pub(super) struct EmergencyApproverSlot;
pub(super) struct EmergencyReviewSlot;
pub(super) struct ReviewEstateSlot;
pub(super) struct ReviewReviewerSlot;

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

pub(super) fn capability_account() -> ApplicationQueryResultRelationRef<
    EstateGovernanceQuery,
    CapabilityAccountSlot,
    BankSchema,
    CapabilityAccount,
    CapabilityGrant,
    Account,
    ForwardResultTraversal,
    OptionalOneResult,
> {
    ApplicationQueryResultRelationRef::forward_optional("account", CapabilityAccount::reference())
}

pub(super) fn capability_institution() -> ApplicationQueryResultRelationRef<
    EstateGovernanceQuery,
    CapabilityInstitutionSlot,
    BankSchema,
    CapabilityInstitution,
    CapabilityGrant,
    Institution,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one(
        "institution",
        CapabilityInstitution::reference(),
    )
}

pub(super) fn capability_branch() -> ApplicationQueryResultRelationRef<
    EstateGovernanceQuery,
    CapabilityBranchSlot,
    BankSchema,
    CapabilityBranch,
    CapabilityGrant,
    Branch,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("branch", CapabilityBranch::reference())
}

pub(super) fn capability_parent() -> ApplicationQueryResultRelationRef<
    EstateGovernanceQuery,
    CapabilityParentSlot,
    BankSchema,
    CapabilityParent,
    CapabilityGrant,
    CapabilityGrant,
    ForwardResultTraversal,
    OptionalOneResult,
> {
    ApplicationQueryResultRelationRef::forward_optional("parent", CapabilityParent::reference())
}

pub(super) fn emergency_review() -> ApplicationQueryResultRelationRef<
    EstateGovernanceQuery,
    EmergencyReviewSlot,
    BankSchema,
    EmergencyReview,
    EmergencyAccess,
    MandatoryReview,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("review", EmergencyReview::reference())
}

pub(super) fn review_estate() -> ApplicationQueryResultRelationRef<
    EstateGovernanceQuery,
    ReviewEstateSlot,
    BankSchema,
    ReviewEstate,
    MandatoryReview,
    EstateCase,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("estate", ReviewEstate::reference())
}

pub(super) fn review_reviewer() -> ApplicationQueryResultRelationRef<
    EstateGovernanceQuery,
    ReviewReviewerSlot,
    BankSchema,
    ReviewPrincipal,
    Principal,
    MandatoryReview,
    ReverseResultTraversal,
    OptionalOneResult,
> {
    ApplicationQueryResultRelationRef::reverse_optional("reviewer", ReviewPrincipal::reference())
}
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
