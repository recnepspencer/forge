#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterEvidenceStrength {
    Exact,
    Bounded,
    Sampled,
    Derived,
    CertificationOnly,
    Unavailable,
}

impl CounterEvidenceStrength {
    pub const fn satisfies(self, required: Self) -> bool {
        match required {
            Self::Exact => matches!(self, Self::Exact),
            Self::Bounded => matches!(self, Self::Exact | Self::Bounded),
            Self::Sampled => matches!(self, Self::Exact | Self::Bounded | Self::Sampled),
            Self::Derived => matches!(self, Self::Exact | Self::Derived),
            Self::CertificationOnly => !matches!(self, Self::Unavailable),
            Self::Unavailable => matches!(self, Self::Unavailable),
        }
    }

    pub const fn is_declared(self) -> bool {
        true
    }
}
