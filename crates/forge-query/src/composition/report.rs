use crate::identity::hash_parts;
use crate::query_context::QueryContextFamily;
use crate::saved_query::{runtime_backed_saved_query_support_profile, SavedQueryComplexityStatus, SavedQueryPersistenceFamily};
use crate::view_shape::{runtime_backed_view_shape_support_profile, ViewShapeComplexityStatus, ViewShapeFamily};

use super::counters::CompositionCounters;
use super::digests::{CompositionDigest, ScopeLineageDigest, TemplateBindingDigest};
use super::families::{
    QueryCompositionComplexityStatus, QueryCompositionFamily, ScopeFamily, TemplateFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum QueryCompositionDeferredScopeMarker {
    ObservedInspectorDetailTemplate,
    FocusedInspectorDetailTemplate,
    GroupedCollectionTemplate,
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
    admitted_saved_query_persistence_families: Vec<SavedQueryPersistenceFamily>,
    saved_query_statuses: Vec<(SavedQueryPersistenceFamily, SavedQueryComplexityStatus)>,
    admitted_view_families: Vec<ViewShapeFamily>,
    deferred_view_families: Vec<ViewShapeFamily>,
    view_shape_statuses: Vec<(ViewShapeFamily, ViewShapeComplexityStatus)>,
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

    pub fn composition_statuses(&self) -> &[(QueryCompositionFamily, QueryCompositionComplexityStatus)] {
        &self.composition_statuses
    }

    pub fn admitted_saved_query_persistence_families(&self) -> &[SavedQueryPersistenceFamily] {
        &self.admitted_saved_query_persistence_families
    }

    pub fn saved_query_statuses(&self) -> &[(SavedQueryPersistenceFamily, SavedQueryComplexityStatus)] {
        &self.saved_query_statuses
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
    let saved_query_statuses = runtime_backed_saved_query_support_profile();
    let admitted_saved_query_persistence_families = saved_query_statuses
        .iter()
        .map(|(family, _)| *family)
        .collect::<Vec<_>>();
    let (admitted_view_families, deferred_view_families, view_shape_statuses) =
        runtime_backed_view_shape_support_profile();
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
    ]);

    QueryCompositionSupportProfile {
        admitted_scope_families,
        admitted_template_families,
        deferred_scope_markers,
        composition_statuses,
        admitted_saved_query_persistence_families,
        saved_query_statuses,
        admitted_view_families,
        deferred_view_families,
        view_shape_statuses,
        profile_digest,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionReport {
    family: QueryCompositionFamily,
    composition_digest: CompositionDigest,
    scope_lineage_digest: Option<ScopeLineageDigest>,
    template_binding_digest: Option<TemplateBindingDigest>,
    basis_digest: Option<String>,
    basis_family: Option<QueryContextFamily>,
    counters: CompositionCounters,
}

impl CompositionReport {
    pub(crate) fn scope_expansion(
        composition_digest: CompositionDigest,
        scope_lineage_digest: ScopeLineageDigest,
        basis_digest: Option<String>,
        basis_family: Option<QueryContextFamily>,
        counters: CompositionCounters,
    ) -> Self {
        Self {
            family: QueryCompositionFamily::NamedScopeExpansion,
            composition_digest,
            scope_lineage_digest: Some(scope_lineage_digest),
            template_binding_digest: None,
            basis_digest,
            basis_family,
            counters,
        }
    }

    pub(crate) fn template_instantiation(
        composition_digest: CompositionDigest,
        template_binding_digest: TemplateBindingDigest,
        basis_digest: Option<String>,
        basis_family: Option<QueryContextFamily>,
        counters: CompositionCounters,
    ) -> Self {
        Self {
            family: QueryCompositionFamily::TemplateInstantiation,
            composition_digest,
            scope_lineage_digest: None,
            template_binding_digest: Some(template_binding_digest),
            basis_digest,
            basis_family,
            counters,
        }
    }

    pub fn family(&self) -> QueryCompositionFamily {
        self.family
    }

    pub fn composition_digest(&self) -> &CompositionDigest {
        &self.composition_digest
    }

    pub fn scope_lineage_digest(&self) -> Option<&ScopeLineageDigest> {
        self.scope_lineage_digest.as_ref()
    }

    pub fn template_binding_digest(&self) -> Option<&TemplateBindingDigest> {
        self.template_binding_digest.as_ref()
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn basis_family(&self) -> Option<&QueryContextFamily> {
        self.basis_family.as_ref()
    }

    pub fn counters(&self) -> &CompositionCounters {
        &self.counters
    }
}
