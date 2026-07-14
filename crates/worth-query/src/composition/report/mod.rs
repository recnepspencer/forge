mod support_profile;

use crate::query_context::QueryContextFamily;

use super::counters::CompositionCounters;
use super::digests::{CompositionDigest, ScopeLineageDigest, TemplateBindingDigest};
use super::families::QueryCompositionFamily;

pub use support_profile::{
    runtime_backed_query_composition_support_profile, QueryCompositionDeferredScopeMarker,
    QueryCompositionSupportProfile, QueryCompositionTemporalAsyncPosture,
};

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
