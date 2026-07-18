pub(super) struct RustTestFunction {
    pub(super) name: String,
    pub(super) line: usize,
    pub(super) ignored: bool,
    pub(super) assertion_predicates: Vec<String>,
    pub(super) behavior_fingerprint: String,
}

pub(super) fn rust_test_functions(source: &str) -> Vec<RustTestFunction> {
    let lines: Vec<_> = source.lines().collect();
    let mut tests = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if !line.trim().starts_with("#[test]") {
            continue;
        }
        let mut ignored = false;
        for candidate in lines.iter().skip(index + 1).take(6) {
            let trimmed = candidate.trim();
            ignored |= trimmed.starts_with("#[ignore");
            if let Some(name) = function_name(trimmed) {
                tests.push(RustTestFunction {
                    name,
                    line: index + 1,
                    ignored,
                    assertion_predicates: assertion_predicates(&lines, index + 1),
                    behavior_fingerprint: behavior_fingerprint(&lines, index + 1),
                });
                break;
            }
        }
    }
    tests
}

fn behavior_fingerprint(lines: &[&str], function_start: usize) -> String {
    use sha2::{Digest, Sha256};

    let mut depth = 0_i32;
    let mut entered_body = false;
    let mut canonical = String::new();
    for line in lines.iter().skip(function_start) {
        depth += line.matches('{').count() as i32;
        entered_body |= depth > 0;
        canonical.extend(line.split_whitespace());
        depth -= line.matches('}').count() as i32;
        if entered_body && depth <= 0 {
            break;
        }
    }
    format!("sha256:{:x}", Sha256::digest(canonical.as_bytes()))
}

fn assertion_predicates(lines: &[&str], function_start: usize) -> Vec<String> {
    let mut depth = 0_i32;
    let mut entered_body = false;
    let mut predicates = Vec::new();
    for line in lines.iter().skip(function_start) {
        depth += line.matches('{').count() as i32;
        entered_body |= depth > 0;
        let trimmed = line.trim();
        if trimmed.contains("assert")
            || trimmed.contains("expect(")
            || trimmed.contains("unwrap_err(")
            || trimmed.contains("is_err(")
        {
            predicates.push(trimmed.split_whitespace().collect::<Vec<_>>().join(" "));
        }
        depth -= line.matches('}').count() as i32;
        if entered_body && depth <= 0 {
            break;
        }
    }
    if predicates.is_empty() {
        predicates.push("behavioral_completion".to_owned());
    }
    predicates
}

fn function_name(line: &str) -> Option<String> {
    let after_fn = line.split_once("fn ")?.1;
    let name: String = after_fn
        .chars()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

pub(super) fn launches_child_process(source: &str) -> bool {
    source.contains("Command::new") || source.contains("std::process::Command")
}

pub(super) fn launches_nested_cargo(source: &str) -> bool {
    launches_child_process(source)
        && (source.contains("Command::new(\"cargo\")")
            || source.contains("Command::new(cargo")
            || source.contains("var_os(\"CARGO\")")
            || source.contains("var(\"CARGO\")"))
}

pub(super) fn external_tools(source: &str) -> Vec<String> {
    let mut tools = Vec::new();
    for tool in ["cargo", "java", "rustc", "powershell", "pwsh", "bash"] {
        let direct = format!("Command::new(\"{tool}\")");
        let qualified = format!("std::process::Command::new(\"{tool}\")");
        if source.contains(&direct) || source.contains(&qualified) {
            tools.push(tool.to_owned());
        }
    }
    if !tools.iter().any(|tool| tool == "cargo")
        && (source.contains("var_os(\"CARGO\")") || source.contains("var(\"CARGO\")"))
    {
        tools.push("cargo".to_owned());
    }
    tools.sort();
    tools.dedup();
    tools
}
