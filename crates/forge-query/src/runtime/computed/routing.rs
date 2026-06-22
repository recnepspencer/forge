use super::*;
use crate::runtime::ForgeQueryAspectTouch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct ForgeQueryComputedRouteResult {
    affected_view_ids: Vec<String>,
    refresh_fallback: bool,
    considered_view_count: usize,
}

impl ForgeQueryComputedRouteResult {
    pub(in crate::runtime) fn new(
        affected_view_ids: Vec<String>,
        refresh_fallback: bool,
        considered_view_count: usize,
    ) -> Self {
        Self {
            affected_view_ids,
            refresh_fallback,
            considered_view_count,
        }
    }

    pub(in crate::runtime) fn affected_view_ids(self) -> Vec<String> {
        self.affected_view_ids
    }

    pub(in crate::runtime) fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }

    pub(in crate::runtime) fn considered_view_count(&self) -> usize {
        self.considered_view_count
    }
}

pub(in crate::runtime) fn admit_derived_view_declaration(
    derived_views: &BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    live_view_names: &BTreeSet<String>,
    declaration: &ForgeQueryDerivedView,
) -> Result<(), ForgeQueryComputedAdmissionError> {
    let name = declaration.name();
    for upstream in declaration.upstream_live_views() {
        if !live_view_names.contains(upstream) {
            return Err(ForgeQueryComputedAdmissionError::MissingUpstreamLive {
                upstream: upstream.clone(),
            });
        }
    }
    for upstream in declaration.upstream_derived_views() {
        if upstream == name {
            return Err(ForgeQueryComputedAdmissionError::SelfDependency);
        }
        if !derived_views.contains_key(upstream) {
            return Err(ForgeQueryComputedAdmissionError::MissingUpstreamComputed {
                upstream: upstream.clone(),
            });
        }
        if reaches_derived_view(derived_views, upstream, name, &mut BTreeSet::new()) {
            return Err(ForgeQueryComputedAdmissionError::Cycle {
                upstream: upstream.clone(),
            });
        }
    }
    Ok(())
}

pub(in crate::runtime) fn insert_derived_runtime(
    derived_views: &mut BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    dependency_index: &mut ForgeQueryComputedDependencyIndex,
    view: ForgeQueryDerivedView,
    maintainer: Option<Box<dyn ForgeQueryDerivedViewMaintainer>>,
) {
    dependency_index.register(&view);
    derived_views.insert(
        view.name().to_string(),
        ForgeQueryDerivedViewRuntime::new(view, maintainer),
    );
}

pub(in crate::runtime) fn route_derived_view_patches(
    derived_views: &mut BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    dependency_index: &ForgeQueryComputedDependencyIndex,
    candidate_live_view_names: impl IntoIterator<Item = String>,
    retained_live_rows: &BTreeMap<String, Vec<crate::memory_workspace::ForgeQueryEntity>>,
    receipt: &ForgeQueryMutationReceipt,
    mutation_metadata: &crate::runtime::ForgeQueryMutationMetadata,
) -> ForgeQueryComputedRouteResult {
    let mut affected = Vec::new();
    let mut refresh_fallback = false;
    let mut considered_view_count = 0;
    let mut candidates = dependency_index.live_candidates(candidate_live_view_names);
    let mut emitted_by_view: BTreeMap<String, Vec<ForgeQueryDerivedPatch>> = BTreeMap::new();
    let refresh = ForgeQueryRetainedRefreshContext::from_mutation(
        receipt.commit_identity.clone(),
        receipt.snapshot_identity.clone(),
        receipt
            .deltas
            .iter()
            .flat_map(|delta| delta.admitted_touched_aspects().iter().cloned()),
        mutation_metadata.clone(),
    );

    for view_name in topological_derived_order(derived_views) {
        if !candidates.contains(&view_name) {
            continue;
        }
        let Some(view) = derived_views.get(&view_name) else {
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
            let Some(view) = derived_views.get_mut(&view_name) else {
                continue;
            };
            affected.push(view.declaration.name().to_string());
            let mut patch = if let Some(maintainer) = view.maintainer.as_mut() {
                maintainer
                    .refresh_from_upstreams(
                        &view.declaration,
                        &refresh,
                        &upstreams,
                        &mut view.materialization,
                    )
                    .unwrap_or_else(|| {
                        ForgeQueryDerivedPatch::whole_refresh_fallback(
                            view.declaration.name(),
                            receipt.commit_identity.clone(),
                            "derived view declared whole-refresh fallback",
                        )
                    })
            } else {
                ForgeQueryDerivedPatch::whole_refresh_fallback(
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
                .entry(view.declaration.name().to_string())
                .or_default()
                .push(patch.clone());
            view.patches.push(patch);
            candidates.extend(dependency_index.dependents(&view_name));
            continue;
        }

        let Some(view) = derived_views.get_mut(&view_name) else {
            continue;
        };
        for delta in source_deltas {
            affected.push(view.declaration.name().to_string());
            let mut patch = if let Some(maintainer) = view.maintainer.as_mut() {
                maintainer.maintain(&view.declaration, &delta, &mut view.materialization)
            } else {
                ForgeQueryDerivedPatch::incremental(
                    view.declaration.name(),
                    receipt.commit_identity.clone(),
                    delta.entity_identity.clone(),
                    effective_patch_aspects(&view.declaration, &delta),
                    ForgeQueryDerivedPatchPayload::empty(),
                )
            };
            patch.bind_commit_identity(receipt.commit_identity.clone());
            if patch.is_refresh_fallback() {
                refresh_fallback = true;
            }
            emitted_by_view
                .entry(view.declaration.name().to_string())
                .or_default()
                .push(patch.clone());
            view.patches.push(patch);
        }
        if emitted_by_view.contains_key(&view_name) {
            candidates.extend(dependency_index.dependents(&view_name));
        }
    }

    affected.sort();
    affected.dedup();
    ForgeQueryComputedRouteResult::new(affected, refresh_fallback, considered_view_count)
}

pub(in crate::runtime) fn retained_live_view_names_for_candidates(
    derived_views: &BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    dependency_index: &ForgeQueryComputedDependencyIndex,
    candidate_live_view_names: impl IntoIterator<Item = String>,
) -> BTreeSet<String> {
    let candidate_live_view_names = candidate_live_view_names.into_iter().collect::<Vec<_>>();
    let mut retained = candidate_live_view_names
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut pending = dependency_index
        .live_candidates(candidate_live_view_names)
        .into_iter()
        .collect::<Vec<_>>();
    let mut seen_views = BTreeSet::new();

    while let Some(view_name) = pending.pop() {
        if !seen_views.insert(view_name.clone()) {
            continue;
        }
        let Some(view) = derived_views.get(&view_name) else {
            continue;
        };
        retained.extend(view.declaration.upstream_live_views().iter().cloned());
        pending.extend(dependency_index.dependents(&view_name));
    }
    retained
}

fn retained_upstream_inputs(
    declaration: &ForgeQueryDerivedView,
    retained_live_rows: &BTreeMap<String, Vec<crate::memory_workspace::ForgeQueryEntity>>,
    derived_views: &BTreeMap<String, ForgeQueryDerivedViewRuntime>,
) -> ForgeQueryRetainedUpstreamInputs {
    ForgeQueryRetainedUpstreamInputs::from_retained_computed_rows(
        declaration.upstream_live_views().iter().filter_map(|name| {
            retained_live_rows
                .get(name)
                .cloned()
                .map(|rows| (name.clone(), rows))
        }),
        declaration
            .upstream_derived_views()
            .iter()
            .filter_map(|name| {
                derived_views
                    .get(name)
                    .map(|view| (name.clone(), view.materialization.retained_rows().to_vec()))
            }),
    )
}

fn reaches_derived_view(
    derived_views: &BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    start: &str,
    target: &str,
    seen: &mut BTreeSet<String>,
) -> bool {
    if start == target {
        return true;
    }
    if !seen.insert(start.to_string()) {
        return false;
    }
    let Some(view) = derived_views.get(start) else {
        return false;
    };
    view.declaration
        .upstream_derived_views()
        .iter()
        .any(|upstream| reaches_derived_view(derived_views, upstream, target, seen))
}

fn topological_derived_order(
    derived_views: &BTreeMap<String, ForgeQueryDerivedViewRuntime>,
) -> Vec<String> {
    let mut ordered = Vec::new();
    let mut permanent = BTreeSet::new();
    let mut temporary = BTreeSet::new();
    for name in derived_views.keys() {
        visit_derived_view(
            name,
            derived_views,
            &mut permanent,
            &mut temporary,
            &mut ordered,
        );
    }
    ordered
}

fn visit_derived_view(
    name: &str,
    derived_views: &BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    permanent: &mut BTreeSet<String>,
    temporary: &mut BTreeSet<String>,
    ordered: &mut Vec<String>,
) {
    if permanent.contains(name) || !temporary.insert(name.to_string()) {
        return;
    }
    if let Some(view) = derived_views.get(name) {
        for upstream in view.declaration.upstream_derived_views() {
            visit_derived_view(upstream, derived_views, permanent, temporary, ordered);
        }
    }
    temporary.remove(name);
    permanent.insert(name.to_string());
    ordered.push(name.to_string());
}

fn relevant_source_deltas(
    view: &ForgeQueryDerivedViewRuntime,
    receipt: &ForgeQueryMutationReceipt,
    emitted_by_view: &BTreeMap<String, Vec<ForgeQueryDerivedPatch>>,
) -> Vec<ForgeQueryMutationDelta> {
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
        let Some(upstream_patches) = emitted_by_view.get(upstream) else {
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
    view: &ForgeQueryDerivedViewRuntime,
    delta: &ForgeQueryMutationDelta,
) -> bool {
    delta.admitted_touched_aspects().is_empty()
        || delta.admitted_touched_aspects().iter().any(|aspect_touch| {
            view.declaration
                .dependency_aspect_touches()
                .iter()
                .any(|dependency| dependency.matches_or_contains(aspect_touch))
        })
}

fn effective_patch_aspects(
    view: &ForgeQueryDerivedView,
    delta: &ForgeQueryMutationDelta,
) -> Vec<ForgeQueryAspectTouch> {
    if !view.produced_aspect_touches().is_empty() {
        view.produced_aspect_touches().to_vec()
    } else {
        delta.admitted_touched_aspects().to_vec()
    }
}
