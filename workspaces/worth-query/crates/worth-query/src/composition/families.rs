#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QueryCompositionFamily {
    NamedScopeExpansion,
    TemplateInstantiation,
}

impl QueryCompositionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NamedScopeExpansion => "named_scope_expansion",
            Self::TemplateInstantiation => "template_instantiation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ScopeFamily {
    PredicateScope,
    OrderingScope,
    ProjectionScope,
    TraversalBoundScope,
    BasisAwareScope,
    #[cfg(test)]
    UnsupportedScope,
}

impl ScopeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PredicateScope => "predicate_scope",
            Self::OrderingScope => "ordering_scope",
            Self::ProjectionScope => "projection_scope",
            Self::TraversalBoundScope => "traversal_bound_scope",
            Self::BasisAwareScope => "basis_aware_scope",
            #[cfg(test)]
            Self::UnsupportedScope => "unsupported_scope",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TemplateFamily {
    DetailTemplate,
    CollectionTemplate,
    ObservedInspectorDetailTemplate,
    FocusedInspectorDetailTemplate,
    GroupedCollectionTemplate,
    #[cfg(test)]
    UnsupportedTemplate,
}

impl TemplateFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailTemplate => "detail_template",
            Self::CollectionTemplate => "collection_template",
            Self::ObservedInspectorDetailTemplate => "observed_inspector_detail_template",
            Self::FocusedInspectorDetailTemplate => "focused_inspector_detail_template",
            Self::GroupedCollectionTemplate => "grouped_collection_template",
            #[cfg(test)]
            Self::UnsupportedTemplate => "unsupported_template",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QueryCompositionComplexityStatus {
    Verified,
    Debt,
}

impl QueryCompositionComplexityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Debt => "debt",
        }
    }
}
