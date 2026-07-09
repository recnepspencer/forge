use std::marker::PhantomData;

mod handles;

use crate::application::{
    WorthQueryBridgeContinuationAuthority, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryContinuationExecutionReadmissionObservation,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationRouteContract,
    WorthQueryDeclarationSignalCompatibilityContract, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQuerySignalCompatiblePosture,
};
use crate::continuation_pipeline::execution::drifted_observation_from_retained;
use crate::continuation_pipeline::{
    WorthQueryPreparedContinuationDriftKind, WorthQueryPreparedContinuationFreshnessPosture,
};
pub(crate) use handles::{
    admitted_handle, context_request, drifted_readmission_handle, envelope,
    historical_disabled_handle, historical_truth_view_request, preview_disabled_handle,
    preview_session_request, runtime_route_request, target_request,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContinuationDomain;

impl WorthQueryDomainEntryMarker for ContinuationDomain {
    fn domain_key(&self) -> &'static str {
        "test.continuation.domain"
    }

    fn display_name(&self) -> &'static str {
        "ContinuationDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContinuationWorld(pub(super) &'static str);

impl WorthQueryDomainOperatingContext<ContinuationDomain> for ContinuationWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::QueryComposition,
            WorthQueryCapabilityFamily::PreviewSession,
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("continuation-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct LenientContinuationWorld(pub(super) &'static str);

impl WorthQueryDomainOperatingContext<ContinuationDomain> for LenientContinuationWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Query]
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

impl WorthQueryDomainOperatingContext<ContinuationDomain> for DriftedContinuationWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::QueryComposition,
            WorthQueryCapabilityFamily::PreviewSession,
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("continuation-world-{}", self.label)
    }

    fn continuation_execution_readmission_observation(
        &self,
        retained: &crate::continuation_pipeline::WorthQueryPreparedContinuationExecutionReadmission,
        _support_snapshot: &crate::application::WorthQueryDomainEntrySupportSnapshot,
    ) -> WorthQueryContinuationExecutionReadmissionObservation {
        match self.drift {
            ReadmissionDrift::Stale => drifted_observation_from_retained(
                retained,
                WorthQueryPreparedContinuationFreshnessPosture::Stale,
                None,
                None,
                None,
            ),
            ReadmissionDrift::AsyncRequest => drifted_observation_from_retained(
                retained,
                WorthQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(WorthQueryPreparedContinuationDriftKind::AsyncRequest),
            ),
            ReadmissionDrift::Replay => drifted_observation_from_retained(
                retained,
                WorthQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(WorthQueryPreparedContinuationDriftKind::Replay),
            ),
            ReadmissionDrift::Remask => drifted_observation_from_retained(
                retained,
                WorthQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(WorthQueryPreparedContinuationDriftKind::Remask),
            ),
            ReadmissionDrift::PreviewCrossedResidue => drifted_observation_from_retained(
                retained,
                WorthQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(WorthQueryPreparedContinuationDriftKind::PreviewCrossedResidue),
            ),
            ReadmissionDrift::StaleCompletion => drifted_observation_from_retained(
                retained,
                WorthQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                None,
                Some(WorthQueryPreparedContinuationDriftKind::StaleCompletion),
            ),
            ReadmissionDrift::BasisMismatch => drifted_observation_from_retained(
                retained,
                WorthQueryPreparedContinuationFreshnessPosture::Stable,
                Some(format!(
                    "{}:drifted",
                    retained.basis_witness().basis_identity_digest()
                )),
                None,
                None,
            ),
            ReadmissionDrift::AuthorityMismatch => drifted_observation_from_retained(
                retained,
                WorthQueryPreparedContinuationFreshnessPosture::Stable,
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

        impl WorthQueryDeclarationFamilyMarker<ContinuationDomain> for $name {
            type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
            type SignalCompatibility = WorthQuerySignalCompatiblePosture;
            type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn aspect_contract() -> WorthQueryDeclarationAspectContract {
                WorthQueryDeclarationAspectContract::from_slices(
                    &["selection.face"],
                    &[],
                    &[],
                    &[],
                    &[],
                )
            }

            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> WorthQueryDeclarationRouteContract {
                WorthQueryDeclarationRouteContract::bridge_only()
            }

            fn bridge_continuation_contract(
            ) -> Option<crate::application::WorthQueryDeclarationBridgeContinuationContract> {
                Some($bridge)
            }

            fn signal_compatibility_contract(
            ) -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
                Some($signal)
            }
        }
    };
}

define_family!(
    RuntimeFamily,
    crate::application::WorthQueryDeclarationBridgeContinuationContract::runtime_route_current(),
    WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
);
define_family!(
    HistoricalFamily,
    crate::application::WorthQueryDeclarationBridgeContinuationContract::truth_view_historical(),
    WorthQueryDeclarationSignalCompatibilityContract::historical_derived_execution()
);
define_family!(
    PreviewFamily,
    crate::application::WorthQueryDeclarationBridgeContinuationContract::preview_session(),
    WorthQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
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
        impl WorthQueryDeclarationInput<ContinuationDomain> for Input<$family> {
            type Family = $family;

            fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
            }
        }
    )+};
}

impl_input!(RuntimeFamily, HistoricalFamily, PreviewFamily);
