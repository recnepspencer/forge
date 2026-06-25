use serde::Serialize;

use super::super::inventory::DerivedInvalidationAuthorityDisposition;
use super::super::inventory::{
    current_derived_invalidation_authority_inventory, DerivedInvalidationAuthorityInventoryCloseout,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDeletionSourceFirewallViolation {
    source_path: String,
    forbidden_surface: String,
    owner: String,
    removal_trigger: String,
}

impl DerivedInvalidationDeletionSourceFirewallViolation {
    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn forbidden_surface(&self) -> &str {
        &self.forbidden_surface
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub(crate) fn is_dirty_path_authority(&self) -> bool {
        self.forbidden_surface.contains("dirty")
            || self.forbidden_surface.contains("Dirty")
            || self.forbidden_surface.contains("fallback_count")
    }

    pub(crate) fn is_whole_view_rebuild_authority(&self) -> bool {
        self.forbidden_surface.contains("WholeView")
            || self.forbidden_surface.contains("whole_view")
            || self.forbidden_surface.contains("materialize")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDeletionSourceFirewall {
    violations: Vec<DerivedInvalidationDeletionSourceFirewallViolation>,
    scanned_source_count: usize,
    observed_pattern_count: usize,
    report_digest: String,
}

impl DerivedInvalidationDeletionSourceFirewall {
    pub(crate) fn from_current_sources() -> Self {
        match DerivedInvalidationAuthorityInventoryCloseout::close(
            current_derived_invalidation_authority_inventory(),
        ) {
            Ok(inventory_closeout) => Self::from_inventory_closeout(&inventory_closeout),
            Err(error) => Self::from_closeout_error(error.to_string()),
        }
    }

    pub(crate) fn from_inventory_closeout(
        inventory_closeout: &DerivedInvalidationAuthorityInventoryCloseout,
    ) -> Self {
        let source_scan = inventory_closeout.source_scan();
        let mut violations = source_scan
            .uncovered_patterns()
            .iter()
            .map(
                |pattern| DerivedInvalidationDeletionSourceFirewallViolation {
                    source_path: "derived-invalidation-source-scan".to_string(),
                    forbidden_surface: pattern.to_string(),
                    owner: "derived-invalidation deletion closeout".to_string(),
                    removal_trigger: "add explicit inventory disposition or delete old authority"
                        .to_string(),
                },
            )
            .collect::<Vec<_>>();
        violations.extend(
            inventory_closeout
                .inventory()
                .rows()
                .iter()
                .filter(|row| row.ordinary_path())
                .filter_map(|row| match row.disposition() {
                    DerivedInvalidationAuthorityDisposition::Migrate => {
                        Some(DerivedInvalidationDeletionSourceFirewallViolation {
                            source_path: row.source_path().to_string(),
                            forbidden_surface: row.surface().to_string(),
                            owner: row.owner().as_str().to_string(),
                            removal_trigger:
                                "ordinary migration row cannot satisfy final deletion closeout"
                                    .to_string(),
                        })
                    }
                    DerivedInvalidationAuthorityDisposition::Delete => {
                        lingering_deleted_surface_violation(row)
                    }
                    DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue
                    | DerivedInvalidationAuthorityDisposition::TrueQueryCapabilityGap => None,
                }),
        );
        Self::from_parts(
            violations,
            source_scan.scanned_source_count(),
            source_scan.observed_pattern_count(),
        )
    }

    fn from_closeout_error(reason: String) -> Self {
        Self::from_parts(
            vec![DerivedInvalidationDeletionSourceFirewallViolation {
                source_path: "derived-invalidation-inventory-closeout".to_string(),
                forbidden_surface: reason,
                owner: "derived-invalidation deletion closeout".to_string(),
                removal_trigger: "fix inventory source scan before deletion closeout".to_string(),
            }],
            0,
            0,
        )
    }

    #[cfg(test)]
    fn from_source_inputs(sources: impl IntoIterator<Item = SourceInput>) -> Self {
        let sources = sources.into_iter().collect::<Vec<_>>();
        let scanned_source_count = sources.len();
        let violations = collect_forbidden_source_violations(sources);
        let observed_pattern_count = violations.len();
        Self::from_parts(violations, scanned_source_count, observed_pattern_count)
    }

    fn from_parts(
        violations: Vec<DerivedInvalidationDeletionSourceFirewallViolation>,
        scanned_source_count: usize,
        observed_pattern_count: usize,
    ) -> Self {
        let mut parts = vec![
            "worth-topo:derived-invalidation-deletion-source-firewall:v1".to_string(),
            format!("violations:{}", violations.len()),
            format!("scanned-sources:{scanned_source_count}"),
            format!("observed-patterns:{observed_pattern_count}"),
        ];
        parts.extend(violations.iter().map(|violation| {
            format!(
                "violation:{}:{}",
                violation.source_path, violation.forbidden_surface
            )
        }));
        let report_digest = super::super::catalog::catalog_digest(parts);
        Self {
            violations,
            scanned_source_count,
            observed_pattern_count,
            report_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_sources_for_tests(
        sources: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> Self {
        Self::from_source_inputs(
            sources
                .into_iter()
                .map(|(path, contents)| SourceInput { path, contents }),
        )
    }

    pub fn violations(&self) -> &[DerivedInvalidationDeletionSourceFirewallViolation] {
        &self.violations
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub const fn observed_pattern_count(&self) -> usize {
        self.observed_pattern_count
    }
}

pub fn current_deletion_source_firewall() -> DerivedInvalidationDeletionSourceFirewall {
    DerivedInvalidationDeletionSourceFirewall::from_current_sources()
}

fn lingering_deleted_surface_violation(
    row: &super::super::inventory::DerivedInvalidationAuthorityInventoryRow,
) -> Option<DerivedInvalidationDeletionSourceFirewallViolation> {
    let source_path = row
        .source_path()
        .strip_prefix("crates/worth-topo/")
        .unwrap_or_else(|| row.source_path());
    let absolute_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(source_path);
    let contents = std::fs::read_to_string(absolute_path).ok()?;
    if !contents.contains(row.surface()) {
        return None;
    }
    Some(DerivedInvalidationDeletionSourceFirewallViolation {
        source_path: row.source_path().to_string(),
        forbidden_surface: row.surface().to_string(),
        owner: row.owner().as_str().to_string(),
        removal_trigger: "delete old ordinary authority source instead of merely classifying it"
            .to_string(),
    })
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct SourceInput {
    path: &'static str,
    contents: &'static str,
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct ForbiddenPattern {
    forbidden_surface: &'static str,
    owner: &'static str,
    removal_trigger: &'static str,
}

#[cfg(test)]
fn collect_forbidden_source_violations(
    sources: impl IntoIterator<Item = SourceInput>,
) -> Vec<DerivedInvalidationDeletionSourceFirewallViolation> {
    sources
        .into_iter()
        .flat_map(|source| {
            forbidden_patterns()
                .into_iter()
                .filter(move |pattern| source.contents.contains(pattern.forbidden_surface))
                .map(
                    move |pattern| DerivedInvalidationDeletionSourceFirewallViolation {
                        source_path: source.path.to_string(),
                        forbidden_surface: pattern.forbidden_surface.to_string(),
                        owner: pattern.owner.to_string(),
                        removal_trigger: pattern.removal_trigger.to_string(),
                    },
                )
        })
        .collect()
}

#[cfg(test)]
fn forbidden_patterns() -> [ForbiddenPattern; 6] {
    [
        ForbiddenPattern {
            forbidden_surface: "operator_dirty_products",
            owner: "derived-invalidation deletion closeout",
            removal_trigger: "delete operator-authored dirty product paths",
        },
        ForbiddenPattern {
            forbidden_surface: "dirty_product_expectations",
            owner: "derived-invalidation deletion closeout",
            removal_trigger: "delete local derived-product expectation arrays",
        },
        ForbiddenPattern {
            forbidden_surface: "derived_expectation_array",
            owner: "derived-invalidation deletion closeout",
            removal_trigger: "consume covered product migration rows instead",
        },
        ForbiddenPattern {
            forbidden_surface: "expand_dirty_scope",
            owner: "projection read-stage deletion closeout",
            removal_trigger: "route dirty expansion through selected invalidation plans",
        },
        ForbiddenPattern {
            forbidden_surface: "fallback_policy_accepted_as_invalidation",
            owner: "operator closeout deletion closeout",
            removal_trigger: "require Milestone 10 invalidation receipts",
        },
        ForbiddenPattern {
            forbidden_surface: "old_dirty_data_to_invalidation_receipt",
            owner: "derived-invalidation deletion closeout",
            removal_trigger: "mint receipts only from selected invalidation plan execution",
        },
    ]
}
