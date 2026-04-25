use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;

use serde_json::Value;

use crate::memory_workspace::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
};
use crate::program::ForgeQueryDerivedView;

use super::ForgeQueryAuthorityLane;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ForgeQueryComputedAdmissionError {
    MissingUpstreamLive { upstream: String },
    MissingUpstreamComputed { upstream: String },
    SelfDependency,
    Cycle { upstream: String },
}

impl ForgeQueryComputedAdmissionError {
    pub(super) fn message(&self) -> String {
        match self {
            Self::MissingUpstreamLive { upstream } => {
                format!("live upstream '{upstream}' is not declared")
            }
            Self::MissingUpstreamComputed { upstream } => {
                format!("computed upstream '{upstream}' is not declared")
            }
            Self::SelfDependency => "computed declaration may not depend on itself".to_string(),
            Self::Cycle { upstream } => {
                format!("computed declaration would create a cycle through '{upstream}'")
            }
        }
    }
}

pub(super) struct ForgeQueryDerivedViewRuntime {
    pub(super) declaration: ForgeQueryDerivedView,
    pub(super) patches: Vec<ForgeQueryDerivedPatch>,
    pub(super) materialization: ForgeQueryDerivedViewMaterialization,
    pub(super) maintainer: Option<Box<dyn ForgeQueryDerivedViewMaintainer>>,
}

impl ForgeQueryDerivedViewRuntime {
    pub(super) fn new(
        declaration: ForgeQueryDerivedView,
        maintainer: Option<Box<dyn ForgeQueryDerivedViewMaintainer>>,
    ) -> Self {
        Self {
            declaration,
            patches: Vec::new(),
            materialization: ForgeQueryDerivedViewMaterialization::default(),
            maintainer,
        }
    }
}

#[derive(Default)]
pub(super) struct ForgeQueryComputedDependencyIndex {
    live_to_computed: BTreeMap<String, BTreeSet<String>>,
    computed_to_dependents: BTreeMap<String, BTreeSet<String>>,
    unscoped_authoritative_computed: BTreeSet<String>,
}

impl ForgeQueryComputedDependencyIndex {
    pub(super) fn register(&mut self, declaration: &ForgeQueryDerivedView) {
        self.unregister(declaration.name());
        let view_name = declaration.name().to_string();
        if declaration.upstream_live_views().is_empty()
            && declaration.upstream_derived_views().is_empty()
        {
            self.unscoped_authoritative_computed
                .insert(view_name.clone());
        }
        for live_view in declaration.upstream_live_views() {
            self.live_to_computed
                .entry(live_view.clone())
                .or_default()
                .insert(view_name.clone());
        }
        for upstream_computed in declaration.upstream_derived_views() {
            self.computed_to_dependents
                .entry(upstream_computed.clone())
                .or_default()
                .insert(view_name.clone());
        }
    }

    fn unregister(&mut self, view_name: &str) {
        self.unscoped_authoritative_computed.remove(view_name);
        remove_from_index(&mut self.live_to_computed, view_name);
        remove_from_index(&mut self.computed_to_dependents, view_name);
    }

    fn live_candidates(
        &self,
        live_view_names: impl IntoIterator<Item = String>,
    ) -> BTreeSet<String> {
        let mut candidates = self.unscoped_authoritative_computed.clone();
        for live_view_name in live_view_names {
            if let Some(computed_views) = self.live_to_computed.get(&live_view_name) {
                candidates.extend(computed_views.iter().cloned());
            }
        }
        candidates
    }

    fn dependents(&self, computed_view_name: &str) -> impl Iterator<Item = String> + '_ {
        self.computed_to_dependents
            .get(computed_view_name)
            .into_iter()
            .flatten()
            .cloned()
    }
}

fn remove_from_index(index: &mut BTreeMap<String, BTreeSet<String>>, view_name: &str) {
    let empty_keys: Vec<String> = index
        .iter_mut()
        .filter_map(|(key, values)| {
            values.remove(view_name);
            values.is_empty().then(|| key.clone())
        })
        .collect();
    for key in empty_keys {
        index.remove(&key);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ForgeQueryComputedRouteResult {
    affected_view_ids: Vec<String>,
    refresh_fallback: bool,
    considered_view_count: usize,
}

impl ForgeQueryComputedRouteResult {
    pub(super) fn new(
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

    pub(super) fn affected_view_ids(self) -> Vec<String> {
        self.affected_view_ids
    }

    pub(super) fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }

    pub(super) fn considered_view_count(&self) -> usize {
        self.considered_view_count
    }
}

pub(super) fn admit_derived_view_declaration(
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

pub(super) fn insert_derived_runtime(
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

pub(super) fn route_derived_view_patches(
    derived_views: &mut BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    dependency_index: &ForgeQueryComputedDependencyIndex,
    candidate_live_view_names: impl IntoIterator<Item = String>,
    receipt: &ForgeQueryMutationReceipt,
) -> ForgeQueryComputedRouteResult {
    let mut affected = Vec::new();
    let mut refresh_fallback = false;
    let mut considered_view_count = 0;
    let mut candidates = dependency_index.live_candidates(candidate_live_view_names);
    let mut emitted_by_view: BTreeMap<String, Vec<ForgeQueryDerivedPatch>> = BTreeMap::new();

    for view_name in topological_derived_order(derived_views) {
        if !candidates.contains(&view_name) {
            continue;
        }
        let Some(view) = derived_views.get_mut(&view_name) else {
            continue;
        };
        considered_view_count += 1;
        let source_deltas = relevant_source_deltas(view, receipt, &emitted_by_view);
        for delta in source_deltas {
            affected.push(view.declaration.name().to_string());
            let mut patch = if let Some(maintainer) = view.maintainer.as_mut() {
                maintainer.maintain(&view.declaration, &delta, &mut view.materialization)
            } else if view.declaration.incremental() {
                ForgeQueryDerivedPatch::incremental(
                    view.declaration.name(),
                    receipt.commit_identity.clone(),
                    delta.entity_identity.clone(),
                    effective_patch_aspects(&view.declaration, &delta),
                    Value::Null,
                )
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
        }
        if emitted_by_view.contains_key(&view_name) {
            candidates.extend(dependency_index.dependents(&view_name));
        }
    }

    affected.sort();
    affected.dedup();
    ForgeQueryComputedRouteResult::new(affected, refresh_fallback, considered_view_count)
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
    delta.aspect_paths.is_empty()
        || delta.aspect_paths.iter().any(|aspect_path| {
            view.declaration
                .dependency_aspects()
                .iter()
                .any(|dependency| {
                    aspect_path == dependency
                        || aspect_path.starts_with(&format!("{dependency}."))
                        || dependency.starts_with(&format!("{aspect_path}."))
                })
        })
}

fn effective_patch_aspects(
    view: &ForgeQueryDerivedView,
    delta: &ForgeQueryMutationDelta,
) -> Vec<String> {
    if !view.produced_aspects().is_empty() {
        view.produced_aspects().to_vec()
    } else {
        delta.aspect_paths.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedViewMaterialization {
    rows: Vec<Value>,
}

impl Default for ForgeQueryDerivedViewMaterialization {
    fn default() -> Self {
        Self { rows: Vec::new() }
    }
}

impl ForgeQueryDerivedViewMaterialization {
    pub fn rows(&self) -> &[Value] {
        &self.rows
    }

    pub fn replace_rows(&mut self, rows: impl IntoIterator<Item = Value>) {
        self.rows = rows.into_iter().collect();
    }

    pub fn push_row(&mut self, row: Value) {
        self.rows.push(row);
    }

    pub fn retain_rows(&mut self, mut predicate: impl FnMut(&Value) -> bool) {
        self.rows.retain(|row| predicate(row));
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryComputedInspectionEvidence {
    name: String,
    authority_lane: ForgeQueryAuthorityLane,
    upstream_live_views: Vec<String>,
    upstream_derived_views: Vec<String>,
    dependency_aspects: Vec<String>,
    produced_aspects: Vec<String>,
    materialized_row_count: usize,
    pending_patch_count: usize,
}

impl ForgeQueryComputedInspectionEvidence {
    pub(super) fn from_runtime(view: &ForgeQueryDerivedViewRuntime) -> Self {
        Self {
            name: view.declaration.name().to_string(),
            authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            upstream_live_views: view.declaration.upstream_live_views().to_vec(),
            upstream_derived_views: view.declaration.upstream_derived_views().to_vec(),
            dependency_aspects: view.declaration.dependency_aspects().to_vec(),
            produced_aspects: view.declaration.produced_aspects().to_vec(),
            materialized_row_count: view.materialization.rows().len(),
            pending_patch_count: view.patches.len(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn upstream_live_views(&self) -> &[String] {
        &self.upstream_live_views
    }

    pub fn upstream_derived_views(&self) -> &[String] {
        &self.upstream_derived_views
    }

    pub fn dependency_aspects(&self) -> &[String] {
        &self.dependency_aspects
    }

    pub fn produced_aspects(&self) -> &[String] {
        &self.produced_aspects
    }

    pub fn materialized_row_count(&self) -> usize {
        self.materialized_row_count
    }

    pub fn pending_patch_count(&self) -> usize {
        self.pending_patch_count
    }
}

pub trait ForgeQueryDerivedViewMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch;
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryDerivedPatchFamily {
    Incremental,
    RefreshFallback,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedPatch {
    view_name: String,
    commit_identity: String,
    authority_lane: ForgeQueryAuthorityLane,
    entity_identity: Option<String>,
    aspect_paths: Vec<String>,
    family: ForgeQueryDerivedPatchFamily,
    payload: Value,
    reason: Option<String>,
}

impl ForgeQueryDerivedPatch {
    pub fn incremental(
        view_name: impl Into<String>,
        commit_identity: impl Into<String>,
        entity_identity: impl Into<String>,
        aspect_paths: impl IntoIterator<Item = String>,
        payload: Value,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity: commit_identity.into(),
            authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            entity_identity: Some(entity_identity.into()),
            aspect_paths: aspect_paths.into_iter().collect(),
            family: ForgeQueryDerivedPatchFamily::Incremental,
            payload,
            reason: None,
        }
    }

    pub fn whole_refresh_fallback(
        view_name: impl Into<String>,
        commit_identity: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity: commit_identity.into(),
            authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            entity_identity: None,
            aspect_paths: Vec::new(),
            family: ForgeQueryDerivedPatchFamily::RefreshFallback,
            payload: Value::Null,
            reason: Some(reason.into()),
        }
    }

    pub fn note(&self) -> String {
        match self.family {
            ForgeQueryDerivedPatchFamily::Incremental => format!(
                "incremental:{}:{}",
                self.commit_identity,
                self.entity_identity.as_deref().unwrap_or("unknown")
            ),
            ForgeQueryDerivedPatchFamily::RefreshFallback => format!(
                "whole-refresh-fallback:{}:{}",
                self.commit_identity,
                self.reason.as_deref().unwrap_or("unspecified")
            ),
        }
    }

    pub fn is_refresh_fallback(&self) -> bool {
        self.family == ForgeQueryDerivedPatchFamily::RefreshFallback
    }

    pub fn payload(&self) -> &Value {
        &self.payload
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn commit_identity(&self) -> &str {
        &self.commit_identity
    }

    pub fn aspect_paths(&self) -> &[String] {
        &self.aspect_paths
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    fn bind_commit_identity(&mut self, commit_identity: impl Into<String>) {
        self.commit_identity = commit_identity.into();
    }

    fn to_mutation_delta(&self, upstream_view: &str) -> ForgeQueryMutationDelta {
        ForgeQueryMutationDelta {
            collection: format!("derived:{upstream_view}"),
            entity_identity: self
                .entity_identity
                .clone()
                .unwrap_or_else(|| upstream_view.to_string()),
            kind: ForgeQueryMutationKind::Updated,
            aspect_paths: self.aspect_paths.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDerivedViewHandle<T = Value> {
    name: String,
    authority_lane: ForgeQueryAuthorityLane,
    marker: PhantomData<T>,
}

impl<T> ForgeQueryDerivedViewHandle<T> {
    pub(super) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            marker: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }
}
