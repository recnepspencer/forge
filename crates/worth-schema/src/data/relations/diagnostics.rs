use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DiagnosticsRelationKind {
    WireHasInterpretation,
    ShellHasInterpretation,
}

impl DiagnosticsRelationKind {
    pub const WRAPPED_ALL: [super::RelationKind; 2] = [
        super::RelationKind::Diagnostics(Self::WireHasInterpretation),
        super::RelationKind::Diagnostics(Self::ShellHasInterpretation),
    ];

    pub const ALL: [Self; 2] = [Self::WireHasInterpretation, Self::ShellHasInterpretation];

    pub const fn kind_id(self) -> KindId {
        match self {
            Self::WireHasInterpretation => KindId(501),
            Self::ShellHasInterpretation => KindId(502),
        }
    }

    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::WireHasInterpretation => ".wire_has_interpretation",
            Self::ShellHasInterpretation => ".shell_has_interpretation",
        }
    }

    pub fn from_kind_id(kind_id: KindId) -> Option<Self> {
        Some(match kind_id {
            KindId(501) => Self::WireHasInterpretation,
            KindId(502) => Self::ShellHasInterpretation,
            _ => return None,
        })
    }
}
