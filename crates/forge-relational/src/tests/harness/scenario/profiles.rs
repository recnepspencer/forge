#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CertificationPressureProfile {
    Smoke,
    WindowSplit,
    HistoryPressure,
    RewriteStorm,
    ThousandStep,
}

impl CertificationPressureProfile {
    pub(crate) fn steps(self) -> usize {
        match self {
            Self::Smoke => 32,
            Self::WindowSplit => 96,
            Self::HistoryPressure => 256,
            Self::RewriteStorm => 1536,
            Self::ThousandStep => 1024,
        }
    }

    pub(crate) fn default_windows(self) -> &'static [usize] {
        match self {
            Self::Smoke => &[1, 2, 4, 8],
            Self::WindowSplit => &[1, 2, 3, 5, 8],
            Self::HistoryPressure => &[1, 2, 4, 8, 16],
            Self::RewriteStorm => &[1, 2, 3, 5, 8, 13, 21, 34],
            Self::ThousandStep => &[1, 2, 3, 5, 8, 13, 21],
        }
    }
}
