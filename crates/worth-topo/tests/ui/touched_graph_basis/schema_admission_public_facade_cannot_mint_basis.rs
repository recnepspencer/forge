use schema::facade::platform::authority::{
    WorthGeometryEvidenceSupportPosture, WorthGeometryOnlyEvidenceAdmission,
    WorthGeometryOnlyEvidenceCounters, WorthTouchedGraphBooleanEvidenceStage,
};
use topology::facade::{
    topology_geometry_only_evidence_touched_graph_basis_from_schema_admission,
    TopologyTouchedOperatingWorld,
};

fn main() {
    let copied_admission = WorthGeometryOnlyEvidenceAdmission::from_spatial_boolean_receipt(
        WorthTouchedGraphBooleanEvidenceStage::Split,
        "copied-spatial-receipt-text",
        WorthGeometryEvidenceSupportPosture::Admitted,
        WorthGeometryOnlyEvidenceCounters::from_evidence_rows(1),
    )
    .unwrap();

    let _ = topology_geometry_only_evidence_touched_graph_basis_from_schema_admission(
        &copied_admission,
        TopologyTouchedOperatingWorld::mainline(),
    );
}
