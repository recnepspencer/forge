#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NmtTopologyScopeKind {
    OpenWire,
    OpenSheet,
    OpenRadialFan,
    OpenLayer,
}

impl NmtTopologyScopeKind {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::OpenWire => "open wire topology scope",
            Self::OpenSheet => "open sheet topology scope",
            Self::OpenRadialFan => "open radial fan topology scope",
            Self::OpenLayer => "open layer topology scope",
        }
    }
}
