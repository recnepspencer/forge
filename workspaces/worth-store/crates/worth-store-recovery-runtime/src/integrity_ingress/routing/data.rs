use worth_store::physical_runtime::ObservedRecoveryArtifact;
use worth_store_physical_integrity::{
    IntegrityValidatedExtentChunkFrame, IntegrityValidatedExtentManifest,
    IntegrityValidatedFreeSpaceHeader, IntegrityValidatedFreeSpaceMembershipBlock,
    IntegrityValidatedPageFrame, IntegrityValidatedSegmentMembershipBlock,
};

use super::super::admitted_artifact::IntegrityAdmittedRecoveryArtifact;
use super::super::families::{
    extent::{IntegrityAdmittedExtentChunkFrame, IntegrityAdmittedExtentManifest},
    free_space::{IntegrityAdmittedFreeSpaceHeader, IntegrityAdmittedFreeSpaceMembershipBlock},
    page::IntegrityAdmittedPageFrame,
    segment_membership::IntegrityAdmittedSegmentMembershipBlock,
};
use super::super::{ObservedRecoverySource, RecoveryIntegrityIngressCounters};
use super::{recorded, RecoveryIntegrityIngressAttempt};

macro_rules! complete_source_binding {
    ($name:ident, $validated:ty, $wrapper:ty, $variant:ident) => {
        pub(crate) fn $name(
            observed: &'media ObservedRecoveryArtifact,
            validated: $validated,
            counters: &mut RecoveryIntegrityIngressCounters,
        ) -> RecoveryIntegrityIngressAttempt<'media> {
            let scope = validated.scope();
            recorded(
                scope,
                <$wrapper>::bind(ObservedRecoverySource::complete(observed, scope), validated)
                    .map(Self::$variant),
                counters,
            )
        }
    };
}

impl<'media> IntegrityAdmittedRecoveryArtifact<'media> {
    complete_source_binding!(
        bind_segment_membership_block,
        IntegrityValidatedSegmentMembershipBlock<'media>,
        IntegrityAdmittedSegmentMembershipBlock<'media>,
        SegmentMembershipBlock
    );
    complete_source_binding!(
        bind_page_frame,
        IntegrityValidatedPageFrame<'media>,
        IntegrityAdmittedPageFrame<'media>,
        PageFrame
    );
    complete_source_binding!(
        bind_extent_manifest,
        IntegrityValidatedExtentManifest<'media>,
        IntegrityAdmittedExtentManifest<'media>,
        ExtentManifest
    );
    complete_source_binding!(
        bind_extent_chunk,
        IntegrityValidatedExtentChunkFrame<'media>,
        IntegrityAdmittedExtentChunkFrame<'media>,
        ExtentChunk
    );
    complete_source_binding!(
        bind_free_space_header,
        IntegrityValidatedFreeSpaceHeader<'media>,
        IntegrityAdmittedFreeSpaceHeader<'media>,
        FreeSpaceHeader
    );
    complete_source_binding!(
        bind_free_space_membership_block,
        IntegrityValidatedFreeSpaceMembershipBlock<'media>,
        IntegrityAdmittedFreeSpaceMembershipBlock<'media>,
        FreeSpaceMembershipBlock
    );
}
