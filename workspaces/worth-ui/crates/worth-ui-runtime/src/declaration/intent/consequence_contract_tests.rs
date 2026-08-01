use crate::capability::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentDefinition, UiIntentId, UiIntentPayload,
    UiIntentPayloadFieldSet, UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
    UiIntentProductConsequenceFamilies, UiIntentProductConsequences, UiIntentProductOutcome,
    UiIntentSchema, UiSemanticInteractionFamily,
};

use super::resolve_consequence_contract;

const DECLARATION: &str = "test.intent.route";
const QUERY: &str = "test.intent.query";

struct EmptyPayload;
struct ProjectionOutcome;
struct CollectionOutcome;
struct ProjectionIntent;
struct CollectionIntent;

impl UiIntentPayload for EmptyPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("test.intent.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

impl UiIntentProductOutcome for ProjectionOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("test.intent.projection", 1);
    const CONSEQUENCE_FAMILIES: UiIntentProductConsequenceFamilies =
        UiIntentProductConsequenceFamilies::QUERY_PROJECTION;

    fn into_consequences(self) -> UiIntentProductConsequences {
        unreachable!("catalog tests never execute a product outcome")
    }
}

impl UiIntentProductOutcome for CollectionOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("test.intent.collection", 1);
    const CONSEQUENCE_FAMILIES: UiIntentProductConsequenceFamilies =
        UiIntentProductConsequenceFamilies::QUERY_COLLECTION_CHANGE;

    fn into_consequences(self) -> UiIntentProductConsequences {
        unreachable!("catalog tests never execute a product outcome")
    }
}

impl UiIntent for ProjectionIntent {
    type Payload = EmptyPayload;
    type ProductOutcome = ProjectionOutcome;

    const ID: UiIntentId = UiIntentId::stable("test.intent.projection");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

impl UiIntent for CollectionIntent {
    type Payload = EmptyPayload;
    type ProductOutcome = CollectionOutcome;

    const ID: UiIntentId = UiIntentId::stable("test.intent.collection");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

#[test]
fn projection_outcome_resolves_only_a_registered_projection_identity() {
    let projection_plan = projection_plan("projection-family");
    let resolved = resolve::<ProjectionIntent>(&projection_plan)
        .expect("the typed projection family resolves its projection registration");
    assert_eq!(resolved.query_collection_change().unwrap().as_str(), QUERY);

    let view_plan = installed_view_plan("projection-cross-family");
    assert_unknown(resolve::<ProjectionIntent>(&view_plan));
}

#[test]
fn collection_outcome_resolves_only_an_installed_view_identity() {
    let view_plan = installed_view_plan("collection-family");
    let resolved = resolve::<CollectionIntent>(&view_plan)
        .expect("the typed collection family resolves its installed view");
    assert_eq!(resolved.query_collection_change().unwrap().as_str(), QUERY);

    let projection_plan = projection_plan("collection-cross-family");
    assert_unknown(resolve::<CollectionIntent>(&projection_plan));
}

fn resolve<I: UiIntent>(
    plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
) -> Result<
    super::UiResolvedIntentConsequenceContract,
    super::super::UiIntentCatalogPreparationDenial,
> {
    resolve_consequence_contract(
        DECLARATION,
        &worth_ui_dsl::WorthUiIntentConsequenceContractSpec::query_collection_change(QUERY),
        &UiIntentDefinition::<I>::application_effect().descriptor(),
        plan,
    )
}

fn projection_plan(label: &str) -> worth_ui_query_binding::WorthUiQueryBindingPlan {
    let domain = worth_ui_query_binding::certification::worth_ui_installed_test_domain(label);
    let registration = worth_ui_query_binding::UiScalarProjectionRegistration::text(
        domain.projection_view(QUERY).unwrap(),
        worth_ui_query_binding::UiProjectionFieldRequirement::declared("status").unwrap(),
    );
    worth_ui_query_binding::WorthUiQueryBindingPlan::default()
        .register_scalar_projection(registration)
        .unwrap()
}

fn installed_view_plan(label: &str) -> worth_ui_query_binding::WorthUiQueryBindingPlan {
    let domain = worth_ui_query_binding::certification::worth_ui_installed_test_domain(label);
    let view = domain.live_measurement_view(QUERY).unwrap();
    worth_ui_query_binding::WorthUiQueryBindingPlan::default()
        .register_view(view)
        .unwrap()
}

fn assert_unknown(
    result: Result<
        super::UiResolvedIntentConsequenceContract,
        super::super::UiIntentCatalogPreparationDenial,
    >,
) {
    assert_eq!(
        result.unwrap_err(),
        super::super::UiIntentCatalogPreparationDenial::UnknownConsequenceQuery {
            declaration: DECLARATION.into(),
            query: QUERY.into(),
        }
    );
}
