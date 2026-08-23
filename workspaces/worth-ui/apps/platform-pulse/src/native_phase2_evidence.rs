pub(super) fn native_phase2_evidence(
    receipt: &worth_ui_native_platform::UiNativePlatformCloseReceipt,
) -> serde_json::Value {
    serde_json::json!({
        "schema": "worth-ui-native-phase2-evidence-v1",
        "presentation": presentation_evidence(receipt),
        "runtime_attribution": attribution_evidence(receipt),
        "counters": counter_evidence(receipt),
        "graphics": graphics_evidence(receipt),
        "peak": peak_census_evidence(receipt),
        "terminal_census": terminal_census_evidence(receipt),
        "terminal_zero": receipt.terminal_census().is_zero(),
    })
}

fn presentation_evidence(
    receipt: &worth_ui_native_platform::UiNativePlatformCloseReceipt,
) -> serde_json::Value {
    let presentation = receipt.presentation();
    serde_json::json!({
        "presented_source": presentation.source_rgba8(),
        "retained_center": presentation.retained_center_rgba8(),
        "retained_baseline": presentation.retained_baseline_rgba8(),
        "client_physical_size": presentation.client_physical_size(),
        "scale_factor_milli": presentation.scale_factor_milli(),
        "frame": presentation.presented_frame(),
        "surface": presentation.semantic_surface(),
        "binding": presentation.binding_generation(),
        "mounted_instance": presentation.mounted_instance(),
        "node_receipt": presentation.node_receipt(),
        "presentation_attempt": presentation.presentation_attempt(),
        "logical_bounds_milli": presentation.logical_bounds_milli(),
        "order_ordinal": presentation.order_ordinal(),
    })
}

fn attribution_evidence(
    receipt: &worth_ui_native_platform::UiNativePlatformCloseReceipt,
) -> serde_json::Value {
    let attribution = receipt.client_attribution();
    serde_json::json!({
        "frame": attribution.frame(),
        "surface": attribution.surface(),
        "binding": attribution.binding(),
        "mounted_instance": attribution.mounted_instance(),
        "node_receipt": attribution.node_receipt(),
        "presentation_attempt": attribution.presentation_attempt(),
        "authored_provenance_digest": attribution.authored_provenance_digest(),
        "authored_semantic_identity_digest": attribution.authored_semantic_identity_digest(),
    })
}

fn counter_evidence(
    receipt: &worth_ui_native_platform::UiNativePlatformCloseReceipt,
) -> serde_json::Value {
    let cost = receipt.presentation().cost();
    serde_json::json!({
        "surface_acquisitions": cost.surface_acquisitions(),
        "queue_submissions": cost.queue_submissions(),
        "presents": cost.presents(),
        "render_passes": cost.render_passes(),
        "readiness_signals": receipt.readiness_signals(),
        "redraw_turns": receipt.redraw_turns(),
        "idle_wait_turns": receipt.idle_wait_turns(),
        "coalesced_wakes": receipt.coalesced_wakes(),
        "port_crossings": receipt.port_crossings(),
    })
}

fn graphics_evidence(
    receipt: &worth_ui_native_platform::UiNativePlatformCloseReceipt,
) -> serde_json::Value {
    serde_json::json!({
        "event_loop_thread": receipt.event_loop_thread(),
        "event_loop_thread_matches_launch": receipt.event_loop_thread_matches_launch(),
        "event_loop_thread_posture": receipt.event_loop_thread_posture().label(),
        "adapter": receipt.graphics().adapter_name(),
        "vendor": receipt.graphics().vendor(),
        "device": receipt.graphics().device(),
        "driver": receipt.graphics().driver(),
        "driver_info": receipt.graphics().driver_info(),
        "device_type": receipt.graphics().device_type(),
        "backend": receipt.graphics().backend(),
        "surface_format": receipt.graphics().surface_format(),
        "present_mode": receipt.graphics().present_mode(),
        "alpha_mode": receipt.graphics().alpha_mode(),
        "retained_format": receipt.graphics().retained_format(),
        "max_texture_dimension_2d": receipt.graphics().max_texture_dimension_2d(),
    })
}

fn peak_census_evidence(
    receipt: &worth_ui_native_platform::UiNativePlatformCloseReceipt,
) -> serde_json::Value {
    serde_json::Value::Object(
        receipt
            .peak_census()
            .entries()
            .map(|(class, count)| (class.to_owned(), serde_json::Value::from(count)))
            .collect::<serde_json::Map<_, _>>(),
    )
}

fn terminal_census_evidence(
    receipt: &worth_ui_native_platform::UiNativePlatformCloseReceipt,
) -> serde_json::Value {
    serde_json::Value::Object(
        receipt
            .terminal_census()
            .entries()
            .map(|(class, count)| (class.to_owned(), serde_json::Value::from(count)))
            .collect::<serde_json::Map<_, _>>(),
    )
}
