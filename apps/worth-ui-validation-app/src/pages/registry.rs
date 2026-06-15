use crate::shell::ValidationPageId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationPageDescriptor {
    id: ValidationPageId,
    label: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationPageRegistry {
    pages: &'static [ValidationPageDescriptor],
}

impl ValidationPageDescriptor {
    pub const fn new(id: ValidationPageId, label: &'static str) -> Self {
        Self { id, label }
    }

    pub fn id(self) -> ValidationPageId {
        self.id
    }

    pub fn label(self) -> &'static str {
        self.label
    }
}

impl ValidationPageRegistry {
    pub const DEFAULT: Self = Self {
        pages: &[
            ValidationPageDescriptor::new(ValidationPageId::SurfaceAtlas, "Surface atlas"),
            ValidationPageDescriptor::new(ValidationPageId::ScenarioRuns, "Scenario runs"),
            ValidationPageDescriptor::new(ValidationPageId::Evidence, "Evidence"),
        ],
    };

    pub fn pages(self) -> &'static [ValidationPageDescriptor] {
        self.pages
    }
}
