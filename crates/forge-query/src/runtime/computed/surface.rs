use super::refresh_context::ForgeQueryRetainedRefreshContext;
use super::*;
use crate::runtime::retained_rows::decode_single_retained_row;
use crate::runtime::ForgeQueryRuntimeError;

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntity, ForgeQueryEntityIdentity,
};
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedViewMaterialization {
    rows: Vec<Value>,
    published: bool,
}

impl Default for ForgeQueryDerivedViewMaterialization {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            published: false,
        }
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

    pub fn is_published(&self) -> bool {
        self.published
    }

    pub fn replace_rows(&mut self, rows: impl IntoIterator<Item = Value>) {
        self.rows = rows.into_iter().collect();
        self.published = true;
    }

    pub fn push_row(&mut self, row: Value) {
        self.rows.push(row);
        self.published = true;
    }

    pub fn retain_rows(&mut self, mut predicate: impl FnMut(&Value) -> bool) {
        self.rows.retain(|row| predicate(row));
        self.published = true;
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
        let declaration_identity = computed_definition_inspection_identity(
            view.declaration.name(),
            incremental_delivery,
            &upstream_live_views,
            &upstream_derived_views,
            &dependency_aspects,
            &produced_aspects,
        );
        let declaration_digest = declaration_identity.reporting_projection().to_string();
        let dependency_identity = computed_dependency_inspection_identity(
            view.declaration.name(),
            &upstream_live_views,
            &upstream_derived_views,
            &dependency_aspects,
        );
        let dependency_digest = dependency_identity.reporting_projection().to_string();
        let produced_aspect_identity = computed_produced_aspect_inspection_identity(
            view.declaration.name(),
            &produced_aspects,
        );
        let produced_aspect_digest = produced_aspect_identity.reporting_projection().to_string();
        let materialization_identity = computed_materialization_inspection_identity(
            view.declaration.name(),
            materialized_row_count,
            view.materialization.rows(),
        );
        let materialization_digest = materialization_identity.reporting_projection().to_string();
        let patch_identities: Vec<ForgeQueryEvidenceIdentity> = view
            .patches
            .iter()
            .map(|patch| derived_patch_inspection_identity(view.declaration.name(), patch))
            .collect();
        let pending_patch_identity = computed_pending_patch_inspection_identity(
            view.declaration.name(),
            pending_patch_count,
            pending_incremental_patch_count,
            pending_refresh_fallback_count,
            &patch_identities,
        );
        let pending_patch_digest = pending_patch_identity.reporting_projection().to_string();
        let inspection_identity = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_computed_inspection_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("declaration"),
            &declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("dependency"),
            &dependency_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("produced_aspects"),
            &produced_aspect_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("materialization"),
            &materialization_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("pending_patches"),
            &pending_patch_identity,
        )
        .seal();
        let inspection_digest = inspection_identity.reporting_projection().to_string();
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

    pub fn dependency_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_computed_dependency_inspection_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("name"), &self.name)
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("live"),
                self.upstream_live_views.iter().map(String::as_str),
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("derived"),
                self.upstream_derived_views.iter().map(String::as_str),
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("dependencies"),
                self.dependency_aspects.iter().map(String::as_str),
            )
            .seal()
    }

    pub fn produced_aspect_digest(&self) -> &str {
        &self.produced_aspect_digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }

    pub fn materialization_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_computed_materialization_inspection_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("name"), &self.name)
            .field_usize(
                ForgeQueryEvidenceTag::new("rows"),
                self.materialized_row_count,
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("pending_patches"),
                self.pending_patch_count,
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("pending_incremental_patches"),
                self.pending_incremental_patch_count,
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("pending_refresh_fallbacks"),
                self.pending_refresh_fallback_count,
            )
            .seal()
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
    commit_identity: ForgeQueryCommitIdentity,
    authority_lane: ForgeQueryAuthorityLane,
    entity_identity: Option<ForgeQueryEntityIdentity>,
    aspect_paths: Vec<String>,
    family: ForgeQueryDerivedPatchFamily,
    payload: Value,
    reason: Option<String>,
}

impl ForgeQueryDerivedPatch {
    pub fn incremental(
        view_name: impl Into<String>,
        commit_identity: ForgeQueryCommitIdentity,
        entity_identity: ForgeQueryEntityIdentity,
        aspect_paths: impl IntoIterator<Item = String>,
        payload: Value,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity,
            authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            entity_identity: Some(entity_identity),
            aspect_paths: aspect_paths.into_iter().collect(),
            family: ForgeQueryDerivedPatchFamily::Incremental,
            payload,
            reason: None,
        }
    }

    pub fn whole_refresh_fallback(
        view_name: impl Into<String>,
        commit_identity: ForgeQueryCommitIdentity,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity,
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
        commit_identity: ForgeQueryCommitIdentity,
        aspect_paths: impl IntoIterator<Item = String>,
        payload: Value,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity,
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
                self.commit_identity
                    .evidence_identity()
                    .reporting_projection(),
                self.entity_identity
                    .as_ref()
                    .map(|identity| identity.evidence_identity().reporting_projection().to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            ),
            ForgeQueryDerivedPatchFamily::RefreshFallback => format!(
                "whole-refresh-fallback:{}:{}",
                self.commit_identity
                    .evidence_identity()
                    .reporting_projection(),
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

    pub fn commit_identity(&self) -> &ForgeQueryCommitIdentity {
        &self.commit_identity
    }

    pub fn aspect_paths(&self) -> &[String] {
        &self.aspect_paths
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub(in crate::runtime) fn bind_commit_identity(
        &mut self,
        commit_identity: ForgeQueryCommitIdentity,
    ) {
        self.commit_identity = commit_identity;
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
                .unwrap_or_else(|| crate::memory_workspace::admit_authored_entity_label(upstream_view)),
            kind: ForgeQueryMutationKind::Updated,
            aspect_paths: self.aspect_paths.clone(),
        }
    }
}

fn computed_definition_inspection_identity(
    name: &str,
    incremental_delivery: bool,
    upstream_live_views: &[String],
    upstream_derived_views: &[String],
    dependency_aspects: &[String],
    produced_aspects: &[String],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_computed_definition_inspection_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("name"), name)
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            "derived_runtime_state",
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("incremental"),
            incremental_delivery,
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("live"),
            upstream_live_views.iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("derived"),
            upstream_derived_views.iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("dependencies"),
            dependency_aspects.iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("produces"),
            produced_aspects.iter().map(String::as_str),
        )
        .seal()
}

fn computed_dependency_inspection_identity(
    name: &str,
    upstream_live_views: &[String],
    upstream_derived_views: &[String],
    dependency_aspects: &[String],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_computed_dependency_inspection_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("name"), name)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("live"),
            upstream_live_views.iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("derived"),
            upstream_derived_views.iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("dependencies"),
            dependency_aspects.iter().map(String::as_str),
        )
        .seal()
}

fn computed_produced_aspect_inspection_identity(
    name: &str,
    produced_aspects: &[String],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_computed_produced_aspect_inspection_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("name"), name)
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            "derived_runtime_state",
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("produces"),
            produced_aspects.iter().map(String::as_str),
        )
        .seal()
}

fn computed_materialization_inspection_identity(
    name: &str,
    materialized_row_count: usize,
    rows: &[Value],
) -> ForgeQueryEvidenceIdentity {
    let row_shapes: Vec<String> = rows
        .iter()
        .map(|row| serde_json::to_string(row).unwrap_or_else(|_| row.to_string()))
        .collect();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_computed_materialization_inspection_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("name"), name)
        .field_usize(
            ForgeQueryEvidenceTag::new("rows"),
            materialized_row_count,
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("materialized_rows"),
            row_shapes.iter().map(String::as_str),
        )
        .seal()
}

fn computed_pending_patch_inspection_identity(
    name: &str,
    pending_patch_count: usize,
    pending_incremental_patch_count: usize,
    pending_refresh_fallback_count: usize,
    patch_identities: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_computed_pending_patch_inspection_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("name"), name)
        .field_usize(
            ForgeQueryEvidenceTag::new("pending"),
            pending_patch_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("incremental"),
            pending_incremental_patch_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("refresh"),
            pending_refresh_fallback_count,
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("patches"),
            patch_identities.iter(),
        )
        .seal()
}

fn derived_patch_inspection_identity(
    view_name: &str,
    patch: &ForgeQueryDerivedPatch,
) -> ForgeQueryEvidenceIdentity {
    let family = match patch.family {
        ForgeQueryDerivedPatchFamily::Incremental => "incremental",
        ForgeQueryDerivedPatchFamily::RefreshFallback => "refresh_fallback",
    };
    let mut encoder = forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_computed_patch_inspection_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("view_name"), view_name)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family)
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_lane"),
            "derived_runtime_state",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit"),
            &patch.commit_identity.evidence_identity(),
        );
    if let Some(entity) = &patch.entity_identity {
        encoder = encoder.field_evidence_identity(
            ForgeQueryEvidenceTag::new("entity"),
            &entity.evidence_identity(),
        );
    }
    if let Some(reason) = patch.reason.as_deref() {
        encoder = encoder.field_shape(ForgeQueryEvidenceTag::new("reason"), reason);
    }
    if !patch.aspect_paths.is_empty() {
        encoder = encoder.field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_paths"),
            patch.aspect_paths.iter().map(String::as_str),
        );
    }
    encoder.seal()
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
