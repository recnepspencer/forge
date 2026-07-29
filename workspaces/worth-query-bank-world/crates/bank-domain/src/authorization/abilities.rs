use worth_query_decl::facade::worth_query_ability;

use crate::schema::{Account, BankSchema, Business, Institution, PaymentIntent, Principal};

worth_query_ability!(pub OpenAccount scoped_to Institution, in BankSchema);
worth_query_ability!(pub DiscoverOwnAccounts scoped_to Principal, in BankSchema);
worth_query_ability!(pub ViewAccount scoped_to Account, in BankSchema);
worth_query_ability!(pub ViewAccountAccess scoped_to Account, in BankSchema);
worth_query_ability!(pub ViewPayment scoped_to PaymentIntent, in BankSchema);
worth_query_ability!(pub SendPersonalFunds scoped_to Account, in BankSchema);
worth_query_ability!(pub ManageAccountAccess scoped_to Account, in BankSchema);
worth_query_ability!(pub InitiateBusinessFunds scoped_to Business, in BankSchema);
worth_query_ability!(pub ApproveBusinessFunds scoped_to PaymentIntent, in BankSchema);
worth_query_ability!(pub ServiceInstitutionAccount scoped_to Institution, in BankSchema);
worth_query_ability!(pub AuditInstitution scoped_to Institution, in BankSchema);
