use forge_foundational::facade::MaterializedFoundationalProfileSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRecoveryMaterialization {
    profile: Option<MaterializedFoundationalProfileSet>,
}

impl ForgeQueryRecoveryMaterialization {
    pub const fn lean() -> Self {
        Self { profile: None }
    }

    pub(crate) fn from_profile(profile: Option<MaterializedFoundationalProfileSet>) -> Self {
        Self { profile }
    }

    pub const fn profile(&self) -> Option<&MaterializedFoundationalProfileSet> {
        self.profile.as_ref()
    }
}
