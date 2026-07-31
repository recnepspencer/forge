use worth_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalSingleSequenceDigestAlgorithmSlot,
};
use worth_proof::TransitionOutcome;

use super::canonical_basis::PhysicalScenarioCanonicalBasis;
use super::definition::PhysicalSimulationScenarioDefinition;
use super::denial::PhysicalScenarioDefinitionDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioCanonicalIdentity {
    digest: CanonicalDerivedDigest,
}

impl PhysicalScenarioCanonicalIdentity {
    pub(crate) fn from_definition(
        definition: &PhysicalSimulationScenarioDefinition,
    ) -> Result<Self, PhysicalScenarioDefinitionDenial> {
        let canonical_basis = PhysicalScenarioCanonicalBasis::from_definition(definition)?;
        let ready_basis = canonical_basis.ready();
        let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            CanonicalDigestAlgorithmId::sha256(),
            ready_basis.payload().domain(),
            ready_basis.payload().version().clone(),
        );
        match admit_canonical_sequence_digest_derivation(ready_basis, slot) {
            TransitionOutcome::Success(ready) => Ok(Self {
                digest: derive_canonical_digest(ready),
            }),
            TransitionOutcome::Denied(denial) => {
                Err(PhysicalScenarioDefinitionDenial::ScenarioDigestDerivationDenied(denial))
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        }
    }

    pub fn digest_bytes(&self) -> &[u8; 32] {
        self.digest.value().bytes()
    }

    pub fn canonical_basis_entry_count(&self) -> u32 {
        self.digest.metadata().entry_count()
    }
}
