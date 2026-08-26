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
    worth_query_entity, worth_query_field, worth_query_portable_type, worth_query_relation,
};

use crate::facade::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryInstallationRuntimeIdentity, WorthQueryInstalledPackageIndex,
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
worth_query_aspect!(pub AccountFacts in QueryTestSchema, Account; identity = AspectIdentity(0x9161104a), revision = AspectContractRevision(1),);
worth_query_aspect!(pub ActivityFacts in QueryTestSchema, Activity; identity = AspectIdentity(0x9161104b), revision = AspectContractRevision(1),);
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

worth_query_portable_type!(ActivityQueryResult => "worth.query.test.installation.activity-result.v1");
worth_query_portable_type!(AccountIdSlot => "worth.query.test.installation.account-id-slot.v1");
worth_query_portable_type!(ActivitySequenceSlot => "worth.query.test.installation.sequence-slot.v1");
worth_query_portable_type!(ActivityRelationSlot => "worth.query.test.installation.relation-slot.v1");

worth_query_application_query!(
    pub(super) ActivityQuery in QueryTestSchema,
    parameters ActivityQueryParameters,
    result ActivityQueryResult,
    scope Account,
    name "account_activity"
);
worth_query_application_query!(
    ActivityScopedQuery in QueryTestSchema,
    parameters ActivityQueryParameters,
    result ActivityQueryResult,
    scope Activity,
    name "activity_scoped_account_activity"
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

fn account_parameter<Query>() -> ApplicationQueryParameterRef<Query, AccountParameter, u64> {
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
    definition_for::<ActivityQuery, Account, ActivitySequenceSlot>(
        ActivityQuery::reference(),
        Account::reference(),
        direction,
        output_name,
    )
}

pub(super) fn definition_with_sequence_slot<
    SequenceSlot: worth_query_declaration::facade::portable_identity::WorthQueryPortableType,
>(
    direction: ApplicationQueryOrderingDirection,
    output_name: &'static str,
) -> ApplicationQueryDefinition<
    QueryTestSchema,
    ActivityQuery,
    ActivityQueryParameters,
    ActivityQueryResult,
    Account,
> {
    definition_for::<ActivityQuery, Account, SequenceSlot>(
        ActivityQuery::reference(),
        Account::reference(),
        direction,
        output_name,
    )
}

fn definition_for<
    Query,
    Scope,
    SequenceSlot: worth_query_declaration::facade::portable_identity::WorthQueryPortableType,
>(
    reference: ApplicationQueryReference<
        QueryTestSchema,
        Query,
        ActivityQueryParameters,
        ActivityQueryResult,
        Scope,
    >,
    scope: ApplicationEntityRef<QueryTestSchema, Scope>,
    direction: ApplicationQueryOrderingDirection,
    output_name: &'static str,
) -> ApplicationQueryDefinition<
    QueryTestSchema,
    Query,
    ActivityQueryParameters,
    ActivityQueryResult,
    Scope,
>
where
    Query: worth_query_declaration::facade::application_query::ApplicationQueryMarkerIdentity,
{
    let sequence =
        ApplicationQueryResultFieldRef::<Query, SequenceSlot, _, _, _, _, _, _, _, _>::new(
            output_name,
            ActivitySequence::reference(),
        );
    let nested = ApplicationQueryResultShapeBuilder::<QueryTestSchema, Query, Activity, ()>::new(
        Activity::reference(),
    )
    .field(sequence);
    let shape = ApplicationQueryResultShapeBuilder::<
        QueryTestSchema,
        Query,
        Account,
        ActivityQueryResult,
    >::new(Account::reference())
    .field(ApplicationQueryResultFieldRef::<
        Query,
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
            Query,
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
    ApplicationQueryDefinitionBuilder::declare(reference)
        .root(Account::reference())
        .scope(scope)
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::Many)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(1, 1, 2))
        .disclosure(ApplicationQueryDisclosureContract::installed_policy(
            "account-holder",
        ))
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot().with_historical())
        .public()
        .parameter(account_parameter::<Query>())
        .where_equal(AccountId::reference(), account_parameter::<Query>())
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
    assert_eq!(
        left.graph_obligations().identity(),
        equivalent.graph_obligations().identity()
    );
    assert_eq!(left.graph_obligations().rows().len(), 1);
    assert_eq!(
        left.graph_obligations().rows()[0].kind(),
        crate::graph_obligation::WorthQueryInstalledGraphObligationKind::GraphRead
    );
    assert_eq!(
        left.graph_obligations()
            .installation_evidence()
            .canonical_work()
            .digest_text_materializations(),
        0
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
        <ActivitySequenceSlot as worth_query_declaration::facade::portable_identity::WorthQueryPortableType>::PORTABLE_TYPE_IDENTITY.as_str()
    );
}

#[test]
fn authorization_scope_is_identity_bearing() {
    let account_scoped = definition_for::<ActivityQuery, Account, ActivitySequenceSlot>(
        ActivityQuery::reference(),
        Account::reference(),
        ApplicationQueryOrderingDirection::Descending,
        "sequence",
    )
    .into_erased();
    let activity_scoped = definition_for::<ActivityScopedQuery, Activity, ActivitySequenceSlot>(
        ActivityScopedQuery::reference(),
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
