use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, Seek, SeekFrom},
    process::{Command, Stdio},
    sync::OnceLock,
    time::{Duration, Instant},
};

use serde::Deserialize;
use worth_store::physical_runtime::{
    PhysicalWorkFeatureGraphEvidence, PhysicalWorkFeatureNodeEvidence,
    PhysicalWorkFilesystemProfileEvidence, PhysicalWorkPlatformEvidence, PhysicalWorkRerunEvidence,
    PhysicalWorkRunEnvironmentEvidence,
};
use worth_store_physical_backend::FilesystemBackendProfile;

pub(super) fn for_test(
    filesystem: &FilesystemBackendProfile,
    test_filter: &str,
) -> PhysicalWorkRunEnvironmentEvidence {
    PhysicalWorkRunEnvironmentEvidence::new(
        feature_graph(),
        PhysicalWorkPlatformEvidence::current(),
        PhysicalWorkFilesystemProfileEvidence::from_backend(filesystem).unwrap(),
        rerun(test_filter),
    )
}

fn rerun(test_filter: &str) -> PhysicalWorkRerunEvidence {
    if let Some(runner) = std::env::var_os("WORTH_STORE_C5_1_COURTROOM_A_RUNNER") {
        return PhysicalWorkRerunEvidence::new(
            runner.to_string_lossy().into_owned(),
            [
                "courtrooms".to_owned(),
                "--courtroom".to_owned(),
                "a".to_owned(),
                "--mutant-report".to_owned(),
                required_environment_path("WORTH_STORE_C5_1_MUTANT_REPORT"),
                "--report".to_owned(),
                required_environment_path("WORTH_STORE_C5_1_COURTROOM_A_REPORT"),
            ],
        )
        .unwrap();
    }
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    PhysicalWorkRerunEvidence::new(
        "cargo",
        [
            "test".to_owned(),
            "--manifest-path".to_owned(),
            manifest.display().to_string(),
            "--package".to_owned(),
            "worth-store".to_owned(),
            "--features".to_owned(),
            "certification-test-authority".to_owned(),
            "--test".to_owned(),
            "physical_record_journeys".to_owned(),
            test_filter.to_owned(),
            "--".to_owned(),
            "--exact".to_owned(),
        ],
    )
    .unwrap()
}

fn required_environment_path(variable: &str) -> String {
    std::env::var_os(variable)
        .unwrap_or_else(|| panic!("{variable} must accompany the Courtroom A runner binding"))
        .to_string_lossy()
        .into_owned()
}

fn feature_graph() -> PhysicalWorkFeatureGraphEvidence {
    static GRAPH: OnceLock<PhysicalWorkFeatureGraphEvidence> = OnceLock::new();
    GRAPH
        .get_or_init(|| {
            discover_feature_graph()
                .unwrap_or_else(|failure| panic!("cannot bind courtroom feature graph: {failure}"))
        })
        .clone()
}

#[derive(Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
    resolve: Option<CargoResolve>,
}

#[derive(Deserialize)]
struct CargoPackage {
    id: String,
    name: String,
    source: Option<String>,
}

#[derive(Deserialize)]
struct CargoResolve {
    nodes: Vec<CargoNode>,
}

#[derive(Deserialize)]
struct CargoNode {
    id: String,
    deps: Vec<CargoDependency>,
    features: Vec<String>,
}

#[derive(Deserialize)]
struct CargoDependency {
    pkg: String,
}

fn discover_feature_graph() -> Result<PhysicalWorkFeatureGraphEvidence, String> {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.args([
        "metadata",
        "--format-version",
        "1",
        "--locked",
        "--features",
        "certification-test-authority",
        "--manifest-path",
    ]);
    command.arg(manifest);
    let encoded = bounded_output(&mut command, Duration::from_secs(10))?;
    let metadata: CargoMetadata = serde_json::from_slice(&encoded)
        .map_err(|error| format!("cannot decode Cargo metadata: {error}"))?;
    let roots = metadata
        .packages
        .iter()
        .filter(|package| package.name == "worth-store" && package.source.is_none())
        .map(|package| package.id.clone())
        .collect::<Vec<_>>();
    let [root] = roots.as_slice() else {
        return Err("expected exactly one local worth-store metadata root".into());
    };
    let resolved = metadata
        .resolve
        .ok_or_else(|| "Cargo metadata omitted its resolved graph".to_owned())?;
    lower_feature_graph(root, resolved.nodes)
}

fn lower_feature_graph(
    root: &str,
    nodes: Vec<CargoNode>,
) -> Result<PhysicalWorkFeatureGraphEvidence, String> {
    let nodes = nodes
        .into_iter()
        .map(|node| (node.id.clone(), node))
        .collect::<BTreeMap<_, _>>();
    let mut pending = vec![root.to_owned()];
    let mut closure = BTreeSet::new();
    while let Some(package) = pending.pop() {
        if !closure.insert(package.clone()) {
            continue;
        }
        let node = nodes
            .get(&package)
            .ok_or_else(|| format!("resolved package `{package}` omitted its node"))?;
        pending.extend(node.deps.iter().map(|dependency| dependency.pkg.clone()));
    }
    let evidence = closure
        .iter()
        .map(|package| {
            let node = &nodes[package];
            PhysicalWorkFeatureNodeEvidence::new(
                package.clone(),
                node.features.clone(),
                node.deps
                    .iter()
                    .filter(|dependency| closure.contains(&dependency.pkg))
                    .map(|dependency| dependency.pkg.clone()),
            )
            .map_err(|denial| format!("feature node denied: {denial:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    PhysicalWorkFeatureGraphEvidence::new([root.to_owned()], evidence)
        .map_err(|denial| format!("feature graph denied: {denial:?}"))
}

fn bounded_output(command: &mut Command, timeout: Duration) -> Result<Vec<u8>, String> {
    let mut stdout = tempfile::tempfile()
        .map_err(|error| format!("cannot create Cargo metadata stdout capture: {error}"))?;
    let mut stderr = tempfile::tempfile()
        .map_err(|error| format!("cannot create Cargo metadata stderr capture: {error}"))?;
    command
        .stdout(Stdio::from(stdout.try_clone().map_err(|error| {
            format!("cannot clone metadata stdout: {error}")
        })?))
        .stderr(Stdio::from(stderr.try_clone().map_err(|error| {
            format!("cannot clone metadata stderr: {error}")
        })?));
    let mut child = command
        .spawn()
        .map_err(|error| format!("cannot spawn Cargo metadata: {error}"))?;
    let deadline = Instant::now() + timeout;
    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("cannot inspect Cargo metadata: {error}"))?
        {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!("Cargo metadata exceeded {}ms", timeout.as_millis()));
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let stdout = read_capture(&mut stdout)?;
    let stderr = read_capture(&mut stderr)?;
    if !status.success() {
        return Err(format!(
            "Cargo metadata exited with {status}\n{}",
            String::from_utf8_lossy(&stderr)
        ));
    }
    Ok(stdout)
}

fn read_capture(file: &mut std::fs::File) -> Result<Vec<u8>, String> {
    file.seek(SeekFrom::Start(0))
        .map_err(|error| format!("cannot rewind Cargo metadata capture: {error}"))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read Cargo metadata capture: {error}"))?;
    Ok(bytes)
}
