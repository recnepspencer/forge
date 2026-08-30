use super::{
    WorthQueryPackageBuildMetadata, WorthQueryPackageReleaseMetadata,
    WorthQueryPackageReleaseProvenance, WorthQueryPackageReleaseSignerDescriptor,
};

/// Complete host-supplied descriptive fields covered by one release signature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageReleaseEnvelopeDescriptor {
    build_metadata: WorthQueryPackageBuildMetadata,
    release_metadata: WorthQueryPackageReleaseMetadata,
    provenance: WorthQueryPackageReleaseProvenance,
    signer: WorthQueryPackageReleaseSignerDescriptor,
}

impl WorthQueryPackageReleaseEnvelopeDescriptor {
    pub const fn new(
        build_metadata: WorthQueryPackageBuildMetadata,
        release_metadata: WorthQueryPackageReleaseMetadata,
        provenance: WorthQueryPackageReleaseProvenance,
        signer: WorthQueryPackageReleaseSignerDescriptor,
    ) -> Self {
        Self {
            build_metadata,
            release_metadata,
            provenance,
            signer,
        }
    }

    pub const fn build_metadata(&self) -> &WorthQueryPackageBuildMetadata {
        &self.build_metadata
    }
    pub const fn release_metadata(&self) -> &WorthQueryPackageReleaseMetadata {
        &self.release_metadata
    }
    pub const fn provenance(&self) -> &WorthQueryPackageReleaseProvenance {
        &self.provenance
    }
    pub const fn signer(&self) -> &WorthQueryPackageReleaseSignerDescriptor {
        &self.signer
    }
}
