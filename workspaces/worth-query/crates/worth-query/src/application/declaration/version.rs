use worth_foundational::facade::CanonicalizationRuleVersion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationCanonicalizationVersion {
    foundational: CanonicalizationRuleVersion,
}

impl WorthQueryDeclarationCanonicalizationVersion {
    pub fn pinned_v1() -> Self {
        Self {
            foundational: CanonicalizationRuleVersion::new("worth.query.declaration.v1")
                .expect("pinned declaration version should be valid"),
        }
    }

    pub fn explicit(foundational: CanonicalizationRuleVersion) -> Self {
        Self { foundational }
    }

    pub fn foundational(&self) -> &CanonicalizationRuleVersion {
        &self.foundational
    }
}

impl Default for WorthQueryDeclarationCanonicalizationVersion {
    fn default() -> Self {
        Self::pinned_v1()
    }
}
