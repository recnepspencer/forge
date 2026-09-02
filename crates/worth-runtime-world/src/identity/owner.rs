use std::sync::atomic::{AtomicU64, Ordering};

/// One process-local live Runtime World composition owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuntimeWorldOwnerIdentity(u64);

impl RuntimeWorldOwnerIdentity {
    const fn from_ordinal(ordinal: u64) -> Self {
        Self(ordinal)
    }
}

/// The identity family whose checked sequence reached its terminal value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeWorldIdentityFamily {
    Owner,
    ProductBranch,
    ProductBranchReferenceGeneration,
    BranchLifecycle,
    CompositeCommit,
    BootstrapAttempt,
    PublicationAttempt,
    ProductUnpublishedOwnerEffects,
}

/// Identity exhaustion is a typed pre-effect denial. No family wraps or
/// reuses an earlier value after this error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeWorldIdentityExhaustion {
    family: RuntimeWorldIdentityFamily,
}

impl RuntimeWorldIdentityExhaustion {
    pub(crate) const fn new(family: RuntimeWorldIdentityFamily) -> Self {
        Self { family }
    }

    pub const fn family(self) -> RuntimeWorldIdentityFamily {
        self.family
    }
}

static NEXT_OWNER_IDENTITY: AtomicU64 = AtomicU64::new(0);

/// Owner-local checked issuers used by the later Runtime World owner.
///
/// The owner sequence itself is process-unique. All other sequences are
/// intentionally local to one owner, and every emitted value carries that
/// owner identity in its type's value. Composite basis identities are bound
/// from owner-issued component admission identities rather than a cursor.
#[derive(Debug)]
pub(crate) struct RuntimeWorldIdentityIssuer {
    owner: RuntimeWorldOwnerIdentity,
    next_product_branch: u64,
    next_branch_lifecycle: u64,
    next_composite_commit: u64,
    next_bootstrap_attempt: u64,
    next_publication_attempt: u64,
    next_product_unpublished: u64,
}

impl RuntimeWorldIdentityIssuer {
    pub(crate) fn new() -> Result<(Self, RuntimeWorldOwnerIdentity), RuntimeWorldIdentityExhaustion>
    {
        let owner = issue_owner_identity()?;
        Ok((Self::for_owner(owner), owner))
    }

    pub(crate) fn for_owner(owner: RuntimeWorldOwnerIdentity) -> Self {
        Self {
            owner,
            next_product_branch: 0,
            next_branch_lifecycle: 0,
            next_composite_commit: 0,
            next_bootstrap_attempt: 0,
            next_publication_attempt: 0,
            next_product_unpublished: 0,
        }
    }

    pub(crate) const fn owner(&self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    fn next(
        cursor: &mut u64,
        family: RuntimeWorldIdentityFamily,
    ) -> Result<u64, RuntimeWorldIdentityExhaustion> {
        let current = *cursor;
        *cursor = current
            .checked_add(1)
            .ok_or(RuntimeWorldIdentityExhaustion::new(family))?;
        Ok(current)
    }

    pub(crate) fn product_branch(
        &mut self,
    ) -> Result<super::ProductBranchIdentity, RuntimeWorldIdentityExhaustion> {
        Ok(super::ProductBranchIdentity::issued(
            self.owner,
            Self::next(
                &mut self.next_product_branch,
                RuntimeWorldIdentityFamily::ProductBranch,
            )?,
        ))
    }

    pub(crate) fn branch_lifecycle(
        &mut self,
    ) -> Result<super::ProductBranchLifecycleIncarnation, RuntimeWorldIdentityExhaustion> {
        Ok(super::ProductBranchLifecycleIncarnation::issued(
            self.owner,
            Self::next(
                &mut self.next_branch_lifecycle,
                RuntimeWorldIdentityFamily::BranchLifecycle,
            )?,
        ))
    }

    pub(crate) fn composite_basis(
        &self,
        relational: worth_relational::facade::branch::RelationalBranchBasisAdmissionIdentity,
        signal: worth_signal::facade::branch::SignalBranchBasisAdmissionIdentity,
        correspondence: worth_runtime_bridge::facade::BridgeCorrespondenceAdmissionIdentity,
    ) -> super::CompositeBasisIdentity {
        super::CompositeBasisIdentity::issued(self.owner, relational, signal, correspondence)
    }

    pub(crate) fn composite_commit(
        &mut self,
    ) -> Result<super::CompositeCommitIdentity, RuntimeWorldIdentityExhaustion> {
        Ok(super::CompositeCommitIdentity::issued(
            self.owner,
            Self::next(
                &mut self.next_composite_commit,
                RuntimeWorldIdentityFamily::CompositeCommit,
            )?,
        ))
    }

    pub(crate) fn bootstrap_attempt(
        &mut self,
    ) -> Result<super::RuntimeWorldBootstrapAttemptIdentity, RuntimeWorldIdentityExhaustion> {
        Ok(super::RuntimeWorldBootstrapAttemptIdentity::issued(
            self.owner,
            Self::next(
                &mut self.next_bootstrap_attempt,
                RuntimeWorldIdentityFamily::BootstrapAttempt,
            )?,
        ))
    }

    pub(crate) fn publication_attempt(
        &mut self,
    ) -> Result<super::CompositePublicationAttemptIdentity, RuntimeWorldIdentityExhaustion> {
        Ok(super::CompositePublicationAttemptIdentity::issued(
            self.owner,
            Self::next(
                &mut self.next_publication_attempt,
                RuntimeWorldIdentityFamily::PublicationAttempt,
            )?,
        ))
    }

    pub(crate) fn product_unpublished(
        &mut self,
    ) -> Result<super::ProductUnpublishedOwnerEffectsIdentity, RuntimeWorldIdentityExhaustion> {
        Ok(super::ProductUnpublishedOwnerEffectsIdentity::issued(
            self.owner,
            Self::next(
                &mut self.next_product_unpublished,
                RuntimeWorldIdentityFamily::ProductUnpublishedOwnerEffects,
            )?,
        ))
    }
}

fn issue_owner_identity() -> Result<RuntimeWorldOwnerIdentity, RuntimeWorldIdentityExhaustion> {
    let mut current = NEXT_OWNER_IDENTITY.load(Ordering::Relaxed);
    loop {
        let next = current
            .checked_add(1)
            .ok_or(RuntimeWorldIdentityExhaustion::new(
                RuntimeWorldIdentityFamily::Owner,
            ))?;
        match NEXT_OWNER_IDENTITY.compare_exchange_weak(
            current,
            next,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return Ok(RuntimeWorldOwnerIdentity::from_ordinal(current)),
            Err(observed) => current = observed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RuntimeWorldIdentityFamily, RuntimeWorldIdentityIssuer};

    #[test]
    fn issuer_keeps_identity_families_owner_scoped_and_distinct() {
        let (mut issuer, owner) = RuntimeWorldIdentityIssuer::new().expect("owner identity");
        assert_eq!(issuer.owner(), owner);

        assert_eq!(issuer.product_branch().unwrap().owner_identity(), owner);
        assert_eq!(issuer.branch_lifecycle().unwrap().owner_identity(), owner);
        assert_eq!(issuer.composite_commit().unwrap().owner_identity(), owner);
        assert_eq!(issuer.bootstrap_attempt().unwrap().owner_identity(), owner);
        assert_eq!(
            issuer.publication_attempt().unwrap().owner_identity(),
            owner
        );
        assert_eq!(
            issuer.product_unpublished().unwrap().owner_identity(),
            owner
        );

        let first = issuer.composite_commit().unwrap();
        let second = issuer.composite_commit().unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn identity_exhaustion_is_pre_effect_and_family_specific() {
        let (mut issuer, _) = RuntimeWorldIdentityIssuer::new().expect("owner identity");
        issuer.next_publication_attempt = u64::MAX;

        let denial = issuer
            .publication_attempt()
            .expect_err("the checked sequence must not wrap");
        assert_eq!(
            denial.family(),
            RuntimeWorldIdentityFamily::PublicationAttempt
        );
        assert!(issuer.publication_attempt().is_err());
    }
}
