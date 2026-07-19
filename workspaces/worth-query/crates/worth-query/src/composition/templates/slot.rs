use crate::composition::TemplateFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TemplateParameterSlotKind {
    Predicate,
    Ordering,
    Projection,
    Traversal,
}

impl TemplateParameterSlotKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Predicate => "predicate",
            Self::Ordering => "ordering",
            Self::Projection => "projection",
            Self::Traversal => "traversal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TemplateParameterSlot {
    name: String,
    kind: TemplateParameterSlotKind,
}

impl TemplateParameterSlot {
    pub fn predicate(name: impl Into<String>) -> Self {
        Self::new(name, TemplateParameterSlotKind::Predicate)
    }

    pub fn ordering(name: impl Into<String>) -> Self {
        Self::new(name, TemplateParameterSlotKind::Ordering)
    }

    pub fn projection(name: impl Into<String>) -> Self {
        Self::new(name, TemplateParameterSlotKind::Projection)
    }

    pub fn traversal(name: impl Into<String>) -> Self {
        Self::new(name, TemplateParameterSlotKind::Traversal)
    }

    fn new(name: impl Into<String>, kind: TemplateParameterSlotKind) -> Self {
        Self {
            name: name.into(),
            kind,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> TemplateParameterSlotKind {
        self.kind
    }

    pub(crate) fn digest_part(&self, family: TemplateFamily) -> String {
        format!(
            "slot:{}:{}:{}",
            family.as_str(),
            self.kind.as_str(),
            self.name
        )
    }
}
