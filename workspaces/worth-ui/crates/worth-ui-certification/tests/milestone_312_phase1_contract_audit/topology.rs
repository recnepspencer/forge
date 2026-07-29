use std::collections::{BTreeMap, BTreeSet};

use worth_ui_certification::topology::WorkspaceSourceInventory;

pub(super) fn validate(
    contract: &toml::Value,
    inventory: &WorkspaceSourceInventory,
) -> Result<(), String> {
    validate_module_homes(contract)?;
    validate_future_insertions(contract)?;
    validate_dependency_denials(contract, inventory)?;
    validate_target_economy(contract, inventory)?;
    validate_model_purity(inventory)
}

fn validate_module_homes(contract: &toml::Value) -> Result<(), String> {
    let homes = contract["module_home"]
        .as_array()
        .ok_or_else(|| "module_home is not an array".to_owned())?;
    let actual = homes
        .iter()
        .map(|home| Ok((required_text(home, "owner")?, required_text(home, "path")?)))
        .collect::<Result<BTreeSet<_>, String>>()?;
    let expected = BTreeSet::from([
        ("independent certification", "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/observation_rebind"),
        ("observation authority", "workspaces/worth-ui/crates/worth-ui-runtime/src/runtime/observation"),
        ("produced and consumed fact contracts", "workspaces/worth-ui/crates/worth-ui-runtime/src/fact_contract"),
        ("product orchestration", "workspaces/worth-ui/apps/platform-pulse/src/bounded_rebind_pulse.rs"),
        ("rebind inspection projections", "workspaces/worth-ui/crates/worth-ui-inspection/src/receipt/rebind"),
        ("rebind public facade", "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/rebind"),
        ("rebind scope and planning", "workspaces/worth-ui/crates/worth-ui-runtime/src/runtime/rebind"),
        ("source attempt", "workspaces/worth-ui/crates/worth-ui-runtime/src/runtime/source_ingress/source_rebind_attempt"),
        ("visual comparison meaning", "workspaces/worth-ui/crates/worth-ui-inspection/src/snapshot/comparison"),
        ("visual comparison runtime", "workspaces/worth-ui/crates/worth-ui-runtime/src/inspection/visual_snapshot/comparison"),
    ]);
    if actual != expected {
        return Err("destination module-home set drifted".to_owned());
    }
    Ok(())
}

fn validate_future_insertions(contract: &toml::Value) -> Result<(), String> {
    let insertions = contract["future_insertion"]
        .as_array()
        .ok_or_else(|| "future_insertion is not an array".to_owned())?;
    let milestones = insertions
        .iter()
        .map(|entry| required_text(entry, "milestone"))
        .collect::<Result<Vec<_>, _>>()?;
    if milestones != ["3.13", "3.14", "3.15"] {
        return Err("future insertion milestones drifted".to_owned());
    }
    for insertion in insertions {
        required_text(insertion, "owner")?;
        required_text(insertion, "home")?;
        required_text(insertion, "forbidden_reopen")?;
    }
    Ok(())
}

fn validate_dependency_denials(
    contract: &toml::Value,
    inventory: &WorkspaceSourceInventory,
) -> Result<(), String> {
    let denials = contract["forbidden_dependency"]
        .as_array()
        .ok_or_else(|| "forbidden_dependency is not an array".to_owned())?;
    let actual = denials
        .iter()
        .map(|denial| {
            let owner = required_text(denial, "from")?;
            let forbidden = string_set(denial, "forbidden")?;
            Ok((owner, forbidden))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    let expected = BTreeMap::from([
        (
            "worth-ui-host-contract",
            BTreeSet::from(["worth-query", "worth-ui-inspection", "worth-ui-runtime"]),
        ),
        (
            "worth-ui-inspection",
            BTreeSet::from(["worth-query", "worth-ui-host-egui", "worth-ui-runtime"]),
        ),
        (
            "worth-ui-platform-pulse",
            BTreeSet::from([
                "worth-query",
                "worth-ui-certification",
                "worth-ui-dsl",
                "worth-ui-runtime",
                "worth-ui-test-support",
            ]),
        ),
        (
            "worth-ui-runtime",
            BTreeSet::from([
                "worth-query",
                "worth-query-replay",
                "worth-ui-certification",
                "worth-ui-test-support",
            ]),
        ),
    ]);
    if actual != expected {
        return Err("forbidden dependency contract drifted".to_owned());
    }
    for (owner, forbidden) in actual {
        let manifest = manifest_for(owner)?;
        assert_manifest_excludes(inventory.text(manifest), &forbidden, owner)?;
    }
    Ok(())
}

fn validate_target_economy(
    contract: &toml::Value,
    inventory: &WorkspaceSourceInventory,
) -> Result<(), String> {
    let targets = contract["existing_target"]
        .as_array()
        .ok_or_else(|| "existing_target is not an array".to_owned())?;
    let names = targets
        .iter()
        .map(|target| required_text(target, "name"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if names
        != BTreeSet::from([
            "application_contracts",
            "compile_contracts",
            "executable_world",
            "topology_contracts",
        ])
    {
        return Err("existing target inventory drifted".to_owned());
    }
    let pulse = inventory.text("apps/platform-pulse/Cargo.toml");
    if pulse.matches("[[bin]]").count() != 1 || pulse.matches("[[test]]").count() != 1 {
        return Err("Platform Pulse must retain one binary and one test target".to_owned());
    }
    let gates = contract["phase_gate"]
        .as_array()
        .ok_or_else(|| "phase_gate is not an array".to_owned())?;
    let ids = gates
        .iter()
        .map(|gate| required_text(gate, "id"))
        .collect::<Result<Vec<_>, _>>()?;
    let expected = (1..=11)
        .map(|number| format!("P1-{number:02}"))
        .collect::<Vec<_>>();
    if ids != expected.iter().map(String::as_str).collect::<Vec<_>>() {
        return Err("Phase 1 gate order drifted".to_owned());
    }
    let owners = gates
        .iter()
        .map(|gate| required_text(gate, "owner"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if owners.len() != gates.len() {
        return Err("Phase 1 gate owners are not unique".to_owned());
    }
    Ok(())
}

fn validate_model_purity(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let root = "crates/worth-ui-certification/tests/application_contracts/observation_rebind/model";
    let files = inventory.rust_files_under(root).collect::<Vec<_>>();
    if files.len() != 2 {
        return Err(format!(
            "independent model expected 2 leaf Rust files, found {}",
            files.len()
        ));
    }
    let roots = [
        inventory.text(
            "crates/worth-ui-certification/tests/application_contracts/observation_rebind.rs",
        ),
        inventory.text(
            "crates/worth-ui-certification/tests/application_contracts/observation_rebind/model.rs",
        ),
    ];
    for source in roots
        .into_iter()
        .chain(files.iter().map(|file| file.text()))
    {
        if source.contains("worth_ui") || source.contains("worth-ui") {
            return Err(format!(
                "independent model imports production semantics under {root}"
            ));
        }
    }
    Ok(())
}

fn manifest_for(owner: &str) -> Result<&'static str, String> {
    match owner {
        "worth-ui-host-contract" => Ok("crates/worth-ui-host-contract/Cargo.toml"),
        "worth-ui-inspection" => Ok("crates/worth-ui-inspection/Cargo.toml"),
        "worth-ui-platform-pulse" => Ok("apps/platform-pulse/Cargo.toml"),
        "worth-ui-runtime" => Ok("crates/worth-ui-runtime/Cargo.toml"),
        other => Err(format!("no governed manifest for {other}")),
    }
}

fn assert_manifest_excludes(
    manifest: &str,
    forbidden: &BTreeSet<&str>,
    owner: &str,
) -> Result<(), String> {
    for dependency in forbidden {
        if manifest.lines().any(|line| {
            let line = line.trim_start();
            line.starts_with(&format!("{dependency} "))
                || line.starts_with(&format!("{dependency}="))
        }) {
            return Err(format!("{owner} depends on forbidden {dependency}"));
        }
    }
    Ok(())
}

fn required_text<'a>(value: &'a toml::Value, field: &str) -> Result<&'a str, String> {
    value[field]
        .as_str()
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| format!("missing nonempty `{field}`"))
}

fn string_set<'a>(value: &'a toml::Value, field: &str) -> Result<BTreeSet<&'a str>, String> {
    value[field]
        .as_array()
        .ok_or_else(|| format!("`{field}` is not an array"))?
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .filter(|text| !text.trim().is_empty())
                .ok_or_else(|| format!("`{field}` contains an empty or non-string entry"))
        })
        .collect()
}
