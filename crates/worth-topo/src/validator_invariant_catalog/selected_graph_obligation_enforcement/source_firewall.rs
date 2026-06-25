use std::path::Path;

const FORBIDDEN_PHASE_SIX_SOURCE_TOKENS: [&str; 5] = [
    "HashMap<String, WorthTopologySelectedGraphObligationEnforcementReceipt>",
    "fabricated_graph_obligation_receipt",
    "local_graph_obligation_executor",
    "private_legality_graph",
    "string_list_support_pin",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologySelectedGraphObligationEnforcementSourceFirewallReport {
    scanned_file_count: usize,
    violations: Vec<String>,
    report_digest: String,
}

impl WorthTopologySelectedGraphObligationEnforcementSourceFirewallReport {
    pub(in crate::validator_invariant_catalog) fn current() -> Self {
        Self::from_scan_root(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join("validator_invariant_catalog"),
        )
    }

    pub(in crate::validator_invariant_catalog) fn from_scan_root(root: impl AsRef<Path>) -> Self {
        let mut scanned_file_count = 0;
        let mut violations = Vec::new();
        scan_dir(root.as_ref(), &mut scanned_file_count, &mut violations);
        Self::from_scan_parts(scanned_file_count, violations)
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn from_source_pairs(
        sources: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> Self {
        let mut scanned_file_count = 0;
        let mut violations = Vec::new();
        for (label, source) in sources {
            scanned_file_count += 1;
            scan_source(label.as_ref(), source.as_ref(), &mut violations);
        }
        Self::from_scan_parts(scanned_file_count, violations)
    }

    fn from_scan_parts(scanned_file_count: usize, mut violations: Vec<String>) -> Self {
        violations.sort();
        let mut parts = vec![
            "worth-topo-selected-graph-obligation-enforcement-source-firewall-v1".to_string(),
            format!("scanned-file-count:{scanned_file_count}"),
        ];
        parts.extend(
            violations
                .iter()
                .map(|violation| format!("violation:{violation}")),
        );
        Self {
            scanned_file_count,
            violations,
            report_digest: parts.join("|"),
        }
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub fn violations(&self) -> &[String] {
        &self.violations
    }

    pub const fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn scan_dir(path: &Path, scanned_file_count: &mut usize, violations: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_dir(&path, scanned_file_count, violations);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        if path.file_name().and_then(|name| name.to_str()) == Some("source_firewall.rs") {
            continue;
        }
        *scanned_file_count += 1;
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        scan_source(&path.display().to_string(), &source, violations);
    }
}

fn scan_source(label: &str, source: &str, violations: &mut Vec<String>) {
    for forbidden in FORBIDDEN_PHASE_SIX_SOURCE_TOKENS {
        if source.contains(forbidden) {
            violations.push(format!("{label}::{forbidden}"));
        }
    }
}
