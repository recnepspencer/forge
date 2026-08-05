use super::WorthQueryInstalledGraphReadContract;

/// Installed binding from one application query to canonical read-family
/// meaning.
///
/// The binding owns the complete graph contract. Its identity is descriptive;
/// executable authority still requires the installed query, runtime-derived
/// support, an admitted plan, and a runtime-owned basis.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledApplicationReadFamilyBinding {
    planning_contract: WorthQueryInstalledGraphReadContract,
}

impl WorthQueryInstalledApplicationReadFamilyBinding {
    pub(super) fn bind(planning_contract: WorthQueryInstalledGraphReadContract) -> Self {
        Self { planning_contract }
    }

    pub fn identity(&self) -> &worth_foundational::facade::CanonicalDigestId {
        self.planning_contract.canonical_planning_basis().digest()
    }

    pub fn canonical_planning_identity(&self) -> &worth_foundational::facade::CanonicalDigestId {
        self.identity()
    }

    pub const fn planning_contract(&self) -> &WorthQueryInstalledGraphReadContract {
        &self.planning_contract
    }
}
