use std::sync::Arc;

use crate::branch::{
    ProductBranchHeadProtection, ProductBranchName, ProductBranchObservation,
    ProductBranchReferenceCell, ProductBranchReferenceSnapshot, ProductBranchRetirementReport,
    RuntimeWorldBranchAdmissionDenial, RuntimeWorldBranchRetirementDenial,
};
use crate::identity::{
    ProductBranchIdentity, ProductBranchIncarnation, ProductBranchReferenceGeneration,
};

use super::super::ports::{RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchCreationRequest};
use super::RuntimeWorldOwnerRoot;

mod creation;

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

    /// The branch identity is the owner plus the normalized name, so retiring
    /// and recreating one name yields the same identity with a new
    /// incarnation. Only the incarnation is drawn from the issuer.
    pub(super) fn issue_branch_identities(
        &self,
        name: ProductBranchName,
    ) -> Result<(ProductBranchIdentity, ProductBranchIncarnation), ()> {
        let mut identities = self
            .state
            .identities
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let incarnation = identities.branch_incarnation().map_err(|_| ())?;
        Ok((
            ProductBranchIdentity::issued(self.owner_identity(), name),
            incarnation,
        ))
    }

    fn create_reused_branch(
        &self,
        source: ProductBranchObservation,
        name: ProductBranchName,
    ) -> Result<ProductBranchObservation, RuntimeWorldBranchAdmissionDenial> {
        let (basis, commit) = self.reused_commit(&source)?;

        let reservation = self
            .state
            .branches
            .reserve_branch(self.owner_identity(), name.clone())
            .map_err(map_registry_denial)?;
        let (branch, lifecycle) = self
            .issue_branch_identities(name)
            .map_err(|_| RuntimeWorldBranchAdmissionDenial::IdentityExhausted)?;
        let (cell, snapshot) =
            self.issue_reused_head(branch.clone(), lifecycle, &basis, Arc::clone(&commit))?;
        let observation = self.issue_reused_observation(snapshot, commit)?;

        reservation
            .install(branch, lifecycle, cell)
            .map_err(|(_, denial)| map_registry_denial(denial))?;
        Ok(observation)
    }

    fn reused_commit(
        &self,
        source: &ProductBranchObservation,
    ) -> Result<
        (
            crate::basis::AdmittedCompositeRuntimeWorldBasis,
            Arc<crate::history::CompositeRuntimeWorldCommit>,
        ),
        RuntimeWorldBranchAdmissionDenial,
    > {
        let basis = source.basis().clone();
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
        Ok((basis, commit))
    }

    fn issue_reused_head(
        &self,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchIncarnation,
        basis: &crate::basis::AdmittedCompositeRuntimeWorldBasis,
        commit: Arc<crate::history::CompositeRuntimeWorldCommit>,
    ) -> Result<
        (ProductBranchReferenceCell, ProductBranchReferenceSnapshot),
        RuntimeWorldBranchAdmissionDenial,
    > {
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
        Ok((cell, snapshot))
    }

    fn issue_reused_observation(
        &self,
        snapshot: ProductBranchReferenceSnapshot,
        commit: Arc<crate::history::CompositeRuntimeWorldCommit>,
    ) -> Result<ProductBranchObservation, RuntimeWorldBranchAdmissionDenial> {
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
        request: RuntimeWorldBranchCreationRequest<'_>,
    ) -> Result<RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchAdmissionDenial> {
        let (source, intent, cancellation) = request.into_parts();
        if source.owner_identity() != self.owner_identity()
            || source.branch_identity().owner_identity() != self.owner_identity()
        {
            return Err(RuntimeWorldBranchAdmissionDenial::ForeignOwner);
        }
        if !self.branch_service_is_available() {
            return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
        }
        let plans = intent
            .plans()
            .ok_or(RuntimeWorldBranchAdmissionDenial::PlansOmitted)?;
        if plans.is_exact_reuse() {
            return self
                .create_reused_branch(source, intent.name().clone())
                .map(RuntimeWorldBranchCreationOutcome::Performed);
        }
        creation::create_forked_branch(self, source, intent, cancellation)
    }

    /// Retirement releases the product reference this world owns and reports
    /// the component branches it created but must not delete itself.
    fn retire_product_branch(
        &self,
        branch: ProductBranchIdentity,
    ) -> Result<ProductBranchRetirementReport, RuntimeWorldBranchRetirementDenial> {
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
        // LANE A owns turning this branch's custody records into typed owner
        // work. The registry is installed on the owner state and
        // `take_for_branch` already returns the exact records this branch
        // created; what is still undecided is the retirement contract itself,
        // so the records stay charged until Lane A claims them. Reporting an
        // empty work list is the honest statement that this world names no
        // component branch for deletion yet, not a fabricated one.
        Ok(ProductBranchRetirementReport::new(branch, Vec::new()))
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
