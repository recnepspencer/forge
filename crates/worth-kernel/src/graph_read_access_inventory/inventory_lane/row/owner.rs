#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthGraphReadAccessOwner {
    ForgeQuery,
    WorthKernel,
    WorthTopo,
    WorthSpatial,
}

impl WorthGraphReadAccessOwner {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForgeQuery => "forge-query",
            Self::WorthKernel => "worth-kernel",
            Self::WorthTopo => "worth-topo",
            Self::WorthSpatial => "worth-spatial",
        }
    }
}
