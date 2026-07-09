use crate::{
    ArtifactCompatibilityDenial, ArtifactFormatVersion, BackwardReadCompatibilityWitness,
    ForwardReadCompatibilityWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactCompatibilityWindow {
    minimum_readable: ArtifactFormatVersion,
    write_version: ArtifactFormatVersion,
    maximum_readable: ArtifactFormatVersion,
}

impl ArtifactCompatibilityWindow {
    pub fn new(
        minimum_readable: ArtifactFormatVersion,
        write_version: ArtifactFormatVersion,
        maximum_readable: ArtifactFormatVersion,
    ) -> Result<Self, ArtifactCompatibilityDenial> {
        if minimum_readable.0 > maximum_readable.0 {
            return Err(ArtifactCompatibilityDenial::EmptyCompatibilityWindow);
        }
        if write_version.0 < minimum_readable.0 || write_version.0 > maximum_readable.0 {
            return Err(ArtifactCompatibilityDenial::WriteVersionOutsideCompatibilityWindow);
        }
        Ok(Self {
            minimum_readable,
            write_version,
            maximum_readable,
        })
    }

    pub const fn minimum_readable(self) -> ArtifactFormatVersion {
        self.minimum_readable
    }

    pub const fn write_version(self) -> ArtifactFormatVersion {
        self.write_version
    }

    pub const fn maximum_readable(self) -> ArtifactFormatVersion {
        self.maximum_readable
    }

    pub const fn contains(self, version: ArtifactFormatVersion) -> bool {
        version.0 >= self.minimum_readable.0 && version.0 <= self.maximum_readable.0
    }

    pub fn admit_backward_read(
        self,
        version: ArtifactFormatVersion,
    ) -> Result<BackwardReadCompatibilityWitness, ArtifactCompatibilityDenial> {
        if self.contains(version) && version.0 <= self.write_version.0 {
            return Ok(BackwardReadCompatibilityWitness::new(self, version));
        }
        Err(ArtifactCompatibilityDenial::VersionOutsideCompatibilityWindow)
    }

    pub fn admit_forward_read(
        self,
        version: ArtifactFormatVersion,
    ) -> Result<ForwardReadCompatibilityWitness, ArtifactCompatibilityDenial> {
        if self.contains(version) && version.0 >= self.write_version.0 {
            return Ok(ForwardReadCompatibilityWitness::new(self, version));
        }
        Err(ArtifactCompatibilityDenial::VersionOutsideCompatibilityWindow)
    }
}
