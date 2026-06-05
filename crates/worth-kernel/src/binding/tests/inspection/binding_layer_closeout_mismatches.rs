use forge_query::facade::ForgeQueryDeclarationEntryInspectionInput;
use worth_spatial::facade::bindings::{
    NeighborhoodBindingFamily, ReplacementCandidate, SpatialAdmittedPrimitiveBinding,
};

use crate::{
    binding::rebinding::{
        primitive_rebinding_certification_bundle, BindingLayerCertificationBundleError,
        PrimitiveRebindingBranchLocalInspection, PrimitiveRebindingDeclarationEntry,
        PrimitiveRebindingHistoricalInspection, PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    },
    facade::authoring::binding::{
        author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
    },
};

use super::super::support::{
    admitted_rebinding_handle, anchored_surface, progress_rebinding_entry,
    replacement_neighborhood, scoped_branch_head_inspection_basis,
};

#[test]
fn binding_layer_certification_bundle_rejects_retained_proofs_from_the_wrong_declaration() {
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new(
                        "weaker",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker.clone()),
                    )
                    .expect("weaker"),
                    ReplacementCandidate::new(
                        "exact",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact.clone()),
                    )
                    .expect("exact"),
                ],
            ),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new(
                        "exact",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact),
                    )
                    .expect("exact"),
                    ReplacementCandidate::new(
                        "weaker",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker),
                    )
                    .expect("weaker"),
                ],
            ),
        ),
    );
    let foreign_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(anchored_surface(
                "face-foreign",
                "surface-foreign",
                [0.25, 0.5],
                1.0,
            )),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-foreign",
                vec![ReplacementCandidate::new(
                    "foreign",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(anchored_surface(
                        "face-foreign-new",
                        "surface-foreign-new",
                        [0.25, 0.5],
                        1.0,
                    )),
                )
                .expect("foreign")],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-sixteen-closeout-mismatch");
    let branch_basis =
        scoped_branch_head_inspection_basis("branch:phase-sixteen-closeout-mismatch");

    let historical_mismatch = primitive_rebinding_certification_bundle(
        &left_entry,
        &right_entry,
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
        &left_entry,
        &right_entry,
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
    entry
        .historical_inspection_with_query(
            handle,
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    entry, handle,
                )),
            ),
        )
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
    entry
        .branch_local_inspection_with_query(
            handle,
            branch_basis,
            forge_query::facade::LowerRuntimeBasisEvidence::from_relational_facade(
                branch_basis
                    .expected_lower_runtime_binding_digest()
                    .expect("basis digest"),
                evidence,
                1,
            ),
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    entry, handle,
                )),
            ),
        )
        .expect("branch-local inspection")
}
