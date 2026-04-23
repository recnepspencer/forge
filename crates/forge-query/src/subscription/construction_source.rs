#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionConstructionSource {
    Direct,
    ScopeExpanded,
    TemplateInstantiated,
    SavedExactReuse,
    FacadeLive,
}

impl QuerySubscriptionConstructionSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::ScopeExpanded => "scope_expanded",
            Self::TemplateInstantiated => "template_instantiated",
            Self::SavedExactReuse => "saved_exact_reuse",
            Self::FacadeLive => "facade_live",
        }
    }
}
