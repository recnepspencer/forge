use crate::basis::ResolvedSnapshotBasis;
use crate::identity_evolution::InspectorIdentityArtifact;
use crate::live::LiveQueryPlan;
use crate::view_shape::{GroupedDeltaAdmissionPolicy, ViewShapePlanArtifact};

use super::super::counters::ViewShapeLiveCounters;
use super::super::grouped_state::GroupedDesiredStateArtifact;
use super::lowering::ViewShapeLiveLowering;

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
