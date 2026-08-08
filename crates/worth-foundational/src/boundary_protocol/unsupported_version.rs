use super::BoundaryProtocolVersion;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BoundaryProtocolUnsupportedVersionPosture {
    PredatesWindow,
    ExceedsWindow,
    Retired,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoundaryProtocolUnsupportedVersion {
    produced: BoundaryProtocolVersion,
    posture: BoundaryProtocolUnsupportedVersionPosture,
}

impl BoundaryProtocolUnsupportedVersion {
    pub(super) const fn new(
        produced: BoundaryProtocolVersion,
        posture: BoundaryProtocolUnsupportedVersionPosture,
    ) -> Self {
        Self { produced, posture }
    }

    pub const fn produced(self) -> BoundaryProtocolVersion {
        self.produced
    }

    pub const fn posture(self) -> BoundaryProtocolUnsupportedVersionPosture {
        self.posture
    }
}
