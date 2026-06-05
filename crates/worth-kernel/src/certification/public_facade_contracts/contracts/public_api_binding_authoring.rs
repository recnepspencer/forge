use forge_query::facade::ForgeQueryOrdinaryOutcome;
use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryDeclarationEntryInspectionInput};
use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_kernel::facade::authoring::binding::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
    PrimitiveBindingQueryDomain, PrimitiveBindingQueryWorld,
};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    AnchorCarrierOwnership, CarrierOwnedParameterPointAnchorSpec, FaceBindingSite,
    FaceSurfaceBindingSpec, SpatialBindingKind,
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
fn kernel_public_facade_exports_declaration_entry_first_anchor_binding_authoring_surface() {
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
        AuthorPrimitiveBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
                contract,
                geometry,
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::plane())
                    .expect("carrier ownership"),
                ParameterSpacePoint::try_new([0.25, 0.5]).expect("anchor point"),
            )
            .expect("anchor spec"),
        ),
    );
    let admitted = entry.clone().admit().expect("admitted anchored binding");
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveBindingQueryDomain)
        .with_operating_context(PrimitiveBindingQueryWorld::new("public-api-anchor"))
        .validate()
        .expect("validated binding query handle")
        .admit()
        .expect("admitted binding query handle");
    let progressed = entry
        .progress_with_query(&handle)
        .unwrap_or_else(|_| panic!("progressed anchor declaration entry"));
    let inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            handle.orchestrate_envelope_from_progressed_checked(progressed.clone()),
        ))
        .unwrap_or_else(|_| panic!("inspected anchor declaration entry"));
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
