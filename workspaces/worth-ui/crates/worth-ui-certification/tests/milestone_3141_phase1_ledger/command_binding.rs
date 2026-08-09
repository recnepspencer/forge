const PACKAGES: [&str; 6] = [
    "worth-ui-host-contract",
    "worth-ui-runtime",
    "worth-ui-host-headless",
    "worth-ui-host-native",
    "worth-ui-native-platform",
    "worth-ui-certification",
];

pub(super) struct CommandBinding {
    pub(super) requirement: String,
    pub(super) package: String,
    pub(super) target_kind: String,
    pub(super) target_name: String,
    pub(super) test_name: String,
    pub(super) sources: Vec<String>,
    pub(super) artifact: String,
}

pub(super) fn validate(
    command: &str,
    requirement: &str,
    production_entry: &str,
    oracle_entry: &str,
    source_identity: &str,
) -> Result<CommandBinding, String> {
    let (production_source, _) = split_entry(production_entry)?;
    let (oracle_source, oracle_symbol) = split_entry(oracle_entry)?;
    let expected_sources = source_identity.split(';').collect::<Vec<_>>();
    if !expected_sources.contains(&production_source) || !expected_sources.contains(&oracle_source)
    {
        return Err("evidence source identity omits a named entry".to_owned());
    }
    let binding = parse_words(&command.split_whitespace().collect::<Vec<_>>())?;
    if binding.requirement != requirement
        || binding.package != crate_name(oracle_source)?
        || binding.test_name.rsplit("::").next() != Some(oracle_symbol)
        || !binding.test_name.contains("::")
        || binding.sources != expected_sources
    {
        return Err("runner command is not bound to the oracle and sources".to_owned());
    }
    Ok(binding)
}

fn parse_words(words: &[&str]) -> Result<CommandBinding, String> {
    let fixed = [
        "python",
        "scripts/ci/run_worth_ui_ledger_test.py",
        "--manifest-path",
        "workspaces/worth-ui/Cargo.toml",
        "--package",
    ];
    if words.get(..fixed.len()) != Some(fixed.as_slice()) {
        return Err("unapproved ledger runner prefix".to_owned());
    }
    let package = required(words, 5, "package")?;
    if !PACKAGES.contains(&package) {
        return Err("ledger runner package is not governed".to_owned());
    }
    let (target_kind, target_name, mut cursor) = parse_target(words)?;
    if required(words, cursor, "test-name flag")? != "--test-name" {
        return Err("ledger runner lacks an exact test name".to_owned());
    }
    let test_name = required(words, cursor + 1, "test name")?;
    cursor += 2;
    if words.get(cursor) != Some(&"--requirement") {
        return Err("ledger runner lacks a requirement identity".to_owned());
    }
    let requirement = required(words, cursor + 1, "requirement identity")?;
    cursor += 2;
    let mut sources = Vec::new();
    while words.get(cursor) == Some(&"--source") {
        sources.push(required(words, cursor + 1, "source identity")?.to_owned());
        cursor += 2;
    }
    if sources.is_empty() || words.get(cursor) != Some(&"--artifact") || words.len() != cursor + 2 {
        return Err("ledger runner needs sources and one terminal artifact".to_owned());
    }
    Ok(CommandBinding {
        requirement: requirement.to_owned(),
        package: package.to_owned(),
        target_kind: target_kind.to_owned(),
        target_name: target_name.to_owned(),
        test_name: test_name.to_owned(),
        sources,
        artifact: words[cursor + 1].to_owned(),
    })
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
    source
        .strip_prefix("workspaces/worth-ui/crates/")
        .and_then(|tail| tail.split('/').next())
        .ok_or_else(|| "oracle is not owned by a governed workspace crate".to_owned())
}
