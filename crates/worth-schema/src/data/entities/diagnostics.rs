use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthDiagnosticsEntityKind {
    WireInterpretation,
    ShellInterpretation,
}

impl WorthDiagnosticsEntityKind {
    pub const WRAPPED_ALL: [super::WorthEntityKind; 2] = [
        super::WorthEntityKind::Diagnostics(Self::WireInterpretation),
        super::WorthEntityKind::Diagnostics(Self::ShellInterpretation),
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
            Self::WireInterpretation => "worth.wire_interpretation",
            Self::ShellInterpretation => "worth.shell_interpretation",
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
