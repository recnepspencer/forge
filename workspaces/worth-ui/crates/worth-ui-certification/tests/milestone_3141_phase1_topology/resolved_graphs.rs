use std::collections::BTreeSet;
use std::process::Command;

const NATIVE_DEPS: &[&str] = &[
    "pollster",
    "rustybuzz",
    "sha2",
    "swash",
    "toml",
    "wgpu",
    "winit",
    "winsafe",
    "worth_proof",
    "worth_signal",
    "worth_ui_host_contract",
    "worth_ui_retained_order",
];

#[test]
fn default_all_feature_and_windows_resolved_graphs_are_exact_and_mutation_sensitive() {
    let modes = [
        &[][..],
        &["--all-features"][..],
        &["--filter-platform", "x86_64-pc-windows-msvc"][..],
    ];
    let graphs = modes.map(metadata);
    for graph in &graphs {
        validate(graph).expect("resolved graph matches the frozen topology");
    }

    let mut widened = graphs[0].clone();
    node_mut(&mut widened, "wgpu", "29.0.4")["features"]
        .as_array_mut()
        .unwrap()
        .push("vulkan".into());
    assert!(validate(&widened).is_err());

    let mut hidden_edge = graphs[0].clone();
    node_mut(&mut hidden_edge, "worth-ui-host-native", "0.1.0")["deps"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(validate(&hidden_edge).is_err());
    let features = strings(&node(&graphs[0], "wgpu", "29.0.4").unwrap()["features"]).unwrap();
    let qualified_backends = ["dx12", "vulkan", "gles", "metal", "webgpu"]
        .into_iter()
        .filter(|backend| features.contains(backend))
        .count();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P1-BACKEND-FEATURES-01\":{qualified_backends}}}");
}

fn validate(graph: &serde_json::Value) -> Result<(), String> {
    exact_features(
        graph,
        "wgpu",
        "29.0.4",
        &["dx12", "parking_lot", "std", "wgsl"],
    )?;
    exact_features(graph, "winit", "0.30.13", &["rwh_06"])?;
    exact_dependencies(graph, "worth-ui-host-native", "0.1.0", NATIVE_DEPS)?;
    exact_dependencies(
        graph,
        "worth-ui-native-platform",
        "0.1.0",
        &["worth_ui_runtime"],
    )?;
    exact_dependencies(
        graph,
        "worth-ui-host-headless",
        "0.1.0",
        &[
            "worth_ui_host_contract",
            "worth_ui_retained_order",
            "worth_ui_test_support",
        ],
    )
}

fn exact_features(
    graph: &serde_json::Value,
    package: &str,
    version: &str,
    expected: &[&str],
) -> Result<(), String> {
    let node = node(graph, package, version)?;
    let observed = strings(&node["features"])?;
    equality(package, observed, expected)
}

fn exact_dependencies(
    graph: &serde_json::Value,
    package: &str,
    version: &str,
    expected: &[&str],
) -> Result<(), String> {
    let node = node(graph, package, version)?;
    let observed = node["deps"]
        .as_array()
        .ok_or_else(|| format!("{package} deps are absent"))?
        .iter()
        .filter_map(|dependency| dependency["name"].as_str())
        .collect::<BTreeSet<_>>();
    equality(package, observed, expected)
}

fn equality<'a>(
    package: &str,
    observed: BTreeSet<&'a str>,
    expected: &[&'a str],
) -> Result<(), String> {
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    (observed == expected)
        .then_some(())
        .ok_or_else(|| format!("{package} resolved posture drifted: {observed:?} != {expected:?}"))
}

fn strings(value: &serde_json::Value) -> Result<BTreeSet<&str>, String> {
    value
        .as_array()
        .ok_or_else(|| "resolved features are absent".to_owned())?
        .iter()
        .map(|feature| {
            feature
                .as_str()
                .ok_or_else(|| "non-string feature".to_owned())
        })
        .collect()
}

fn node<'a>(
    graph: &'a serde_json::Value,
    package: &str,
    version: &str,
) -> Result<&'a serde_json::Value, String> {
    let identity = package_identity(graph, package, version)?;
    graph["resolve"]["nodes"]
        .as_array()
        .and_then(|nodes| {
            nodes
                .iter()
                .find(|node| node["id"].as_str() == Some(identity))
        })
        .ok_or_else(|| format!("missing resolved node {package} {version}"))
}

fn node_mut<'a>(
    graph: &'a mut serde_json::Value,
    package: &str,
    version: &str,
) -> &'a mut serde_json::Value {
    let identity = package_identity(graph, package, version)
        .unwrap()
        .to_owned();
    graph["resolve"]["nodes"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|node| node["id"].as_str() == Some(&identity))
        .unwrap()
}

fn package_identity<'a>(
    graph: &'a serde_json::Value,
    package: &str,
    version: &str,
) -> Result<&'a str, String> {
    graph["packages"]
        .as_array()
        .and_then(|packages| {
            packages.iter().find(|candidate| {
                candidate["name"].as_str() == Some(package)
                    && candidate["version"].as_str() == Some(version)
            })
        })
        .and_then(|package| package["id"].as_str())
        .ok_or_else(|| format!("missing package {package} {version}"))
}

fn metadata(extra: &[&str]) -> serde_json::Value {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--manifest-path"])
        .arg("workspaces/worth-ui/Cargo.toml")
        .args(extra)
        .current_dir(repository_root())
        .output()
        .expect("cargo metadata executes");
    assert!(output.status.success(), "resolved graph failed: {extra:?}");
    serde_json::from_slice(&output.stdout).expect("metadata is JSON")
}

fn repository_root() -> std::path::PathBuf {
    super::workspace_source_inventory()
        .root()
        .parent()
        .and_then(std::path::Path::parent)
        .expect("repository root")
        .to_owned()
}
