use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeGoldenTranscript {
    label: &'static str,
    path: &'static str,
    dx_focus: &'static str,
}

impl WorthQueryLowerRuntimeGoldenTranscript {
    const fn new(label: &'static str, path: &'static str, dx_focus: &'static str) -> Self {
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

const GOLDEN_TRANSCRIPTS: [WorthQueryLowerRuntimeGoldenTranscript; 3] = [
    WorthQueryLowerRuntimeGoldenTranscript::new(
        "common_path_closeout",
        "tests/ui/lower_runtime_routing/golden/lower_runtime_routing_common_path_closeout_golden_transcript_compiles.rs",
        "common-path closeout report consumption through the public runtime facade",
    ),
    WorthQueryLowerRuntimeGoldenTranscript::new(
        "support_and_closeout_inspection",
        "tests/ui/lower_runtime_routing/golden/lower_runtime_routing_support_and_closeout_inspection_golden_transcript_compiles.rs",
        "support lookup plus deferred closeout inspection through public routing DX helpers",
    ),
    WorthQueryLowerRuntimeGoldenTranscript::new(
        "certification_surface_readout",
        "tests/ui/lower_runtime_routing/golden/lower_runtime_routing_certification_surface_readout_golden_transcript_compiles.rs",
        "named closure test, reconciliation, and synthetic-tail readout through public certification surfaces",
    ),
];

pub fn worth_query_lower_runtime_golden_transcripts(
) -> &'static [WorthQueryLowerRuntimeGoldenTranscript] {
    &GOLDEN_TRANSCRIPTS
}

pub fn worth_query_lower_runtime_target_dx_digest() -> String {
    hash_parts(
        &GOLDEN_TRANSCRIPTS
            .iter()
            .map(|row| format!("{}|{}", row.label(), row.dx_focus()))
            .collect::<Vec<_>>(),
    )
}

pub(crate) fn worth_query_lower_runtime_golden_transcript_digest() -> String {
    hash_parts(
        &GOLDEN_TRANSCRIPTS
            .iter()
            .map(|row| format!("{}|{}", row.label(), row.path()))
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn golden_transcript_manifest_is_duplicate_free_and_nonempty() {
        let rows = worth_query_lower_runtime_golden_transcripts();
        let labels = rows.iter().map(|row| row.label()).collect::<Vec<_>>();
        let paths = rows.iter().map(|row| row.path()).collect::<Vec<_>>();

        assert_eq!(rows.len(), 3);
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
            worth_query_lower_runtime_target_dx_digest(),
            worth_query_lower_runtime_golden_transcript_digest()
        );
    }
}
