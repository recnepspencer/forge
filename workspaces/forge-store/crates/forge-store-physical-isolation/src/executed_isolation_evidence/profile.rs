#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S5IsolationEvidenceRichness {
    MinimalRequired,
    Forensic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIsolationEvidenceProfile {
    richness: S5IsolationEvidenceRichness,
}

impl PhysicalIsolationEvidenceProfile {
    pub const fn minimal_required() -> Self {
        Self {
            richness: S5IsolationEvidenceRichness::MinimalRequired,
        }
    }

    pub const fn forensic() -> Self {
        Self {
            richness: S5IsolationEvidenceRichness::Forensic,
        }
    }

    pub const fn richness(self) -> S5IsolationEvidenceRichness {
        self.richness
    }

    pub const fn includes_optional_forensics(self) -> bool {
        matches!(self.richness, S5IsolationEvidenceRichness::Forensic)
    }
}
