use std::collections::BTreeSet;

use bank_domain::schema::BankSchema;
use worth_query_decl::facade::application_schema::{
    ApplicationOperationProgramTarget, ApplicationSchemaMember,
};

#[test]
fn bank_manifest_matches_the_frozen_phase_one_world() {
    let declaration = BankSchema::declaration().unwrap();
    let members = declaration.erased().members();
    assert_entity_and_relation_inventory(members);
    assert_operation_inventory(members);
    assert_field_and_governance_inventory(members);
    assert_account_creation_programs(members);
    assert_money_programs(members);
    assert_payment_and_authorization_programs(members);
}

fn assert_entity_and_relation_inventory(members: &[ApplicationSchemaMember]) {
    assert_eq!(
        names(members, entity_name),
        expected(&[
            "Account",
            "AccountAuthorization",
            "Approval",
            "Business",
            "Customer",
            "EmployeeAssignment",
            "ExternalPrincipalMapping",
            "IdempotencyRecord",
            "Institution",
            "JournalEntry",
            "PaymentIntent",
            "Posting",
            "Principal",
        ])
    );
    assert_eq!(
        names(members, relation_name),
        expected(&[
            "AccountAuthorizedUser",
            "ApprovalPrincipal",
            "AssignmentPrincipal",
            "AuthorizationAccount",
            "BusinessAccount",
            "BusinessOwner",
            "ExternalPrincipal",
            "InstitutionAccount",
            "InstitutionEmployee",
            "JournalPosting",
            "PaymentApproval",
            "PaymentDestination",
            "PaymentSource",
            "PersonalOwner",
            "PostingAccount",
            "PrincipalCustomer",
        ])
    );
}

fn assert_operation_inventory(members: &[ApplicationSchemaMember]) {
    assert_eq!(
        names(members, operation_name),
        expected(&[
            "ApplyOpeningFundingOperation",
            "ApprovePaymentOperation",
            "CreateBusinessAccountOperation",
            "CreatePersonalAccountOperation",
            "DepositOperation",
            "GrantAccountAuthorizationOperation",
            "InitiateBusinessPaymentOperation",
            "RejectPaymentOperation",
            "ReverseJournalOperation",
            "RevokeAccountAuthorizationOperation",
            "SendMoneyOperation",
            "WithdrawOperation",
        ])
    );
}

fn assert_field_and_governance_inventory(members: &[ApplicationSchemaMember]) {
    assert_eq!(
        names(members, aspect_name),
        expected(&[
            "AccountProfile",
            "AccountState",
            "AuthorizationScope",
            "EmployeeScope",
            "Identity",
            "PaymentState",
            "PostingValue",
        ])
    );
    assert_eq!(
        names(members, field_name),
        expected(&[
            "AccountDisplayName",
            "AccountIdentity",
            "AssignmentRole",
            "AuthorizationRole",
            "AvailableBalance",
            "Kind",
            "PaymentStatusField",
            "PostingAmount",
            "Purpose",
            "Status",
        ])
    );
    assert_eq!(
        names(members, policy_name),
        expected(&[
            "AccountMutationScopePolicy",
            "AccountVisibilityPolicy",
            "DistinctApproverPolicy",
            "EmployeeScopePolicy",
        ])
    );
    assert_eq!(names(members, currency_name), expected(&["UsdCurrency"]));
    assert_eq!(
        names(members, effect_name),
        expected(&["AccountActivityEffect"])
    );
}

fn assert_account_creation_programs(members: &[ApplicationSchemaMember]) {
    assert_program(
        members,
        "CreatePersonalAccountOperation",
        &[
            "create:Account",
            "link:InstitutionAccount:Institution->Account",
            "link:PersonalOwner:Principal->Account",
            "write:Account/AccountProfile/AccountDisplayName",
            "write:Account/AccountProfile/Kind",
            "write:Account/AccountState/Status",
        ],
    );
    assert_program(
        members,
        "CreateBusinessAccountOperation",
        &[
            "create:Account",
            "link:BusinessAccount:Business->Account",
            "link:InstitutionAccount:Institution->Account",
            "write:Account/AccountProfile/AccountDisplayName",
            "write:Account/AccountProfile/Kind",
            "write:Account/AccountState/Status",
        ],
    );
}

fn assert_money_programs(members: &[ApplicationSchemaMember]) {
    for operation in [
        "ApplyOpeningFundingOperation",
        "DepositOperation",
        "ReverseJournalOperation",
        "SendMoneyOperation",
        "WithdrawOperation",
    ] {
        assert_money_program(members, operation);
    }
}

fn assert_payment_and_authorization_programs(members: &[ApplicationSchemaMember]) {
    assert_program(
        members,
        "InitiateBusinessPaymentOperation",
        &[
            "create:PaymentIntent",
            "link:PaymentDestination:PaymentIntent->Account",
            "link:PaymentSource:PaymentIntent->Account",
        ],
    );
    assert_program(
        members,
        "ApprovePaymentOperation",
        &[
            "create:Approval",
            "emit:AccountActivityEffect",
            "link:ApprovalPrincipal:Approval->Principal",
            "link:PaymentApproval:PaymentIntent->Approval",
            "write:PaymentIntent/PaymentState/PaymentStatusField",
        ],
    );
    assert_program(
        members,
        "RejectPaymentOperation",
        &["write:PaymentIntent/PaymentState/PaymentStatusField"],
    );
    assert_program(
        members,
        "GrantAccountAuthorizationOperation",
        &[
            "create:AccountAuthorization",
            "link:AccountAuthorizedUser:Principal->AccountAuthorization",
            "link:AuthorizationAccount:AccountAuthorization->Account",
            "write:AccountAuthorization/AuthorizationScope/AuthorizationRole",
        ],
    );
    assert_program(
        members,
        "RevokeAccountAuthorizationOperation",
        &[
            "delete:AccountAuthorization",
            "unlink:AccountAuthorizedUser:Principal->AccountAuthorization",
            "unlink:AuthorizationAccount:AccountAuthorization->Account",
        ],
    );
}

#[test]
fn bank_schema_source_has_no_raw_query_descriptor_or_dynamic_key_lane() {
    let schema_sources = [
        include_str!("../src/schema/entities.rs"),
        include_str!("../src/schema/fields.rs"),
        include_str!("../src/schema/governance.rs"),
        include_str!("../src/schema/manifest.rs"),
        include_str!("../src/schema/operations.rs"),
        include_str!("../src/schema/program_manifest.rs"),
        include_str!("../src/schema/relations.rs"),
        include_str!("../src/schema/values.rs"),
    ]
    .join("\n");
    for forbidden in [
        "from_schema_identifier(",
        "from_schema_identifiers(",
        "ApplicationEntityRef::<",
        "ApplicationFieldRef::<",
        "DynamicApplication",
    ] {
        assert!(
            !schema_sources.contains(forbidden),
            "bank schema contains forbidden raw lane: {forbidden}"
        );
    }

    let manifest = include_str!("../Cargo.toml");
    for forbidden_dependency in [
        "worth-query-declaration",
        "worth-query-installation",
        "worth-query-execution",
        "worth-query-replay",
        "worth-runtime-bridge",
        "worth-relational",
    ] {
        assert!(
            !manifest.contains(forbidden_dependency),
            "bank-domain crosses audience boundary through {forbidden_dependency}"
        );
    }
}

fn names(
    members: &[ApplicationSchemaMember],
    select: fn(&ApplicationSchemaMember) -> Option<&str>,
) -> BTreeSet<&str> {
    members.iter().filter_map(select).collect()
}

fn entity_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Entity { entity } => Some(entity),
        _ => None,
    }
}

fn relation_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Relation { relation, .. } => Some(relation),
        _ => None,
    }
}

fn aspect_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Aspect { aspect, .. } => Some(aspect),
        _ => None,
    }
}

fn field_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Field { field, .. } => Some(field),
        _ => None,
    }
}

fn operation_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Operation { operation, .. } => Some(operation),
        _ => None,
    }
}

fn policy_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Policy { policy } => Some(policy),
        _ => None,
    }
}

fn currency_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Currency { currency } => Some(currency),
        _ => None,
    }
}

fn effect_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Effect { effect, .. } => Some(effect),
        _ => None,
    }
}

fn assert_money_program(members: &[ApplicationSchemaMember], operation: &str) {
    assert_program(
        members,
        operation,
        &[
            "create:JournalEntry",
            "create:Posting",
            "emit:AccountActivityEffect",
            "link:JournalPosting:JournalEntry->Posting",
            "link:PostingAccount:Posting->Account",
            "write:Posting/PostingValue/PostingAmount",
            "write:Posting/PostingValue/Purpose",
        ],
    );
}

fn assert_program(members: &[ApplicationSchemaMember], operation: &str, expected_targets: &[&str]) {
    let actual = members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::OperationProgram {
                operation: installed,
                target,
            } if installed == operation => Some(program_target(target)),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let expected = expected_targets
        .iter()
        .map(|target| (*target).to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "operation program drift: {operation}");
}

fn program_target(target: &ApplicationOperationProgramTarget) -> String {
    match target {
        ApplicationOperationProgramTarget::Create { entity } => format!("create:{entity}"),
        ApplicationOperationProgramTarget::Delete { entity } => format!("delete:{entity}"),
        ApplicationOperationProgramTarget::Write {
            entity,
            aspect,
            field,
        } => format!("write:{entity}/{aspect}/{field}"),
        ApplicationOperationProgramTarget::Link { relation, from, to } => {
            format!("link:{relation}:{from}->{to}")
        }
        ApplicationOperationProgramTarget::Unlink { relation, from, to } => {
            format!("unlink:{relation}:{from}->{to}")
        }
        ApplicationOperationProgramTarget::Emit { effect } => format!("emit:{effect}"),
    }
}

fn expected<'a>(values: &'a [&'a str]) -> BTreeSet<&'a str> {
    values.iter().copied().collect()
}
