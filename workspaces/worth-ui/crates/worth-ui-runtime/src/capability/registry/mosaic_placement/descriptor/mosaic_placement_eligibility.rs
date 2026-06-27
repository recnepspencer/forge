use super::{MosaicPlacementSource, MosaicPlacementTarget};

/// Source and target families that a mosaic placement policy makes eligible.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MosaicPlacementEligibility {
    source: MosaicPlacementSource,
    target: MosaicPlacementTarget,
}

impl MosaicPlacementEligibility {
    pub fn new(source: MosaicPlacementSource, target: MosaicPlacementTarget) -> Self {
        Self { source, target }
    }

    pub fn source(&self) -> &MosaicPlacementSource {
        &self.source
    }

    pub fn target(&self) -> &MosaicPlacementTarget {
        &self.target
    }
}
