mod doctest_parser;
mod module_reachability;
mod rust_test_parser;
mod source_inventory;
mod test_identity;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::cargo_surface::{normalized, CargoSurface, TestTargetIdentity};
use doctest_parser::{declared_doctest_features, declared_doctests, DoctestKind};
use module_reachability::reachable_target_files;
use rust_test_parser::{
    external_tools, launches_child_process, launches_nested_cargo, rust_test_functions,
    uses_standardized_ui_harness,
};
use source_inventory::{candidate_source_paths, is_ui_fixture, read_source_snapshot, rust_sources};
use test_identity::{
    assigned_target, explicit_stable_identity, responsibility_for, stable_identity,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CaseKind {
    RustTest,
    UiFixture,
    DoctestSurface,
    DoctestRunnable,
    DoctestCompileFail,
    DoctestIgnored,
    TestExecutable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum CompilerBoundaryHarness {
    StandardizedCargoUi,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestCaseIdentity {
    pub stable_id: String,
    pub package: String,
    pub responsibility: String,
    pub case_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseSurface {
    pub identity: TestCaseIdentity,
    pub kind: CaseKind,
    pub source_path: String,
    pub source_line: usize,
    pub target_identity: Option<String>,
    #[serde(default)]
    pub registration_alias_targets: Vec<String>,
    pub current_invocation: String,
    #[serde(default)]
    pub registration_authority: String,
    pub ignored: bool,
    #[serde(default)]
    pub runs_by_default: bool,
    #[serde(default)]
    pub process_model: String,
    #[serde(default)]
    pub external_tools: Vec<String>,
    #[serde(default)]
    pub compiler_boundary_harness: Option<CompilerBoundaryHarness>,
    pub launches_child_process: bool,
    pub launches_nested_cargo: bool,
    pub assertion_predicates: Vec<String>,
    #[serde(default)]
    pub behavior_fingerprint: String,
    #[serde(default)]
    pub required_features: Vec<String>,
}

pub(crate) fn discover_test_cases(surface: &CargoSurface) -> Result<Vec<TestCaseSurface>, String> {
    let package_sources: BTreeMap<_, _> = surface
        .packages
        .iter()
        .map(|package| {
            Ok((
                package.name.clone(),
                rust_sources(Path::new(&package.package_root))?,
            ))
        })
        .collect::<Result<_, String>>()?;
    let candidate_sources =
        candidate_source_paths(Path::new(&surface.workspace_root), &package_sources)?;
    let mut source_texts = BTreeMap::new();
    let mut target_files = BTreeMap::<String, BTreeSet<String>>::new();
    for target in &surface.targets {
        target_files.insert(
            target.identity.clone(),
            reachable_target_files(target, &mut source_texts)?,
        );
    }
    let missing_candidates = candidate_sources
        .iter()
        .filter(|path| !source_texts.contains_key(normalized(path).as_str()))
        .cloned()
        .collect();
    source_texts.extend(read_source_snapshot(&missing_candidates)?);
    let target_text = target_registration_text(&target_files, &source_texts)?;
    let mut cases = Vec::new();
    for package in &surface.packages {
        let package_root = Path::new(&package.package_root);
        for source in package_sources
            .get(&package.name)
            .expect("source snapshot covers every package")
            .iter()
            .filter(|source| candidate_sources.contains(*source))
        {
            let source_path = normalized(&source);
            let text = source_texts
                .get(&source_path)
                .ok_or_else(|| format!("source snapshot omitted {}", source.display()))?;
            let direct_assignment =
                assigned_target(&package.name, &source_path, &surface.targets, &target_files);
            let ui_registrations = if direct_assignment.is_none() && is_ui_fixture(&source) {
                ui_registration_targets(&package.name, &source, &surface.targets, &target_text)?
            } else {
                Vec::new()
            };
            let ui_assignment = ui_registrations.first().copied();
            let registration_alias_targets = ui_registrations
                .iter()
                .skip(1)
                .map(|target| target.identity.clone())
                .collect::<Vec<_>>();
            let assignment = direct_assignment.or(ui_assignment);
            let registration_authority = if direct_assignment.is_some() {
                "cargo-target-module-reachability"
            } else if ui_assignment.is_some() {
                "ui-runner-explicit-fixture-reference"
            } else {
                "unregistered"
            };
            let tests = rust_test_functions(&text);
            let tests_were_empty = tests.is_empty();
            for test in tests {
                let execution_text = test.execution_source.as_str();
                let child_process = launches_child_process(execution_text);
                let nested_cargo = launches_nested_cargo(execution_text);
                let compiler_boundary_harness = uses_standardized_ui_harness(execution_text)
                    .then_some(CompilerBoundaryHarness::StandardizedCargoUi);
                let responsibility = responsibility_for(package_root, &source, &test.name);
                let identity = explicit_stable_identity(&text, &test.name)?.unwrap_or_else(|| {
                    stable_identity(&package.name, &responsibility, &source, &test.name)
                });
                cases.push(TestCaseSurface {
                    identity,
                    kind: CaseKind::RustTest,
                    source_path: source_path.clone(),
                    source_line: test.line,
                    target_identity: assignment.map(|target| target.identity.clone()),
                    registration_alias_targets: registration_alias_targets.clone(),
                    current_invocation: assignment.map_or_else(
                        || "unregistered".to_owned(),
                        |target| format!("cargo test -p {} --target {}", package.name, target.name),
                    ),
                    registration_authority: registration_authority.to_owned(),
                    ignored: test.ignored,
                    runs_by_default: !test.ignored
                        && assignment.is_some_and(|target| target.required_features.is_empty()),
                    process_model: process_model(
                        child_process,
                        nested_cargo,
                        compiler_boundary_harness,
                    )
                    .to_owned(),
                    external_tools: external_tools(execution_text),
                    compiler_boundary_harness,
                    launches_child_process: child_process,
                    launches_nested_cargo: nested_cargo,
                    assertion_predicates: test.assertion_predicates,
                    behavior_fingerprint: test.behavior_fingerprint,
                    required_features: assignment
                        .map(|target| target.required_features.clone())
                        .unwrap_or_default(),
                });
            }
            if is_ui_fixture(&source) && tests_were_empty && direct_assignment.is_none() {
                let execution_text = assignment
                    .and_then(|target| target_text.get(&target.identity))
                    .map(String::as_str)
                    .unwrap_or(&text);
                let child_process = launches_child_process(execution_text);
                let nested_cargo = launches_nested_cargo(execution_text);
                let compiler_boundary_harness = uses_standardized_ui_harness(execution_text)
                    .then_some(CompilerBoundaryHarness::StandardizedCargoUi);
                let responsibility = responsibility_for(package_root, &source, "compile_denial");
                let case_name = source
                    .file_stem()
                    .and_then(|value| value.to_str())
                    .unwrap_or("unnamed_ui_fixture");
                cases.push(TestCaseSurface {
                    identity: stable_identity(&package.name, &responsibility, &source, case_name),
                    kind: CaseKind::UiFixture,
                    source_path: source_path.clone(),
                    source_line: 1,
                    target_identity: assignment.map(|target| target.identity.clone()),
                    registration_alias_targets: registration_alias_targets.clone(),
                    current_invocation: "cargo store-ui".to_owned(),
                    registration_authority: registration_authority.to_owned(),
                    ignored: false,
                    runs_by_default: assignment
                        .is_some_and(|target| target.required_features.is_empty()),
                    process_model: process_model(
                        child_process,
                        nested_cargo,
                        compiler_boundary_harness,
                    )
                    .to_owned(),
                    external_tools: external_tools(execution_text),
                    compiler_boundary_harness,
                    launches_child_process: child_process,
                    launches_nested_cargo: nested_cargo,
                    assertion_predicates: vec!["compiler_denial".to_owned()],
                    behavior_fingerprint: source_fingerprint(&text),
                    required_features: assignment
                        .map(|target| target.required_features.clone())
                        .unwrap_or_default(),
                });
            }
            let doctest_features = declared_doctest_features(&text);
            for doctest in declared_doctests(&source, &text)? {
                let Some(doc_target) = surface.targets.iter().find(|target| {
                    target.package == package.name
                        && target.kinds.iter().any(|kind| kind == "doc")
                        && target.required_features == doctest_features
                }) else {
                    return Err(format!(
                        "doctest source has no rustdoc execution target: {source_path}"
                    ));
                };
                let responsibility =
                    responsibility_for(package_root, &source, &doctest.stable_case_name);
                let ignored = doctest.kind == DoctestKind::Ignored;
                cases.push(TestCaseSurface {
                    identity: stable_identity(
                        &package.name,
                        &responsibility,
                        &source,
                        &doctest.stable_case_name,
                    ),
                    kind: match doctest.kind {
                        DoctestKind::Runnable => CaseKind::DoctestRunnable,
                        DoctestKind::CompileFail => CaseKind::DoctestCompileFail,
                        DoctestKind::Ignored => CaseKind::DoctestIgnored,
                    },
                    source_path: source_path.clone(),
                    source_line: doctest.source_line,
                    target_identity: Some(doc_target.identity.clone()),
                    registration_alias_targets: Vec::new(),
                    current_invocation: format!("cargo test -p {} --doc", package.name),
                    registration_authority: "rustdoc-fenced-code-block".to_owned(),
                    ignored,
                    runs_by_default: !ignored && doc_target.required_features.is_empty(),
                    process_model: "rustdoc-test-process".to_owned(),
                    external_tools: vec!["rustdoc".to_owned()],
                    compiler_boundary_harness: None,
                    launches_child_process: true,
                    launches_nested_cargo: false,
                    assertion_predicates: vec![match doctest.kind {
                        DoctestKind::Runnable => "documentation_example_executes".to_owned(),
                        DoctestKind::CompileFail => "compiler_denial".to_owned(),
                        DoctestKind::Ignored => "documentation_example_only".to_owned(),
                    }],
                    behavior_fingerprint: doctest.stable_case_name.clone(),
                    required_features: doctest_features.clone(),
                });
            }
        }
    }
    cases.sort_by(|left, right| left.identity.cmp(&right.identity));
    Ok(cases)
}

fn source_fingerprint(source: &str) -> String {
    use sha2::{Digest, Sha256};

    format!("sha256:{:x}", Sha256::digest(source.as_bytes()))
}

fn process_model(
    child_process: bool,
    nested_cargo: bool,
    compiler_boundary_harness: Option<CompilerBoundaryHarness>,
) -> &'static str {
    if compiler_boundary_harness.is_some() {
        "standardized-ui-harness"
    } else if nested_cargo {
        "nested-cargo-process"
    } else if child_process {
        "fresh-child-process"
    } else {
        "in-process-libtest"
    }
}

fn target_registration_text(
    target_files: &BTreeMap<String, BTreeSet<String>>,
    source_texts: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>, String> {
    target_files
        .iter()
        .map(|(target, files)| {
            let mut text = String::new();
            for file in files {
                text.push_str(
                    source_texts
                        .get(file)
                        .ok_or_else(|| format!("source snapshot omitted target source {file}"))?,
                );
                text.push('\n');
            }
            Ok((target.clone(), text))
        })
        .collect()
}

fn ui_registration_targets<'a>(
    package: &str,
    fixture: &Path,
    targets: &'a [TestTargetIdentity],
    target_text: &BTreeMap<String, String>,
) -> Result<Vec<&'a TestTargetIdentity>, String> {
    let fixture_name = fixture
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("UI fixture has no UTF-8 file name: {}", fixture.display()))?;
    let mut registered: Vec<_> = targets
        .iter()
        .filter(|target| target.package == package)
        .filter(|target| {
            target_text
                .get(&target.identity)
                .is_some_and(|source| source.contains(fixture_name))
        })
        .collect();
    let fixture_responsibility = fixture
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    registered.sort_by(|left, right| {
        let left_is_owner = left.name.contains(fixture_responsibility);
        let right_is_owner = right.name.contains(fixture_responsibility);
        right_is_owner
            .cmp(&left_is_owner)
            .then_with(|| left.identity.cmp(&right.identity))
    });
    Ok(registered)
}
