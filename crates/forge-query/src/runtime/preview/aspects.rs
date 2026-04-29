use super::*;

pub(super) fn relevant_live_aspects(
    request: &DeclarativeLiveQueryRequest,
    deltas: &[ForgeQueryMutationDelta],
) -> Vec<String> {
    let mut aspects = BTreeSet::new();
    for delta in deltas {
        if delta.collection != request.target() {
            continue;
        }
        if delta.aspect_paths.is_empty()
            || matches!(
                delta.kind,
                ForgeQueryMutationKind::Created | ForgeQueryMutationKind::Deleted
            )
        {
            for field in request.projection() {
                aspects.insert(format!("{}.{}", field.aspect(), field.field()));
            }
            continue;
        }
        for changed in &delta.aspect_paths {
            if request.projection().iter().any(|field| {
                let projected = format!("{}.{}", field.aspect(), field.field());
                changed == &projected
                    || changed.starts_with(&format!("{}.", field.aspect()))
                    || projected.starts_with(&format!("{changed}."))
            }) {
                aspects.insert(changed.clone());
            }
        }
    }
    aspects.into_iter().collect()
}

pub(super) fn relevant_computed_aspects(
    runtime: &ForgeQueryDerivedViewRuntime,
    live_affected: &BTreeMap<String, Vec<String>>,
    computed_affected: &BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let mut matched = BTreeSet::new();
    for upstream in runtime.declaration.upstream_live_views() {
        if let Some(aspects) = live_affected.get(upstream) {
            matched.extend(aspects.iter().cloned());
        }
    }
    for upstream in runtime.declaration.upstream_derived_views() {
        if let Some(aspects) = computed_affected.get(upstream) {
            matched.extend(aspects.iter().cloned());
        }
    }
    if matched.is_empty() {
        return Vec::new();
    }
    if !runtime.declaration.produced_aspects().is_empty() {
        runtime.declaration.produced_aspects().to_vec()
    } else {
        matched.into_iter().collect()
    }
}

pub(super) fn relevant_effect_aspects(
    inspected: &ForgeQueryEffectInspectionEvidence,
    live_affected: &BTreeMap<String, Vec<String>>,
    computed_affected: &BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let source_aspects = match inspected.trigger_source_kind() {
        ForgeQueryEffectTriggerSourceKind::LiveView => {
            live_affected.get(inspected.trigger_source())
        }
        ForgeQueryEffectTriggerSourceKind::ComputedView => {
            computed_affected.get(inspected.trigger_source())
        }
    };
    let Some(source_aspects) = source_aspects else {
        return Vec::new();
    };
    source_aspects
        .iter()
        .filter(|aspect| {
            inspected.trigger_aspects().iter().any(|declared| {
                aspect == &declared
                    || aspect.starts_with(&format!("{declared}."))
                    || declared.starts_with(&format!("{aspect}."))
            })
        })
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
