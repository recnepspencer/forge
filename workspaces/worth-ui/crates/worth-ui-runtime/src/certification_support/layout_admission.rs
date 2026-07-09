//! SUPPORT AUTHORITY — layout participation seeding for certification fixtures.
//!
//! Applies layout admission through named participation transitions, then commits a
//! same-world successor snapshot. Not a production graph mutation owner.

use crate::facade::WorthUiApp;
use crate::graph::{UiGraphMutationStage, UiGraphNodeIdentity, UiGraphSnapshot};

/// Seed layout-admitted participation for support fixtures.
///
/// Each admitted node transitions Layout → Admitted via
/// [`UiGraphParticipationMutation::axis_transition`], then the snapshot is rebuilt as a
/// same-world successor. Production callers must not use this path.
pub(crate) fn snapshot_after_layout_admission_support(
    app: &WorthUiApp,
    admitted_nodes: &[UiGraphNodeIdentity],
) -> UiGraphSnapshot {
    UiGraphMutationStage::layout_admitted_successor(app.graph_snapshot(), admitted_nodes).commit()
}
