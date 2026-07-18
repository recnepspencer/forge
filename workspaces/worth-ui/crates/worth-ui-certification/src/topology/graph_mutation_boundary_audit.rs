use std::path::Path;

use super::workspace_source_inventory::WorkspaceSourceInventory;

struct ForbiddenCall<'a> {
    type_name: &'a str,
    method_name: &'a str,
    allowed_paths: &'a [&'a Path],
    message: &'a str,
}

fn strip_comments_and_literals(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut output = String::with_capacity(text.len());
    let mut index = 0;

    while index < bytes.len() {
        let current = bytes[index];
        let next = bytes.get(index + 1).copied();

        match (current, next) {
            (b'/', Some(b'/')) => {
                output.push(' ');
                output.push(' ');
                index += 2;
                while index < bytes.len() && bytes[index] != b'\n' {
                    output.push(' ');
                    index += 1;
                }
            }
            (b'/', Some(b'*')) => {
                output.push(' ');
                output.push(' ');
                index += 2;
                while index + 1 < bytes.len() {
                    if bytes[index] == b'*' && bytes[index + 1] == b'/' {
                        output.push(' ');
                        output.push(' ');
                        index += 2;
                        break;
                    }
                    output.push(if bytes[index] == b'\n' { '\n' } else { ' ' });
                    index += 1;
                }
            }
            (b'"', _) => {
                output.push(' ');
                index += 1;
                while index < bytes.len() {
                    if bytes[index] == b'\\' {
                        output.push(' ');
                        index += 1;
                        if index < bytes.len() {
                            output.push(' ');
                            index += 1;
                        }
                        continue;
                    }
                    let ch = bytes[index] as char;
                    output.push(if ch == '\n' { '\n' } else { ' ' });
                    index += 1;
                    if ch == '"' {
                        break;
                    }
                }
            }
            (b'\'', _) => {
                output.push(' ');
                index += 1;
                while index < bytes.len() {
                    if bytes[index] == b'\\' {
                        output.push(' ');
                        index += 1;
                        if index < bytes.len() {
                            output.push(' ');
                            index += 1;
                        }
                        continue;
                    }
                    let ch = bytes[index] as char;
                    output.push(if ch == '\n' { '\n' } else { ' ' });
                    index += 1;
                    if ch == '\'' {
                        break;
                    }
                }
            }
            _ => {
                output.push(current as char);
                index += 1;
            }
        }
    }

    output
}

fn tokenize_source(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }

        if ch.is_ascii_alphanumeric() || ch == '_' {
            let mut ident = String::from(ch);
            while let Some(next) = chars.peek() {
                if next.is_ascii_alphanumeric() || *next == '_' {
                    ident.push(chars.next().expect("peeked identifier char should exist"));
                } else {
                    break;
                }
            }
            tokens.push(ident);
            continue;
        }

        if ch == ':' && matches!(chars.peek(), Some(':')) {
            chars.next();
            tokens.push("::".to_string());
            continue;
        }

        tokens.push(ch.to_string());
    }

    tokens
}

fn source_aliases_for_type(tokens: &[String], type_name: &str) -> Vec<String> {
    let mut aliases = vec![type_name.to_string()];

    let mut index = 0;
    while index < tokens.len() {
        match tokens[index].as_str() {
            "use" => {
                let statement_end = tokens[index..]
                    .iter()
                    .position(|token| token == ";")
                    .map(|offset| index + offset)
                    .unwrap_or(tokens.len());
                for position in index..statement_end {
                    if tokens[position] != type_name {
                        continue;
                    }
                    if position + 2 < statement_end && tokens[position + 1] == "as" {
                        aliases.push(tokens[position + 2].clone());
                    }
                }
                index = statement_end.saturating_add(1);
            }
            "type" => {
                if index + 2 >= tokens.len() {
                    break;
                }
                let alias = tokens[index + 1].clone();
                let statement_end = tokens[index..]
                    .iter()
                    .position(|token| token == ";")
                    .map(|offset| index + offset)
                    .unwrap_or(tokens.len());
                if tokens[index + 2] == "="
                    && tokens[index + 3..statement_end]
                        .iter()
                        .any(|token| token == type_name)
                {
                    aliases.push(alias);
                }
                index = statement_end.saturating_add(1);
            }
            _ => {
                index += 1;
            }
        }
    }

    aliases.sort();
    aliases.dedup();
    aliases
}

fn contains_qualified_forbidden_call(
    tokens: &[String],
    aliases: &[String],
    method_name: &str,
) -> bool {
    tokens.windows(4).any(|window| {
        aliases.iter().any(|alias| {
            window[0] == *alias && window[1] == "::" && window[2] == method_name && window[3] == "("
        })
    })
}

fn contains_qualified_forbidden_reference(
    tokens: &[String],
    aliases: &[String],
    method_name: &str,
) -> bool {
    tokens.windows(3).any(|window| {
        aliases
            .iter()
            .any(|alias| window[0] == *alias && window[1] == "::" && window[2] == method_name)
    })
}

fn binding_invokes_forbidden_call(
    tokens: &[String],
    aliases: &[String],
    method_name: &str,
) -> bool {
    let mut rebound_names = Vec::new();
    let mut index = 0;

    while index < tokens.len() {
        if tokens[index] != "let" {
            index += 1;
            continue;
        }

        let binding_index = match tokens.get(index + 1).map(String::as_str) {
            Some("mut") => index + 2,
            _ => index + 1,
        };
        let Some(binding_name) = tokens.get(binding_index).cloned() else {
            break;
        };
        let Some(equal_sign_index) = tokens[binding_index + 1..]
            .iter()
            .position(|token| token == "=")
            .map(|offset| binding_index + 1 + offset)
        else {
            index = binding_index + 1;
            continue;
        };
        let statement_end = tokens[equal_sign_index + 1..]
            .iter()
            .position(|token| token == ";")
            .map(|offset| equal_sign_index + 1 + offset)
            .unwrap_or(tokens.len());

        if contains_qualified_forbidden_reference(
            &tokens[equal_sign_index + 1..statement_end],
            aliases,
            method_name,
        ) {
            rebound_names.push(binding_name);
        }

        index = statement_end.saturating_add(1);
    }

    if rebound_names.is_empty() {
        return false;
    }

    tokens.windows(2).any(|window| {
        rebound_names
            .iter()
            .any(|binding_name| window[0] == *binding_name && window[1] == "(")
    })
}

fn contains_forbidden_call(tokens: &[String], aliases: &[String], method_name: &str) -> bool {
    contains_qualified_forbidden_call(tokens, aliases, method_name)
        || binding_invokes_forbidden_call(tokens, aliases, method_name)
}

fn audit_forbidden_call(
    path: &Path,
    text: &str,
    forbidden_call: &ForbiddenCall<'_>,
) -> Option<String> {
    if forbidden_call.allowed_paths.contains(&path) {
        return None;
    }

    let sanitized_source = strip_comments_and_literals(text);
    let tokens = tokenize_source(&sanitized_source);
    let aliases = source_aliases_for_type(&tokens, forbidden_call.type_name);

    contains_forbidden_call(&tokens, &aliases, forbidden_call.method_name)
        .then(|| format!("{} {}", path.display(), forbidden_call.message))
}

pub fn audit_graph_mutation_boundary_owns_snapshot_and_index_commit(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let runtime_root = inventory.absolute_path("crates/worth-ui-runtime/src");
    let mutation_stage_file = runtime_root.join("graph/mutation/graph_mutation_stage.rs");
    let snapshot_file = runtime_root.join("graph/snapshot/graph_snapshot.rs");
    let topology_mutation_file = runtime_root.join("graph/topology/topology_mutation.rs");
    let mounted_receipt_store_file =
        runtime_root.join("graph/mounted_receipt/mounted_receipt_authority_seed_store.rs");
    let forbidden_calls = [
        ForbiddenCall {
            type_name: "UiGraphCoreIndexes",
            method_name: "build",
            allowed_paths: &[&mutation_stage_file],
            message: "rebuilds core indexes outside the graph mutation boundary",
        },
        ForbiddenCall {
            type_name: "UiGraphSnapshot",
            method_name: "new",
            allowed_paths: &[&mutation_stage_file, &snapshot_file],
            message: "constructs committed graph snapshots outside the graph mutation boundary",
        },
        ForbiddenCall {
            type_name: "UiGraphNode",
            method_name: "new",
            allowed_paths: &[&mutation_stage_file],
            message: "constructs graph nodes outside the graph mutation boundary",
        },
        ForbiddenCall {
            type_name: "UiGraphTopology",
            method_name: "new",
            allowed_paths: &[&topology_mutation_file],
            message: "constructs authoritative topology outside the graph mutation boundary",
        },
        ForbiddenCall {
            type_name: "UiGraphNodeTopology",
            method_name: "new",
            allowed_paths: &[&topology_mutation_file],
            message: "constructs authoritative node topology outside the graph mutation boundary",
        },
        ForbiddenCall {
            type_name: "UiGraphMembershipFacts",
            method_name: "new",
            allowed_paths: &[&topology_mutation_file],
            message: "constructs authoritative membership facts outside the graph mutation boundary",
        },
        ForbiddenCall {
            type_name: "UiGraphMountedReceiptAuthoritySeedStore",
            method_name: "new",
            allowed_paths: &[&mounted_receipt_store_file],
            message: "constructs mounted-receipt authority seed state outside the graph mutation boundary",
        },
        ForbiddenCall {
            type_name: "UiGraphMountedReceiptSlot",
            method_name: "new",
            allowed_paths: &[&mounted_receipt_store_file],
            message: "constructs mounted-receipt authority slots outside the graph mutation boundary",
        },
    ];
    let mut violations = Vec::new();

    for source in inventory.rust_files_under("crates/worth-ui-runtime/src") {
        let path = source.absolute_path();
        let text = source.text();
        for forbidden_call in forbidden_calls.iter() {
            if let Some(violation) = audit_forbidden_call(path, text, forbidden_call) {
                violations.push(violation);
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{audit_forbidden_call, ForbiddenCall};

    fn forbidden_call() -> ForbiddenCall<'static> {
        ForbiddenCall {
            type_name: "UiGraphCoreIndexes",
            method_name: "build",
            allowed_paths: &[],
            message: "rebuilds core indexes outside the graph mutation boundary",
        }
    }

    fn violates(source: &str) -> bool {
        audit_forbidden_call(Path::new("graph/fake.rs"), source, &forbidden_call()).is_some()
    }

    #[test]
    fn detects_direct_qualified_calls() {
        assert!(violates("fn bad() { UiGraphCoreIndexes::build(plan); }"));
    }

    #[test]
    fn detects_use_alias_calls() {
        assert!(violates(
            "use crate::graph::UiGraphCoreIndexes as Indexes; fn bad() { Indexes::build(plan); }"
        ));
    }

    #[test]
    fn detects_type_alias_calls() {
        assert!(violates(
            "type Indexes = UiGraphCoreIndexes; fn bad() { Indexes::build(plan); }"
        ));
    }

    #[test]
    fn detects_local_rebinding_calls() {
        assert!(violates(
            "fn bad() { let build = UiGraphCoreIndexes::build; build(plan); }"
        ));
    }

    #[test]
    fn ignores_comments_and_strings() {
        assert!(!violates(
            r#"fn okay() { let _ = "UiGraphCoreIndexes::build(plan)"; // UiGraphCoreIndexes::build(plan)
            /* UiGraphCoreIndexes::build(plan) */ }"#
        ));
    }
}
