use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthDiagnosticsRelationKind {
    WireHasInterpretation,
    ShellHasInterpretation,
}

impl WorthDiagnosticsRelationKind {
    pub const WRAPPED_ALL: [super::WorthRelationKind; 2] = [
        super::WorthRelationKind::Diagnostics(Self::WireHasInterpretation),
        super::WorthRelationKind::Diagnostics(Self::ShellHasInterpretation),
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
            Self::WireHasInterpretation => "worth.wire_has_interpretation",
            Self::ShellHasInterpretation => "worth.shell_has_interpretation",
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
