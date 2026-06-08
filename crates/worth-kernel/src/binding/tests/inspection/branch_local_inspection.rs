use forge_query::facade::{
    admit_basis_capability, evaluate_basis_inspection_eligibility, normalize_raw_basis_intent,
    scope_basis_for_inspection, DeniedBasisCapabilityKind,
    ForgeQueryDeclarationEntryInspectionInput, LowerRuntimeBasisEvidence, RawBasisIntent,
    ScopedInspectionBasis,
};
use worth_spatial::facade::bindings::{
    BindingContinuityClass, NeighborhoodBindingFamily, ReplacementCandidate,
    SpatialAdmittedPrimitiveBinding,
};

use crate::{
    binding::rebinding::PrimitiveRebindingBranchLocalInspectionError,
    facade::authoring::binding::{
        author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
    },
};

use super::super::support::{
    admitted_rebinding_handle, anchored_surface, progress_rebinding_entry,
    replacement_neighborhood, retained_digest_for_decision,
};

#[test]
fn branch_local_binding_inspection_distinguishes_branch_state_from_authoritative_state() {
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let correspondence = anchored_surface("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let authoritative = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![ReplacementCandidate::new(
                    "exact",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
                )
                .expect("exact candidate")],
            ),
        ),
    )
    .admit()
    .expect("authoritative decision");
    let branch_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![ReplacementCandidate::new(
                    "correspondence",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(correspondence),
                )
                .expect("correspondence candidate")],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-fourteen-branch-divergence");
    let branch_basis = scoped_branch_head_inspection_basis("branch:diverged");
    let branch_progression = progress_rebinding_entry(&branch_entry, &handle);
    let branch_subject = ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
        handle.orchestrate_envelope_from_progressed_checked(branch_progression),
    );
    let branch_local = branch_entry
        .branch_local_inspection_with_query(
            &handle,
            &branch_basis,
            branch_basis_evidence(&branch_basis, "branch-evidence:diverged"),
            branch_subject,
        )
        .expect("branch-local inspection");

    assert_eq!(
        branch_local.branch_basis_digest(),
        branch_basis.scoped_basis_digest()
    );
    assert!(!branch_local.branch_binding_digest().is_empty());
    assert_ne!(
        branch_local.branch_binding_digest(),
        branch_basis
            .expected_lower_runtime_binding_digest()
            .expect("branch binding digest")
    );
    assert!(branch_local.inspection().progression_digest().is_some());
    assert_ne!(
        branch_local.decision().explanation().continuity_class(),
        authoritative.explanation().continuity_class()
    );
    assert_ne!(
        branch_local
            .decision()
            .explanation()
            .selected_candidate_identity(),
        authoritative.explanation().selected_candidate_identity()
    );
    assert_ne!(
        branch_local.branch_local_digest(),
        retained_digest_for_decision(&authoritative)
    );
}

#[test]
fn branch_local_correspondence_never_upgrades_to_authoritative_continuity_under_replay() {
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let correspondence = anchored_surface("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let authoritative_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![ReplacementCandidate::new(
                    "exact",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
                )
                .expect("exact candidate")],
            ),
        ),
    );
    let branch_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![ReplacementCandidate::new(
                    "correspondence",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(correspondence),
                )
                .expect("correspondence candidate")],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-fourteen-branch-correspondence");
    let branch_basis = scoped_branch_head_inspection_basis("branch:correspondence");
    let branch_local = branch_entry
        .branch_local_inspection_with_query(
            &handle,
            &branch_basis,
            branch_basis_evidence(&branch_basis, "branch-evidence:correspondence"),
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    &branch_entry,
                    &handle,
                )),
            ),
        )
        .expect("branch-local inspection");
    let authoritative = authoritative_entry
        .historical_inspection_with_query(
            &handle,
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    &authoritative_entry,
                    &handle,
                )),
            ),
        )
        .expect("authoritative historical inspection");

    assert_eq!(
        branch_local.decision().explanation().continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_ne!(
        branch_local.decision().explanation().continuity_class(),
        authoritative.decision().explanation().continuity_class()
    );
    assert_ne!(
        branch_local.branch_local_digest(),
        authoritative.historical_digest()
    );
}

#[test]
fn wrong_branch_binding_inspection_is_denied_before_cross_branch_reconstruction() {
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![ReplacementCandidate::new(
                    "exact",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact),
                )
                .expect("exact candidate")],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-fourteen-wrong-branch");
    let branch_basis = scoped_branch_head_inspection_basis("branch:expected");
    let result = entry.branch_local_inspection_with_query(
        &handle,
        &branch_basis,
        LowerRuntimeBasisEvidence::from_relational_facade(
            "relational-branch:other",
            "branch-evidence:other",
            1,
        ),
        ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                &entry, &handle,
            )),
        ),
    );

    match result {
        Err(PrimitiveRebindingBranchLocalInspectionError::LowerRuntimeBasis(denial)) => {
            assert_eq!(
                denial.denial_kind(),
                DeniedBasisCapabilityKind::RelationalAuthorityMismatch
            );
            assert!(!denial.decision_trace().trace_digest().is_empty());
        }
        _ => panic!("expected wrong-branch basis mismatch"),
    }
}

fn scoped_branch_head_inspection_basis(branch_identity: &str) -> ScopedInspectionBasis {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: branch_identity.to_string(),
            accessible: true,
        },
        "inspection",
    )
    .expect("branch-head inspection should normalize");
    let eligibility = evaluate_basis_inspection_eligibility(normalized)
        .expect("branch-head inspection should be eligible");

    scope_basis_for_inspection(admit_basis_capability(eligibility))
}

fn branch_basis_evidence(
    scoped_basis: &ScopedInspectionBasis,
    evidence_digest: &str,
) -> LowerRuntimeBasisEvidence {
    LowerRuntimeBasisEvidence::from_relational_facade(
        scoped_basis
            .expected_lower_runtime_binding_digest()
            .expect("branch basis digest"),
        evidence_digest,
        1,
    )
}
