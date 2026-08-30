//! Pure physical-integrity validation contracts.
//!
//! This crate validates bounded untrusted physical bytes against exact format
//! declarations. Its results are descriptive and sealed: they grant no media,
//! resident-frame, decoder, recovery, quarantine, or repair authority.
#![forbid(unsafe_code)]

mod artifact;
mod localization;
mod observation;
mod quarantine;
mod scrub;
mod validation;

pub use artifact::{
    validate_current_root_selector, validate_inline_page, validate_physical_work_obligation,
    validate_previous_root_selector, validate_root_manifest,
    CurrentRootSelectorIntegrityValidation, InlinePageIntegrityValidation,
    PhysicalWorkObligationIntegrityValidation, PreviousRootSelectorIntegrityValidation,
    RootManifestIntegrityValidation,
};
pub use localization::{
    PhysicalBlastRadius, PhysicalByteRange, PhysicalByteRangeDenial, PhysicalDamageCause,
    PhysicalDamageLocalization, PhysicalFormatField,
};
pub use observation::{
    PhysicalIntegrityCounterDenial, PhysicalIntegrityObservationCounters,
    PhysicalIntegrityObservationOutcome, PhysicalIntegrityRejectionClass,
};
pub use quarantine::{PhysicalQuarantineObservation, PhysicalQuarantinePosture};
pub use scrub::{
    PhysicalIntegrityScrubCounters, PhysicalIntegrityScrubInspection, PhysicalIntegrityScrubWindow,
    PhysicalIntegrityScrubWindowOutcome,
};
pub use validation::{
    CheckpointStreamHeaderScopeIdentity, IndeterminatePhysicalIntegrityCause,
    IndeterminatePhysicalIntegrityPosture, IntegrityValidatedCurrentRootSelector,
    IntegrityValidatedPageFrame, IntegrityValidatedPhysicalWorkObligation,
    IntegrityValidatedPreviousRootSelector, IntegrityValidatedRootManifest, PhysicalArtifactScope,
    PhysicalArtifactScopeDenial, PhysicalIntegrityRejection, PhysicalIntegrityValidationDigest,
    PhysicalIntegrityValidationMechanism, PhysicalIntegrityValidationRecord,
    PhysicalIntegrityVersionAxis, UnknownPhysicalIntegrityCause, UnknownPhysicalIntegrityPosture,
    UnsupportedPhysicalIntegrityVersion, UntrustedPhysicalArtifact,
};
