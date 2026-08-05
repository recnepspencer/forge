use worth_query_decl::facade::worth_query_relation;

use crate::schema::BankSchema;
use crate::schema::{Account, EmployeeAssignment, Institution, Principal};

use super::entities::{
    Branch, CapabilityGrant, DeathNotice, EmergencyAccess, EstateCase, LegalAuthority,
    MandatoryReview,
};

worth_query_relation!(pub BranchInstitution in BankSchema, Branch => Institution);
worth_query_relation!(pub DeathNoticeSubject in BankSchema, DeathNotice => Principal);
worth_query_relation!(pub EstateDeathNotice in BankSchema, EstateCase => DeathNotice);
worth_query_relation!(pub EstateDeceased in BankSchema, EstateCase => Principal);
worth_query_relation!(pub EstateAccount in BankSchema, EstateCase => Account);
worth_query_relation!(pub EstateBranch in BankSchema, EstateCase => Branch);
worth_query_relation!(pub EstateExecutor in BankSchema, Principal => EstateCase);
worth_query_relation!(pub EstateBeneficiary in BankSchema, Principal => EstateCase);
worth_query_relation!(pub EstateJointOwner in BankSchema, Principal => Account);
worth_query_relation!(pub EstateAuthorizedSigner in BankSchema, Principal => Account);
worth_query_relation!(
    pub EstateAssignment in BankSchema,
    EmployeeAssignment => EstateCase
);
worth_query_relation!(
    pub LegalAuthorityEstate in BankSchema,
    LegalAuthority => EstateCase
);
worth_query_relation!(
    pub LegalAuthorityHolder in BankSchema,
    LegalAuthority => Principal
);
worth_query_relation!(
    pub CapabilityGrantee in BankSchema,
    Principal => CapabilityGrant
);
worth_query_relation!(
    pub CapabilityGrantor in BankSchema,
    Principal => CapabilityGrant
);
worth_query_relation!(
    pub CapabilityEstate in BankSchema,
    CapabilityGrant => EstateCase
);
worth_query_relation!(
    pub CapabilityAccount in BankSchema,
    CapabilityGrant => Account
);
worth_query_relation!(
    pub CapabilityInstitution in BankSchema,
    CapabilityGrant => Institution
);
worth_query_relation!(
    pub CapabilityBranch in BankSchema,
    CapabilityGrant => Branch
);
worth_query_relation!(
    pub CapabilityParent in BankSchema,
    CapabilityGrant => CapabilityGrant
);
worth_query_relation!(
    pub EmergencyRequester in BankSchema,
    Principal => EmergencyAccess
);
worth_query_relation!(
    pub EmergencyApprover in BankSchema,
    Principal => EmergencyAccess
);
worth_query_relation!(
    pub EmergencyGrant in BankSchema,
    EmergencyAccess => CapabilityGrant
);
worth_query_relation!(
    pub EmergencyEstate in BankSchema,
    EmergencyAccess => EstateCase
);
worth_query_relation!(
    pub EmergencyReview in BankSchema,
    EmergencyAccess => MandatoryReview
);
worth_query_relation!(
    pub ReviewPrincipal in BankSchema,
    Principal => MandatoryReview
);
worth_query_relation!(
    pub ReviewEstate in BankSchema,
    MandatoryReview => EstateCase
);
