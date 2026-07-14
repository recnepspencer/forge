use crate::application::WorthQueryDeclarationEntryContributionCategoryFamily;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationContributionCompatibilityKind {
    None,
    DeclarationScoped,
    GroupedNeighborhood,
}

impl WorthQueryOrchestrationContributionCompatibilityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DeclarationScoped => "declaration_scoped",
            Self::GroupedNeighborhood => "grouped_neighborhood",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOrchestrationContributionCompatibility {
    kind: WorthQueryOrchestrationContributionCompatibilityKind,
    supported_families: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
}

impl WorthQueryOrchestrationContributionCompatibility {
    pub fn none() -> Self {
        Self {
            kind: WorthQueryOrchestrationContributionCompatibilityKind::None,
            supported_families: Vec::new(),
        }
    }

    pub fn declaration_scoped(
        supported_families: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
    ) -> Self {
        let mut supported_families = supported_families;
        supported_families.sort_by_key(|family| family.as_str());
        supported_families.dedup();
        Self {
            kind: WorthQueryOrchestrationContributionCompatibilityKind::DeclarationScoped,
            supported_families,
        }
    }

    pub fn grouped_neighborhood() -> Self {
        Self {
            kind: WorthQueryOrchestrationContributionCompatibilityKind::GroupedNeighborhood,
            supported_families: Vec::new(),
        }
    }

    pub fn kind(&self) -> WorthQueryOrchestrationContributionCompatibilityKind {
        self.kind
    }

    pub fn supported_families(&self) -> &[WorthQueryDeclarationEntryContributionCategoryFamily] {
        &self.supported_families
    }

    pub fn supports(&self, family: WorthQueryDeclarationEntryContributionCategoryFamily) -> bool {
        self.supported_families.contains(&family)
    }

    pub fn as_digest_fragment(&self) -> String {
        let supported = self
            .supported_families
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>()
            .join("|");
        format!("{}:{supported}", self.kind.as_str())
    }
}
