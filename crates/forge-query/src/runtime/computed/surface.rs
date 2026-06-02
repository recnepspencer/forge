use super::refresh_context::ForgeQueryRetainedRefreshContext;
use super::*;
use crate::runtime::retained_rows::decode_single_retained_row;
use crate::runtime::ForgeQueryRuntimeError;

use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryEntity;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedViewMaterialization {
    rows: Vec<Value>,
}

impl Default for ForgeQueryDerivedViewMaterialization {
    fn default() -> Self {
        Self { rows: Vec::new() }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForgeQueryRetainedUpstreamInputs {
    live_rows: BTreeMap<String, Vec<ForgeQueryEntity>>,
    computed_rows: BTreeMap<String, Vec<Value>>,
}

impl ForgeQueryRetainedUpstreamInputs {
    pub fn new(
        live_rows: impl IntoIterator<Item = (String, Vec<ForgeQueryEntity>)>,
        computed_rows: impl IntoIterator<Item = (String, Vec<Value>)>,
    ) -> Self {
        Self {
            live_rows: live_rows.into_iter().collect(),
            computed_rows: computed_rows.into_iter().collect(),
        }
    }

    pub fn live_rows(&self, view_name: &str) -> Option<&[ForgeQueryEntity]> {
        self.live_rows.get(view_name).map(Vec::as_slice)
    }

    pub fn computed_rows(&self, view_name: &str) -> Option<&[Value]> {
        self.computed_rows.get(view_name).map(Vec::as_slice)
    }

    pub fn live_view_names(&self) -> impl Iterator<Item = &str> {
        self.live_rows.keys().map(String::as_str)
    }

    pub fn computed_view_names(&self) -> impl Iterator<Item = &str> {
        self.computed_rows.keys().map(String::as_str)
    }

    pub fn decode_single_computed_row<T>(
        &self,
        view_name: &str,
    ) -> Result<T, ForgeQueryRuntimeError>
    where
        T: DeserializeOwned,
    {
        decode_single_retained_row(
            self.computed_rows(view_name).unwrap_or(&[]),
            view_name,
            "retained-upstream",
        )
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
    incremental_delivery: bool,
    materialized_row_count: usize,
    pending_patch_count: usize,
    pending_incremental_patch_count: usize,
    pending_refresh_fallback_count: usize,
    declaration_digest: String,
    dependency_digest: String,
    produced_aspect_digest: String,
    materialization_digest: String,
    pending_patch_digest: String,
    inspection_digest: String,
}

impl ForgeQueryComputedInspectionEvidence {
    pub(in crate::runtime) fn from_runtime(view: &ForgeQueryDerivedViewRuntime) -> Self {
        let upstream_live_views = view.declaration.upstream_live_views().to_vec();
        let upstream_derived_views = view.declaration.upstream_derived_views().to_vec();
        let dependency_aspects = view.declaration.dependency_aspects().to_vec();
        let produced_aspects = view.declaration.produced_aspects().to_vec();
        let incremental_delivery = view.declaration.incremental();
        let materialized_row_count = view.materialization.rows().len();
        let pending_patch_count = view.patches.len();
        let pending_incremental_patch_count = view
            .patches
            .iter()
            .filter(|patch| !patch.is_refresh_fallback())
            .count();
        let pending_refresh_fallback_count = view
            .patches
            .iter()
            .filter(|patch| patch.is_refresh_fallback())
            .count();
        let declaration_digest = hash_parts(&[
            "forge_query_computed_definition_inspection_v1".to_string(),
            format!("name:{}", view.declaration.name()),
            format!("authority:{}", ForgeQueryAuthorityLane::DerivedRuntimeState),
            format!("incremental:{incremental_delivery}"),
            format!("live:{}", upstream_live_views.join("|")),
            format!("derived:{}", upstream_derived_views.join("|")),
            format!("dependencies:{}", dependency_aspects.join("|")),
            format!("produces:{}", produced_aspects.join("|")),
        ]);
        let dependency_digest = hash_parts(&[
            "forge_query_computed_dependency_inspection_v1".to_string(),
            format!("name:{}", view.declaration.name()),
            format!("live:{}", upstream_live_views.join("|")),
            format!("derived:{}", upstream_derived_views.join("|")),
            format!("dependencies:{}", dependency_aspects.join("|")),
        ]);
        let produced_aspect_digest = hash_parts(&[
            "forge_query_computed_produced_aspect_inspection_v1".to_string(),
            format!("name:{}", view.declaration.name()),
            format!("authority:{}", ForgeQueryAuthorityLane::DerivedRuntimeState),
            format!("produces:{}", produced_aspects.join("|")),
        ]);
        let mut materialization_parts = vec![
            "forge_query_computed_materialization_inspection_v1".to_string(),
            format!("name:{}", view.declaration.name()),
            format!("rows:{materialized_row_count}"),
        ];
        materialization_parts.extend(
            view.materialization
                .rows()
                .iter()
                .map(|row| serde_json::to_string(row).unwrap_or_else(|_| row.to_string())),
        );
        let materialization_digest = hash_parts(&materialization_parts);
        let mut pending_patch_parts = vec![
            "forge_query_computed_pending_patch_inspection_v1".to_string(),
            format!("name:{}", view.declaration.name()),
            format!("pending:{pending_patch_count}"),
            format!("incremental:{pending_incremental_patch_count}"),
            format!("refresh:{pending_refresh_fallback_count}"),
        ];
        pending_patch_parts.extend(view.patches.iter().map(|patch| {
            format!(
                "{}:{}:{}:{}",
                patch.note(),
                patch.authority_lane(),
                patch.commit_identity(),
                patch.aspect_paths().join("|")
            )
        }));
        let pending_patch_digest = hash_parts(&pending_patch_parts);
        let inspection_digest = hash_parts(&[
            "forge_query_computed_inspection_v1".to_string(),
            declaration_digest.clone(),
            dependency_digest.clone(),
            produced_aspect_digest.clone(),
            materialization_digest.clone(),
            pending_patch_digest.clone(),
        ]);
        Self {
            name: view.declaration.name().to_string(),
            authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            upstream_live_views,
            upstream_derived_views,
            dependency_aspects,
            produced_aspects,
            incremental_delivery,
            materialized_row_count,
            pending_patch_count,
            pending_incremental_patch_count,
            pending_refresh_fallback_count,
            declaration_digest,
            dependency_digest,
            produced_aspect_digest,
            materialization_digest,
            pending_patch_digest,
            inspection_digest,
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

    pub fn incremental_delivery(&self) -> bool {
        self.incremental_delivery
    }

    pub fn materialized_row_count(&self) -> usize {
        self.materialized_row_count
    }

    pub fn pending_patch_count(&self) -> usize {
        self.pending_patch_count
    }

    pub fn pending_incremental_patch_count(&self) -> usize {
        self.pending_incremental_patch_count
    }

    pub fn pending_refresh_fallback_count(&self) -> usize {
        self.pending_refresh_fallback_count
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn dependency_digest(&self) -> &str {
        &self.dependency_digest
    }

    pub fn produced_aspect_digest(&self) -> &str {
        &self.produced_aspect_digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }

    pub fn pending_patch_digest(&self) -> &str {
        &self.pending_patch_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

pub trait ForgeQueryDerivedViewMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch;

    fn refresh_from_upstreams(
        &mut self,
        _view: &ForgeQueryDerivedView,
        _refresh: &ForgeQueryRetainedRefreshContext,
        _upstreams: &ForgeQueryRetainedUpstreamInputs,
        _materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        None
    }
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

    pub fn whole_refresh_materialized(
        view_name: impl Into<String>,
        commit_identity: impl Into<String>,
        aspect_paths: impl IntoIterator<Item = String>,
        payload: Value,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity: commit_identity.into(),
            authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            entity_identity: None,
            aspect_paths: aspect_paths.into_iter().collect(),
            family: ForgeQueryDerivedPatchFamily::RefreshFallback,
            payload,
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

    pub(in crate::runtime) fn bind_commit_identity(&mut self, commit_identity: impl Into<String>) {
        self.commit_identity = commit_identity.into();
    }

    pub(in crate::runtime) fn to_mutation_delta(
        &self,
        upstream_view: &str,
    ) -> ForgeQueryMutationDelta {
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
    pub(in crate::runtime) fn new(name: impl Into<String>) -> Self {
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
