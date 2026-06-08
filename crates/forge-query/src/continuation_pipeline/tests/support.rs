use std::marker::PhantomData;

mod handles;

use crate::application::{
    ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryContinuationExecutionReadmissionObservation,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQuerySignalCompatiblePosture,
};
use crate::continuation_pipeline::execution::drifted_observation_from_retained;
use crate::continuation_pipeline::{
    ForgeQueryPreparedContinuationDriftKind, ForgeQueryPreparedContinuationFreshnessPosture,
};
pub(crate) use handles::{
    admitted_handle, context_request, drifted_readmission_handle, envelope,
    historical_disabled_handle, historical_truth_view_request, preview_disabled_handle,
    preview_session_request, runtime_route_request, target_request,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContinuationDomain;

impl ForgeQueryDomainEntryMarker for ContinuationDomain {
    fn domain_key(&self) -> &'static str {
        "test.continuation.domain"
    }

    fn display_name(&self) -> &'static str {
        "ContinuationDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContinuationWorld(pub(super) &'static str);

impl ForgeQueryDomainOperatingContext<ContinuationDomain> for ContinuationWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::PreviewSession,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("continuation-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct LenientContinuationWorld(pub(super) &'static str);

impl ForgeQueryDomainOperatingContext<ContinuationDomain> for LenientContinuationWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Query]
    }

    fn context_identity_digest(&self) -> String {
        format!("continuation-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ReadmissionDrift {
    Stale,
    AsyncRequest,
    Replay,
    Remask,
    PreviewCrossedResidue,
    StaleCompletion,
    BasisMismatch,
    AuthorityMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DriftedContinuationWorld {
    pub(super) label: &'static str,
    pub(super) drift: ReadmissionDrift,
}

impl ForgeQueryDomainOperatingContext<ContinuationDomain> for DriftedContinuationWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::PreviewSession,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("continuation-world-{}", self.label)
    }

    fn continuation_execution_readmission_observation(
        &self,
        retained: &crate::continuation_pipeline::ForgeQueryPreparedContinuationExecutionReadmission,
        _support_snapshot: &crate::application::ForgeQueryDomainEntrySupportSnapshot,
    ) -> ForgeQueryContinuationExecutionReadmissionObservation {
        match self.drift {
            ReadmissionDrift::Stale => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stale,
                None,
                None,
                None,
            ),
            ReadmissionDrift::AsyncRequest => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(ForgeQueryPreparedContinuationDriftKind::AsyncRequest),
            ),
            ReadmissionDrift::Replay => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(ForgeQueryPreparedContinuationDriftKind::Replay),
            ),
            ReadmissionDrift::Remask => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(ForgeQueryPreparedContinuationDriftKind::Remask),
            ),
            ReadmissionDrift::PreviewCrossedResidue => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(ForgeQueryPreparedContinuationDriftKind::PreviewCrossedResidue),
            ),
            ReadmissionDrift::StaleCompletion => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(ForgeQueryPreparedContinuationDriftKind::StaleCompletion),
            ),
            ReadmissionDrift::BasisMismatch => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stable,
                Some(format!(
                    "{}:drifted",
                    retained.basis_witness().basis_identity_digest()
                )),
                None,
                None,
            ),
            ReadmissionDrift::AuthorityMismatch => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                Some(crate::basis_lifecycle::LowerRuntimeEvidenceAuthority::RuntimeBridgeFacade),
                None,
            ),
        }
    }
}

macro_rules! define_family {
    ($name:ident, $bridge:expr, $signal:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(super) struct $name;

        impl ForgeQueryDeclarationFamilyMarker<ContinuationDomain> for $name {
            type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
            type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
                ForgeQueryDeclarationAspectContract::from_slices(
                    &["selection.face"],
                    &[],
                    &[],
                    &[],
                    &[],
                )
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                ForgeQueryDeclarationRouteContract::bridge_only()
            }

            fn bridge_continuation_contract(
            ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
                Some($bridge)
            }

            fn signal_compatibility_contract(
            ) -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
                Some($signal)
            }
        }
    };
}

define_family!(
    RuntimeFamily,
    crate::application::ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current(),
    ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
);
define_family!(
    HistoricalFamily,
    crate::application::ForgeQueryDeclarationBridgeContinuationContract::truth_view_historical(),
    ForgeQueryDeclarationSignalCompatibilityContract::historical_derived_execution()
);
define_family!(
    PreviewFamily,
    crate::application::ForgeQueryDeclarationBridgeContinuationContract::preview_session(),
    ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Input<F> {
    id: &'static str,
    _marker: PhantomData<F>,
}

impl<F> Input<F> {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_input {
    ($($family:ty),+ $(,)?) => {$(
        impl ForgeQueryDeclarationInput<ContinuationDomain> for Input<$family> {
            type Family = $family;

            fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
            }
        }
    )+};
}

impl_input!(RuntimeFamily, HistoricalFamily, PreviewFamily);
