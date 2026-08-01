use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
    ApplicationQueryOrderingDirection, ApplicationQueryParameterRef, ApplicationQueryReference,
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef,
    ApplicationQueryResultShapeBuilder, ForwardResultTraversal, ManyResults,
};
use worth_query_declaration::facade::application_schema::ApplicationEntityRef;
use worth_query_declaration::{
    worth_query_application_query, worth_query_application_schema, worth_query_aspect,
    worth_query_entity, worth_query_field, worth_query_relation,
};

use crate::facade::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryInstallationRuntimeIdentity, WorthQueryInstalledGraphObligationEffectPosture,
    WorthQueryInstalledGraphObligationKind, WorthQueryInstalledGraphObligationOwner,
    WorthQueryInstalledGraphObligationResourcePosture,
    WorthQueryInstalledGraphObligationSelectionBasis,
    WorthQueryInstalledGraphObligationTerminalRequirement, WorthQueryInstalledPackageIndex,
    WorthQueryPortableDefinition, WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage,
};

use super::WorthQueryApplicationQueryInstallationDenialKind;

mod authority_validation_tests;
mod canonical_basis_residue;
mod selector_identity;
mod shape_identity;

worth_query_application_schema! {
    pub schema QueryTestSchema {
        owner: application_query_installation_test,
        version: (1, 0),
        members: |schema| {
            schema
                .entity(Account::reference())
                .entity(Activity::reference())
                .aspect(Account::reference(), AccountFacts::reference())
                .aspect(Activity::reference(), ActivityFacts::reference())
                .field(Account::reference(), AccountId::reference())
                .field(Activity::reference(), ActivitySequence::reference())
                .field(Activity::reference(), ActivityKind::reference())
                .field(Activity::reference(), ActivityStatus::reference())
                .relation(
                    AccountActivity::reference(),
                    Account::reference(),
                    Activity::reference(),
                )
                .application_query(definition(
                    ApplicationQueryOrderingDirection::Descending,
                    "sequence",
                ))
                .application_query(shape_identity::grouped_one_definition())
                .application_query(shape_identity::grouped_two_definition())
        }
    }
}

worth_query_entity!(pub Account in QueryTestSchema);
worth_query_entity!(pub Activity in QueryTestSchema);
worth_query_aspect!(pub AccountFacts in QueryTestSchema, Account);
worth_query_aspect!(pub ActivityFacts in QueryTestSchema, Activity);
worth_query_field!(
    pub AccountId in QueryTestSchema, Account, AccountFacts:
    u64, read_only, equality
);
worth_query_field!(
    pub ActivitySequence in QueryTestSchema, Activity, ActivityFacts:
    u64, read_only, equality
);
worth_query_field!(
    pub ActivityKind in QueryTestSchema, Activity, ActivityFacts:
    u64, read_only, equality
);
worth_query_field!(
    pub ActivityStatus in QueryTestSchema, Activity, ActivityFacts:
    u64, read_only, equality
);
worth_query_relation!(
    pub AccountActivity in QueryTestSchema, Account => Activity
);

pub(super) struct ActivityQueryParameters;
pub(super) struct ActivityQueryResult;
struct AccountParameter;
struct AccountIdSlot;
struct ActivitySequenceSlot;
struct ActivityRelationSlot;

worth_query_application_query!(
    pub(super) ActivityQuery in QueryTestSchema,
    parameters ActivityQueryParameters,
    result ActivityQueryResult,
    scope Account,
    name "account_activity"
);
fn query_reference() -> ApplicationQueryReference<
    QueryTestSchema,
    ActivityQuery,
    ActivityQueryParameters,
    ActivityQueryResult,
    Account,
> {
    ActivityQuery::reference()
}

fn account_parameter() -> ApplicationQueryParameterRef<ActivityQuery, AccountParameter, u64> {
    ApplicationQueryParameterRef::from_query_identifier("account")
}

fn definition(
    direction: ApplicationQueryOrderingDirection,
    output_name: &'static str,
) -> ApplicationQueryDefinition<
    QueryTestSchema,
    ActivityQuery,
    ActivityQueryParameters,
    ActivityQueryResult,
    Account,
> {
    definition_with_scope_and_sequence_slot::<Account, ActivitySequenceSlot>(
        Account::reference(),
        direction,
        output_name,
    )
}

fn definition_with_scope<Scope>(
    scope: ApplicationEntityRef<QueryTestSchema, Scope>,
    direction: ApplicationQueryOrderingDirection,
    output_name: &'static str,
) -> ApplicationQueryDefinition<
    QueryTestSchema,
    ActivityQuery,
    ActivityQueryParameters,
    ActivityQueryResult,
    Scope,
> {
    definition_with_scope_and_sequence_slot::<Scope, ActivitySequenceSlot>(
        scope,
        direction,
        output_name,
    )
}

pub(super) fn definition_with_scope_and_sequence_slot<Scope, SequenceSlot: 'static>(
    scope: ApplicationEntityRef<QueryTestSchema, Scope>,
    direction: ApplicationQueryOrderingDirection,
    output_name: &'static str,
) -> ApplicationQueryDefinition<
    QueryTestSchema,
    ActivityQuery,
    ActivityQueryParameters,
    ActivityQueryResult,
    Scope,
> {
    let sequence =
        ApplicationQueryResultFieldRef::<ActivityQuery, SequenceSlot, _, _, _, _, _, _, _, _>::new(
            output_name,
            ActivitySequence::reference(),
        );
    let nested =
        ApplicationQueryResultShapeBuilder::<QueryTestSchema, ActivityQuery, Activity, ()>::new(
            Activity::reference(),
        )
        .field(sequence);
    let shape = ApplicationQueryResultShapeBuilder::<
        QueryTestSchema,
        ActivityQuery,
        Account,
        ActivityQueryResult,
    >::new(Account::reference())
    .field(ApplicationQueryResultFieldRef::<
        ActivityQuery,
        AccountIdSlot,
        _,
        _,
        _,
        _,
        _,
        _,
        _,
        _,
    >::new("account_id", AccountId::reference()))
    .relation(
        ApplicationQueryResultRelationRef::<
            ActivityQuery,
            ActivityRelationSlot,
            _,
            _,
            _,
            _,
            ForwardResultTraversal,
            ManyResults,
        >::forward_many("activity", AccountActivity::reference()),
        nested,
    )
    .build();
    ApplicationQueryDefinitionBuilder::public(
        ApplicationQueryReference::from_schema_identifier("account_activity"),
        Account::reference(),
        scope,
        shape,
        ApplicationQueryCardinality::Many,
        ApplicationQueryDependencyCeiling::bounded(1, 1, 2),
        ApplicationQueryDisclosureContract::installed_policy("account-holder"),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot().with_historical(),
    )
    .parameter(account_parameter())
    .where_equal(AccountId::reference(), account_parameter())
    .order_by(sequence, direction)
    .build()
    .unwrap()
}

#[test]
fn equivalent_installed_queries_converge_and_identity_dimensions_do_not_alias() {
    let schema = installed_schema();
    let left = schema.application_query(query_reference()).unwrap();
    let equivalent = schema.application_query(query_reference()).unwrap();
    let changed_order =
        definition(ApplicationQueryOrderingDirection::Ascending, "sequence").into_erased();
    let changed_shape =
        definition(ApplicationQueryOrderingDirection::Descending, "position").into_erased();

    assert_eq!(left.identity(), equivalent.identity());
    assert_eq!(
        left.read_family_binding().identity(),
        equivalent.read_family_binding().identity()
    );
    assert_eq!(
        left.read_family_binding().planning_contract(),
        left.read_graph()
    );
    assert_eq!(
        left.read_family_binding().canonical_planning_identity(),
        left.read_graph().canonical_planning_basis().digest()
    );
    assert_ne!(
        definition(ApplicationQueryOrderingDirection::Descending, "sequence")
            .into_erased()
            .canonical_basis(),
        changed_order.canonical_basis()
    );
    assert_ne!(
        definition(ApplicationQueryOrderingDirection::Descending, "sequence")
            .into_erased()
            .canonical_basis(),
        changed_shape.canonical_basis()
    );
    assert_eq!(
        left.read_graph().relations()[0].relation(),
        "AccountActivity"
    );
    assert_eq!(
        left.read_graph().ordering()[0].collection_path(),
        "root/relation[0]"
    );
    assert_eq!(
        left.read_graph().ordering()[0].slot_type(),
        std::any::type_name::<ActivitySequenceSlot>()
    );
}

#[test]
fn installed_public_query_owns_one_exact_graph_read_obligation() {
    let schema = installed_schema();
    let query = schema.application_query(query_reference()).unwrap();
    let [read] = query.obligations().rows() else {
        panic!("public query must install exactly one graph-read obligation");
    };

    assert_eq!(
        read.kind(),
        WorthQueryInstalledGraphObligationKind::GraphRead
    );
    assert_eq!(
        read.owner_progression(),
        [
            WorthQueryInstalledGraphObligationOwner::Relational,
            WorthQueryInstalledGraphObligationOwner::QueryExecution,
        ]
    );
    assert!(matches!(
        read.selection_basis(),
        WorthQueryInstalledGraphObligationSelectionBasis::ApplicationQueryGraph(_)
    ));
    assert!(matches!(
        read.resource_posture(),
        WorthQueryInstalledGraphObligationResourcePosture::ApplicationQuery { .. }
    ));
    assert_eq!(
        read.effect_posture(),
        WorthQueryInstalledGraphObligationEffectPosture::Observational
    );
    assert_eq!(
        read.terminal_requirement(),
        WorthQueryInstalledGraphObligationTerminalRequirement::GraphReadProduct
    );
}

#[test]
fn installed_query_rejects_foreign_runtime_and_successor_generation_substitution() {
    let index = installed_index();
    let schema = index
        .bind_application_schema(QueryTestSchema::declaration().unwrap())
        .unwrap();
    let query = schema.application_query(query_reference()).unwrap();

    let foreign = installed_schema()
        .validate_installed_query(&query)
        .unwrap_err();
    assert_eq!(
        foreign.kind(),
        WorthQueryApplicationQueryInstallationDenialKind::ForeignRuntime
    );
    let successor = index.successor_generation();
    let successor_schema = successor
        .bind_application_schema(QueryTestSchema::declaration().unwrap())
        .unwrap();
    let stale = successor_schema
        .validate_installed_query(&query)
        .unwrap_err();
    assert_eq!(
        stale.kind(),
        WorthQueryApplicationQueryInstallationDenialKind::StaleGeneration
    );
}

#[test]
fn authorization_scope_is_identity_bearing() {
    let account_scoped = definition_with_scope(
        Account::reference(),
        ApplicationQueryOrderingDirection::Descending,
        "sequence",
    )
    .into_erased();
    let activity_scoped = definition_with_scope(
        Activity::reference(),
        ApplicationQueryOrderingDirection::Descending,
        "sequence",
    )
    .into_erased();

    assert_ne!(
        account_scoped.canonical_basis(),
        activity_scoped.canonical_basis()
    );
    assert_eq!(account_scoped.scope_entity(), "Account");
    assert_eq!(activity_scoped.scope_entity(), "Activity");
}

fn installed_schema() -> crate::facade::WorthQueryInstalledApplicationSchema<QueryTestSchema> {
    installed_index()
        .bind_application_schema(QueryTestSchema::declaration().unwrap())
        .unwrap()
}

fn installed_index() -> WorthQueryInstalledPackageIndex {
    installed_index_with(WorthQueryInstallationRuntimeIdentity::fresh(), false)
}

fn installed_index_with(
    runtime: WorthQueryInstallationRuntimeIdentity,
    package_drift: bool,
) -> WorthQueryInstalledPackageIndex {
    let mut package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "application_query_installation_test",
        1,
        0,
    ))
    .application_schema(QueryTestSchema::declaration().unwrap());
    if package_drift {
        package = package.definition(WorthQueryPortableDefinition::declaration_family(
            "extra",
            "query-package-drift",
        ));
    }
    let package = package.validate().unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    WorthQueryInstalledPackageIndex::build(
        runtime,
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap()
}
