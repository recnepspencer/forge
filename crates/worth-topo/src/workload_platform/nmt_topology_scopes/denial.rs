use super::NmtTopologyScopeKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NmtTopologyScopeDenial {
    MissingScopeEvidence { reason: String },
    UnsupportedScopeShape { reason: String },
    MissingScopeKind { kind: NmtTopologyScopeKind },
}

impl NmtTopologyScopeDenial {
    pub(crate) fn missing(reason: impl Into<String>) -> Self {
        Self::MissingScopeEvidence {
            reason: reason.into(),
        }
    }

    pub(crate) fn unsupported(reason: impl Into<String>) -> Self {
        Self::UnsupportedScopeShape {
            reason: reason.into(),
        }
    }

    pub fn human_reason(&self) -> &str {
        match self {
            Self::MissingScopeEvidence { reason } | Self::UnsupportedScopeShape { reason } => {
                reason
            }
            Self::MissingScopeKind { kind } => kind.human_name(),
        }
    }
}
