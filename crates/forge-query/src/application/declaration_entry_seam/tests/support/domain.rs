use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily, ForgeQueryConfig,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationBridgeContinuationContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRelationalTruthContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySignalConfig, ForgeQuerySignalDeferredPosture,
    ForgeQuerySignalNotCompatiblePosture,
};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthorityRichFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AuthorityRichFamily {
    type PrimaryAuthority = ForgeQueryMixedAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AuthorityRichFamily"
    }
    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }
    fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
        Some(
            ForgeQueryDeclarationRelationalTruthContract::grouped_truth().with_required_aspects(
                ForgeQueryDeclarationAspectContract::from_slices(
                    &["selection.active_face"],
                    &["selection.neighborhood"],
                    &[],
                    &[],
                    &[],
                ),
            ),
        )
    }
    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(
            ForgeQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(ForgeQueryDeclarationAspectContract::from_slices(
                    &["continuation.preview_ready"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }
    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(
            ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
                .with_aspects(
                    ForgeQueryDeclarationAspectContract::from_slices(
                        &["signal.material_edit"],
                        &[],
                        &[],
                        &[],
                        &[],
                    ),
                    ForgeQueryDeclarationAspectContract::from_slices(
                        &["signal.preview_patch"],
                        &[],
                        &[],
                        &[],
                        &[],
                    ),
                ),
        )
    }
    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &["selection.neighborhood"],
            &["continuation.preview_ready", "signal.material_edit"],
            &["selection.private_authority"],
            &[],
        )
    }
    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
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
    MixedFamily,
    AuthorityRichFamily
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

pub fn authority_rich_envelope<C: ForgeQueryDomainOperatingContext<GeometryDomain>>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, C>,
    edge_ref: &'static str,
) -> ForgeQueryDeclarationEnvelope<GeometryDomain, Input<AuthorityRichFamily>> {
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

pub fn admitted_plan() -> crate::runtime::ForgeQueryAdmittedIntentPlan {
    admitted_plan_for(crate::basis_lifecycle::RawBasisIntent::CurrentHead)
}

pub fn admitted_branch_plan(branch_identity: &str) -> crate::runtime::ForgeQueryAdmittedIntentPlan {
    admitted_plan_for(crate::basis_lifecycle::RawBasisIntent::BranchHead {
        branch_identity: branch_identity.to_string(),
        accessible: true,
    })
}

fn admitted_plan_for(
    raw_intent: crate::basis_lifecycle::RawBasisIntent,
) -> crate::runtime::ForgeQueryAdmittedIntentPlan {
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(
            raw_intent,
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
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "declaration-entry-seam-target",
        )
        .field_value(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("test_target"),
            target_digest,
        )
        .seal(),
    );
    let detail_identity = crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
    )
    .field_value(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("test_detail"),
        "detail",
    )
    .seal();
    let eligibility = crate::lower_runtime_routing::ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request,
        &detail_identity,
    );
    let route = crate::lower_runtime_routing::ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "declaration-entry-seam-route",
            &detail_identity,
        ),
    );
    let retained_evidence =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "declaration-entry-seam-test",
            &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_value(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("test_retained"),
                format!("retained:{target_digest}"),
            )
            .seal(),
        );
    let boundary =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
            &route,
            &retained_evidence,
        );
    crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        &route,
        &boundary,
        &retained_evidence,
    )
}
