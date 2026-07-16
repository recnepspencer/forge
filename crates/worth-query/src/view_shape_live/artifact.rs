use crate::basis::ResolvedSnapshotBasis;
use crate::identity::hash_parts;
use crate::identity_evolution::InspectorIdentityArtifact;
use crate::live::{
    LiveExecutionEnvelope, LiveQueryFamily, LiveQueryPlan, LiveReplayBundle, RefreshFallback,
    SuppressionReason,
};
use crate::view_shape::{GroupedDeltaAdmissionPolicy, ViewShapePlanArtifact};
use worth_foundational::facade::AspectKey;

use super::counters::ViewShapeLiveCounters;
use super::family::LiveViewShapeFamily;
use super::grouped_delta::GroupedDeltaArtifact;
use super::grouped_state::GroupedDesiredStateArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeLiveLowering {
    digest: String,
    family: LiveViewShapeFamily,
    underlying_live_family: LiveQueryFamily,
}

impl ViewShapeLiveLowering {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn family(&self) -> LiveViewShapeFamily {
        self.family
    }

    pub fn underlying_live_family(&self) -> &LiveQueryFamily {
        &self.underlying_live_family
    }

    pub(crate) fn new(family: LiveViewShapeFamily) -> Self {
        let underlying_live_family = family.underlying_live_family();
        let digest = hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("underlying:{}", underlying_live_family.as_str()),
        ]);
        Self {
            digest,
            family,
            underlying_live_family,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ViewShapePatchFamily {
    TableRowPatch,
    DetailFieldPatch,
    ObservedInspectorPatch,
    FocusedInspectorAspectPatch,
    KanbanGroupMembershipPatch,
}

impl ViewShapePatchFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TableRowPatch => "table_row_patch",
            Self::DetailFieldPatch => "detail_field_patch",
            Self::ObservedInspectorPatch => "observed_inspector_patch",
            Self::FocusedInspectorAspectPatch => "focused_inspector_aspect_patch",
            Self::KanbanGroupMembershipPatch => "kanban_group_membership_patch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableRowPatchArtifact {
    digest: String,
    row_delta_count: usize,
}

impl TableRowPatchArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn row_delta_count(&self) -> usize {
        self.row_delta_count
    }
    #[cfg(test)]
    pub(crate) fn new(digest: impl Into<String>, row_delta_count: usize) -> Self {
        Self {
            digest: digest.into(),
            row_delta_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailFieldPatchArtifact {
    digest: String,
    field_delta_count: usize,
}

impl DetailFieldPatchArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn field_delta_count(&self) -> usize {
        self.field_delta_count
    }
    #[cfg(test)]
    pub(crate) fn new(digest: impl Into<String>, field_delta_count: usize) -> Self {
        Self {
            digest: digest.into(),
            field_delta_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedInspectorPatchArtifact {
    digest: String,
    field_delta_count: usize,
    delivery_width: usize,
    inspector_identity: Option<InspectorIdentityArtifact>,
}

impl ObservedInspectorPatchArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn field_delta_count(&self) -> usize {
        self.field_delta_count
    }

    pub fn delivery_width(&self) -> usize {
        self.delivery_width
    }

    pub fn inspector_identity(&self) -> Option<&InspectorIdentityArtifact> {
        self.inspector_identity.as_ref()
    }
    #[cfg(test)]
    pub(crate) fn new(
        digest: impl Into<String>,
        field_delta_count: usize,
        delivery_width: usize,
        inspector_identity: Option<InspectorIdentityArtifact>,
    ) -> Self {
        Self {
            digest: digest.into(),
            field_delta_count,
            delivery_width,
            inspector_identity,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusedInspectorAspectPatchArtifact {
    digest: String,
    focus_aspect: AspectKey,
    field_delta_count: usize,
    inspector_identity: Option<InspectorIdentityArtifact>,
}

impl FocusedInspectorAspectPatchArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn native_focus_aspect_key(&self) -> &AspectKey {
        &self.focus_aspect
    }

    pub fn field_delta_count(&self) -> usize {
        self.field_delta_count
    }

    pub fn inspector_identity(&self) -> Option<&InspectorIdentityArtifact> {
        self.inspector_identity.as_ref()
    }
    #[cfg(test)]
    pub(crate) fn new(
        digest: impl Into<String>,
        focus_aspect: AspectKey,
        field_delta_count: usize,
        inspector_identity: Option<InspectorIdentityArtifact>,
    ) -> Self {
        Self {
            digest: digest.into(),
            focus_aspect,
            field_delta_count,
            inspector_identity,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewShapePatchPayload {
    TableRowPatch(TableRowPatchArtifact),
    DetailFieldPatch(DetailFieldPatchArtifact),
    ObservedInspectorPatch(ObservedInspectorPatchArtifact),
    FocusedInspectorAspectPatch(FocusedInspectorAspectPatchArtifact),
    KanbanGroupMembershipPatch(GroupedDeltaArtifact),
    Refresh(ViewShapeRefreshDisposition),
    Suppressed(ViewShapeSuppressionDisposition),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewShapeRefreshDisposition {
    Admitted {
        family: LiveViewShapeFamily,
        fallback: RefreshFallback,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewShapeSuppressionDisposition {
    SuppressedByCore(SuppressionReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapePatchEnvelope {
    family: LiveViewShapeFamily,
    patch_family: Option<ViewShapePatchFamily>,
    delivery_digest: String,
    replay_digest: String,
    payload: ViewShapePatchPayload,
}

impl ViewShapePatchEnvelope {
    pub fn family(&self) -> LiveViewShapeFamily {
        self.family
    }

    pub fn patch_family(&self) -> Option<ViewShapePatchFamily> {
        self.patch_family
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn payload(&self) -> &ViewShapePatchPayload {
        &self.payload
    }
    #[cfg(test)]
    pub(crate) fn new(
        family: LiveViewShapeFamily,
        patch_family: Option<ViewShapePatchFamily>,
        delivery_digest: impl Into<String>,
        replay_digest: impl Into<String>,
        payload: ViewShapePatchPayload,
    ) -> Self {
        Self {
            family,
            patch_family,
            delivery_digest: delivery_digest.into(),
            replay_digest: replay_digest.into(),
            payload,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeReplayBundle {
    delivery_digest: String,
    replay_digest: String,
    core: Option<LiveReplayBundle>,
    counters: ViewShapeLiveCounters,
}

impl ViewShapeReplayBundle {
    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn core(&self) -> Option<&LiveReplayBundle> {
        self.core.as_ref()
    }

    pub fn counters(&self) -> &ViewShapeLiveCounters {
        &self.counters
    }
    #[cfg(test)]
    pub(crate) fn new(
        delivery_digest: impl Into<String>,
        replay_digest: impl Into<String>,
        core: Option<LiveReplayBundle>,
        counters: ViewShapeLiveCounters,
    ) -> Self {
        Self {
            delivery_digest: delivery_digest.into(),
            replay_digest: replay_digest.into(),
            core,
            counters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeLiveReport {
    digest: String,
    family: LiveViewShapeFamily,
    delivery_digest: String,
    replay_digest: String,
}

impl ViewShapeLiveReport {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn family(&self) -> LiveViewShapeFamily {
        self.family
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }
    #[cfg(test)]
    pub(crate) fn new(
        family: LiveViewShapeFamily,
        delivery_digest: impl Into<String>,
        replay_digest: impl Into<String>,
    ) -> Self {
        let delivery_digest = delivery_digest.into();
        let replay_digest = replay_digest.into();
        let digest = hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("delivery:{delivery_digest}"),
            format!("replay:{replay_digest}"),
        ]);
        Self {
            digest,
            family,
            delivery_digest,
            replay_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveViewShapeArtifact {
    plan: ViewShapePlanArtifact,
    basis: ResolvedSnapshotBasis,
    lowering: ViewShapeLiveLowering,
    core_live_plan: LiveQueryPlan,
    counters: ViewShapeLiveCounters,
    grouped_state: Option<GroupedDesiredStateArtifact>,
    grouped_policy: Option<GroupedDeltaAdmissionPolicy>,
    inspector_identity: Option<InspectorIdentityArtifact>,
}

impl LiveViewShapeArtifact {
    pub fn plan(&self) -> &ViewShapePlanArtifact {
        &self.plan
    }

    pub fn basis(&self) -> &ResolvedSnapshotBasis {
        &self.basis
    }

    pub fn lowering(&self) -> &ViewShapeLiveLowering {
        &self.lowering
    }

    pub fn core_live_plan(&self) -> &LiveQueryPlan {
        &self.core_live_plan
    }

    pub fn counters(&self) -> &ViewShapeLiveCounters {
        &self.counters
    }

    pub fn grouped_state(&self) -> Option<&GroupedDesiredStateArtifact> {
        self.grouped_state.as_ref()
    }

    pub fn grouped_policy(&self) -> Option<&GroupedDeltaAdmissionPolicy> {
        self.grouped_policy.as_ref()
    }

    pub fn inspector_identity(&self) -> Option<&InspectorIdentityArtifact> {
        self.inspector_identity.as_ref()
    }

    pub(crate) fn new(
        plan: ViewShapePlanArtifact,
        basis: ResolvedSnapshotBasis,
        lowering: ViewShapeLiveLowering,
        core_live_plan: LiveQueryPlan,
        counters: ViewShapeLiveCounters,
        grouped_state: Option<GroupedDesiredStateArtifact>,
        grouped_policy: Option<GroupedDeltaAdmissionPolicy>,
        inspector_identity: Option<InspectorIdentityArtifact>,
    ) -> Self {
        Self {
            plan,
            basis,
            lowering,
            core_live_plan,
            counters,
            grouped_state,
            grouped_policy,
            inspector_identity,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GroupedLiveViewShapeArtifact<'a> {
    live_view: &'a LiveViewShapeArtifact,
}

impl<'a> GroupedLiveViewShapeArtifact<'a> {
    pub fn live_view(&self) -> &'a LiveViewShapeArtifact {
        self.live_view
    }
    #[cfg(test)]
    pub(crate) fn new(live_view: &'a LiveViewShapeArtifact) -> Self {
        Self { live_view }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveViewShapeExecutionEnvelope {
    report: ViewShapeLiveReport,
    patch_envelope: ViewShapePatchEnvelope,
    replay_bundle: ViewShapeReplayBundle,
    counters: ViewShapeLiveCounters,
    core_execution: Option<LiveExecutionEnvelope>,
    next_live_view: LiveViewShapeArtifact,
}

impl LiveViewShapeExecutionEnvelope {
    pub fn report(&self) -> &ViewShapeLiveReport {
        &self.report
    }

    pub fn patch_envelope(&self) -> &ViewShapePatchEnvelope {
        &self.patch_envelope
    }

    pub fn replay_bundle(&self) -> &ViewShapeReplayBundle {
        &self.replay_bundle
    }

    pub fn counters(&self) -> &ViewShapeLiveCounters {
        &self.counters
    }

    pub fn core_execution(&self) -> Option<&LiveExecutionEnvelope> {
        self.core_execution.as_ref()
    }

    pub fn next_live_view(&self) -> &LiveViewShapeArtifact {
        &self.next_live_view
    }
    #[cfg(test)]
    pub(crate) fn new(
        report: ViewShapeLiveReport,
        patch_envelope: ViewShapePatchEnvelope,
        replay_bundle: ViewShapeReplayBundle,
        counters: ViewShapeLiveCounters,
        core_execution: Option<LiveExecutionEnvelope>,
        next_live_view: LiveViewShapeArtifact,
    ) -> Self {
        Self {
            report,
            patch_envelope,
            replay_bundle,
            counters,
            core_execution,
            next_live_view,
        }
    }
}
