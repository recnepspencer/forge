use forge_proof::prelude::{
    recipe, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    ExecutionReadyRecipeDxExt, LoweredRecipeDxExt, ResolvedRecipeDxExt, UnresolvedRecipeDxExt,
};
use forge_proof::{
    AssumptionBasis, CurrentValidity, ExecutedRecipe, ExecutionReadyRecipe, FreshnessScopedBasis,
    Lowered, Recipe, Resolved,
};
use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_readiness::{
    S6ClosedS7PlacementAdmissionSeed, S6LaterMilestoneDestination, S7PlacementReadinessNonClaim,
};

use crate::BlobLifecycleDeclaration;

#[derive(Debug)]
struct BlobLifecycleResolutionAuthorityMarker;
impl AuthorityMarker for BlobLifecycleResolutionAuthorityMarker {}

#[derive(Debug)]
struct BlobLifecycleLoweringCapabilityMarker;
impl CapabilityMarker for BlobLifecycleLoweringCapabilityMarker {}

#[derive(Debug)]
struct BlobLifecycleReadinessAuthorityMarker;
impl AuthorityMarker for BlobLifecycleReadinessAuthorityMarker {}

#[derive(Debug)]
pub struct BlobLifecycleStoreAuthority {
    current_authority: StoreCurrentAuthorityWitness,
    resolution_authority: AuthorityWitness<BlobLifecycleResolutionAuthorityMarker>,
}

#[derive(Debug)]
pub struct BlobLifecycleLoweringCapability {
    capability: CapabilityWitness<BlobLifecycleLoweringCapabilityMarker>,
}

#[derive(Debug)]
pub struct BlobLifecycleReadinessAuthority {
    placement_readiness: BlobLifecyclePlacementReadiness,
    readiness_authority: AuthorityWitness<BlobLifecycleReadinessAuthorityMarker>,
}

impl BlobLifecycleStoreAuthority {
    pub fn from_current_store_authority(current_authority: StoreCurrentAuthorityWitness) -> Self {
        Self {
            current_authority,
            resolution_authority: AuthorityWitness::from_authority_marker(
                BlobLifecycleResolutionAuthorityMarker,
            ),
        }
    }

    pub fn lowering_capability(&self) -> BlobLifecycleLoweringCapability {
        BlobLifecycleLoweringCapability {
            capability: CapabilityWitness::from_capability_marker(
                BlobLifecycleLoweringCapabilityMarker,
            ),
        }
    }

    fn into_resolution_parts(
        self,
    ) -> (
        StoreCurrentAuthorityWitness,
        AuthorityWitness<BlobLifecycleResolutionAuthorityMarker>,
    ) {
        (self.current_authority, self.resolution_authority)
    }
}

impl BlobLifecycleLoweringCapability {
    fn into_capability(self) -> CapabilityWitness<BlobLifecycleLoweringCapabilityMarker> {
        self.capability
    }
}

impl BlobLifecycleReadinessAuthority {
    pub fn from_s6_placement_seed(placement_seed: S6ClosedS7PlacementAdmissionSeed) -> Self {
        Self {
            placement_readiness: BlobLifecyclePlacementReadiness::from_s6_seed(placement_seed),
            readiness_authority: AuthorityWitness::from_authority_marker(
                BlobLifecycleReadinessAuthorityMarker,
            ),
        }
    }

    pub(crate) const fn placement_readiness(&self) -> BlobLifecyclePlacementReadiness {
        self.placement_readiness
    }

    fn into_readiness_authority(self) -> AuthorityWitness<BlobLifecycleReadinessAuthorityMarker> {
        self.readiness_authority
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BlobLifecyclePlacementReadiness {
    destination: S6LaterMilestoneDestination,
    s6_non_claims: [S7PlacementReadinessNonClaim; 3],
}

impl BlobLifecyclePlacementReadiness {
    const fn from_s6_seed(seed: S6ClosedS7PlacementAdmissionSeed) -> Self {
        Self {
            destination: seed.destination(),
            s6_non_claims: *seed.non_claims(),
        }
    }

    pub(crate) const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub(crate) const fn s6_non_claims(&self) -> &[S7PlacementReadinessNonClaim; 3] {
        &self.s6_non_claims
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlobLifecycleProofPayload {
    declaration: BlobLifecycleDeclaration,
}

type BlobLifecycleProofBasis =
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<StoreCurrentAuthorityWitness>>;
pub(crate) type BlobLifecycleResolvedRecipe =
    Recipe<Resolved, BlobLifecycleProofPayload, BlobLifecycleProofBasis>;
pub(crate) type BlobLifecycleLoweredRecipe =
    Recipe<Lowered, BlobLifecycleProofPayload, BlobLifecycleProofBasis>;
pub(crate) type BlobLifecycleExecutionReadyRecipe =
    ExecutionReadyRecipe<BlobLifecycleProofPayload, BlobLifecycleProofBasis>;
pub(crate) type BlobLifecycleExecutedRecipe =
    ExecutedRecipe<BlobLifecycleProofPayload, BlobLifecycleProofBasis>;

impl BlobLifecycleProofPayload {
    pub(crate) const fn from_declaration(declaration: BlobLifecycleDeclaration) -> Self {
        Self { declaration }
    }

    pub(crate) const fn declaration(&self) -> &BlobLifecycleDeclaration {
        &self.declaration
    }
}

pub(crate) fn prove_lifecycle_resolution(
    authority: BlobLifecycleStoreAuthority,
    declaration: BlobLifecycleDeclaration,
) -> BlobLifecycleResolvedRecipe {
    let (current_authority, resolution_authority) = authority.into_resolution_parts();
    recipe(BlobLifecycleProofPayload::from_declaration(declaration))
        .resolve_with(resolution_authority, current_authority)
}

pub(crate) fn prove_lifecycle_lowering(
    resolved: BlobLifecycleResolvedRecipe,
    capability: BlobLifecycleLoweringCapability,
) -> BlobLifecycleLoweredRecipe {
    resolved.lower_with(capability.into_capability())
}

pub(crate) fn prove_lifecycle_readiness(
    lowered: BlobLifecycleLoweredRecipe,
    readiness: BlobLifecycleReadinessAuthority,
) -> (
    BlobLifecyclePlacementReadiness,
    BlobLifecycleExecutionReadyRecipe,
) {
    let placement_readiness = readiness.placement_readiness();
    let ready = lowered.ready_with(readiness.into_readiness_authority(), placement_readiness);
    (placement_readiness, ready)
}

pub(crate) fn execute_lifecycle_proof(
    ready: BlobLifecycleExecutionReadyRecipe,
) -> BlobLifecycleExecutedRecipe {
    ready.execute()
}
