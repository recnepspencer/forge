use super::*;

pub(super) fn installed_query(
) -> worth_query_installation::facade::WorthQueryInstalledApplicationQuery<
    PlanningTestSchema,
    ActivityQuery,
    ActivityParameters,
    ActivityResult,
    Account,
> {
    installed_schema()
        .application_query(query_reference())
        .unwrap()
}

pub(crate) fn installed_query_obligations(
) -> worth_query_installation::facade::WorthQueryInstalledGraphObligationSet {
    installed_query().obligations().clone()
}

pub(crate) fn installed_query_obligations_with_authority() -> (
    worth_query_installation::facade::WorthQueryInstalledGraphObligationSet,
    worth_query_installation::facade::WorthQueryInstalledGraphAdmissionAuthority,
) {
    let package = portable_package();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    let (index, authority) = WorthQueryInstalledPackageIndex::build_for_execution(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap();
    let schema = index
        .bind_application_schema(PlanningTestSchema::declaration().unwrap())
        .unwrap();
    (
        schema
            .application_query(query_reference())
            .unwrap()
            .obligations()
            .clone(),
        authority,
    )
}

pub(crate) fn installed_query_graph_read_review(
    required: crate::graph_obligation::WorthQueryRequiredGraphWork,
) -> crate::graph_obligation::WorthQueryReviewedApplicationQueryGraphWork {
    let query = installed_query();
    let parameters = admit_application_query_parameters(
        &query,
        ApplicationQueryParameterSet::new().bind(account_parameter(), 7_u64),
    )
    .unwrap();
    let requirements = admitted_requirements(
        query.read_family_binding().planning_contract(),
        WorthQueryApplicationQueryLane::OneShot,
        32,
        parameters.identity(),
    );
    crate::graph_obligation::review_application_query_graph_work(
        required,
        requirements,
        crate::graph_read_access::WorthQueryGraphIndexInventory::from_current_runtime_support(),
        crate::graph_read_access::WorthQueryGraphReadBudget::bounded(
            usize::MAX,
            usize::MAX,
            usize::MAX,
        ),
    )
    .unwrap()
}

pub(super) fn query_definition(
) -> worth_query_declaration::facade::application_query::ApplicationQueryDefinition<
    PlanningTestSchema,
    ActivityQuery,
    ActivityParameters,
    ActivityResult,
    Account,
> {
    let sequence =
        ApplicationQueryResultFieldRef::<ActivityQuery, SequenceSlot, _, _, _, _, _, _, _, _>::new(
            "sequence",
            Sequence::reference(),
        );
    let account_id = ApplicationQueryResultFieldRef::<
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
    >::new("account_id", AccountId::reference());
    let activity_relation = ApplicationQueryResultRelationRef::<
        ActivityQuery,
        ActivitySlot,
        _,
        _,
        _,
        _,
        ForwardResultTraversal,
        ManyResults,
    >::forward_many("activity", AccountActivity::reference());
    let nested =
        ApplicationQueryResultShapeBuilder::<PlanningTestSchema, ActivityQuery, Activity, ()>::new(
            Activity::reference(),
        )
        .field(sequence);
    let shape = ApplicationQueryResultShapeBuilder::<
        PlanningTestSchema,
        ActivityQuery,
        Account,
        ActivityResult,
    >::new(Account::reference())
    .field(account_id)
    .relation(activity_relation, nested)
    .build();
    ApplicationQueryDefinitionBuilder::public(
        query_reference(),
        Account::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(1, 1, 2),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot().with_live(),
    )
    .parameter(account_parameter())
    .where_equal(AccountId::reference(), account_parameter())
    .order_by(sequence, ApplicationQueryOrderingDirection::Ascending)
    .continue_by(activity_relation)
    .live_by::<Activity, live_lane::PlanningLiveCause, _, _, _, _, _, _, _, _>(
        account_id,
        sequence,
        ApplicationQueryLiveResourceContract::bounded(4, 2_048, 4_096),
    )
    .build()
    .unwrap()
}

fn installed_schema(
) -> worth_query_installation::facade::WorthQueryInstalledApplicationSchema<PlanningTestSchema> {
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(portable_package())
        .unwrap();
    WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap()
    .bind_application_schema(PlanningTestSchema::declaration().unwrap())
    .unwrap()
}

fn portable_package() -> worth_query_installation::facade::WorthQueryValidatedPortableDomainPackage
{
    WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "application_query_planning_test",
        1,
        0,
    ))
    .application_schema(PlanningTestSchema::declaration().unwrap())
    .validate()
    .unwrap()
}
