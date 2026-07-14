use sha2::{Digest, Sha256};
use syn::{Item, UseTree, Visibility};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOrdinaryApiSnapshot {
    namespace: &'static str,
    source_path: &'static str,
    symbol_count: usize,
    symbol_digest: String,
}

impl WorthQueryOrdinaryApiSnapshot {
    pub fn namespace(&self) -> &'static str {
        self.namespace
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn symbol_count(&self) -> usize {
        self.symbol_count
    }

    pub fn symbol_digest(&self) -> &str {
        &self.symbol_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOrdinaryApiSnapshotFinding {
    namespace: &'static str,
    expected_symbol_count: usize,
    actual_symbol_count: usize,
    expected_symbol_digest: &'static str,
    actual_symbol_digest: String,
}

impl WorthQueryOrdinaryApiSnapshotFinding {
    pub fn namespace(&self) -> &'static str {
        self.namespace
    }

    pub fn expected_symbol_count(&self) -> usize {
        self.expected_symbol_count
    }

    pub fn actual_symbol_count(&self) -> usize {
        self.actual_symbol_count
    }

    pub fn expected_symbol_digest(&self) -> &'static str {
        self.expected_symbol_digest
    }

    pub fn actual_symbol_digest(&self) -> &str {
        &self.actual_symbol_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOrdinaryApiSnapshotAudit {
    snapshots: Vec<WorthQueryOrdinaryApiSnapshot>,
    findings: Vec<WorthQueryOrdinaryApiSnapshotFinding>,
}

impl WorthQueryOrdinaryApiSnapshotAudit {
    pub fn snapshots(&self) -> &[WorthQueryOrdinaryApiSnapshot] {
        &self.snapshots
    }

    pub fn findings(&self) -> &[WorthQueryOrdinaryApiSnapshotFinding] {
        &self.findings
    }

    pub fn is_complete(&self) -> bool {
        self.findings.is_empty()
    }
}

#[derive(Clone, Copy)]
struct ExpectedSnapshot {
    namespace: &'static str,
    source_path: &'static str,
    source: &'static str,
    symbol_count: usize,
    symbol_digest: &'static str,
}

macro_rules! expected_snapshot {
    ($namespace:literal, $file:literal, $count:literal, $digest:literal) => {
        ExpectedSnapshot {
            namespace: $namespace,
            source_path: concat!("src/facade/", $file),
            source: include_str!(concat!("../../facade/", $file)),
            symbol_count: $count,
            symbol_digest: $digest,
        }
    };
}

const EXPECTED_SNAPSHOTS: &[ExpectedSnapshot] = &[
    expected_snapshot!(
        "read",
        "exports_read.rs",
        93,
        "0ea14c9e54c33ec300997d95aba41d5338a7c303dc4b4f4f6e07674159464d00"
    ),
    expected_snapshot!(
        "aggregate",
        "exports_aggregate.rs",
        53,
        "7e5ac3ed3b094aa72c6301dd3b60d7934a02d7f0064f8636bd42216e2c62b837"
    ),
    expected_snapshot!(
        "live",
        "exports_live_capability.rs",
        88,
        "169430266b03b252915ca44f36f79f56266a152ca96ec6c75a07f878ecda1118"
    ),
    expected_snapshot!(
        "history",
        "exports_history.rs",
        53,
        "d0e52f20abcb366fe435f7e986a08a93b5535be618f48acd0455811b09510316"
    ),
    expected_snapshot!(
        "comparison",
        "exports_comparison.rs",
        80,
        "df1532a361775c0602acd9c5433a44bd72eb6ad57a8a2885a9260f973b2d71f6"
    ),
    expected_snapshot!(
        "mutation",
        "exports_mutation.rs",
        23,
        "3a3bf227e29b11a80418105bba61e37775ed8bd3205b8321b505b1581f46bc7a"
    ),
    expected_snapshot!(
        "preview",
        "exports_preview.rs",
        36,
        "80c5a7212e0ee37904c389abf4f0cacae582e82d6c7113d5d89939159453bb60"
    ),
    expected_snapshot!(
        "workflow",
        "exports_workflow.rs",
        66,
        "fa318d953503884b315226b4c7a7ab044b38a8c13e9dc59c6e7e9d31bbaf89c5"
    ),
    expected_snapshot!(
        "domain",
        "exports_domain.rs",
        27,
        "ab88b3f81e9621c15c626e57fd823547fda551281c17597ff06eff4100039f0c"
    ),
    expected_snapshot!(
        "inspection",
        "exports_inspection.rs",
        21,
        "119acb84d2f4eca5629045cbfec7836d602a0570f6efdd02837a981b2d918bb5"
    ),
];

pub fn current_ordinary_api_snapshot_audit() -> WorthQueryOrdinaryApiSnapshotAudit {
    audit_expected_snapshots(EXPECTED_SNAPSHOTS)
}

fn audit_expected_snapshots(expected: &[ExpectedSnapshot]) -> WorthQueryOrdinaryApiSnapshotAudit {
    let mut snapshots = Vec::with_capacity(expected.len());
    let mut findings = Vec::new();
    for row in expected {
        let snapshot = snapshot_source(row.namespace, row.source_path, row.source);
        if snapshot.symbol_count != row.symbol_count || snapshot.symbol_digest != row.symbol_digest
        {
            findings.push(WorthQueryOrdinaryApiSnapshotFinding {
                namespace: row.namespace,
                expected_symbol_count: row.symbol_count,
                actual_symbol_count: snapshot.symbol_count,
                expected_symbol_digest: row.symbol_digest,
                actual_symbol_digest: snapshot.symbol_digest.clone(),
            });
        }
        snapshots.push(snapshot);
    }
    WorthQueryOrdinaryApiSnapshotAudit {
        snapshots,
        findings,
    }
}

fn snapshot_source(
    namespace: &'static str,
    source_path: &'static str,
    source: &str,
) -> WorthQueryOrdinaryApiSnapshot {
    let syntax = syn::parse_file(source)
        .unwrap_or_else(|error| panic!("invalid facade export source {source_path}: {error}"));
    let mut symbols = Vec::new();
    for item in &syntax.items {
        collect_public_item(item, &mut symbols);
    }
    symbols.sort();
    symbols.dedup();
    let mut hasher = Sha256::new();
    for symbol in &symbols {
        hasher.update(symbol.as_bytes());
        hasher.update(b"\n");
    }
    WorthQueryOrdinaryApiSnapshot {
        namespace,
        source_path,
        symbol_count: symbols.len(),
        symbol_digest: format!("{:x}", hasher.finalize()),
    }
}

fn collect_public_item(item: &Item, symbols: &mut Vec<String>) {
    match item {
        Item::Use(item) if matches!(item.vis, Visibility::Public(_)) => {
            collect_use_tree(&item.tree, symbols)
        }
        Item::Const(item) if matches!(item.vis, Visibility::Public(_)) => {
            symbols.push(item.ident.to_string())
        }
        Item::Enum(item) if matches!(item.vis, Visibility::Public(_)) => {
            symbols.push(item.ident.to_string())
        }
        Item::Fn(item) if matches!(item.vis, Visibility::Public(_)) => {
            symbols.push(item.sig.ident.to_string())
        }
        Item::Mod(item) if matches!(item.vis, Visibility::Public(_)) => {
            symbols.push(item.ident.to_string())
        }
        Item::Static(item) if matches!(item.vis, Visibility::Public(_)) => {
            symbols.push(item.ident.to_string())
        }
        Item::Struct(item) if matches!(item.vis, Visibility::Public(_)) => {
            symbols.push(item.ident.to_string())
        }
        Item::Trait(item) if matches!(item.vis, Visibility::Public(_)) => {
            symbols.push(item.ident.to_string())
        }
        Item::Type(item) if matches!(item.vis, Visibility::Public(_)) => {
            symbols.push(item.ident.to_string())
        }
        _ => {}
    }
}

fn collect_use_tree(tree: &UseTree, symbols: &mut Vec<String>) {
    match tree {
        UseTree::Name(name) => symbols.push(name.ident.to_string()),
        UseTree::Rename(rename) => symbols.push(rename.rename.to_string()),
        UseTree::Path(path) => collect_use_tree(&path.tree, symbols),
        UseTree::Group(group) => {
            for tree in &group.items {
                collect_use_tree(tree, symbols);
            }
        }
        UseTree::Glob(_) => symbols.push("*".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_namespaces_match_their_golden_api_snapshots() {
        let audit = current_ordinary_api_snapshot_audit();
        assert!(
            audit.is_complete(),
            "snapshot findings: {:?}",
            audit.findings()
        );
    }

    #[test]
    fn seeded_phase_reexport_changes_the_exact_namespace_snapshot() {
        let expected = [ExpectedSnapshot {
            namespace: "read",
            source_path: "seeded/exports_read.rs",
            source: "pub use crate::ordinary::read::declare;\npub use crate::planning::plan_validated_bundle;",
            symbol_count: 1,
            symbol_digest: snapshot_source("read", "seeded/baseline.rs", "pub use crate::ordinary::read::declare;")
                .symbol_digest
                .leak(),
        }];
        let audit = audit_expected_snapshots(&expected);

        assert_eq!(audit.findings().len(), 1);
        assert_eq!(audit.findings()[0].namespace(), "read");
        assert_eq!(audit.findings()[0].actual_symbol_count(), 2);
    }
}
