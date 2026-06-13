use topology::facade::{NmtTopologyConstructionReceipt, NmtTopologyPattern, NmtTopologyScopeKind};

use super::denial::{OpenClassTriadParityDenial, OpenClassTriadParityDenialKind};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OpenTopologyClass {
    Wire,
    Sheet,
    NmtFan,
}

impl OpenTopologyClass {
    pub const REQUIRED: [Self; 3] = [Self::Wire, Self::Sheet, Self::NmtFan];

    pub fn human_name(self) -> &'static str {
        match self {
            Self::Wire => "open wire",
            Self::Sheet => "open sheet",
            Self::NmtFan => "open NMT radial fan",
        }
    }

    pub(crate) fn from_topology(
        topology: &NmtTopologyConstructionReceipt,
    ) -> Result<Self, OpenClassTriadParityDenial> {
        match topology.pattern() {
            NmtTopologyPattern::OpenWireChain(_) => Ok(Self::Wire),
            NmtTopologyPattern::OpenSheetPatch(_) => Ok(Self::Sheet),
            NmtTopologyPattern::OpenRadialFan(_) => Ok(Self::NmtFan),
            NmtTopologyPattern::OpenLayerStack(_) => Err(OpenClassTriadParityDenial::new(
                OpenClassTriadParityDenialKind::UnsupportedOpenClass,
                None,
                "Open-class triad parity supports open wire, open sheet, and open NMT radial fan evidence; open layer stacks need their own parity phase.",
            )),
        }
    }

    pub(crate) fn from_scope_kind(
        kind: NmtTopologyScopeKind,
    ) -> Result<Self, OpenClassTriadParityDenial> {
        match kind {
            NmtTopologyScopeKind::OpenWire => Ok(Self::Wire),
            NmtTopologyScopeKind::OpenSheet => Ok(Self::Sheet),
            NmtTopologyScopeKind::OpenRadialFan => Ok(Self::NmtFan),
            NmtTopologyScopeKind::OpenLayer => Err(OpenClassTriadParityDenial::new(
                OpenClassTriadParityDenialKind::UnsupportedOpenClass,
                None,
                "Open-class triad parity supports wire, sheet, and radial fan scopes; layer stacks belong to the basket stack boss.",
            )),
        }
    }
}
