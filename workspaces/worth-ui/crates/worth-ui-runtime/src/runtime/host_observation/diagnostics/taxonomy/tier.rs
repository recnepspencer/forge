#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiDiagnosticRichnessTier {
    Off,
    Minimal,
    Standard,
    Full,
    Support,
}

impl WorthUiDiagnosticRichnessTier {
    pub fn emits_rows(self) -> bool {
        !matches!(self, Self::Off)
    }

    pub fn emits_phase_references(self) -> bool {
        matches!(self, Self::Standard | Self::Full | Self::Support)
    }

    pub fn emits_query_links(self) -> bool {
        matches!(self, Self::Full | Self::Support)
    }

    pub fn emits_support_sections(self) -> bool {
        matches!(self, Self::Support)
    }
}
