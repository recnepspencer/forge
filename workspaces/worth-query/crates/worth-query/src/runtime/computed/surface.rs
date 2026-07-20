use super::refresh_context::WorthQueryRetainedRefreshContext;
use super::*;
use worth_foundational::facade::AspectValue;

use crate::runtime::{
    WorthQueryAspectTouch, WorthQueryDerivedMaterializationTarget, WorthQueryLiveArtifactTarget,
    WorthQueryLiveView, WorthQueryRetainedFieldPath, WorthQueryRetainedMaterializedRow,
    WorthQueryRuntimeError,
};

use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::memory_workspace::{
    WorthQueryCommitIdentity, WorthQueryEntity, WorthQueryEntityIdentity,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDerivedViewMaterialization {
    rows: Vec<WorthQueryRetainedMaterializedRow>,
    published: bool,
}

impl Default for WorthQueryDerivedViewMaterialization {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            published: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorthQueryRetainedUpstreamInputs {
    live_rows: BTreeMap<WorthQueryLiveArtifactTarget, Vec<WorthQueryEntity>>,
    computed_rows:
        BTreeMap<WorthQueryDerivedMaterializationTarget, Vec<WorthQueryRetainedMaterializedRow>>,
}

impl WorthQueryRetainedUpstreamInputs {
    pub(in crate::runtime) fn new(
        live_rows: impl IntoIterator<Item = (WorthQueryLiveArtifactTarget, Vec<WorthQueryEntity>)>,
        computed_rows: impl IntoIterator<
            Item = (
                WorthQueryDerivedMaterializationTarget,
                Vec<WorthQueryRetainedMaterializedRow>,
            ),
        >,
    ) -> Self {
        Self {
            live_rows: live_rows.into_iter().collect(),
            computed_rows: computed_rows.into_iter().collect(),
        }
    }

    pub(in crate::runtime) fn from_retained_computed_rows(
        live_rows: impl IntoIterator<Item = (WorthQueryLiveArtifactTarget, Vec<WorthQueryEntity>)>,
        computed_rows: impl IntoIterator<
            Item = (
                WorthQueryDerivedMaterializationTarget,
                Vec<WorthQueryRetainedMaterializedRow>,
            ),
        >,
    ) -> Self {
        Self::new(live_rows, computed_rows)
    }

    pub fn live_rows_for<T>(&self, view: &WorthQueryLiveView<T>) -> Option<&[WorthQueryEntity]> {
        self.live_rows
            .get(
                &WorthQueryLiveArtifactTarget::from_subscription_installation(
                    view.subscription_installation(),
                ),
            )
            .map(Vec::as_slice)
    }

    fn live_rows_by_name(&self, view_name: &str) -> Option<&[WorthQueryEntity]> {
        self.live_rows
            .get(&WorthQueryLiveArtifactTarget::from_view_name(view_name))
            .map(Vec::as_slice)
    }

    pub fn declared_live_rows_for<T>(
        &self,
        declaration: &WorthQueryDerivedView,
        view: &WorthQueryLiveView<T>,
    ) -> Option<&[WorthQueryEntity]> {
        self.declared_live_rows_by_name(declaration, view.name())
    }

    pub fn declared_live_row_sets<'a>(
        &'a self,
        declaration: &'a WorthQueryDerivedView,
    ) -> impl Iterator<Item = &'a [WorthQueryEntity]> + 'a {
        declaration
            .upstream_live_views()
            .iter()
            .filter_map(|view_name| self.live_rows_by_name(view_name))
    }

    fn declared_live_rows_by_name(
        &self,
        declaration: &WorthQueryDerivedView,
        view_name: &str,
    ) -> Option<&[WorthQueryEntity]> {
        declaration
            .upstream_live_views()
            .iter()
            .any(|declared| declared == view_name)
            .then(|| self.live_rows_by_name(view_name))
            .flatten()
    }

    pub fn retained_computed_rows_for<T>(
        &self,
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> &[WorthQueryRetainedMaterializedRow] {
        self.retained_computed_rows_by_name(view.name())
    }

    fn retained_computed_rows_by_name(
        &self,
        view_name: &str,
    ) -> &[WorthQueryRetainedMaterializedRow] {
        self.computed_rows
            .get(&WorthQueryDerivedMaterializationTarget::new(view_name))
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    pub fn declared_retained_computed_rows_for<T>(
        &self,
        declaration: &WorthQueryDerivedView,
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> &[WorthQueryRetainedMaterializedRow] {
        self.declared_retained_computed_rows_by_name(declaration, view.name())
    }

    pub fn declared_retained_computed_row_sets<'a>(
        &'a self,
        declaration: &'a WorthQueryDerivedView,
    ) -> impl Iterator<Item = &'a [WorthQueryRetainedMaterializedRow]> + 'a {
        declaration
            .upstream_derived_views()
            .iter()
            .map(|view_name| self.retained_computed_rows_by_name(view_name))
    }

    fn declared_retained_computed_rows_by_name(
        &self,
        declaration: &WorthQueryDerivedView,
        view_name: &str,
    ) -> &[WorthQueryRetainedMaterializedRow] {
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
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> Result<&WorthQueryRetainedMaterializedRow, WorthQueryRuntimeError> {
        self.single_retained_computed_row_by_name(view.name())
    }

    pub fn single_declared_retained_computed_row_for<T>(
        &self,
        declaration: &WorthQueryDerivedView,
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> Result<&WorthQueryRetainedMaterializedRow, WorthQueryRuntimeError> {
        self.single_declared_retained_computed_row_by_name(declaration, view.name())
    }

    fn single_declared_retained_computed_row_by_name(
        &self,
        declaration: &WorthQueryDerivedView,
        view_name: &str,
    ) -> Result<&WorthQueryRetainedMaterializedRow, WorthQueryRuntimeError> {
        if !declaration
            .upstream_derived_views()
            .iter()
            .any(|declared| declared == view_name)
        {
            return Err(WorthQueryRuntimeError::RetainedRowDecode {
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
    ) -> Result<&WorthQueryRetainedMaterializedRow, WorthQueryRuntimeError> {
        match self.retained_computed_rows_by_name(view_name) {
            [] => Err(WorthQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "retained-upstream",
                message: "expected one retained row, found none".to_string(),
            }),
            [row] => Ok(row),
            rows => Err(WorthQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "retained-upstream",
                message: format!("expected one retained row, found {}", rows.len()),
            }),
        }
    }
}

impl WorthQueryDerivedViewMaterialization {
    pub(in crate::runtime) fn retained_rows(&self) -> &[WorthQueryRetainedMaterializedRow] {
        &self.rows
    }

    pub fn is_published(&self) -> bool {
        self.published
    }

    pub(in crate::runtime) fn replace_retained_rows(
        &mut self,
        rows: impl IntoIterator<Item = WorthQueryRetainedMaterializedRow>,
    ) {
        self.rows = rows.into_iter().collect();
        self.published = true;
    }

    pub(in crate::runtime) fn push_retained_row(&mut self, row: WorthQueryRetainedMaterializedRow) {
        self.rows.push(row);
        self.published = true;
    }

    pub fn replace_retained_scalar_row(
        &mut self,
        scalar_values: impl IntoIterator<Item = (WorthQueryRetainedFieldPath, AspectValue)>,
    ) -> Result<(), String> {
        let row = retained_materialized_row_from_scalar_values(scalar_values)?;
        self.replace_retained_rows([row]);
        Ok(())
    }

    pub fn push_retained_scalar_row(
        &mut self,
        scalar_values: impl IntoIterator<Item = (WorthQueryRetainedFieldPath, AspectValue)>,
    ) -> Result<(), String> {
        let row = retained_materialized_row_from_scalar_values(scalar_values)?;
        self.push_retained_row(row);
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryComputedInspectionEvidence {
    name: String,
    authority_lane: WorthQueryAuthorityLane,
    upstream_live_views: Vec<String>,
    upstream_derived_views: Vec<String>,
    dependency_aspects: Vec<WorthQueryAspectTouch>,
    produced_aspects: Vec<WorthQueryAspectTouch>,
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

impl WorthQueryComputedInspectionEvidence {
    pub(in crate::runtime) fn from_runtime(view: &WorthQueryDerivedViewRuntime) -> Self {
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
        let patch_identities: Vec<WorthQueryEvidenceIdentity> = view
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
            worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "worth_query_computed_inspection_v1",
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("declaration"),
                    &declaration_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("dependency"),
                    &dependency_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("produced_aspects"),
                    &produced_aspect_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("materialization"),
                    &materialization_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("pending_patches"),
                    &pending_patch_identity,
                )
                .seal();
        let inspection_digest = inspection_identity.reporting_projection().to_string();
        Self {
            name: view.declaration.name().to_string(),
            authority_lane: WorthQueryAuthorityLane::DerivedRuntimeState,
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

    pub fn authority_lane(&self) -> WorthQueryAuthorityLane {
        self.authority_lane
    }

    pub fn upstream_live_views(&self) -> &[String] {
        &self.upstream_live_views
    }

    pub fn upstream_derived_views(&self) -> &[String] {
        &self.upstream_derived_views
    }

    pub fn dependency_aspect_touches(&self) -> &[WorthQueryAspectTouch] {
        &self.dependency_aspects
    }

    pub fn produced_aspect_touches(&self) -> &[WorthQueryAspectTouch] {
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

    pub fn dependency_identity(&self) -> WorthQueryEvidenceIdentity {
        worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "worth_query_computed_dependency_inspection_v1",
            )
            .field_shape(WorthQueryEvidenceTag::new("name"), &self.name)
            .field_value_sequence(
                WorthQueryEvidenceTag::new("live"),
                self.upstream_live_views.iter().map(String::as_str),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("derived"),
                self.upstream_derived_views.iter().map(String::as_str),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("dependencies"),
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

    pub fn materialization_identity(&self) -> WorthQueryEvidenceIdentity {
        worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "worth_query_computed_materialization_inspection_v1",
            )
            .field_shape(WorthQueryEvidenceTag::new("name"), &self.name)
            .field_usize(
                WorthQueryEvidenceTag::new("rows"),
                self.materialized_row_count,
            )
            .field_usize(
                WorthQueryEvidenceTag::new("pending_patches"),
                self.pending_patch_count,
            )
            .field_usize(
                WorthQueryEvidenceTag::new("pending_incremental_patches"),
                self.pending_incremental_patch_count,
            )
            .field_usize(
                WorthQueryEvidenceTag::new("pending_refresh_fallbacks"),
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

pub trait WorthQueryDerivedViewMaintainer {
    fn maintain(
        &mut self,
        view: &WorthQueryDerivedView,
        delta: &WorthQueryMutationDelta,
        materialization: &mut WorthQueryDerivedViewMaterialization,
    ) -> WorthQueryDerivedPatch;

    fn refresh_from_upstreams(
        &mut self,
        _view: &WorthQueryDerivedView,
        _refresh: &WorthQueryRetainedRefreshContext,
        _upstreams: &WorthQueryRetainedUpstreamInputs,
        _materialization: &mut WorthQueryDerivedViewMaterialization,
    ) -> Option<WorthQueryDerivedPatch> {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryDerivedPatchFamily {
    Incremental,
    RefreshFallback,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDerivedPatchPayload {
    kind: WorthQueryDerivedPatchPayloadKind,
}

#[derive(Clone, Debug, PartialEq)]
enum WorthQueryDerivedPatchPayloadKind {
    Empty,
    RetainedRows(Vec<WorthQueryRetainedMaterializedRow>),
}

impl WorthQueryDerivedPatchPayload {
    pub fn empty() -> Self {
        Self {
            kind: WorthQueryDerivedPatchPayloadKind::Empty,
        }
    }

    pub(in crate::runtime) fn from_retained_row(row: WorthQueryRetainedMaterializedRow) -> Self {
        Self::from_retained_rows([row])
    }

    pub(in crate::runtime) fn from_retained_rows(
        rows: impl IntoIterator<Item = WorthQueryRetainedMaterializedRow>,
    ) -> Self {
        Self {
            kind: WorthQueryDerivedPatchPayloadKind::RetainedRows(rows.into_iter().collect()),
        }
    }

    pub fn from_retained_scalar_values(
        scalar_values: impl IntoIterator<Item = (WorthQueryRetainedFieldPath, AspectValue)>,
    ) -> Result<Self, String> {
        let row = retained_materialized_row_from_scalar_values(scalar_values)?;
        Ok(Self::from_retained_row(row))
    }

    pub(in crate::runtime) fn empty_refresh_fallback() -> Self {
        Self::empty()
    }

    #[cfg(test)]
    pub fn retained_rows(&self) -> &[WorthQueryRetainedMaterializedRow] {
        match &self.kind {
            WorthQueryDerivedPatchPayloadKind::RetainedRows(rows) => rows,
            WorthQueryDerivedPatchPayloadKind::Empty => &[],
        }
    }
}

fn retained_materialized_row_from_scalar_values(
    scalar_values: impl IntoIterator<Item = (WorthQueryRetainedFieldPath, AspectValue)>,
) -> Result<WorthQueryRetainedMaterializedRow, String> {
    WorthQueryRetainedMaterializedRow::from_scalar_values(BTreeMap::from_iter(scalar_values))
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDerivedPatch {
    view_name: String,
    commit_identity: WorthQueryCommitIdentity,
    authority_lane: WorthQueryAuthorityLane,
    entity_identity: Option<WorthQueryEntityIdentity>,
    aspect_touches: Vec<WorthQueryAspectTouch>,
    family: WorthQueryDerivedPatchFamily,
    payload: WorthQueryDerivedPatchPayload,
    reason: Option<String>,
}

impl WorthQueryDerivedPatch {
    pub fn incremental(
        view_name: impl Into<String>,
        commit_identity: WorthQueryCommitIdentity,
        entity_identity: WorthQueryEntityIdentity,
        aspect_touches: impl IntoIterator<Item = WorthQueryAspectTouch>,
        payload: WorthQueryDerivedPatchPayload,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity,
            authority_lane: WorthQueryAuthorityLane::DerivedRuntimeState,
            entity_identity: Some(entity_identity),
            aspect_touches: aspect_touches.into_iter().collect(),
            family: WorthQueryDerivedPatchFamily::Incremental,
            payload,
            reason: None,
        }
    }

    pub fn whole_refresh_fallback(
        view_name: impl Into<String>,
        commit_identity: WorthQueryCommitIdentity,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity,
            authority_lane: WorthQueryAuthorityLane::DerivedRuntimeState,
            entity_identity: None,
            aspect_touches: Vec::new(),
            family: WorthQueryDerivedPatchFamily::RefreshFallback,
            payload: WorthQueryDerivedPatchPayload::empty_refresh_fallback(),
            reason: Some(reason.into()),
        }
    }

    pub fn whole_refresh_materialized(
        view_name: impl Into<String>,
        commit_identity: WorthQueryCommitIdentity,
        aspect_touches: impl IntoIterator<Item = WorthQueryAspectTouch>,
        payload: WorthQueryDerivedPatchPayload,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity,
            authority_lane: WorthQueryAuthorityLane::DerivedRuntimeState,
            entity_identity: None,
            aspect_touches: aspect_touches.into_iter().collect(),
            family: WorthQueryDerivedPatchFamily::RefreshFallback,
            payload,
            reason: Some(reason.into()),
        }
    }

    pub fn note(&self) -> String {
        match self.family {
            WorthQueryDerivedPatchFamily::Incremental => format!(
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
            WorthQueryDerivedPatchFamily::RefreshFallback => format!(
                "whole-refresh-fallback:{}:{}",
                self.commit_identity
                    .evidence_identity()
                    .reporting_projection(),
                self.reason.as_deref().unwrap_or("unspecified")
            ),
        }
    }

    pub fn is_refresh_fallback(&self) -> bool {
        self.family == WorthQueryDerivedPatchFamily::RefreshFallback
    }

    #[cfg(test)]
    pub fn retained_payload_rows(&self) -> &[WorthQueryRetainedMaterializedRow] {
        self.payload.retained_rows()
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn commit_identity(&self) -> &WorthQueryCommitIdentity {
        &self.commit_identity
    }

    pub fn aspect_touches(&self) -> &[WorthQueryAspectTouch] {
        &self.aspect_touches
    }

    pub fn authority_lane(&self) -> WorthQueryAuthorityLane {
        self.authority_lane
    }

    pub(in crate::runtime) fn bind_commit_identity(
        &mut self,
        commit_identity: WorthQueryCommitIdentity,
    ) {
        self.commit_identity = commit_identity;
    }

    pub(in crate::runtime) fn to_mutation_delta(
        &self,
        upstream_view: &str,
    ) -> WorthQueryMutationDelta {
        WorthQueryMutationDelta::from_touched_aspects(
            format!("derived:{upstream_view}"),
            self.entity_identity.clone().unwrap_or_else(|| {
                crate::memory_workspace::admit_authored_entity_label(upstream_view)
            }),
            WorthQueryMutationKind::Updated,
            self.aspect_touches.clone(),
        )
    }
}

fn computed_definition_inspection_identity(
    name: &str,
    incremental_delivery: bool,
    upstream_live_views: &[String],
    upstream_derived_views: &[String],
    dependency_aspects: &[WorthQueryAspectTouch],
    produced_aspects: &[WorthQueryAspectTouch],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_computed_definition_inspection_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("name"), name)
        .field_shape(
            WorthQueryEvidenceTag::new("authority"),
            "derived_runtime_state",
        )
        .field_bool(
            WorthQueryEvidenceTag::new("incremental"),
            incremental_delivery,
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("live"),
            upstream_live_views.iter().map(String::as_str),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("derived"),
            upstream_derived_views.iter().map(String::as_str),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("dependencies"),
            dependency_aspects
                .iter()
                .map(WorthQueryAspectTouch::admitted_touch_digest_part),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("produces"),
            produced_aspects
                .iter()
                .map(WorthQueryAspectTouch::admitted_touch_digest_part),
        )
        .seal()
}

fn computed_dependency_inspection_identity(
    name: &str,
    upstream_live_views: &[String],
    upstream_derived_views: &[String],
    dependency_aspects: &[WorthQueryAspectTouch],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_computed_dependency_inspection_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("name"), name)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("live"),
            upstream_live_views.iter().map(String::as_str),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("derived"),
            upstream_derived_views.iter().map(String::as_str),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("dependencies"),
            dependency_aspects
                .iter()
                .map(WorthQueryAspectTouch::admitted_touch_digest_part),
        )
        .seal()
}

fn computed_produced_aspect_inspection_identity(
    name: &str,
    produced_aspects: &[WorthQueryAspectTouch],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_computed_produced_aspect_inspection_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("name"), name)
        .field_shape(
            WorthQueryEvidenceTag::new("authority"),
            "derived_runtime_state",
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("produces"),
            produced_aspects
                .iter()
                .map(WorthQueryAspectTouch::admitted_touch_digest_part),
        )
        .seal()
}

fn computed_materialization_inspection_identity(
    name: &str,
    materialized_row_count: usize,
    rows: &[WorthQueryRetainedMaterializedRow],
) -> WorthQueryEvidenceIdentity {
    let row_shapes: Vec<String> = rows
        .iter()
        .map(|row| row.terminal_digest_parts().join(";"))
        .collect();
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_computed_materialization_inspection_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("name"), name)
        .field_usize(WorthQueryEvidenceTag::new("rows"), materialized_row_count)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("materialized_rows"),
            row_shapes.iter().map(String::as_str),
        )
        .seal()
}

fn computed_pending_patch_inspection_identity(
    name: &str,
    pending_patch_count: usize,
    pending_incremental_patch_count: usize,
    pending_refresh_fallback_count: usize,
    patch_identities: &[WorthQueryEvidenceIdentity],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_computed_pending_patch_inspection_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("name"), name)
        .field_usize(WorthQueryEvidenceTag::new("pending"), pending_patch_count)
        .field_usize(
            WorthQueryEvidenceTag::new("incremental"),
            pending_incremental_patch_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("refresh"),
            pending_refresh_fallback_count,
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("patches"),
            patch_identities.iter(),
        )
        .seal()
}

fn derived_patch_inspection_identity(
    view_name: &str,
    patch: &WorthQueryDerivedPatch,
) -> WorthQueryEvidenceIdentity {
    let family = match patch.family {
        WorthQueryDerivedPatchFamily::Incremental => "incremental",
        WorthQueryDerivedPatchFamily::RefreshFallback => "refresh_fallback",
    };
    let mut encoder =
        worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "worth_query_computed_patch_inspection_v1",
            )
            .field_shape(WorthQueryEvidenceTag::new("view_name"), view_name)
            .field_shape(WorthQueryEvidenceTag::new("family"), family)
            .field_shape(
                WorthQueryEvidenceTag::new("authority_lane"),
                "derived_runtime_state",
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("commit"),
                &patch.commit_identity.evidence_identity(),
            );
    if let Some(entity) = &patch.entity_identity {
        encoder = encoder.field_evidence_identity(
            WorthQueryEvidenceTag::new("entity"),
            &entity.evidence_identity(),
        );
    }
    if let Some(reason) = patch.reason.as_deref() {
        encoder = encoder.field_shape(WorthQueryEvidenceTag::new("reason"), reason);
    }
    if !patch.aspect_touches().is_empty() {
        encoder = encoder.field_value_sequence(
            WorthQueryEvidenceTag::new("admitted_aspect_touches"),
            patch
                .aspect_touches()
                .iter()
                .map(WorthQueryAspectTouch::admitted_touch_digest_part),
        );
    }
    encoder.seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDerivedViewHandle<T = crate::runtime::WorthQueryUnrefinedLiveShape> {
    name: String,
    authority_lane: WorthQueryAuthorityLane,
    marker: PhantomData<T>,
}

impl<T> WorthQueryDerivedViewHandle<T> {
    pub(in crate::runtime) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            authority_lane: WorthQueryAuthorityLane::DerivedRuntimeState,
            marker: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn authority_lane(&self) -> WorthQueryAuthorityLane {
        self.authority_lane
    }
}
