use super::refresh_context::ForgeQueryRetainedRefreshContext;
use super::*;
use forge_foundational::facade::AspectValue;

use crate::runtime::{
    ForgeQueryAspectTouch, ForgeQueryDerivedMaterializationTarget, ForgeQueryLiveArtifactTarget,
    ForgeQueryLiveView, ForgeQueryRetainedFieldPath, ForgeQueryRetainedMaterializedRow,
    ForgeQueryRuntimeError,
};

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntity, ForgeQueryEntityIdentity,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedViewMaterialization {
    rows: Vec<ForgeQueryRetainedMaterializedRow>,
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
    live_rows: BTreeMap<ForgeQueryLiveArtifactTarget, Vec<ForgeQueryEntity>>,
    computed_rows:
        BTreeMap<ForgeQueryDerivedMaterializationTarget, Vec<ForgeQueryRetainedMaterializedRow>>,
}

impl ForgeQueryRetainedUpstreamInputs {
    pub(in crate::runtime) fn new(
        live_rows: impl IntoIterator<Item = (ForgeQueryLiveArtifactTarget, Vec<ForgeQueryEntity>)>,
        computed_rows: impl IntoIterator<
            Item = (
                ForgeQueryDerivedMaterializationTarget,
                Vec<ForgeQueryRetainedMaterializedRow>,
            ),
        >,
    ) -> Self {
        Self {
            live_rows: live_rows.into_iter().collect(),
            computed_rows: computed_rows.into_iter().collect(),
        }
    }

    pub(in crate::runtime) fn from_retained_computed_rows(
        live_rows: impl IntoIterator<Item = (ForgeQueryLiveArtifactTarget, Vec<ForgeQueryEntity>)>,
        computed_rows: impl IntoIterator<
            Item = (
                ForgeQueryDerivedMaterializationTarget,
                Vec<ForgeQueryRetainedMaterializedRow>,
            ),
        >,
    ) -> Self {
        Self::new(live_rows, computed_rows)
    }

    pub fn live_rows_for<T>(&self, view: &ForgeQueryLiveView<T>) -> Option<&[ForgeQueryEntity]> {
        self.live_rows
            .get(
                &ForgeQueryLiveArtifactTarget::from_subscription_installation(
                    view.subscription_installation(),
                ),
            )
            .map(Vec::as_slice)
    }

    fn live_rows_by_name(&self, view_name: &str) -> Option<&[ForgeQueryEntity]> {
        self.live_rows
            .get(&ForgeQueryLiveArtifactTarget::from_view_name(view_name))
            .map(Vec::as_slice)
    }

    pub fn declared_live_rows_for<T>(
        &self,
        declaration: &ForgeQueryDerivedView,
        view: &ForgeQueryLiveView<T>,
    ) -> Option<&[ForgeQueryEntity]> {
        self.declared_live_rows_by_name(declaration, view.name())
    }

    pub fn declared_live_row_sets<'a>(
        &'a self,
        declaration: &'a ForgeQueryDerivedView,
    ) -> impl Iterator<Item = &'a [ForgeQueryEntity]> + 'a {
        declaration
            .upstream_live_views()
            .iter()
            .filter_map(|view_name| self.live_rows_by_name(view_name))
    }

    fn declared_live_rows_by_name(
        &self,
        declaration: &ForgeQueryDerivedView,
        view_name: &str,
    ) -> Option<&[ForgeQueryEntity]> {
        declaration
            .upstream_live_views()
            .iter()
            .any(|declared| declared == view_name)
            .then(|| self.live_rows_by_name(view_name))
            .flatten()
    }

    pub fn retained_computed_rows_for<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> &[ForgeQueryRetainedMaterializedRow] {
        self.retained_computed_rows_by_name(view.name())
    }

    fn retained_computed_rows_by_name(
        &self,
        view_name: &str,
    ) -> &[ForgeQueryRetainedMaterializedRow] {
        self.computed_rows
            .get(&ForgeQueryDerivedMaterializationTarget::new(view_name))
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    pub fn declared_retained_computed_rows_for<T>(
        &self,
        declaration: &ForgeQueryDerivedView,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> &[ForgeQueryRetainedMaterializedRow] {
        self.declared_retained_computed_rows_by_name(declaration, view.name())
    }

    pub fn declared_retained_computed_row_sets<'a>(
        &'a self,
        declaration: &'a ForgeQueryDerivedView,
    ) -> impl Iterator<Item = &'a [ForgeQueryRetainedMaterializedRow]> + 'a {
        declaration
            .upstream_derived_views()
            .iter()
            .map(|view_name| self.retained_computed_rows_by_name(view_name))
    }

    fn declared_retained_computed_rows_by_name(
        &self,
        declaration: &ForgeQueryDerivedView,
        view_name: &str,
    ) -> &[ForgeQueryRetainedMaterializedRow] {
        if declaration
            .upstream_derived_views()
            .iter()
            .any(|declared| declared == view_name)
        {
            self.retained_computed_rows_by_name(view_name)
        } else {
            &[]
        }
    }

    pub fn single_retained_computed_row_for<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<&ForgeQueryRetainedMaterializedRow, ForgeQueryRuntimeError> {
        self.single_retained_computed_row_by_name(view.name())
    }

    pub fn single_declared_retained_computed_row_for<T>(
        &self,
        declaration: &ForgeQueryDerivedView,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<&ForgeQueryRetainedMaterializedRow, ForgeQueryRuntimeError> {
        self.single_declared_retained_computed_row_by_name(declaration, view.name())
    }

    fn single_declared_retained_computed_row_by_name(
        &self,
        declaration: &ForgeQueryDerivedView,
        view_name: &str,
    ) -> Result<&ForgeQueryRetainedMaterializedRow, ForgeQueryRuntimeError> {
        if !declaration
            .upstream_derived_views()
            .iter()
            .any(|declared| declared == view_name)
        {
            return Err(ForgeQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "retained-upstream",
                message: "retained computed row was not declared as an upstream".to_string(),
            });
        }
        self.single_retained_computed_row_by_name(view_name)
    }

    fn single_retained_computed_row_by_name(
        &self,
        view_name: &str,
    ) -> Result<&ForgeQueryRetainedMaterializedRow, ForgeQueryRuntimeError> {
        match self.retained_computed_rows_by_name(view_name) {
            [] => Err(ForgeQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "retained-upstream",
                message: "expected one retained row, found none".to_string(),
            }),
            [row] => Ok(row),
            rows => Err(ForgeQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "retained-upstream",
                message: format!("expected one retained row, found {}", rows.len()),
            }),
        }
    }
}

impl ForgeQueryDerivedViewMaterialization {
    pub(in crate::runtime) fn retained_rows(&self) -> &[ForgeQueryRetainedMaterializedRow] {
        &self.rows
    }

    pub fn is_published(&self) -> bool {
        self.published
    }

    pub(in crate::runtime) fn replace_retained_rows(
        &mut self,
        rows: impl IntoIterator<Item = ForgeQueryRetainedMaterializedRow>,
    ) {
        self.rows = rows.into_iter().collect();
        self.published = true;
    }

    pub(in crate::runtime) fn push_retained_row(&mut self, row: ForgeQueryRetainedMaterializedRow) {
        self.rows.push(row);
        self.published = true;
    }

    pub fn replace_retained_scalar_row(
        &mut self,
        scalar_values: impl IntoIterator<Item = (ForgeQueryRetainedFieldPath, AspectValue)>,
    ) -> Result<(), String> {
        let row = retained_materialized_row_from_scalar_values(scalar_values)?;
        self.replace_retained_rows([row]);
        Ok(())
    }

    pub fn push_retained_scalar_row(
        &mut self,
        scalar_values: impl IntoIterator<Item = (ForgeQueryRetainedFieldPath, AspectValue)>,
    ) -> Result<(), String> {
        let row = retained_materialized_row_from_scalar_values(scalar_values)?;
        self.push_retained_row(row);
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryComputedInspectionEvidence {
    name: String,
    authority_lane: ForgeQueryAuthorityLane,
    upstream_live_views: Vec<String>,
    upstream_derived_views: Vec<String>,
    dependency_aspects: Vec<ForgeQueryAspectTouch>,
    produced_aspects: Vec<ForgeQueryAspectTouch>,
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
        let dependency_aspects = view.declaration.dependency_aspect_touches().to_vec();
        let produced_aspects = view.declaration.produced_aspect_touches().to_vec();
        let incremental_delivery = view.declaration.incremental();
        let materialized_row_count = view.materialization.retained_rows().len();
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
            view.materialization.retained_rows(),
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
        let inspection_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
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

    pub fn dependency_aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.dependency_aspects
    }

    pub fn produced_aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
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
                self.dependency_aspects
                    .iter()
                    .map(|touch| touch.admitted_touch_digest_part()),
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
pub struct ForgeQueryDerivedPatchPayload {
    kind: ForgeQueryDerivedPatchPayloadKind,
}

#[derive(Clone, Debug, PartialEq)]
enum ForgeQueryDerivedPatchPayloadKind {
    Empty,
    RetainedRows(Vec<ForgeQueryRetainedMaterializedRow>),
}

impl ForgeQueryDerivedPatchPayload {
    pub fn empty() -> Self {
        Self {
            kind: ForgeQueryDerivedPatchPayloadKind::Empty,
        }
    }

    pub(in crate::runtime) fn from_retained_row(row: ForgeQueryRetainedMaterializedRow) -> Self {
        Self::from_retained_rows([row])
    }

    pub(in crate::runtime) fn from_retained_rows(
        rows: impl IntoIterator<Item = ForgeQueryRetainedMaterializedRow>,
    ) -> Self {
        Self {
            kind: ForgeQueryDerivedPatchPayloadKind::RetainedRows(rows.into_iter().collect()),
        }
    }

    pub fn from_retained_scalar_values(
        scalar_values: impl IntoIterator<Item = (ForgeQueryRetainedFieldPath, AspectValue)>,
    ) -> Result<Self, String> {
        let row = retained_materialized_row_from_scalar_values(scalar_values)?;
        Ok(Self::from_retained_row(row))
    }

    pub(in crate::runtime) fn empty_refresh_fallback() -> Self {
        Self::empty()
    }

    #[cfg(test)]
    pub fn retained_rows(&self) -> &[ForgeQueryRetainedMaterializedRow] {
        match &self.kind {
            ForgeQueryDerivedPatchPayloadKind::RetainedRows(rows) => rows,
            ForgeQueryDerivedPatchPayloadKind::Empty => &[],
        }
    }
}

fn retained_materialized_row_from_scalar_values(
    scalar_values: impl IntoIterator<Item = (ForgeQueryRetainedFieldPath, AspectValue)>,
) -> Result<ForgeQueryRetainedMaterializedRow, String> {
    ForgeQueryRetainedMaterializedRow::from_scalar_values(BTreeMap::from_iter(scalar_values))
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedPatch {
    view_name: String,
    commit_identity: ForgeQueryCommitIdentity,
    authority_lane: ForgeQueryAuthorityLane,
    entity_identity: Option<ForgeQueryEntityIdentity>,
    aspect_touches: Vec<ForgeQueryAspectTouch>,
    family: ForgeQueryDerivedPatchFamily,
    payload: ForgeQueryDerivedPatchPayload,
    reason: Option<String>,
}

impl ForgeQueryDerivedPatch {
    pub fn incremental(
        view_name: impl Into<String>,
        commit_identity: ForgeQueryCommitIdentity,
        entity_identity: ForgeQueryEntityIdentity,
        aspect_touches: impl IntoIterator<Item = ForgeQueryAspectTouch>,
        payload: ForgeQueryDerivedPatchPayload,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity,
            authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            entity_identity: Some(entity_identity),
            aspect_touches: aspect_touches.into_iter().collect(),
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
            aspect_touches: Vec::new(),
            family: ForgeQueryDerivedPatchFamily::RefreshFallback,
            payload: ForgeQueryDerivedPatchPayload::empty_refresh_fallback(),
            reason: Some(reason.into()),
        }
    }

    pub fn whole_refresh_materialized(
        view_name: impl Into<String>,
        commit_identity: ForgeQueryCommitIdentity,
        aspect_touches: impl IntoIterator<Item = ForgeQueryAspectTouch>,
        payload: ForgeQueryDerivedPatchPayload,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity,
            authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            entity_identity: None,
            aspect_touches: aspect_touches.into_iter().collect(),
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
                    .map(|identity| identity
                        .evidence_identity()
                        .reporting_projection()
                        .to_string())
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

    #[cfg(test)]
    pub fn retained_payload_rows(&self) -> &[ForgeQueryRetainedMaterializedRow] {
        self.payload.retained_rows()
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn commit_identity(&self) -> &ForgeQueryCommitIdentity {
        &self.commit_identity
    }

    pub fn aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.aspect_touches
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
        ForgeQueryMutationDelta::from_touched_aspects(
            format!("derived:{upstream_view}"),
            self.entity_identity.clone().unwrap_or_else(|| {
                crate::memory_workspace::admit_authored_entity_label(upstream_view)
            }),
            ForgeQueryMutationKind::Updated,
            self.aspect_touches.clone(),
        )
    }
}

fn computed_definition_inspection_identity(
    name: &str,
    incremental_delivery: bool,
    upstream_live_views: &[String],
    upstream_derived_views: &[String],
    dependency_aspects: &[ForgeQueryAspectTouch],
    produced_aspects: &[ForgeQueryAspectTouch],
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
            dependency_aspects
                .iter()
                .map(ForgeQueryAspectTouch::admitted_touch_digest_part),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("produces"),
            produced_aspects
                .iter()
                .map(ForgeQueryAspectTouch::admitted_touch_digest_part),
        )
        .seal()
}

fn computed_dependency_inspection_identity(
    name: &str,
    upstream_live_views: &[String],
    upstream_derived_views: &[String],
    dependency_aspects: &[ForgeQueryAspectTouch],
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
            dependency_aspects
                .iter()
                .map(ForgeQueryAspectTouch::admitted_touch_digest_part),
        )
        .seal()
}

fn computed_produced_aspect_inspection_identity(
    name: &str,
    produced_aspects: &[ForgeQueryAspectTouch],
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
            produced_aspects
                .iter()
                .map(ForgeQueryAspectTouch::admitted_touch_digest_part),
        )
        .seal()
}

fn computed_materialization_inspection_identity(
    name: &str,
    materialized_row_count: usize,
    rows: &[ForgeQueryRetainedMaterializedRow],
) -> ForgeQueryEvidenceIdentity {
    let row_shapes: Vec<String> = rows
        .iter()
        .map(|row| row.terminal_digest_parts().join(";"))
        .collect();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_computed_materialization_inspection_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("name"), name)
        .field_usize(ForgeQueryEvidenceTag::new("rows"), materialized_row_count)
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
        .field_usize(ForgeQueryEvidenceTag::new("pending"), pending_patch_count)
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
    let mut encoder =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
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
    if !patch.aspect_touches().is_empty() {
        encoder = encoder.field_value_sequence(
            ForgeQueryEvidenceTag::new("admitted_aspect_touches"),
            patch
                .aspect_touches()
                .iter()
                .map(ForgeQueryAspectTouch::admitted_touch_digest_part),
        );
    }
    encoder.seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDerivedViewHandle<T = crate::runtime::ForgeQueryNativeRow> {
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
