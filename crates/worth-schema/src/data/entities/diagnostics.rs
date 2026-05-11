use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DiagnosticsEntityKind {
    WireInterpretation,
    ShellInterpretation,
}

impl DiagnosticsEntityKind {
    pub const WRAPPED_ALL: [super::EntityKind; 2] = [
        super::EntityKind::Diagnostics(Self::WireInterpretation),
        super::EntityKind::Diagnostics(Self::ShellInterpretation),
    ];

    pub const ALL: [Self; 2] = [Self::WireInterpretation, Self::ShellInterpretation];

    pub const fn kind_id(self) -> KindId {
        match self {
            Self::WireInterpretation => KindId(401),
            Self::ShellInterpretation => KindId(402),
        }
    }

    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::WireInterpretation => ".wire_interpretation",
            Self::ShellInterpretation => ".shell_interpretation",
        }
    }

    pub fn from_kind_id(kind_id: KindId) -> Option<Self> {
        Some(match kind_id {
            KindId(401) => Self::WireInterpretation,
            KindId(402) => Self::ShellInterpretation,
            _ => return None,
        })
    }
}
