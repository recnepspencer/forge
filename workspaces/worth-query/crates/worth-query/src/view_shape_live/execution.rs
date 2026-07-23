use crate::identity::hash_parts;
use crate::live::{
    execute_live_change, BridgeChangeSummary, DetailPatch, LiveCollectionPatchError,
    LiveExecutionError, LivePatchPayload, OrderedCollectionPatch, ProjectionFieldDelta,
};
use worth_foundational::facade::AspectKey;

use super::artifact::{
    DetailFieldPatchArtifact, FocusedInspectorAspectPatchArtifact, GroupedLiveViewShapeArtifact,
    LiveViewShapeArtifact, LiveViewShapeExecutionEnvelope, ObservedInspectorPatchArtifact,
    TableRowPatchArtifact, ViewShapeLiveReport, ViewShapePatchEnvelope, ViewShapePatchFamily,
    ViewShapePatchPayload, ViewShapeRefreshDisposition, ViewShapeReplayBundle,
    ViewShapeSuppressionDisposition,
};
use super::error::{ViewShapeLiveError, ViewShapeLiveFailureClass};
use super::family::LiveViewShapeFamily;
use super::grouped_delta::{build_grouped_delta, GroupedDeltaInvariantFailure};
use super::grouped_execution::GroupedExecutionSurfaceArtifact;
use super::grouped_state::{desired_state_from_members, WorthQueryGroupedBaselineMember};
#[cfg(test)]
fn patch_field_count(patch: &DetailPatch) -> usize {
    patch.field_deltas().len()
}
#[cfg(test)]
fn ordered_patch_width(patch: &OrderedCollectionPatch) -> usize {
    patch.projected_field_deltas().len() + 1
}
#[cfg(test)]
fn focus_projection<'a>(
    deltas: &'a [ProjectionFieldDelta],
    focus_aspect: &AspectKey,
) -> Result<Vec<&'a ProjectionFieldDelta>, Vec<AspectKey>> {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    for delta in deltas {
        if delta.field().native_aspect_key() == focus_aspect {
            accepted.push(delta);
        } else {
            rejected.push(delta.field().native_aspect_key().clone());
        }
    }
    if rejected.is_empty() {
        Ok(accepted)
    } else {
        Err(rejected)
    }
}
#[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
fn execute_live_view_shape_change_inner(
    live_view: &LiveViewShapeArtifact,
    change: &BridgeChangeSummary,
    next_grouped_execution: Option<&GroupedExecutionSurfaceArtifact>,
) -> Result<LiveViewShapeExecutionEnvelope, ViewShapeLiveError> {
    let family = live_view.lowering().family();
    let core_execution = execute_live_change(live_view.core_live_plan(), change);
    let mut counters = live_view.counters().clone();
    if let Ok(core_execution_counters) = &core_execution {
        counters = counters.with_core(core_execution_counters.counters().clone());
    }
    if family == LiveViewShapeFamily::KanbanGrouped {
        return execute_grouped_live_view_shape_change_inner(
            live_view,
            change,
            next_grouped_execution,
            core_execution,
            counters,
        );
    }
    let core_execution = core_execution?;

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
                .native_focus_aspect_key()
                .expect("focused inspector planning guarantees a native focus aspect")
                .clone();
            let focused =
                focus_projection(patch.field_deltas(), &focus_aspect).map_err(|received| {
                    counters.add_focused_inspector_widening_denial();
                    counters.add_view_family_fallback_denial();
                    let received = received
                        .iter()
                        .map(|aspect| aspect.as_str())
                        .collect::<Vec<_>>()
                        .join(",");
                    ViewShapeLiveError::new(
                        ViewShapeLiveFailureClass::FocusedInspectorWideningDenied,
                        format!(
                            "focused inspector aspect '{}' denied widening into aspects '{}'",
                            focus_aspect.as_str(),
                            received
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
                    format!("focus:{}", focus_aspect.as_str()),
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
#[cfg(test)]
fn execute_grouped_live_view_shape_change_inner(
    live_view: &LiveViewShapeArtifact,
    change: &BridgeChangeSummary,
    next_grouped_execution: Option<&GroupedExecutionSurfaceArtifact>,
    core_execution: Result<crate::live::LiveExecutionEnvelope, LiveExecutionError>,
    mut counters: super::counters::ViewShapeLiveCounters,
) -> Result<LiveViewShapeExecutionEnvelope, ViewShapeLiveError> {
    let family = live_view.lowering().family();
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

    counters.set_grouped_desired_state_row_count(
        delta.prior().result().row_count() + delta.next().result().row_count(),
    );
    counters.set_grouped_delta_row_count(delta.transitions().len());
    counters.set_grouped_membership_transition_count(delta.transitions().len());
    counters.set_grouped_lane_count(delta.next().result().lane_count());
    counters.set_view_patch_width(delta.transitions().len());
    counters.set_view_delivery_width(delta.transitions().len());

    let delivery_digest = match &core_execution {
        Ok(execution) => execution.patch_envelope().delivery_digest().to_string(),
        Err(LiveExecutionError::OrderedCollection(
            LiveCollectionPatchError::CoalescingRequired { .. },
        )) => hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("plan:{}", live_view.plan().view_plan_digest().as_str()),
            format!("basis:{}", live_view.basis().proof().digest().as_str()),
            format!("grouped_execution:{}", next_grouped_execution.digest()),
            format!("change:{change:?}"),
        ]),
        Err(_) => unreachable!("non-coalescing grouped core errors return earlier"),
    };
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
    let report = ViewShapeLiveReport::new(
        family,
        patch_envelope.delivery_digest(),
        patch_envelope.replay_digest(),
    );
    let replay_bundle = ViewShapeReplayBundle::new(
        patch_envelope.delivery_digest(),
        patch_envelope.replay_digest(),
        core_execution
            .as_ref()
            .ok()
            .map(|execution| execution.replay_bundle().clone()),
        counters.clone(),
    );

    Ok(LiveViewShapeExecutionEnvelope::new(
        report,
        patch_envelope,
        replay_bundle,
        counters,
        core_execution.ok(),
        next_live_view,
    ))
}
