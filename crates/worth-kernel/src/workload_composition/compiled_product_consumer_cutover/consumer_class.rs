#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum KernelCompiledProductConsumerResponsibility {
    TopologyDerived,
    SpatialEvidenceDerived,
    QueryBacked,
    PublicCloseout,
    RetainedReplay,
    OrdinarySweep,
}

impl KernelCompiledProductConsumerResponsibility {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyDerived => "topology-derived",
            Self::SpatialEvidenceDerived => "spatial-evidence-derived",
            Self::QueryBacked => "query-backed",
            Self::PublicCloseout => "public-closeout",
            Self::RetainedReplay => "retained-replay",
            Self::OrdinarySweep => "ordinary-sweep",
        }
    }
}
