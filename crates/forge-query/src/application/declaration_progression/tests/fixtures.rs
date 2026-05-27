use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
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
        "test.geometry.progression"
    }

    fn display_name(&self) -> &'static str {
        "GeometryProgressionDomain"
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
    fn named(regime: &'static str) -> Self {
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
    (
        $name:ident,
        $authority:ty,
        $signal:ty,
        $grouped:ty,
        $aspect:expr,
        $legality:expr,
        $progression:expr
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(super) struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = $authority;
            type SignalCompatibility = $signal;
            type GroupedPosture = $grouped;

            fn semantic_family_key() -> &'static str {
                "split-edge"
            }

            fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
                $aspect
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
    AdmittedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[]
    ),
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
declare_family!(
    ReceiptFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[]
    ),
    ForgeQueryDeclarationLegalityContract::receipt_hot_boundary(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
declare_family!(
    DeferredFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[]
    ),
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::deferred_support()
);
declare_family!(
    DeniedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[]
    ),
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::denied_boundary()
);
declare_family!(
    StaleFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[]
    ),
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::stale_readable()
);
declare_family!(
    FailedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[]
    ),
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::failed_transition()
);
declare_family!(
    AlternateAspectFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_face"],
        &["selection.active_face"],
        &["selection.active_face"],
        &[],
        &[]
    ),
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
declare_family!(
    DescriptiveDeferredSignalFamily,
    ForgeQueryDescriptiveOnlyAuthority,
    ForgeQuerySignalDeferredPosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[]
    ),
    ForgeQueryDeclarationLegalityContract::receipt_hot_boundary(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MaskedCoverageFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for MaskedCoverageFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        ForgeQueryDeclarationProgressionContract::admitted_current()
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

macro_rules! impl_declaration_input {
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

impl_declaration_input!(
    AdmittedFamily,
    AlternateAspectFamily,
    ReceiptFamily,
    DeferredFamily,
    DeniedFamily,
    StaleFamily,
    FailedFamily,
    DescriptiveDeferredSignalFamily,
    MaskedCoverageFamily,
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

pub(super) fn legal<F>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        GeometryDomain,
        CollaborativeWorld,
    >,
    declaration: Declaration<F>,
) -> crate::application::ForgeQueryDeclarationLegalityEvidence<GeometryDomain, Declaration<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    Declaration<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_and_review(declaration)
        .unwrap_or_else(|_| panic!("legality review should pass"))
}

pub(super) fn progressed<F>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        GeometryDomain,
        CollaborativeWorld,
    >,
    declaration: Declaration<F>,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<GeometryDomain, Declaration<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    Declaration<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("progression should admit"))
}
