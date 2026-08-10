use crate::identity::hash_parts;
use crate::live::{
    BridgeChangeSummary, LiveCollectionPatchError, LiveExecutionEnvelope, LiveExecutionError,
};

use super::super::artifact::{
    LiveViewShapeArtifact, ViewShapePatchEnvelope, ViewShapePatchFamily, ViewShapePatchPayload,
};
use super::super::counters::ViewShapeLiveCounters;
use super::super::error::{ViewShapeLiveError, ViewShapeLiveFailureClass};
use super::super::grouped_delta::{build_grouped_delta, GroupedDeltaInvariantFailure};
use super::super::grouped_execution::GroupedExecutionSurfaceArtifact;
use super::super::grouped_state::{desired_state_from_members, WorthQueryGroupedBaselineMember};
use super::result_assembly::LiveExecutionAssembly;

pub(super) struct GroupedLiveTransitionInput<'a> {
    pub(super) live_view: &'a LiveViewShapeArtifact,
    pub(super) change: &'a BridgeChangeSummary,
    pub(super) next_grouped_execution: Option<&'a GroupedExecutionSurfaceArtifact>,
    pub(super) core_execution: Result<LiveExecutionEnvelope, LiveExecutionError>,
    pub(super) counters: ViewShapeLiveCounters,
}

pub(super) fn resolve_grouped_transition(
    input: GroupedLiveTransitionInput<'_>,
) -> Result<LiveExecutionAssembly, ViewShapeLiveError> {
    let GroupedLiveTransitionInput {
        live_view,
        change,
        next_grouped_execution,
        core_execution,
        mut counters,
    } = input;
    let family = live_view.lowering().family();
    let grouped_state = live_view
        .grouped_state()
        .expect("grouped live artifact must carry grouped desired-state");
    let grouped_policy = live_view
        .grouped_policy()
        .expect("grouped live artifact must carry grouped delta policy");
    let next_grouped_execution =
        admit_next_grouped_execution(live_view, next_grouped_execution, &mut counters)?;
    let next_grouped_state = desired_state_from_members(
        next_grouped_execution
            .grouped_planning()
            .native_grouping_aspect_key()
            .clone(),
        next_grouped_execution
            .member_rows()
            .iter()
            .map(|member_row| {
                WorthQueryGroupedBaselineMember::from_authoritative_member_lane_keys(
                    member_row.member_key().to_string(),
                    member_row.lane().lane_key().to_string(),
                )
            })
            .collect(),
    );
    let delta = build_grouped_delta(grouped_state, &next_grouped_state, grouped_policy).map_err(
        |reason| {
            counters.add_view_family_fallback_denial();
            let message = match reason {
                GroupedDeltaInvariantFailure::GroupingAspectMismatch => {
                    "grouped delta grouping aspect must remain stable across grouped execution truth"
                }
            };
            ViewShapeLiveError::new(
                ViewShapeLiveFailureClass::GroupedBaselineMismatch,
                message,
                counters.clone(),
            )
        },
    )?;

    match &core_execution {
        Ok(_) => {}
        Err(LiveExecutionError::OrderedCollection(
            LiveCollectionPatchError::CoalescingRequired { .. },
        )) => {}
        Err(error) => return Err(error.clone().into()),
    }

    record_grouped_delta_counters(&mut counters, &delta, &next_grouped_state);
    let delivery_digest = resolve_grouped_delivery_digest(GroupedDeliveryDigestInput {
        family,
        live_view,
        next_grouped_execution,
        change,
        core_execution: &core_execution,
    });
    let patch_envelope = ViewShapePatchEnvelope::new(
        family,
        Some(ViewShapePatchFamily::KanbanGroupMembershipPatch),
        delivery_digest,
        hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("delta:{}", delta.digest()),
        ]),
        ViewShapePatchPayload::KanbanGroupMembershipPatch(delta.clone()),
    );
    let next_live_view = LiveViewShapeArtifact::new(
        live_view.plan().clone(),
        live_view.basis().clone(),
        live_view.lowering().clone(),
        live_view.core_live_plan().clone(),
        counters.clone(),
        Some(delta.next().clone()),
        live_view.grouped_policy().cloned(),
        live_view.inspector_identity().cloned(),
    );
    Ok(LiveExecutionAssembly {
        patch_envelope,
        counters,
        core_execution: core_execution.ok(),
        next_live_view,
    })
}

fn admit_next_grouped_execution<'a>(
    live_view: &LiveViewShapeArtifact,
    next_grouped_execution: Option<&'a GroupedExecutionSurfaceArtifact>,
    counters: &mut ViewShapeLiveCounters,
) -> Result<&'a GroupedExecutionSurfaceArtifact, ViewShapeLiveError> {
    let Some(next_grouped_execution) = next_grouped_execution else {
        counters.add_view_family_fallback_denial();
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedRefreshForbidden,
            "grouped live execution requires next authoritative grouped execution truth",
            counters.clone(),
        ));
    };
    if next_grouped_execution.plan_digest() != live_view.plan().view_plan_digest() {
        counters.add_view_family_fallback_denial();
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "next grouped execution surface plan digest does not match live grouped plan",
            counters.clone(),
        ));
    }
    if next_grouped_execution.basis_digest() != live_view.basis().proof().digest() {
        counters.add_view_family_fallback_denial();
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "next grouped execution surface basis digest does not match live grouped basis",
            counters.clone(),
        ));
    }
    if next_grouped_execution.grouped_planning()
        != live_view
            .plan()
            .grouped_planning_artifact()
            .expect("grouped live artifact must retain grouped planning")
    {
        counters.add_view_family_fallback_denial();
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "next grouped execution surface planning artifact does not match live grouped planning",
            counters.clone(),
        ));
    }
    Ok(next_grouped_execution)
}

fn record_grouped_delta_counters(
    counters: &mut ViewShapeLiveCounters,
    delta: &super::super::grouped_delta::GroupedDeltaArtifact,
    next_grouped_state: &super::super::grouped_state::GroupedDesiredStateArtifact,
) {
    counters.set_grouped_desired_state_row_count(
        delta.prior().result().row_count() + delta.next().result().row_count(),
    );
    counters.set_grouped_delta_row_count(delta.transitions().len());
    counters.set_grouped_membership_transition_count(delta.transitions().len());
    counters.set_grouped_lane_count(next_grouped_state.result().lane_count());
    counters.set_view_patch_width(delta.transitions().len());
    counters.set_view_delivery_width(delta.transitions().len());
}

struct GroupedDeliveryDigestInput<'a> {
    family: super::super::family::LiveViewShapeFamily,
    live_view: &'a LiveViewShapeArtifact,
    next_grouped_execution: &'a GroupedExecutionSurfaceArtifact,
    change: &'a BridgeChangeSummary,
    core_execution: &'a Result<LiveExecutionEnvelope, LiveExecutionError>,
}

fn resolve_grouped_delivery_digest(input: GroupedDeliveryDigestInput<'_>) -> String {
    match input.core_execution {
        Ok(execution) => execution.patch_envelope().delivery_digest().to_string(),
        Err(LiveExecutionError::OrderedCollection(
            LiveCollectionPatchError::CoalescingRequired { .. },
        )) => hash_parts(&[
            format!("family:{}", input.family.as_str()),
            format!(
                "plan:{}",
                input.live_view.plan().view_plan_digest().as_str()
            ),
            format!(
                "basis:{}",
                input.live_view.basis().proof().digest().as_str()
            ),
            format!(
                "grouped_execution:{}",
                input.next_grouped_execution.digest()
            ),
            format!("change:{:?}", input.change),
        ]),
        Err(_) => unreachable!("non-coalescing grouped core errors return earlier"),
    }
}
