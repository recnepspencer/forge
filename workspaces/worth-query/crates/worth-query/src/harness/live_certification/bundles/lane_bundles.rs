use crate::facade::foundation::{
    promote_preflight_bundle_to_live, replay_live_sequence, BridgeChangeSummary, BridgeFieldDelta,
    LiveChangeOrdinal, LiveReplayStepInput, MilestoneFiveLiveAdapter, RefreshAdmissionClass,
};

use super::super::super::profiles::CertificationProfile;
use super::super::model::LiveCertificationBundle;
use super::assembly::{bundle_from_lane, bundle_from_replay_run};
use super::changes::{
    bounded_materialization_patch_change, detail_patch_change, ordered_collection_patch_change,
};

pub(in crate::harness::live_certification) fn detail_patch_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let change = detail_patch_change();
    let lane = MilestoneFiveLiveAdapter::detail_patch_lane(&live, &change)
        .expect("detail patch lane should build");

    bundle_from_lane(profile, &lane)
}

pub(in crate::harness::live_certification) fn detail_suppression_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Esther"),
        Some("Ess"),
    ));
    let lane = MilestoneFiveLiveAdapter::suppression_lane(&live, &change)
        .expect("suppression lane should build");

    bundle_from_lane(profile, &lane)
}

pub(in crate::harness::live_certification) fn ordered_collection_patch_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let change = ordered_collection_patch_change();
    let lane = MilestoneFiveLiveAdapter::ordered_collection_patch_lane(&live, &change)
        .expect("ordered collection lane should build");

    bundle_from_lane(profile, &lane)
}

pub(in crate::harness::live_certification) fn bounded_materialization_patch_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");
    let change = bounded_materialization_patch_change();
    let lane = MilestoneFiveLiveAdapter::bounded_materialization_patch_lane(&live, &change)
        .expect("bounded materialization lane should build");

    bundle_from_lane(profile, &lane)
}

pub(in crate::harness::live_certification) fn detail_replay_end_state_control_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::alternate_basis_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let lane = MilestoneFiveLiveAdapter::detail_patch_lane(&live, &detail_patch_change())
        .expect("detail patch lane should build");

    bundle_from_lane(profile, &lane)
}

pub(in crate::harness::live_certification) fn detail_replay_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let run = replay_live_sequence(
        &live,
        &[LiveReplayStepInput::new(
            detail_patch_change(),
            LiveChangeOrdinal::from_value(1),
            crate::harness::fixtures::resolved_bases::runtime_basis(
                &crate::harness::fixtures::validated_bundles::runtime_detail_bundle(),
                &crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
            ),
        )],
    )
    .expect("detail replay sequence should succeed");

    bundle_from_replay_run(profile, &run)
}

pub(in crate::harness::live_certification) fn ordered_collection_replay_end_state_control_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::alternate_basis_ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let lane = MilestoneFiveLiveAdapter::ordered_collection_patch_lane(
        &live,
        &ordered_collection_patch_change(),
    )
    .expect("ordered collection patch lane should build");

    bundle_from_lane(profile, &lane)
}

pub(in crate::harness::live_certification) fn ordered_collection_replay_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let run = replay_live_sequence(
        &live,
        &[LiveReplayStepInput::new(
            ordered_collection_patch_change(),
            LiveChangeOrdinal::from_value(1),
            crate::harness::fixtures::resolved_bases::runtime_basis(
                &crate::harness::fixtures::validated_bundles::ordered_collection_without_traversal_bundle(),
                &crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
            ),
        )],
    )
    .expect("ordered collection replay sequence should succeed");

    bundle_from_replay_run(profile, &run)
}

pub(in crate::harness::live_certification) fn bounded_materialization_replay_end_state_control_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::alternate_basis_bounded_materialization_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");
    let lane = MilestoneFiveLiveAdapter::bounded_materialization_patch_lane(
        &live,
        &bounded_materialization_patch_change(),
    )
    .expect("bounded materialization patch lane should build");

    bundle_from_lane(profile, &lane)
}

pub(in crate::harness::live_certification) fn bounded_materialization_replay_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");
    let run = replay_live_sequence(
        &live,
        &[LiveReplayStepInput::new(
            bounded_materialization_patch_change(),
            LiveChangeOrdinal::from_value(1),
            crate::harness::fixtures::resolved_bases::runtime_basis(
                &crate::harness::fixtures::validated_bundles::ordered_collection_bundle(),
                &crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
            ),
        )],
    )
    .expect("bounded materialization replay sequence should succeed");

    bundle_from_replay_run(profile, &run)
}

pub(in crate::harness::live_certification) fn bounded_materialization_refresh_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");

    let lane = MilestoneFiveLiveAdapter::refresh_fallback_lane(
        &live,
        RefreshAdmissionClass::WidthOverflow,
    )
    .expect("refresh fallback lane should build");

    bundle_from_lane(profile, &lane)
}

pub(in crate::harness::live_certification) fn coalesced_delivery_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");

    let lane = MilestoneFiveLiveAdapter::coalesced_delivery_lane(&live, 3)
        .expect("coalesced delivery lane should build");

    bundle_from_lane(profile, &lane)
}

pub(in crate::harness::live_certification) fn progress_advance_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let lane = MilestoneFiveLiveAdapter::progress_advance_lane(
        &live,
        LiveChangeOrdinal::from_value(1),
        preflight.basis().clone(),
    )
    .expect("progress advance lane should build");

    bundle_from_lane(profile, &lane)
}
