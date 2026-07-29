use worth_query_decl::facade::worth_query_policy;

use crate::schema::BankSchema;

worth_query_policy!(pub EstateCapabilityScopePolicy in BankSchema);
worth_query_policy!(pub EstateConflictOfInterestPolicy in BankSchema);
worth_query_policy!(pub EstateSeparationOfDutyPolicy in BankSchema);
worth_query_policy!(pub EstateDistinctActorPolicy in BankSchema);
worth_query_policy!(pub EstateBeneficiaryExclusionPolicy in BankSchema);
worth_query_policy!(pub EstateDisclosurePolicy in BankSchema);
worth_query_policy!(pub EmergencyElevationPolicy in BankSchema);
