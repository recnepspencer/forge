use serde_json::Value;

pub(super) fn resource_census_fixture(count: u64, retained_targets: u64) -> Value {
    let mut census = serde_json::Map::new();
    for class in worth_ui_host_native::UiNativeResourceCensus::field_names() {
        let observed = if [
            "windows",
            "surfaces",
            "adapters",
            "devices",
            "queues",
            "retained_targets",
            "registrations",
            "readback_buffers",
            "pending_submissions",
            "event_wake_registrations",
            "application_drivers",
            "physical_signal_runtimes",
            "physical_signal_workers",
            "physical_signal_transition_observations",
            "retained_draw_lists",
            "presentation_epochs",
            "retained_frame_observations",
        ]
        .contains(&class)
        {
            count
        } else {
            0
        };
        census.insert(class.to_owned(), Value::from(observed));
    }
    census.insert("retained_targets".to_owned(), Value::from(retained_targets));
    Value::Object(census)
}

pub(super) fn expected_native_seed_authored_provenance_digest() -> u64 {
    independent_text_digest("app/native_seed.wui") ^ 1_u64.rotate_left(13)
}

pub(super) fn expected_native_seed_authored_semantic_identity_digest() -> u64 {
    independent_text_digest("component:platform.pulse.native_seed.rectangle")
}

fn independent_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325_u64, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
