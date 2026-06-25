use serde::Serialize;

use super::classification::DerivedInvalidationOldAuthorityKind;
use super::error::DerivedInvalidationAuthorityInventoryError;
use super::report::DerivedInvalidationAuthorityInventoryReport;
use super::row::{digest_strings, DerivedInvalidationAuthorityInventoryRow};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationSourceScanReport {
    scanned_source_count: usize,
    observed_pattern_count: usize,
    uncovered_pattern_count: usize,
    uncovered_patterns: Vec<String>,
    report_digest: String,
}

impl DerivedInvalidationSourceScanReport {
    pub(crate) fn from_inventory(
        inventory: &DerivedInvalidationAuthorityInventoryReport,
    ) -> Result<Self, DerivedInvalidationAuthorityInventoryError> {
        DerivedInvalidationSourceCorpus::worth_topo_default()
            .scan_against_inventory(inventory.rows())
    }

    pub(crate) fn from_observed_patterns(
        rows: &[DerivedInvalidationAuthorityInventoryRow],
        observed_patterns: Vec<ObservedDerivedInvalidationPattern>,
        scanned_source_count: usize,
    ) -> Self {
        let uncovered_patterns = observed_patterns
            .iter()
            .filter(|pattern| !row_covers_pattern(rows, pattern))
            .map(|pattern| {
                format!(
                    "{}::{:?}::{}",
                    pattern.source_path, pattern.authority_class, pattern.pattern
                )
            })
            .collect::<Vec<_>>();
        let report_digest = digest_strings(
            observed_patterns
                .iter()
                .map(|pattern| format!("observed:{}:{}", pattern.source_path, pattern.pattern))
                .chain(
                    uncovered_patterns
                        .iter()
                        .map(|pattern| format!("uncovered:{pattern}")),
                )
                .collect(),
        );
        Self {
            scanned_source_count,
            observed_pattern_count: observed_patterns.len(),
            uncovered_pattern_count: uncovered_patterns.len(),
            uncovered_patterns,
            report_digest,
        }
    }

    pub fn uncovered_pattern_count(&self) -> usize {
        self.uncovered_pattern_count
    }

    pub fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub fn observed_pattern_count(&self) -> usize {
        self.observed_pattern_count
    }

    pub fn uncovered_patterns(&self) -> &[String] {
        &self.uncovered_patterns
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ObservedDerivedInvalidationPattern {
    source_path: &'static str,
    pattern: &'static str,
    authority_class: DerivedInvalidationForbiddenAuthorityClass,
}

pub(crate) struct DerivedInvalidationSourceCorpus {
    sources: Vec<DerivedInvalidationSourceCorpusEntry>,
}

struct DerivedInvalidationSourceCorpusEntry {
    source_path: &'static str,
    contents: DerivedInvalidationSourceContents,
}

enum DerivedInvalidationSourceContents {
    WorkspaceFile,
    #[cfg(test)]
    Inline(&'static str),
}

impl DerivedInvalidationSourceCorpus {
    pub(crate) fn worth_topo_default() -> Self {
        Self {
            sources: DERIVED_INVALIDATION_SCAN_SOURCES
                .iter()
                .map(|source_path| DerivedInvalidationSourceCorpusEntry {
                    source_path,
                    contents: DerivedInvalidationSourceContents::WorkspaceFile,
                })
                .collect(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_inline_sources(sources: Vec<(&'static str, &'static str)>) -> Self {
        Self {
            sources: sources
                .into_iter()
                .map(
                    |(source_path, contents)| DerivedInvalidationSourceCorpusEntry {
                        source_path,
                        contents: DerivedInvalidationSourceContents::Inline(contents),
                    },
                )
                .collect(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_workspace_sources(sources: Vec<&'static str>) -> Self {
        Self {
            sources: sources
                .into_iter()
                .map(|source_path| DerivedInvalidationSourceCorpusEntry {
                    source_path,
                    contents: DerivedInvalidationSourceContents::WorkspaceFile,
                })
                .collect(),
        }
    }

    pub(crate) fn scan_against_inventory(
        &self,
        rows: &[DerivedInvalidationAuthorityInventoryRow],
    ) -> Result<DerivedInvalidationSourceScanReport, DerivedInvalidationAuthorityInventoryError>
    {
        let observed_patterns = self.scan_patterns()?;
        Ok(DerivedInvalidationSourceScanReport::from_observed_patterns(
            rows,
            observed_patterns,
            self.sources.len(),
        ))
    }

    fn scan_patterns(
        &self,
    ) -> Result<Vec<ObservedDerivedInvalidationPattern>, DerivedInvalidationAuthorityInventoryError>
    {
        let mut observed = Vec::new();
        for source in &self.sources {
            let contents = source.contents.read_source(source.source_path)?;
            for pattern in DERIVED_INVALIDATION_PATTERNS {
                if contents.contains(pattern.token) {
                    observed.push(ObservedDerivedInvalidationPattern {
                        source_path: source.source_path,
                        pattern: pattern.token,
                        authority_class: pattern.authority_class,
                    });
                }
            }
        }
        Ok(observed)
    }
}

impl DerivedInvalidationSourceContents {
    fn read_source(
        &self,
        source_path: &'static str,
    ) -> Result<String, DerivedInvalidationAuthorityInventoryError> {
        match self {
            Self::WorkspaceFile => {
                let absolute_path =
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(source_path);
                Ok(std::fs::read_to_string(absolute_path).unwrap_or_default())
            }
            #[cfg(test)]
            Self::Inline(contents) => Ok((*contents).to_string()),
        }
    }
}

pub(crate) fn scan_current_derived_invalidation_sources(
    inventory: &DerivedInvalidationAuthorityInventoryReport,
) -> Result<DerivedInvalidationSourceScanReport, DerivedInvalidationAuthorityInventoryError> {
    DerivedInvalidationSourceScanReport::from_inventory(inventory)
}

const DERIVED_INVALIDATION_SCAN_SOURCES: &[&str] = &[
    "src/derived_topology/materialized_graph/mod.rs",
    "src/derived_topology/materialized_graph/types.rs",
    "src/derived_topology/traversal_views/facade.rs",
    "src/derived_topology/invalidation_plan/migrated_products/loop_cycles/old_authority_residue.rs",
    "src/derived_topology/invalidation_plan/migrated_products/radial_rings/old_authority_residue.rs",
    "src/derived_topology/invalidation_plan/migrated_products/shell_views/legacy_interpretation.rs",
    "src/derived_topology/invalidation_plan/migrated_products/vertex_disks/legacy_interpretation.rs",
    "src/derived_topology/invalidation_plan/migrated_products/wire_views/legacy_interpretation.rs",
    "src/derived_topology/traversal_views/shell_compatibility.rs",
    "src/derived_topology/traversal_views/vertex_disk_compatibility.rs",
    "src/derived_topology/traversal_views/wire_compatibility.rs",
    "src/projection/runtime_boundary/read_stage.rs",
    "src/certification/topology_operator_closeout/derived_fallout/derived_work_breadth_rows.rs",
    "src/certification/topology_operator_closeout/derived_fallout/derived_work_breadth.rs",
    "src/certification/topology_operator_closeout/derived_fallout/fallback_policy_denial_rows.rs",
    "src/certification/topology_operator_closeout/derived_fallout/fallback_policy_denial.rs",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DerivedInvalidationForbiddenAuthorityPattern {
    token: &'static str,
    authority_class: DerivedInvalidationForbiddenAuthorityClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DerivedInvalidationForbiddenAuthorityClass {
    WholeViewRebuild,
    ProjectionExpansion,
    OperatorCloseoutBreadth,
    TraversalInterpretation,
    DirtyProducer,
    LocalMaintenanceHook,
}

const DERIVED_INVALIDATION_PATTERNS: &[DerivedInvalidationForbiddenAuthorityPattern] = &[
    pattern(
        "WholeViewRebuild",
        DerivedInvalidationForbiddenAuthorityClass::WholeViewRebuild,
    ),
    pattern(
        "whole_view_materialization",
        DerivedInvalidationForbiddenAuthorityClass::WholeViewRebuild,
    ),
    pattern(
        "materialize_from_truth",
        DerivedInvalidationForbiddenAuthorityClass::WholeViewRebuild,
    ),
    pattern(
        "materialize_query_input",
        DerivedInvalidationForbiddenAuthorityClass::WholeViewRebuild,
    ),
    pattern(
        "stage_topology_read_from_view",
        DerivedInvalidationForbiddenAuthorityClass::ProjectionExpansion,
    ),
    pattern(
        "derived_region_count",
        DerivedInvalidationForbiddenAuthorityClass::OperatorCloseoutBreadth,
    ),
    pattern(
        "fallback_count",
        DerivedInvalidationForbiddenAuthorityClass::OperatorCloseoutBreadth,
    ),
    pattern(
        "interpret_topology_view",
        DerivedInvalidationForbiddenAuthorityClass::TraversalInterpretation,
    ),
    pattern(
        "interpret_wires",
        DerivedInvalidationForbiddenAuthorityClass::TraversalInterpretation,
    ),
    pattern(
        "interpret_shells",
        DerivedInvalidationForbiddenAuthorityClass::TraversalInterpretation,
    ),
    pattern(
        "interpret_radial_surface",
        DerivedInvalidationForbiddenAuthorityClass::TraversalInterpretation,
    ),
    pattern(
        "interpret_shell_radial_surface",
        DerivedInvalidationForbiddenAuthorityClass::TraversalInterpretation,
    ),
    pattern(
        "interpret_wire_branching",
        DerivedInvalidationForbiddenAuthorityClass::TraversalInterpretation,
    ),
    pattern(
        "interpret_boundaries",
        DerivedInvalidationForbiddenAuthorityClass::TraversalInterpretation,
    ),
    pattern(
        "dirty_products",
        DerivedInvalidationForbiddenAuthorityClass::DirtyProducer,
    ),
    pattern(
        "local_derived_maintenance",
        DerivedInvalidationForbiddenAuthorityClass::LocalMaintenanceHook,
    ),
];

const fn pattern(
    token: &'static str,
    authority_class: DerivedInvalidationForbiddenAuthorityClass,
) -> DerivedInvalidationForbiddenAuthorityPattern {
    DerivedInvalidationForbiddenAuthorityPattern {
        token,
        authority_class,
    }
}

fn row_covers_pattern(
    rows: &[DerivedInvalidationAuthorityInventoryRow],
    pattern: &ObservedDerivedInvalidationPattern,
) -> bool {
    let normalized_path = format!("crates/worth-topo/{}", pattern.source_path);
    rows.iter().any(|row| {
        row.source_path() == normalized_path && row_authority_covers_pattern(row, pattern.pattern)
    })
}

fn row_authority_covers_pattern(
    row: &DerivedInvalidationAuthorityInventoryRow,
    pattern: &str,
) -> bool {
    row.surface().contains(pattern)
        || row.authority_kind().as_str() == pattern
        || authority_kind_aliases(row).contains(&pattern)
}

fn authority_kind_aliases(
    row: &DerivedInvalidationAuthorityInventoryRow,
) -> &'static [&'static str] {
    match row.authority_kind() {
        DerivedInvalidationOldAuthorityKind::WholeViewMaterialization => &[
            "WholeViewRebuild",
            "whole_view_materialization",
            "materialize_from_truth",
        ],
        DerivedInvalidationOldAuthorityKind::QueryInputMaterialization => {
            &["materialize_query_input"]
        }
        DerivedInvalidationOldAuthorityKind::TraversalInterpretation => &[
            "interpret_topology_view",
            "interpret_wires",
            "interpret_shells",
            "interpret_radial_surface",
            "interpret_shell_radial_surface",
            "interpret_wire_branching",
            "interpret_boundaries",
        ],
        DerivedInvalidationOldAuthorityKind::ProjectionReadStage => {
            &["stage_topology_read_from_view", "materialize_from_truth"]
        }
        DerivedInvalidationOldAuthorityKind::OperatorDerivedBreadthCloseout => {
            &["derived_region_count", "fallback_count"]
        }
        DerivedInvalidationOldAuthorityKind::FallbackPolicyDenial => {
            &["fallback_count", "observed_fallback_count"]
        }
        DerivedInvalidationOldAuthorityKind::DerivedValidationDiagnostic
        | DerivedInvalidationOldAuthorityKind::TestOnlyWholeViewFixture
        | DerivedInvalidationOldAuthorityKind::CertificationBootstrapMaterialization => &[],
    }
}
