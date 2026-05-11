use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyClass {
    WireOpen,
    WireClosed,
    WireBranch,
    SheetDisk,
    SheetPatch,
    SolidShellGenus0,
    NmtEdgeFan,
}

impl TopologyClass {
    pub const ALL: [Self; 7] = [
        Self::WireOpen,
        Self::WireClosed,
        Self::WireBranch,
        Self::SheetDisk,
        Self::SheetPatch,
        Self::SolidShellGenus0,
        Self::NmtEdgeFan,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WireOpen => ".class.wire_open",
            Self::WireClosed => ".class.wire_closed",
            Self::WireBranch => ".class.wire_branch",
            Self::SheetDisk => ".class.sheet_disk",
            Self::SheetPatch => ".class.sheet_patch",
            Self::SolidShellGenus0 => ".class.solid_shell_genus0",
            Self::NmtEdgeFan => ".class.nmt_edge_fan",
        }
    }
}
