use std::collections::BTreeSet;

mod syntax_evidence;

use super::documents::{read_repository_document, split_csv, PERSISTED_INPUTS};
use super::repository_root;
use syntax_evidence::{source_defines_surface, source_has_active_call_edges};

const HEADER: &str = "role,producer_type,admission_surface,schema_owner,producer_source,admission_source,posture,disposition,delivery_phase,causal_sources";
const REQUIRED_ROLES: &[&str] = &[
    "stable-store-identity",
    "current-root-selector",
    "previous-root-selector",
    "root-manifest",
    "checkpoint-source",
    "checkpoint-binding-compaction",
    "wal-segment-identity",
    "wal-frame-payload",
    "wal-prefix-range",
    "page-image-header",
    "page-lsn",
    "extent-manifest",
    "compaction-cutover-record",
    "wal-security-binding",
    "checkpoint-security-binding",
    "backend-durability-profile",
    "operation-attempt-binding-wal",
    "operation-terminal-fate",
    "operation-binding-compaction-state",
];
const DURABILITY_ROOT: &str =
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability";
const SEMANTIC_CAUSAL_SOURCES: &[(&str, &[&str])] = &[
    (
        "checkpoint-binding-compaction",
        &[
            "mutation/idempotency/runtime_owner.rs",
            "checkpoint/publication.rs",
            "checkpoint/reopen/binding_compaction.rs",
        ],
    ),
    (
        "operation-attempt-binding-wal",
        &[
            "mutation/idempotency/attempt_binding.rs",
            "wal/group_reservation/member_planning.rs",
            "wal/inventory/reopened_member.rs",
            "mutation/idempotency/persisted_binding.rs",
            "mutation/idempotency/persisted_binding/decoding.rs",
            "mutation/idempotency/bootstrap.rs",
        ],
    ),
    (
        "operation-terminal-fate",
        &[
            "mutation/idempotency/fate/persisted.rs",
            "mutation/idempotency/binding_compaction/encoding.rs",
            "mutation/idempotency/binding_compaction/decoding.rs",
            "mutation/idempotency/binding_compaction.rs",
            "mutation/idempotency/bootstrap.rs",
            "mutation/idempotency/runtime_owner.rs",
            "checkpoint/publication.rs",
            "checkpoint/reopen/binding_compaction.rs",
        ],
    ),
    (
        "operation-binding-compaction-state",
        &[
            "mutation/idempotency/binding_compaction/encoding.rs",
            "mutation/idempotency/binding_compaction/decoding.rs",
            "mutation/idempotency/persisted_binding/decoding.rs",
            "mutation/idempotency/fate/persisted.rs",
            "mutation/idempotency/binding_compaction.rs",
            "mutation/idempotency/bootstrap.rs",
            "mutation/idempotency/runtime_owner.rs",
            "checkpoint/publication.rs",
            "checkpoint/reopen/binding_compaction.rs",
        ],
    ),
];
#[rustfmt::skip]
const SEMANTIC_CAUSAL_SYNTAX: &[(&str, &str, &str, &[&str])] = &[
    ("checkpoint-binding-compaction", "mutation/idempotency/runtime_owner.rs", "PhysicalMutationBindingCompactionCutover::for_each_record", &["method:self.pending_ref", "method:self.pending_ref().for_each_record"]),
    ("checkpoint-binding-compaction", "checkpoint/publication.rs", "CreatedCheckpointCandidate::finish", &["method:binding_compaction.for_each_record", "method:encoder.encode_binding_record", "method:self.work.execute"]),
    ("checkpoint-binding-compaction", "checkpoint/reopen/binding_compaction.rs", "NamespaceDurablePhysicalBindingCompactionReopen::stream_records", &["method:tree.read_exact_at", "path:decode_checkpoint_binding_record", "callback:consume"]),
    ("operation-attempt-binding-wal", "mutation/idempotency/attempt_binding.rs", "PhysicalMutationAttemptBinding::encode_persisted", &["path:write_field"]),
    ("operation-attempt-binding-wal", "wal/group_reservation/member_planning.rs", "encode_member_payload", &["method:binding.encode_persisted", "path:write_field"]),
    ("operation-attempt-binding-wal", "wal/inventory/reopened_member.rs", "ReopenedPhysicalWalMember::decode_retained_frame", &["path:take_field"]),
    ("operation-attempt-binding-wal", "mutation/idempotency/persisted_binding.rs", "PersistedPhysicalMutationAttemptBinding::from_allocated", &["method:persisted.encode"]),
    ("operation-attempt-binding-wal", "mutation/idempotency/persisted_binding/decoding.rs", "PersistedPhysicalMutationAttemptBinding::decode_from_wal_member", &["path:Self::decode"]),
    ("operation-attempt-binding-wal", "mutation/idempotency/bootstrap.rs", "PhysicalIdempotencyRegistryRebuilder::consume_wal_member", &["method:member.persisted_binding", "path:super::PersistedPhysicalMutationAttemptBinding::decode_from_wal_member"]),
    ("operation-terminal-fate", "mutation/idempotency/fate/persisted.rs", "PersistedPhysicalMutationFate::encode", &["path:write_field"]),
    ("operation-terminal-fate", "mutation/idempotency/fate/persisted.rs", "PersistedPhysicalMutationFate::decode", &["path:Self::decode_completed", "path:Self::decode_indeterminate"]),
    ("operation-terminal-fate", "mutation/idempotency/binding_compaction/encoding.rs", "encode_terminal", &["path:encode_basis", "method:fate.encode"]),
    ("operation-terminal-fate", "mutation/idempotency/binding_compaction/decoding.rs", "DecodedPhysicalMutationBindingRecord::decode", &["path:PersistedPhysicalMutationFate::decode"]),
    ("operation-terminal-fate", "mutation/idempotency/binding_compaction.rs", "encode_retained_record", &["path:encode_terminal"]),
    ("operation-terminal-fate", "mutation/idempotency/bootstrap.rs", "PhysicalIdempotencyRegistryRebuilder::consume_compaction_record", &["path:DecodedPhysicalMutationBindingRecord::decode"]),
    ("operation-terminal-fate", "mutation/idempotency/runtime_owner.rs", "PhysicalMutationBindingCompactionCutover::for_each_record", &["method:self.pending_ref", "method:self.pending_ref().for_each_record"]),
    ("operation-terminal-fate", "checkpoint/publication.rs", "CreatedCheckpointCandidate::finish", &["method:binding_compaction.for_each_record", "method:encoder.encode_binding_record", "method:self.work.execute"]),
    ("operation-terminal-fate", "checkpoint/reopen/binding_compaction.rs", "NamespaceDurablePhysicalBindingCompactionReopen::stream_records", &["method:tree.read_exact_at", "path:decode_checkpoint_binding_record", "callback:consume"]),
    ("operation-binding-compaction-state", "mutation/idempotency/binding_compaction/encoding.rs", "encode_unsealed", &["path:encode_basis"]),
    ("operation-binding-compaction-state", "mutation/idempotency/binding_compaction/encoding.rs", "encode_group_sealed", &["path:encode_basis", "path:write_field"]),
    ("operation-binding-compaction-state", "mutation/idempotency/binding_compaction/encoding.rs", "encode_wal_bound", &["path:write_field"]),
    ("operation-binding-compaction-state", "mutation/idempotency/binding_compaction/decoding.rs", "DecodedPhysicalMutationBindingRecord::decode", &["path:decode_basis", "path:PersistedPhysicalMutationAttemptBinding::decode_from_compaction", "path:require_canonical"]),
    ("operation-binding-compaction-state", "mutation/idempotency/binding_compaction/decoding.rs", "decode_basis", &["path:decode_binding_basis"]),
    ("operation-binding-compaction-state", "mutation/idempotency/persisted_binding/decoding.rs", "PersistedPhysicalMutationAttemptBinding::decode_from_compaction", &["path:Self::decode"]),
    ("operation-binding-compaction-state", "mutation/idempotency/fate/persisted.rs", "PersistedPhysicalMutationFate::decode", &["path:Self::decode_completed", "path:Self::decode_indeterminate"]),
    ("operation-binding-compaction-state", "mutation/idempotency/binding_compaction.rs", "encode_retained_record", &["path:encode_unsealed", "path:encode_group_sealed", "path:encode_wal_bound"]),
    ("operation-binding-compaction-state", "mutation/idempotency/bootstrap.rs", "PhysicalIdempotencyRegistryRebuilder::consume_compaction_record", &["path:DecodedPhysicalMutationBindingRecord::decode"]),
    ("operation-binding-compaction-state", "mutation/idempotency/runtime_owner.rs", "PhysicalMutationBindingCompactionCutover::for_each_record", &["method:self.pending_ref", "method:self.pending_ref().for_each_record"]),
    ("operation-binding-compaction-state", "checkpoint/publication.rs", "CreatedCheckpointCandidate::finish", &["method:binding_compaction.for_each_record", "method:encoder.encode_binding_record", "method:self.work.execute"]),
    ("operation-binding-compaction-state", "checkpoint/reopen/binding_compaction.rs", "NamespaceDurablePhysicalBindingCompactionReopen::stream_records", &["method:tree.read_exact_at", "path:decode_checkpoint_binding_record", "callback:consume"]),
];
const REQUIRED_GAPS: &[(&str, &str, &str)] = &[
    (
        "previous-root-selector",
        "worth-store-physical-format/root-selector",
        "phase-2",
    ),
    (
        "compaction-cutover-record",
        "worth-store-physical-format/compaction-cutover-record",
        "phase-3",
    ),
    (
        "wal-security-binding",
        "worth-store-wal/security-binding",
        "phase-4",
    ),
    (
        "checkpoint-security-binding",
        "worth-store-physical-format/checkpoint-security-binding",
        "phase-4",
    ),
];
const FORBIDDEN_PROXIES: &[&str] = &[
    "Store",
    "ServingPhysicalRuntime",
    "PhysicalDurabilityRecoveryHandoff",
    "BufferPoolHandle",
    "SignalGraph",
    "Scheduler",
    "DecodedArtifactCollection",
    "ExpectedRecordModel",
    "PriorRuntimeIdentity",
    "CompactionCutoverRecoveryPosture",
    "AdmittedCompactionCutoverRecord",
];

#[test]
fn persisted_input_roles_bind_real_producers_or_explicit_gaps() {
    let document = read_repository_document(PERSISTED_INPUTS).expect("read C.8 persisted inputs");
    let rows = parse_rows(&document).expect("parse C.8 persisted inputs");
    validate_rows(&rows).expect("validate C.8 persisted inputs");
    for row in rows
        .iter()
        .filter(|row| row.posture != "required-producer-gap")
    {
        source_defines_surface(&row.producer_source, &row.producer_type)
            .expect("bind persisted producer type");
        source_defines_surface(&row.admission_source, &row.admission_surface)
            .expect("bind persisted decoder or admission surface");
    }
}

#[test]
fn omitted_foreign_and_derived_proxy_mutants_are_rejected() {
    let document = read_repository_document(PERSISTED_INPUTS).expect("read C.8 persisted inputs");
    let rows = parse_rows(&document).expect("parse C.8 persisted inputs");
    assert!(validate_rows(&rows[1..]).is_err());

    let mut foreign = rows.clone();
    foreign[0].producer_type = "Store".into();
    assert!(validate_rows(&foreign).is_err());

    let mut derived = rows.clone();
    let gap = derived
        .iter_mut()
        .find(|row| row.role == "compaction-cutover-record")
        .unwrap();
    gap.producer_type = "CompactionCutoverRecoveryPosture".into();
    gap.admission_surface = "CompactionCutoverRecoveryPosture::admit_visible_product".into();
    gap.producer_source = "workspaces/worth-store/crates/worth-store-recovery-physics/src/source_precedence/compaction_visibility/artifact_residue.rs".into();
    gap.admission_source = gap.producer_source.clone();
    gap.posture = "decoded-from-persisted-bytes".into();
    gap.disposition = "preserve".into();
    assert!(validate_rows(&derived).is_err());

    let mut shallow = rows.clone();
    shallow
        .iter_mut()
        .find(|row| row.role == "operation-terminal-fate")
        .unwrap()
        .causal_sources = format!("{DURABILITY_ROOT}/mutation/idempotency/fate/persisted.rs");
    assert!(validate_rows(&shallow).is_err());

    let mut unframed = rows.clone();
    let attempt = unframed
        .iter_mut()
        .find(|row| row.role == "operation-attempt-binding-wal")
        .unwrap();
    attempt.causal_sources = attempt.causal_sources.replace(
        ";workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/group_reservation/member_planning.rs",
        "",
    );
    assert!(validate_rows(&unframed).is_err());

    let mut unpublished = rows.clone();
    let checkpoint = unpublished
        .iter_mut()
        .find(|row| row.role == "checkpoint-binding-compaction")
        .unwrap();
    checkpoint.causal_sources = checkpoint.causal_sources.replace(
        ";workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/publication.rs",
        "",
    );
    assert!(validate_rows(&unpublished).is_err());
}

fn validate_rows(rows: &[PersistedRow]) -> Result<(), String> {
    let roles = rows
        .iter()
        .map(|row| row.role.as_str())
        .collect::<BTreeSet<_>>();
    let required = REQUIRED_ROLES.iter().copied().collect::<BTreeSet<_>>();
    if roles != required || rows.len() != required.len() {
        return Err("C.8 persisted role set is incomplete or duplicated".into());
    }
    for row in rows {
        if FORBIDDEN_PROXIES.contains(&row.producer_type.as_str()) {
            return Err(format!("{} substitutes a live or derived proxy", row.role));
        }
        let gap = REQUIRED_GAPS.iter().find(|(role, _, _)| *role == row.role);
        if let Some((_, owner, phase)) = gap {
            if row.producer_type != "none"
                || row.admission_surface != "none"
                || row.schema_owner != *owner
                || row.producer_source != "none"
                || row.admission_source != "none"
                || row.posture != "required-producer-gap"
                || row.disposition != "create"
                || row.delivery_phase != *phase
            {
                return Err(format!("{} must remain an explicit producer gap", row.role));
            }
        } else if row.producer_type == "none"
            || row.admission_surface == "none"
            || row.producer_source == "none"
            || row.admission_source == "none"
            || row.posture == "required-producer-gap"
            || row.disposition != "preserve"
        {
            return Err(format!(
                "{} lacks a real persisted producer/admission pair",
                row.role
            ));
        }
        validate_causal_sources(row)?;
    }
    Ok(())
}

fn validate_causal_sources(row: &PersistedRow) -> Result<(), String> {
    let expected = SEMANTIC_CAUSAL_SOURCES
        .iter()
        .find(|(role, _)| *role == row.role)
        .map(|(_, sources)| {
            sources
                .iter()
                .map(|source| format!("{DURABILITY_ROOT}/{source}"))
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_else(|| BTreeSet::from(["none".to_owned()]));
    let actual = row
        .causal_sources
        .split(';')
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!("{} has an incomplete causal codec chain", row.role));
    }
    let syntax_sources = SEMANTIC_CAUSAL_SYNTAX
        .iter()
        .filter(|(role, _, _, _)| *role == row.role)
        .map(|(_, source, _, _)| format!("{DURABILITY_ROOT}/{source}"))
        .collect::<BTreeSet<_>>();
    if syntax_sources
        != actual
            .iter()
            .filter(|source| *source != "none")
            .cloned()
            .collect()
    {
        return Err(format!(
            "{} has an incomplete causal syntax contract",
            row.role
        ));
    }
    for source in actual.iter().filter(|source| source.as_str() != "none") {
        if !repository_root().join(source).is_file() {
            return Err(format!("{} binds missing causal source {source}", row.role));
        }
    }
    for (role, source, function, identifiers) in SEMANTIC_CAUSAL_SYNTAX
        .iter()
        .filter(|(role, _, _, _)| *role == row.role)
    {
        let path = format!("{DURABILITY_ROOT}/{source}");
        source_has_active_call_edges(&path, function, identifiers)
            .map_err(|error| format!("{role} causal syntax is not bound: {error}"))?;
    }
    Ok(())
}

fn parse_rows(document: &str) -> Result<Vec<PersistedRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.8 persisted-input inventory has an invalid schema".into());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let columns = split_csv(line, 10)?;
            Ok(PersistedRow {
                role: columns[0].into(),
                producer_type: columns[1].into(),
                admission_surface: columns[2].into(),
                schema_owner: columns[3].into(),
                producer_source: columns[4].into(),
                admission_source: columns[5].into(),
                posture: columns[6].into(),
                disposition: columns[7].into(),
                delivery_phase: columns[8].into(),
                causal_sources: columns[9].into(),
            })
        })
        .collect()
}

#[derive(Clone)]
struct PersistedRow {
    role: String,
    producer_type: String,
    admission_surface: String,
    schema_owner: String,
    producer_source: String,
    admission_source: String,
    posture: String,
    disposition: String,
    delivery_phase: String,
    causal_sources: String,
}
