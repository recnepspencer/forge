use std::collections::BTreeMap;
use std::path::Path;

const EXPECTED_PATHS: [&str; 5] = [
    "families",
    "namespace",
    "namespace/identity",
    "namespace/mutation.lock",
    "staging",
];

pub(super) struct ObserverReport {
    values: BTreeMap<String, String>,
    entries: Vec<(String, String)>,
}

impl ObserverReport {
    pub(super) fn value(&self, name: &str) -> &str {
        self.values
            .get(name)
            .unwrap_or_else(|| panic!("observer omitted {name}"))
    }

    pub(super) fn path_kinds(&self) -> Vec<(String, String)> {
        self.entries.clone()
    }
}

pub(super) fn observe_namespace(root: &Path) -> ObserverReport {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_physical_media_os_observer"))
        .arg("--namespace")
        .arg(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = String::from_utf8(output.stdout).unwrap();
    let mut values = BTreeMap::new();
    let mut entries = Vec::new();
    for line in output.lines() {
        let (name, value) = line.split_once('=').expect("observer line must be named");
        if name == "entry" {
            let mut fields = value.splitn(4, '|');
            let kind = fields.next().unwrap().to_owned();
            let _length = fields.next().unwrap();
            let _digest = fields.next().unwrap();
            let path = fields.next().unwrap().to_owned();
            entries.push((path, kind));
        } else {
            assert!(values.insert(name.to_owned(), value.to_owned()).is_none());
        }
    }
    ObserverReport { values, entries }
}

pub(super) fn assert_namespace_shape(report: &ObserverReport) {
    assert_eq!(report.value("namespace_version"), "1");
    assert_eq!(report.value("encoding_version"), "1");
    assert_eq!(report.value("identity_record_length"), "72");
    let paths = report
        .entries
        .iter()
        .map(|(path, _)| path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(paths, EXPECTED_PATHS);
    assert_eq!(report.entries[0].1, "directory");
    assert_eq!(report.entries[1].1, "directory");
    assert_eq!(report.entries[2].1, "file");
    assert_eq!(report.entries[3].1, "file");
    assert_eq!(report.entries[4].1, "directory");
}
