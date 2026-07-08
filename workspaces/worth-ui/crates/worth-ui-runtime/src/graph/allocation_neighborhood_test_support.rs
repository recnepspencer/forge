//! SUPPORT AUTHORITY — test-only graph layout admission seeding.
//!
//! Routes through certification_support named participation transitions. Not production law.

use crate::certification_support::snapshot_after_layout_admission_support;
use crate::facade::WorthUiApp;
use crate::graph::{UiGraphNodeIdentity, UiGraphSnapshot};

pub(crate) fn snapshot_with_admitted_layout(
    app: &WorthUiApp,
    admitted_nodes: &[UiGraphNodeIdentity],
) -> UiGraphSnapshot {
    snapshot_after_layout_admission_support(app, admitted_nodes)
}
