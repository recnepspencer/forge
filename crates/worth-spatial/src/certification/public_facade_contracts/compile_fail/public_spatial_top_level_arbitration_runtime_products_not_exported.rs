use worth_spatial::facade::{
    declare_spatial_arbitration_runtime, materialize_spatial_arbitration_support_report,
    SpatialArbitrationMaterializationDenial, SpatialArbitrationMaterializationProfilePlan,
    SpatialArbitrationRuntimeDeclaration, SpatialArbitrationSupportMaterialization,
};

fn main() {
    let _ = declare_spatial_arbitration_runtime;
    let _ = materialize_spatial_arbitration_support_report;
    let _: Option<SpatialArbitrationMaterializationDenial> = None;
    let _: Option<SpatialArbitrationMaterializationProfilePlan> = None;
    let _: Option<SpatialArbitrationRuntimeDeclaration> = None;
    let _: Option<SpatialArbitrationSupportMaterialization> = None;
}
