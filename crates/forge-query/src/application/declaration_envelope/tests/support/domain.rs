use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.envelope"
    }

    fn display_name(&self) -> &'static str {
        "GeometryEnvelopeDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
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
        format!("geometry.envelope.{}", self.regime)
    }
}

macro_rules! define_family {
    ($name:ident, $contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) struct $name;

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
    RelationalEnvelopeFamily,
    ForgeQueryDeclarationRouteContract::relational_only()
);
define_family!(
    MixedEnvelopeFamily,
    ForgeQueryDeclarationRouteContract::relational_and_bridge()
);
define_family!(
    RequiredIntentEnvelopeFamily,
    ForgeQueryDeclarationRouteContract::required_relational_intent()
);
define_family!(
    DeferredEnvelopeFamily,
    ForgeQueryDeclarationRouteContract::deferred_auto()
);
define_family!(
    FailedEnvelopeFamily,
    ForgeQueryDeclarationRouteContract::unresolved_mixed()
);
define_family!(
    SignalEnvelopeFamily,
    ForgeQueryDeclarationRouteContract::signal_only()
);

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
            impl ForgeQueryDeclarationInput<GeometryDomain> for EnvelopeInput<$family> {
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
    RelationalEnvelopeFamily,
    MixedEnvelopeFamily,
    RequiredIntentEnvelopeFamily,
    DeferredEnvelopeFamily,
    FailedEnvelopeFamily,
    SignalEnvelopeFamily,
);

pub(crate) fn admitted_handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld::named(regime))
        .validate()
        .expect("envelope world should validate")
        .admit()
        .expect("envelope world should admit")
}
