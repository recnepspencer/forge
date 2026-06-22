use std::sync::Arc;

use crate::runtime::tests::support::*;
use crate::runtime::{ForgeQueryDerivedPatch, ForgeQueryDerivedViewMaintainer};

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
        materialization.replace_rows([json!({ "title": { "value": title } })]);
        ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::memory_workspace::admit_external_commit_label(format!(
                "phase16-publication-{}",
                title
            )),
            ["title.value".to_string()],
            json!({ "published": true, "title": title }),
            format!("phase16-publication-{}", title),
        )
    }
}
