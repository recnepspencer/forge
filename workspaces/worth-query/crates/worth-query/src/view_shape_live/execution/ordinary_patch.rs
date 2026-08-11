use crate::identity::hash_parts;
use crate::live::{
    DetailPatch, LiveExecutionEnvelope, LivePatchPayload, OrderedCollectionPatch,
    ProjectionFieldDelta,
};
use worth_foundational::facade::AspectKey;

use super::super::artifact::{
    DetailFieldPatchArtifact, FocusedInspectorAspectPatchArtifact, LiveViewShapeArtifact,
    ObservedInspectorPatchArtifact, TableRowPatchArtifact, ViewShapePatchEnvelope,
    ViewShapePatchFamily, ViewShapePatchPayload, ViewShapeRefreshDisposition,
    ViewShapeSuppressionDisposition,
};
use super::super::counters::ViewShapeLiveCounters;
use super::super::error::{ViewShapeLiveError, ViewShapeLiveFailureClass};
use super::super::family::LiveViewShapeFamily;
use super::result_assembly::LiveExecutionAssembly;

fn patch_field_count(patch: &DetailPatch) -> usize {
    patch.field_deltas().len()
}

fn ordered_patch_width(patch: &OrderedCollectionPatch) -> usize {
    patch.projected_field_deltas().len() + 1
}

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

struct OrdinaryPatchResolution<'a> {
    live_view: &'a LiveViewShapeArtifact,
    family: LiveViewShapeFamily,
    core_execution: &'a LiveExecutionEnvelope,
    counters: &'a mut ViewShapeLiveCounters,
}

impl<'a> OrdinaryPatchResolution<'a> {
    fn core_delivery_digest(&self) -> &str {
        self.core_execution.patch_envelope().delivery_digest()
    }

    fn core_replay_digest(&self) -> &str {
        self.core_execution.patch_envelope().replay_digest()
    }
}

pub(super) fn resolve_ordinary_patch(
    live_view: &LiveViewShapeArtifact,
    core_execution: crate::live::LiveExecutionEnvelope,
    mut counters: ViewShapeLiveCounters,
) -> Result<LiveExecutionAssembly, ViewShapeLiveError> {
    let family = live_view.lowering().family();
    let patch_envelope = {
        let mut resolution = OrdinaryPatchResolution {
            live_view,
            family,
            core_execution: &core_execution,
            counters: &mut counters,
        };
        resolve_ordinary_patch_envelope(&mut resolution)?
    };
    Ok(LiveExecutionAssembly {
        patch_envelope,
        counters,
        core_execution: Some(core_execution),
        next_live_view: live_view.clone(),
    })
}

fn resolve_ordinary_patch_envelope(
    resolution: &mut OrdinaryPatchResolution<'_>,
) -> Result<ViewShapePatchEnvelope, ViewShapeLiveError> {
    match (
        resolution.family,
        resolution.core_execution.patch_envelope().payload(),
    ) {
        (LiveViewShapeFamily::Table, LivePatchPayload::OrderedCollection(patch)) => {
            Ok(resolve_table_patch(resolution, patch))
        }
        (LiveViewShapeFamily::Detail, LivePatchPayload::Detail(patch)) => {
            Ok(resolve_detail_patch(resolution, patch))
        }
        (LiveViewShapeFamily::InspectorDetailObserved, LivePatchPayload::Detail(patch)) => {
            Ok(resolve_observed_inspector_patch(resolution, patch))
        }
        (LiveViewShapeFamily::InspectorDetailFocused, LivePatchPayload::Detail(patch)) => {
            resolve_focused_inspector_patch(resolution, patch)
        }
        (LiveViewShapeFamily::InspectorDetailFocused, LivePatchPayload::Refresh(_)) => {
            Err(reject_focused_inspector_refresh(resolution))
        }
        (_, LivePatchPayload::Refresh(fallback)) => Ok(resolve_refresh_patch(resolution, fallback)),
        (_, LivePatchPayload::Suppressed(reason)) => {
            Ok(resolve_suppressed_patch(resolution, reason))
        }
        _ => Err(reject_incompatible_core_payload(resolution)),
    }
}

fn resolve_table_patch(
    resolution: &mut OrdinaryPatchResolution<'_>,
    patch: &OrderedCollectionPatch,
) -> ViewShapePatchEnvelope {
    let width = ordered_patch_width(patch);
    resolution.counters.set_view_patch_width(width);
    resolution.counters.set_view_delivery_width(width);
    ViewShapePatchEnvelope::new(
        resolution.family,
        Some(ViewShapePatchFamily::TableRowPatch),
        resolution.core_delivery_digest(),
        hash_parts(&[
            format!("family:{}", resolution.family.as_str()),
            format!("core_replay:{}", resolution.core_replay_digest()),
        ]),
        ViewShapePatchPayload::TableRowPatch(TableRowPatchArtifact::new(
            patch.digest().as_str(),
            width,
        )),
    )
}

fn resolve_detail_patch(
    resolution: &mut OrdinaryPatchResolution<'_>,
    patch: &DetailPatch,
) -> ViewShapePatchEnvelope {
    let width = patch_field_count(patch);
    resolution.counters.set_view_patch_width(width);
    resolution.counters.set_view_delivery_width(width);
    ViewShapePatchEnvelope::new(
        resolution.family,
        Some(ViewShapePatchFamily::DetailFieldPatch),
        resolution.core_delivery_digest(),
        hash_parts(&[
            format!("family:{}", resolution.family.as_str()),
            format!("core_replay:{}", resolution.core_replay_digest()),
        ]),
        ViewShapePatchPayload::DetailFieldPatch(DetailFieldPatchArtifact::new(
            patch.digest().as_str(),
            width,
        )),
    )
}

fn resolve_observed_inspector_patch(
    resolution: &mut OrdinaryPatchResolution<'_>,
    patch: &DetailPatch,
) -> ViewShapePatchEnvelope {
    let width = patch_field_count(patch);
    resolution.counters.set_view_patch_width(width);
    resolution.counters.set_view_delivery_width(width);
    resolution
        .counters
        .set_observed_inspector_delivery_width(width);
    let inspector_identity = resolution.live_view.inspector_identity().cloned();
    ViewShapePatchEnvelope::new(
        resolution.family,
        Some(ViewShapePatchFamily::ObservedInspectorPatch),
        resolution.core_delivery_digest(),
        hash_parts(&[
            format!("family:{}", resolution.family.as_str()),
            format!("core_replay:{}", resolution.core_replay_digest()),
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

fn resolve_focused_inspector_patch(
    resolution: &mut OrdinaryPatchResolution<'_>,
    patch: &DetailPatch,
) -> Result<ViewShapePatchEnvelope, ViewShapeLiveError> {
    let focus_aspect = resolution
        .live_view
        .plan()
        .delivery_metadata()
        .native_focus_aspect_key()
        .expect("focused inspector planning guarantees a native focus aspect")
        .clone();
    let focused = focus_projection(patch.field_deltas(), &focus_aspect).map_err(|received| {
        resolution.counters.add_focused_inspector_widening_denial();
        resolution.counters.add_view_family_fallback_denial();
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
            resolution.counters.clone(),
        )
    })?;
    resolution
        .counters
        .set_focused_inspector_projection_width(patch.field_deltas().len());
    resolution
        .counters
        .set_focused_inspector_aspect_focus_width(focused.len());
    resolution.counters.set_view_patch_width(focused.len());
    resolution.counters.set_view_delivery_width(focused.len());
    let inspector_identity = resolution.live_view.inspector_identity().cloned();
    Ok(ViewShapePatchEnvelope::new(
        resolution.family,
        Some(ViewShapePatchFamily::FocusedInspectorAspectPatch),
        resolution.core_delivery_digest(),
        hash_parts(&[
            format!("family:{}", resolution.family.as_str()),
            format!("core_replay:{}", resolution.core_replay_digest()),
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
    ))
}

fn reject_focused_inspector_refresh(
    resolution: &mut OrdinaryPatchResolution<'_>,
) -> ViewShapeLiveError {
    resolution.counters.add_view_family_refresh_forbidden();
    ViewShapeLiveError::new(
        ViewShapeLiveFailureClass::FocusedInspectorRefreshForbidden,
        "focused inspector may not silently degrade to generic refresh delivery",
        resolution.counters.clone(),
    )
}

fn resolve_refresh_patch(
    resolution: &mut OrdinaryPatchResolution<'_>,
    fallback: &crate::live::RefreshFallback,
) -> ViewShapePatchEnvelope {
    resolution.counters.add_view_refresh_fallback();
    resolution.counters.add_view_family_refresh_admission();
    ViewShapePatchEnvelope::new(
        resolution.family,
        None,
        resolution.core_delivery_digest(),
        hash_parts(&[
            format!("family:{}", resolution.family.as_str()),
            format!("core_replay:{}", resolution.core_replay_digest()),
            format!("refresh:{:?}", fallback.admission_class()),
        ]),
        ViewShapePatchPayload::Refresh(ViewShapeRefreshDisposition::Admitted {
            family: resolution.family,
            fallback: fallback.clone(),
        }),
    )
}

fn resolve_suppressed_patch(
    resolution: &mut OrdinaryPatchResolution<'_>,
    reason: &crate::live::SuppressionReason,
) -> ViewShapePatchEnvelope {
    ViewShapePatchEnvelope::new(
        resolution.family,
        None,
        resolution.core_delivery_digest(),
        hash_parts(&[
            format!("family:{}", resolution.family.as_str()),
            format!("core_replay:{}", resolution.core_replay_digest()),
            format!("suppression:{reason:?}"),
        ]),
        ViewShapePatchPayload::Suppressed(ViewShapeSuppressionDisposition::SuppressedByCore(
            reason.clone(),
        )),
    )
}

fn reject_incompatible_core_payload(
    resolution: &mut OrdinaryPatchResolution<'_>,
) -> ViewShapeLiveError {
    resolution.counters.add_cosmetic_view_semantics_denial();
    ViewShapeLiveError::new(
        ViewShapeLiveFailureClass::UnderlyingLiveFamilyMismatch,
        format!(
            "core payload '{:?}' is incompatible with view family '{}'",
            resolution.core_execution.patch_envelope().payload(),
            resolution.family.as_str()
        ),
        resolution.counters.clone(),
    )
}
