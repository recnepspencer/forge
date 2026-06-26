#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewBindingFamily {
    Collection,
    Detail,
    Grouped,
    Relationship,
    OrderedEvent,
    Spatial,
    CustomAdmitted(String),
}

impl ViewBindingFamily {
    pub fn collection() -> Self {
        Self::Collection
    }

    pub fn detail() -> Self {
        Self::Detail
    }

    pub fn grouped() -> Self {
        Self::Grouped
    }

    pub fn relationship() -> Self {
        Self::Relationship
    }

    pub fn ordered_event() -> Self {
        Self::OrderedEvent
    }

    pub fn spatial() -> Self {
        Self::Spatial
    }

    pub fn custom_admitted(name: impl Into<String>) -> Self {
        Self::CustomAdmitted(name.into())
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Collection => "collection".to_string(),
            Self::Detail => "detail".to_string(),
            Self::Grouped => "grouped".to_string(),
            Self::Relationship => "relationship".to_string(),
            Self::OrderedEvent => "ordered_event".to_string(),
            Self::Spatial => "spatial".to_string(),
            Self::CustomAdmitted(name) => format!("custom_admitted:{name}"),
        }
    }
}
