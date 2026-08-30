#![forbid(unsafe_code)]
//! Independent, bounded observation of the C.9 selector-to-root protocol.

mod integrity_observation;

pub use integrity_observation::{
    emit_offline_integrity_report, encode_offline_integrity_report, observe_store,
    OfflineArtifactDuplicateEvidence, OfflineArtifactFamily, OfflineArtifactObservation,
    OfflineIndeterminatePhysicalReason, OfflineIntegrityObservationCounters,
    OfflineIntegrityObservationDenial, OfflineIntegrityObservationLimits,
    OfflineIntegrityObservationLimitsDenial, OfflineIntegrityObservationRequest,
    OfflineIntegrityObservationRequestDenial, OfflineIntegrityOutcome,
    OfflineIntegrityProtocolContext, OfflineIntegrityProtocolContextDenial, OfflineIntegrityReport,
    OfflineIntegrityReportBoundaryDenial, OfflineIntegrityReportCompleteness,
    OfflineIntegrityReportDestination, OfflineIntegrityReportDestinationDenial,
    OfflineIntegrityReportEmissionDenial, OfflineIntegrityReportWireDenial,
    OfflineIntegrityRootProtocolDeclarations, OfflinePhysicalBlastRadius,
    OfflinePhysicalDamageCause, OfflinePhysicalDamageLocalization, OfflinePhysicalFormatField,
    OfflineUnknownPhysicalReason, OfflineUnsupportedPhysicalVersion, OfflineUnsupportedVersionAxis,
    OFFLINE_INTEGRITY_ROOT_PROTOCOL_DECLARATIONS, OFFLINE_OBSERVER_ROLE_IDENTITY,
    PHYSICAL_INTEGRITY_OBSERVATION_COMPATIBILITY, PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_IDENTITY,
    PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_VERSION,
};
