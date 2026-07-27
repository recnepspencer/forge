use worth_query_decl::facade::worth_query_relation;

use super::entities::{
    Account, AccountAuthorization, Approval, Business, Customer, EmployeeAssignment,
    ExternalPrincipalMapping, Institution, JournalEntry, PaymentIntent, Posting, Principal,
};
use super::BankSchema;

worth_query_relation!(pub ExternalPrincipal in BankSchema, ExternalPrincipalMapping => Principal);
worth_query_relation!(pub PrincipalCustomer in BankSchema, Principal => Customer);
worth_query_relation!(pub PersonalOwner in BankSchema, Principal => Account);
worth_query_relation!(pub BusinessOwner in BankSchema, Business => Principal);
worth_query_relation!(pub BusinessAccount in BankSchema, Business => Account);
worth_query_relation!(
    pub AccountAuthorizedUser in BankSchema,
    Principal => AccountAuthorization
);
worth_query_relation!(
    pub AuthorizationAccount in BankSchema,
    AccountAuthorization => Account
);
worth_query_relation!(
    pub InstitutionEmployee in BankSchema,
    Institution => EmployeeAssignment
);
worth_query_relation!(
    pub AssignmentPrincipal in BankSchema,
    EmployeeAssignment => Principal
);
worth_query_relation!(pub InstitutionAccount in BankSchema, Institution => Account);
worth_query_relation!(pub PaymentSource in BankSchema, PaymentIntent => Account);
worth_query_relation!(pub PaymentDestination in BankSchema, PaymentIntent => Account);
worth_query_relation!(pub PaymentApproval in BankSchema, PaymentIntent => Approval);
worth_query_relation!(pub ApprovalPrincipal in BankSchema, Approval => Principal);
worth_query_relation!(pub JournalPosting in BankSchema, JournalEntry => Posting);
worth_query_relation!(pub PostingAccount in BankSchema, Posting => Account);
