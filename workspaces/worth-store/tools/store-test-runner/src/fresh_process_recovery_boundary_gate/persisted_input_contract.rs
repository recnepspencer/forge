use std::collections::BTreeSet;

mod causal_contract;
mod role_contract;
mod syntax_evidence;
#[cfg(test)]
mod tests;

use super::documents::{read_repository_document, split_csv, PERSISTED_INPUTS};
use super::repository_root;
use causal_contract::{SEMANTIC_CAUSAL_SOURCES, SEMANTIC_CAUSAL_SYNTAX};
use role_contract::REQUIRED_ROLES;
use syntax_evidence::{source_defines_surface, source_has_active_call_edges};

const HEADER: &str = "role,producer_type,admission_surface,schema_owner,producer_source,admission_source,posture,disposition,delivery_phase,causal_sources";
const DURABILITY_ROOT: &str =
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability";
const REQUIRED_GAPS: &[(&str, &str, &str)] = &[];
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
                .map(|source| causal_path(source))
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
        .map(|(_, source, _, _)| causal_path(source))
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
        let path = causal_path(source);
        source_has_active_call_edges(&path, function, identifiers)
            .map_err(|error| format!("{role} causal syntax is not bound: {error}"))?;
    }
    Ok(())
}

fn causal_path(source: &str) -> String {
    if source.starts_with("workspaces/") {
        source.to_owned()
    } else {
        format!("{DURABILITY_ROOT}/{source}")
    }
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
