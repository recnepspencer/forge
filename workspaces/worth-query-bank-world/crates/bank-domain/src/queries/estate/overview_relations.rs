use worth_query_decl::facade::application_query::{
    ApplicationQueryResultRelationRef, ExactlyOneResult, ForwardResultTraversal, ManyResults,
    OptionalOneResult, ReverseResultTraversal,
};

use crate::schema::{
    Account, AssignmentPrincipal, BankSchema, Branch, DeathNotice, EmployeeAssignment,
    EstateAccount, EstateAssignment, EstateBeneficiary, EstateBranch, EstateCase,
    EstateDeathNotice, EstateDeceased, EstateExecutor, LegalAuthority, LegalAuthorityEstate,
    LegalAuthorityHolder, MandatoryReview, Principal, ReviewEstate, ReviewPrincipal,
};

use super::overview::EstateCaseOverviewQuery;

pub(super) struct AccountSlot;
pub(super) struct BranchSlot;
pub(super) struct NoticeSlot;
pub(super) struct DeceasedSlot;
pub(super) struct ExecutorsSlot;
pub(super) struct BeneficiariesSlot;
pub(super) struct AssignmentsSlot;
pub(super) struct AssignmentPrincipalSlot;
pub(super) struct AuthoritiesSlot;
pub(super) struct AuthorityHolderSlot;
pub(super) struct ReviewsSlot;
pub(super) struct ReviewPrincipalSlot;

pub(super) fn estate_account() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    AccountSlot,
    BankSchema,
    EstateAccount,
    EstateCase,
    Account,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("account", EstateAccount::reference())
}

pub(super) fn estate_branch() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    BranchSlot,
    BankSchema,
    EstateBranch,
    EstateCase,
    Branch,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("branch", EstateBranch::reference())
}

pub(super) fn estate_notice() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    NoticeSlot,
    BankSchema,
    EstateDeathNotice,
    EstateCase,
    DeathNotice,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("death_notice", EstateDeathNotice::reference())
}

pub(super) fn estate_deceased() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    DeceasedSlot,
    BankSchema,
    EstateDeceased,
    EstateCase,
    Principal,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("deceased", EstateDeceased::reference())
}

pub(super) fn estate_executors() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    ExecutorsSlot,
    BankSchema,
    EstateExecutor,
    Principal,
    EstateCase,
    ReverseResultTraversal,
    ManyResults,
> {
    ApplicationQueryResultRelationRef::reverse_many("executors", EstateExecutor::reference())
}

pub(super) fn estate_beneficiaries() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    BeneficiariesSlot,
    BankSchema,
    EstateBeneficiary,
    Principal,
    EstateCase,
    ReverseResultTraversal,
    ManyResults,
> {
    ApplicationQueryResultRelationRef::reverse_many("beneficiaries", EstateBeneficiary::reference())
}

pub(super) fn estate_assignments() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    AssignmentsSlot,
    BankSchema,
    EstateAssignment,
    EmployeeAssignment,
    EstateCase,
    ReverseResultTraversal,
    ManyResults,
> {
    ApplicationQueryResultRelationRef::reverse_many("assignments", EstateAssignment::reference())
}

pub(super) fn assignment_principal() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
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

pub(super) fn estate_authorities() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    AuthoritiesSlot,
    BankSchema,
    LegalAuthorityEstate,
    LegalAuthority,
    EstateCase,
    ReverseResultTraversal,
    ManyResults,
> {
    ApplicationQueryResultRelationRef::reverse_many(
        "legal_authorities",
        LegalAuthorityEstate::reference(),
    )
}

pub(super) fn authority_holder() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    AuthorityHolderSlot,
    BankSchema,
    LegalAuthorityHolder,
    LegalAuthority,
    Principal,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("holder", LegalAuthorityHolder::reference())
}

pub(super) fn estate_reviews() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    ReviewsSlot,
    BankSchema,
    ReviewEstate,
    MandatoryReview,
    EstateCase,
    ReverseResultTraversal,
    ManyResults,
> {
    ApplicationQueryResultRelationRef::reverse_many("reviews", ReviewEstate::reference())
}

pub(super) fn review_principal() -> ApplicationQueryResultRelationRef<
    EstateCaseOverviewQuery,
    ReviewPrincipalSlot,
    BankSchema,
    ReviewPrincipal,
    Principal,
    MandatoryReview,
    ReverseResultTraversal,
    OptionalOneResult,
> {
    ApplicationQueryResultRelationRef::reverse_optional("reviewer", ReviewPrincipal::reference())
}
