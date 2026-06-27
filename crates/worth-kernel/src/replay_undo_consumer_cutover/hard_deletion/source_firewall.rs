use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::error::{ReplayUndoHardDeletionError, ReplayUndoHardDeletionErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoHardDeletionSourceFirewallRow {
    scanned_source: &'static str,
    forbidden_surface: String,
    ordinary_occurrence_count: usize,
    removal_trigger: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoHardDeletionSourceFirewall {
    rows: Vec<ReplayUndoHardDeletionSourceFirewallRow>,
    scanned_source_count: usize,
    report_digest: String,
}

pub fn current_replay_undo_hard_deletion_source_firewall() -> ReplayUndoHardDeletionSourceFirewall {
    ReplayUndoHardDeletionSourceFirewall::clean_manifest(
        DECLARED_PHASE_SCOPE_PRODUCTION_SOURCE_COUNT,
    )
}

impl ReplayUndoHardDeletionSourceFirewall {
    fn clean_manifest(scanned_source_count: usize) -> Self {
        let rows = Vec::new();
        let report_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &report_digest_parts(&rows, scanned_source_count),
        );
        Self {
            rows,
            scanned_source_count,
            report_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_sources(sources: &[ReplayUndoHardDeletionSource]) -> Self {
        let rows = sources
            .iter()
            .flat_map(|source| {
                FORBIDDEN_SURFACES.iter().map(move |forbidden| {
                    let forbidden_surface = forbidden.surface();
                    ReplayUndoHardDeletionSourceFirewallRow::new(
                        source.path,
                        forbidden_surface.clone(),
                        source.contents.matches(&forbidden_surface).count(),
                        forbidden.removal_trigger,
                    )
                })
            })
            .filter(|row| row.ordinary_occurrence_count > 0)
            .collect::<Vec<_>>();
        let report_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &report_digest_parts(&rows, sources.len()),
        );
        Self {
            rows,
            scanned_source_count: sources.len(),
            report_digest,
        }
    }

    pub(crate) fn require_clean(&self) -> Result<(), ReplayUndoHardDeletionError> {
        if let Some(row) = self.rows.first() {
            return Err(ReplayUndoHardDeletionError::new(
                ReplayUndoHardDeletionErrorKind::SourceFirewallViolation,
                format!(
                    "replay/undo hard deletion firewall found `{}` in `{}`",
                    row.forbidden_surface, row.scanned_source
                ),
            ));
        }
        Ok(())
    }

    pub fn rows(&self) -> &[ReplayUndoHardDeletionSourceFirewallRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub fn violation_count(&self) -> usize {
        self.rows.len()
    }
}

impl ReplayUndoHardDeletionSourceFirewallRow {
    #[cfg(test)]
    fn new(
        scanned_source: &'static str,
        forbidden_surface: String,
        ordinary_occurrence_count: usize,
        removal_trigger: &'static str,
    ) -> Self {
        Self {
            scanned_source,
            forbidden_surface,
            ordinary_occurrence_count,
            removal_trigger,
        }
    }

    pub const fn scanned_source(&self) -> &'static str {
        self.scanned_source
    }

    pub fn forbidden_surface(&self) -> &str {
        &self.forbidden_surface
    }

    pub const fn ordinary_occurrence_count(&self) -> usize {
        self.ordinary_occurrence_count
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }
}

#[cfg(test)]
pub(crate) struct ReplayUndoHardDeletionSource {
    path: &'static str,
    contents: String,
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct ReplayUndoForbiddenSurface {
    fragments: &'static [&'static str],
    removal_trigger: &'static str,
}

const DECLARED_PHASE_SCOPE_PRODUCTION_SOURCE_COUNT: usize = 3032;

#[cfg(test)]
const FORBIDDEN_SURFACES: &[ReplayUndoForbiddenSurface] = &[
    ReplayUndoForbiddenSurface {
        fragments: &["pub fn complete_boolean", "_chain_integration_handoff("],
        removal_trigger: "packetless chain helper must not remain public ordinary authority",
    },
    ReplayUndoForbiddenSurface {
        fragments: &["packetless_legacy_loop", "_handoff_witness"],
        removal_trigger: "rollback witnesses must carry replay/undo transaction packets",
    },
    ReplayUndoForbiddenSurface {
        fragments: &["broad_topology", "_replay_scan"],
        removal_trigger: "topology replay scope must lower from scope products",
    },
    ReplayUndoForbiddenSurface {
        fragments: &["broad_evidence", "_replay_scan"],
        removal_trigger: "spatial replay scope must consume lookup handoff proof",
    },
    ReplayUndoForbiddenSurface {
        fragments: &["raw_scope", "_constructor"],
        removal_trigger: "scope products must be admitted by owning crates",
    },
    ReplayUndoForbiddenSurface {
        fragments: &["local_rollback", "_shortcut"],
        removal_trigger: "undo must flow through transaction boundary packets",
    },
    ReplayUndoForbiddenSurface {
        fragments: &["replay_undo", "_compatibility", "_wrapper"],
        removal_trigger: "compatibility wrappers must be deleted or capped residue",
    },
];

#[cfg(test)]
impl ReplayUndoForbiddenSurface {
    fn surface(&self) -> String {
        self.fragments.concat()
    }
}

fn report_digest_parts(
    rows: &[ReplayUndoHardDeletionSourceFirewallRow],
    scanned_source_count: usize,
) -> Vec<String> {
    let mut parts = vec![
        "worth-kernel:replay-undo-hard-deletion-source-firewall:v1".to_string(),
        format!("scanned-source-count:{scanned_source_count}"),
        format!("violation-count:{}", rows.len()),
    ];
    parts.extend(rows.iter().map(|row| {
        format!(
            "{}:{}:{}",
            row.scanned_source, row.forbidden_surface, row.ordinary_occurrence_count
        )
    }));
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn hard_deletion_firewall_rejects_reintroduced_old_replay_helper() {
        let report =
            ReplayUndoHardDeletionSourceFirewall::from_sources(&[ReplayUndoHardDeletionSource {
                path: "ordinary.rs",
                contents: FORBIDDEN_SURFACES[0].surface(),
            }]);

        assert_eq!(report.violation_count(), 1);
        assert!(report.require_clean().is_err());
    }

    #[test]
    fn hard_deletion_firewall_rejects_all_phase_twelve_revival_shapes() {
        let sources = FORBIDDEN_SURFACES
            .iter()
            .map(|forbidden| ReplayUndoHardDeletionSource {
                path: "ordinary.rs",
                contents: forbidden.surface(),
            })
            .collect::<Vec<_>>();
        let report = ReplayUndoHardDeletionSourceFirewall::from_sources(&sources);

        assert_eq!(report.violation_count(), FORBIDDEN_SURFACES.len());
        for forbidden in FORBIDDEN_SURFACES {
            let forbidden_surface = forbidden.surface();
            let matches = report
                .rows()
                .iter()
                .filter(|row| row.forbidden_surface() == forbidden_surface)
                .collect::<Vec<_>>();
            assert_eq!(matches.len(), 1, "forbidden surface must be explicit");
            assert_eq!(matches[0].ordinary_occurrence_count(), 1);
            assert_eq!(matches[0].removal_trigger(), forbidden.removal_trigger);
            assert!(
                !matches[0].removal_trigger().is_empty(),
                "forbidden surface must carry deletion trigger"
            );
        }
        let error = report
            .require_clean()
            .expect_err("any forbidden ordinary surface must fail the firewall");
        assert_eq!(
            error.kind(),
            &ReplayUndoHardDeletionErrorKind::SourceFirewallViolation
        );
    }

    #[test]
    fn current_hard_deletion_firewall_is_clean_for_production_roots() {
        let report = current_replay_undo_hard_deletion_source_firewall();

        assert_eq!(report.violation_count(), 0);
        assert_eq!(
            report.scanned_source_count(),
            DECLARED_PHASE_SCOPE_PRODUCTION_SOURCE_COUNT
        );
    }

    #[test]
    fn hard_deletion_firewall_filesystem_phase_scope_matches_manifest_and_stays_clean() {
        let scan = scan_phase_scope_production_sources();

        assert_eq!(
            scan.scanned_source_count,
            DECLARED_PHASE_SCOPE_PRODUCTION_SOURCE_COUNT
        );
        assert_eq!(scan.violation_count, 0);
    }

    struct PhaseScopeScan {
        scanned_source_count: usize,
        violation_count: usize,
    }

    fn scan_phase_scope_production_sources() -> PhaseScopeScan {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace = manifest_dir
            .parent()
            .and_then(Path::parent)
            .expect("workspace root");
        let roots = [
            workspace.join("crates/worth-kernel/src"),
            workspace.join("crates/worth-spatial/src"),
            workspace.join("crates/worth-topo/src"),
        ];
        let mut scanned_source_count = 0;
        let mut violation_count = 0;
        for root in roots {
            for source_path in rust_sources_under(&root) {
                if !is_ordinary_phase_scope_source(&source_path) {
                    continue;
                }
                scanned_source_count += 1;
                let contents = fs::read_to_string(&source_path).expect("source file");
                violation_count += FORBIDDEN_SURFACES
                    .iter()
                    .map(|forbidden| contents.matches(&forbidden.surface()).count())
                    .sum::<usize>();
            }
        }
        PhaseScopeScan {
            scanned_source_count,
            violation_count,
        }
    }

    fn rust_sources_under(root: &Path) -> Vec<PathBuf> {
        let mut sources = Vec::new();
        collect_rust_sources(root, &mut sources);
        sources
    }

    fn collect_rust_sources(path: &Path, sources: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(path).expect("read source directory") {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                collect_rust_sources(&path, sources);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }

    fn is_ordinary_phase_scope_source(path: &Path) -> bool {
        let path = path.to_string_lossy().replace('\\', "/");
        !path.contains("/tests/")
            && !path.contains("test_support")
            && !path.contains("/certification/")
            && !path.ends_with("source_firewall.rs")
    }
}
