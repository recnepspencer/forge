use worth_query_decl::facade::application_schema::{
    ApplicationFieldRef, ApplicationPrincipalMappingIdentityRequirement, NoEqualityPredicate,
    ReadWrite,
};
use worth_query_decl::facade::authentication::WorthQueryPrincipalMappingStatus;

struct Schema;
struct Mapping;
struct Aspect;
struct StatusField;

fn substitute_mapping_status_for_identity(
    status: ApplicationFieldRef<
        Schema,
        Mapping,
        Aspect,
        StatusField,
        WorthQueryPrincipalMappingStatus,
        ReadWrite,
        NoEqualityPredicate,
    >,
) {
    let _ = ApplicationPrincipalMappingIdentityRequirement::<Schema, Mapping>::from_field(status);
}

fn main() {}
