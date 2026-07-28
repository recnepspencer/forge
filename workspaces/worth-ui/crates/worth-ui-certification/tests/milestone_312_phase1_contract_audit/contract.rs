use std::collections::{BTreeMap, BTreeSet};

use worth_ui_certification::topology::WorkspaceSourceInventory;

const ROUTE_DISPOSITIONS: [&str; 5] = [
    "retained proof producer",
    "canonical cutover",
    "typed unavailable family",
    "committed successor",
    "deleted predecessor",
];

pub(super) fn validate(
    contract: &toml::Value,
    inventory: &WorkspaceSourceInventory,
) -> Result<(), String> {
    validate_header(contract)?;
    validate_routes(contract, inventory)?;
    validate_authority(contract)?;
    validate_profile(contract)?;
    super::protocol_contract::validate(contract)?;
    super::topology::validate(contract, inventory)
}

fn validate_header(contract: &toml::Value) -> Result<(), String> {
    require_exact(
        contract,
        "schema",
        "worth-ui.milestone-3.12.phase-1-contract.v1",
    )?;
    require_exact(contract, "milestone", "3.12")?;
    if !matches!(
        required_text(contract, "status")?,
        "implementation" | "closed"
    ) {
        return Err("Phase 1 status is neither implementation nor closed".to_owned());
    }
    required_text(contract, "closure_claim").map(|_| ())
}

fn validate_routes(
    contract: &toml::Value,
    inventory: &WorkspaceSourceInventory,
) -> Result<(), String> {
    let routes = contract["route"]
        .as_array()
        .ok_or_else(|| "route inventory is not an array".to_owned())?;
    let expected_ids = (1..=18)
        .map(|number| format!("R-{number:02}"))
        .collect::<Vec<_>>();
    let actual_ids = routes
        .iter()
        .map(|route| required_text(route, "id").map(str::to_owned))
        .collect::<Result<Vec<_>, _>>()?;
    if actual_ids != expected_ids {
        return Err("route inventory is not exactly ordered R-01 through R-18".to_owned());
    }
    let anchors = route_anchors();
    for route in routes {
        validate_route_shape(route)?;
        validate_route_anchor(route, &anchors, inventory)?;
    }
    Ok(())
}

fn validate_route_shape(route: &toml::Value) -> Result<(), String> {
    let id = required_text(route, "id")?;
    for field in [
        "family",
        "current_entry",
        "current_owner",
        "current_terminal",
        "disposition",
        "successor_home",
    ] {
        required_text(route, field)?;
    }
    let disposition = required_text(route, "disposition")?;
    if !ROUTE_DISPOSITIONS.contains(&disposition) {
        return Err(format!("{id} has unknown disposition `{disposition}`"));
    }
    let phase = route["cutover_phase"]
        .as_integer()
        .ok_or_else(|| format!("{id} has no cutover phase"))?;
    if !(2..=4).contains(&phase) {
        return Err(format!("{id} has invalid cutover phase {phase}"));
    }
    Ok(())
}

fn validate_route_anchor(
    route: &toml::Value,
    anchors: &BTreeMap<&str, (&str, &str)>,
    inventory: &WorkspaceSourceInventory,
) -> Result<(), String> {
    let id = required_text(route, "id")?;
    if id == "R-18" {
        return validate_unimplemented_comparison_successor(route, inventory);
    }
    let (path, symbol) = anchors
        .get(id)
        .ok_or_else(|| format!("{id} has no production anchor"))?;
    let predecessor = inventory.text(path);
    let predecessor_is_active = predecessor.contains(symbol);
    let successors = route_successor_anchors();
    let successor = successors.get(id).and_then(|(path, symbol)| {
        inventory
            .source(path)
            .map(|source| (source.text(), *symbol))
    });
    if !anchor_is_reachable(predecessor, symbol, successor) {
        return Err(format!(
            "{id} has neither predecessor `{symbol}` in {path} nor its exact committed successor"
        ));
    }
    if id == "R-03"
        && predecessor_is_active
        && !inventory
            .text("apps/platform-pulse/src/application.rs")
            .contains("lower_to_candidate_submission")
    {
        return Err("R-03 omits the initial product source-lowering caller".to_owned());
    }
    if id == "R-03"
        && !predecessor_is_active
        && [
            "apps/platform-pulse/src/application.rs",
            "apps/platform-pulse/src/native_frame.rs",
        ]
        .iter()
        .any(|path| !inventory.text(path).contains("attempt_source_rebind"))
    {
        return Err("R-03 omits a committed Platform Pulse source-attempt caller".to_owned());
    }
    Ok(())
}

fn validate_unimplemented_comparison_successor(
    route: &toml::Value,
    inventory: &WorkspaceSourceInventory,
) -> Result<(), String> {
    let parent = inventory.text("crates/worth-ui-runtime/src/inspection/visual_snapshot/mod.rs");
    if route["disposition"].as_str() != Some("committed successor") {
        return Err("R-18 lost its committed-successor disposition".to_owned());
    }
    if !parent.contains("mod comparison") {
        return Ok(());
    }
    let successor = inventory
        .source("crates/worth-ui-runtime/src/inspection/visual_snapshot/comparison/mod.rs")
        .map(|source| source.text());
    if successor.is_some_and(|source| source.contains("UiVisualSnapshotComparison")) {
        Ok(())
    } else {
        Err("R-18 comparison module exists without its committed comparison owner".to_owned())
    }
}

fn anchor_is_reachable(
    predecessor: &str,
    predecessor_symbol: &str,
    successor: Option<(&str, &str)>,
) -> bool {
    predecessor.contains(predecessor_symbol)
        || successor.is_some_and(|(source, symbol)| source.contains(symbol))
}

fn route_anchors() -> BTreeMap<&'static str, (&'static str, &'static str)> {
    BTreeMap::from([
        ("R-01", ("crates/worth-ui-runtime/src/runtime/source_ingress/filesystem/operating_system_watcher.rs", "pub fn settle(")),
        ("R-02", ("crates/worth-ui-runtime/src/runtime/source_ingress/candidate_submission.rs", "lower_to_candidate_submission")),
        ("R-03", ("apps/platform-pulse/src/native_frame.rs", "lower_to_candidate_submission")),
        ("R-04", ("crates/worth-ui-runtime/src/facade/entry/observation_report.rs", "validate_host_observation_batch")),
        ("R-05", ("crates/worth-ui-runtime/src/facade/entry/observation_report.rs", "validate_enqueued_host_observation_batches")),
        ("R-06", ("crates/worth-ui-host-contract/src/observation_report/payload.rs", "PointerMotion")),
        ("R-07", ("crates/worth-ui-runtime/src/host_exchange/measurement_admission/lifecycle.rs", "pub(crate) fn begin(")),
        ("R-08", ("crates/worth-ui-runtime/src/runtime/allocation_frame_dispatch/framework_turn/source_capabilities.rs", "admit_collection_change")),
        ("R-09", ("crates/worth-ui-runtime/src/runtime/viewport_resize/outcome.rs", "UiViewportCommittedReplan")),
        ("R-10", ("crates/worth-ui-runtime/src/runtime/invalidation_narrowing/sources/committed_scroll_sources.rs", "committed_scroll_sources")),
        ("R-11", ("crates/worth-ui-runtime/src/runtime/invalidation_narrowing/sources/committed_portal_source.rs", "committed_portal_source")),
        ("R-12", ("crates/worth-ui-runtime/src/runtime/invalidation_narrowing/mod.rs", "narrow_resolved_frame")),
        ("R-13", ("crates/worth-ui-runtime/src/facade/entry/native_application_replacement.rs", "pub fn replace_application(")),
        ("R-14", ("crates/worth-ui-runtime/src/facade/entry/application_replacement.rs", "WorthUiApplicationReplacementOutcome")),
        ("R-15", ("crates/worth-ui-runtime/src/facade/entry/application_replacement/cutover.rs", "commit_application_activation")),
        ("R-16", ("crates/worth-ui-runtime/src/mounting/session_state/replacement.rs", "commit_graph_replacement_successor")),
        ("R-17", ("crates/worth-ui-runtime/src/mounting/publication.rs", "UiMountedFramePublicationReceipt")),
    ])
}

fn route_successor_anchors() -> BTreeMap<&'static str, (&'static str, &'static str)> {
    BTreeMap::from([
        (
            "R-02",
            (
                "crates/worth-ui-runtime/src/runtime/source_ingress/source_rebind_attempt/mod.rs",
                "UiSourceRebindAttempt",
            ),
        ),
        (
            "R-03",
            (
                "apps/platform-pulse/src/native_frame.rs",
                "attempt_source_rebind",
            ),
        ),
        (
            "R-05",
            (
                "crates/worth-ui-host-contract/src/observation_report/drain.rs",
                "UiHostObservationRetention",
            ),
        ),
        (
            "R-12",
            (
                "crates/worth-ui-runtime/src/runtime/rebind/scope/mod.rs",
                "UiResolvedAffectedScope",
            ),
        ),
        (
            "R-13",
            (
                "crates/worth-ui-runtime/src/facade/entry/rebind/mod.rs",
                "begin_source_rebind",
            ),
        ),
        (
            "R-14",
            (
                "crates/worth-ui-runtime/src/facade/entry/rebind/mod.rs",
                "begin_source_rebind",
            ),
        ),
    ])
}

#[test]
fn migration_anchor_requires_a_real_predecessor_or_exact_successor() {
    assert!(anchor_is_reachable(
        "fn predecessor() {}",
        "predecessor",
        None
    ));
    assert!(anchor_is_reachable(
        "",
        "predecessor",
        Some(("struct ExactSuccessor;", "ExactSuccessor"))
    ));
    assert!(!anchor_is_reachable("", "predecessor", None));
    assert!(!anchor_is_reachable(
        "",
        "predecessor",
        Some(("struct Proxy;", "ExactSuccessor"))
    ));
}

fn validate_authority(contract: &toml::Value) -> Result<(), String> {
    let progression = &contract["authority_progression"];
    for field in ["ordinary", "advanced", "public_executor"] {
        required_text(progression, field)?;
    }
    let forbidden = required_string_set(progression, "forbidden_inputs")?;
    for required in [
        "raw source bytes",
        "pixel identity",
        "generic authority marker",
    ] {
        if !forbidden.contains(required) {
            return Err(format!("authority freeze omits `{required}`"));
        }
    }
    let subsystems = required_string_set(progression, "required_subsystems")?;
    if subsystems != BTreeSet::from(["runtime observation state", "runtime rebind state"]) {
        return Err("required observation/rebind subsystem set drifted".to_owned());
    }
    require_exact(
        &contract["ordering"],
        "cross_family",
        "framework rank plus owner-issued ordering key",
    )?;
    require_exact(
        &contract["ordering"],
        "forbidden",
        "global wall-clock order",
    )?;
    let outcomes = required_string_set(&contract["outcome_topology"], "terminal")?;
    if outcomes.len() != 10
        || !outcomes.contains("Published")
        || !outcomes.contains("InternalDefect")
    {
        return Err("terminal outcome topology drifted".to_owned());
    }
    Ok(())
}

fn validate_profile(contract: &toml::Value) -> Result<(), String> {
    let profile = &contract["platform_pulse_profile"];
    require_exact(
        profile,
        "profile_identity",
        "worth-ui.platform-pulse.change-profile.v1",
    )?;
    for (field, expected) in [
        ("admitted_observations_per_turn", 8),
        ("retained_observation_bytes_per_turn", 65_536),
        ("queued_observations_during_effecting_rebind", 16),
        ("changed_facts", 16),
        ("affected_aspects", 16),
        ("distinct_indexed_consumers", 64),
        ("graph_index_and_mounted_lifecycle_entries", 128),
        ("measurement_allocation_entries", 64),
        ("query_binding_transitions", 16),
        ("obligations", 64),
        ("native_surfaces", 1),
        ("prepared_presentation_bytes", 4_194_304),
        ("pending_plans", 2),
        ("effecting_rebinds", 1),
        ("completion_handles", 1),
        ("recovery_handles", 1),
        ("retained_terminal_decision_records", 64),
        ("evidence_linkage_entries", 512),
        ("causal_neighborhood_bytes", 262_144),
        ("retained_comparison_snapshots", 2),
        ("retained_comparison_rebind_receipts", 1),
        ("comparison_structural_entries", 128),
    ] {
        if profile[field].as_integer() != Some(expected) {
            return Err(format!("Platform Pulse profile field `{field}` drifted"));
        }
    }
    require_exact(
        profile,
        "comparison_pixel_bytes",
        "already-retained bytes only; no duplicate buffer",
    )?;
    Ok(())
}

fn require_exact(value: &toml::Value, field: &str, expected: &str) -> Result<(), String> {
    let actual = required_text(value, field)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("`{field}` expected `{expected}`, found `{actual}`"))
    }
}

fn required_text<'a>(value: &'a toml::Value, field: &str) -> Result<&'a str, String> {
    value[field]
        .as_str()
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| format!("missing nonempty `{field}`"))
}

fn required_string_set<'a>(
    value: &'a toml::Value,
    field: &str,
) -> Result<BTreeSet<&'a str>, String> {
    value[field]
        .as_array()
        .ok_or_else(|| format!("`{field}` is not an array"))?
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .filter(|text| !text.trim().is_empty())
                .ok_or_else(|| format!("`{field}` contains a non-string or empty value"))
        })
        .collect()
}
