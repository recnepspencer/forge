use std::collections::BTreeSet;
use std::path::Path;

use sha2::{Digest, Sha256};

use super::super::documents::{
    API_INVENTORY, AUTHORITY_TRACE, CARGO_GRAPH, CUTOVER_INVENTORY, DESTINATION_TOPOLOGY,
    PERSISTED_INPUTS, QA_AUDITS, QA_SOURCE_MANIFESTS, SPECIFICATION,
};
use super::super::repository_root;

const ROADMAP: &str = "_docs/worth-store/physical-foundation-reconstruction-roadmap.md";
const GATE_ROOT: &str =
    "workspaces/worth-store/tools/store-test-runner/src/fresh_process_recovery_boundary_gate";
const TRYBUILD: &[&str] = &[
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/c8_recovery_handoff_constructor_is_sealed.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/c8_recovery_handoff_constructor_is_sealed.stderr",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/c8_recovery_handoff_is_linear.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/c8_recovery_handoff_is_linear.stderr",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/c8_recovery_handoff_rejects_report_conversion.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/c8_recovery_handoff_rejects_report_conversion.stderr",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/c8_recovery_operation_fact_cannot_mint_handoff.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/c8_recovery_operation_fact_cannot_mint_handoff.stderr",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/closeout/handoff.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/closeout/operation_fates/fact.rs",
];

pub(super) fn phase_one_source_identity(guarantee: &str) -> Result<String, String> {
    let root = repository_root();
    let paths = phase_one_source_paths(guarantee)?;
    let mut digest = Sha256::new();
    for relative in paths {
        let path = root.join(&relative);
        digest.update(relative.as_bytes());
        digest.update([0]);
        digest.update(
            std::fs::read(&path)
                .map_err(|error| format!("cannot hash {}: {error}", path.display()))?,
        );
        digest.update([0xff]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub(super) fn phase_one_source_paths(guarantee: &str) -> Result<BTreeSet<String>, String> {
    let root = repository_root();
    let mut paths = BTreeSet::from([SPECIFICATION.to_owned()]);
    match guarantee {
        "C8-P1-TRUTH-01" => add_truth_sources(&root, &mut paths)?,
        "C8-P1-ENTRY-01" => add_entry_sources(&mut paths),
        "C8-P1-PERSISTED-01" => add_persisted_sources(&root, &mut paths)?,
        "C8-P1-API-01" => add_api_sources(&root, &mut paths)?,
        "C8-P1-AUTHORITY-01" => add_authority_sources(&mut paths),
        "C8-P1-SESSION-01" => add_session_sources(&mut paths),
        "C8-P1-EFFECT-01" => add_effect_sources(&mut paths),
        "C8-P1-FRESHNESS-01" => add_freshness_sources(&mut paths),
        "C8-P1-PROTOCOL-01" => add_protocol_sources(&mut paths),
        "C8-P1-TOPOLOGY-01" => add_topology_sources(&mut paths),
        "C8-P1-DEPENDENCY-01" => add_dependency_sources(&mut paths),
        "C8-P1-CUTOVER-01" | "C8-P1-CLEANUP-01" => add_cutover_sources(&root, &mut paths)?,
        "C8-P1-COMPILE-01" => TRYBUILD.iter().for_each(|path| add(&mut paths, path)),
        "C8-P1-DOCUMENTATION-01" => add_documentation_sources(&mut paths),
        "C8-P1-LEDGER-01" | "C8-P1-LEDGER-02" => add_ledger_sources(guarantee, &mut paths),
        other => return Err(format!("unknown C.8 guarantee source closure {other}")),
    }
    Ok(paths)
}

fn add_truth_sources(root: &Path, paths: &mut BTreeSet<String>) -> Result<(), String> {
    add(paths, PERSISTED_INPUTS);
    add(paths, AUTHORITY_TRACE);
    add(paths, &format!("{GATE_ROOT}/persisted_input_contract.rs"));
    add(
        paths,
        &format!("{GATE_ROOT}/persisted_input_contract/syntax_evidence.rs"),
    );
    add(
        paths,
        &format!("{GATE_ROOT}/persisted_input_contract/syntax_evidence/expression_activity.rs"),
    );
    add(
        paths,
        &format!("{GATE_ROOT}/persisted_input_contract/syntax_evidence/tests.rs"),
    );
    add(paths, &format!("{GATE_ROOT}/authority_trace.rs"));
    collect_persisted_producers(root, paths)
}

fn add_entry_sources(paths: &mut BTreeSet<String>) {
    add_authority_contract(paths);
}

fn add_persisted_sources(root: &Path, paths: &mut BTreeSet<String>) -> Result<(), String> {
    add(paths, PERSISTED_INPUTS);
    add(paths, &format!("{GATE_ROOT}/persisted_input_contract.rs"));
    add(
        paths,
        &format!("{GATE_ROOT}/persisted_input_contract/syntax_evidence.rs"),
    );
    add(
        paths,
        &format!("{GATE_ROOT}/persisted_input_contract/syntax_evidence/expression_activity.rs"),
    );
    add(
        paths,
        &format!("{GATE_ROOT}/persisted_input_contract/syntax_evidence/tests.rs"),
    );
    collect_persisted_producers(root, paths)
}

fn add_api_sources(root: &Path, paths: &mut BTreeSet<String>) -> Result<(), String> {
    add(paths, API_INVENTORY);
    add(
        paths,
        "workspaces/worth-store/tools/store-test-runner/Cargo.toml",
    );
    for source in [
        "facade_inventory.rs",
        "facade_inventory/bounded_decode_surface_contract.rs",
        "facade_inventory/cross_file_surface_contract.rs",
        "facade_inventory/delivered_api.rs",
        "facade_inventory/delivered_api/cfg_reachability.rs",
        "facade_inventory/delivered_api/exactness.rs",
        "facade_inventory/delivered_api/export_resolution.rs",
        "facade_inventory/delivered_api/external_resolution.rs",
        "facade_inventory/delivered_api/facade_exports.rs",
        "facade_inventory/delivered_api/namespace_exports.rs",
        "facade_inventory/delivered_api/pre_c8_surface.rs",
        "facade_inventory/delivered_api/source_layout.rs",
        "facade_inventory/delivered_api/tests.rs",
        "facade_inventory/delivered_api/tests/namespace_tests.rs",
        "facade_inventory/destination_surface_contract.rs",
        "facade_inventory/disposition_contract.rs",
        "facade_inventory/reachable_api.rs",
        "facade_inventory/runtime_phase_three_surface_contract.rs",
        "facade_inventory/runtime_phase_four_surface_contract.rs",
        "facade_inventory/runtime_phase_four_plan_surface_contract.rs",
        "facade_inventory/runtime_phase_four_projection_surface_contract.rs",
        "facade_inventory/runtime_phase_five_surface_contract.rs",
        "facade_inventory/runtime_phase_six_surface_contract.rs",
        "facade_inventory/supporting_delivery_surface_contract.rs",
    ] {
        add(paths, &format!("{GATE_ROOT}/{source}"));
    }
    collect_api_sources(root, paths)
}

fn add_authority_sources(paths: &mut BTreeSet<String>) {
    add_authority_contract(paths);
    add_substrate_files(
        paths,
        &[
            "crates/worth-proof/src/binding/mod.rs",
            "crates/worth-proof/src/binding/axes.rs",
            "crates/worth-proof/src/binding/authoring.rs",
            "crates/worth-proof/src/proof/markers.rs",
            "crates/worth-proof/src/proof/marker_authoring.rs",
            "crates/worth-proof/src/proof/witnesses.rs",
        ],
    );
}

fn add_session_sources(paths: &mut BTreeSet<String>) {
    add_authority_contract(paths);
    add_substrate_files(paths, &["crates/worth-proof/src/linear.rs"]);
}

fn add_effect_sources(paths: &mut BTreeSet<String>) {
    add_authority_contract(paths);
    add_topology_sources(paths);
    add_substrate_files(
        paths,
        &[
            "crates/worth-proof/src/effect/mod.rs",
            "crates/worth-proof/src/effect/causality.rs",
            "crates/worth-proof/src/effect/performed.rs",
        ],
    );
}

fn add_freshness_sources(paths: &mut BTreeSet<String>) {
    add_authority_contract(paths);
    add(paths, DESTINATION_TOPOLOGY);
    add(paths, &format!("{GATE_ROOT}/destination_topology.rs"));
    add(
        paths,
        &format!("{GATE_ROOT}/destination_topology/required_destinations.rs"),
    );
    add(
        paths,
        &format!("{GATE_ROOT}/destination_topology/semantic_contract.rs"),
    );
    add(
        paths,
        &format!("{GATE_ROOT}/destination_topology/semantic_contract/responsibility.rs"),
    );
    add(
        paths,
        &format!("{GATE_ROOT}/destination_topology/semantic_tests.rs"),
    );
    add_substrate_files(
        paths,
        &[
            "crates/worth-proof/src/assumption/mod.rs",
            "crates/worth-proof/src/assumption/basis.rs",
            "crates/worth-proof/src/assumption/freshness.rs",
            "crates/worth-proof/src/assumption/source.rs",
        ],
    );
}

fn add_protocol_sources(paths: &mut BTreeSet<String>) {
    add_authority_contract(paths);
    add_substrate_files(
        paths,
        &[
            "crates/worth-foundational/src/facade.rs",
            "crates/worth-foundational/src/boundary_protocol/mod.rs",
            "crates/worth-foundational/src/boundary_protocol/identity.rs",
            "crates/worth-foundational/src/boundary_protocol/version.rs",
            "crates/worth-foundational/src/boundary_protocol/compatibility_window.rs",
            "crates/worth-foundational/src/boundary_protocol/unsupported_version.rs",
        ],
    );
}

fn add_authority_contract(paths: &mut BTreeSet<String>) {
    add(paths, AUTHORITY_TRACE);
    add(paths, &format!("{GATE_ROOT}/authority_trace.rs"));
}

fn add_substrate_files(paths: &mut BTreeSet<String>, sources: &[&str]) {
    for source in sources {
        add(paths, source);
    }
}

fn add_topology_sources(paths: &mut BTreeSet<String>) {
    add(paths, DESTINATION_TOPOLOGY);
    add(paths, AUTHORITY_TRACE);
    add_gate_source(paths, "destination_topology.rs");
    add_gate_source(paths, "destination_topology/required_destinations.rs");
    add_gate_source(paths, "destination_topology/semantic_contract.rs");
    add_gate_source(
        paths,
        "destination_topology/semantic_contract/responsibility.rs",
    );
    add_gate_source(paths, "destination_topology/semantic_tests.rs");
}

fn add_dependency_sources(paths: &mut BTreeSet<String>) {
    add(paths, CARGO_GRAPH);
    add(paths, &format!("{GATE_ROOT}/cargo_graph.rs"));
    add(paths, "workspaces/worth-store/Cargo.toml");
    add(
        paths,
        "workspaces/worth-store/crates/worth-store-recovery-physics/Cargo.toml",
    );
}

fn add_cutover_sources(root: &Path, paths: &mut BTreeSet<String>) -> Result<(), String> {
    add(paths, CUTOVER_INVENTORY);
    add(paths, API_INVENTORY);
    add(paths, &format!("{GATE_ROOT}/cutover_inventory.rs"));
    add(
        paths,
        &format!("{GATE_ROOT}/cutover_inventory/import_reconciliation.rs"),
    );
    collect_cutover_sources(root, paths)
}

fn add_documentation_sources(paths: &mut BTreeSet<String>) {
    add(paths, ROADMAP);
    add(paths, &format!("{GATE_ROOT}/documents.rs"));
}

fn add_ledger_sources(guarantee: &str, paths: &mut BTreeSet<String>) {
    add_gate_source(paths, "closure_ledger.rs");
    add_gate_source(paths, "closure_ledger/history_contract.rs");
    add_gate_source(paths, "closure_ledger/history_contract/audit_contracts.rs");
    add_gate_source(
        paths,
        "closure_ledger/history_contract/finding_inventory.rs",
    );
    add_gate_source(paths, "closure_ledger/audit_source_manifest.rs");
    add_gate_source(paths, "closure_ledger/audit_source_manifest/tests.rs");
    add(paths, QA_AUDITS);
    add(paths, QA_SOURCE_MANIFESTS);
    if guarantee == "C8-P1-LEDGER-01" {
        return;
    }
    add_gate_source(paths, "closure_ledger/source_identity.rs");
    for artifact in [
        API_INVENTORY,
        AUTHORITY_TRACE,
        CARGO_GRAPH,
        CUTOVER_INVENTORY,
        DESTINATION_TOPOLOGY,
        PERSISTED_INPUTS,
    ] {
        add(paths, artifact);
    }
}
fn add(paths: &mut BTreeSet<String>, path: &str) {
    paths.insert(path.replace('\\', "/"));
}

fn add_gate_source(paths: &mut BTreeSet<String>, relative: &str) {
    add(paths, &format!("{GATE_ROOT}/{relative}"));
}

fn collect_persisted_producers(root: &Path, paths: &mut BTreeSet<String>) -> Result<(), String> {
    let document = std::fs::read_to_string(root.join(PERSISTED_INPUTS))
        .map_err(|error| format!("cannot read C.8 persisted inputs: {error}"))?;
    for line in document
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
    {
        let columns = line.split(',').collect::<Vec<_>>();
        for source_set in [columns.get(4), columns.get(5), columns.get(9)]
            .into_iter()
            .flatten()
        {
            for source in source_set.split(';').filter(|source| *source != "none") {
                add(paths, source);
            }
        }
    }
    Ok(())
}

fn collect_api_sources(root: &Path, paths: &mut BTreeSet<String>) -> Result<(), String> {
    let relative_root = "workspaces/worth-store/crates/worth-store-recovery-physics/src";
    collect_rust_sources(root, relative_root, paths)
}

fn collect_rust_sources(
    root: &Path,
    relative_root: &str,
    paths: &mut BTreeSet<String>,
) -> Result<(), String> {
    let mut pending = vec![root.join(relative_root)];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot enumerate {}: {error}", directory.display()))?
        {
            let path = entry
                .map_err(|error| format!("cannot enumerate {}: {error}", directory.display()))?
                .path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let relative = path
                    .strip_prefix(root)
                    .map_err(|error| format!("API source escaped repository root: {error}"))?;
                add(paths, &relative.to_string_lossy());
            }
        }
    }
    Ok(())
}

fn collect_cutover_sources(root: &Path, paths: &mut BTreeSet<String>) -> Result<(), String> {
    let document = std::fs::read_to_string(root.join(CUTOVER_INVENTORY))
        .map_err(|error| format!("cannot read C.8 cutover inventory: {error}"))?;
    for line in document
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
    {
        let path = line
            .split(',')
            .next()
            .ok_or_else(|| "cutover row has no path".to_owned())?;
        if path.starts_with("_docs/") {
            add(paths, path);
        } else {
            add(paths, &format!("workspaces/worth-store/{path}"));
        }
    }
    Ok(())
}
