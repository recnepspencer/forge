use std::collections::BTreeSet;

use toml::Value;

use super::evidence_document::{toml_rows, toml_text, toml_texts};

const OBSERVATION_VARIANTS: &[&str] = &[
    "ProcessStarted",
    "FirstFramePublished",
    "ReplacementPublished",
    "ReplacementDeniedPreserving",
    "ShutdownCompleted",
    "TerminalFailure",
];
const TRANSITION_IDS: &[&str] = &[
    "T01_INSTALL",
    "T02_FIRST_FRAME",
    "T03_VALID_EDIT",
    "T04_REPLACEMENT",
    "T05_MALFORMED_EDIT",
    "T06_PRESERVATION",
    "T07_RECOVERY_EDIT",
    "T08_CLOSE",
];
const TRANSITIONS: &[(&str, &str, &str, &str)] = &[
    (
        "T01_INSTALL",
        "PulseExecutableWorld<Installed>",
        "launch",
        "PulseExecutableWorld<AwaitingFirstFrame>",
    ),
    (
        "T02_FIRST_FRAME",
        "PulseExecutableWorld<AwaitingFirstFrame>",
        "observe_first_frame",
        "PulseExecutableWorld<Published>",
    ),
    (
        "T03_VALID_EDIT",
        "PulseExecutableWorld<Published>",
        "apply_valid_edit",
        "PulseExecutableWorld<AwaitingReplacement>",
    ),
    (
        "T04_REPLACEMENT",
        "PulseExecutableWorld<AwaitingReplacement>",
        "observe_replacement",
        "PulseExecutableWorld<Published>",
    ),
    (
        "T05_MALFORMED_EDIT",
        "PulseExecutableWorld<Published>",
        "apply_malformed_edit",
        "PulseExecutableWorld<AwaitingPreservation>",
    ),
    (
        "T06_PRESERVATION",
        "PulseExecutableWorld<AwaitingPreservation>",
        "observe_preserved_predecessor",
        "PulseExecutableWorld<PreservedPredecessor>",
    ),
    (
        "T07_RECOVERY_EDIT",
        "PulseExecutableWorld<PreservedPredecessor>",
        "restore_canonical_source",
        "PulseExecutableWorld<AwaitingReplacement>",
    ),
    (
        "T08_CLOSE",
        "PulseExecutableWorld<Published>",
        "close_native_window",
        "PulseExecutableWorld<Closed>",
    ),
];
const WORLD_IDS: &[&str] = &[
    "W01_CANONICAL_PLATFORM_PULSE",
    "W02_MALFORMED_PULSE_SOURCE",
    "W03_MISSING_PULSE_SOURCE",
    "W04_INTERRUPTED_PULSE_PROCESS",
];
const ORACLE_IDS: &[&str] = &["O01_EXTERNAL_CONSEQUENCE", "O02_PRODUCT_CAUSAL"];
const MUTATION_IDS: &[&str] = &[
    "M01_PREMATURE_EXIT",
    "M02_EVENT_ONLY",
    "M03_PIXEL_ONLY",
    "M04_WATCHER_BYPASS",
    "M05_WRONG_DENIAL",
    "M06_PREDECESSOR_DRIFT",
    "M07_DIRECT_PAINT",
    "M08_FORCED_TERMINATION",
    "M09_SKIPPED_PLATFORM",
];
const MUTATION_EVIDENCE: &[(&str, &[&str])] = &[
    (
        "M01_PREMATURE_EXIT",
        &["process-liveness", "process-bound-window", "external-blue"],
    ),
    (
        "M02_EVENT_ONLY",
        &[
            "process-liveness",
            "process-bound-window",
            "external-pixels",
        ],
    ),
    (
        "M03_PIXEL_ONLY",
        &[
            "run-identity",
            "sequence",
            "published-generation",
            "mounted-frame",
        ],
    ),
    (
        "M04_WATCHER_BYPASS",
        &["settled-snapshot", "replacement-publication"],
    ),
    (
        "M05_WRONG_DENIAL",
        &["typed-source-denial", "predecessor-preservation"],
    ),
    (
        "M06_PREDECESSOR_DRIFT",
        &[
            "generation-continuity",
            "frame-continuity",
            "green-continuity",
        ],
    ),
    (
        "M07_DIRECT_PAINT",
        &["host-derived-native-effect", "mounted-frame-correlation"],
    ),
    (
        "M08_FORCED_TERMINATION",
        &["normal-native-close", "typed-shutdown", "zero-residue"],
    ),
    (
        "M09_SKIPPED_PLATFORM",
        &["certified-windows-execution-count"],
    ),
];
const SUCCESSOR_HOMES: &[(&str, &str)] = &[
    ("3.11", "adjudication/identity_trace.rs"),
    (
        "3.12",
        "host_action/viewport.rs and adjudication/bounded_rebind.rs",
    ),
    ("3.13", "installation/query_projection.rs"),
    ("3.14", "host_action/intent.rs"),
    ("3.15", "host_action/service.rs"),
    ("3.16", "source_delta/theme_appearance.rs"),
    ("3.17", "source_delta/authored_expression.rs"),
    ("3.18", "source_delta/authored_module.rs"),
    ("3.19", "external_observation/inspection.rs"),
    ("3.20", "external_observation/inspection.rs"),
    ("3.21", "external_observation/inspection.rs"),
    ("3.22", "external_observation/inspection.rs"),
    ("3.23", "courtroom/workflow_editor.rs"),
];
const HOSTILE_STEPS: &[&str] = &[
    "launch-exact-cargo-built-binary",
    "observe-first-frame-within-five-seconds",
    "bind-native-window-to-child",
    "hold-live-for-500-milliseconds-and-observe-blue",
    "atomically-edit-blue-to-green",
    "observe-typed-replacement-within-five-seconds",
    "observe-green-client-area",
    "write-stable-malformed-source",
    "observe-typed-preservation-within-five-seconds",
    "prove-same-live-green-predecessor",
    "restore-exact-canonical-bytes",
    "observe-fresh-blue-successor-within-five-seconds",
    "request-normal-native-close",
    "observe-typed-shutdown-and-successful-exit-within-five-seconds",
    "prove-zero-owned-residue",
];

pub(super) fn audit(document: &Value) -> Result<(), String> {
    audit_protocol(document)?;
    audit_typestate(document)?;
    audit_worlds(document)?;
    audit_native_boundary(document)?;
    audit_oracles(document)?;
    audit_hostile_sequence(document)?;
    audit_mutations(document)?;
    audit_successor_extensions(document)
}

fn audit_protocol(document: &Value) -> Result<(), String> {
    let protocol = table(document, "observation_protocol")?;
    if toml_text(protocol, "identity")? != "worth-ui.platform-pulse.lifecycle-observation"
        || protocol.get("schema_version").and_then(Value::as_integer) != Some(1)
        || toml_text(protocol, "stdout_prefix")? != "WORTH_UI_PLATFORM_PULSE_EVENT "
        || protocol.get("sequence_origin").and_then(Value::as_integer) != Some(1)
        || toml_text(protocol, "sequence_rule")? != "increase-exactly-by-one"
        || protocol.get("maximum_events").and_then(Value::as_integer) != Some(256)
        || protocol
            .get("maximum_encoded_bytes")
            .and_then(Value::as_integer)
            != Some(1_048_576)
    {
        return Err("Phase 1 lifecycle observation protocol drifted".to_owned());
    }
    require_exact_texts(protocol, "variants", OBSERVATION_VARIANTS)?;
    require_exact_texts(
        protocol,
        "forbidden_payloads",
        &[
            "raw-source-text",
            "credentials",
            "arbitrary-debug-output",
            "unbounded-diagnostics",
        ],
    )
}

fn audit_typestate(document: &Value) -> Result<(), String> {
    require_exact_ids(document, "typestate_transition", TRANSITION_IDS)?;
    for (id, expected_from, expected_action, expected_to) in TRANSITIONS {
        let transition = row(document, "typestate_transition", id)?;
        let actual = (
            toml_text(transition, "from")?,
            toml_text(transition, "action")?,
            toml_text(transition, "to")?,
        );
        let expected = (*expected_from, *expected_action, *expected_to);
        if actual != expected {
            return Err(format!(
                "typestate transition `{id}` should be {expected:?}; found {actual:?}"
            ));
        }
    }
    Ok(())
}

fn audit_worlds(document: &Value) -> Result<(), String> {
    require_exact_ids(document, "canonical_world", WORLD_IDS)?;
    for row in toml_rows(document, "canonical_world")? {
        toml_text(row, "kind")?;
        toml_text(row, "source")?;
    }
    Ok(())
}

fn audit_native_boundary(document: &Value) -> Result<(), String> {
    let native = table(document, "windows_native_boundary")?;
    if toml_text(native, "required_posture")? != "CertifiedExecutable"
        || toml_text(native, "other_platform_posture")? != "CompileOnlyOrNotYetCertifiedExecutable"
    {
        return Err("native platform certification posture drifted".to_owned());
    }
    require_exact_texts(
        native,
        "capabilities",
        &[
            "bind-window-to-exact-child-process",
            "observe-client-area-bounds-and-visibility",
            "capture-client-area-pixels-outside-product-and-egui",
            "request-normal-native-close",
            "distinguish-unsupported-platform-from-product-failure",
        ],
    )
}

fn audit_oracles(document: &Value) -> Result<(), String> {
    require_exact_ids(document, "independent_oracle", ORACLE_IDS)?;
    for row in toml_rows(document, "independent_oracle")? {
        toml_text(row, "owner")?;
        toml_text(row, "observes")?;
    }
    Ok(())
}

fn audit_hostile_sequence(document: &Value) -> Result<(), String> {
    let sequence = table(document, "hostile_sequence")?;
    let actual = toml_texts(sequence, "steps")?;
    if actual == HOSTILE_STEPS {
        Ok(())
    } else {
        Err(format!(
            "hostile executable-world sequence should be {HOSTILE_STEPS:?}; found {actual:?}"
        ))
    }
}

fn audit_mutations(document: &Value) -> Result<(), String> {
    require_exact_ids(document, "mutation_control", MUTATION_IDS)?;
    for (id, expected) in MUTATION_EVIDENCE {
        let mutation = row(document, "mutation_control", id)?;
        toml_text(mutation, "fault")?;
        let actual = toml_texts(mutation, "must_invalidate")?
            .into_iter()
            .collect::<BTreeSet<_>>();
        let expected = expected.iter().copied().collect::<BTreeSet<_>>();
        if actual != expected {
            return Err(format!(
                "mutation `{id}` should invalidate {expected:?}; found {actual:?}"
            ));
        }
    }
    Ok(())
}

fn audit_successor_extensions(document: &Value) -> Result<(), String> {
    let rows = toml_rows(document, "successor_extension")?;
    if rows.len() != 13 {
        return Err(format!(
            "Milestones 3.11 through 3.23 require thirteen successor homes; found {}",
            rows.len()
        ));
    }
    for ((expected_milestone, expected_home), extension) in SUCCESSOR_HOMES.iter().zip(rows.iter())
    {
        let actual_milestone = toml_text(extension, "milestone")?;
        let actual_home = toml_text(extension, "home")?;
        if actual_milestone != *expected_milestone || actual_home != *expected_home {
            return Err(format!(
                "successor `{expected_milestone}` should use `{expected_home}`; found `{actual_milestone}` at `{actual_home}`"
            ));
        }
        toml_text(extension, "inherits")?;
    }
    Ok(())
}

fn table<'a>(document: &'a Value, name: &str) -> Result<&'a Value, String> {
    document
        .get(name)
        .filter(|value| value.is_table())
        .ok_or_else(|| format!("Phase 1 inventory should contain `[{name}]`"))
}

fn row<'a>(document: &'a Value, family: &str, id: &str) -> Result<&'a Value, String> {
    toml_rows(document, family)?
        .iter()
        .find(|row| row.get("id").and_then(Value::as_str) == Some(id))
        .ok_or_else(|| format!("Phase 1 `{family}` row `{id}` is missing"))
}

fn require_exact_ids(document: &Value, family: &str, expected: &[&str]) -> Result<(), String> {
    let actual = toml_rows(document, family)?
        .iter()
        .map(|row| toml_text(row, "id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "Phase 1 `{family}` ids should be {expected:?}; found {actual:?}"
        ))
    }
}

fn require_exact_texts(value: &Value, field: &str, expected: &[&str]) -> Result<(), String> {
    let actual = toml_texts(value, field)?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "Phase 1 `{field}` should be {expected:?}; found {actual:?}"
        ))
    }
}
