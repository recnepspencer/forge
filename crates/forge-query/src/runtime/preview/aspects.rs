use super::*;
use crate::runtime::{ForgeQueryDerivedMaterializationTarget, ForgeQueryLiveArtifactTarget};

pub(super) fn relevant_live_aspects(
    request: &DeclarativeLiveQueryRequest,
    deltas: &[ForgeQueryMutationDelta],
) -> Vec<ForgeQueryAspectTouch> {
    let mut aspects: BTreeSet<ForgeQueryAspectTouch> = BTreeSet::new();
    let request_target = request.target_collection_identity();
    for delta in deltas {
        if !delta
            .target_collection_identity()
            .same_target_collection_as(&request_target)
        {
            continue;
        }
        if delta.admitted_touched_aspects().is_empty()
            || matches!(
                delta.kind,
                ForgeQueryMutationKind::Created | ForgeQueryMutationKind::Deleted
            )
        {
            for field in request.projection() {
                aspects.insert(preview_field_touch(field.source_field_key()));
            }
            continue;
        }
        for touch in delta.admitted_touched_aspects() {
            if request.projection().iter().any(|field| {
                let projected = preview_field_touch(field.source_field_key());
                touch.matches_or_contains(&projected)
            }) {
                aspects.insert(touch.clone());
            }
        }
    }
    aspects.into_iter().collect()
}

pub(super) fn relevant_computed_aspects(
    runtime: &ForgeQueryDerivedViewRuntime,
    live_affected: &BTreeMap<ForgeQueryLiveArtifactTarget, Vec<ForgeQueryAspectTouch>>,
    computed_affected: &BTreeMap<
        ForgeQueryDerivedMaterializationTarget,
        Vec<ForgeQueryAspectTouch>,
    >,
) -> Vec<ForgeQueryAspectTouch> {
    let mut matched = BTreeSet::new();
    for upstream in runtime.declaration.upstream_live_views() {
        let upstream = ForgeQueryLiveArtifactTarget::from_view_name(upstream.to_string());
        if let Some(aspects) = live_affected.get(&upstream) {
            matched.extend(aspects.iter().cloned());
        }
    }
    for upstream in runtime.declaration.upstream_derived_views() {
        let upstream = ForgeQueryDerivedMaterializationTarget::new(upstream.to_string());
        if let Some(aspects) = computed_affected.get(&upstream) {
            matched.extend(aspects.iter().cloned());
        }
    }
    if matched.is_empty() {
        return Vec::new();
    }
    if !runtime.declaration.produced_aspect_touches().is_empty() {
        runtime.declaration.produced_aspect_touches().to_vec()
    } else {
        matched.into_iter().collect()
    }
}

pub(super) fn relevant_effect_aspects(
    inspected: &ForgeQueryEffectInspectionEvidence,
    live_affected: &BTreeMap<ForgeQueryLiveArtifactTarget, Vec<ForgeQueryAspectTouch>>,
    computed_affected: &BTreeMap<
        ForgeQueryDerivedMaterializationTarget,
        Vec<ForgeQueryAspectTouch>,
    >,
) -> Vec<ForgeQueryAspectTouch> {
    let source_aspects = match inspected.trigger_source_kind() {
        ForgeQueryEffectTriggerSourceKind::LiveView => live_affected.get(
            &ForgeQueryLiveArtifactTarget::from_view_name(inspected.trigger_source().to_string()),
        ),
        ForgeQueryEffectTriggerSourceKind::ComputedView => computed_affected.get(
            &ForgeQueryDerivedMaterializationTarget::new(inspected.trigger_source().to_string()),
        ),
    };
    let Some(source_aspects) = source_aspects else {
        return Vec::new();
    };
    source_aspects
        .iter()
        .filter(|aspect| {
            inspected
                .trigger_aspect_touches()
                .iter()
                .any(|declared| declared.matches_or_contains(aspect))
        })
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn preview_field_touch(field: &crate::authoring::AspectFieldKey) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::from_native_parts(
        field.native_aspect_key(),
        Some(forge_foundational::facade::CanonicalFieldPath::single(
            field.native_field_key(),
        )),
    )
}
