#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIntegrityEvidenceRichness {
    Full,
    Reduced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIntegrityEvidenceProfile {
    richness: PhysicalIntegrityEvidenceRichness,
}

impl PhysicalIntegrityEvidenceProfile {
    pub const fn full() -> Self {
        Self {
            richness: PhysicalIntegrityEvidenceRichness::Full,
        }
    }

    pub const fn reduced() -> Self {
        Self {
            richness: PhysicalIntegrityEvidenceRichness::Reduced,
        }
    }

    pub const fn richness(self) -> PhysicalIntegrityEvidenceRichness {
        self.richness
    }

    pub const fn optional_forensic_material_count(self) -> u8 {
        match self.richness {
            PhysicalIntegrityEvidenceRichness::Full => 3,
            PhysicalIntegrityEvidenceRichness::Reduced => 0,
        }
    }
}
