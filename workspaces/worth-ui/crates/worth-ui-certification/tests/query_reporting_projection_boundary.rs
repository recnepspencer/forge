use std::path::Path;

const PRODUCTION_ROOTS: &[&str] = &[
    "crates/worth-ui-query-binding/src",
    "crates/worth-ui-runtime/src",
];

const REPORTING_PROJECTION_HOME: &str = "crates/worth-ui-runtime/src/evidence/measurement/\
projection/inspection_receipt/query_reporting_projection.rs";

const BANNED_OPERATIONAL_CALLS: &[&str] = &[
    ".contract().contract_digest()",
    ".receipt().declaration_digest()",
    ".receipt().receipt_digest()",
    ".receipt().fact_set_digest()",
    ".source_identity().as_str()",
    "terminal_projection_for_reporting(",
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
    let reporting_projection_home = Path::new(REPORTING_PROJECTION_HOME);
    let mut findings = Vec::new();

    for production_root in PRODUCTION_ROOTS {
        for source in inventory.rust_files_under(production_root) {
            let compact = source
                .text()
                .chars()
                .filter(|character| !character.is_whitespace())
                .collect::<String>();
            if source.relative_path() != reporting_projection_home {
                for banned in BANNED_OPERATIONAL_CALLS {
                    if compact.contains(banned) {
                        findings.push(format!(
                            "{} uses Query reporting projection `{banned}` outside the inspection-only projection home",
                            source.absolute_path().display()
                        ));
                    }
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
