use forge_query::facade::consumer_kit::{
    hard_prohibition_seeded_consumer_sources, ForgeQueryBoundaryAuditSourceSet,
    ForgeQueryProhibitedSeam,
};

use super::query_boundary_sources::worth_kernel_query_boundary_sources;

#[derive(Clone, Debug)]
pub(crate) struct SeededQueryBypassSourceSet {
    seam: ForgeQueryProhibitedSeam,
    source_label: String,
    source_path: String,
    sources: ForgeQueryBoundaryAuditSourceSet,
}

impl SeededQueryBypassSourceSet {
    pub(crate) fn seam(&self) -> ForgeQueryProhibitedSeam {
        self.seam
    }

    pub(crate) fn source_label(&self) -> &str {
        &self.source_label
    }

    pub(crate) fn source_path(&self) -> &str {
        &self.source_path
    }

    pub(crate) fn sources(&self) -> ForgeQueryBoundaryAuditSourceSet {
        self.sources.clone()
    }
}

pub(crate) fn seeded_query_bypass_source_sets() -> Vec<SeededQueryBypassSourceSet> {
    hard_prohibition_seeded_consumer_sources("worth-kernel")
        .into_iter()
        .map(|seeded_source| {
            let sources = worth_kernel_query_boundary_sources().source_file(
                seeded_source.label(),
                seeded_source.source_path(),
                seeded_source.source(),
            );
            SeededQueryBypassSourceSet {
                seam: seeded_source.seam(),
                source_label: seeded_source.label().to_string(),
                source_path: seeded_source.source_path().to_string(),
                sources,
            }
        })
        .collect()
}
