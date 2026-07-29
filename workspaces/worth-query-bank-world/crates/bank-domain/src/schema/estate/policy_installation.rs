use worth_query_decl::facade::application_schema::ApplicationSchemaDeclarationBuilder;

use super::{
    EmergencyElevationPolicy, EstateBeneficiaryExclusionPolicy, EstateCapabilityScopePolicy,
    EstateConflictOfInterestPolicy, EstateDisclosurePolicy, EstateDistinctActorPolicy,
    EstateSeparationOfDutyPolicy,
};
use crate::schema::BankSchema;

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema
        .policy(EstateCapabilityScopePolicy::reference())
        .policy(EstateConflictOfInterestPolicy::reference())
        .policy(EstateSeparationOfDutyPolicy::reference())
        .policy(EstateDistinctActorPolicy::reference())
        .policy(EstateBeneficiaryExclusionPolicy::reference())
        .policy(EstateDisclosurePolicy::reference())
        .policy(EmergencyElevationPolicy::reference())
}
