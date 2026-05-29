use crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ForgeQueryOrchestrationContributionCompatibilityKind {
    None,
    DeclarationScoped,
    GroupedNeighborhood,
}

impl ForgeQueryOrchestrationContributionCompatibilityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DeclarationScoped => "declaration_scoped",
            Self::GroupedNeighborhood => "grouped_neighborhood",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryOrchestrationContributionCompatibility {
    kind: ForgeQueryOrchestrationContributionCompatibilityKind,
    supported_families: Vec<ForgeQueryDeclarationEntryContributionCategoryFamily>,
}

impl ForgeQueryOrchestrationContributionCompatibility {
    pub fn none() -> Self {
        Self {
            kind: ForgeQueryOrchestrationContributionCompatibilityKind::None,
            supported_families: Vec::new(),
        }
    }

    pub fn declaration_scoped(
        supported_families: Vec<ForgeQueryDeclarationEntryContributionCategoryFamily>,
    ) -> Self {
        let mut supported_families = supported_families;
        supported_families.sort_by_key(|family| family.as_str());
        supported_families.dedup();
        Self {
            kind: ForgeQueryOrchestrationContributionCompatibilityKind::DeclarationScoped,
            supported_families,
        }
    }

    pub fn grouped_neighborhood() -> Self {
        Self {
            kind: ForgeQueryOrchestrationContributionCompatibilityKind::GroupedNeighborhood,
            supported_families: Vec::new(),
        }
    }

    pub fn kind(&self) -> ForgeQueryOrchestrationContributionCompatibilityKind {
        self.kind
    }

    pub fn supported_families(&self) -> &[ForgeQueryDeclarationEntryContributionCategoryFamily] {
        &self.supported_families
    }

    pub fn supports(&self, family: ForgeQueryDeclarationEntryContributionCategoryFamily) -> bool {
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
