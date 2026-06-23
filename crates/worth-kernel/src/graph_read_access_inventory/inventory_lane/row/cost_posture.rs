#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessCostPosture {
    NoGraphTraversal,
    BoundedTouchedRegion,
    PerResultNeighborLookup,
    AdjacencyMapMaterialization,
    FrontierOrVisitedSet,
    BroadScan,
    LocalCache,
    FabricatedReceiptOrSupportRow,
}
