use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDeclarationRoutePlanInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.receipt"
    }

    fn display_name(&self) -> &'static str {
        "GeometryReceiptDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
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
        format!("geometry.receipt.{}", self.regime)
    }
}

macro_rules! define_family {
    ($name:ident, $contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(super) struct $name;

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
    RelationalReceiptFamily,
    ForgeQueryDeclarationRouteContract::relational_only()
);
define_family!(
    MixedReceiptFamily,
    ForgeQueryDeclarationRouteContract::relational_and_bridge()
);
define_family!(
    RequiredIntentReceiptFamily,
    ForgeQueryDeclarationRouteContract::required_relational_intent()
);
define_family!(
    DeferredReceiptFamily,
    ForgeQueryDeclarationRouteContract::deferred_auto()
);
define_family!(
    ForbiddenIntentReceiptFamily,
    ForgeQueryDeclarationRouteContract::relational_intent_forbidden()
);
define_family!(
    FailedReceiptFamily,
    ForgeQueryDeclarationRouteContract::unresolved_mixed()
);
define_family!(
    SignalReceiptFamily,
    ForgeQueryDeclarationRouteContract::signal_only()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectRichReceiptFamily;

fn aspect_rich_contract() -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.local_topology"],
        &["selection.material_edit"],
        &["selection.private_authority"],
        &[],
    )
}

fn aspect_rich_coverage() -> ForgeQueryDeclarationAspectCoverage {
    ForgeQueryDeclarationAspectCoverage::from_slices(
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

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AspectRichReceiptFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectRichReceiptFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        aspect_rich_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        aspect_rich_coverage()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectDeferredReceiptFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AspectDeferredReceiptFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectDeferredReceiptFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        aspect_rich_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        aspect_rich_coverage()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::deferred_auto()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectSignalReceiptFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AspectSignalReceiptFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectSignalReceiptFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        aspect_rich_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        aspect_rich_coverage()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::signal_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectFailedReceiptFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AspectFailedReceiptFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectFailedReceiptFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        aspect_rich_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        aspect_rich_coverage()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::unresolved_mixed()
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
            impl ForgeQueryDeclarationInput<GeometryDomain> for ReceiptInput<$family> {
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
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld::named(regime))
        .validate()
        .expect("receipt world should validate")
        .admit()
        .expect("receipt world should admit")
}

pub(super) fn progressed<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: ReceiptInput<F>,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<GeometryDomain, ReceiptInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    ReceiptInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("receipt progression should admit"))
}

pub(super) fn foundational_from_progressed<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    progression: crate::application::ForgeQueryAdmittedDeclarationProgression<
        GeometryDomain,
        ReceiptInput<F>,
    >,
) -> crate::application::ForgeQueryDeclarationFoundationalEvidence<GeometryDomain, ReceiptInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    ReceiptInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .describe_foundational(
            crate::application::ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression,
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"))
}

pub(super) fn route_checked_from_input<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: ReceiptInput<F>,
) -> crate::application::ForgeQueryDeclarationRoutePlanChecked<GeometryDomain, ReceiptInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    ReceiptInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let progression = progressed(handle, declaration);
    let evidence = foundational_from_progressed(handle, progression.clone());
    handle.plan_routes_checked(ForgeQueryDeclarationRoutePlanInput::admitted(
        progression,
        evidence,
    ))
}

pub(super) fn route_checked_with_intent<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: ReceiptInput<F>,
    intent: ForgeQueryDeclarationRouteIntent,
) -> crate::application::ForgeQueryDeclarationRoutePlanChecked<GeometryDomain, ReceiptInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    ReceiptInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let progression = progressed(handle, declaration);
    let evidence = foundational_from_progressed(handle, progression.clone());
    handle.plan_routes_checked(ForgeQueryDeclarationRoutePlanInput::with_intent(
        progression,
        evidence,
        intent,
    ))
}
