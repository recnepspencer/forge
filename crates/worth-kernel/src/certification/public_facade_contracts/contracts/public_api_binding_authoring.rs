use forge_query::facade::ForgeQueryOrdinaryOutcome;
use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryDeclarationEntryInspectionInput};
use worth_kernel::facade::authoring::binding::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
    PrimitiveBindingAuthoringError, PrimitiveBindingQueryDomain, PrimitiveBindingQueryWorld,
};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    AdmittedPartialBindingPosture, EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite,
    FaceSurfaceBindingSpec, SpatialBindingAuthorityError, SpatialBindingCompleteness,
    SpatialBindingIllegalityReason, SpatialBindingIncompleteness, SpatialBindingKind,
    SpatialBindingUnsupportedReason,
};

#[test]
fn kernel_public_facade_exports_declaration_entry_first_binding_authoring_surface() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let geometry = PrimitiveGeometryIdentityBundle::new(
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
    let entry = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
            contract,
            geometry,
        )),
    );
    let admitted = entry.clone().admit().expect("admitted binding");
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveBindingQueryDomain)
        .with_operating_context(PrimitiveBindingQueryWorld::new("public-api"))
        .validate()
        .expect("validated binding query handle")
        .admit()
        .expect("admitted binding query handle");
    let progressed = entry
        .progress_with_query(&handle)
        .unwrap_or_else(|_| panic!("progressed declaration entry"));
    let inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            handle.orchestrate_envelope_from_progressed_checked(progressed.clone()),
        ))
        .unwrap_or_else(|_| panic!("inspected declaration entry"));
    let ordinary = entry.ordinary_outcome_with_query(&handle);

    assert_eq!(entry.binding_kind(), SpatialBindingKind::FaceSurface);
    assert_eq!(admitted.kind(), SpatialBindingKind::FaceSurface);
    assert!(admitted.completeness().is_complete());
    assert_eq!(
        progressed.declaration_family_key(),
        "PrimitiveBindingDeclarationFamily"
    );
    assert_eq!(
        Some(progressed.progression_digest()),
        inspection.progression_digest()
    );
    match ordinary {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            assert_eq!(
                envelope.progression_digest(),
                Some(progressed.progression_digest())
            );
        }
        _ => panic!("expected bound ordinary outcome"),
    }
}

#[test]
fn kernel_public_facade_exports_phase_four_completeness_and_denial_taxonomy() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::ShellWithHole {
            outer_loop_edge_count: 4,
            hole_loop_edge_counts: vec![3],
        },
    );
    let unsupported_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::WireBody { edge_count: 4 },
    );

    let partial = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1"),
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
            EdgeBindingSite::new("edge-1"),
            contract,
            PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
        )),
    );
    let unsupported = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-unsupported"),
            unsupported_contract,
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
    let illegal = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(""),
            contract,
            PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
        )),
    );

    let partial_admitted = partial.admit().expect("partial admitted");
    let denied_admitted = denied.admit().expect("denied admitted");

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
        unsupported.admit(),
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
        illegal.admit(),
        Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ),
        ))
    );
}
