use worth_ui_runtime::facade::{
    UiMeasurementBasis, UiMeasurementBasisGeneration, UiMeasurementBasisPosture,
    UiMeasurementGenerationCompatibility, UiMeasurementNeighborhoodClassHint,
};

fn bogus<T>() -> T {
    unreachable!()
}

fn main() {
    let _ = UiMeasurementBasis {
        identity_digest: 0,
        generation: UiMeasurementBasisGeneration::new(0),
        declaration_identity: bogus(),
        graph_node_identity: bogus(),
        world_profile: bogus(),
        declaration_support_authority_generation: bogus(),
        declared_measurement_policy: bogus(),
        basis_posture: UiMeasurementBasisPosture::HostOnly,
        evidence_inputs: Box::new([]),
        generation_compatibility: UiMeasurementGenerationCompatibility::Compatible,
        dependency_lineage: bogus(),
        dependency_map: bogus(),
        neighborhood_class_hint: UiMeasurementNeighborhoodClassHint::ViewportDependency,
        denial_posture: None,
    };
}
