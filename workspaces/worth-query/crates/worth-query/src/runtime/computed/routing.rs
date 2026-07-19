use super::*;
use crate::memory_workspace::WorthQueryMutationKind;
use crate::runtime::{
    WorthQueryAspectTouch, WorthQueryDerivedMaterializationTarget, WorthQueryLiveArtifactTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct WorthQueryComputedRouteResult {
    affected_view_targets: Vec<WorthQueryDerivedMaterializationTarget>,
    refresh_fallback: bool,
    considered_view_count: usize,
}

impl WorthQueryComputedRouteResult {
    pub(in crate::runtime) fn new(
        affected_view_targets: Vec<WorthQueryDerivedMaterializationTarget>,
        refresh_fallback: bool,
        considered_view_count: usize,
    ) -> Self {
        Self {
            affected_view_targets,
            refresh_fallback,
            considered_view_count,
        }
    }

    pub(in crate::runtime) fn affected_view_targets(
        self,
    ) -> Vec<WorthQueryDerivedMaterializationTarget> {
        self.affected_view_targets
    }

    pub(in crate::runtime) fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }

    pub(in crate::runtime) fn considered_view_count(&self) -> usize {
        self.considered_view_count
    }
}

pub(in crate::runtime) fn admit_derived_view_declaration(
    derived_views: &BTreeMap<WorthQueryDerivedMaterializationTarget, WorthQueryDerivedViewRuntime>,
    live_view_targets: &BTreeSet<WorthQueryLiveArtifactTarget>,
    declaration: &WorthQueryDerivedView,
) -> Result<(), WorthQueryComputedAdmissionError> {
    let name = declaration.name();
    for upstream in declaration.upstream_live_views() {
        if !live_view_targets.contains(&WorthQueryLiveArtifactTarget::from_view_name(upstream)) {
            return Err(WorthQueryComputedAdmissionError::MissingUpstreamLive {
                upstream: upstream.clone(),
            });
        }
    }
    for upstream in declaration.upstream_derived_views() {
        if upstream == name {
            return Err(WorthQueryComputedAdmissionError::SelfDependency);
        }
        let upstream_target = WorthQueryDerivedMaterializationTarget::new(upstream);
        if !derived_views.contains_key(&upstream_target) {
            return Err(WorthQueryComputedAdmissionError::MissingUpstreamComputed {
                upstream: upstream.clone(),
            });
        }
        if reaches_derived_view(
            derived_views,
            &upstream_target,
            &WorthQueryDerivedMaterializationTarget::new(name),
            &mut BTreeSet::new(),
        ) {
            return Err(WorthQueryComputedAdmissionError::Cycle {
                upstream: upstream.clone(),
            });
        }
    }
    Ok(())
}

pub(in crate::runtime) fn insert_derived_runtime(
    derived_views: &mut BTreeMap<
        WorthQueryDerivedMaterializationTarget,
        WorthQueryDerivedViewRuntime,
    >,
    dependency_index: &mut WorthQueryComputedDependencyIndex,
    view: WorthQueryDerivedView,
    maintainer: Option<Box<dyn WorthQueryDerivedViewMaintainer>>,
) {
    dependency_index.register(&view);
    let target = WorthQueryDerivedMaterializationTarget::new(view.name());
    derived_views.insert(target, WorthQueryDerivedViewRuntime::new(view, maintainer));
}

pub(in crate::runtime) fn route_derived_view_patches(
    derived_views: &mut BTreeMap<
        WorthQueryDerivedMaterializationTarget,
        WorthQueryDerivedViewRuntime,
    >,
    dependency_index: &WorthQueryComputedDependencyIndex,
    candidate_live_view_targets: impl IntoIterator<Item = WorthQueryLiveArtifactTarget>,
    retained_live_rows: &BTreeMap<
        WorthQueryLiveArtifactTarget,
        Vec<crate::memory_workspace::WorthQueryEntity>,
    >,
    receipt: &WorthQueryMutationReceipt,
    mutation_metadata: &crate::runtime::WorthQueryMutationMetadata,
) -> WorthQueryComputedRouteResult {
    let mut affected = Vec::new();
    let mut refresh_fallback = false;
    let mut considered_view_count = 0;
    let mut candidates = dependency_index.live_candidates(candidate_live_view_targets);
    let mut emitted_by_view: BTreeMap<
        WorthQueryDerivedMaterializationTarget,
        Vec<WorthQueryDerivedPatch>,
    > = BTreeMap::new();
    let refresh = WorthQueryRetainedRefreshContext::from_mutation(
        receipt.commit_identity.clone(),
        receipt.snapshot_identity.clone(),
        receipt
            .deltas
            .iter()
            .flat_map(|delta| delta.admitted_touched_aspects().iter().cloned()),
        mutation_metadata.clone(),
    );

    for view_target in topological_derived_order(derived_views) {
        if !candidates.contains(&view_target) {
            continue;
        }
        let Some(view) = derived_views.get(&view_target) else {
            continue;
        };
        considered_view_count += 1;
        let source_deltas = relevant_source_deltas(view, receipt, &emitted_by_view);
        if source_deltas.is_empty() {
            continue;
        }

        if !view.declaration.incremental() {
            let upstreams =
                retained_upstream_inputs(&view.declaration, retained_live_rows, derived_views);
            let Some(view) = derived_views.get_mut(&view_target) else {
                continue;
            };
            affected.push(view_target.clone());
            let mut patch = if let Some(maintainer) = view.maintainer.as_mut() {
                maintainer
                    .refresh_from_upstreams(
                        &view.declaration,
                        &refresh,
                        &upstreams,
                        &mut view.materialization,
                    )
                    .unwrap_or_else(|| {
                        WorthQueryDerivedPatch::whole_refresh_fallback(
                            view.declaration.name(),
                            receipt.commit_identity.clone(),
                            "derived view declared whole-refresh fallback",
                        )
                    })
            } else {
                WorthQueryDerivedPatch::whole_refresh_fallback(
                    view.declaration.name(),
                    receipt.commit_identity.clone(),
                    "derived view declared whole-refresh fallback",
                )
            };
            patch.bind_commit_identity(receipt.commit_identity.clone());
            if patch.is_refresh_fallback() {
                refresh_fallback = true;
            }
            emitted_by_view
                .entry(view_target.clone())
                .or_default()
                .push(patch.clone());
            view.patches.push(patch);
            candidates.extend(dependency_index.dependents(&view_target));
            continue;
        }

        let Some(view) = derived_views.get_mut(&view_target) else {
            continue;
        };
        for delta in source_deltas {
            affected.push(view_target.clone());
            let mut patch = if let Some(maintainer) = view.maintainer.as_mut() {
                maintainer.maintain(&view.declaration, &delta, &mut view.materialization)
            } else {
                WorthQueryDerivedPatch::incremental(
                    view.declaration.name(),
                    receipt.commit_identity.clone(),
                    delta.entity_identity.clone(),
                    effective_patch_aspects(&view.declaration, &delta),
                    WorthQueryDerivedPatchPayload::empty(),
                )
            };
            patch.bind_commit_identity(receipt.commit_identity.clone());
            if patch.is_refresh_fallback() {
                refresh_fallback = true;
            }
            emitted_by_view
                .entry(view_target.clone())
                .or_default()
                .push(patch.clone());
            view.patches.push(patch);
        }
        if emitted_by_view.contains_key(&view_target) {
            candidates.extend(dependency_index.dependents(&view_target));
        }
    }

    affected.sort();
    affected.dedup();
    WorthQueryComputedRouteResult::new(affected, refresh_fallback, considered_view_count)
}

pub(in crate::runtime) fn retained_live_view_names_for_candidates(
    derived_views: &BTreeMap<WorthQueryDerivedMaterializationTarget, WorthQueryDerivedViewRuntime>,
    dependency_index: &WorthQueryComputedDependencyIndex,
    candidate_live_view_targets: impl IntoIterator<Item = WorthQueryLiveArtifactTarget>,
) -> BTreeSet<WorthQueryLiveArtifactTarget> {
    let candidate_live_view_targets = candidate_live_view_targets.into_iter().collect::<Vec<_>>();
    let mut retained = candidate_live_view_targets
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut pending = dependency_index
        .live_candidates(candidate_live_view_targets)
        .into_iter()
        .collect::<Vec<_>>();
    let mut seen_views = BTreeSet::new();

    while let Some(view_target) = pending.pop() {
        if !seen_views.insert(view_target.clone()) {
            continue;
        }
        let Some(view) = derived_views.get(&view_target) else {
            continue;
        };
        retained.extend(
            view.declaration
                .upstream_live_views()
                .iter()
                .cloned()
                .map(WorthQueryLiveArtifactTarget::from_view_name),
        );
        pending.extend(dependency_index.dependents(&view_target));
    }
    retained
}

fn retained_upstream_inputs(
    declaration: &WorthQueryDerivedView,
    retained_live_rows: &BTreeMap<
        WorthQueryLiveArtifactTarget,
        Vec<crate::memory_workspace::WorthQueryEntity>,
    >,
    derived_views: &BTreeMap<WorthQueryDerivedMaterializationTarget, WorthQueryDerivedViewRuntime>,
) -> WorthQueryRetainedUpstreamInputs {
    WorthQueryRetainedUpstreamInputs::from_retained_computed_rows(
        declaration.upstream_live_views().iter().filter_map(|name| {
            let target = WorthQueryLiveArtifactTarget::from_view_name(name);
            retained_live_rows
                .get(&target)
                .cloned()
                .map(|rows| (target, rows))
        }),
        declaration
            .upstream_derived_views()
            .iter()
            .filter_map(|name| {
                let target = WorthQueryDerivedMaterializationTarget::new(name);
                derived_views
                    .get(&target)
                    .map(|view| (target, view.materialization.retained_rows().to_vec()))
            }),
    )
}

fn reaches_derived_view(
    derived_views: &BTreeMap<WorthQueryDerivedMaterializationTarget, WorthQueryDerivedViewRuntime>,
    start: &WorthQueryDerivedMaterializationTarget,
    target: &WorthQueryDerivedMaterializationTarget,
    seen: &mut BTreeSet<WorthQueryDerivedMaterializationTarget>,
) -> bool {
    if start == target {
        return true;
    }
    if !seen.insert(start.clone()) {
        return false;
    }
    let Some(view) = derived_views.get(start) else {
        return false;
    };
    view.declaration
        .upstream_derived_views()
        .iter()
        .any(|upstream| {
            reaches_derived_view(
                derived_views,
                &WorthQueryDerivedMaterializationTarget::new(upstream),
                target,
                seen,
            )
        })
}

fn topological_derived_order(
    derived_views: &BTreeMap<WorthQueryDerivedMaterializationTarget, WorthQueryDerivedViewRuntime>,
) -> Vec<WorthQueryDerivedMaterializationTarget> {
    let mut ordered = Vec::new();
    let mut permanent = BTreeSet::new();
    let mut temporary = BTreeSet::new();
    for target in derived_views.keys() {
        visit_derived_view(
            target,
            derived_views,
            &mut permanent,
            &mut temporary,
            &mut ordered,
        );
    }
    ordered
}

fn visit_derived_view(
    target: &WorthQueryDerivedMaterializationTarget,
    derived_views: &BTreeMap<WorthQueryDerivedMaterializationTarget, WorthQueryDerivedViewRuntime>,
    permanent: &mut BTreeSet<WorthQueryDerivedMaterializationTarget>,
    temporary: &mut BTreeSet<WorthQueryDerivedMaterializationTarget>,
    ordered: &mut Vec<WorthQueryDerivedMaterializationTarget>,
) {
    if permanent.contains(target) || !temporary.insert(target.clone()) {
        return;
    }
    if let Some(view) = derived_views.get(target) {
        for upstream in view.declaration.upstream_derived_views() {
            visit_derived_view(
                &WorthQueryDerivedMaterializationTarget::new(upstream),
                derived_views,
                permanent,
                temporary,
                ordered,
            );
        }
    }
    temporary.remove(target);
    permanent.insert(target.clone());
    ordered.push(target.clone());
}

fn relevant_source_deltas(
    view: &WorthQueryDerivedViewRuntime,
    receipt: &WorthQueryMutationReceipt,
    emitted_by_view: &BTreeMap<WorthQueryDerivedMaterializationTarget, Vec<WorthQueryDerivedPatch>>,
) -> Vec<WorthQueryMutationDelta> {
    let mut deltas = Vec::new();
    let accepts_authoritative_deltas = view.declaration.upstream_derived_views().is_empty();
    if accepts_authoritative_deltas {
        deltas.extend(
            receipt
                .deltas
                .iter()
                .filter(|delta| mutation_delta_matches_view(view, delta))
                .cloned(),
        );
    }

    for upstream in view.declaration.upstream_derived_views() {
        let upstream_target = WorthQueryDerivedMaterializationTarget::new(upstream);
        let Some(upstream_patches) = emitted_by_view.get(&upstream_target) else {
            continue;
        };
        for patch in upstream_patches {
            let delta = patch.to_mutation_delta(upstream);
            if mutation_delta_matches_view(view, &delta) {
                deltas.push(delta);
            }
        }
    }

    deltas
}

fn mutation_delta_matches_view(
    view: &WorthQueryDerivedViewRuntime,
    delta: &WorthQueryMutationDelta,
) -> bool {
    if matches!(
        delta.kind(),
        WorthQueryMutationKind::Created | WorthQueryMutationKind::Deleted
    ) {
        return true;
    }
    delta.admitted_touched_aspects().is_empty()
        || delta.admitted_touched_aspects().iter().any(|aspect_touch| {
            view.declaration
                .dependency_aspect_touches()
                .iter()
                .any(|dependency| dependency.matches_or_contains(aspect_touch))
        })
}

fn effective_patch_aspects(
    view: &WorthQueryDerivedView,
    delta: &WorthQueryMutationDelta,
) -> Vec<WorthQueryAspectTouch> {
    if !view.produced_aspect_touches().is_empty() {
        view.produced_aspect_touches().to_vec()
    } else {
        delta.admitted_touched_aspects().to_vec()
    }
}
