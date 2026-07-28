use crate::facade::application_schema::{
    ApplicationPrincipalBindingRef, ApplicationSchemaDeclarationDenial, ApplicationSchemaMember,
};
use crate::facade::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};

worth_query_application_schema! {
    pub schema IdentitySchema {
        owner: identity_test,
        version: (1, 0),
        members: |schema| {
            schema
                .entity(ExternalMapping::reference())
                .entity(Principal::reference())
                .aspect(ExternalMapping::reference(), ExternalIdentity::reference())
                .aspect(Principal::reference(), PrincipalIdentity::reference())
                .field(ExternalMapping::reference(), ExternalIdentityField::reference())
                .field(ExternalMapping::reference(), MappingStatusField::reference())
                .field(Principal::reference(), PrincipalIdentityField::reference())
                .relation(
                    MappingTarget::reference(),
                    ExternalMapping::reference(),
                    Principal::reference(),
                )
                .principal_binding(IdentityBinding::reference())
        }
    }
}

worth_query_entity!(pub ExternalMapping in IdentitySchema);
worth_query_entity!(pub Principal in IdentitySchema);
worth_query_aspect!(pub ExternalIdentity in IdentitySchema, ExternalMapping);
worth_query_field!(
    pub ExternalIdentityField in IdentitySchema, ExternalMapping, ExternalIdentity:
    WorthQueryExternalPrincipalIdentity, read_only, equality
);
worth_query_aspect!(pub PrincipalIdentity in IdentitySchema, Principal);
worth_query_field!(
    pub PrincipalIdentityField in IdentitySchema, Principal, PrincipalIdentity:
    u64, read_only, equality
);
worth_query_field!(
    pub MutablePrincipalIdentityField in IdentitySchema, Principal, PrincipalIdentity:
    u64, read_write, equality
);
worth_query_field!(
    pub WrongPrincipalIdentityField in IdentitySchema, Principal, PrincipalIdentity:
    String, read_only, equality
);
worth_query_field!(
    pub MappingStatusField in IdentitySchema, ExternalMapping, ExternalIdentity:
    WorthQueryPrincipalMappingStatus, read_write, equality
);
worth_query_field!(
    pub MutableExternalIdentityField in IdentitySchema, ExternalMapping, ExternalIdentity:
    WorthQueryExternalPrincipalIdentity, read_write, equality
);
worth_query_field!(
    pub ImmutableMappingStatusField in IdentitySchema, ExternalMapping, ExternalIdentity:
    WorthQueryPrincipalMappingStatus, read_only, equality
);
worth_query_relation!(
    pub MappingTarget in IdentitySchema,
    ExternalMapping => Principal
);
worth_query_relation!(
    pub ReversedMappingTarget in IdentitySchema,
    Principal => ExternalMapping
);
worth_query_principal_binding!(
    pub IdentityBinding in IdentitySchema,
    mapping ExternalMapping {
        identity: ExternalIdentityField,
        status: MappingStatusField,
        target: MappingTarget => Principal,
        principal_identity: PrincipalIdentityField
    }
);

#[test]
fn typed_principal_binding_enters_canonical_schema_members() {
    let declaration = IdentitySchema::declaration().unwrap();
    assert!(declaration.erased().members().iter().any(|member| matches!(
        member,
        ApplicationSchemaMember::PrincipalBinding { binding, .. }
            if binding == "IdentityBinding"
    )));
}

#[test]
fn forged_principal_binding_dependencies_fail_closed() {
    let forged = ApplicationPrincipalBindingRef::<
        IdentitySchema,
        IdentityBinding,
        ExternalMapping,
        Principal,
        u64,
    >::from_schema_identifiers(
        "IdentityBinding",
        "ExternalMapping",
        "ExternalIdentity",
        "ExternalIdentityField",
        "ExternalIdentity",
        "MissingStatusField",
        "MappingTarget",
        "Principal",
        "PrincipalIdentity",
        "PrincipalIdentityField",
    );
    let denial = crate::facade::application_schema::ApplicationSchemaDeclarationBuilder::<
        IdentitySchema,
    >::for_schema()
    .entity(ExternalMapping::reference())
    .entity(Principal::reference())
    .aspect(ExternalMapping::reference(), ExternalIdentity::reference())
    .field(
        ExternalMapping::reference(),
        ExternalIdentityField::reference(),
    )
    .field(
        ExternalMapping::reference(),
        MappingStatusField::reference(),
    )
    .aspect(Principal::reference(), PrincipalIdentity::reference())
    .field(Principal::reference(), PrincipalIdentityField::reference())
    .relation(
        MappingTarget::reference(),
        ExternalMapping::reference(),
        Principal::reference(),
    )
    .principal_binding(forged)
    .build()
    .unwrap_err();
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::MissingPrincipalBindingDependency
    );
}

#[test]
fn principal_binding_rejects_wrong_field_type_and_write_posture() {
    for case in [
        HostileBindingCase::new("ExternalIdentityField", "ExternalIdentityField"),
        HostileBindingCase::new("MutableExternalIdentityField", "MappingStatusField")
            .with_mutable_identity(),
        HostileBindingCase::new("ExternalIdentityField", "ImmutableMappingStatusField")
            .with_immutable_status(),
    ] {
        let denial = identity_declaration_with(case).unwrap_err();
        assert_eq!(
            denial,
            ApplicationSchemaDeclarationDenial::MissingPrincipalBindingDependency
        );
    }
}

#[test]
fn principal_binding_rejects_reversed_mapping_target_relation() {
    let denial = identity_declaration_with(
        HostileBindingCase::new("ExternalIdentityField", "MappingStatusField")
            .with_reversed_relation(),
    )
    .unwrap_err();
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::MissingPrincipalBindingDependency
    );
}

#[test]
fn principal_binding_rejects_mutable_target_principal_identity() {
    let forged = ApplicationPrincipalBindingRef::<
        IdentitySchema,
        IdentityBinding,
        ExternalMapping,
        Principal,
        u64,
    >::from_schema_identifiers(
        "IdentityBinding",
        "ExternalMapping",
        "ExternalIdentity",
        "ExternalIdentityField",
        "ExternalIdentity",
        "MappingStatusField",
        "MappingTarget",
        "Principal",
        "PrincipalIdentity",
        "MutablePrincipalIdentityField",
    );
    let denial = identity_declaration_builder()
        .field(
            Principal::reference(),
            MutablePrincipalIdentityField::reference(),
        )
        .principal_binding(forged)
        .build()
        .unwrap_err();
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::MissingPrincipalBindingDependency
    );
}

#[test]
fn principal_binding_rejects_wrong_target_principal_identity_type() {
    let forged = ApplicationPrincipalBindingRef::<
        IdentitySchema,
        IdentityBinding,
        ExternalMapping,
        Principal,
        String,
    >::from_schema_identifiers(
        "IdentityBinding",
        "ExternalMapping",
        "ExternalIdentity",
        "ExternalIdentityField",
        "ExternalIdentity",
        "MappingStatusField",
        "MappingTarget",
        "Principal",
        "PrincipalIdentity",
        "PrincipalIdentityField",
    );
    let denial = identity_declaration_builder()
        .field(
            Principal::reference(),
            WrongPrincipalIdentityField::reference(),
        )
        .principal_binding(forged)
        .build()
        .unwrap_err();
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::MissingPrincipalBindingDependency
    );
}

struct HostileBindingCase {
    identity_field: &'static str,
    status_field: &'static str,
    mutable_identity: bool,
    immutable_status: bool,
    reversed_relation: bool,
}

impl HostileBindingCase {
    const fn new(identity_field: &'static str, status_field: &'static str) -> Self {
        Self {
            identity_field,
            status_field,
            mutable_identity: false,
            immutable_status: false,
            reversed_relation: false,
        }
    }

    const fn with_mutable_identity(mut self) -> Self {
        self.mutable_identity = true;
        self
    }

    const fn with_immutable_status(mut self) -> Self {
        self.immutable_status = true;
        self
    }

    const fn with_reversed_relation(mut self) -> Self {
        self.reversed_relation = true;
        self
    }

    const fn target_relation(&self) -> &'static str {
        if self.reversed_relation {
            "ReversedMappingTarget"
        } else {
            "MappingTarget"
        }
    }
}

fn identity_declaration_with(
    case: HostileBindingCase,
) -> Result<
    crate::facade::application_schema::ApplicationSchemaDeclaration<IdentitySchema>,
    ApplicationSchemaDeclarationDenial,
> {
    let binding = ApplicationPrincipalBindingRef::<
        IdentitySchema,
        IdentityBinding,
        ExternalMapping,
        Principal,
        u64,
    >::from_schema_identifiers(
        "IdentityBinding",
        "ExternalMapping",
        "ExternalIdentity",
        case.identity_field,
        "ExternalIdentity",
        case.status_field,
        case.target_relation(),
        "Principal",
        "PrincipalIdentity",
        "PrincipalIdentityField",
    );
    let builder = crate::facade::application_schema::ApplicationSchemaDeclarationBuilder::<
        IdentitySchema,
    >::for_schema()
    .entity(ExternalMapping::reference())
    .entity(Principal::reference())
    .aspect(ExternalMapping::reference(), ExternalIdentity::reference())
    .aspect(Principal::reference(), PrincipalIdentity::reference())
    .field(Principal::reference(), PrincipalIdentityField::reference());
    let builder = if case.mutable_identity {
        builder.field(
            ExternalMapping::reference(),
            MutableExternalIdentityField::reference(),
        )
    } else {
        builder.field(
            ExternalMapping::reference(),
            ExternalIdentityField::reference(),
        )
    };
    let builder = if case.immutable_status {
        builder.field(
            ExternalMapping::reference(),
            ImmutableMappingStatusField::reference(),
        )
    } else {
        builder.field(
            ExternalMapping::reference(),
            MappingStatusField::reference(),
        )
    };
    let builder = if case.reversed_relation {
        builder.relation(
            ReversedMappingTarget::reference(),
            Principal::reference(),
            ExternalMapping::reference(),
        )
    } else {
        builder.relation(
            MappingTarget::reference(),
            ExternalMapping::reference(),
            Principal::reference(),
        )
    };
    builder.principal_binding(binding).build()
}

fn identity_declaration_builder(
) -> crate::facade::application_schema::ApplicationSchemaDeclarationBuilder<IdentitySchema> {
    crate::facade::application_schema::ApplicationSchemaDeclarationBuilder::<IdentitySchema>::for_schema()
        .entity(ExternalMapping::reference())
        .entity(Principal::reference())
        .aspect(ExternalMapping::reference(), ExternalIdentity::reference())
        .aspect(Principal::reference(), PrincipalIdentity::reference())
        .field(
            ExternalMapping::reference(),
            ExternalIdentityField::reference(),
        )
        .field(
            ExternalMapping::reference(),
            MappingStatusField::reference(),
        )
        .relation(
            MappingTarget::reference(),
            ExternalMapping::reference(),
            Principal::reference(),
        )
}
