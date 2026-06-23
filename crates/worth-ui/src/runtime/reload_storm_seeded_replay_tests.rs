use super::reload_storm_certification_test_support::{
    file_token_provider, invalid_file_provider, runtime_with_token, rust_token_provider, storm_app,
};
use super::*;
use crate::facade::WorthUiApp;

#[test]
fn seeded_mixed_reload_storms_replay_full_runtime_truth() {
    for seed in [0x51, 0x93, 0xd7, 0x12f] {
        let app = storm_app();
        let mut original_runtime = runtime_with_token(&app, "theme.text.primary");
        let mut replay_runtime = runtime_with_token(&app, "theme.text.primary");
        let scenario = seeded_scenario(&app, seed);

        let original = original_runtime
            .certify_reload_storm_against_snapshot(scenario.clone(), app.capabilities())
            .expect("original seeded storm certifies");
        let replayed = replay_runtime
            .certify_reload_storm_against_snapshot(scenario, app.capabilities())
            .expect("replayed seeded storm certifies");
        let replay = WorthUiReloadReplayCertification::certify(&original, &replayed)
            .expect("seeded mixed storm converges under replay");

        assert_eq!(
            replay.final_active_artifact_digest(),
            original.ordered_truth().final_active_artifact_digest()
        );
        assert_eq!(
            replay.final_capability_snapshot_digest(),
            original.ordered_truth().final_capability_snapshot_digest()
        );
        assert_eq!(
            replay.final_authoring_snapshot_digest(),
            original.ordered_truth().final_authoring_snapshot_digest()
        );
    }
}

fn seeded_scenario(app: &WorthUiApp, seed: u64) -> WorthUiReloadStormScenario {
    let mut cursor = seed;
    let mut scenario = WorthUiReloadStormScenario::named(format!("seeded-reload-replay-{seed}"))
        .with_file_candidate(
            "required invalid file",
            invalid_file_provider("not worth ui"),
        )
        .with_rust_candidate(
            "required rust equivalent",
            rust_token_provider(app, "theme.text.primary"),
        );
    for index in 0..5 {
        cursor = cursor.wrapping_mul(6364136223846793005).wrapping_add(1);
        let label = format!("generated step {index}");
        scenario = match cursor % 4 {
            0 => scenario.with_file_candidate(label, file_token_provider("theme.text.primary")),
            1 => scenario.with_file_candidate(label, file_token_provider("theme.text.secondary")),
            2 => {
                scenario.with_rust_candidate(label, rust_token_provider(app, "theme.text.primary"))
            }
            _ => scenario
                .with_rust_candidate(label, rust_token_provider(app, "theme.text.secondary")),
        };
    }
    scenario
}
