use crate::identity::hash_parts;
use crate::saved_query::{
    runtime_backed_saved_query_future_support_profile, runtime_backed_saved_query_support_profile,
    SavedQueryComplexityStatus, SavedQueryPersistenceFamily, SavedQueryTemporalAsyncSurfacePosture,
};
use crate::view_shape::{
    runtime_backed_view_shape_future_support_profile, runtime_backed_view_shape_support_profile,
    ViewShapeComplexityStatus, ViewShapeFamily, ViewShapeTemporalAsyncSupportPosture,
};

use super::super::families::{
    QueryCompositionComplexityStatus, QueryCompositionFamily, ScopeFamily, TemplateFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum QueryCompositionDeferredScopeMarker {
    ObservedInspectorDetailTemplate,
    FocusedInspectorDetailTemplate,
    GroupedCollectionTemplate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QueryCompositionTemporalAsyncPosture {
    OrdinaryOnly,
    FuturePreserving,
    VisibleButDeferred,
}

impl QueryCompositionTemporalAsyncPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryOnly => "ordinary_only",
            Self::FuturePreserving => "future_preserving",
            Self::VisibleButDeferred => "visible_but_deferred",
        }
    }
}

impl QueryCompositionDeferredScopeMarker {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ObservedInspectorDetailTemplate => "observed_inspector_detail_template",
            Self::FocusedInspectorDetailTemplate => "focused_inspector_detail_template",
            Self::GroupedCollectionTemplate => "grouped_collection_template",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryCompositionSupportProfile {
    admitted_scope_families: Vec<ScopeFamily>,
    admitted_template_families: Vec<TemplateFamily>,
    deferred_scope_markers: Vec<QueryCompositionDeferredScopeMarker>,
    composition_statuses: Vec<(QueryCompositionFamily, QueryCompositionComplexityStatus)>,
    scope_temporal_async_postures: Vec<(ScopeFamily, QueryCompositionTemporalAsyncPosture)>,
    template_temporal_async_postures: Vec<(TemplateFamily, QueryCompositionTemporalAsyncPosture)>,
    admitted_saved_query_persistence_families: Vec<SavedQueryPersistenceFamily>,
    saved_query_statuses: Vec<(SavedQueryPersistenceFamily, SavedQueryComplexityStatus)>,
    saved_query_temporal_async_postures: Vec<(
        SavedQueryPersistenceFamily,
        SavedQueryTemporalAsyncSurfacePosture,
    )>,
    admitted_view_families: Vec<ViewShapeFamily>,
    deferred_view_families: Vec<ViewShapeFamily>,
    view_shape_statuses: Vec<(ViewShapeFamily, ViewShapeComplexityStatus)>,
    view_shape_temporal_async_postures:
        Vec<(ViewShapeFamily, ViewShapeTemporalAsyncSupportPosture)>,
    profile_digest: String,
}

impl QueryCompositionSupportProfile {
    pub fn admitted_scope_families(&self) -> &[ScopeFamily] {
        &self.admitted_scope_families
    }

    pub fn admitted_template_families(&self) -> &[TemplateFamily] {
        &self.admitted_template_families
    }

    pub fn deferred_scope_markers(&self) -> &[QueryCompositionDeferredScopeMarker] {
        &self.deferred_scope_markers
    }

    pub fn composition_statuses(
        &self,
    ) -> &[(QueryCompositionFamily, QueryCompositionComplexityStatus)] {
        &self.composition_statuses
    }

    pub fn scope_temporal_async_postures(
        &self,
    ) -> &[(ScopeFamily, QueryCompositionTemporalAsyncPosture)] {
        &self.scope_temporal_async_postures
    }

    pub fn template_temporal_async_postures(
        &self,
    ) -> &[(TemplateFamily, QueryCompositionTemporalAsyncPosture)] {
        &self.template_temporal_async_postures
    }

    pub fn admitted_saved_query_persistence_families(&self) -> &[SavedQueryPersistenceFamily] {
        &self.admitted_saved_query_persistence_families
    }

    pub fn saved_query_statuses(
        &self,
    ) -> &[(SavedQueryPersistenceFamily, SavedQueryComplexityStatus)] {
        &self.saved_query_statuses
    }

    pub fn saved_query_temporal_async_postures(
        &self,
    ) -> &[(
        SavedQueryPersistenceFamily,
        SavedQueryTemporalAsyncSurfacePosture,
    )] {
        &self.saved_query_temporal_async_postures
    }

    pub fn admitted_view_families(&self) -> &[ViewShapeFamily] {
        &self.admitted_view_families
    }

    pub fn deferred_view_families(&self) -> &[ViewShapeFamily] {
        &self.deferred_view_families
    }

    pub fn view_shape_statuses(&self) -> &[(ViewShapeFamily, ViewShapeComplexityStatus)] {
        &self.view_shape_statuses
    }

    pub fn view_shape_temporal_async_postures(
        &self,
    ) -> &[(ViewShapeFamily, ViewShapeTemporalAsyncSupportPosture)] {
        &self.view_shape_temporal_async_postures
    }

    pub fn profile_digest(&self) -> &str {
        &self.profile_digest
    }
}

pub fn runtime_backed_query_composition_support_profile() -> QueryCompositionSupportProfile {
    let admitted_scope_families = vec![
        ScopeFamily::PredicateScope,
        ScopeFamily::OrderingScope,
        ScopeFamily::ProjectionScope,
        ScopeFamily::TraversalBoundScope,
        ScopeFamily::BasisAwareScope,
    ];
    let admitted_template_families = vec![
        TemplateFamily::DetailTemplate,
        TemplateFamily::CollectionTemplate,
    ];
    let deferred_scope_markers = vec![
        QueryCompositionDeferredScopeMarker::ObservedInspectorDetailTemplate,
        QueryCompositionDeferredScopeMarker::FocusedInspectorDetailTemplate,
        QueryCompositionDeferredScopeMarker::GroupedCollectionTemplate,
    ];
    let composition_statuses = vec![
        (
            QueryCompositionFamily::NamedScopeExpansion,
            QueryCompositionComplexityStatus::Debt,
        ),
        (
            QueryCompositionFamily::TemplateInstantiation,
            QueryCompositionComplexityStatus::Debt,
        ),
    ];
    let scope_temporal_async_postures = vec![
        (
            ScopeFamily::PredicateScope,
            QueryCompositionTemporalAsyncPosture::OrdinaryOnly,
        ),
        (
            ScopeFamily::OrderingScope,
            QueryCompositionTemporalAsyncPosture::OrdinaryOnly,
        ),
        (
            ScopeFamily::ProjectionScope,
            QueryCompositionTemporalAsyncPosture::OrdinaryOnly,
        ),
        (
            ScopeFamily::TraversalBoundScope,
            QueryCompositionTemporalAsyncPosture::OrdinaryOnly,
        ),
        (
            ScopeFamily::BasisAwareScope,
            QueryCompositionTemporalAsyncPosture::FuturePreserving,
        ),
    ];
    let template_temporal_async_postures = vec![
        (
            TemplateFamily::DetailTemplate,
            QueryCompositionTemporalAsyncPosture::OrdinaryOnly,
        ),
        (
            TemplateFamily::CollectionTemplate,
            QueryCompositionTemporalAsyncPosture::OrdinaryOnly,
        ),
        (
            TemplateFamily::ObservedInspectorDetailTemplate,
            QueryCompositionTemporalAsyncPosture::VisibleButDeferred,
        ),
        (
            TemplateFamily::FocusedInspectorDetailTemplate,
            QueryCompositionTemporalAsyncPosture::VisibleButDeferred,
        ),
        (
            TemplateFamily::GroupedCollectionTemplate,
            QueryCompositionTemporalAsyncPosture::VisibleButDeferred,
        ),
    ];
    let saved_query_statuses = runtime_backed_saved_query_support_profile();
    let saved_query_temporal_async_postures = runtime_backed_saved_query_future_support_profile();
    let admitted_saved_query_persistence_families = saved_query_statuses
        .iter()
        .map(|(family, _)| *family)
        .collect::<Vec<_>>();
    let (admitted_view_families, deferred_view_families, view_shape_statuses) =
        runtime_backed_view_shape_support_profile();
    let view_shape_temporal_async_postures = runtime_backed_view_shape_future_support_profile();
    let profile_digest = hash_parts(&[
        format!(
            "scopes:{}",
            admitted_scope_families
                .iter()
                .map(ScopeFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "templates:{}",
            admitted_template_families
                .iter()
                .map(TemplateFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "deferred:{}",
            deferred_scope_markers
                .iter()
                .map(QueryCompositionDeferredScopeMarker::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "statuses:{}",
            composition_statuses
                .iter()
                .map(|(family, status)| format!("{}:{}", family.as_str(), status.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "scope_future_postures:{}",
            scope_temporal_async_postures
                .iter()
                .map(|(family, posture)| format!("{}:{}", family.as_str(), posture.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "template_future_postures:{}",
            template_temporal_async_postures
                .iter()
                .map(|(family, posture)| format!("{}:{}", family.as_str(), posture.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "saved_query_families:{}",
            admitted_saved_query_persistence_families
                .iter()
                .map(SavedQueryPersistenceFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "saved_query_statuses:{}",
            saved_query_statuses
                .iter()
                .map(|(family, status)| format!("{}:{}", family.as_str(), status.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "saved_query_future_postures:{}",
            saved_query_temporal_async_postures
                .iter()
                .map(|(family, posture)| format!("{}:{}", family.as_str(), posture.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "admitted_views:{}",
            admitted_view_families
                .iter()
                .map(ViewShapeFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "deferred_views:{}",
            deferred_view_families
                .iter()
                .map(ViewShapeFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "view_statuses:{}",
            view_shape_statuses
                .iter()
                .map(|(family, status)| format!("{}:{}", family.as_str(), status.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "view_future_postures:{}",
            view_shape_temporal_async_postures
                .iter()
                .map(|(family, posture)| format!("{}:{}", family.as_str(), posture.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    ]);

    QueryCompositionSupportProfile {
        admitted_scope_families,
        admitted_template_families,
        deferred_scope_markers,
        composition_statuses,
        scope_temporal_async_postures,
        template_temporal_async_postures,
        admitted_saved_query_persistence_families,
        saved_query_statuses,
        saved_query_temporal_async_postures,
        admitted_view_families,
        deferred_view_families,
        view_shape_statuses,
        view_shape_temporal_async_postures,
        profile_digest,
    }
}
