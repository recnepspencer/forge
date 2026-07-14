use std::marker::PhantomData;

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryInstalledDomainDeclarationContext,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.envelope"
    }

    fn display_name(&self) -> &'static str {
        "GeometryEnvelopeDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryWorld {
    regime: &'static str,
}

impl GeometryWorld {
    pub(crate) fn named(regime: &'static str) -> Self {
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
        format!("geometry.envelope.{}", self.regime)
    }
}

macro_rules! define_family {
    ($name:ident, $contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) struct $name;

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
    RelationalEnvelopeFamily,
    WorthQueryDeclarationRouteContract::relational_only()
);
define_family!(
    MixedEnvelopeFamily,
    WorthQueryDeclarationRouteContract::relational_and_bridge()
);
define_family!(
    RequiredIntentEnvelopeFamily,
    WorthQueryDeclarationRouteContract::required_relational_intent()
);
define_family!(
    DeferredEnvelopeFamily,
    WorthQueryDeclarationRouteContract::deferred_auto()
);
define_family!(
    FailedEnvelopeFamily,
    WorthQueryDeclarationRouteContract::unresolved_mixed()
);
define_family!(
    SignalEnvelopeFamily,
    WorthQueryDeclarationRouteContract::signal_only()
);

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AspectRichEnvelopeFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AspectRichEnvelopeFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectRichEnvelopeFamily"
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EnvelopeInput<F> {
    edge_ref: &'static str,
    _marker: PhantomData<F>,
}

impl<F> EnvelopeInput<F> {
    pub(crate) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_declaration_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl WorthQueryDeclarationInput<GeometryDomain> for EnvelopeInput<$family> {
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
    RelationalEnvelopeFamily,
    MixedEnvelopeFamily,
    RequiredIntentEnvelopeFamily,
    DeferredEnvelopeFamily,
    FailedEnvelopeFamily,
    SignalEnvelopeFamily,
    AspectRichEnvelopeFamily,
);

pub(crate) fn admitted_handle(
    regime: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld> {
    WorthQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld::named(regime))
        .validate()
        .expect("envelope world should validate")
        .admit()
        .expect("envelope world should admit")
}
