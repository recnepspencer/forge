use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyClass {
    WireOpen,
    WireClosed,
    WireBranch,
    SheetDisk,
    SheetPatch,
    SolidShellGenus0,
    NmtEdgeFan,
}

impl WorthTopologyClass {
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
            Self::WireOpen => "worth.class.wire_open",
            Self::WireClosed => "worth.class.wire_closed",
            Self::WireBranch => "worth.class.wire_branch",
            Self::SheetDisk => "worth.class.sheet_disk",
            Self::SheetPatch => "worth.class.sheet_patch",
            Self::SolidShellGenus0 => "worth.class.solid_shell_genus0",
            Self::NmtEdgeFan => "worth.class.nmt_edge_fan",
        }
    }
}
