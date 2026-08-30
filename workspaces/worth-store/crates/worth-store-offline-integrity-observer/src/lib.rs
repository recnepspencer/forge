#![forbid(unsafe_code)]
//! Independent, descriptive physical-integrity observation contracts.

mod integrity_observation;

pub use integrity_observation::{
    OfflineIntegrityObservationLimits, OfflineIntegrityObservationLimitsDenial,
    OfflineIntegrityObservationRequest, OfflineIntegrityObservationRequestDenial,
    OfflineIntegrityReportBoundary, OfflineIntegrityReportBoundaryDenial,
    OfflineIntegrityReportDestination, OfflineIntegrityReportDestinationDenial,
    OfflineIntegrityRootProtocolDeclarations, OFFLINE_INTEGRITY_ROOT_PROTOCOL_DECLARATIONS,
    PHYSICAL_INTEGRITY_OBSERVATION_COMPATIBILITY, PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_IDENTITY,
    PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_VERSION,
};
