use worth_spatial::facade::evidence_lookup_family_catalog::*;

fn main() {
    let _declaration = EvidenceLookupFamilyDeclaration {
        identity: fake(),
        spatial_touch_authority:
            EvidenceLookupSpatialTouchAuthorityRequirement::SealedSpatialTouchAuthorityRequired,
        topology_input_posture: fake(),
        stage_applicability: fake(),
        evidence_classes: fake(),
        lookup_product_posture: EvidenceLookupProductPosture::DeclarationOnlySelectionRequired,
        index_posture: fake(),
        query_posture: fake(),
        diagnostic_witness: fake(),
        source_inventory_pressure: fake(),
        declaration_digest: fake(),
    };
}

fn fake<T>() -> T {
    panic!("compile-fail fixture must fail before fake values are evaluated")
}
