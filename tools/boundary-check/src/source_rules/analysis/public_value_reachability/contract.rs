//! Exact coverage contract between sealed definitions and downstream witnesses.

use super::public_value_exports::PublicValueExport;
use super::witness_source::WitnessSignature;
use crate::config::{
    PublicValueExemptionPosture, PublicValueReachabilityContract, PublicValueWitness,
};
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct WorldInventory<'a> {
    pub(super) graph: &'a super::super::crate_modules::ModuleGraph,
    pub(super) target: &'a str,
    pub(super) exports: &'a [PublicValueExport],
}

pub(super) fn validate(
    contract: &PublicValueReachabilityContract,
    worlds: &[WorldInventory<'_>],
    signatures: &BTreeMap<String, WitnessSignature>,
) -> Vec<Diagnostic> {
    let mut diagnostics = validate_contract_shape(contract);
    let exports = exported_union(worlds);
    validate_unambiguous_value_names(contract, &exports, &mut diagnostics);
    let witnesses = collect_witnesses(contract, &mut diagnostics);
    let exemptions = collect_exemptions(contract, &mut diagnostics);
    super::witness_rows::validate(
        super::witness_rows::Inputs {
            contract,
            worlds,
            exports: &exports,
            witnesses: &witnesses,
            signatures,
        },
        &mut diagnostics,
    );
    validate_exemptions(
        contract,
        worlds,
        &exports,
        &witnesses,
        &exemptions,
        &mut diagnostics,
    );
    validate_world_coverage(contract, worlds, &witnesses, &exemptions, &mut diagnostics);
    diagnostics
}

fn validate_unambiguous_value_names(
    contract: &PublicValueReachabilityContract,
    exports: &BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut paths_by_name = BTreeMap::<&str, Vec<&str>>::new();
    for path in exports {
        if let Some(name) = path.rsplit("::").next() {
            paths_by_name.entry(name).or_default().push(path);
        }
    }
    for (name, paths) in paths_by_name {
        if paths.len() > 1 {
            diagnostics.push(config_diagnostic(
                contract,
                format!(
                    "public value name `{name}` is ambiguous across exact definitions: {}",
                    paths.join(", ")
                ),
            ));
        }
    }
}

fn validate_exemptions(
    contract: &PublicValueReachabilityContract,
    worlds: &[WorldInventory<'_>],
    exports: &BTreeSet<String>,
    witnesses: &BTreeMap<String, &PublicValueWitness>,
    exemptions: &BTreeMap<String, &crate::config::PublicValueReachabilityExemption>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (path, exemption) in exemptions {
        if !exports.contains(path) {
            diagnostics.push(config_diagnostic(
                contract,
                format!("exemption `{path}` does not resolve to an exported public value type"),
            ));
            continue;
        }
        if witnesses.contains_key(path) {
            diagnostics.push(config_diagnostic(
                contract,
                format!("exemption `{path}` is stale because a witness is configured"),
            ));
        }
        match exemption.posture {
            PublicValueExemptionPosture::UninhabitedType => {
                for world in worlds {
                    let Some(export) = world
                        .exports
                        .iter()
                        .find(|export| display_key(&export.key) == *path)
                    else {
                        continue;
                    };
                    if !super::uninhabited_exemption::is_verified(world.graph, &export.key) {
                        diagnostics.push(config_diagnostic(
                            contract,
                            format!(
                                "exemption `{path}` is not a zero-variant enum in target `{}`",
                                world.target
                            ),
                        ));
                    }
                }
            }
        }
    }
}

fn validate_world_coverage(
    contract: &PublicValueReachabilityContract,
    worlds: &[WorldInventory<'_>],
    witnesses: &BTreeMap<String, &PublicValueWitness>,
    exemptions: &BTreeMap<String, &crate::config::PublicValueReachabilityExemption>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for world in worlds {
        for export in world.exports {
            let path = display_key(&export.key);
            let covered = witnesses
                .get(&path)
                .is_some_and(|witness| witness.worlds.iter().any(|name| name == world.target));
            if covered || exemptions.contains_key(&path) {
                continue;
            }
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Bc7004PublicValueReachability,
                export.relative_source.clone(),
                format!(
                    "exported public value `{path}` has no downstream witness for Cargo world `{}`; {}",
                    world.target, contract.guidance
                ),
            ));
        }
    }
}

fn validate_contract_shape(contract: &PublicValueReachabilityContract) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for (field, value) in [
        ("package", contract.package.as_str()),
        ("crate_root", contract.crate_root.as_str()),
        ("witness_source", contract.witness_source.as_str()),
        ("guidance", contract.guidance.as_str()),
    ] {
        if value.trim().is_empty() {
            diagnostics.push(config_diagnostic(
                contract,
                format!("public-value reachability `{field}` must not be blank"),
            ));
        }
    }
    if contract.worlds.is_empty() {
        diagnostics.push(config_diagnostic(
            contract,
            "public-value reachability requires at least one Cargo world",
        ));
    }
    if !(1..=300_000).contains(&contract.host_timeout_ms) {
        diagnostics.push(config_diagnostic(
            contract,
            "public-value reachability host_timeout_ms must be between 1 and 300000",
        ));
    }
    if !(1..=300_000).contains(&contract.compilation_timeout_ms) {
        diagnostics.push(config_diagnostic(
            contract,
            "public-value reachability compilation_timeout_ms must be between 1 and 300000",
        ));
    }
    if !(1..=67_108_864).contains(&contract.max_output_bytes) {
        diagnostics.push(config_diagnostic(
            contract,
            "public-value reachability max_output_bytes must be between 1 and 67108864",
        ));
    }
    diagnostics
}

fn collect_witnesses<'a>(
    contract: &'a PublicValueReachabilityContract,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, &'a PublicValueWitness> {
    let mut witnesses = BTreeMap::new();
    let mut public_paths = BTreeSet::new();
    for witness in &contract.witnesses {
        if witness.definition_path.trim().is_empty()
            || witness.public_type_path.trim().is_empty()
            || witness.function.trim().is_empty()
        {
            diagnostics.push(config_diagnostic(
                contract,
                "public-value witnesses require exact definition_path, public_type_path, and function",
            ));
            continue;
        }
        let canonical_public_path =
            super::super::type_alias_reachability::canonical_public_type_path(
                &witness.public_type_path,
            );
        if canonical_public_path.is_none() {
            diagnostics.push(config_diagnostic(
                contract,
                format!(
                    "public-value public type path `{}` must be an absolute `::worth_proof` type",
                    witness.public_type_path
                ),
            ));
        }
        if !public_paths
            .insert(canonical_public_path.unwrap_or_else(|| witness.public_type_path.clone()))
        {
            diagnostics.push(config_diagnostic(
                contract,
                format!(
                    "duplicate public-value public type path `{}`",
                    witness.public_type_path
                ),
            ));
        }
        if witnesses
            .insert(witness.definition_path.clone(), witness)
            .is_some()
        {
            diagnostics.push(config_diagnostic(
                contract,
                format!(
                    "duplicate public-value witness `{}`",
                    witness.definition_path
                ),
            ));
        }
    }
    witnesses
}

fn collect_exemptions<'a>(
    contract: &'a PublicValueReachabilityContract,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, &'a crate::config::PublicValueReachabilityExemption> {
    let mut exemptions = BTreeMap::new();
    for exemption in &contract.exemptions {
        if exemption.type_path.trim().is_empty() || exemption.reason.trim().is_empty() {
            diagnostics.push(config_diagnostic(
                contract,
                "public-value exemptions require exact type_path and non-empty reason",
            ));
            continue;
        }
        if exemptions
            .insert(exemption.type_path.clone(), exemption)
            .is_some()
        {
            diagnostics.push(config_diagnostic(
                contract,
                format!("duplicate public-value exemption `{}`", exemption.type_path),
            ));
        }
    }
    exemptions
}

fn exported_union(worlds: &[WorldInventory<'_>]) -> BTreeSet<String> {
    worlds
        .iter()
        .flat_map(|world| world.exports.iter())
        .map(|export| display_key(&export.key))
        .collect()
}

pub(super) fn config_diagnostic(
    contract: &PublicValueReachabilityContract,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::Bc7004PublicValueReachability,
        contract.crate_root.clone(),
        message,
    )
}

pub(super) fn display_key(key: &super::super::public_reachability::ReachableItemKey) -> String {
    key.module_path
        .iter()
        .chain(std::iter::once(&key.item_name))
        .cloned()
        .collect::<Vec<_>>()
        .join("::")
}
