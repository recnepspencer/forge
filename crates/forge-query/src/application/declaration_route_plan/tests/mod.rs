use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFoundationalEvidenceInput,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDeclarationRoutePlanDenialCause,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.route-plan"
    }

    fn display_name(&self) -> &'static str {
        "GeometryRoutePlanDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryWorld {
    regime: &'static str,
}

impl GeometryWorld {
    fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.route-plan.{}", self.regime)
    }
}

macro_rules! define_family {
    ($name:ident, $contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
            type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                $contract
            }
        }
    };
}

define_family!(
    RelationalRouteFamily,
    ForgeQueryDeclarationRouteContract::relational_only()
);
define_family!(
    MixedRouteFamily,
    ForgeQueryDeclarationRouteContract::relational_and_bridge()
);
define_family!(
    RequiredIntentFamily,
    ForgeQueryDeclarationRouteContract::required_relational_intent()
);
define_family!(
    ForbiddenIntentFamily,
    ForgeQueryDeclarationRouteContract::relational_intent_forbidden()
);
define_family!(
    DeferredRouteFamily,
    ForgeQueryDeclarationRouteContract::deferred_auto()
);
define_family!(
    FailedRouteFamily,
    ForgeQueryDeclarationRouteContract::unresolved_mixed()
);

#[derive(Clone, Debug, Eq, PartialEq)]
struct RouteInput<F> {
    edge_ref: &'static str,
    _marker: PhantomData<F>,
}

impl<F> RouteInput<F> {
    fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_declaration_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl ForgeQueryDeclarationInput<GeometryDomain> for RouteInput<$family> {
                type Family = $family;

                fn canonical_declaration_entries(
                    &self,
                ) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                    vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }
            }
        )+
    };
}

impl_declaration_input!(
    RelationalRouteFamily,
    MixedRouteFamily,
    RequiredIntentFamily,
    ForbiddenIntentFamily,
    DeferredRouteFamily,
    FailedRouteFamily,
);

fn admitted_handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld::named(regime))
        .validate()
        .expect("route-plan world should validate")
        .admit()
        .expect("route-plan world should admit")
}

fn progressed<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RouteInput<F>,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<GeometryDomain, RouteInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    RouteInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("route-plan progression should admit"))
}

fn route_input<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RouteInput<F>,
) -> super::ForgeQueryDeclarationRoutePlanInput<GeometryDomain, RouteInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    RouteInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let progressed = progressed(handle, declaration);
    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("same-handle foundational evidence should materialize"));
    super::ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence)
}

#[test]
fn route_plan_common_lane_reads_like_intent() {
    let plan = admitted_handle("primary")
        .declare_review_progress_describe_and_plan(RouteInput::<RelationalRouteFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("route plan should admit"));

    assert_eq!(plan.route_count(), 1);
    assert_eq!(plan.declaration_family_key(), "RelationalRouteFamily");
}

#[test]
fn explicit_and_common_paths_converge_on_one_route_plan_digest() {
    let handle = admitted_handle("primary");
    let explicit = handle
        .plan_routes(route_input(
            &handle,
            RouteInput::<MixedRouteFamily>::new("edge:42"),
        ))
        .unwrap_or_else(|_| panic!("explicit route planning should succeed"));
    let common = handle
        .declare_review_progress_describe_and_plan(RouteInput::<MixedRouteFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("common route planning should succeed"));

    assert_eq!(explicit.route_plan_digest(), common.route_plan_digest());
}

#[test]
fn route_planning_keeps_plural_routes_first_class() {
    let plan = admitted_handle("primary")
        .declare_review_progress_describe_and_plan(RouteInput::<MixedRouteFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("mixed route plan should succeed"));

    assert_eq!(plan.route_count(), 2);
    assert_eq!(plan.route_families().len(), 2);
}

#[test]
fn required_intent_is_a_typed_denial() {
    let handle = admitted_handle("primary");

    match handle.plan_routes_checked(route_input(
        &handle,
        RouteInput::<RequiredIntentFamily>::new("edge:42"),
    )) {
        ForgeQueryDeclarationRoutePlanChecked::Denied(denial) => {
            assert_eq!(
                denial.cause(),
                ForgeQueryDeclarationRoutePlanDenialCause::IntentRequired
            );
            assert!(denial
                .reason()
                .contains("requires explicit caller route intent"));
        }
        _ => panic!("required intent should deny without explicit route intent"),
    }
}

#[test]
fn forbidden_intent_is_a_typed_denial() {
    let handle = admitted_handle("primary");
    let progressed = progressed(&handle, RouteInput::<ForbiddenIntentFamily>::new("edge:42"));
    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("same-handle foundational evidence should materialize"));

    match handle.plan_routes_checked(super::ForgeQueryDeclarationRoutePlanInput::with_intent(
        progressed,
        evidence,
        ForgeQueryDeclarationRouteIntent::RelationalOnly,
    )) {
        ForgeQueryDeclarationRoutePlanChecked::Denied(denial) => {
            assert_eq!(
                denial.cause(),
                ForgeQueryDeclarationRoutePlanDenialCause::IntentForbidden
            );
            assert!(denial
                .reason()
                .contains("forbids caller-owned route narrowing"));
        }
        _ => panic!("forbidden intent should deny explicit route intent"),
    }
}

#[test]
fn route_planning_rejects_mismatched_admitted_world_inputs() {
    let primary = admitted_handle("primary");
    let alternate = admitted_handle("alternate");
    let primary_progressed = progressed(
        &primary,
        RouteInput::<RelationalRouteFamily>::new("edge:42"),
    );
    let alternate_evidence = alternate
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed(
                &alternate,
                RouteInput::<RelationalRouteFamily>::new("edge:42"),
            )),
        )
        .unwrap_or_else(|_| panic!("alternate foundational evidence should materialize"));

    match primary.plan_routes_checked(super::ForgeQueryDeclarationRoutePlanInput::admitted(
        primary_progressed,
        alternate_evidence,
    )) {
        ForgeQueryDeclarationRoutePlanChecked::Denied(denial) => {
            assert_eq!(
                denial.cause(),
                ForgeQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld
            );
        }
        _ => panic!("mismatched admitted worlds should deny route planning"),
    }
}

#[test]
fn deferred_and_failed_paths_remain_typed() {
    let handle = admitted_handle("primary");

    assert!(matches!(
        handle.plan_routes_checked(route_input(
            &handle,
            RouteInput::<DeferredRouteFamily>::new("edge:42"),
        )),
        ForgeQueryDeclarationRoutePlanChecked::Deferred(_)
    ));

    assert!(matches!(
        handle.plan_routes_checked(route_input(
            &handle,
            RouteInput::<FailedRouteFamily>::new("edge:42"),
        )),
        ForgeQueryDeclarationRoutePlanChecked::Failed(_)
    ));
}

#[test]
fn route_plan_digest_changes_when_admitted_world_changes() {
    let primary = admitted_handle("primary")
        .declare_review_progress_describe_and_plan(RouteInput::<RelationalRouteFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("primary world should plan"));
    let alternate = admitted_handle("alternate")
        .declare_review_progress_describe_and_plan(RouteInput::<RelationalRouteFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("alternate world should plan"));

    assert_ne!(primary.route_plan_digest(), alternate.route_plan_digest());
}

#[test]
fn route_plan_explanation_preserves_route_reasoning() {
    let plan = admitted_handle("primary")
        .declare_review_progress_describe_and_plan(RouteInput::<MixedRouteFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("mixed route plan should succeed"));

    assert!(plan
        .explain()
        .route_contract_reason()
        .contains("relational and bridge"));
    assert_eq!(plan.explain().route_segment_reasons().len(), 2);
    assert!(plan
        .explain()
        .retained_facts()
        .iter()
        .any(|fact| fact.contains("operating_context:geometry.route-plan.primary")));
}
