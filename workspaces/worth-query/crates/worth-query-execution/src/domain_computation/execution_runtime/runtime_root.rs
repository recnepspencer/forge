use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryAdmittedPortableDomainPackage, WorthQueryInstallationGeneration,
    WorthQueryInstallationRuntimeIdentity, WorthQueryInstalledPackageIndex,
    WorthQueryInstalledPackageIndexDenial, WorthQueryInstalledPackageIndexRelation,
};

use super::{WorthQueryExecutionRuntimeInstallation, WorthQueryRuntimeAuthorityIdentity};

/// Execution-owned root for one installed Query operating world.
///
/// The root retains the exact installed package index that later operation
/// binding and provider routing consume. It is not reconstructible from an
/// index digest or workspace label.
pub struct WorthQueryExecutionRuntime {
    authority_identity: WorthQueryRuntimeAuthorityIdentity,
    installed_packages: Arc<WorthQueryInstalledPackageIndex>,
    current_generation: Arc<AtomicU64>,
}

/// Move-only construction authority for one execution runtime.
///
/// Provider installation may borrow its identities, but only consuming the
/// installer can publish the runtime root.
pub struct WorthQueryExecutionRuntimeInstaller {
    authority_identity: WorthQueryRuntimeAuthorityIdentity,
    installation_runtime: WorthQueryInstallationRuntimeIdentity,
}

impl WorthQueryExecutionRuntimeInstaller {
    pub fn new() -> Self {
        Self {
            authority_identity: WorthQueryRuntimeAuthorityIdentity::mint(),
            installation_runtime: WorthQueryInstallationRuntimeIdentity::fresh(),
        }
    }

    pub fn authority_identity(&self) -> WorthQueryRuntimeAuthorityIdentity {
        self.authority_identity
    }

    pub fn installation_runtime(&self) -> &WorthQueryInstallationRuntimeIdentity {
        &self.installation_runtime
    }

    pub fn install(
        self,
        generation: WorthQueryInstallationGeneration,
        packages: impl IntoIterator<Item = WorthQueryAdmittedPortableDomainPackage>,
    ) -> Result<WorthQueryExecutionRuntimeInstallation, WorthQueryInstalledPackageIndexDenial> {
        let installed_packages = WorthQueryInstalledPackageIndex::build(
            self.installation_runtime,
            generation,
            packages,
        )?;
        Ok(WorthQueryExecutionRuntimeInstallation::new(
            WorthQueryExecutionRuntime {
                authority_identity: self.authority_identity,
                current_generation: Arc::new(AtomicU64::new(generation.ordinal())),
                installed_packages: Arc::new(installed_packages),
            },
        ))
    }
}

impl Default for WorthQueryExecutionRuntimeInstaller {
    fn default() -> Self {
        Self::new()
    }
}

impl WorthQueryExecutionRuntime {
    pub fn authority_identity(&self) -> WorthQueryRuntimeAuthorityIdentity {
        self.authority_identity
    }

    pub fn installed_packages(&self) -> &WorthQueryInstalledPackageIndex {
        &self.installed_packages
    }

    pub fn retain_installed_packages(&self) -> Arc<WorthQueryInstalledPackageIndex> {
        Arc::clone(&self.installed_packages)
    }

    pub(crate) fn retain_current_generation(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.current_generation)
    }

    pub fn commit_successor_installation(
        &mut self,
        successor: Arc<WorthQueryInstalledPackageIndex>,
    ) -> Result<(), WorthQueryExecutionInstallationCommitDenial> {
        match self.installed_packages.relation_to(&successor) {
            WorthQueryInstalledPackageIndexRelation::ExactSuccessor => {
                self.current_generation
                    .store(successor.generation().ordinal(), Ordering::Release);
                self.installed_packages = successor;
                Ok(())
            }
            WorthQueryInstalledPackageIndexRelation::ForeignRuntime => {
                Err(WorthQueryExecutionInstallationCommitDenial::ForeignRuntime)
            }
            WorthQueryInstalledPackageIndexRelation::EquivalentGeneration
            | WorthQueryInstalledPackageIndexRelation::SameGenerationMeaningChanged
            | WorthQueryInstalledPackageIndexRelation::NonSuccessorGeneration => {
                Err(WorthQueryExecutionInstallationCommitDenial::ExactSuccessorRequired)
            }
        }
    }

    pub fn replace_rebuilt_installation(
        &mut self,
        rebuilt: Arc<WorthQueryInstalledPackageIndex>,
    ) -> Result<(), WorthQueryExecutionInstallationCommitDenial> {
        match self.installed_packages.relation_to(&rebuilt) {
            WorthQueryInstalledPackageIndexRelation::EquivalentGeneration => {
                self.current_generation
                    .store(rebuilt.generation().ordinal(), Ordering::Release);
                self.installed_packages = rebuilt;
                Ok(())
            }
            WorthQueryInstalledPackageIndexRelation::ForeignRuntime => {
                Err(WorthQueryExecutionInstallationCommitDenial::ForeignRuntime)
            }
            WorthQueryInstalledPackageIndexRelation::SameGenerationMeaningChanged
            | WorthQueryInstalledPackageIndexRelation::ExactSuccessor
            | WorthQueryInstalledPackageIndexRelation::NonSuccessorGeneration => {
                Err(WorthQueryExecutionInstallationCommitDenial::EquivalentRebuildRequired)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryExecutionInstallationCommitDenial {
    EquivalentRebuildRequired,
    ExactSuccessorRequired,
    ForeignRuntime,
}
