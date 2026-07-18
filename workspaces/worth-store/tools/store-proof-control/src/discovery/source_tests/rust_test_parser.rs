use std::collections::{BTreeMap, BTreeSet};

pub(super) struct RustTestFunction {
    pub(super) name: String,
    pub(super) line: usize,
    pub(super) ignored: bool,
    pub(super) assertion_predicates: Vec<String>,
    pub(super) behavior_fingerprint: String,
    pub(super) execution_source: String,
}

pub(super) struct ExecutionGraph {
    function_sources: BTreeMap<String, Vec<String>>,
}

pub(super) fn execution_graph(source: &str) -> ExecutionGraph {
    let lines: Vec<_> = source.lines().collect();
    ExecutionGraph {
        function_sources: declared_function_sources(&lines),
    }
}

pub(super) fn rust_test_functions(
    source: &str,
    execution_graph: &ExecutionGraph,
) -> Vec<RustTestFunction> {
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
                let direct_source = function_source(&lines, index + 1);
                tests.push(RustTestFunction {
                    execution_source: execution_source(&direct_source, execution_graph),
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

fn declared_function_sources(lines: &[&str]) -> BTreeMap<String, Vec<String>> {
    let mut functions = BTreeMap::<String, Vec<String>>::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(name) = function_name(line.trim()) else {
            continue;
        };
        functions
            .entry(name)
            .or_default()
            .push(function_source(lines, index));
    }
    functions
}

fn execution_source(root: &str, graph: &ExecutionGraph) -> String {
    let mut reachable_names = BTreeSet::new();
    let mut pending = called_function_names(root).into_iter().collect::<Vec<_>>();
    let mut execution = root.to_owned();
    while let Some(name) = pending.pop() {
        if !reachable_names.insert(name.clone()) {
            continue;
        }
        let Some(definitions) = graph.function_sources.get(&name) else {
            continue;
        };
        for definition in definitions {
            execution.push_str(definition);
            pending.extend(called_function_names(definition));
        }
    }
    execution
}

fn called_function_names(source: &str) -> BTreeSet<String> {
    let bytes = source.as_bytes();
    let mut names = BTreeSet::new();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if !bytes[cursor].is_ascii_alphabetic() && bytes[cursor] != b'_' {
            cursor += 1;
            continue;
        }
        let start = cursor;
        cursor += 1;
        while cursor < bytes.len()
            && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
        {
            cursor += 1;
        }
        let mut after = cursor;
        while after < bytes.len() && bytes[after].is_ascii_whitespace() {
            after += 1;
        }
        if after < bytes.len() && bytes[after] == b'(' {
            names.insert(source[start..cursor].to_owned());
        }
    }
    names
}

fn function_source(lines: &[&str], function_start: usize) -> String {
    let mut depth = 0_i32;
    let mut entered_body = false;
    let mut source = String::new();
    for line in lines.iter().skip(function_start) {
        depth += line.matches('{').count() as i32;
        entered_body |= depth > 0;
        source.push_str(line);
        source.push('\n');
        depth -= line.matches('}').count() as i32;
        if entered_body && depth <= 0 {
            break;
        }
    }
    source
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
    source.contains("Command::new")
        || source.contains("std::process::Command")
        || uses_standardized_ui_harness(source)
}

pub(super) fn launches_nested_cargo(source: &str) -> bool {
    uses_standardized_ui_harness(source)
        || (launches_child_process(source)
            && (source.contains("Command::new(\"cargo\")")
                || source.contains("Command::new(cargo")
                || source.contains("var_os(\"CARGO\")")
                || source.contains("var(\"CARGO\")")))
}

pub(super) fn uses_standardized_ui_harness(source: &str) -> bool {
    source.contains("run_cargo_ui_fixture_suite") || source.contains("run_ui_proof_suite")
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
        && (source.contains("var_os(\"CARGO\")")
            || source.contains("var(\"CARGO\")")
            || uses_standardized_ui_harness(source))
    {
        tools.push("cargo".to_owned());
    }
    tools.sort();
    tools.dedup();
    tools
}
