use crate::domain_capabilities::identity::{
    compose_golden_transcript_digest, compose_target_dx_digest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainCapabilityGoldenTranscript {
    label: &'static str,
    path: &'static str,
    dx_focus: &'static str,
}

impl WorthQueryDomainCapabilityGoldenTranscript {
    pub(crate) const fn new(
        label: &'static str,
        path: &'static str,
        dx_focus: &'static str,
    ) -> Self {
        Self {
            label,
            path,
            dx_focus,
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn dx_focus(&self) -> &'static str {
        self.dx_focus
    }
}

const GOLDEN_TRANSCRIPTS: [WorthQueryDomainCapabilityGoldenTranscript; 11] = [
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "common_declaration_lane",
        "tests/ui/domain_capabilities/golden/domain_capability_common_declaration_lane_golden_transcript_compiles.rs",
        "ordinary declaration-bound domain capability authoring",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "admission_materialization",
        "tests/ui/domain_capabilities/golden/domain_capability_admission_contribution_materialization_compiles.rs",
        "admitted-plan admission through the common lane",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "support_materialization",
        "tests/ui/domain_capabilities/golden/domain_capability_support_contribution_materialization_compiles.rs",
        "declaration-bound support traceability through the common lane",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "invariant_materialization",
        "tests/ui/domain_capabilities/golden/domain_capability_invariant_contribution_materialization_compiles.rs",
        "declaration-bound invariant registration through the common lane",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "query_invariant_lowering",
        "tests/ui/domain_capabilities/golden/domain_capability_query_invariant_registration_lowering_compiles.rs",
        "ordinary Query invariant registration facade lowering",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "workflow_materialization",
        "tests/ui/domain_capabilities/golden/domain_capability_workflow_contribution_materialization_compiles.rs",
        "preview workflow planning through the ordinary lane",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "continuity_materialization",
        "tests/ui/domain_capabilities/golden/domain_capability_continuity_contribution_materialization_compiles.rs",
        "continuity closure through the admitted-plan common lane",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "aftermath_materialization",
        "tests/ui/domain_capabilities/golden/domain_capability_aftermath_contribution_materialization_compiles.rs",
        "aftermath projection contract materialization through the common lane",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "explanation_materialization",
        "tests/ui/domain_capabilities/golden/domain_capability_explanation_contribution_materialization_compiles.rs",
        "explanation contribution materialization through the common lane",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "certification_surface_readout",
        "tests/ui/domain_capabilities/golden/domain_capability_certification_surface_readout_compiles.rs",
        "named certification surface readout through the public facade",
    ),
    WorthQueryDomainCapabilityGoldenTranscript::new(
        "lower_runtime_boundary_source_contributions",
        "tests/ui/domain_capabilities/golden/lower_runtime_boundary_source_contributions_compile.rs",
        "lower-runtime contribution binding from real boundary receipt sources",
    ),
];

pub fn worth_query_domain_capability_golden_transcripts(
) -> &'static [WorthQueryDomainCapabilityGoldenTranscript] {
    &GOLDEN_TRANSCRIPTS
}

pub fn worth_query_domain_capability_target_dx_digest() -> String {
    compose_target_dx_digest(worth_query_domain_capability_golden_transcripts())
}

pub fn worth_query_domain_capability_golden_transcript_digest() -> String {
    compose_golden_transcript_digest(worth_query_domain_capability_golden_transcripts())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn manifest_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ui/domain_capabilities/golden")
    }

    #[test]
    fn golden_transcript_manifest_is_duplicate_free_and_nonempty() {
        let rows = worth_query_domain_capability_golden_transcripts();
        let labels = rows.iter().map(|row| row.label()).collect::<Vec<_>>();
        let paths = rows.iter().map(|row| row.path()).collect::<Vec<_>>();

        assert_eq!(rows.len(), 11);
        assert_eq!(
            labels.len(),
            labels.iter().copied().collect::<BTreeSet<_>>().len()
        );
        assert_eq!(
            paths.len(),
            paths.iter().copied().collect::<BTreeSet<_>>().len()
        );
        assert!(rows.iter().all(|row| !row.dx_focus().is_empty()));
    }

    #[test]
    fn target_dx_and_golden_transcript_digests_remain_distinct() {
        assert_ne!(
            worth_query_domain_capability_target_dx_digest(),
            worth_query_domain_capability_golden_transcript_digest()
        );
    }

    #[test]
    fn golden_transcript_manifest_matches_checked_in_golden_suite() {
        let expected = worth_query_domain_capability_golden_transcripts()
            .iter()
            .map(|row| row.path())
            .collect::<BTreeSet<_>>();
        let actual = fs::read_dir(manifest_root())
            .expect("golden transcript directory should exist")
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("rs"))
            .map(|path| {
                path.strip_prefix(Path::new(env!("CARGO_MANIFEST_DIR")))
                    .expect("golden path should live under crate manifest dir")
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect::<BTreeSet<_>>();

        assert_eq!(
            expected,
            actual.iter().map(String::as_str).collect::<BTreeSet<_>>()
        );
    }
}
