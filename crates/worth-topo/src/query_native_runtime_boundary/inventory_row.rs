use super::residue_status::WorthTopologyQueryNativeRuntimeBoundaryResidueStatus;
use super::stale_symbol::WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyQueryNativeRuntimeBoundaryInventoryRow {
    source_path: String,
    stale_symbol: WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol,
    observed_count: usize,
    status: WorthTopologyQueryNativeRuntimeBoundaryResidueStatus,
    owner: &'static str,
    replacement_boundary: &'static str,
    blocker: &'static str,
    removal_trigger: &'static str,
    row_digest: String,
}

impl WorthTopologyQueryNativeRuntimeBoundaryInventoryRow {
    pub(crate) fn new(
        source_path: impl Into<String>,
        stale_symbol: WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol,
        observed_count: usize,
    ) -> Self {
        let source_path = source_path.into();
        let classification = classify_stale_symbol_occurrence(&source_path, stale_symbol);
        let row_digest = format!(
            "worth-topo-query-native-runtime-boundary-row-v1|{}|{}|{}|{}|{}|{}|{}|{}",
            source_path,
            stale_symbol.as_str(),
            observed_count,
            classification.status.as_str(),
            classification.owner,
            classification.replacement_boundary,
            classification.blocker,
            classification.removal_trigger
        );
        Self {
            source_path,
            stale_symbol,
            observed_count,
            status: classification.status,
            owner: classification.owner,
            replacement_boundary: classification.replacement_boundary,
            blocker: classification.blocker,
            removal_trigger: classification.removal_trigger,
            row_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn unclassified_for_test(
        source_path: impl Into<String>,
        stale_symbol: WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol,
    ) -> Self {
        let source_path = source_path.into();
        Self {
            row_digest: format!(
                "worth-topo-query-native-runtime-boundary-row-v1|{}|{}|unclassified",
                source_path,
                stale_symbol.as_str()
            ),
            source_path,
            stale_symbol,
            observed_count: 1,
            status: WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::Unclassified,
            owner: "",
            replacement_boundary: "",
            blocker: "",
            removal_trigger: "",
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn stale_symbol(&self) -> WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol {
        self.stale_symbol
    }

    pub const fn observed_count(&self) -> usize {
        self.observed_count
    }

    pub const fn status(&self) -> WorthTopologyQueryNativeRuntimeBoundaryResidueStatus {
        self.status
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn replacement_boundary(&self) -> &'static str {
        self.replacement_boundary
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub fn looks_like_compatibility_shim(&self) -> bool {
        let source_path = self.source_path();
        source_path.contains("compat")
            || source_path.contains("shim")
            || source_path.contains("external_projection")
            || source_path.contains("external_row")
    }
}

struct StaleSymbolClassification {
    status: WorthTopologyQueryNativeRuntimeBoundaryResidueStatus,
    owner: &'static str,
    replacement_boundary: &'static str,
    blocker: &'static str,
    removal_trigger: &'static str,
}

fn classify_stale_symbol_occurrence(
    source_path: &str,
    stale_symbol: WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol,
) -> StaleSymbolClassification {
    let status = classify_status(source_path, stale_symbol);
    StaleSymbolClassification {
        status,
        owner: "worth-topo",
        replacement_boundary: replacement_boundary_for(source_path, stale_symbol),
        blocker: blocker_for(status),
        removal_trigger: removal_trigger_for(status),
    }
}

fn classify_status(
    source_path: &str,
    stale_symbol: WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol,
) -> WorthTopologyQueryNativeRuntimeBoundaryResidueStatus {
    if source_path.contains("source_firewall.rs") {
        return WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::FirewallPatternOnly;
    }
    if source_path.contains("certification/support/historical_query_snapshot") {
        return WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::TerminalSupportCodecOnly;
    }
    if source_path.starts_with("certification/") {
        return WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::CertificationSupportCutover;
    }
    if source_path.contains("query_runtime/adapters/write_authority")
        || stale_symbol
            == WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol::CallerBuiltWriteCommand
    {
        return WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::MigrateToNativeBoundary;
    }
    if source_path.contains("query_runtime/adapters")
        || source_path.contains("query_support")
        || source_path.contains("read_execution")
        || source_path.contains("derived_topology/materialized_graph")
        || source_path.contains("operator_bindings")
        || source_path.contains("declared_query_surfaces")
        || source_path.contains("construction/query_native_boundary")
    {
        return WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::MigrateToNativeBoundary;
    }
    WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::Unclassified
}

fn replacement_boundary_for(
    source_path: &str,
    stale_symbol: WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol,
) -> &'static str {
    if source_path.contains("write_authority")
        || stale_symbol
            == WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol::CallerBuiltWriteCommand
    {
        "query-native-runtime-boundary/backend-admissible-write-authority"
    } else if source_path.contains("existing_truth") || source_path.contains("field_value") {
        "query-native-runtime-boundary/native-existing-truth-and-retained-fields"
    } else if stale_symbol
        == WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol::LiveViewNameRouting
    {
        "query-native-runtime-boundary/live-artifact-target-routing"
    } else if source_path.contains("certification/") {
        "query-native-runtime-boundary/certification-cutover"
    } else {
        "query-native-runtime-boundary/native-entity-rows-and-read-decode"
    }
}

fn blocker_for(status: WorthTopologyQueryNativeRuntimeBoundaryResidueStatus) -> &'static str {
    match status {
        WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::TerminalSupportCodecOnly => {
            "terminal report codec must be named before JSON projection can remain"
        }
        WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::FirewallPatternOnly => {
            "firewall literals must survive until the hard-deletion phase owns them"
        }
        WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::CertificationSupportCutover => {
            "certification must consume the new native runtime boundary first"
        }
        WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::MigrateToNativeBoundary
        | WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::DeleteWithNativeReplacement => {
            "native Query carrier replacement must be built in Milestone 9.1"
        }
        WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::ExplicitUpstreamBlocker => {
            "upstream Query capability must be named before Worth may proceed"
        }
        WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::Unclassified => "",
    }
}

fn removal_trigger_for(
    status: WorthTopologyQueryNativeRuntimeBoundaryResidueStatus,
) -> &'static str {
    match status {
        WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::FirewallPatternOnly => {
            "Phase 8 source firewall replaces literal ownership with hard-deletion report"
        }
        WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::TerminalSupportCodecOnly => {
            "terminal support document codec is isolated and ordinary runtime source is clean"
        }
        _ => "native runtime boundary cutover compiles without stale Query terminal API usage",
    }
}
