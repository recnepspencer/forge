use super::error::WorkloadCatalogError;
use topology::facade::{
    NmtTopologyConstruction, NmtTopologyConstructionReceipt, OpenLayerStackSpec, OpenRadialFanSpec,
    OpenSheetPatchSpec, OpenWireChainSpec,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum WorkloadCatalogTopologyConstructionPlan {
    Receipt(NmtTopologyConstructionReceipt),
    OpenWire(OpenWireChainSpec),
    OpenSheet(OpenSheetPatchSpec),
    OpenRadialFan(OpenRadialFanSpec),
    OpenLayerStack(OpenLayerStackSpec),
}

impl WorkloadCatalogTopologyConstructionPlan {
    pub(super) fn support_denial(&self) -> Option<String> {
        match self {
            Self::Receipt(_) => None,
            Self::OpenWire(spec) if !(2..=128).contains(&spec.edge_count()) => Some(format!(
                "open wire chain topology requires edge count 2 through 128 before workload binding; requested {}",
                spec.edge_count()
            )),
            Self::OpenSheet(spec) if !(1..=64).contains(&spec.strip_count()) => Some(format!(
                "open sheet patch topology requires strip count 1 through 64 before workload binding; requested {}",
                spec.strip_count()
            )),
            Self::OpenRadialFan(spec) if !(3..=128).contains(&spec.incident_face_count()) => {
                Some(format!(
                    "open radial fan topology requires incident face count 3 through 128 before workload binding; requested {}",
                    spec.incident_face_count()
                ))
            }
            Self::OpenLayerStack(spec) if !(2..=16).contains(&spec.layer_count()) => Some(format!(
                "open layer stack topology requires layer count 2 through 16 before workload binding; requested {}",
                spec.layer_count()
            )),
            Self::OpenLayerStack(spec) if !spec.requests_layer_identity_receipts() => Some(
                "open layer stack topology requires layer identity receipts so downstream workload stages can bind projection and replay evidence to each layer"
                    .to_string(),
            ),
            Self::OpenLayerStack(spec) if !spec.requests_open_boundary_receipts() => Some(
                "open layer stack topology requires open-boundary receipts before downstream NMT workloads can reason about no-options outcomes"
                    .to_string(),
            ),
            Self::OpenLayerStack(spec) if !spec.requests_radial_adjacency_receipts() => Some(
                "open layer stack topology requires radial-adjacency receipts so non-manifold evidence cannot be laundered as ordinary sheet topology"
                    .to_string(),
            ),
            _ => None,
        }
    }

    pub(super) fn compile(
        &self,
        declaration: &str,
    ) -> Result<NmtTopologyConstructionReceipt, WorkloadCatalogError> {
        match self {
            Self::Receipt(receipt) => Ok(receipt.clone()),
            Self::OpenWire(spec) => NmtTopologyConstruction::open_wire_chain(*spec)
                .declared(format!("topology construction for {declaration}"))
                .construct()
                .map_err(WorkloadCatalogError::from),
            Self::OpenSheet(spec) => NmtTopologyConstruction::open_sheet_patch(*spec)
                .declared(format!("topology construction for {declaration}"))
                .construct()
                .map_err(WorkloadCatalogError::from),
            Self::OpenRadialFan(spec) => NmtTopologyConstruction::open_radial_fan(*spec)
                .declared(format!("topology construction for {declaration}"))
                .construct()
                .map_err(WorkloadCatalogError::from),
            Self::OpenLayerStack(spec) => NmtTopologyConstruction::open_layer_stack(spec.clone())
                .declared(format!("topology construction for {declaration}"))
                .construct()
                .map_err(WorkloadCatalogError::from),
        }
    }
}
