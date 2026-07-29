use worth_query_decl::facade::worth_query_entity;

use super::BankSchema;

worth_query_entity!(pub Institution in BankSchema);
worth_query_entity!(pub ExternalPrincipalMapping in BankSchema);
worth_query_entity!(pub Principal in BankSchema);
worth_query_entity!(pub Customer in BankSchema);
worth_query_entity!(pub Business in BankSchema);
worth_query_entity!(pub Account in BankSchema);
worth_query_entity!(pub AccountAuthorization in BankSchema);
worth_query_entity!(pub EmployeeAssignment in BankSchema);
worth_query_entity!(pub PaymentIntent in BankSchema);
worth_query_entity!(pub Approval in BankSchema);
worth_query_entity!(pub JournalEntry in BankSchema);
worth_query_entity!(pub Posting in BankSchema);
