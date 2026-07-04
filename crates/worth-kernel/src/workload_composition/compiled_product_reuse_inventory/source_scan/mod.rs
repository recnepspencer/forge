mod matcher;
mod scope;

use std::collections::BTreeMap;
use std::path::Path;

use super::error::CompiledProductReuseInventoryError;
use super::report::CompiledProductReuseInventoryReport;

pub(crate) use scope::workspace_root;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CompiledProductReuseScanPattern {
    ReuseIdentifier,
    EquivalenceIdentifier,
    ParityIdentifier,
    RebuildSuppressionLine,
    RowCountShortcutLine,
    RenderedShapeEqualityLine,
    PointerIdentityLine,
    RetainedFolkloreIdentifier,
    PublicReadModelReuseLine,
    LookupConsumerIdentifier,
    CloseoutConsumerIdentifier,
}

impl CompiledProductReuseScanPattern {
    pub const fn all() -> [Self; 11] {
        [
            Self::ReuseIdentifier,
            Self::EquivalenceIdentifier,
            Self::ParityIdentifier,
            Self::RebuildSuppressionLine,
            Self::RowCountShortcutLine,
            Self::RenderedShapeEqualityLine,
            Self::PointerIdentityLine,
            Self::RetainedFolkloreIdentifier,
            Self::PublicReadModelReuseLine,
            Self::LookupConsumerIdentifier,
            Self::CloseoutConsumerIdentifier,
        ]
    }

    pub const fn pattern(self) -> &'static str {
        match self {
            Self::ReuseIdentifier => "reuse helper identifier",
            Self::EquivalenceIdentifier => "equivalence helper identifier",
            Self::ParityIdentifier => "parity helper identifier",
            Self::RebuildSuppressionLine => "rebuild-suppression line",
            Self::RowCountShortcutLine => "row-count shortcut line",
            Self::RenderedShapeEqualityLine => "rendered-shape equality line",
            Self::PointerIdentityLine => "pointer identity line",
            Self::RetainedFolkloreIdentifier => "retained folklore identifier",
            Self::PublicReadModelReuseLine => "public read-model reuse line",
            Self::LookupConsumerIdentifier => "lookup-consumed consumer identifier",
            Self::CloseoutConsumerIdentifier => "closeout consumer identifier",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledProductReuseSourceScanReport {
    scanned_file_count: usize,
    uncovered_pattern_count: usize,
    uncovered_patterns: Vec<String>,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CompiledProductReuseScanScopeReport {
    scanned_file_count: usize,
    scanned_relative_paths: Vec<String>,
}

impl CompiledProductReuseSourceScanReport {
    pub fn from_inventory(
        inventory: &CompiledProductReuseInventoryReport,
    ) -> Result<Self, CompiledProductReuseInventoryError> {
        Self::from_inventory_with_workspace_root(inventory, &workspace_root()?)
    }

    pub(crate) fn from_inventory_with_workspace_root(
        inventory: &CompiledProductReuseInventoryReport,
        workspace_root: &Path,
    ) -> Result<Self, CompiledProductReuseInventoryError> {
        let observed = scan_scope_sources(workspace_root)?;
        let allowed = allowed_pattern_counts(inventory);
        let uncovered_patterns = observed
            .pattern_counts
            .into_iter()
            .filter_map(|((path, pattern), observed_count)| {
                let allowed_count = allowed.get(&(path.clone(), pattern)).copied().unwrap_or(0);
                (observed_count > allowed_count).then(|| {
                    format!(
                        "{path} contains {} uncovered `{}` occurrence(s)",
                        observed_count - allowed_count,
                        pattern.pattern()
                    )
                })
            })
            .collect::<Vec<_>>();

        Ok(Self {
            scanned_file_count: observed.scanned_file_count,
            uncovered_pattern_count: uncovered_patterns.len(),
            uncovered_patterns,
        })
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub const fn uncovered_pattern_count(&self) -> usize {
        self.uncovered_pattern_count
    }

    pub fn uncovered_patterns(&self) -> &[String] {
        &self.uncovered_patterns
    }
}

#[cfg(test)]
impl CompiledProductReuseScanScopeReport {
    pub(crate) fn from_workspace_root(
        workspace_root: &Path,
    ) -> Result<Self, CompiledProductReuseInventoryError> {
        let scoped_files = scope::scope_files(workspace_root)?;
        Ok(Self {
            scanned_file_count: scoped_files.len(),
            scanned_relative_paths: scoped_files
                .into_iter()
                .map(|(_, relative_path)| relative_path)
                .collect(),
        })
    }

    pub(crate) const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub(crate) fn scanned_relative_paths(&self) -> &[String] {
        &self.scanned_relative_paths
    }
}

struct ObservedPatterns {
    scanned_file_count: usize,
    pattern_counts: BTreeMap<(String, CompiledProductReuseScanPattern), usize>,
}

fn allowed_pattern_counts(
    inventory: &CompiledProductReuseInventoryReport,
) -> BTreeMap<(String, CompiledProductReuseScanPattern), usize> {
    let mut allowed = BTreeMap::<(String, CompiledProductReuseScanPattern), usize>::new();
    for row in inventory.rows() {
        *allowed
            .entry((row.source_path().replace('\\', "/"), row.scan_pattern()))
            .or_default() += 1;
        if let Some(pattern) = row.secondary_scan_pattern() {
            *allowed
                .entry((row.source_path().replace('\\', "/"), pattern))
                .or_default() += 1;
        }
    }
    allowed
}

fn scan_scope_sources(
    workspace_root: &Path,
) -> Result<ObservedPatterns, CompiledProductReuseInventoryError> {
    let mut observed = ObservedPatterns {
        scanned_file_count: 0,
        pattern_counts: BTreeMap::new(),
    };
    for (path, relative_path) in scope::scope_files(workspace_root)? {
        matcher::scan_file(&path, &relative_path, &mut observed)?;
    }
    Ok(observed)
}
