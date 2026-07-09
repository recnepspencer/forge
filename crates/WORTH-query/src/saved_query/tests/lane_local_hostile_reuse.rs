use super::temporal_async_fixtures::{
    exact_policy_basis_reuse_descriptor, freeze_future_preserving_grouped_saved_query,
    saved_query_reuse_descriptor_for_saved,
};
use crate::saved_query::{
    evaluate_saved_query_reuse, SavedQueryRebindingDimension, SavedQueryRebindingLegality,
    SavedQueryReuseDescriptor, SavedQueryReuseOutcome, SavedQueryTemporalAsyncSurfacePosture,
};

#[test]
fn preserved_grouped_reuse_denies_when_basis_family_is_erased_even_if_other_digests_match() {
    let saved = freeze_future_preserving_grouped_saved_query();
    let descriptor = SavedQueryReuseDescriptor::new(
        saved.metadata().schema_basis_digest().clone(),
        None,
        saved.metadata().template_binding_digest().cloned(),
        saved.metadata().template_slot_count(),
        saved.metadata().view_shape_digest().clone(),
        saved.metadata().view_shape_family(),
        saved.metadata().result_shape_family().clone(),
        saved.metadata().composition_digest().clone(),
        saved.metadata().scope_lineage_digest().cloned(),
        saved.metadata().support_profile_digest().to_string(),
        saved.metadata().capability_family_identity().to_string(),
    )
    .with_identity_consumption(saved.metadata().identity_consumption().clone());

    let SavedQueryReuseOutcome::Denied(denial) = evaluate_saved_query_reuse(&saved, &descriptor)
    else {
        panic!("erasing basis family must deny preserved grouped reuse");
    };
    assert_eq!(
        denial.temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly
    );
    assert_eq!(
        denial.temporal_async_drift_dimension(),
        Some(SavedQueryRebindingDimension::TemporalAsyncSurface)
    );
    let basis_family_row = denial
        .matrix()
        .rows()
        .iter()
        .find(|row| row.dimension() == SavedQueryRebindingDimension::BasisFamily)
        .expect("basis-family drift row should exist");
    let temporal_async_row = denial
        .matrix()
        .rows()
        .iter()
        .find(|row| row.dimension() == SavedQueryRebindingDimension::TemporalAsyncSurface)
        .expect("temporal/async drift row should exist");
    let support_profile_row = denial
        .matrix()
        .rows()
        .iter()
        .find(|row| row.dimension() == SavedQueryRebindingDimension::SupportProfile)
        .expect("support-profile row should exist");

    assert_eq!(
        basis_family_row.legality(),
        SavedQueryRebindingLegality::IllegalSemanticDrift
    );
    assert_eq!(
        temporal_async_row.legality(),
        SavedQueryRebindingLegality::IllegalSemanticDrift
    );
    assert_eq!(
        support_profile_row.legality(),
        SavedQueryRebindingLegality::LegalNoSemanticChange
    );
}

#[test]
fn preserved_grouped_reuse_requires_policy_basis_evidence_even_when_lane_local_shape_matches() {
    let saved = freeze_future_preserving_grouped_saved_query();
    let exact_descriptor = saved_query_reuse_descriptor_for_saved(&saved)
        .with_policy_basis_reuse_descriptor(exact_policy_basis_reuse_descriptor(
            saved.digest().as_str(),
            saved
                .metadata()
                .basis_family()
                .expect("future-preserving grouped saved query should carry basis family"),
        ));

    let SavedQueryReuseOutcome::Admitted(_) = evaluate_saved_query_reuse(&saved, &exact_descriptor)
    else {
        panic!("exact grouped descriptor with policy-basis evidence should admit");
    };

    let SavedQueryReuseOutcome::Denied(denial) =
        evaluate_saved_query_reuse(&saved, &saved_query_reuse_descriptor_for_saved(&saved))
    else {
        panic!("broad descriptor equality without policy-basis evidence must deny");
    };
    assert_eq!(
        denial.temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
    );
    assert_eq!(
        denial.temporal_async_drift_dimension(),
        Some(SavedQueryRebindingDimension::PolicyBasisReuse)
    );
}
