use forge_query::facade::ForgeQueryOrdinaryOutcome;
use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryDeclarationEntryInspectionInput};
use worth_kernel::facade::authoring::binding::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
    PrimitiveRebindingQueryDomain, PrimitiveRebindingQueryWorld,
};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    attach_surface_to_face, attach_vertex_geometry, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidate, ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};

fn plane_geometry(vertices: [[f64; 3]; 2]) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vertices
            .into_iter()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

#[test]
fn kernel_public_facade_exports_declaration_entry_first_rebinding_surface() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let successor = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-new"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("successor");
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurface(prior),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "successor",
                    SpatialAdmittedPrimitiveBinding::FaceSurface(successor),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let admitted = entry.clone().admit().expect("rebinding decision");
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveRebindingQueryDomain)
        .with_operating_context(PrimitiveRebindingQueryWorld::new("public-api-rebinding"))
        .validate()
        .expect("validated rebinding query handle")
        .admit()
        .expect("admitted rebinding query handle");
    let progressed = entry
        .progress_with_query(&handle)
        .unwrap_or_else(|_| panic!("progressed rebinding declaration entry"));
    let inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            handle.orchestrate_envelope_from_progressed_checked(progressed.clone()),
        ))
        .unwrap_or_else(|_| panic!("inspected rebinding declaration entry"));
    let ordinary = entry.ordinary_outcome_with_query(&handle);

    assert_eq!(
        admitted.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        progressed.declaration_family_key(),
        "PrimitiveRebindingDeclarationFamily"
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
fn kernel_public_facade_exports_vertex_rebinding_intent_surface() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_vertex_geometry(VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    ))
    .expect("prior");
    let successor = attach_vertex_geometry(VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-new"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    ))
    .expect("successor");
    let admitted = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_geometry_binding(
            SpatialAdmittedPrimitiveBinding::VertexGeometry(prior),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "successor",
                    SpatialAdmittedPrimitiveBinding::VertexGeometry(successor),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    )
    .admit()
    .expect("admitted");

    assert_eq!(
        admitted.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
}

#[test]
fn kernel_public_facade_exports_typed_unsupported_rebinding_outcome_transport() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_vertex_geometry(VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    ))
    .expect("prior");
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::VertexGeometry(prior.clone()),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "successor",
                    SpatialAdmittedPrimitiveBinding::VertexGeometry(prior),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveRebindingQueryDomain)
        .with_operating_context(PrimitiveRebindingQueryWorld::new(
            "public-api-rebinding-unsupported",
        ))
        .validate()
        .expect("validated rebinding query handle")
        .admit()
        .expect("admitted rebinding query handle");

    assert_eq!(
        entry.clone().admit().expect("decision").outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert!(matches!(
        entry.ordinary_outcome_with_query(&handle),
        ForgeQueryOrdinaryOutcome::Unsupported(_)
    ));
}

#[test]
fn kernel_public_facade_exports_typed_rebinding_explanation_surface() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let successor = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-new"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    ))
    .expect("successor");
    let admitted = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurface(prior),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "successor",
                    SpatialAdmittedPrimitiveBinding::FaceSurface(successor),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    )
    .admit()
    .expect("admitted");

    assert_eq!(admitted.explanation().candidate_labels(), ["successor"]);
    assert!(admitted
        .explanation()
        .selected_candidate_identity()
        .is_some());
}
