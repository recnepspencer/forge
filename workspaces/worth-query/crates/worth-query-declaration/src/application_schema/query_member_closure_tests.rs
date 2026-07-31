use worth_foundational::facade::ScalarAspectType;

use super::validate_application_query_members;
use crate::{
    application_query::{
        ApplicationQueryBasisSupport, ApplicationQueryCardinality,
        ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
        ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
        ApplicationQueryReference, ApplicationQueryResultFieldRef,
        ApplicationQueryResultShapeBuilder, ErasedApplicationQueryDefinition,
    },
    application_schema::{
        ApplicationAbilityRef, ApplicationEntityRef, ApplicationFieldRef,
        ApplicationSchemaDeclarationDenial, ApplicationSchemaMember, EqualityPredicate, ReadOnly,
    },
};

struct TestSchema;
struct Account;
struct AccountFacts;
struct AccountId;
struct AccountQuery;
struct AccountParameters;
struct AccountResult;
struct AccountIdSlot;
struct ViewAccount;

#[test]
fn valid_query_dependencies_and_projection_types_close() {
    assert_eq!(
        validate_application_query_members(&members(query("AccountId", "id"))),
        Ok(())
    );
}

#[test]
fn forged_projection_field_or_value_type_cannot_enter_package_meaning() {
    assert_eq!(
        validate_application_query_members(&members(query("MissingField", "id"))),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationQuery)
    );
    assert_eq!(
        validate_application_query_members(&members(string_query())),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationQuery)
    );
}

#[test]
fn one_schema_cannot_install_two_meanings_under_one_query_name() {
    let mut duplicated = dependencies();
    duplicated.push(ApplicationSchemaMember::ApplicationQuery {
        definition: query("AccountId", "id"),
    });
    duplicated.push(ApplicationSchemaMember::ApplicationQuery {
        definition: query("AccountId", "other_id"),
    });
    assert_eq!(
        validate_application_query_members(&duplicated),
        Err(ApplicationSchemaDeclarationDenial::DuplicateApplicationQuery)
    );
}

#[test]
fn governed_query_requires_its_exact_installed_ability_and_policy() {
    let governed = governed_query();
    assert_eq!(
        validate_application_query_members(&members(governed.clone())),
        Err(ApplicationSchemaDeclarationDenial::MissingAbilityDependency)
    );
    let mut ability_only = dependencies();
    ability_only.push(ApplicationSchemaMember::Ability {
        ability: "ViewAccount".to_string(),
        scope_entity: "Account".to_string(),
    });
    ability_only.push(ApplicationSchemaMember::ApplicationQuery {
        definition: governed.clone(),
    });
    assert_eq!(
        validate_application_query_members(&ability_only),
        Err(ApplicationSchemaDeclarationDenial::MissingAbilityPolicyDependency)
    );
    ability_only.insert(
        4,
        ApplicationSchemaMember::AbilityPolicy {
            ability: "ViewAccount".to_string(),
            scope_entity: "Account".to_string(),
            policy: "AccountVisibility".to_string(),
            paths: Vec::new(),
        },
    );
    assert_eq!(validate_application_query_members(&ability_only), Ok(()));
}

fn members(definition: ErasedApplicationQueryDefinition) -> Vec<ApplicationSchemaMember> {
    let mut members = dependencies();
    members.push(ApplicationSchemaMember::ApplicationQuery { definition });
    members
}

fn dependencies() -> Vec<ApplicationSchemaMember> {
    vec![
        ApplicationSchemaMember::Entity {
            entity: "Account".to_string(),
        },
        ApplicationSchemaMember::Aspect {
            entity: "Account".to_string(),
            aspect: "AccountFacts".to_string(),
        },
        ApplicationSchemaMember::Field {
            entity: "Account".to_string(),
            aspect: "AccountFacts".to_string(),
            field: "AccountId".to_string(),
            presence: crate::application_schema::ApplicationFieldPresence::Required,
            scalar_family: ScalarAspectType::UInt64,
            value_type: std::any::type_name::<u64>().to_string(),
            currency: None,
            writable: false,
            equality_queryable: true,
        },
    ]
}

fn query(field_name: &'static str, output_name: &'static str) -> ErasedApplicationQueryDefinition {
    let field = ApplicationFieldRef::<
        TestSchema,
        Account,
        AccountFacts,
        AccountId,
        u64,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_identifiers("Account", "AccountFacts", field_name);
    build_query(output_name, field)
}

fn string_query() -> ErasedApplicationQueryDefinition {
    let field = ApplicationFieldRef::<
        TestSchema,
        Account,
        AccountFacts,
        AccountId,
        String,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_identifiers("Account", "AccountFacts", "AccountId");
    build_query("id", field)
}

fn governed_query() -> ErasedApplicationQueryDefinition {
    let field = ApplicationFieldRef::<
        TestSchema,
        Account,
        AccountFacts,
        AccountId,
        u64,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_identifiers("Account", "AccountFacts", "AccountId");
    let account = ApplicationEntityRef::<TestSchema, Account>::from_schema_identifier("Account");
    let shape = ApplicationQueryResultShapeBuilder::<
        TestSchema,
        AccountQuery,
        Account,
        AccountResult,
    >::new(account)
    .field(ApplicationQueryResultFieldRef::<
        AccountQuery,
        AccountIdSlot,
        TestSchema,
        Account,
        AccountFacts,
        AccountId,
        u64,
        ReadOnly,
        EqualityPredicate,
        crate::application_schema::NoApplicationCurrency,
    >::new("id", field))
    .build();
    ApplicationQueryDefinitionBuilder::requires_ability(
        ApplicationQueryReference::<
            TestSchema,
            AccountQuery,
            AccountParameters,
            AccountResult,
            Account,
        >::from_schema_identifier("account"),
        account,
        account,
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(0, 0, 1),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        ApplicationAbilityRef::<TestSchema, ViewAccount, Account>::from_schema_identifiers(
            "ViewAccount",
            "Account",
        ),
    )
    .build()
    .unwrap()
    .into_erased()
}

fn build_query<Value>(
    output_name: &'static str,
    field: ApplicationFieldRef<
        TestSchema,
        Account,
        AccountFacts,
        AccountId,
        Value,
        ReadOnly,
        EqualityPredicate,
    >,
) -> ErasedApplicationQueryDefinition
where
    Value: crate::application_schema::TypedApplicationValue,
{
    let account = ApplicationEntityRef::<TestSchema, Account>::from_schema_identifier("Account");
    let result_field = ApplicationQueryResultFieldRef::<
        AccountQuery,
        AccountIdSlot,
        TestSchema,
        Account,
        AccountFacts,
        AccountId,
        Value,
        ReadOnly,
        EqualityPredicate,
        crate::application_schema::NoApplicationCurrency,
    >::new(output_name, field);
    let shape = ApplicationQueryResultShapeBuilder::<
        TestSchema,
        AccountQuery,
        Account,
        AccountResult,
    >::new(account)
    .field(result_field)
    .build();
    ApplicationQueryDefinitionBuilder::public(
        ApplicationQueryReference::<
            TestSchema,
            AccountQuery,
            AccountParameters,
            AccountResult,
            Account,
        >::from_schema_identifier("account"),
        account,
        account,
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(0, 0, 1),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
    )
    .build()
    .unwrap()
    .into_erased()
}
