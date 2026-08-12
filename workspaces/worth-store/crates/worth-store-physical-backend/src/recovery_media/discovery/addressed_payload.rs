use worth_store_physical_format::RecordArtifactFile;

use super::{
    BoundedRecoveryFilesystemDiscovery, ObservedRecoveryArtifact, RecoveryDiscoveryFailure,
};

impl BoundedRecoveryFilesystemDiscovery {
    pub fn read_free_space_manifest(
        &mut self,
        generation: u64,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed(
            RecordArtifactFile::FreeSpaceManifest { generation },
            byte_limit,
        )
    }

    pub fn read_free_space_membership_block(
        &mut self,
        generation: u64,
        block: u64,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed(
            RecordArtifactFile::FreeSpaceMembershipBlock { generation, block },
            byte_limit,
        )
    }

    pub fn read_segment_manifest(
        &mut self,
        segment: u64,
        generation: u64,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed(
            RecordArtifactFile::SegmentManifest {
                segment,
                generation,
            },
            byte_limit,
        )
    }

    pub fn read_segment(
        &mut self,
        segment: u64,
        generation: u64,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed(
            RecordArtifactFile::Segment {
                segment,
                generation,
            },
            byte_limit,
        )
    }

    pub fn read_segment_membership_block(
        &mut self,
        generation: u64,
        block: u64,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed(
            RecordArtifactFile::SegmentMembershipBlock { generation, block },
            byte_limit,
        )
    }

    pub fn read_segment_range(
        &mut self,
        segment: u64,
        generation: u64,
        offset: u64,
        length: u32,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed_range(
            RecordArtifactFile::Segment {
                segment,
                generation,
            },
            offset,
            length,
            byte_limit,
        )
    }

    pub fn read_extent_manifest(
        &mut self,
        extent: u64,
        generation: u64,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed(
            RecordArtifactFile::ExtentManifest { extent, generation },
            byte_limit,
        )
    }

    pub fn read_extent(
        &mut self,
        extent: u64,
        generation: u64,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed(
            RecordArtifactFile::Extent { extent, generation },
            byte_limit,
        )
    }

    pub fn read_extent_range(
        &mut self,
        extent: u64,
        generation: u64,
        offset: u64,
        length: u32,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed_range(
            RecordArtifactFile::Extent { extent, generation },
            offset,
            length,
            byte_limit,
        )
    }
}
