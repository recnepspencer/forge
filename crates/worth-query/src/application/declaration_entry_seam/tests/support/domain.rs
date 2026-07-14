use std::marker::PhantomData;

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryBridgeContinuationAuthority, WorthQueryCapabilityFamily,
    WorthQueryConfig, WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationBridgeContinuationContract,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationRelationalTruthContract,
    WorthQueryDeclarationRouteContract, WorthQueryDeclarationSignalCompatibilityContract,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext, WorthQueryMixedAuthority,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture, WorthQuerySignalConfig, WorthQuerySignalDeferredPosture,
    WorthQuerySignalNotCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.entry-seam"
    }
    fn display_name(&self) -> &'static str {
        "GeometryEntrySeamDomain"
    }
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryWorld(pub &'static str);

impl WorthQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::HistoricalEvaluation,
            WorthQueryCapabilityFamily::WorkflowOrchestration,
            WorthQueryCapabilityFamily::PreviewSession,
            WorthQueryCapabilityFamily::QueryComposition,
            WorthQueryCapabilityFamily::LiveQuery,
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQueryConfigSectionFamily::Signal,
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
        impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = $authority;
            type SignalCompatibility = $signal;
            type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;
            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }
            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }
            fn route_contract() -> WorthQueryDeclarationRouteContract {
                $route
            }
            fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
                $rel
            }
            fn bridge_continuation_contract(
            ) -> Option<WorthQueryDeclarationBridgeContinuationContract> {
                $bridge
            }
            fn signal_compatibility_contract(
            ) -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
                $signal_contract
            }
        }
    };
}

define_family!(
    RelationalFamily,
    WorthQueryRelationalTruthAuthority,
    WorthQuerySignalNotCompatiblePosture,
    WorthQueryDeclarationRouteContract::relational_only(),
    Some(WorthQueryDeclarationRelationalTruthContract::authoritative_current_truth()),
    None,
    None
);
define_family!(
    BridgeSignalFamily,
    WorthQueryBridgeContinuationAuthority,
    WorthQuerySignalCompatiblePosture,
    WorthQueryDeclarationRouteContract::bridge_only(),
    None,
    Some(WorthQueryDeclarationBridgeContinuationContract::preview_session()),
    Some(WorthQueryDeclarationSignalCompatibilityContract::preview_derived_execution())
);
define_family!(
    DeferredSignalFamily,
    WorthQueryBridgeContinuationAuthority,
    WorthQuerySignalDeferredPosture,
    WorthQueryDeclarationRouteContract::bridge_only(),
    None,
    Some(WorthQueryDeclarationBridgeContinuationContract::runtime_route_current()),
    None
);
define_family!(
    MixedFamily,
    WorthQueryMixedAuthority,
    WorthQuerySignalCompatiblePosture,
    WorthQueryDeclarationRouteContract::relational_and_bridge(),
    Some(WorthQueryDeclarationRelationalTruthContract::grouped_truth()),
    Some(WorthQueryDeclarationBridgeContinuationContract::preview_session()),
    Some(WorthQueryDeclarationSignalCompatibilityContract::preview_derived_execution())
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthorityRichFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AuthorityRichFamily {
    type PrimaryAuthority = WorthQueryMixedAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AuthorityRichFamily"
    }
    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_and_bridge()
    }
    fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
        Some(
            WorthQueryDeclarationRelationalTruthContract::grouped_truth().with_required_aspects(
                WorthQueryDeclarationAspectContract::from_slices(
                    &["selection.active_face"],
                    &["selection.neighborhood"],
                    &[],
                    &[],
                    &[],
                ),
            ),
        )
    }
    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(
            WorthQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(WorthQueryDeclarationAspectContract::from_slices(
                    &["continuation.preview_ready"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }
    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(
            WorthQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
                .with_aspects(
                    WorthQueryDeclarationAspectContract::from_slices(
                        &["signal.material_edit"],
                        &[],
                        &[],
                        &[],
                        &[],
                    ),
                    WorthQueryDeclarationAspectContract::from_slices(
                        &["signal.preview_patch"],
                        &[],
                        &[],
                        &[],
                        &[],
                    ),
                ),
        )
    }
    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &["selection.neighborhood"],
            &["continuation.preview_ready", "signal.material_edit"],
            &["selection.private_authority"],
            &[],
        )
    }
    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_face",
                "selection.neighborhood",
                "continuation.preview_ready",
                "signal.material_edit",
                "selection.private_authority",
            ],
            &["selection.private_authority"],
            &[],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input<F>(pub &'static str, pub PhantomData<F>);
impl<F> Input<F> {
    pub fn new(edge_ref: &'static str) -> Self {
        Self(edge_ref, PhantomData)
    }
}

macro_rules! impl_input {
    ($($family:ty),+ $(,)?) => {$(
        impl WorthQueryDeclarationInput<GeometryDomain> for Input<$family> {
            type Family = $family;
            fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.0)]
            }
        }
    )+};
}
impl_input!(
    RelationalFamily,
    BridgeSignalFamily,
    DeferredSignalFamily,
    MixedFamily,
    AuthorityRichFamily
);

pub fn handle(
    regime: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld> {
    crate::application::domain_test_support::installed_declaration_context(
        GeometryDomain,
        GeometryWorld(regime),
        [
            crate::application::domain_test_support::family::<GeometryDomain, RelationalFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, BridgeSignalFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, DeferredSignalFamily>(
            ),
            crate::application::domain_test_support::family::<GeometryDomain, MixedFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, AuthorityRichFamily>(
            ),
        ],
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignallessWorld(pub &'static str);

impl WorthQueryDomainOperatingContext<GeometryDomain> for SignallessWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::HistoricalEvaluation,
            WorthQueryCapabilityFamily::WorkflowOrchestration,
            WorthQueryCapabilityFamily::PreviewSession,
            WorthQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
            WorthQueryConfigSectionFamily::RuntimeBridge,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("entry.seam.signalless.{}", self.0)
    }
}

pub fn signal_disabled_handle(
    regime: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<GeometryDomain, SignallessWorld> {
    crate::application::domain_test_support::installed_declaration_context(
        GeometryDomain,
        SignallessWorld(regime),
        [
            crate::application::domain_test_support::family::<GeometryDomain, RelationalFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, BridgeSignalFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, DeferredSignalFamily>(
            ),
            crate::application::domain_test_support::family::<GeometryDomain, MixedFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, AuthorityRichFamily>(
            ),
        ],
    )
}

pub fn bridge_signal_envelope<C: WorthQueryDomainOperatingContext<GeometryDomain>>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, C>,
    edge_ref: &'static str,
) -> WorthQueryDeclarationEnvelope<GeometryDomain, Input<BridgeSignalFamily>> {
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

pub fn authority_rich_envelope<C: WorthQueryDomainOperatingContext<GeometryDomain>>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, C>,
    edge_ref: &'static str,
) -> WorthQueryDeclarationEnvelope<GeometryDomain, Input<AuthorityRichFamily>> {
    let progressed =
        match handle.declare_review_and_progress(Input::<AuthorityRichFamily>::new(edge_ref)) {
            Ok(progressed) => progressed,
            Err(_) => panic!("progression should succeed"),
        };
    match handle.envelope_routes_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(_) => panic!("envelope should succeed"),
    }
}

pub fn admitted_plan() -> crate::runtime::WorthQueryAdmittedIntentPlan {
    admitted_plan_for(crate::basis_lifecycle::RawBasisIntent::CurrentHead)
}

pub fn admitted_branch_plan(branch_identity: &str) -> crate::runtime::WorthQueryAdmittedIntentPlan {
    admitted_plan_for(crate::basis_lifecycle::RawBasisIntent::BranchHead {
        branch_identity: branch_identity.to_string(),
        accessible: true,
    })
}

fn admitted_plan_for(
    raw_intent: crate::basis_lifecycle::RawBasisIntent,
) -> crate::runtime::WorthQueryAdmittedIntentPlan {
    let request =
        crate::intent_admission::WorthQueryRawIntentAdmissionRequest::basis_observation_lane(
            raw_intent,
        )
        .expect("basis-observation request should build");
    match crate::intent_admission::admit_runtime_intent_request(request) {
        crate::intent_admission::WorthQueryIntentAdmissionDecision::Admitted(plan) => plan,
        other => panic!("expected admitted basis-observation plan, got {other:?}"),
    }
}

pub fn lower_runtime_envelope(
    target_digest: &str,
) -> crate::runtime::WorthQueryLowerRuntimeBoundaryEnvelope {
    let request = crate::lower_runtime_routing::WorthQueryLowerRuntimeCapabilityRequest::new(
        crate::lower_runtime_routing::WorthQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        crate::lower_runtime_routing::WorthQueryLowerRuntimeRouteKind::RoutePlanning,
        crate::lower_runtime_routing::WorthQueryLowerRuntimeAuthorityOwner::Query,
        "signal-invalidation-routing",
        crate::lower_runtime_routing::WorthQueryLowerRuntimeSubjectIdentity::compose(
            "declaration-entry-seam-target",
        )
        .field_value(
            crate::evidence_identity::WorthQueryEvidenceTag::new("test_target"),
            target_digest,
        )
        .seal(),
    );
    let detail_identity = crate::evidence_identity::WorthQueryEvidenceIdentity::compose(
        crate::evidence_identity::WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
    )
    .field_value(
        crate::evidence_identity::WorthQueryEvidenceTag::new("test_detail"),
        "detail",
    )
    .seal();
    let eligibility = crate::lower_runtime_routing::WorthQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request,
        &detail_identity,
    );
    let route = crate::lower_runtime_routing::WorthQueryLowerRuntimeRoutePlan::new(
        eligibility,
        crate::lower_runtime_routing::WorthQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "declaration-entry-seam-route",
            &detail_identity,
        ),
    );
    let retained_evidence =
        crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
            "declaration-entry-seam-test",
            &crate::evidence_identity::WorthQueryEvidenceIdentity::compose(
                crate::evidence_identity::WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_value(
                crate::evidence_identity::WorthQueryEvidenceTag::new("test_retained"),
                format!("retained:{target_digest}"),
            )
            .seal(),
        );
    let boundary =
        crate::lower_runtime_routing::WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
            &route,
            &retained_evidence,
        );
    crate::runtime::WorthQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        crate::lower_runtime_routing::WorthQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        &route,
        &boundary,
        &retained_evidence,
    )
}
