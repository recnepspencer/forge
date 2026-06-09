use super::super::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_prior_fact_from_declaration, canonical_geometry, orthotope_contract,
    primitive_rebinding_certification_bundle, progress_rebinding_entry, replacement_neighborhood,
    scoped_branch_head_inspection_basis, BindingLayerCertificationBundleError,
};
use worth_spatial::facade::bindings::NeighborhoodBindingFamily;
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_rebinding_declaration,
    primitive_rebinding_retained_fact_source, AuthorPrimitiveAnchorBindingIntent,
    PrimitiveRebindingDeclarationEntry, PrimitiveRebindingQueryDomain,
    PrimitiveRebindingQueryWorld, PrimitiveRebindingRetainedFactSource,
};
use worth_spatial::facade::bindings::{
    AnchorCarrierOwnership, CarrierOwnedParameterPointAnchorSpec, FaceBindingSite,
    FaceSurfaceBindingSpec,
};
use worth_spatial::facade::inspection::{
    branch_local_geometry_inspection_entry, historical_geometry_inspection_entry,
    primitive_rebinding_retained_subject, PrimitiveRebindingBranchLocalInspection,
    PrimitiveRebindingHistoricalInspection,
};

#[test]
fn binding_layer_certification_bundle_rejects_retained_proofs_from_the_wrong_declaration() {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "closeout-mismatch-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "closeout-mismatch-left-weaker",
                    )
                    .expect("weaker"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "closeout-mismatch-left-exact",
                    )
                    .expect("exact"),
                ],
            ),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "closeout-mismatch-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "closeout-mismatch-right-exact",
                    )
                    .expect("exact"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "closeout-mismatch-right-weaker",
                    )
                    .expect("weaker"),
                ],
            ),
        ),
    );
    let foreign_prior =
        anchored_surface_declaration("face-foreign", "surface-foreign", [0.25, 0.5], 1.0);
    let foreign_candidate =
        anchored_surface_declaration("face-foreign-new", "surface-foreign-new", [0.25, 0.5], 1.0);
    let foreign_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(
                &foreign_prior,
                "closeout-mismatch-foreign-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-foreign",
                vec![anchored_surface_candidate_from_declaration(
                    "foreign",
                    &foreign_candidate,
                    "closeout-mismatch-foreign-candidate",
                )
                .expect("foreign")],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-sixteen-closeout-mismatch");
    let branch_basis =
        scoped_branch_head_inspection_basis("branch:phase-sixteen-closeout-mismatch");

    let historical_mismatch = primitive_rebinding_certification_bundle(
        retained_fact_source(&left_entry, &handle),
        retained_fact_source(&right_entry, &handle),
        historical_inspection(&foreign_entry, &handle),
        historical_inspection(&right_entry, &handle),
        branch_local_inspection(&left_entry, &handle, &branch_basis, "left"),
        branch_local_inspection(&right_entry, &handle, &branch_basis, "right"),
        &handle,
    );
    assert!(matches!(
        historical_mismatch,
        Err(BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch { .. })
    ));

    let branch_local_mismatch = primitive_rebinding_certification_bundle(
        retained_fact_source(&left_entry, &handle),
        retained_fact_source(&right_entry, &handle),
        historical_inspection(&left_entry, &handle),
        historical_inspection(&right_entry, &handle),
        branch_local_inspection(&foreign_entry, &handle, &branch_basis, "foreign"),
        branch_local_inspection(&right_entry, &handle, &branch_basis, "right"),
        &handle,
    );
    assert!(matches!(
        branch_local_mismatch,
        Err(BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch { .. })
    ));
}

fn historical_inspection(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
) -> PrimitiveRebindingHistoricalInspection {
    let subject = handle
        .orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(entry, handle));
    historical_geometry_inspection_entry(
        retained_fact_source(entry, handle),
        primitive_rebinding_retained_subject(entry.binding_kind(), &subject),
    )
    .inspect_checked(handle, subject)
    .expect("historical inspection")
}

fn branch_local_inspection(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
    branch_basis: &forge_query::facade::ScopedInspectionBasis,
    evidence: &str,
) -> PrimitiveRebindingBranchLocalInspection {
    let subject = handle
        .orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(entry, handle));
    branch_local_geometry_inspection_entry(
        retained_fact_source(entry, handle),
        branch_basis.clone(),
        forge_query::facade::LowerRuntimeBasisEvidence::from_relational_facade(
            branch_basis
                .expected_lower_runtime_binding_digest()
                .expect("basis digest"),
            evidence,
            1,
        ),
        primitive_rebinding_retained_subject(entry.binding_kind(), &subject),
    )
    .inspect_checked(handle, subject)
    .expect("branch-local inspection")
}

fn retained_fact_source(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
) -> PrimitiveRebindingRetainedFactSource {
    primitive_rebinding_retained_fact_source(entry, handle).expect("retained fact source")
}

fn anchored_surface_declaration(
    face_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
    extent: f64,
) -> worth_spatial::facade::bindings::PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_identity).with_persistent_name(persistent_name),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [extent, 0.0, 0.0]]),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(
                    face_identity,
                    worth_geom::facade::ParameterDomain::plane(),
                )
                .expect("ownership"),
                worth_geom::facade::ParameterSpacePoint::try_new(parameter).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
}
