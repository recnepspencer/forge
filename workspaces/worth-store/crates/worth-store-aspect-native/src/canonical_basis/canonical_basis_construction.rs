use worth_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisReadyArtifact,
    CanonicalizationRuleVersion,
};
use worth_foundational::{
    prepare_aspect_patch_for_canonical_basis, prepare_aspect_state_for_canonical_basis,
};
use worth_proof::TransitionOutcome;

use super::canonical_basis_entries::{aspect_boundary_entries, aspect_patch_entries};
use crate::{
    certify_canonical_basis_source, StoreAspectBoundaryFact, StoreAspectPatchBoundaryFact,
    StoreCanonicalBasisConstructionDenial, StoreCanonicalBasisFamily,
    StoreCanonicalBasisSourceKind,
};

pub type StoreCanonicalBasisConstructionOutcome =
    TransitionOutcome<CanonicalBasisReadyArtifact, StoreCanonicalBasisConstructionDenial>;

#[derive(Debug, Clone)]
pub struct StoreCanonicalBasisConstruction {
    family: StoreCanonicalBasisFamily,
    native_source: Option<StoreCanonicalBasisNativeSource>,
    conflicting_native_source: bool,
}

impl StoreCanonicalBasisConstruction {
    pub const fn for_family(family: StoreCanonicalBasisFamily) -> Self {
        Self {
            family,
            native_source: None,
            conflicting_native_source: false,
        }
    }

    pub fn with_aspect_boundary_fact(mut self, fact: &StoreAspectBoundaryFact) -> Self {
        self.put_native_source(StoreCanonicalBasisNativeSource::AspectState(fact.clone()));
        self
    }

    pub fn with_aspect_patch_boundary_fact(mut self, fact: &StoreAspectPatchBoundaryFact) -> Self {
        self.put_native_source(StoreCanonicalBasisNativeSource::AspectPatch(fact.clone()));
        self
    }

    pub fn prepare(
        self,
        version: CanonicalizationRuleVersion,
    ) -> StoreCanonicalBasisConstructionOutcome {
        let Self {
            family,
            native_source,
            conflicting_native_source,
        } = self;

        if conflicting_native_source {
            return TransitionOutcome::denied(
                StoreCanonicalBasisConstructionDenial::ConflictingNativeSources { family },
            );
        }

        let Some(source) = native_source else {
            return TransitionOutcome::denied(
                StoreCanonicalBasisConstructionDenial::MissingNativeSource { family },
            );
        };

        match source {
            StoreCanonicalBasisNativeSource::AspectState(fact) => {
                prepare_aspect_state(family, version, fact)
            }
            StoreCanonicalBasisNativeSource::AspectPatch(fact) => {
                prepare_aspect_patch(family, version, fact)
            }
        }
    }

    fn put_native_source(&mut self, source: StoreCanonicalBasisNativeSource) {
        if self.native_source.is_some() {
            self.conflicting_native_source = true;
        }
        self.native_source = Some(source);
    }
}

fn prepare_aspect_state(
    family: StoreCanonicalBasisFamily,
    version: CanonicalizationRuleVersion,
    fact: StoreAspectBoundaryFact,
) -> StoreCanonicalBasisConstructionOutcome {
    if let Err(denial) = certify_canonical_basis_source(
        family,
        StoreCanonicalBasisSourceKind::FoundationalAspectState,
    ) {
        return TransitionOutcome::denied(denial.into());
    }

    let boundary_witness = fact.authority_input().physical_witness();
    let foundational = match prepare_aspect_state_for_canonical_basis(
        version,
        fact.authority_input().admitted_state().clone(),
    ) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            return TransitionOutcome::denied(StoreCanonicalBasisConstructionDenial::Foundational(
                denial,
            ));
        }
        TransitionOutcome::Deferred(value) => match value {},
        TransitionOutcome::Stale(value) => match value {},
        TransitionOutcome::RebindRequired(value) => match value {},
        TransitionOutcome::Failed(value) => match value {},
    };

    map_foundational_outcome(prepare_canonical_basis_sequence(
        foundational.payload().version().clone(),
        CanonicalBasisDomain::Future("store.aspect.boundary.fact"),
        aspect_boundary_entries(foundational.payload().entries(), boundary_witness),
    ))
}

fn prepare_aspect_patch(
    family: StoreCanonicalBasisFamily,
    version: CanonicalizationRuleVersion,
    fact: StoreAspectPatchBoundaryFact,
) -> StoreCanonicalBasisConstructionOutcome {
    if let Err(denial) = certify_canonical_basis_source(
        family,
        StoreCanonicalBasisSourceKind::FoundationalAspectPatch,
    ) {
        return TransitionOutcome::denied(denial.into());
    }

    let boundary_witness = fact.patch_input().physical_witness();
    let foundational =
        match prepare_aspect_patch_for_canonical_basis(version, fact.patch_input().patch()) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return TransitionOutcome::denied(
                    StoreCanonicalBasisConstructionDenial::Foundational(denial),
                );
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };

    map_foundational_outcome(prepare_canonical_basis_sequence(
        foundational.payload().version().clone(),
        CanonicalBasisDomain::Future("store.aspect.patch.boundary.fact"),
        aspect_patch_entries(foundational.payload().entries(), boundary_witness),
    ))
}

#[derive(Debug, Clone)]
enum StoreCanonicalBasisNativeSource {
    AspectState(StoreAspectBoundaryFact),
    AspectPatch(StoreAspectPatchBoundaryFact),
}

fn map_foundational_outcome(
    outcome: TransitionOutcome<
        CanonicalBasisReadyArtifact,
        worth_foundational::CanonicalBasisConstructionDenial,
    >,
) -> StoreCanonicalBasisConstructionOutcome {
    match outcome {
        TransitionOutcome::Success(ready) => TransitionOutcome::success(ready),
        TransitionOutcome::Denied(denial) => {
            TransitionOutcome::denied(StoreCanonicalBasisConstructionDenial::Foundational(denial))
        }
        TransitionOutcome::Deferred(value) => match value {},
        TransitionOutcome::Stale(value) => match value {},
        TransitionOutcome::RebindRequired(value) => match value {},
        TransitionOutcome::Failed(value) => match value {},
    }
}
