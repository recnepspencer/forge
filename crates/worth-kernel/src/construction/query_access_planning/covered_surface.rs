#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionQueryAccessSurface {
    TopologyBirth,
    PhaseChainTopologyCheck,
    TopologyBirthBroadScan,
}

impl PrimitiveConstructionQueryAccessSurface {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::TopologyBirth => "primitive-construction.topology-birth",
            Self::PhaseChainTopologyCheck => "primitive-construction.phase-chain-topology-check",
            Self::TopologyBirthBroadScan => "primitive-construction.topology-birth-broad-scan",
        }
    }
}
