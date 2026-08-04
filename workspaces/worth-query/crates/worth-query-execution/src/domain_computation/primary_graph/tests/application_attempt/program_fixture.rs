use worth_query_declaration::facade::application_schema::TypedMutationPreconditions;

use super::{AccountStatus, TouchAccountOperation};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationEffectProgram, WorthQueryApplicationEntityIdentity,
    WorthQueryAuthenticatedPrincipal,
};

type Schema = super::super::fixture::IdentityExecutionSchema;
type Principal = super::super::fixture::Principal;
type Account = super::super::fixture::Account;
type Input = super::super::fixture::TouchAccountInput;
type World = super::super::fixture::AuthorizationWorld;
type Preconditions = TypedMutationPreconditions<Schema, TouchAccountOperation, Account>;
pub(super) type Program =
    WorthQueryApplicationEffectProgram<Schema, TouchAccountOperation, Input, Account>;

pub(super) fn admitted_program(
    world: &World,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, u64>,
    account: &WorthQueryApplicationEntityIdentity<Schema, Account>,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
    replacement: &str,
) -> Program {
    admitted_program_with_emit(world, principal, account, request, replacement, None)
}

pub(super) fn admitted_program_with_emit(
    world: &World,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, u64>,
    account: &WorthQueryApplicationEntityIdentity<Schema, Account>,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
    replacement: &str,
    emission: Option<&str>,
) -> Program {
    admitted_program_with_emissions(world, principal, account, request, replacement, emission)
}

pub(super) fn admitted_program_with_emissions<'a>(
    world: &World,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, u64>,
    account: &WorthQueryApplicationEntityIdentity<Schema, Account>,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
    replacement: &str,
    emissions: impl IntoIterator<Item = &'a str>,
) -> Program {
    admitted_program_from_options(
        world,
        principal,
        account,
        request,
        ProgramOptions {
            replacement,
            emissions: emissions.into_iter().collect(),
            preconditions: Preconditions::new(),
        },
    )
}

pub(super) fn admitted_program_with_expected_status(
    world: &World,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, u64>,
    account: &WorthQueryApplicationEntityIdentity<Schema, Account>,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
    status_and_replacement: (&str, &str),
) -> Program {
    admitted_program_from_options(
        world,
        principal,
        account,
        request,
        ProgramOptions {
            replacement: status_and_replacement.1,
            emissions: Vec::new(),
            preconditions: Preconditions::new().expect_fact(
                AccountStatus::reference(),
                status_and_replacement.0.to_owned(),
            ),
        },
    )
}

struct ProgramOptions<'a> {
    replacement: &'a str,
    emissions: Vec<&'a str>,
    preconditions: Preconditions,
}

fn admitted_program_from_options(
    world: &World,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, u64>,
    account: &WorthQueryApplicationEntityIdentity<Schema, Account>,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
    options: ProgramOptions<'_>,
) -> Program {
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let admission = world
        .application
        .authorize_operation(
            principal,
            account,
            &operation,
            options.preconditions,
            request,
        )
        .unwrap();
    let (_, projection, _) = world
        .invariant
        .project_admitted_operation(&admission, |reader, projected| {
            reader
                .require_decision_field(projected, AccountStatus::reference())
                .unwrap();
        })
        .unwrap()
        .into_parts();
    let reads = world
        .application
        .begin_projected_application_read_attempt(admission, projection)
        .unwrap();
    let mut effects = reads
        .complete_projected_dependencies()
        .unwrap()
        .begin_effect_program();
    let account = effects.existing_entity(account).unwrap();
    effects
        .write_field(
            &account,
            AccountStatus::reference(),
            options.replacement.to_owned(),
        )
        .unwrap();
    for payload in options.emissions {
        effects
            .emit(
                super::super::fixture::AccountActivityEffect::reference(),
                payload.to_owned(),
            )
            .unwrap();
    }
    effects.finish().unwrap()
}
