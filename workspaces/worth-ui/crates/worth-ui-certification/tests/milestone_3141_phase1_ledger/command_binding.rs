const PACKAGES: [&str; 9] = [
    "worth-ui-host-contract",
    "worth-ui-host-egui",
    "worth-ui-runtime",
    "worth-ui-host-headless",
    "worth-ui-host-native",
    "worth-ui-native-platform",
    "worth-ui-platform-pulse",
    "worth-ui-certification",
    "worth-ui-text",
];

pub(super) struct CommandBinding {
    pub(super) shared_main: bool,
    pub(super) requirement: String,
    pub(super) package: String,
    pub(super) target_kind: String,
    pub(super) target_name: String,
    pub(super) features: Vec<String>,
    pub(super) test_name: String,
    pub(super) sources: Vec<String>,
    pub(super) artifact: String,
    pub(super) control: Option<ControlBinding>,
}

pub(super) struct ControlBinding {
    pub(super) package: String,
    pub(super) target_kind: String,
    pub(super) target_name: String,
    pub(super) features: Vec<String>,
    pub(super) test_name: String,
}

pub(super) struct CommandClaim<'a> {
    pub(super) command: &'a str,
    pub(super) requirement: &'a str,
    pub(super) production_entry: &'a str,
    pub(super) oracle_entry: &'a str,
    pub(super) source_identity: &'a str,
    pub(super) current_source: bool,
}

pub(super) fn validate(claim: CommandClaim<'_>) -> Result<CommandBinding, String> {
    let (production_source, _) = split_entry(claim.production_entry)?;
    let (oracle_source, oracle_symbol) = split_entry(claim.oracle_entry)?;
    let expected_sources = claim.source_identity.split(';').collect::<Vec<_>>();
    if !expected_sources.contains(&production_source) || !expected_sources.contains(&oracle_source)
    {
        return Err("evidence source identity omits a named entry".to_owned());
    }
    let binding = parse_words(&claim.command.split_whitespace().collect::<Vec<_>>())?;
    if binding.requirement != claim.requirement
        || binding.package != crate_name(oracle_source)?
        || binding.test_name.rsplit("::").next() != Some(oracle_symbol)
        || !binding.test_name.contains("::")
        || binding.sources != expected_sources
    {
        return Err("runner command is not bound to the oracle and sources".to_owned());
    }
    if super::execution_contract::control_for(claim.requirement).is_some()
        != binding.control.is_some()
        || binding
            .control
            .as_ref()
            .is_some_and(|control| !control.test_name.contains("::"))
    {
        return Err("governed command lacks one exact hostile control".to_owned());
    }
    validate_execution_identity(&binding, claim.current_source)?;
    Ok(binding)
}

fn validate_execution_identity(
    binding: &CommandBinding,
    current_source: bool,
) -> Result<(), String> {
    let expected_shared = super::execution_contract::is_shared_main(&binding.requirement);
    if binding.shared_main != expected_shared {
        return Err("ledger command has the wrong shared-world posture".to_owned());
    }
    let expected = (if current_source {
        super::execution_contract::current_predecessor_main_for(&binding.requirement)
    } else {
        super::execution_contract::main_for(&binding.requirement)
    })
    .ok_or_else(|| "requirement omits an execution contract".to_owned())?;
    if !matches_test(TestBinding::main(binding), expected) {
        return Err("ledger command swapped its exact main test".to_owned());
    }
    let expected_control = super::execution_contract::control_for(&binding.requirement);
    let observed_control = binding.control.as_ref();
    match (observed_control, expected_control) {
        (None, None) => Ok(()),
        (Some(observed), Some(expected))
            if matches_test(TestBinding::control(observed), expected) =>
        {
            Ok(())
        }
        _ => Err("ledger command swapped its exact hostile control".to_owned()),
    }
}

struct TestBinding<'a> {
    package: &'a str,
    target_kind: &'a str,
    target_name: &'a str,
    features: &'a [String],
    test_name: &'a str,
}

impl<'a> TestBinding<'a> {
    fn main(binding: &'a CommandBinding) -> Self {
        Self {
            package: &binding.package,
            target_kind: &binding.target_kind,
            target_name: &binding.target_name,
            features: &binding.features,
            test_name: &binding.test_name,
        }
    }

    fn control(binding: &'a ControlBinding) -> Self {
        Self {
            package: &binding.package,
            target_kind: &binding.target_kind,
            target_name: &binding.target_name,
            features: &binding.features,
            test_name: &binding.test_name,
        }
    }
}

fn matches_test(
    observed: TestBinding<'_>,
    expected: super::execution_contract::TestIdentity,
) -> bool {
    observed.package == expected.package
        && observed.target_kind == expected.target_kind
        && observed.target_name == expected.target_name
        && observed
            .features
            .iter()
            .map(String::as_str)
            .eq(expected.features.iter().copied())
        && observed.test_name == expected.test_name
}

fn parse_words(words: &[&str]) -> Result<CommandBinding, String> {
    let main = parse_main(words)?;
    let (control, mut cursor) = parse_control(words, main.cursor)?;
    if words.get(cursor) != Some(&"--requirement") {
        return Err("ledger runner lacks a requirement identity".to_owned());
    }
    let requirement = required(words, cursor + 1, "requirement identity")?;
    cursor += 2;
    let (sources, artifact) = parse_sources(words, cursor)?;
    Ok(CommandBinding {
        shared_main: main.shared,
        requirement: requirement.to_owned(),
        package: main.package,
        target_kind: main.target_kind,
        target_name: main.target_name,
        features: main.features,
        test_name: main.test_name,
        sources,
        artifact,
        control,
    })
}

struct ParsedMain {
    shared: bool,
    package: String,
    target_kind: String,
    target_name: String,
    features: Vec<String>,
    test_name: String,
    cursor: usize,
}

fn parse_main(words: &[&str]) -> Result<ParsedMain, String> {
    let shared_main = words.get(1) == Some(&"scripts/ci/run_worth_ui_shared_ledger_control.py");
    let approved_runner =
        shared_main || words.get(1) == Some(&"scripts/ci/run_worth_ui_ledger_test.py");
    if words.first() != Some(&"python")
        || !approved_runner
        || words.get(2..5)
            != Some(
                [
                    "--manifest-path",
                    "workspaces/worth-ui/Cargo.toml",
                    "--package",
                ]
                .as_slice(),
            )
    {
        return Err("unapproved ledger runner prefix".to_owned());
    }
    let package = required(words, 5, "package")?;
    if !PACKAGES.contains(&package) {
        return Err("ledger runner package is not governed".to_owned());
    }
    let (target_kind, target_name, mut cursor) = parse_target(words)?;
    let mut features = Vec::new();
    while words.get(cursor) == Some(&"--features") {
        features.push(required(words, cursor + 1, "feature")?.to_owned());
        cursor += 2;
    }
    if !features.is_empty()
        && (package != "worth-ui-platform-pulse" || features != ["executable-world"])
    {
        return Err("ledger runner feature set is not governed".to_owned());
    }
    if required(words, cursor, "test-name flag")? != "--test-name" {
        return Err("ledger runner lacks an exact test name".to_owned());
    }
    let test_name = required(words, cursor + 1, "test name")?;
    cursor += 2;
    Ok(ParsedMain {
        shared: shared_main,
        package: package.to_owned(),
        target_kind: target_kind.to_owned(),
        target_name: target_name.to_owned(),
        features,
        test_name: test_name.to_owned(),
        cursor,
    })
}

fn parse_sources(words: &[&str], mut cursor: usize) -> Result<(Vec<String>, String), String> {
    let mut sources = Vec::new();
    while words.get(cursor) == Some(&"--source") {
        sources.push(required(words, cursor + 1, "source identity")?.to_owned());
        cursor += 2;
    }
    if sources.is_empty() || words.get(cursor) != Some(&"--artifact") || words.len() != cursor + 2 {
        return Err("ledger runner needs sources and one terminal artifact".to_owned());
    }
    Ok((sources, words[cursor + 1].to_owned()))
}

fn parse_control(words: &[&str], cursor: usize) -> Result<(Option<ControlBinding>, usize), String> {
    if words.get(cursor) != Some(&"--control-package") {
        return Ok((None, cursor));
    }
    let package = required(words, cursor + 1, "control package")?;
    if !PACKAGES.contains(&package) {
        return Err("ledger control package is not governed".to_owned());
    }
    let (target_kind, target_name, next) = match words.get(cursor + 2) {
        Some(&"--control-lib") => ("lib", "lib", cursor + 3),
        Some(&"--control-test") => (
            "test",
            required(words, cursor + 3, "control target")?,
            cursor + 4,
        ),
        _ => return Err("ledger control must select one target".to_owned()),
    };
    let mut next = next;
    let mut features = Vec::new();
    while words.get(next) == Some(&"--control-features") {
        features.push(required(words, next + 1, "control feature")?.to_owned());
        next += 2;
    }
    if !features.is_empty()
        && (package != "worth-ui-platform-pulse" || features != ["executable-world"])
    {
        return Err("ledger control feature set is not governed".to_owned());
    }
    if words.get(next) != Some(&"--control-test-name") {
        return Err("ledger control lacks an exact test name".to_owned());
    }
    Ok((
        Some(ControlBinding {
            package: package.to_owned(),
            target_kind: target_kind.to_owned(),
            target_name: target_name.to_owned(),
            features,
            test_name: required(words, next + 1, "control test name")?.to_owned(),
        }),
        next + 2,
    ))
}

fn parse_target<'a>(words: &'a [&str]) -> Result<(&'a str, &'a str, usize), String> {
    match words.get(6) {
        Some(&"--lib") => Ok(("lib", "lib", 7)),
        Some(&"--test") => Ok(("test", required(words, 7, "test target")?, 8)),
        _ => Err("ledger runner must select one test target".to_owned()),
    }
}

fn required<'a>(words: &'a [&str], index: usize, name: &str) -> Result<&'a str, String> {
    words
        .get(index)
        .copied()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("ledger runner lacks {name}"))
}

fn split_entry(value: &str) -> Result<(&str, &str), String> {
    value
        .rsplit_once("::")
        .ok_or_else(|| "evidence entry lacks a named symbol".to_owned())
}

fn crate_name(source: &str) -> Result<&str, String> {
    if let Some(application) = source
        .strip_prefix("workspaces/worth-ui/apps/")
        .and_then(|tail| tail.split('/').next())
    {
        return match application {
            "platform-pulse" => Ok("worth-ui-platform-pulse"),
            _ => Err("oracle application is not a governed package".to_owned()),
        };
    }
    source
        .strip_prefix("workspaces/worth-ui/crates/")
        .and_then(|tail| tail.split('/').next())
        .ok_or_else(|| "oracle is not owned by a governed workspace crate".to_owned())
}

#[test]
fn phase_two_control_must_precede_the_requirement_binding() {
    let lawful = "python scripts/ci/run_worth_ui_ledger_test.py \
        --manifest-path workspaces/worth-ui/Cargo.toml --package worth-ui-host-native --lib \
        --test-name native::readiness::tests::lawful \
        --control-package worth-ui-host-native --control-lib \
        --control-test-name native::readiness::tests::hostile \
        --requirement P2-READINESS-01 --source source.rs --artifact artifact.json";
    assert!(parse_words(&lawful.split_whitespace().collect::<Vec<_>>()).is_ok());
    let wrong_order = "python scripts/ci/run_worth_ui_ledger_test.py \
        --manifest-path workspaces/worth-ui/Cargo.toml --package worth-ui-host-native --lib \
        --test-name native::readiness::tests::lawful --requirement P2-READINESS-01 \
        --control-package worth-ui-host-native --control-lib \
        --control-test-name native::readiness::tests::hostile \
        --source source.rs --artifact artifact.json";
    assert!(parse_words(&wrong_order.split_whitespace().collect::<Vec<_>>()).is_err());
}

#[test]
fn exact_execution_contract_rejects_main_control_and_feature_swaps() {
    let mut binding = CommandBinding {
        shared_main: true,
        requirement: "P2-READINESS-01".to_owned(),
        package: "worth-ui-platform-pulse".to_owned(),
        target_kind: "test".to_owned(),
        target_name: "executable_world".to_owned(),
        features: vec!["executable-world".to_owned()],
        test_name: "courtroom::native_phase2::windows_native_boundary_world_presents_quiesces_and_closes_without_residue".to_owned(),
        sources: vec!["source.rs".to_owned()],
        artifact: "artifact.json".to_owned(),
        control: Some(ControlBinding {
            package: "worth-ui-host-native".to_owned(),
            target_kind: "lib".to_owned(),
            target_name: "lib".to_owned(),
            features: Vec::new(),
            test_name: "native::readiness::tests::committed_readiness_requests_exactly_one_redraw_and_preserves_the_latest_generation".to_owned(),
        }),
    };
    validate_execution_identity(&binding, false).unwrap();
    binding.shared_main = false;
    assert!(validate_execution_identity(&binding, false).is_err());
    binding.shared_main = true;
    binding.control.as_mut().unwrap().test_name =
        "native::graphics::tests::window_basis_classifier_rearms_only_for_new_scale_or_nonzero_extent".to_owned();
    assert!(validate_execution_identity(&binding, false).is_err());
    binding.requirement = "P2-PIXELS-01".to_owned();
    binding.control = Some(ControlBinding {
        package: "worth-ui-platform-pulse".to_owned(),
        target_kind: "test".to_owned(),
        target_name: "executable_world".to_owned(),
        features: vec!["executable-world".to_owned()],
        test_name: "native_platform::windows::independent_window_capture_rejects_monitor_pixel_substitution".to_owned(),
    });
    validate_execution_identity(&binding, false).unwrap();
    binding.control.as_mut().unwrap().features.clear();
    assert!(validate_execution_identity(&binding, false).is_err());
    binding.test_name = "courtroom::unrelated::passing_test".to_owned();
    assert!(validate_execution_identity(&binding, false).is_err());
}
