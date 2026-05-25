use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily, ForgeQueryConfig,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationBridgeContinuationContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRelationalTruthContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySignalConfig, ForgeQuerySignalDeferredPosture,
    ForgeQuerySignalNotCompatiblePosture,
};
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution, ForgeQueryAdmissionContributionAuthoring,
    ForgeQueryAdmittedAdmissionContribution, ForgeQueryAdmittedAftermathContribution,
    ForgeQueryAdmittedContinuityContribution, ForgeQueryAdmittedExplanationContribution,
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryAdmittedSupportContribution,
    ForgeQueryAdmittedWorkflowContribution, ForgeQueryAftermathContributionAuthoring,
    ForgeQueryContinuityContributionAuthoring, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryExplanationContributionAuthoring, ForgeQuerySupportContributionAuthoring,
    ForgeQueryWorkflowContributionAuthoring,
};
use forge_proof::TransitionOutcome;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.entry-seam"
    }
    fn display_name(&self) -> &'static str {
        "GeometryEntrySeamDomain"
    }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryWorld(pub &'static str);

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::LiveQuery,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("entry.seam.{}", self.0)
    }
}

macro_rules! define_family {
    ($name:ident, $authority:ty, $signal:ty, $route:expr, $rel:expr, $bridge:expr, $signal_contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;
        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = $authority;
            type SignalCompatibility = $signal;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;
            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }
            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }
            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                $route
            }
            fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
                $rel
            }
            fn bridge_continuation_contract(
            ) -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
                $bridge
            }
            fn signal_compatibility_contract(
            ) -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
                $signal_contract
            }
        }
    };
}

define_family!(
    RelationalFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQueryDeclarationRouteContract::relational_only(),
    Some(ForgeQueryDeclarationRelationalTruthContract::authoritative_current_truth()),
    None,
    None
);
define_family!(
    BridgeSignalFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    None,
    Some(ForgeQueryDeclarationBridgeContinuationContract::preview_session()),
    Some(ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution())
);
define_family!(
    DeferredSignalFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalDeferredPosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    None,
    Some(ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current()),
    None
);
define_family!(
    MixedFamily,
    ForgeQueryMixedAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationRouteContract::relational_and_bridge(),
    Some(ForgeQueryDeclarationRelationalTruthContract::grouped_truth()),
    Some(ForgeQueryDeclarationBridgeContinuationContract::preview_session()),
    Some(ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution())
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input<F>(pub &'static str, pub PhantomData<F>);
impl<F> Input<F> {
    pub fn new(edge_ref: &'static str) -> Self {
        Self(edge_ref, PhantomData)
    }
}

macro_rules! impl_input {
    ($($family:ty),+ $(,)?) => {$(
        impl ForgeQueryDeclarationInput<GeometryDomain> for Input<$family> {
            type Family = $family;
            fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.0)]
            }
        }
    )+};
}
impl_input!(
    RelationalFamily,
    BridgeSignalFamily,
    DeferredSignalFamily,
    MixedFamily
);

pub fn handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld(regime))
        .validate()
        .expect("world should validate")
        .admit()
        .expect("world should admit")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignallessWorld(pub &'static str);

impl ForgeQueryDomainOperatingContext<GeometryDomain> for SignallessWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("entry.seam.signalless.{}", self.0)
    }
}

pub fn signal_disabled_handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, SignallessWorld> {
    ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_signal(ForgeQuerySignalConfig::disabled()),
    )
    .expect("signal-disabled config should validate")
    .domain(GeometryDomain)
    .with_operating_context(SignallessWorld(regime))
    .validate()
    .expect("world should validate")
    .admit()
    .expect("world should admit")
}

pub fn bridge_signal_envelope<C: ForgeQueryDomainOperatingContext<GeometryDomain>>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, C>,
    edge_ref: &'static str,
) -> ForgeQueryDeclarationEnvelope<GeometryDomain, Input<BridgeSignalFamily>> {
    let progressed =
        match handle.declare_review_and_progress(Input::<BridgeSignalFamily>::new(edge_ref)) {
            Ok(progressed) => progressed,
            Err(_) => panic!("progression should succeed"),
        };
    match handle.envelope_routes_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(_) => panic!("envelope should succeed"),
    }
}

pub fn admitted_declaration_support(
    declaration_digest: &str,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedSupportContribution<ForgeQueryDeclarationBoundContributionTarget> {
    admitted(
        ForgeQuerySupportContributionAuthoring::declaration_support(semantic_code, detail)
            .bind_to_declaration_target(ForgeQueryDeclarationBoundContributionTarget::from_digest(
                declaration_digest,
            )),
    )
}

pub fn admitted_declaration_explanation(
    declaration_digest: &str,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedExplanationContribution<ForgeQueryDeclarationBoundContributionTarget> {
    admitted(
        ForgeQueryExplanationContributionAuthoring::requires_context(semantic_code, detail)
            .bind_to_declaration_target(ForgeQueryDeclarationBoundContributionTarget::from_digest(
                declaration_digest,
            )),
    )
}

pub fn admitted_declaration_advisory(
    declaration_digest: &str,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedAdmissionContribution<ForgeQueryDeclarationBoundContributionTarget> {
    admitted(
        ForgeQueryAdmissionContributionAuthoring::advisory(semantic_code, detail)
            .bind_to_declaration_target(ForgeQueryDeclarationBoundContributionTarget::from_digest(
                declaration_digest,
            )),
    )
}

pub fn admitted_declaration_workflow(
    declaration_digest: &str,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedWorkflowContribution<ForgeQueryDeclarationBoundContributionTarget> {
    admitted_generic(
        ForgeQueryWorkflowContributionAuthoring::preview_only(semantic_code, detail)
            .bind_to_declaration_target(ForgeQueryDeclarationBoundContributionTarget::from_digest(
                declaration_digest,
            )),
    )
}

pub fn admitted_plan_support(
    plan: &crate::runtime::ForgeQueryAdmittedIntentPlan,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedSupportContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
    admitted_generic(
        ForgeQuerySupportContributionAuthoring::narrowed_support(semantic_code, detail)
            .for_admitted_intent_plan(plan),
    )
}

pub fn admitted_plan_workflow(
    plan: &crate::runtime::ForgeQueryAdmittedIntentPlan,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedWorkflowContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
    admitted_generic(
        ForgeQueryWorkflowContributionAuthoring::preview_only(semantic_code, detail)
            .for_admitted_intent_plan(plan),
    )
}

pub fn admitted_plan_continuity(
    plan: &crate::runtime::ForgeQueryAdmittedIntentPlan,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedContinuityContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
    admitted_generic(
        ForgeQueryContinuityContributionAuthoring::preserved(semantic_code, detail)
            .for_admitted_intent_plan(plan),
    )
}

pub fn admitted_plan_aftermath(
    plan: &crate::runtime::ForgeQueryAdmittedIntentPlan,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedAftermathContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
    admitted_generic(
        ForgeQueryAftermathContributionAuthoring::declares_residue(semantic_code, detail)
            .for_admitted_intent_plan(plan),
    )
}

pub fn admitted_lower_runtime_explanation(
    envelope: &crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedExplanationContribution<
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    admitted_generic(
        ForgeQueryExplanationContributionAuthoring::requires_context(semantic_code, detail)
            .for_lower_runtime_boundary_envelope(envelope),
    )
}

pub fn admitted_lower_runtime_aftermath(
    envelope: &crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedAftermathContribution<
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    admitted_generic(
        ForgeQueryAftermathContributionAuthoring::declares_residue(semantic_code, detail)
            .for_lower_runtime_boundary_envelope(envelope),
    )
}

pub fn admitted_plan() -> crate::runtime::ForgeQueryAdmittedIntentPlan {
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(
            crate::basis_lifecycle::RawBasisIntent::CurrentHead,
        )
        .expect("basis-observation request should build");
    match crate::intent_admission::admit_runtime_intent_request(request) {
        crate::intent_admission::ForgeQueryIntentAdmissionDecision::Admitted(plan) => plan,
        other => panic!("expected admitted basis-observation plan, got {other:?}"),
    }
}

pub fn lower_runtime_envelope(
    target_digest: &str,
) -> crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope {
    let request = crate::lower_runtime_routing::ForgeQueryLowerRuntimeCapabilityRequest::new(
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "signal-invalidation-routing",
        target_digest,
    );
    let eligibility =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
            request, "detail",
        );
    let route = crate::lower_runtime_routing::ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility,
        target_digest,
    );
    let boundary =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
            &route,
            format!("retained:{target_digest}"),
        );
    crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        &route,
        &boundary,
        &format!("retained:{target_digest}"),
    )
}

fn admitted<P>(
    requested: crate::domain_capabilities::ForgeQueryRequestedDomainCapabilityContribution<
        P,
        ForgeQueryDeclarationBoundContributionTarget,
    >,
) -> crate::domain_capabilities::ForgeQueryAdmittedDomainCapabilityContribution<
    P,
    ForgeQueryDeclarationBoundContributionTarget,
>
where
    P: crate::domain_capabilities::ForgeQueryDomainCapabilityPayload,
{
    admitted_generic(requested)
}

fn admitted_generic<P, T>(
    requested: crate::domain_capabilities::ForgeQueryRequestedDomainCapabilityContribution<P, T>,
) -> crate::domain_capabilities::ForgeQueryAdmittedDomainCapabilityContribution<P, T>
where
    P: crate::domain_capabilities::ForgeQueryDomainCapabilityPayload,
    T: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetBinding,
{
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    success(admit_eligible_domain_capability_contribution(eligible))
}

fn success<T>(
    outcome: crate::domain_capabilities::ForgeQueryDomainCapabilityTransitionOutcome<T>,
) -> T {
    match outcome {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(denial) => {
            panic!("expected success, got denial {:?}", denial.kind())
        }
        TransitionOutcome::Stale(stale) => {
            panic!("expected success, got stale {}", stale.category())
        }
        TransitionOutcome::RebindRequired(rebind) => {
            panic!("expected success, got rebind {}", rebind.category())
        }
        TransitionOutcome::Failed(failure) => {
            panic!("expected success, got failure {}", failure.message())
        }
        TransitionOutcome::Deferred(never) => match never {},
    }
}
