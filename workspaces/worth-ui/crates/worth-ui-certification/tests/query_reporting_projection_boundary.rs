use std::path::Path;

const PRODUCTION_ROOTS: &[&str] = &[
    "crates/worth-ui-query-binding/src",
    "crates/worth-ui-runtime/src",
];

struct ReportingProjectionHome {
    path: &'static str,
    admitted_calls: &'static [&'static str],
}

const REPORTING_PROJECTION_HOMES: &[ReportingProjectionHome] = &[
    ReportingProjectionHome {
        path: "crates/worth-ui-query-binding/src/projection_consumption/reporting.rs",
        admitted_calls: &["terminal_projection_for_reporting("],
    },
    ReportingProjectionHome {
        path: "crates/worth-ui-query-binding/src/certification/\
scalar_native_authority_projection.rs",
        admitted_calls: &[".certification_projection_contract().contract_digest()"],
    },
];

const BANNED_OPERATIONAL_CALLS: &[&str] = &[
    ".contract().contract_digest()",
    ".certification_projection_contract().contract_digest()",
    ".receipt().declaration_digest()",
    ".receipt().receipt_digest()",
    ".receipt().fact_set_digest()",
    ".source_identity().as_str()",
    "terminal_projection_for_reporting(",
];

const BANNED_WORTH_UI_REPORTING_MIRRORS: &[&str] = &[
    "identity_for_reporting",
    "binding_identity_for_reporting",
    "query_binding_identity_for_reporting",
    "query_world_identity_for_reporting",
    "source_generation_for_reporting",
    "result_generation_for_reporting",
    "predecessor_binding_identity_for_reporting",
    "successor_binding_identity_for_reporting",
];

const BANNED_MIRROR_NAMES: &[&str] = &[
    "QueryViewBindingKey",
    "QueryViewCapabilityReference",
    "QueryResultShapeReference",
    "QueryBasisPostureReference",
    "QueryLiveCompatibility",
];

#[test]
fn query_reporting_projections_stay_out_of_operational_code() {
    let inventory = super::workspace_source_inventory();
    let mut findings = Vec::new();

    for production_root in PRODUCTION_ROOTS {
        for source in inventory.rust_files_under(production_root) {
            let compact = source
                .text()
                .chars()
                .filter(|character| !character.is_whitespace())
                .collect::<String>();
            for banned in BANNED_OPERATIONAL_CALLS {
                if compact.contains(banned)
                    && !projection_home_admits(source.relative_path(), banned)
                {
                    findings.push(format!(
                        "{} uses Query reporting projection `{banned}` outside its explicit projection home",
                        source.absolute_path().display()
                    ));
                }
            }
            for banned in BANNED_WORTH_UI_REPORTING_MIRRORS {
                if source.text().contains(banned) {
                    findings.push(format!(
                        "{} recreates Query reporting text through WORTH UI mirror `{banned}`",
                        source.absolute_path().display()
                    ));
                }
            }
            for banned in BANNED_MIRROR_NAMES {
                if source.text().contains(banned) {
                    findings.push(format!(
                        "{} recreates deleted Query mirror `{banned}`",
                        source.absolute_path().display()
                    ));
                }
            }
        }
    }
    assert!(findings.is_empty(), "{}", findings.join("\n"));
}

fn projection_home_admits(path: &Path, call: &str) -> bool {
    REPORTING_PROJECTION_HOMES
        .iter()
        .any(|home| path == Path::new(home.path) && home.admitted_calls.contains(&call))
}
