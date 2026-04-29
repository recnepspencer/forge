mod artifact;
mod counters;
mod error;
mod execution;
mod family;
mod grouped_baseline;
mod grouped_delta;
mod grouped_execution;
mod grouped_state;
mod promotion;

pub use artifact::{
    DetailFieldPatchArtifact, FocusedInspectorAspectPatchArtifact, GroupedLiveViewShapeArtifact,
    LiveViewShapeArtifact, LiveViewShapeExecutionEnvelope, ObservedInspectorPatchArtifact,
    TableRowPatchArtifact, ViewShapeLiveLowering, ViewShapeLiveReport, ViewShapePatchEnvelope,
    ViewShapePatchFamily, ViewShapePatchPayload, ViewShapeRefreshDisposition,
    ViewShapeReplayBundle, ViewShapeSuppressionDisposition,
};
pub use counters::ViewShapeLiveCounters;
pub use error::{ViewShapeLiveError, ViewShapeLiveFailureClass};
pub use execution::{
    admit_grouped_live_view, execute_grouped_live_view_shape_change, execute_live_view_shape_change,
};
pub use family::LiveViewShapeFamily;
#[cfg(test)]
pub use grouped_baseline::materialize_authoritative_grouped_baseline;
pub use grouped_baseline::{
    materialize_authoritative_grouped_baseline_from_members, AuthoritativeGroupedBaselineArtifact,
};
pub use grouped_delta::{
    GroupedDeltaArtifact, GroupedDeltaComputation, GroupedMembershipTransition,
    GroupedMembershipTransitionKind, GroupedRefreshReason,
};
pub use grouped_execution::{
    materialize_grouped_execution_surface_from_truth_view, GroupedExecutionLaneValue,
    GroupedExecutionMemberRow, GroupedExecutionSurfaceArtifact,
};
pub use grouped_state::{
    GroupedDesiredStateArtifact, GroupedLaneIdentity, GroupedMemberState, GroupedViewResultArtifact,
};
pub use promotion::lower_view_shape_plan_to_live;

#[cfg(test)]
mod tests;
