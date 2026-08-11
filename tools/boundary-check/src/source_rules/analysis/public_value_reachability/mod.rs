mod contract;
mod expansion_policy;
mod public_value_exports;
mod uninhabited_exemption;
mod witness_rows;
mod witness_runner;
mod witness_source;

use super::{bounded_process, configured_crate, public_reachability};
use crate::config::PublicValueReachabilityContract;
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use std::path::Path;

pub(super) fn validate(root: &Path, contract: &PublicValueReachabilityContract) -> Vec<Diagnostic> {
    let governed = match configured_crate::load(root, &contract.crate_root, &contract.package) {
        Ok(governed) => governed,
        Err(error) => return vec![configuration_failure(contract, error)],
    };
    let compilation_limits = match compilation_limits(contract) {
        Ok(limits) => limits,
        Err(error) => return vec![configuration_failure(contract, error)],
    };
    let worlds = match super::production_world::ProductionWorld::load(
        &governed.crate_root,
        &contract.worlds,
        compilation_limits,
    ) {
        Ok(worlds) => worlds,
        Err(error) => return vec![configuration_failure(contract, error)],
    };
    let mut prepared = Vec::new();
    for world in worlds {
        let graph = match super::production_module_graph::parse(&governed, &world) {
            Ok(graph) => super::production_world::project(&graph, &world),
            Err(error) => return vec![configuration_failure(contract, error)],
        };
        if let Err(error) = expansion_policy::verify(&graph) {
            return vec![configuration_failure(contract, error)];
        }
        let reachable =
            match public_reachability::externally_reachable_items(&graph, &governed.crate_root) {
                Ok(reachable) => reachable,
                Err(error) => return vec![configuration_failure(contract, error)],
            };
        let public_values = public_value_exports::collect(&graph, &reachable);
        prepared.push((world, graph, public_values));
    }
    let (witness_source, signatures) = match witness_source::load(root, &contract.witness_source) {
        Ok(source) => source,
        Err(error) => return vec![configuration_failure(contract, error)],
    };
    let inventories = prepared
        .iter()
        .map(|(world, graph, exports)| contract::WorldInventory {
            graph,
            target: &world.name,
            exports,
        })
        .collect::<Vec<_>>();
    let mut diagnostics = contract::validate(contract, &inventories, &signatures);
    if diagnostics.is_empty() {
        for (world, _, _) in &prepared {
            let witnesses = contract
                .witnesses
                .iter()
                .filter(|witness| witness.worlds.iter().any(|name| name == &world.name))
                .collect::<Vec<_>>();
            if witnesses.is_empty() {
                continue;
            }
            if let Err(error) = witness_runner::run(
                root,
                &governed,
                &witness_source,
                &witnesses,
                world,
                contract.host_timeout_ms,
                compilation_limits,
            ) {
                diagnostics.push(configuration_failure(contract, error));
            }
        }
    }
    diagnostics.sort_by(Diagnostic::compare_code_subject_message);
    diagnostics.dedup_by(|left, right| left.has_same_code_subject_message(right));
    diagnostics
}

fn compilation_limits(
    contract: &PublicValueReachabilityContract,
) -> Result<bounded_process::Limits, String> {
    if !(1..=300_000).contains(&contract.compilation_timeout_ms) {
        return Err(
            "public-value reachability compilation_timeout_ms must be between 1 and 300000"
                .to_owned(),
        );
    }
    if !(1..=67_108_864).contains(&contract.max_output_bytes) {
        return Err(
            "public-value reachability max_output_bytes must be between 1 and 67108864".to_owned(),
        );
    }
    Ok(bounded_process::Limits::new(
        std::time::Duration::from_millis(contract.compilation_timeout_ms),
        contract.max_output_bytes,
    ))
}

fn configuration_failure(
    contract: &PublicValueReachabilityContract,
    error: impl std::fmt::Display,
) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::Bc7004PublicValueReachability,
        contract.crate_root.clone(),
        format!("public-value reachability configuration cannot be evaluated: {error}"),
    )
}
