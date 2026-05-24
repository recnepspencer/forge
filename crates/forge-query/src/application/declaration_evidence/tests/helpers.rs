use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationProgressionContract, ForgeQueryDescriptiveOnlyAuthority,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySignalDeferredPosture,
    ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.evidence"
    }

    fn display_name(&self) -> &'static str {
        "GeometryEvidenceDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CollaborativeWorld {
    regime: &'static str,
}

impl CollaborativeWorld {
    pub(super) fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
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
        format!("geometry.{}", self.regime)
    }
}

macro_rules! declare_family {
    ($name:ident, $authority:ty, $signal:ty, $grouped:ty, $legality:expr, $progression:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(super) struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = $authority;
            type SignalCompatibility = $signal;
            type GroupedPosture = $grouped;

            fn semantic_family_key() -> &'static str {
                "split-edge"
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                $legality
            }

            fn progression_contract(
                _handle_identity_digest: &str,
                _operating_context_identity_digest: &str,
            ) -> ForgeQueryDeclarationProgressionContract {
                $progression
            }
        }
    };
}

declare_family!(
    LegalFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::receipt_hot_boundary(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
declare_family!(
    AdmittedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
declare_family!(
    DeferredFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::deferred_support()
);
declare_family!(
    DeniedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::denied_boundary()
);
declare_family!(
    StaleFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::stale_readable()
);
declare_family!(
    FailedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::failed_transition()
);
declare_family!(
    DescriptiveDeferredSignalFamily,
    ForgeQueryDescriptiveOnlyAuthority,
    ForgeQuerySignalDeferredPosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationLegalityContract::receipt_hot_boundary(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct IllegalRoleFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for IllegalRoleFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::unsupported_boundary()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WorldSensitiveFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for WorldSensitiveFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        if operating_context_identity_digest.contains("restricted") {
            ForgeQueryDeclarationProgressionContract::rebind_required()
        } else {
            ForgeQueryDeclarationProgressionContract::admitted_current()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Declaration<F> {
    edge_ref: &'static str,
    _family: PhantomData<F>,
}

impl<F> Declaration<F> {
    pub(super) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _family: PhantomData,
        }
    }
}

macro_rules! impl_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl ForgeQueryDeclarationInput<GeometryDomain> for Declaration<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                    vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }
            }
        )+
    };
}

impl_input!(
    LegalFamily,
    AdmittedFamily,
    DeferredFamily,
    DeniedFamily,
    StaleFamily,
    FailedFamily,
    DescriptiveDeferredSignalFamily,
    IllegalRoleFamily,
    WorldSensitiveFamily,
);

pub(super) fn admitted_handle(
    regime: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, CollaborativeWorld>
{
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(CollaborativeWorld::named(regime))
        .validate()
        .expect("context should validate")
        .admit()
        .expect("context should admit")
}

pub(super) fn digest_text(digest: &forge_foundational::facade::CanonicalDerivedDigest) -> String {
    format!("{digest:?}")
}
