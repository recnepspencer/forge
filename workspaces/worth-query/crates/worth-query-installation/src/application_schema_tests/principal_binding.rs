use super::*;

pub(super) fn test_principal_binding<Schema>(
) -> ApplicationPrincipalBindingRef<Schema, PrincipalBinding, TestEntity, TestEntity, u64>
where
    Schema: ApplicationSchema,
{
    let identity =
        ApplicationFieldRef::<
            Schema,
            TestEntity,
            IdentityAspect,
            ExternalIdentityField,
            WorthQueryExternalPrincipalIdentity,
            ReadOnly,
            EqualityPredicate,
        >::from_schema_identifiers("TestEntity", "IdentityAspect", "ExternalIdentityField");
    let status =
        ApplicationFieldRef::<
            Schema,
            TestEntity,
            IdentityAspect,
            MappingStatusField,
            WorthQueryPrincipalMappingStatus,
            ReadWrite,
            NoEqualityPredicate,
        >::from_schema_identifiers("TestEntity", "IdentityAspect", "MappingStatusField");
    let target = ApplicationRelationRef::<Schema, MappingTarget, TestEntity, TestEntity>::
        from_schema_identifiers("MappingTarget", "TestEntity", "TestEntity");
    let principal_identity =
        ApplicationFieldRef::<
            Schema,
            TestEntity,
            IdentityAspect,
            PrincipalIdentityField,
            u64,
            ReadOnly,
            EqualityPredicate,
        >::from_schema_identifiers("TestEntity", "IdentityAspect", "PrincipalIdentityField");
    ApplicationPrincipalBindingRef::from_requirements(
        "PrincipalBinding",
        ApplicationPrincipalBindingRequirements {
            mapping_identity: ApplicationPrincipalMappingIdentityRequirement::from_field(identity),
            mapping_status: ApplicationPrincipalMappingStatusRequirement::from_field(status),
            target: ApplicationPrincipalTargetRequirement::from_relation(target),
            principal_identity: ApplicationPrincipalIdentityRequirement::from_field(
                principal_identity,
            ),
        },
    )
}
