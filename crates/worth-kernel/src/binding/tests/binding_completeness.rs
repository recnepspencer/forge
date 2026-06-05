use forge_query::facade::ForgeQueryOrdinaryOutcome;
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    AdmittedPartialBindingPosture, EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite,
    FaceSurfaceBindingSpec, SpatialBindingAuthorityError, SpatialBindingCompleteness,
    SpatialBindingIllegalityReason, SpatialBindingIncompleteness, SpatialBindingKind,
    SpatialBindingUnsupportedReason, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::facade::authoring::binding::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
    PrimitiveBindingAuthoringError,
};

use super::support::{
    admitted_binding_handle, declaration_digest_string, inspect_progressed_binding_entry,
    progress_binding_entry, shell_with_hole_contract,
};

#[test]
fn binding_completeness_policy_distinguishes_complete_partial_unsupported_and_illegal() {
    let contract = shell_with_hole_contract();
    let unsupported_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::WireBody { edge_count: 4 },
    );
    let complete_geometry = PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vec![
            PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0]),
            PrimitiveVertexIdentity::from_position([1.0, 0.0, 0.0]),
        ],
    );
    let partial_geometry = PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vec![],
    );
    let denied_geometry = PrimitiveGeometryIdentityBundle::new(vec![], vec![]);

    let complete = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-1").with_persistent_name("vertex-alpha"),
            contract,
            complete_geometry.clone(),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let partial = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
            contract,
            partial_geometry.clone(),
        )),
    );
    let denied = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-1").with_persistent_name("curve-alpha"),
            contract,
            denied_geometry,
        )),
    );
    let unsupported = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-unsupported"),
            unsupported_contract,
            partial_geometry,
        )),
    );
    let illegal =
        author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
            FaceSurfaceBindingSpec::new(FaceBindingSite::new(""), contract, complete_geometry),
        ));
    let handle = admitted_binding_handle("completeness-policy");

    let complete_admitted = complete.clone().admit().expect("complete admitted");
    let partial_admitted = partial.clone().admit().expect("partial admitted");
    let denied_admitted = denied.clone().admit().expect("denied admitted");
    let complete_progressed = progress_binding_entry(&complete, &handle);
    let partial_progressed = progress_binding_entry(&partial, &handle);
    let denied_progressed = progress_binding_entry(&denied, &handle);
    let complete_inspection =
        inspect_progressed_binding_entry(&handle, complete_progressed.clone());
    let partial_inspection = inspect_progressed_binding_entry(&handle, partial_progressed.clone());
    let denied_inspection = inspect_progressed_binding_entry(&handle, denied_progressed.clone());

    assert_eq!(
        complete_admitted.completeness(),
        &SpatialBindingCompleteness::Complete
    );
    assert_eq!(
        partial_admitted.completeness(),
        &SpatialBindingCompleteness::AdmittedPartial(
            AdmittedPartialBindingPosture::FaceSurfaceMissingVertexGeometry,
        )
    );
    assert_eq!(
        denied_admitted.completeness(),
        &SpatialBindingCompleteness::DeniedIncomplete(
            SpatialBindingIncompleteness::EdgeCurveMissingCurveWitnessVertices,
        )
    );
    assert_eq!(
        unsupported.clone().admit(),
        Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Unsupported(
                SpatialBindingUnsupportedReason::TopologyBirthClassDoesNotAdmitBindingKind {
                    binding_kind: SpatialBindingKind::FaceSurface,
                    topology_birth_class: "planar_wire_body",
                },
            ),
        ))
    );
    assert_eq!(
        illegal.clone().admit(),
        Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ),
        ))
    );
    assert_eq!(
        declaration_digest_string(&complete_progressed),
        complete_inspection.declaration_digest()
    );
    assert_eq!(
        declaration_digest_string(&partial_progressed),
        partial_inspection.declaration_digest()
    );
    assert_eq!(
        declaration_digest_string(&denied_progressed),
        denied_inspection.declaration_digest()
    );
    match complete.ordinary_outcome_with_query(&handle) {
        ForgeQueryOrdinaryOutcome::Bound(_) => {}
        _ => panic!("expected bound ordinary outcome for complete binding"),
    }
    match partial.ordinary_outcome_with_query(&handle) {
        ForgeQueryOrdinaryOutcome::Bound(_) => {}
        _ => panic!("expected bound ordinary outcome for partial binding"),
    }
    match denied.ordinary_outcome_with_query(&handle) {
        ForgeQueryOrdinaryOutcome::Bound(_) => {}
        _ => panic!("expected bound ordinary outcome for denied-incomplete binding"),
    }
}

#[test]
fn binding_completeness_replay_does_not_upgrade_missing_evidence_into_success() {
    let contract = shell_with_hole_contract();
    let partial = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
            contract,
            PrimitiveGeometryIdentityBundle::new(
                vec![PrimitiveSupportPlaneIdentity::new(
                    "0".to_string(),
                    "0".to_string(),
                    "1".to_string(),
                    "0".to_string(),
                )],
                vec![],
            ),
        )),
    );
    let equivalent_partial = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
            contract,
            PrimitiveGeometryIdentityBundle::new(
                vec![PrimitiveSupportPlaneIdentity::new(
                    "0".to_string(),
                    "0".to_string(),
                    "1".to_string(),
                    "0".to_string(),
                )],
                vec![],
            ),
        )),
    );
    let denied = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-1").with_persistent_name("curve-alpha"),
            contract,
            PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
        )),
    );
    let equivalent_denied = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-1").with_persistent_name("curve-alpha"),
            contract,
            PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
        )),
    );
    let handle = admitted_binding_handle("completeness-replay");

    let partial_admitted = partial.clone().admit().expect("partial admitted");
    let equivalent_partial_admitted = equivalent_partial
        .clone()
        .admit()
        .expect("equivalent partial admitted");
    let denied_admitted = denied.clone().admit().expect("denied admitted");
    let equivalent_denied_admitted = equivalent_denied
        .clone()
        .admit()
        .expect("equivalent denied admitted");
    let first_partial_progressed = progress_binding_entry(&partial, &handle);
    let first_denied_progressed = progress_binding_entry(&denied, &handle);
    let second_denied_progressed = progress_binding_entry(&equivalent_denied, &handle);
    let second_partial_progressed = progress_binding_entry(&equivalent_partial, &handle);
    let first_partial_inspection =
        inspect_progressed_binding_entry(&handle, first_partial_progressed.clone());
    let first_denied_inspection =
        inspect_progressed_binding_entry(&handle, first_denied_progressed.clone());
    let second_denied_inspection =
        inspect_progressed_binding_entry(&handle, second_denied_progressed.clone());
    let second_partial_inspection =
        inspect_progressed_binding_entry(&handle, second_partial_progressed.clone());

    assert_eq!(
        partial_admitted.completeness(),
        &SpatialBindingCompleteness::AdmittedPartial(
            AdmittedPartialBindingPosture::FaceSurfaceMissingVertexGeometry,
        )
    );
    assert_eq!(
        equivalent_partial_admitted.completeness(),
        partial_admitted.completeness()
    );
    assert_eq!(
        denied_admitted.completeness(),
        &SpatialBindingCompleteness::DeniedIncomplete(
            SpatialBindingIncompleteness::EdgeCurveMissingCurveWitnessVertices,
        )
    );
    assert_eq!(
        equivalent_denied_admitted.completeness(),
        denied_admitted.completeness()
    );
    assert_eq!(
        declaration_digest_string(&first_partial_progressed),
        declaration_digest_string(&second_partial_progressed)
    );
    assert_eq!(
        declaration_digest_string(&first_denied_progressed),
        declaration_digest_string(&second_denied_progressed)
    );
    assert_eq!(
        first_partial_progressed.progression_digest(),
        second_partial_progressed.progression_digest()
    );
    assert_eq!(
        first_denied_progressed.progression_digest(),
        second_denied_progressed.progression_digest()
    );
    assert_eq!(
        first_partial_inspection.inspection_digest(),
        second_partial_inspection.inspection_digest()
    );
    assert_eq!(
        first_denied_inspection.inspection_digest(),
        second_denied_inspection.inspection_digest()
    );
    assert!(!matches!(
        partial_admitted.completeness(),
        SpatialBindingCompleteness::Complete
    ));
    assert!(!matches!(
        denied_admitted.completeness(),
        SpatialBindingCompleteness::Complete
    ));
}
