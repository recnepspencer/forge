use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct NonCurrentStagingMutationScope<'a> {
    root: &'a Path,
    staging_plan_fingerprint: [u8; 32],
}

impl<'a> NonCurrentStagingMutationScope<'a> {
    pub(crate) const fn new(root: &'a Path, staging_plan_fingerprint: [u8; 32]) -> Self {
        Self {
            root,
            staging_plan_fingerprint,
        }
    }

    pub const fn root(self) -> &'a Path {
        self.root
    }

    pub const fn staging_plan_fingerprint(self) -> [u8; 32] {
        self.staging_plan_fingerprint
    }
}

pub trait NonCurrentStagingOwnerEffect {
    fn effect_fingerprint(&self) -> [u8; 32];
}

#[derive(Debug)]
pub enum NonCurrentStagingOwnerExecutionDenial<OwnerDenial> {
    Backend(super::NonCurrentStagingExecutionDenial),
    Owner(OwnerDenial),
}

impl<OwnerDenial> From<super::NonCurrentStagingExecutionDenial>
    for NonCurrentStagingOwnerExecutionDenial<OwnerDenial>
{
    fn from(value: super::NonCurrentStagingExecutionDenial) -> Self {
        Self::Backend(value)
    }
}

impl super::PhysicalRecoveryStagingOwner {
    pub fn execute_lowered_guarded_with_owner_effect<Effect, OwnerDenial>(
        plan: super::LoweredNonCurrentStagingPlan,
        mut continuation: impl FnMut(super::NonCurrentStagingBoundary) -> bool,
        owner_effect: impl FnOnce(NonCurrentStagingMutationScope<'_>) -> Result<Effect, OwnerDenial>,
    ) -> Result<
        (super::NonCurrentStagingExecutionReceipt, Effect),
        NonCurrentStagingOwnerExecutionDenial<OwnerDenial>,
    >
    where
        Effect: NonCurrentStagingOwnerEffect,
    {
        let copied = super::execution::copy_lowered(&plan, &mut continuation)?;
        super::execution::admit_continuation(
            &mut continuation,
            super::NonCurrentStagingBoundary::OwnerEffect,
        )?;
        let effect = owner_effect(NonCurrentStagingMutationScope::new(
            plan.binding().staging_root(),
            plan.binding().fingerprint(),
        ))
        .map_err(NonCurrentStagingOwnerExecutionDenial::Owner)?;
        super::execution::admit_continuation(
            &mut continuation,
            super::NonCurrentStagingBoundary::OwnerEffectApplied,
        )?;
        let fingerprint = effect.effect_fingerprint();
        let receipt =
            super::execution::finalize_lowered(plan, copied, fingerprint, &mut continuation)?;
        Ok((receipt, effect))
    }
}
