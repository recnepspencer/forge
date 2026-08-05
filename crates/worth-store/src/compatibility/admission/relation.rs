use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CompatibilityRelation {
    Native,
    BackwardRead,
    ForwardRead,
    AdapterRequired,
    DerivedRebuildRequired,
    Incompatible,
}
impl CompatibilityRelation {
    pub fn from_declared_edge(edge: Option<&DeclaredCompatibilityEdge>) -> Self {
        edge.map_or(Self::Incompatible, |edge| edge.relation())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CompatibilityAdapterCostClass {
    ZeroCopy,
    BoundedRecordLocal,
    BoundedBatchLocal,
    MaintenanceOnly,
    OutOfScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum CompatibilityAdmissionPath {
    HotRead,
    BatchRead,
    MaintenanceScheduled,
}
