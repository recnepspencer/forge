#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum WorthGraphReadAccessHardDeletionSourceRegion {
    PlanAdoptionAuthority,
    TopologyReadConsumers,
    SpatialReadConsumers,
    KernelGraphReadHelpers,
    StandaloneTestInput,
}

impl WorthGraphReadAccessHardDeletionSourceRegion {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::PlanAdoptionAuthority => "plan_adoption_authority",
            Self::TopologyReadConsumers => "topology_read_consumers",
            Self::SpatialReadConsumers => "spatial_read_consumers",
            Self::KernelGraphReadHelpers => "kernel_graph_read_helpers",
            Self::StandaloneTestInput => "standalone_test_input",
        }
    }
}
