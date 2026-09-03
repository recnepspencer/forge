use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::{
    ProductBranchHeadProtection, ProductBranchObservation, ProductBranchReferenceCell,
    ProductBranchReferenceSnapshot, RuntimeWorldBranchAdmissionDenial,
    RuntimeWorldBranchRetirementDenial,
};
use crate::identity::{
    ProductBranchIdentity, ProductBranchLifecycleIncarnation, ProductBranchReferenceGeneration,
};
use crate::publication::ProductBranchIntent;

use super::RuntimeWorldOwnerRoot;

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn branch_service_is_available(&self) -> bool {
        let bootstrap = self
            .state
            .bootstrap
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if *bootstrap != super::RuntimeWorldBootstrapState::Performed {
            return false;
        }
        self.state
            .close
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .state()
            == super::super::close::RuntimeWorldCloseState::Open
    }

    fn issue_branch_identities(
        &self,
    ) -> Result<(ProductBranchIdentity, ProductBranchLifecycleIncarnation), ()> {
        let mut identities = self
            .state
            .identities
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let branch = identities.product_branch().map_err(|_| ())?;
        let lifecycle = identities.branch_lifecycle().map_err(|_| ())?;
        Ok((branch, lifecycle))
    }

    fn create_reused_branch(
        &self,
        basis: AdmittedCompositeRuntimeWorldBasis,
        intent: ProductBranchIntent,
    ) -> Result<ProductBranchObservation, RuntimeWorldBranchAdmissionDenial> {
        let commit_identity = self
            .state
            .branches
            .commit_for_basis(&basis)
            .ok_or(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable)?;
        let commit = self
            .state
            .history
            .lookup(&commit_identity)
            .ok_or(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable)?;
        if crate::basis::compare_exact(commit.basis(), &basis).is_err() {
            return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
        }

        let reservation = self
            .state
            .branches
            .reserve_branch(self.owner_identity(), intent.creation().name().clone())
            .map_err(map_registry_denial)?;
        let (branch, lifecycle) = self
            .issue_branch_identities()
            .map_err(|_| RuntimeWorldBranchAdmissionDenial::IdentityExhausted)?;

        // The exact basis is already admitted and retained by the source
        // observation/product head. This direct product-head issuance joins
        // existing unique pins and performs no component-owner work for the
        // normal reuse path.
        let product_head = self
            .state
            .retention
            .issue_product_head(&basis)
            .map_err(map_retention_denial)?;
        let product_history = self
            .state
            .history
            .protect_product_head(commit.as_ref())
            .map_err(|_| RuntimeWorldBranchAdmissionDenial::CapacityExhausted)?;
        let snapshot = ProductBranchReferenceSnapshot::owner_issued(
            self.owner_identity(),
            branch.clone(),
            lifecycle,
            ProductBranchReferenceGeneration::initial(),
            Arc::clone(&commit),
        )
        .map_err(|_| RuntimeWorldBranchAdmissionDenial::OwnerUnavailable)?;
        let protection = ProductBranchHeadProtection::bootstrap_issued(
            snapshot.clone(),
            product_head,
            product_history,
        )
        .map_err(|_| RuntimeWorldBranchAdmissionDenial::OwnerUnavailable)?;
        let cell = ProductBranchReferenceCell::new(protection)
            .map_err(|_| RuntimeWorldBranchAdmissionDenial::OwnerUnavailable)?;
        let observation_components = self
            .state
            .retention
            .issue_observation(commit.as_ref())
            .map_err(map_retention_denial)?;
        let observation_history = self
            .state
            .history
            .protect_explicit_commit(commit.as_ref())
            .map_err(|_| RuntimeWorldBranchAdmissionDenial::CapacityExhausted)?;
        let observation = ProductBranchObservation::owner_issued(
            snapshot,
            observation_components,
            observation_history,
        )
        .map_err(|_| RuntimeWorldBranchAdmissionDenial::OwnerUnavailable)?;

        reservation
            .install(branch, lifecycle, cell)
            .map_err(|(_, denial)| map_registry_denial(denial))?;
        Ok(observation)
    }
}

impl<D, I, E, Ctx, T> super::super::ports::RuntimeWorldObservationService
    for RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn observe_product_branch(
        &self,
        branch: &ProductBranchIdentity,
    ) -> Result<ProductBranchObservation, RuntimeWorldBranchAdmissionDenial> {
        if branch.owner_identity() != self.owner_identity() {
            return Err(RuntimeWorldBranchAdmissionDenial::ForeignOwner);
        }
        if !self.branch_service_is_available() {
            return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
        }
        let cell = self
            .state
            .branches
            .branch_cell(branch)
            .ok_or(RuntimeWorldBranchAdmissionDenial::RetiredBranch)?;
        cell.observe(&self.state.history, &self.state.retention)
            .map_err(|_| RuntimeWorldBranchAdmissionDenial::CapacityExhausted)
    }
}

impl<D, I, E, Ctx, T> super::super::ports::RuntimeWorldBranchService
    for RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn create_product_branch(
        &self,
        basis: AdmittedCompositeRuntimeWorldBasis,
        intent: ProductBranchIntent,
    ) -> Result<ProductBranchObservation, RuntimeWorldBranchAdmissionDenial> {
        if basis.owner_identity() != self.owner_identity() {
            return Err(RuntimeWorldBranchAdmissionDenial::ForeignOwner);
        }
        if !self.branch_service_is_available() {
            return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
        }
        let postures = intent.component_postures();
        if !postures.is_exact_reuse() {
            // The frozen seam supplies no source commit, execution borrow,
            // cancellation, or fork-success evidence. Calling either owner
            // here would make a performed component effect unrepresentable.
            return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
        }
        self.create_reused_branch(basis, intent)
    }

    fn retire_product_branch(
        &self,
        branch: ProductBranchIdentity,
    ) -> Result<(), RuntimeWorldBranchRetirementDenial> {
        if branch.owner_identity() != self.owner_identity() {
            return Err(RuntimeWorldBranchRetirementDenial::OwnerUnavailable);
        }
        if !self.branch_service_is_available() {
            return Err(RuntimeWorldBranchRetirementDenial::OwnerUnavailable);
        }
        let cell = self
            .state
            .branches
            .retire(self.owner_identity(), &branch)
            .map_err(map_retirement_denial)?;
        // Drop the product-head and history custody after the registry lock
        // has been released. No component lifecycle/delete port is called.
        drop(cell);
        Ok(())
    }
}

fn map_registry_denial(
    denial: crate::branch::registry::ProductBranchRegistryDenial,
) -> RuntimeWorldBranchAdmissionDenial {
    use crate::branch::registry::ProductBranchRegistryDenial;

    match denial {
        ProductBranchRegistryDenial::ForeignOwner => {
            RuntimeWorldBranchAdmissionDenial::ForeignOwner
        }
        ProductBranchRegistryDenial::CapacityExhausted => {
            RuntimeWorldBranchAdmissionDenial::CapacityExhausted
        }
        ProductBranchRegistryDenial::AlreadyInstalled
        | ProductBranchRegistryDenial::AlreadyRetired
        | ProductBranchRegistryDenial::ReservationMissing
        | ProductBranchRegistryDenial::IdentityMismatch
        | ProductBranchRegistryDenial::BranchAlreadyInstalled
        | ProductBranchRegistryDenial::LifecycleAlreadyInstalled
        | ProductBranchRegistryDenial::UnknownBranch => {
            RuntimeWorldBranchAdmissionDenial::OwnerUnavailable
        }
        ProductBranchRegistryDenial::NameAlreadyReserved
        | ProductBranchRegistryDenial::NameAlreadyInstalled => {
            RuntimeWorldBranchAdmissionDenial::DuplicateName
        }
    }
}

fn map_retention_denial(
    denial: crate::retention::RetentionObligationDenial,
) -> RuntimeWorldBranchAdmissionDenial {
    use crate::retention::RetentionObligationDenial;

    match denial {
        RetentionObligationDenial::LeaseIdentityExhausted => {
            RuntimeWorldBranchAdmissionDenial::IdentityExhausted
        }
        RetentionObligationDenial::ForeignOwner { .. }
        | RetentionObligationDenial::Relational(_)
        | RetentionObligationDenial::Signal(_)
        | RetentionObligationDenial::OwnerOperationPanicked => {
            RuntimeWorldBranchAdmissionDenial::OwnerUnavailable
        }
        RetentionObligationDenial::InvalidComponentPair
        | RetentionObligationDenial::UniquePinCapacityExhausted { .. }
        | RetentionObligationDenial::InFlightAcquisitionCapacityExhausted { .. }
        | RetentionObligationDenial::DependencyCountExhausted => {
            RuntimeWorldBranchAdmissionDenial::CapacityExhausted
        }
    }
}

fn map_retirement_denial(
    denial: crate::branch::registry::ProductBranchRegistryDenial,
) -> RuntimeWorldBranchRetirementDenial {
    use crate::branch::registry::ProductBranchRegistryDenial;

    match denial {
        ProductBranchRegistryDenial::UnknownBranch => {
            RuntimeWorldBranchRetirementDenial::UnknownBranch
        }
        ProductBranchRegistryDenial::AlreadyRetired => {
            RuntimeWorldBranchRetirementDenial::AlreadyRetired
        }
        ProductBranchRegistryDenial::CapacityExhausted => {
            RuntimeWorldBranchRetirementDenial::CapacityExhausted
        }
        ProductBranchRegistryDenial::ForeignOwner
        | ProductBranchRegistryDenial::AlreadyInstalled
        | ProductBranchRegistryDenial::ReservationMissing
        | ProductBranchRegistryDenial::IdentityMismatch
        | ProductBranchRegistryDenial::NameAlreadyReserved
        | ProductBranchRegistryDenial::NameAlreadyInstalled
        | ProductBranchRegistryDenial::BranchAlreadyInstalled
        | ProductBranchRegistryDenial::LifecycleAlreadyInstalled => {
            RuntimeWorldBranchRetirementDenial::OwnerUnavailable
        }
    }
}

#[cfg(test)]
#[path = "../../branch/retirement_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "branch_service_contract_tests.rs"]
mod contract_tests;
