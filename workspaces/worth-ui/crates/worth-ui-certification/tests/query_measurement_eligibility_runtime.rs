#[path = "fixtures/query_measurement_eligibility_support/mod.rs"]
mod query_measurement_eligibility_support;

use worth_ui::facade::admission::{UiAdmissionQueryBasis, UiAdmissionWorld};
use worth_ui::facade::obligations::UiObligationFamily;
use worth_ui_query_binding::WorthUiQueryMeasurementFactFamily;
use worth_ui_test_support::{
    UiMeasurementAdmissionPosture, UiMeasurementUnsupportedReason,
    UiQueryMeasurementBasisAuthority, UiQueryMeasurementEligibilityPosture,
    UiQueryMeasurementUnsupportedQueryReason,
};

use self::query_measurement_eligibility_support::{
    available_measurement_target, denied_display_field_projection_consumption,
    display_and_view_local_projection_consumptions, display_field_projection_consumption,
    display_projection_consumptions_across_basis_generations, measurement_touch,
    query_measurement_app, query_only_measurement_app, synthetic_query_prerequisites_for_world,
    target_bound_to_projection_consumption, view_local_only_projection_consumption,
};

#[test]
fn query_backed_measurement_eligibility_stays_distinct_and_bounds_required_families() {
    let (world_profile, display_consumption) =
        display_field_projection_consumption("eligible-display");
    let app = query_measurement_app(world_profile);
    let touch = measurement_touch(&app, 0);
    let eligibility = app
        .admit_query_measurement_eligibility_for_touch_from_query_authority(
            &touch,
            display_consumption.clone(),
        )
        .expect("query-backed measurement should produce typed query eligibility");
    let selected = app.admission().select_obligations_for_target(
        &touch,
        target_bound_to_projection_consumption(&touch, &display_consumption),
    );
    let measurement_admission = app
        .admission()
        .admit_measurement_requirement(&selected)
        .expect("measurement touch should produce measurement admission");

    assert_eq!(
        measurement_admission.posture(),
        &UiMeasurementAdmissionPosture::Unsupported {
            world: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
            reason: UiMeasurementUnsupportedReason::SelectionDidNotYieldMeasurementRequirement,
        }
    );
    assert!(
        selected
            .obligation_for_family(UiObligationFamily::MeasurementRequirement)
            .is_none(),
        "query measurement eligibility should stay distinct from host-observation-selected measurement requirement admission"
    );
    assert_eq!(
        eligibility.required_families(),
        &[WorthUiQueryMeasurementFactFamily::ScrollContentExtent]
    );
    assert!(eligibility.query_basis_digest_for_diagnostics().is_some());
    assert!(eligibility.query_resolution_mode().is_some());
    let receipt = eligibility.projection_fact_receipt().expect(
        "eligible query-backed measurement should carry the consumed projection fact receipt",
    );
    assert_eq!(
        receipt.required_query_fact_families(),
        &[WorthUiQueryMeasurementFactFamily::ScrollContentExtent]
    );
    assert_eq!(
        receipt.required_query_fact_family_set_digest(),
        eligibility.required_fact_family_set_digest()
    );
    assert_eq!(
        receipt.consumed_fact_families(),
        &[WorthUiQueryMeasurementFactFamily::ScrollContentExtent]
    );
    assert_eq!(
        eligibility.selected_measurement_obligation_identity_digest(),
        measurement_admission.selected_measurement_obligation_identity_digest()
    );
    assert_eq!(
        eligibility.posture(),
        &UiQueryMeasurementEligibilityPosture::Eligible {
            world: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
            available_families: vec![WorthUiQueryMeasurementFactFamily::ScrollContentExtent]
                .into_boxed_slice(),
            available_fact_family_set_digest: eligibility.required_fact_family_set_digest(),
        }
    );
}

#[test]
fn wrong_world_query_measurement_eligibility_remains_explicit_prerequisite_residue() {
    let (current_world, current_display_consumption) =
        display_field_projection_consumption("wrong-world-residue");
    let app = query_measurement_app(current_world.clone());
    let touch = measurement_touch(&app, 0);

    let wrong_world_target = available_measurement_target(&touch).with_query_prerequisites(
        synthetic_query_prerequisites_for_world(
            touch.world().world_profile(),
            UiAdmissionQueryBasis::WrongWorldProjection,
        ),
    );
    let wrong_world_selected = app
        .admission()
        .select_obligations_for_target(&touch, wrong_world_target);
    let wrong_world_measurement = app
        .admission()
        .admit_measurement_requirement(&wrong_world_selected)
        .expect("phase-4 measurement admission should remain available as an identity carrier");
    let wrong_world = app
        .admission()
        .admit_query_measurement_eligibility_from_query_authority(
            &wrong_world_selected,
            &wrong_world_measurement,
            current_display_consumption,
        )
        .expect("query-backed measurement should produce typed query denial");

    assert_eq!(
        wrong_world.posture(),
        &UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
            world: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
            reason: UiQueryMeasurementUnsupportedQueryReason::WrongWorldProjection,
        }
    );
}

#[test]
fn query_measurement_eligibility_from_projection_consumption_rejects_cross_basis_consumption() {
    let ((current_world, current_display_consumption), (_, next_display_consumption)) =
        display_projection_consumptions_across_basis_generations("cross-basis");
    let app = query_measurement_app(current_world);
    let touch = measurement_touch(&app, 0);
    let target = target_bound_to_projection_consumption(&touch, &current_display_consumption);
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target);
    let measurement_admission = app
        .admission()
        .admit_measurement_requirement(&selected)
        .expect("query-backed measurement should still lower a phase-4 admission artifact");
    let stale = app
        .admission()
        .admit_query_measurement_eligibility_from_query_authority(
            &selected,
            &measurement_admission,
            next_display_consumption,
        )
        .expect("cross-basis projection consumption should produce a typed query denial");

    assert!(matches!(
        stale.posture(),
        UiQueryMeasurementEligibilityPosture::StaleBasisGeneration { .. }
    ));
    match stale.posture() {
        UiQueryMeasurementEligibilityPosture::StaleBasisGeneration {
            expected, observed, ..
        } => match (expected, observed) {
            (
                UiQueryMeasurementBasisAuthority::AdmittedPrerequisites { prerequisites },
                UiQueryMeasurementBasisAuthority::ProjectionConsumption { authority },
            ) => {
                assert_ne!(
                    prerequisites.basis_digest_for_diagnostics(),
                    authority.basis_digest_for_diagnostics()
                );
                assert_ne!(
                    prerequisites.canonical_basis_digest().value().bytes(),
                    authority
                        .authority_index_key()
                        .expect("authority should be indexable")
                        .canonical_basis_identity()
                );
                assert_eq!(
                    prerequisites.resolution_mode(),
                    measurement_admission
                        .target()
                        .query_prerequisites()
                        .expect("current target should retain query prerequisites")
                        .resolution_mode()
                );
                assert_eq!(
                    prerequisites
                        .projection_contract_identity()
                        .map(|identity| identity.as_u64()),
                    Some(
                        authority
                            .authority_index_key()
                            .expect("authority should be indexable")
                            .projection_contract_identity()
                    )
                );
            }
            authorities => {
                panic!("expected admitted-prerequisites stale authorities, got {authorities:?}")
            }
        },
        posture => panic!("expected stale basis posture, got {posture:?}"),
    }
}

#[test]
fn query_measurement_eligibility_for_touch_reports_unavailable_required_fact_families() {
    let (world_profile, view_local_consumption) =
        view_local_only_projection_consumption("missing-family");
    let app = query_measurement_app(world_profile);
    let touch = measurement_touch(&app, 0);
    let unavailable = app
        .admit_query_measurement_eligibility_for_touch_from_query_authority(
            &touch,
            view_local_consumption,
        )
        .expect("missing required fact family should produce typed denial");
    assert!(
        unavailable.projection_fact_receipt().is_none(),
        "missing required fact families should not masquerade as an admitted measurement receipt"
    );

    assert_eq!(
        unavailable.posture(),
        &UiQueryMeasurementEligibilityPosture::UnavailableFactFamilies {
            world: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
            available_families: Box::new([]),
            missing_families: vec![WorthUiQueryMeasurementFactFamily::ScrollContentExtent]
                .into_boxed_slice(),
        }
    );
}

#[test]
fn query_measurement_eligibility_for_touch_rejects_unavailable_projection_consumption_without_fallback(
) {
    let (world_profile, denied_consumption) =
        denied_display_field_projection_consumption("unavailable-consumption");
    let _ = world_profile;
    assert!(
        worth_ui_query_binding::compatibility::managed_live::WorthUiQueryAuthorityHandle::from_outcome(denied_consumption)
            .is_err(),
        "denied Query consumption must not produce a downstream authority handle"
    );
}

#[test]
fn host_only_measurement_requirements_do_not_widen_public_query_measurement_identity() {
    let (world_profile, display_consumption) =
        display_field_projection_consumption("public-narrowing");
    let host_and_query_app = query_measurement_app(world_profile.clone());
    let query_only_app = query_only_measurement_app(world_profile);
    let host_and_query = host_and_query_app
        .admit_query_measurement_eligibility_for_touch_from_query_authority(
            &measurement_touch(&host_and_query_app, 0),
            display_consumption.clone(),
        )
        .expect("host-plus-query declaration should admit");
    let query_only = query_only_app
        .admit_query_measurement_eligibility_for_touch_from_query_authority(
            &measurement_touch(&query_only_app, 0),
            display_consumption,
        )
        .expect("query-only declaration should admit");
    let host_and_query_receipt = host_and_query
        .projection_fact_receipt()
        .expect("eligible host-plus-query declaration should carry a receipt");
    let query_only_receipt = query_only
        .projection_fact_receipt()
        .expect("eligible query-only declaration should carry a receipt");

    assert_eq!(
        host_and_query.required_families(),
        query_only.required_families()
    );
    assert_eq!(
        host_and_query.required_fact_family_set_digest(),
        query_only.required_fact_family_set_digest()
    );
    assert_eq!(
        host_and_query_receipt.required_query_fact_family_set_digest(),
        query_only_receipt.required_query_fact_family_set_digest()
    );
    assert_eq!(
        host_and_query_receipt.consumed_fact_family_set_digest(),
        query_only_receipt.consumed_fact_family_set_digest()
    );
    assert_eq!(
        host_and_query_receipt.required_query_fact_families(),
        query_only_receipt.required_query_fact_families()
    );
    assert_eq!(
        host_and_query_receipt.consumed_fact_families(),
        query_only_receipt.consumed_fact_families()
    );
    assert_ne!(
        host_and_query_receipt.required_measurement_dependencies(),
        query_only_receipt.required_measurement_dependencies(),
        "host-only measurement evidence should stay visible on the declaration receipt without widening Query invalidation identity"
    );
}

#[test]
fn portal_anchored_measurement_does_not_widen_into_query_fact_eligibility() {
    let (world_profile, display_consumption) =
        display_field_projection_consumption("portal-display");
    let app = query_measurement_app(world_profile);
    let touch = measurement_touch(&app, 1);
    assert!(
        app.admit_query_measurement_eligibility_for_touch_from_query_authority(
            &touch,
            display_consumption,
        )
        .is_none(),
        "portal-anchored measurement should not acquire query fact eligibility from unrelated query families"
    );
}

#[test]
fn query_measurement_eligibility_from_projection_consumption_rejects_same_basis_different_projection_contract_scope(
) {
    let (world_profile, display_consumption, view_local_consumption) =
        display_and_view_local_projection_consumptions("contract-scope");
    let app = query_measurement_app(world_profile);
    let touch = measurement_touch(&app, 0);
    let target = target_bound_to_projection_consumption(&touch, &display_consumption);
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target);
    let measurement_admission = app
        .admission()
        .admit_measurement_requirement(&selected)
        .expect("query-backed measurement should still lower a phase-4 admission artifact");
    let denial = app
        .admission()
        .admit_query_measurement_eligibility_from_query_authority(
            &selected,
            &measurement_admission,
            view_local_consumption,
        )
        .expect("scope mismatch should produce a typed query denial");
    assert!(
        denial.projection_fact_receipt().is_none(),
        "stale query contract scope should deny before carrying an admitted projection receipt"
    );

    match denial.posture() {
        UiQueryMeasurementEligibilityPosture::StaleBasisGeneration {
            expected, observed, ..
        } => match (expected, observed) {
            (
                UiQueryMeasurementBasisAuthority::AdmittedPrerequisites {
                    prerequisites: expected,
                },
                UiQueryMeasurementBasisAuthority::ProjectionConsumption {
                    authority: observed,
                },
            ) => {
                assert_eq!(
                    expected.basis_digest_for_diagnostics(),
                    observed.basis_digest_for_diagnostics()
                );
                assert_ne!(
                    expected
                        .projection_contract_identity()
                        .map(|identity| identity.as_u64()),
                    Some(
                        observed
                            .authority_index_key()
                            .expect("authority should be indexable")
                            .projection_contract_identity()
                    )
                );
            }
            authorities => {
                panic!("expected retained prerequisites and consumed projection authority, got {authorities:?}")
            }
        },
        posture => panic!("expected contract-scope stale denial, got {posture:?}"),
    }
}
