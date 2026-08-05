#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryInstalledGraphObligationKind {
    GraphRead,
    AuthorizationObservation,
    MutationTouch,
    EffectApplication,
    InvariantExecution,
}

impl WorthQueryInstalledGraphObligationKind {
    pub const ALL: [Self; 5] = [
        Self::GraphRead,
        Self::AuthorizationObservation,
        Self::MutationTouch,
        Self::EffectApplication,
        Self::InvariantExecution,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GraphRead => "graph-read",
            Self::AuthorizationObservation => "authorization-observation",
            Self::MutationTouch => "mutation-touch",
            Self::EffectApplication => "effect-application",
            Self::InvariantExecution => "invariant-execution",
        }
    }

    pub(super) const fn index(self) -> usize {
        match self {
            Self::GraphRead => 0,
            Self::AuthorizationObservation => 1,
            Self::MutationTouch => 2,
            Self::EffectApplication => 3,
            Self::InvariantExecution => 4,
        }
    }
}
