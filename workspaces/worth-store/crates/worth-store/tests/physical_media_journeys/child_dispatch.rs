use std::collections::BTreeMap;
use std::path::Path;

const CHILD_TEST: &str = "child_dispatch::journey_child_role";
const REPORT_PREFIX: &str = "C4_REPORT ";

pub(super) struct ChildReport {
    fields: BTreeMap<String, String>,
}

impl ChildReport {
    pub(super) fn value(&self, name: &str) -> &str {
        self.fields
            .get(name)
            .unwrap_or_else(|| panic!("child report omitted {name}"))
    }

    pub(super) fn number(&self, name: &str) -> u64 {
        self.value(name)
            .parse()
            .unwrap_or_else(|_| panic!("child report field {name} is not numeric"))
    }
}

pub(super) fn run_role(role: &str, root: &Path, environment: &[(&str, &str)]) -> ChildReport {
    let mut command = std::process::Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", CHILD_TEST, "--nocapture"])
        .env("WORTH_STORE_C4_CHILD_ROLE", role)
        .env("WORTH_STORE_C4_CHILD_ROOT", root);
    for (name, value) in environment {
        command.env(name, value);
    }
    let output = command.output().expect("fresh journey process must start");
    assert!(
        output.status.success(),
        "C.4 child role {role} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    parse_report(&output.stdout)
}

pub(super) fn emit(fields: &[(&str, String)]) {
    let encoded = fields
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join(";");
    println!("{REPORT_PREFIX}{encoded}");
}

fn parse_report(stdout: &[u8]) -> ChildReport {
    let stdout = String::from_utf8(stdout.to_vec()).expect("child report must be UTF-8");
    let line = stdout
        .lines()
        .find_map(|line| line.strip_prefix(REPORT_PREFIX))
        .unwrap_or_else(|| panic!("child emitted no C.4 report:\n{stdout}"));
    let mut fields = BTreeMap::new();
    for field in line.split(';') {
        let (name, value) = field
            .split_once('=')
            .unwrap_or_else(|| panic!("malformed child report field: {field}"));
        assert!(fields.insert(name.to_owned(), value.to_owned()).is_none());
    }
    ChildReport { fields }
}

#[test]
fn journey_child_role() {
    let Ok(role) = std::env::var("WORTH_STORE_C4_CHILD_ROLE") else {
        return;
    };
    let root = std::env::var_os("WORTH_STORE_C4_CHILD_ROOT")
        .map(std::path::PathBuf::from)
        .expect("child role requires a root");
    match role.as_str() {
        #[cfg(feature = "certification-test-authority")]
        "namespace-writer" | "namespace-reopener" => {
            super::namespace_discovery::run_child(&role, &root)
        }
        #[cfg(feature = "certification-test-authority")]
        "mutation-contender" | "unrelated-inheritance-probe" | "post-death-successor" => {
            super::mutation_contention::run_child(&role, &root)
        }
        #[cfg(feature = "certification-test-authority")]
        "faulted-media-writer" | "fault-reopener" => {
            super::partial_effects::run_child(&role, &root)
        }
        _ => panic!("unknown C.4 child role: {role}"),
    }
}
