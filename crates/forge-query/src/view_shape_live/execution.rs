use crate::identity::hash_parts;
use crate::live::{
    execute_live_change, BridgeChangeSummary, DetailPatch, LiveCollectionPatchError,
    LiveExecutionError, LivePatchPayload, OrderedCollectionPatch, ProjectionFieldDelta,
};
use crate::view_shape::KanbanGroupedLiveContract;

use super::artifact::{
    DetailFieldPatchArtifact, FocusedInspectorAspectPatchArtifact, GroupedLiveViewShapeArtifact,
    LiveViewShapeArtifact, LiveViewShapeExecutionEnvelope, ObservedInspectorPatchArtifact,
    TableRowPatchArtifact, ViewShapeLiveReport, ViewShapePatchEnvelope, ViewShapePatchFamily,
    ViewShapePatchPayload, ViewShapeRefreshDisposition, ViewShapeReplayBundle,
    ViewShapeSuppressionDisposition,
};
use super::error::{ViewShapeLiveError, ViewShapeLiveFailureClass};
use super::family::LiveViewShapeFamily;
use super::grouped_delta::{build_grouped_delta, GroupedDeltaComputation, GroupedRefreshReason};
use super::grouped_execution::GroupedExecutionSurfaceArtifact;
use super::grouped_state::desired_state_from_members;

fn patch_field_count(patch: &DetailPatch) -> usize {
    patch.field_deltas().len()
}

fn ordered_patch_width(patch: &OrderedCollectionPatch) -> usize {
    patch.projected_field_deltas().len() + 1
}

fn focus_projection<'a>(
    deltas: &'a [ProjectionFieldDelta],
    focus_aspect: &str,
) -> Result<Vec<&'a ProjectionFieldDelta>, Vec<String>> {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    for delta in deltas {
        if delta.field().aspect() == focus_aspect {
            accepted.push(delta);
        } else {
            rejected.push(delta.field().aspect().to_string());
        }
    }
    if rejected.is_empty() {
        Ok(accepted)
    } else {
        Err(rejected)
    }
}

fn grouped_refresh_payload(
    family: LiveViewShapeFamily,
    reason: GroupedRefreshReason,
    fallback: Option<crate::live::RefreshFallback>,
    core_replay_digest: &str,
) -> ViewShapePatchEnvelope {
    ViewShapePatchEnvelope::new(
        family,
        None,
        format!("grouped-refresh:{core_replay_digest}"),
        hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("core_replay:{core_replay_digest}"),
            format!("reason:{reason:?}"),
        ]),
        ViewShapePatchPayload::Refresh(ViewShapeRefreshDisposition::GroupedDeferredDebt {
            contract: KanbanGroupedLiveContract::RefreshDeferredDebt,
            reason,
            fallback,
        }),
    )
}

fn grouped_core_execution_rejection_envelope(
    live_view: &LiveViewShapeArtifact,
    reason: GroupedRefreshReason,
    rejection: &LiveExecutionError,
    mut counters: super::counters::ViewShapeLiveCounters,
) -> LiveViewShapeExecutionEnvelope {
    counters.add_grouped_full_regroup_denial();
    counters.add_view_refresh_fallback();
    counters.add_view_family_refresh_admission();

    let family = live_view.lowering().family();
    let rejection_digest = hash_parts(&[
        format!("family:{}", family.as_str()),
        format!("plan:{:?}", live_view.plan().view_plan_digest()),
        format!("basis:{:?}", live_view.basis().proof().digest()),
        format!("rejection:{rejection:?}"),
    ]);
    let patch_envelope = grouped_refresh_payload(family, reason, None, rejection_digest.as_str());
    let report = ViewShapeLiveReport::new(
        family,
        patch_envelope.delivery_digest(),
        patch_envelope.replay_digest(),
    );
    let replay_bundle = ViewShapeReplayBundle::new(
        patch_envelope.delivery_digest(),
        patch_envelope.replay_digest(),
        None,
        counters.clone(),
    );

    LiveViewShapeExecutionEnvelope::new(
        report,
        patch_envelope,
        replay_bundle,
        counters,
        None,
        live_view.clone(),
    )
}

pub fn execute_live_view_shape_change(
    live_view: &LiveViewShapeArtifact,
    change: &BridgeChangeSummary,
) -> Result<LiveViewShapeExecutionEnvelope, ViewShapeLiveError> {
    if live_view.lowering().family() == LiveViewShapeFamily::KanbanGrouped {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedRefreshForbidden,
            "grouped live execution requires grouped admission and next authoritative grouped truth",
            live_view.counters().clone(),
        ));
    }
    execute_live_view_shape_change_inner(live_view, change, None)
}

pub fn admit_grouped_live_view<'a>(
    live_view: &'a LiveViewShapeArtifact,
) -> Result<GroupedLiveViewShapeArtifact<'a>, ViewShapeLiveError> {
    if live_view.lowering().family() != LiveViewShapeFamily::KanbanGrouped {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedRefreshForbidden,
            format!(
                "view family '{}' is not admitted for grouped live execution",
                live_view.lowering().family().as_str()
            ),
            live_view.counters().clone(),
        ));
    }
    if live_view.grouped_state().is_none() || live_view.grouped_policy().is_none() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineRequired,
            "grouped live artifact must retain grouped desired-state and grouped policy",
            live_view.counters().clone(),
        ));
    }
    Ok(GroupedLiveViewShapeArtifact::new(live_view))
}

pub fn execute_grouped_live_view_shape_change(
    live_view: GroupedLiveViewShapeArtifact<'_>,
    change: &BridgeChangeSummary,
    next_grouped_execution: &GroupedExecutionSurfaceArtifact,
) -> Result<LiveViewShapeExecutionEnvelope, ViewShapeLiveError> {
    execute_live_view_shape_change_inner(
        live_view.live_view(),
        change,
        Some(next_grouped_execution),
    )
}

fn execute_live_view_shape_change_inner(
    live_view: &LiveViewShapeArtifact,
    change: &BridgeChangeSummary,
    next_grouped_execution: Option<&GroupedExecutionSurfaceArtifact>,
) -> Result<LiveViewShapeExecutionEnvelope, ViewShapeLiveError> {
    let family = live_view.lowering().family();
    let core_execution = match execute_live_change(live_view.core_live_plan(), change) {
        Ok(execution) => execution,
        Err(
            error @ LiveExecutionError::OrderedCollection(
                LiveCollectionPatchError::CoalescingRequired { .. },
            ),
        ) if family == LiveViewShapeFamily::KanbanGrouped => {
            return Ok(grouped_core_execution_rejection_envelope(
                live_view,
                GroupedRefreshReason::CoreRefreshRequested,
                &error,
                live_view.counters().clone(),
            ));
        }
        Err(error) => return Err(error.into()),
    };
    let mut counters = live_view
        .counters()
        .clone()
        .with_core(core_execution.counters().clone());

    let patch_envelope = match (family, core_execution.patch_envelope().payload()) {
        (LiveViewShapeFamily::Table, LivePatchPayload::OrderedCollection(patch)) => {
            let width = ordered_patch_width(patch);
            counters.set_view_patch_width(width);
            counters.set_view_delivery_width(width);
            ViewShapePatchEnvelope::new(
                family,
                Some(ViewShapePatchFamily::TableRowPatch),
                core_execution.patch_envelope().delivery_digest(),
                hash_parts(&[
                    format!("family:{}", family.as_str()),
                    format!(
                        "core_replay:{}",
                        core_execution.patch_envelope().replay_digest()
                    ),
                ]),
                ViewShapePatchPayload::TableRowPatch(TableRowPatchArtifact::new(
                    patch.digest().as_str(),
                    width,
                )),
            )
        }
        (LiveViewShapeFamily::Detail, LivePatchPayload::Detail(patch)) => {
            let width = patch_field_count(patch);
            counters.set_view_patch_width(width);
            counters.set_view_delivery_width(width);
            ViewShapePatchEnvelope::new(
                family,
                Some(ViewShapePatchFamily::DetailFieldPatch),
                core_execution.patch_envelope().delivery_digest(),
                hash_parts(&[
                    format!("family:{}", family.as_str()),
                    format!(
                        "core_replay:{}",
                        core_execution.patch_envelope().replay_digest()
                    ),
                ]),
                ViewShapePatchPayload::DetailFieldPatch(DetailFieldPatchArtifact::new(
                    patch.digest().as_str(),
                    width,
                )),
            )
        }
        (LiveViewShapeFamily::InspectorDetailObserved, LivePatchPayload::Detail(patch)) => {
            let width = patch_field_count(patch);
            counters.set_view_patch_width(width);
            counters.set_view_delivery_width(width);
            counters.set_observed_inspector_delivery_width(width);
            let inspector_identity = live_view.inspector_identity().cloned();
            ViewShapePatchEnvelope::new(
                family,
                Some(ViewShapePatchFamily::ObservedInspectorPatch),
                core_execution.patch_envelope().delivery_digest(),
                hash_parts(&[
                    format!("family:{}", family.as_str()),
                    format!(
                        "core_replay:{}",
                        core_execution.patch_envelope().replay_digest()
                    ),
                    format!(
                        "identity:{}",
                        inspector_identity
                            .as_ref()
                            .map(|artifact| artifact.digest().as_str())
                            .unwrap_or("none")
                    ),
                    "observed:narrow".to_string(),
                ]),
                ViewShapePatchPayload::ObservedInspectorPatch(ObservedInspectorPatchArtifact::new(
                    patch.digest().as_str(),
                    width,
                    width,
                    inspector_identity,
                )),
            )
        }
        (LiveViewShapeFamily::InspectorDetailFocused, LivePatchPayload::Detail(patch)) => {
            let focus_aspect = live_view
                .plan()
                .delivery_metadata()
                .focus_aspect()
                .unwrap_or("none");
            let focused =
                focus_projection(patch.field_deltas(), focus_aspect).map_err(|received| {
                    counters.add_focused_inspector_widening_denial();
                    counters.add_view_family_fallback_denial();
                    ViewShapeLiveError::new(
                        ViewShapeLiveFailureClass::FocusedInspectorWideningDenied,
                        format!(
                            "focused inspector aspect '{}' denied widening into aspects '{}'",
                            focus_aspect,
                            received.join(",")
                        ),
                        counters.clone(),
                    )
                })?;
            counters.set_focused_inspector_projection_width(patch.field_deltas().len());
            counters.set_focused_inspector_aspect_focus_width(focused.len());
            counters.set_view_patch_width(focused.len());
            counters.set_view_delivery_width(focused.len());
            let inspector_identity = live_view.inspector_identity().cloned();
            ViewShapePatchEnvelope::new(
                family,
                Some(ViewShapePatchFamily::FocusedInspectorAspectPatch),
                core_execution.patch_envelope().delivery_digest(),
                hash_parts(&[
                    format!("family:{}", family.as_str()),
                    format!(
                        "core_replay:{}",
                        core_execution.patch_envelope().replay_digest()
                    ),
                    format!("focus:{focus_aspect}"),
                    format!(
                        "identity:{}",
                        inspector_identity
                            .as_ref()
                            .map(|artifact| artifact.digest().as_str())
                            .unwrap_or("none")
                    ),
                ]),
                ViewShapePatchPayload::FocusedInspectorAspectPatch(
                    FocusedInspectorAspectPatchArtifact::new(
                        patch.digest().as_str(),
                        focus_aspect,
                        focused.len(),
                        inspector_identity,
                    ),
                ),
            )
        }
        (LiveViewShapeFamily::KanbanGrouped, LivePatchPayload::OrderedCollection(_)) => {
            let grouped_state = live_view
                .grouped_state()
                .expect("grouped live artifact must carry grouped desired-state");
            let grouped_policy = live_view
                .grouped_policy()
                .expect("grouped live artifact must carry grouped delta policy");
            let Some(next_grouped_execution) = next_grouped_execution else {
                counters.add_view_family_fallback_denial();
                return Err(ViewShapeLiveError::new(
                    ViewShapeLiveFailureClass::GroupedRefreshForbidden,
                    "grouped live execution requires next authoritative grouped execution truth",
                    counters,
                ));
            };
            if next_grouped_execution.plan_digest() != live_view.plan().view_plan_digest() {
                counters.add_view_family_fallback_denial();
                return Err(ViewShapeLiveError::new(
                    ViewShapeLiveFailureClass::GroupedBaselineMismatch,
                    "next grouped execution surface plan digest does not match live grouped plan",
                    counters,
                ));
            }
            if next_grouped_execution.basis_digest() != live_view.basis().proof().digest() {
                counters.add_view_family_fallback_denial();
                return Err(ViewShapeLiveError::new(
                    ViewShapeLiveFailureClass::GroupedBaselineMismatch,
                    "next grouped execution surface basis digest does not match live grouped basis",
                    counters,
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
                    counters,
                ));
            }
            let next_grouped_state = desired_state_from_members(
                next_grouped_execution
                    .grouped_planning()
                    .grouping_aspect()
                    .to_string(),
                next_grouped_execution
                    .member_rows()
                    .iter()
                    .map(|member_row| {
                        (
                            member_row.member_key().to_string(),
                            member_row.lane().lane_key().to_string(),
                        )
                    })
                    .collect(),
            );

            match build_grouped_delta(grouped_state, &next_grouped_state, grouped_policy) {
                GroupedDeltaComputation::DeltaBound { delta, .. } => {
                    counters.set_grouped_desired_state_row_count(
                        delta.prior().result().row_count() + delta.next().result().row_count(),
                    );
                    counters.set_grouped_delta_row_count(delta.transitions().len());
                    counters.set_grouped_membership_transition_count(delta.transitions().len());
                    counters.set_grouped_lane_count(delta.next().result().lane_count());
                    counters.set_view_patch_width(delta.transitions().len());
                    counters.set_view_delivery_width(delta.transitions().len());
                    ViewShapePatchEnvelope::new(
                        family,
                        Some(ViewShapePatchFamily::KanbanGroupMembershipPatch),
                        core_execution.patch_envelope().delivery_digest(),
                        hash_parts(&[
                            format!("family:{}", family.as_str()),
                            format!("delta:{}", delta.digest()),
                        ]),
                        ViewShapePatchPayload::KanbanGroupMembershipPatch(delta),
                    )
                }
                GroupedDeltaComputation::RefreshDeferredDebt { reason, .. } => {
                    counters.add_grouped_full_regroup_denial();
                    counters.add_view_refresh_fallback();
                    counters.add_view_family_refresh_admission();
                    grouped_refresh_payload(
                        family,
                        reason,
                        None,
                        core_execution.patch_envelope().replay_digest(),
                    )
                }
            }
        }
        (LiveViewShapeFamily::KanbanGrouped, LivePatchPayload::Refresh(fallback)) => {
            counters.add_grouped_full_regroup_denial();
            counters.add_view_refresh_fallback();
            counters.add_view_family_refresh_admission();
            grouped_refresh_payload(
                family,
                GroupedRefreshReason::CoreRefreshRequested,
                Some(fallback.clone()),
                core_execution.patch_envelope().replay_digest(),
            )
        }
        (LiveViewShapeFamily::InspectorDetailFocused, LivePatchPayload::Refresh(_)) => {
            counters.add_view_family_refresh_forbidden();
            return Err(ViewShapeLiveError::new(
                ViewShapeLiveFailureClass::FocusedInspectorRefreshForbidden,
                "focused inspector may not silently degrade to generic refresh delivery",
                counters,
            ));
        }
        (_, LivePatchPayload::Refresh(fallback)) => {
            counters.add_view_refresh_fallback();
            counters.add_view_family_refresh_admission();
            ViewShapePatchEnvelope::new(
                family,
                None,
                core_execution.patch_envelope().delivery_digest(),
                hash_parts(&[
                    format!("family:{}", family.as_str()),
                    format!(
                        "core_replay:{}",
                        core_execution.patch_envelope().replay_digest()
                    ),
                    format!("refresh:{:?}", fallback.admission_class()),
                ]),
                ViewShapePatchPayload::Refresh(ViewShapeRefreshDisposition::Admitted {
                    family,
                    fallback: fallback.clone(),
                }),
            )
        }
        (_, LivePatchPayload::Suppressed(reason)) => ViewShapePatchEnvelope::new(
            family,
            None,
            core_execution.patch_envelope().delivery_digest(),
            hash_parts(&[
                format!("family:{}", family.as_str()),
                format!(
                    "core_replay:{}",
                    core_execution.patch_envelope().replay_digest()
                ),
                format!("suppression:{reason:?}"),
            ]),
            ViewShapePatchPayload::Suppressed(ViewShapeSuppressionDisposition::SuppressedByCore(
                reason.clone(),
            )),
        ),
        _ => {
            counters.add_cosmetic_view_semantics_denial();
            return Err(ViewShapeLiveError::new(
                ViewShapeLiveFailureClass::UnderlyingLiveFamilyMismatch,
                format!(
                    "core payload '{:?}' is incompatible with view family '{}'",
                    core_execution.patch_envelope().payload(),
                    family.as_str()
                ),
                counters,
            ));
        }
    };

    let next_live_view = match (&patch_envelope.payload(), family) {
        (
            ViewShapePatchPayload::KanbanGroupMembershipPatch(delta),
            LiveViewShapeFamily::KanbanGrouped,
        ) => LiveViewShapeArtifact::new(
            live_view.plan().clone(),
            live_view.basis().clone(),
            live_view.lowering().clone(),
            live_view.core_live_plan().clone(),
            counters.clone(),
            Some(delta.next().clone()),
            live_view.grouped_policy().cloned(),
            live_view.inspector_identity().cloned(),
        ),
        _ => live_view.clone(),
    };

    let report = ViewShapeLiveReport::new(
        family,
        patch_envelope.delivery_digest(),
        patch_envelope.replay_digest(),
    );
    let replay_bundle = ViewShapeReplayBundle::new(
        patch_envelope.delivery_digest(),
        patch_envelope.replay_digest(),
        Some(core_execution.replay_bundle().clone()),
        counters.clone(),
    );

    Ok(LiveViewShapeExecutionEnvelope::new(
        report,
        patch_envelope,
        replay_bundle,
        counters,
        Some(core_execution),
        next_live_view,
    ))
}
