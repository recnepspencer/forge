use super::assert_sources_exclude;

const PHYSICAL_RUNTIME: &str = "src/physical_runtime";

#[test]
fn reopen_cannot_consume_serialized_signal_state() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "serialized-signal-reopen",
        &["SerializedSignalReopenState"],
    );
}

#[test]
fn ordinary_physical_work_cannot_add_an_internal_json_carrier() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "internal-json-carrier",
        &["InternalPhysicalWorkJsonCarrier"],
    );
}
