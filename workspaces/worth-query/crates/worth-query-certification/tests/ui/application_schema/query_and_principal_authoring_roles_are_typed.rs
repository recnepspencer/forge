use worth_query_decl::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinitionBuilder,
    ApplicationQueryDependencyCeiling, ApplicationQueryDisclosureContract,
    ApplicationQueryLaneEligibility, ApplicationQueryReference, TypedApplicationQueryResultShape,
};
use worth_query_decl::facade::application_schema::{
    ApplicationEntityRef, ApplicationFieldRef, ApplicationPrincipalBindingRef,
    ApplicationPrincipalBindingRequirements, ApplicationPrincipalIdentityRequirement,
    ApplicationPrincipalMappingIdentityRequirement, ApplicationPrincipalMappingStatusRequirement,
    ApplicationPrincipalTargetRequirement, ApplicationRelationRef, EqualityPredicate,
    NoEqualityPredicate, ReadOnly, ReadWrite,
};
use worth_query_decl::facade::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};

struct Schema;
struct Query;
struct Parameters;
struct Result;
struct Mapping;
struct Principal;
struct IdentityAspect;
struct IdentityField;
struct StatusField;
struct PrincipalIdentityField;
struct Target;
struct Binding;

fn complete_query_authoring(
    reference: ApplicationQueryReference<Schema, Query, Parameters, Result, Principal>,
    principal: ApplicationEntityRef<Schema, Principal>,
    shape: TypedApplicationQueryResultShape<Schema, Query, Principal, Result>,
) {
    let _ = ApplicationQueryDefinitionBuilder::declare(reference)
        .root(principal)
        .scope(principal)
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(0, 0, 0))
        .disclosure(ApplicationQueryDisclosureContract::public())
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot())
        .public();
}

fn complete_principal_authoring(
    identity: ApplicationFieldRef<
        Schema,
        Mapping,
        IdentityAspect,
        IdentityField,
        WorthQueryExternalPrincipalIdentity,
        ReadOnly,
        EqualityPredicate,
    >,
    status: ApplicationFieldRef<
        Schema,
        Mapping,
        IdentityAspect,
        StatusField,
        WorthQueryPrincipalMappingStatus,
        ReadWrite,
        NoEqualityPredicate,
    >,
    target: ApplicationRelationRef<Schema, Target, Mapping, Principal>,
    principal_identity: ApplicationFieldRef<
        Schema,
        Principal,
        IdentityAspect,
        PrincipalIdentityField,
        u64,
        ReadOnly,
        EqualityPredicate,
    >,
) {
    let _: ApplicationPrincipalBindingRef<Schema, Binding, Mapping, Principal, u64> =
        ApplicationPrincipalBindingRef::from_requirements(
            "Binding",
            ApplicationPrincipalBindingRequirements {
                mapping_identity: ApplicationPrincipalMappingIdentityRequirement::from_field(
                    identity,
                ),
                mapping_status: ApplicationPrincipalMappingStatusRequirement::from_field(status),
                target: ApplicationPrincipalTargetRequirement::from_relation(target),
                principal_identity: ApplicationPrincipalIdentityRequirement::from_field(
                    principal_identity,
                ),
            },
        );
}

fn main() {}
