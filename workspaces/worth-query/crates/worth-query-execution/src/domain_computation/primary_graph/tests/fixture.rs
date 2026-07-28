use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant, SystemTime};

use worth_query_admission::facade::authenticated_principal::*;
use worth_query_declaration::facade::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};
use worth_query_declaration::{
    worth_query_ability, worth_query_application_schema, worth_query_aspect, worth_query_entity,
    worth_query_field, worth_query_operation, worth_query_operation_requires,
    worth_query_operation_writes, worth_query_policy, worth_query_principal_binding,
    worth_query_relation,
};
use worth_query_installation::facade::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryInstalledApplicationSchema, WorthQueryInstalledPrincipalBinding,
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
};

use crate::domain_computation::execution_runtime::{
    WorthQueryExecutionRuntime, WorthQueryExecutionRuntimeInstaller,
};
use crate::domain_computation::primary_graph::WorthQueryApplicationPrincipalKey;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationEntityKey, WorthQueryApplicationEntitySeed,
    WorthQueryApplicationRelationSeed, WorthQueryPrimaryGraphPublication,
};

worth_query_application_schema! {
    pub schema IdentityExecutionSchema {
        owner: identity_execution_test,
        version: (1, 0),
        members: |schema| {
            schema
                .entity(ExternalMapping::reference())
                .entity(Principal::reference())
                .entity(Account::reference())
                .aspect(ExternalMapping::reference(), ExternalIdentity::reference())
                .aspect(Principal::reference(), PrincipalIdentity::reference())
                .field(ExternalMapping::reference(), ExternalIdentityField::reference())
                .field(ExternalMapping::reference(), MappingStatusField::reference())
                .field(Principal::reference(), PrincipalIdentityField::reference())
                .aspect(Account::reference(), AccountPolicy::reference())
                .field(Account::reference(), AccountStatus::reference())
                .relation(
                    MappingTarget::reference(),
                    ExternalMapping::reference(),
                    Principal::reference(),
                )
                .relation(
                    AccountOwner::reference(),
                    Principal::reference(),
                    Account::reference(),
                )
                .principal_binding(IdentityBinding::reference())
                .ability(ViewAccount::reference())
                .operation(TouchAccountOperation::reference())
                .operation_requires_ability(
                    TouchAccountOperation::reference(),
                    ViewAccount::reference(),
                )
                .operation_write(
                    TouchAccountOperation::reference(),
                    AccountStatus::reference(),
                )
                .policy(AccountAccessPolicy::reference())
                .ability_policy(
                    ViewAccount::reference(),
                    AccountAccessPolicy::reference(),
                    [worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                        Principal::reference(),
                    )
                    .forward(AccountOwner::reference())
                    .allow(Account::reference())],
                )
        }
    }
}

worth_query_entity!(pub ExternalMapping in IdentityExecutionSchema);
worth_query_entity!(pub Principal in IdentityExecutionSchema);
worth_query_entity!(pub Account in IdentityExecutionSchema);
worth_query_aspect!(pub ExternalIdentity in IdentityExecutionSchema, ExternalMapping);
worth_query_field!(
    pub ExternalIdentityField in IdentityExecutionSchema, ExternalMapping, ExternalIdentity:
    WorthQueryExternalPrincipalIdentity, read_only, equality
);
worth_query_aspect!(pub PrincipalIdentity in IdentityExecutionSchema, Principal);
worth_query_field!(
    pub PrincipalIdentityField in IdentityExecutionSchema, Principal, PrincipalIdentity:
    u64, read_only, equality
);
worth_query_field!(
    pub MappingStatusField in IdentityExecutionSchema, ExternalMapping, ExternalIdentity:
    WorthQueryPrincipalMappingStatus, read_write, equality
);
worth_query_relation!(
    pub MappingTarget in IdentityExecutionSchema,
    ExternalMapping => Principal
);
worth_query_principal_binding!(
    pub IdentityBinding in IdentityExecutionSchema,
    mapping ExternalMapping {
        identity: ExternalIdentityField,
        status: MappingStatusField,
        target: MappingTarget => Principal,
        principal_identity: PrincipalIdentityField
    }
);
worth_query_aspect!(pub AccountPolicy in IdentityExecutionSchema, Account);
worth_query_field!(
    pub AccountStatus in IdentityExecutionSchema, Account, AccountPolicy:
    String, read_write, equality
);
worth_query_relation!(
    pub AccountOwner in IdentityExecutionSchema,
    Principal => Account
);
worth_query_ability!(pub ViewAccount scoped_to Account, in IdentityExecutionSchema);
worth_query_policy!(pub AccountAccessPolicy in IdentityExecutionSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchAccountInput;

worth_query_operation!(
    pub TouchAccountOperation(TouchAccountInput) in IdentityExecutionSchema
);
worth_query_operation_requires!(TouchAccountOperation => [ViewAccount]);
worth_query_operation_writes!(TouchAccountOperation => [AccountStatus]);

pub(super) type InstalledIdentityBinding = WorthQueryInstalledPrincipalBinding<
    IdentityExecutionSchema,
    IdentityBinding,
    ExternalMapping,
    Principal,
    u64,
>;

pub(super) struct IdentityWorld {
    pub(super) runtime: WorthQueryExecutionRuntime,
    pub(super) schema: WorthQueryInstalledApplicationSchema<IdentityExecutionSchema>,
    pub(super) binding: InstalledIdentityBinding,
    pub(super) publication: WorthQueryPrimaryGraphPublication,
}

pub(super) struct AuthorizationWorld {
    pub(super) application:
        crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            IdentityExecutionSchema,
        >,
    pub(super) binding: InstalledIdentityBinding,
}

impl IdentityWorld {
    pub(super) fn authenticate(
        &self,
        subject: &str,
        lifetime: Duration,
        scope: &WorthQueryRequestScope,
    ) -> WorthQueryAuthenticatedExternalPrincipal<IdentityExecutionSchema> {
        authenticate_external(&self.schema, subject, lifetime, scope)
    }
}

fn authenticate_external(
    schema: &WorthQueryInstalledApplicationSchema<IdentityExecutionSchema>,
    subject: &str,
    lifetime: Duration,
    scope: &WorthQueryRequestScope,
) -> WorthQueryAuthenticatedExternalPrincipal<IdentityExecutionSchema> {
    let adapter = admit_authentication_adapter(
        schema,
        WorthQueryAuthenticationAdapterAdmission::new(
            WorthQueryAuthenticationAudience::new("bank").unwrap(),
            WorthQueryAuthenticationMethod::new("test-oidc").unwrap(),
        ),
        CausalIdentityAdapter,
    )
    .unwrap();
    block_on(adapter.authenticate(
        TestCredential {
            subject: subject.to_string(),
            lifetime,
        },
        scope,
    ))
    .unwrap()
}

impl AuthorizationWorld {
    pub(super) fn authenticate(
        &self,
        subject: &str,
        lifetime: Duration,
        scope: &WorthQueryRequestScope,
    ) -> WorthQueryAuthenticatedExternalPrincipal<IdentityExecutionSchema> {
        authenticate_external(
            self.application.installed_schema(),
            subject,
            lifetime,
            scope,
        )
    }
}

pub(super) fn installed_world(rows: &[(&str, WorthQueryPrincipalMappingStatus)]) -> IdentityWorld {
    installed_world_with_policy_fact(rows, false)
}

pub(super) fn installed_world_with_policy_fact(
    rows: &[(&str, WorthQueryPrincipalMappingStatus)],
    include_policy_fact: bool,
) -> IdentityWorld {
    let declaration = IdentityExecutionSchema::declaration().unwrap();
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "identity_execution_test",
        1,
        0,
    ))
    .application_schema(declaration.clone())
    .validate()
    .unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    let installation = WorthQueryExecutionRuntimeInstaller::new()
        .install(WorthQueryInstallationGeneration::initial(), [admitted])
        .unwrap();
    let (mut runtime, authority) = installation.into_parts();
    let schema = runtime
        .installed_packages()
        .bind_application_schema(declaration)
        .unwrap();
    let binding = schema
        .principal_binding(IdentityBinding::reference())
        .unwrap();
    let mut bootstrap = authority.prepare_primary_graph(&runtime, &schema).unwrap();
    for (ordinal, (subject, status)) in rows.iter().enumerate() {
        bootstrap
            .bind_principal(
                &binding,
                WorthQueryApplicationPrincipalKey::new(format!("principal-{ordinal}")).unwrap(),
                u64::try_from(ordinal + 1).unwrap(),
                external_identity(subject),
                *status,
            )
            .unwrap();
    }
    if include_policy_fact {
        bootstrap
            .bind_entity(
                WorthQueryApplicationEntitySeed::new(
                    Account::reference(),
                    WorthQueryApplicationEntityKey::new("account-1").unwrap(),
                )
                .field(AccountStatus::reference(), "open".to_string()),
            )
            .unwrap();
        bootstrap
            .bind_relation(WorthQueryApplicationRelationSeed::new(
                AccountOwner::reference(),
                "owner-1",
                WorthQueryApplicationEntityKey::new("principal-0").unwrap(),
                WorthQueryApplicationEntityKey::new("account-1").unwrap(),
            ))
            .unwrap();
    }
    let publication = bootstrap.publish(&mut runtime, &authority).unwrap();
    IdentityWorld {
        runtime,
        schema,
        binding,
        publication,
    }
}

pub(super) fn installed_authorization_world(include_owner_relation: bool) -> AuthorizationWorld {
    let declaration = IdentityExecutionSchema::declaration().unwrap();
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "identity_execution_test",
        1,
        0,
    ))
    .application_schema(declaration.clone())
    .validate()
    .unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    let installation = WorthQueryExecutionRuntimeInstaller::new()
        .install(WorthQueryInstallationGeneration::initial(), [admitted])
        .unwrap();
    let (runtime, authority) = installation.into_parts();
    let schema = runtime
        .installed_packages()
        .bind_application_schema(declaration)
        .unwrap();
    let binding = schema
        .principal_binding(IdentityBinding::reference())
        .unwrap();
    let mut bootstrap = authority.prepare_primary_graph(&runtime, &schema).unwrap();
    bootstrap
        .bind_principal(
            &binding,
            WorthQueryApplicationPrincipalKey::new("principal-0").unwrap(),
            1_u64,
            external_identity("alice"),
            WorthQueryPrincipalMappingStatus::Enabled,
        )
        .unwrap();
    bootstrap
        .bind_entity(
            WorthQueryApplicationEntitySeed::new(
                Account::reference(),
                WorthQueryApplicationEntityKey::new("account-1").unwrap(),
            )
            .field(AccountStatus::reference(), "open".to_string()),
        )
        .unwrap();
    if include_owner_relation {
        bootstrap
            .bind_relation(WorthQueryApplicationRelationSeed::new(
                AccountOwner::reference(),
                "owner-1",
                WorthQueryApplicationEntityKey::new("principal-0").unwrap(),
                WorthQueryApplicationEntityKey::new("account-1").unwrap(),
            ))
            .unwrap();
    }
    let application = bootstrap
        .publish_application_runtime(runtime, authority, schema)
        .unwrap();
    AuthorizationWorld {
        application,
        binding,
    }
}

pub(super) fn external_identity(subject: &str) -> WorthQueryExternalPrincipalIdentity {
    WorthQueryExternalPrincipalIdentity::new("https://issuer.example", subject).unwrap()
}

pub(super) fn live_scope() -> WorthQueryRequestScope {
    let source = WorthQueryCancellationSource::new();
    WorthQueryRequestScope::new(Instant::now() + Duration::from_secs(60), source.token())
}

pub(super) fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

struct TestCredential {
    subject: String,
    lifetime: Duration,
}

struct CausalIdentityAdapter;

impl WorthQueryAuthenticationAdapter for CausalIdentityAdapter {
    type Credential = TestCredential;

    fn configuration_identity(&self) -> &str {
        "identity-execution-test-adapter-v1"
    }

    fn validate<'a>(
        &'a self,
        credential: Self::Credential,
        _scope: &'a WorthQueryRequestScope,
    ) -> WorthQueryAuthenticationFuture<'a> {
        Box::pin(async move {
            let now = SystemTime::now();
            WorthQueryValidatedExternalPrincipal::new(
                external_identity(&credential.subject),
                WorthQueryAuthenticationAudience::new("bank").unwrap(),
                WorthQueryAuthenticationMethod::new("test-oidc").unwrap(),
                now,
                now + credential.lifetime,
                vec![WorthQueryPrincipalAttribute::new("display", "Test User").unwrap()],
            )
            .map_err(|_| {
                WorthQueryAuthenticationAdapterFailure::new(
                    WorthQueryAuthenticationAdapterFailureKind::ProtocolViolation,
                )
            })
        })
    }
}
