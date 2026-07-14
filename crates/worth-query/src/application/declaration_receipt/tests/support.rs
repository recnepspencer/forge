use std::marker::PhantomData;

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryDeclarationRouteIntent,
    WorthQueryDeclarationRoutePlanInput, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryInstalledDomainDeclarationContext,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.receipt"
    }

    fn display_name(&self) -> &'static str {
        "GeometryReceiptDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryWorld {
    regime: &'static str,
}

impl GeometryWorld {
    pub(super) fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl WorthQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.receipt.{}", self.regime)
    }
}

macro_rules! define_family {
    ($name:ident, $contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(super) struct $name;

        impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
            type SignalCompatibility = WorthQuerySignalCompatiblePosture;
            type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> WorthQueryDeclarationRouteContract {
                $contract
            }
        }
    };
}

define_family!(
    RelationalReceiptFamily,
    WorthQueryDeclarationRouteContract::relational_only()
);
define_family!(
    MixedReceiptFamily,
    WorthQueryDeclarationRouteContract::relational_and_bridge()
);
define_family!(
    RequiredIntentReceiptFamily,
    WorthQueryDeclarationRouteContract::required_relational_intent()
);
define_family!(
    DeferredReceiptFamily,
    WorthQueryDeclarationRouteContract::deferred_auto()
);
define_family!(
    ForbiddenIntentReceiptFamily,
    WorthQueryDeclarationRouteContract::relational_intent_forbidden()
);
define_family!(
    FailedReceiptFamily,
    WorthQueryDeclarationRouteContract::unresolved_mixed()
);
define_family!(
    SignalReceiptFamily,
    WorthQueryDeclarationRouteContract::signal_only()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectRichReceiptFamily;

fn aspect_rich_contract() -> WorthQueryDeclarationAspectContract {
    WorthQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.local_topology"],
        &["selection.material_edit"],
        &["selection.private_authority"],
        &[],
    )
}

fn aspect_rich_coverage() -> WorthQueryDeclarationAspectCoverage {
    WorthQueryDeclarationAspectCoverage::from_slices(
        &[
            "selection.active_edge",
            "selection.local_topology",
            "selection.material_edit",
            "selection.private_authority",
        ],
        &["selection.private_authority"],
        &[],
    )
}

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AspectRichReceiptFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectRichReceiptFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        aspect_rich_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        aspect_rich_coverage()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectDeferredReceiptFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AspectDeferredReceiptFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectDeferredReceiptFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        aspect_rich_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        aspect_rich_coverage()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::deferred_auto()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectSignalReceiptFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AspectSignalReceiptFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectSignalReceiptFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        aspect_rich_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        aspect_rich_coverage()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::signal_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectFailedReceiptFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AspectFailedReceiptFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectFailedReceiptFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        aspect_rich_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        aspect_rich_coverage()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::unresolved_mixed()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ReceiptInput<F> {
    edge_ref: &'static str,
    _marker: PhantomData<F>,
}

impl<F> ReceiptInput<F> {
    pub(super) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_declaration_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl WorthQueryDeclarationInput<GeometryDomain> for ReceiptInput<$family> {
                type Family = $family;

                fn canonical_declaration_entries(
                    &self,
                ) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                    vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }
            }
        )+
    };
}

impl_declaration_input!(
    RelationalReceiptFamily,
    MixedReceiptFamily,
    RequiredIntentReceiptFamily,
    DeferredReceiptFamily,
    ForbiddenIntentReceiptFamily,
    FailedReceiptFamily,
    SignalReceiptFamily,
    AspectRichReceiptFamily,
    AspectDeferredReceiptFamily,
    AspectSignalReceiptFamily,
    AspectFailedReceiptFamily,
);

pub(super) fn admitted_handle(
    regime: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld> {
    crate::application::domain_test_support::installed_declaration_context(
        GeometryDomain,
        GeometryWorld::named(regime),
        [
            crate::application::domain_test_support::family::<
                GeometryDomain,
                RelationalReceiptFamily,
            >(),
            crate::application::domain_test_support::family::<GeometryDomain, MixedReceiptFamily>(),
            crate::application::domain_test_support::family::<
                GeometryDomain,
                RequiredIntentReceiptFamily,
            >(),
            crate::application::domain_test_support::family::<GeometryDomain, DeferredReceiptFamily>(
            ),
            crate::application::domain_test_support::family::<
                GeometryDomain,
                ForbiddenIntentReceiptFamily,
            >(),
            crate::application::domain_test_support::family::<GeometryDomain, FailedReceiptFamily>(
            ),
            crate::application::domain_test_support::family::<GeometryDomain, SignalReceiptFamily>(
            ),
            crate::application::domain_test_support::family::<
                GeometryDomain,
                AspectRichReceiptFamily,
            >(),
            crate::application::domain_test_support::family::<
                GeometryDomain,
                AspectDeferredReceiptFamily,
            >(),
            crate::application::domain_test_support::family::<
                GeometryDomain,
                AspectSignalReceiptFamily,
            >(),
            crate::application::domain_test_support::family::<
                GeometryDomain,
                AspectFailedReceiptFamily,
            >(),
        ],
    )
}

pub(super) fn progressed<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: ReceiptInput<F>,
) -> crate::application::WorthQueryAdmittedDeclarationProgression<GeometryDomain, ReceiptInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    ReceiptInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("receipt progression should admit"))
}

pub(super) fn foundational_from_progressed<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    progression: crate::application::WorthQueryAdmittedDeclarationProgression<
        GeometryDomain,
        ReceiptInput<F>,
    >,
) -> crate::application::WorthQueryDeclarationFoundationalEvidence<GeometryDomain, ReceiptInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    ReceiptInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    handle
        .describe_foundational(
            crate::application::WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression,
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"))
}

pub(super) fn route_checked_from_input<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: ReceiptInput<F>,
) -> crate::application::WorthQueryDeclarationRoutePlanChecked<GeometryDomain, ReceiptInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    ReceiptInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    let progression = progressed(handle, declaration);
    let evidence = foundational_from_progressed(handle, progression.clone());
    handle.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::admitted(
        progression,
        evidence,
    ))
}

pub(super) fn route_checked_with_intent<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: ReceiptInput<F>,
    intent: WorthQueryDeclarationRouteIntent,
) -> crate::application::WorthQueryDeclarationRoutePlanChecked<GeometryDomain, ReceiptInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    ReceiptInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    let progression = progressed(handle, declaration);
    let evidence = foundational_from_progressed(handle, progression.clone());
    handle.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::with_intent(
        progression,
        evidence,
        intent,
    ))
}
