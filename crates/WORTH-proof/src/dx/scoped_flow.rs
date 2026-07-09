use crate::assumption::{AssumptionBasis, CurrentValidity, FreshnessScopedBasis};
use crate::proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};
use crate::recipe::{Admitted, ExecutionReadyRecipe, Lowered, Recipe, Resolved, Unresolved};

use super::{LoweredRecipeDxExt, ResolvedRecipeDxExt, UnresolvedRecipeDxExt};

type CurrentBasis<B> = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>;

pub struct NoResolutionAuthority;
pub struct NoLoweringCapability;
pub struct NoReadinessAuthority;

pub struct ProofFlow<
    Resolution = NoResolutionAuthority,
    Lowering = NoLoweringCapability,
    Readiness = NoReadinessAuthority,
> {
    resolution_authority: Resolution,
    lowering_capability: Lowering,
    readiness_authority: Readiness,
}

pub fn proof_flow() -> ProofFlow {
    ProofFlow {
        resolution_authority: NoResolutionAuthority,
        lowering_capability: NoLoweringCapability,
        readiness_authority: NoReadinessAuthority,
    }
}

impl<R, L, Ready> ProofFlow<R, L, Ready> {
    pub fn resolution_authority<Auth>(
        self,
        authority: AuthorityWitness<Auth>,
    ) -> ProofFlow<AuthorityWitness<Auth>, L, Ready>
    where
        Auth: AuthorityMarker,
    {
        ProofFlow {
            resolution_authority: authority,
            lowering_capability: self.lowering_capability,
            readiness_authority: self.readiness_authority,
        }
    }

    pub fn lowering_capability<Cap>(
        self,
        capability: CapabilityWitness<Cap>,
    ) -> ProofFlow<R, CapabilityWitness<Cap>, Ready>
    where
        Cap: CapabilityMarker,
    {
        ProofFlow {
            resolution_authority: self.resolution_authority,
            lowering_capability: capability,
            readiness_authority: self.readiness_authority,
        }
    }

    pub fn readiness_authority<Auth>(
        self,
        authority: AuthorityWitness<Auth>,
    ) -> ProofFlow<R, L, AuthorityWitness<Auth>>
    where
        Auth: AuthorityMarker,
    {
        ProofFlow {
            resolution_authority: self.resolution_authority,
            lowering_capability: self.lowering_capability,
            readiness_authority: authority,
        }
    }

    pub fn recipe<T>(self, payload: T) -> ScopedUnresolvedRecipeFlow<T, R, L, Ready> {
        ScopedUnresolvedRecipeFlow {
            recipe: Recipe::<Unresolved, _>::new(payload),
            resolution_authority: self.resolution_authority,
            lowering_capability: self.lowering_capability,
            readiness_authority: self.readiness_authority,
        }
    }
}

pub struct ScopedUnresolvedRecipeFlow<T, R, L, Ready> {
    recipe: Recipe<Unresolved, T>,
    resolution_authority: R,
    lowering_capability: L,
    readiness_authority: Ready,
}

impl<T, R, L, Ready> ScopedUnresolvedRecipeFlow<T, R, L, Ready> {
    pub fn into_raw(self) -> Recipe<Unresolved, T> {
        self.recipe
    }

    pub fn resolve_with<B, Auth>(
        self,
        authority: AuthorityWitness<Auth>,
        basis: B,
    ) -> ScopedResolvedRecipeFlow<T, B, L, Ready>
    where
        Auth: AuthorityMarker,
    {
        ScopedResolvedRecipeFlow {
            recipe: self.recipe.resolve_with(authority, basis),
            lowering_capability: self.lowering_capability,
            readiness_authority: self.readiness_authority,
        }
    }
}

impl<T, Auth, L, Ready> ScopedUnresolvedRecipeFlow<T, AuthorityWitness<Auth>, L, Ready>
where
    Auth: AuthorityMarker,
{
    pub fn resolve<B>(self, basis: B) -> ScopedResolvedRecipeFlow<T, B, L, Ready> {
        ScopedResolvedRecipeFlow {
            recipe: self.recipe.resolve_with(self.resolution_authority, basis),
            lowering_capability: self.lowering_capability,
            readiness_authority: self.readiness_authority,
        }
    }
}

pub struct ScopedResolvedRecipeFlow<T, B, L, Ready> {
    recipe: Recipe<Resolved, T, CurrentBasis<B>>,
    lowering_capability: L,
    readiness_authority: Ready,
}

impl<T, B, L, Ready> ScopedResolvedRecipeFlow<T, B, L, Ready> {
    pub fn into_raw(self) -> Recipe<Resolved, T, CurrentBasis<B>> {
        self.recipe
    }

    pub fn lower_with<Cap>(
        self,
        capability: CapabilityWitness<Cap>,
    ) -> ScopedLoweredRecipeFlow<T, B, Ready>
    where
        Cap: CapabilityMarker,
    {
        ScopedLoweredRecipeFlow {
            recipe: self.recipe.lower_with(capability),
            readiness_authority: self.readiness_authority,
        }
    }
}

impl<T, B, Cap, Ready> ScopedResolvedRecipeFlow<T, B, CapabilityWitness<Cap>, Ready>
where
    Cap: CapabilityMarker,
{
    pub fn lower(self) -> ScopedLoweredRecipeFlow<T, B, Ready> {
        ScopedLoweredRecipeFlow {
            recipe: self.recipe.lower_with(self.lowering_capability),
            readiness_authority: self.readiness_authority,
        }
    }
}

pub struct ScopedLoweredRecipeFlow<T, B, Ready> {
    recipe: Recipe<Lowered, T, CurrentBasis<B>>,
    readiness_authority: Ready,
}

impl<T, B, Ready> ScopedLoweredRecipeFlow<T, B, Ready> {
    pub fn into_raw(self) -> Recipe<Lowered, T, CurrentBasis<B>> {
        self.recipe
    }

    pub fn admit_with<Auth>(
        self,
        authority: AuthorityWitness<Auth>,
    ) -> Recipe<Admitted, T, CurrentBasis<B>>
    where
        Auth: AuthorityMarker,
    {
        self.recipe.admit_with(authority)
    }

    pub fn ready_with<Rt, Auth>(
        self,
        authority: AuthorityWitness<Auth>,
        runtime: Rt,
    ) -> ExecutionReadyRecipe<T, CurrentBasis<B>>
    where
        Auth: AuthorityMarker,
    {
        self.recipe.ready_with(authority, runtime)
    }
}

impl<T, B, Auth> ScopedLoweredRecipeFlow<T, B, AuthorityWitness<Auth>>
where
    Auth: AuthorityMarker,
{
    pub fn ready<Rt>(self, runtime: Rt) -> ExecutionReadyRecipe<T, CurrentBasis<B>> {
        self.recipe.ready_with(self.readiness_authority, runtime)
    }
}
