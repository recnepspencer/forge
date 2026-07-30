use super::{milestone_314_ledger, repository_document, workspace_source_inventory};

const CONTRACT: &str = "_docs/worth-ui/milestone-3.14-phase-2-contract.toml";
const LEDGER: &str = "_docs/worth-ui/milestone-3.14-phase-2-proof-ledger.csv";
const IDS: [&str; 7] = [
    "P2-01", "P2-02", "P2-03", "P2-04", "P2-05", "P2-06", "P2-07",
];

fn inputs() -> (toml::Value, String) {
    let contract =
        toml::from_str(&repository_document(CONTRACT)).expect("Phase 2 contract should parse");
    (contract, repository_document(LEDGER))
}

fn validate(contract: &toml::Value, ledger: &str) -> Result<(), String> {
    if contract["schema"].as_str() != Some("worth-ui.milestone-3.14.phase-2-contract.v1")
        || contract["milestone"].as_str() != Some("3.14")
        || contract["phase"].as_integer() != Some(2)
        || contract["ledger"].as_str() != Some(LEDGER)
    {
        return Err("Phase 2 contract identity drifted".to_owned());
    }
    let status = contract["status"]
        .as_str()
        .ok_or_else(|| "Phase 2 contract has no status".to_owned())?;
    if !matches!(status, "implementation" | "closed") {
        return Err("Phase 2 contract status is invalid".to_owned());
    }
    let gates = contract["phase_gate"]
        .as_array()
        .ok_or_else(|| "Phase 2 gates are missing".to_owned())?;
    let rows = milestone_314_ledger::parse_ledger(ledger)?;
    if gates.len() != IDS.len() || rows.len() != IDS.len() {
        return Err("Phase 2 gate count drifted".to_owned());
    }
    for (index, expected) in IDS.iter().enumerate() {
        if gates[index]["id"].as_str() != Some(expected) || rows[index][0] != *expected {
            return Err(format!("expected Phase 2 gate {expected}"));
        }
        let command = gates[index]["command"]
            .as_str()
            .ok_or_else(|| format!("{expected} has no command"))?;
        if rows[index].len() != 10 || rows[index][7] != command {
            return Err(format!("{expected} evidence command drifted"));
        }
        match rows[index][8].as_str() {
            "OPEN" if rows[index][9].is_empty() => {}
            "PROVED" if rows[index][9].len() >= 80 => {}
            _ => return Err(format!("{expected} status/evidence is dishonest")),
        }
    }
    let all_proved = rows.iter().all(|row| row[8] == "PROVED");
    if (status == "closed") != all_proved {
        return Err("Phase 2 contract status disagrees with its ledger".to_owned());
    }
    Ok(())
}

#[test]
fn milestone_314_phase2_contract_tracks_partial_closure_honestly() {
    let (contract, ledger) = inputs();
    validate(&contract, &ledger).expect("Phase 2 contract and ledger should agree");
    let rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");
    assert_eq!(rows[0][8], "PROVED");
    assert_eq!(rows[1][8], "PROVED");
    assert_eq!(rows[2][8], "PROVED");
    assert!(rows[3..].iter().all(|row| row[8] == "OPEN"));

    let mut premature = contract.clone();
    premature["status"] = toml::Value::String("closed".to_owned());
    assert!(validate(&premature, &ledger).is_err());
    assert_eq!(
        contract["known_broad_topology_debt"]["failures"]
            .as_array()
            .map(Vec::len),
        Some(7)
    );
}

#[test]
fn milestone_314_phase2_native_substrate_is_exact_and_nonduplicated() {
    let (contract, _) = inputs();
    let substrate = &contract["native_substrate"];
    assert_eq!(
        substrate["observation_schema_version"].as_integer(),
        Some(6)
    );
    assert_eq!(
        substrate["pointer_position_basis"].as_str(),
        Some("typed coordinate space and unit carried by every exact position")
    );
    let manifest: toml::Value =
        toml::from_str(&repository_document("workspaces/worth-ui/Cargo.toml"))
            .expect("workspace manifest should parse");
    assert_eq!(
        manifest["workspace"]["dependencies"]["egui"].as_str(),
        Some("=0.35.0")
    );
    assert_eq!(
        manifest["workspace"]["dependencies"]["eframe"]["version"].as_str(),
        Some("=0.35.0")
    );
    assert_eq!(
        manifest["workspace"]["dependencies"]["egui_extras"]["version"].as_str(),
        Some("=0.35.0")
    );

    let lock: toml::Value = toml::from_str(&repository_document("workspaces/worth-ui/Cargo.lock"))
        .expect("workspace lockfile should parse");
    for (package, field) in [
        ("eframe", "eframe_version"),
        ("egui", "egui_version"),
        ("egui-winit", "egui_winit_version"),
        ("egui-wgpu", "egui_wgpu_version"),
        ("egui_glow", "egui_glow_version"),
        ("egui_extras", "egui_extras_version"),
        ("winit", "winit_version"),
    ] {
        let matches = lock["package"]
            .as_array()
            .expect("lock packages")
            .iter()
            .filter(|entry| entry["name"].as_str() == Some(package))
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1, "{package} should resolve exactly once");
        assert_eq!(matches[0]["version"].as_str(), substrate[field].as_str());
    }
}

#[test]
fn milestone_314_phase2_preserves_phase1_history_while_cutting_over_live_input() {
    let historical: toml::Value = toml::from_str(&repository_document(
        "_docs/worth-ui/milestone-3.14-phase-1-contract.toml",
    ))
    .expect("Phase 1 contract should parse");
    assert_eq!(
        historical["native_reachability"]["egui_version"].as_str(),
        Some("0.31.1")
    );
    assert_eq!(
        historical["native_reachability"]["eframe_version"].as_str(),
        Some("0.31.1")
    );

    let inventory = workspace_source_inventory();
    let native_frame = inventory.text("apps/platform-pulse/src/native_frame.rs");
    assert!(native_frame.contains("fn ui(&mut self, ui: &mut egui::Ui"));
    assert!(native_frame.contains("fn raw_input_hook("));
    assert!(!native_frame.contains("fn update(&mut self, context: &egui::Context"));
    let reachability =
        inventory.text("crates/worth-ui-host-egui/src/adapter/input_observation/reachability.rs");
    assert!(reachability.contains("egui::ImeEvent::Preedit { text, .. }"));
    assert!(reachability.contains("text.is_empty()"));
    assert!(!reachability.contains("egui::ImeEvent::Disabled"));
}
