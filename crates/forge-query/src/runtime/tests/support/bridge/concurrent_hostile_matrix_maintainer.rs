use std::sync::Arc;

use crate::runtime::tests::support::*;
use crate::runtime::{
    ForgeQueryDerivedPatch, ForgeQueryDerivedPatchPayload, ForgeQueryDerivedViewMaintainer,
};

#[derive(Clone)]
pub(super) struct ConcurrentMatrixMaintainer {
    titles: Arc<Vec<String>>,
}

impl ConcurrentMatrixMaintainer {
    pub(super) fn seeded() -> Self {
        Self {
            titles: Arc::new(vec!["Task One".to_string()]),
        }
    }
}

impl ForgeQueryDerivedViewMaintainer for ConcurrentMatrixMaintainer {
    fn maintain(
        &mut self,
        view: &crate::program::ForgeQueryDerivedView,
        _delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let title = self
            .titles
            .last()
            .cloned()
            .unwrap_or_else(|| "Unpublished".to_string());
        let retained_row = retained_string_test_row("title.value", title.clone());
        materialization.replace_retained_rows([retained_row.clone()]);
        ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::memory_workspace::admit_external_commit_label(format!(
                "phase16-publication-{}",
                title
            )),
            [test_aspect_touch("title.value")],
            ForgeQueryDerivedPatchPayload::from_retained_row(retained_row),
            format!("phase16-publication-{}", title),
        )
    }
}
