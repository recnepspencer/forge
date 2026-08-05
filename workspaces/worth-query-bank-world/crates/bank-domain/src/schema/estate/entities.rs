use worth_query_decl::facade::worth_query_entity;

use crate::schema::BankSchema;

worth_query_entity!(pub Branch in BankSchema);
worth_query_entity!(pub CapabilityGrant in BankSchema);
worth_query_entity!(pub DeathNotice in BankSchema);
worth_query_entity!(pub EmergencyAccess in BankSchema);
worth_query_entity!(pub EstateCase in BankSchema);
worth_query_entity!(pub LegalAuthority in BankSchema);
worth_query_entity!(pub MandatoryReview in BankSchema);
