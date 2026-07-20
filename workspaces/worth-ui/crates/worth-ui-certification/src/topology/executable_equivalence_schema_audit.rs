use super::WorkspaceSourceInventory;

const NODE_INPUT_PATH: &str =
    "crates/worth-ui-runtime/src/runtime/execution_plan_input/node_input.rs";
const BUNDLE_PATH: &str =
    "crates/worth-ui-runtime/src/runtime/active/sealed_execution_plan_bundle.rs";

pub fn audit_complete_executable_equivalence_schema(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let mut violations = audit_executable_node_schema_source(
        inventory
            .source(NODE_INPUT_PATH)
            .expect("executable node schema source exists")
            .text(),
    );
    let bundle = inventory
        .source(BUNDLE_PATH)
        .expect("sealed bundle source exists")
        .text();
    for required in [
        "region_storage_counters",
        "changed_region_count == 0",
        "self.digest == candidate.digest",
        "lane_admission.executable_contract_matches",
        "host_binding.executable_contract_matches",
        "predecessor_artifact_digest",
        "predecessor_plan_digest",
    ] {
        if !compact(bundle).contains(&compact(required)) {
            violations.push(format!(
                "{BUNDLE_PATH} omits executable equivalence constituent `{required}`"
            ));
        }
    }
    violations.sort();
    violations
}

pub fn audit_executable_node_schema_source(source: &str) -> Vec<String> {
    let syntax = match syn::parse_file(source) {
        Ok(syntax) => syntax,
        Err(error) => return vec![format!("executable node schema does not parse: {error}")],
    };
    let Some(fields) = syntax.items.iter().find_map(|item| match item {
        syn::Item::Struct(item) if item.ident == "WorthUiPlanNodeInput" => Some(
            item.fields
                .iter()
                .filter_map(|field| field.ident.as_ref().map(ToString::to_string))
                .collect::<Vec<_>>(),
        ),
        _ => None,
    }) else {
        return vec!["WorthUiPlanNodeInput schema is missing".to_owned()];
    };
    let exclusions = exclusion_fields(source);
    let comparator = source
        .split("fn executable_schema_matches")
        .nth(1)
        .and_then(|tail| tail.split("pub(crate) fn from_launch_query_binding").next())
        .unwrap_or_default();
    let mut violations = Vec::new();
    for field in fields {
        let compared = comparator.contains(&format!("self.{field}"))
            && comparator.contains(&format!("other.{field}"));
        let excluded = exclusions.iter().any(|entry| entry == &field);
        if compared == excluded {
            violations.push(format!(
                "WorthUiPlanNodeInput field `{field}` must be compared or explicitly excluded exactly once"
            ));
        }
    }
    violations
}

fn exclusion_fields(source: &str) -> Vec<String> {
    let marker = "non-executable-schema-fields:";
    let mut joined = String::new();
    let mut collecting = false;
    for line in source.lines() {
        if let Some((_, suffix)) = line.split_once(marker) {
            collecting = true;
            joined.push_str(suffix);
        } else if collecting && line.trim_start().starts_with("//") {
            joined.push_str(line.trim_start().trim_start_matches("//"));
        } else if collecting {
            break;
        }
    }
    joined
        .split(',')
        .map(str::trim)
        .filter(|field| !field.is_empty())
        .map(str::to_owned)
        .collect()
}

fn compact(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}
