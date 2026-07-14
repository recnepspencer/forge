use crate::strategy::StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BTreeCorruptionRegion {
    Header,
    CellPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTreeNodeFormatLaw {
    minimum_occupancy: u16,
    maximum_occupancy: u16,
    stable_reads_require_published_generation: bool,
}

impl BTreeNodeFormatLaw {
    pub(crate) const fn baseline() -> Self {
        Self {
            minimum_occupancy: 2,
            maximum_occupancy: 8,
            stable_reads_require_published_generation: true,
        }
    }

    pub const fn minimum_occupancy(self) -> u16 {
        self.minimum_occupancy
    }

    pub const fn maximum_occupancy(self) -> u16 {
        self.maximum_occupancy
    }

    pub const fn stable_reads_require_published_generation(self) -> bool {
        self.stable_reads_require_published_generation
    }

    pub const fn verify_leaf_occupancy(self, occupied: u16) -> Result<(), StrategyDenial> {
        if occupied < self.minimum_occupancy || occupied > self.maximum_occupancy {
            return Err(StrategyDenial::OccupancyViolation);
        }
        Ok(())
    }

    pub const fn verify_checksum_localization(
        self,
        header_checksum_valid: bool,
        payload_checksum_valid: bool,
    ) -> Result<BTreeCorruptionRegion, StrategyDenial> {
        match (header_checksum_valid, payload_checksum_valid) {
            (false, true) => Ok(BTreeCorruptionRegion::Header),
            (true, false) => Ok(BTreeCorruptionRegion::CellPayload),
            _ => Err(StrategyDenial::ChecksumLocalizationViolation),
        }
    }
}
