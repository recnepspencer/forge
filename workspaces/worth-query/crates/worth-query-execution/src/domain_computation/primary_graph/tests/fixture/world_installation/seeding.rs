use super::*;

pub(super) fn owner_bindings(include: bool) -> &'static [(&'static str, &'static str)] {
    if include {
        &[("principal-0", "account-1"), ("principal-0", "account-2")]
    } else {
        &[]
    }
}

pub(super) fn portable_package(
    declaration: worth_query_declaration::facade::application_schema::ApplicationSchemaDeclaration<
        IdentityExecutionSchema,
    >,
) -> WorthQueryValidatedPortableDomainPackage {
    WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "identity_execution_test",
        1,
        0,
    ))
    .application_schema(declaration)
    .validate()
    .unwrap()
}

pub(super) fn bind_account(
    bootstrap: &mut crate::domain_computation::primary_graph::WorthQueryPrimaryGraphBootstrap<
        IdentityExecutionSchema,
    >,
    key: &str,
    status: &str,
    label: &str,
) {
    bootstrap
        .bind_entity(
            WorthQueryApplicationEntitySeed::new(
                Account::reference(),
                WorthQueryApplicationEntityKey::new(key).unwrap(),
            )
            .field(AccountIdentity::reference(), key.to_owned())
            .field(AccountStatus::reference(), status.to_string())
            .field(AccountLabel::reference(), label.to_string()),
        )
        .unwrap();
}

pub(super) fn bind_activity(
    bootstrap: &mut crate::domain_computation::primary_graph::WorthQueryPrimaryGraphBootstrap<
        IdentityExecutionSchema,
    >,
    key: &str,
    sequence: u64,
) {
    bootstrap
        .bind_entity(
            WorthQueryApplicationEntitySeed::new(
                Activity::reference(),
                WorthQueryApplicationEntityKey::new(key).unwrap(),
            )
            .field(ActivityIdentity::reference(), key.to_owned())
            .field(ActivitySequence::reference(), sequence),
        )
        .unwrap();
}
