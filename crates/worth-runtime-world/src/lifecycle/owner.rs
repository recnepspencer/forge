use crate::identity::{
    RuntimeWorldIdentityExhaustion, RuntimeWorldIdentityIssuer, RuntimeWorldOwnerIdentity,
};

/// Unforgeable capability held only during the canonical owner construction
/// call. It prevents any other crate module from resetting the identity issuer.
#[derive(Debug)]
pub(crate) struct RuntimeWorldOwnerConstructionCapability {
    _private: (),
}

impl RuntimeWorldOwnerConstructionCapability {
    fn new() -> Self {
        Self { _private: () }
    }
}

/// The serial owner-construction seam. It creates the sole issuer once and
/// does not expose a reset or an owner-identity substitution path.
#[derive(Debug)]
pub(crate) struct RuntimeWorldOwnerConstructionContract {
    issuer: RuntimeWorldIdentityIssuer,
}

impl RuntimeWorldOwnerConstructionContract {
    pub(crate) fn new() -> Result<Self, RuntimeWorldIdentityExhaustion> {
        let capability = RuntimeWorldOwnerConstructionCapability::new();
        let (issuer, _) = crate::identity::issuer_for_owner_construction(&capability)?;
        Ok(Self { issuer })
    }

    pub(crate) const fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.issuer.owner()
    }

    pub(crate) const fn issuer(&self) -> &RuntimeWorldIdentityIssuer {
        &self.issuer
    }

    #[cfg(test)]
    pub(crate) fn issuer_mut(&mut self) -> &mut RuntimeWorldIdentityIssuer {
        &mut self.issuer
    }
}

#[cfg(test)]
mod tests {
    use crate::identity::RuntimeWorldIdentityFamily;

    use super::RuntimeWorldOwnerConstructionContract;

    #[test]
    fn owner_construction_owns_one_non_resettable_issuer() {
        let first =
            RuntimeWorldOwnerConstructionContract::new().expect("first managed owner identity");
        let second =
            RuntimeWorldOwnerConstructionContract::new().expect("second managed owner identity");
        assert_ne!(first.owner_identity(), second.owner_identity());
    }

    #[test]
    fn owner_issuer_keeps_families_scoped_and_checked() {
        let mut construction =
            RuntimeWorldOwnerConstructionContract::new().expect("owner identity");
        let owner = construction.owner_identity();
        let issuer = construction.issuer_mut();
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
        assert_ne!(
            issuer.composite_commit().unwrap(),
            issuer.composite_commit().unwrap()
        );

        issuer.set_next_publication_attempt_for_test(u64::MAX);
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
