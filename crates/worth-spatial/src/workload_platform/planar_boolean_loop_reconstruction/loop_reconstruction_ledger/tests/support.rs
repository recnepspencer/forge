use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedPhaseFourteenSubject;
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopClassifiedProductKind, PlanarBooleanLoopIdentityMap,
    PlanarBooleanLoopIdentityRow, PlanarBooleanLoopPersistentNamePropagationMap,
    PlanarBooleanLoopSubshapeSignatureMap,
};

pub(super) fn admitted_identity_products(
    fixture: &PreparedPhaseFourteenSubject,
) -> (
    PlanarBooleanLoopIdentityMap,
    PlanarBooleanLoopPersistentNamePropagationMap,
    PlanarBooleanLoopSubshapeSignatureMap,
) {
    let reconstructed = fixture
        .reconstructed_boundary
        .reconstructed_loops()
        .rows()
        .first()
        .expect("phase fourteen fixture should reconstruct at least one loop");
    let role_outcome = fixture
        .role_boundary
        .role_outcomes()
        .rows()
        .iter()
        .find(|row| row.loop_identity() == reconstructed.reconstructed_loop_identity())
        .expect("fixture should expose matching role evidence");
    let degenerate_outcome = fixture
        .degenerate_boundary
        .outcomes()
        .rows()
        .iter()
        .find(|row| row.loop_identity() == reconstructed.reconstructed_loop_identity())
        .expect("fixture should expose matching degeneracy posture");
    let identity_map = PlanarBooleanLoopIdentityMap::new(
        format!(
            "admitted-identity-map:{}",
            fixture.request.request_identity()
        ),
        fixture.request.request_identity().to_string(),
        vec![PlanarBooleanLoopIdentityRow::new(
            format!(
                "admitted-identity-row:{}",
                reconstructed.reconstructed_loop_identity()
            ),
            reconstructed.reconstructed_loop_identity().to_string(),
            format!("canonical:{}", reconstructed.reconstructed_loop_identity()),
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
            vec![reconstructed.source_loop_identity().to_string()],
            reconstructed.fragment_identities().to_vec(),
            reconstructed.split_vertex_identities().to_vec(),
            role_outcome.role_outcome_identity().to_string(),
            degenerate_outcome
                .degenerate_loop_outcome_identity()
                .to_string(),
        )],
    );
    let name_map = PlanarBooleanLoopPersistentNamePropagationMap::new(
        format!("admitted-name-map:{}", fixture.request.request_identity()),
        fixture.request.request_identity().to_string(),
        Vec::new(),
    );
    let signature_map = PlanarBooleanLoopSubshapeSignatureMap::new(
        format!(
            "admitted-signature-map:{}",
            fixture.request.request_identity()
        ),
        fixture.request.request_identity().to_string(),
        Vec::new(),
    );
    (identity_map, name_map, signature_map)
}
