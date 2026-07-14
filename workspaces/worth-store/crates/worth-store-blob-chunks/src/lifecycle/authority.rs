use worth_proof::prelude::{
    recipe, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    ExecutionReadyRecipeDxExt, LoweredRecipeDxExt, ResolvedRecipeDxExt, UnresolvedRecipeDxExt,
};
use worth_proof::{
    AssumptionBasis, CurrentValidity, ExecutedRecipe, ExecutionReadyRecipe, FreshnessScopedBasis,
    Lowered, Recipe, Resolved,
};
use worth_store_authority::StoreCurrentAuthorityWitness;

use crate::{AdmittedBlobPlacement, BlobLifecycleDeclaration};

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
    admitted_placement: AdmittedBlobPlacement,
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
    pub fn from_admitted_placement(admitted_placement: AdmittedBlobPlacement) -> Self {
        Self {
            admitted_placement,
            readiness_authority: AuthorityWitness::from_authority_marker(
                BlobLifecycleReadinessAuthorityMarker,
            ),
        }
    }

    pub(crate) const fn admitted_placement(&self) -> &AdmittedBlobPlacement {
        &self.admitted_placement
    }

    fn into_readiness_parts(
        self,
    ) -> (
        AdmittedBlobPlacement,
        AuthorityWitness<BlobLifecycleReadinessAuthorityMarker>,
    ) {
        (self.admitted_placement, self.readiness_authority)
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
) -> (AdmittedBlobPlacement, BlobLifecycleExecutionReadyRecipe) {
    let (admitted_placement, readiness_authority) = readiness.into_readiness_parts();
    let ready = lowered.ready_with(readiness_authority, admitted_placement.clone());
    (admitted_placement, ready)
}

pub(crate) fn execute_lifecycle_proof(
    ready: BlobLifecycleExecutionReadyRecipe,
) -> BlobLifecycleExecutedRecipe {
    ready.execute()
}
