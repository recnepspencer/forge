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
    validate_bootstrap_catalog, validate_checkpoint_binding,
    validate_checkpoint_binding_compaction, validate_checkpoint_dirty_basis,
    validate_checkpoint_footer, validate_checkpoint_stream_header, validate_current_root_selector,
    validate_extent_chunk, validate_extent_chunk_membership, validate_extent_manifest,
    validate_free_space_header, validate_free_space_membership_block, validate_inline_page,
    validate_physical_work_obligation, validate_previous_root_selector, validate_root_manifest,
    validate_root_routing_block, validate_segment_membership_block, validate_wal_frame,
    BootstrapCatalogIntegrityValidation, BootstrapCatalogScopeMismatch,
    BootstrapCatalogUnsupportedFormat, CheckpointBindingCompactionIntegrityValidation,
    CheckpointBindingIntegrityValidation, CheckpointDirtyBasisIntegrityValidation,
    CheckpointFooterIntegrityValidation, CheckpointFooterValidationBasis,
    CheckpointStreamHeaderIntegrityValidation, CurrentRootSelectorIntegrityValidation,
    ExtentChunkIntegrityValidation, ExtentManifestIntegrityValidation,
    FreeSpaceHeaderIntegrityValidation, FreeSpaceMembershipBlockIntegrityValidation,
    InlinePageIntegrityValidation, PhysicalWorkObligationIntegrityValidation,
    PreviousRootSelectorIntegrityValidation, RootManifestIntegrityValidation,
    RootRoutingBlockIntegrityValidation, SegmentMembershipBlockIntegrityValidation,
    WalFrameIntegrityValidation,
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
    CheckpointBindingPayloadProjectionDenial, CheckpointStreamHeaderScopeIdentity,
    ExtentChunkProjectionDenial, IndeterminatePhysicalIntegrityCause,
    IndeterminatePhysicalIntegrityPosture, InlineRecordProjectionDenial,
    IntegrityValidatedBootstrapCatalog, IntegrityValidatedCheckpointBinding,
    IntegrityValidatedCheckpointBindingCompaction,
    IntegrityValidatedCheckpointBindingPayloadProjection, IntegrityValidatedCheckpointDirtyBasis,
    IntegrityValidatedCheckpointFooter, IntegrityValidatedCheckpointStreamHeader,
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedExtentChunkFrame,
    IntegrityValidatedExtentChunkProjection, IntegrityValidatedExtentManifest,
    IntegrityValidatedExtentMembership, IntegrityValidatedFreeSpaceHeader,
    IntegrityValidatedFreeSpaceMembershipBlock, IntegrityValidatedInlineRecordProjection,
    IntegrityValidatedPageFrame, IntegrityValidatedPhysicalWorkObligation,
    IntegrityValidatedPreviousRootSelector, IntegrityValidatedRootManifest,
    IntegrityValidatedRootRoutingBlock, IntegrityValidatedSegmentMembershipBlock,
    IntegrityValidatedWalFrame, IntegrityValidatedWalPayloadProjection, PhysicalArtifactScope,
    PhysicalArtifactScopeDenial, PhysicalIntegrityArtifactVersionAdapter,
    PhysicalIntegrityEnvelopeVersionAdapter, PhysicalIntegrityRejection,
    PhysicalIntegritySupportedVersion, PhysicalIntegrityValidationDigest,
    PhysicalIntegrityValidationMechanism, PhysicalIntegrityValidationRecord,
    PhysicalIntegrityVersionAxis, PhysicalIntegrityVersionWindowOutcome,
    UnknownPhysicalIntegrityCause, UnknownPhysicalIntegrityPosture,
    UnsupportedPhysicalIntegrityVersion, UntrustedPhysicalArtifact, WalPayloadProjectionDenial,
};
