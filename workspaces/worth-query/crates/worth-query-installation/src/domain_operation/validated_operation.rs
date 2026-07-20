use worth_proof::{
    Artifact, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

use super::WorthQueryPortableDomainOperationDefinition;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryValidatedDomainOperation {
    artifact: ValidatedDomainOperationArtifact,
}

impl WorthQueryValidatedDomainOperation {
    pub(crate) fn admit(
        definition: WorthQueryPortableDomainOperationDefinition,
    ) -> Result<Self, &'static str> {
        super::validation::validate_domain_operation_meaning(&definition)?;
        let basis = definition.canonical_identity().to_string();
        let authority =
            AuthorityWitness::from_authority_marker(DomainOperationValidationAuthority {
                _private: (),
            });
        let artifact = Artifact::with_proofs_and_current_basis(
            definition,
            Proof::from_authority_witness(&authority),
            basis,
            authority,
        );
        Ok(Self { artifact })
    }

    pub(crate) fn definition(&self) -> &WorthQueryPortableDomainOperationDefinition {
        self.artifact.payload()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DomainOperationValidated;
impl PhaseMarker for DomainOperationValidated {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DomainOperationMeaningValidated;
impl ProofMarker for DomainOperationMeaningValidated {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DomainOperationValidationAuthority {
    _private: (),
}
impl AuthorityMarker for DomainOperationValidationAuthority {}
impl AuthorityProves<DomainOperationMeaningValidated> for DomainOperationValidationAuthority {}

type ValidatedDomainOperationArtifact = Artifact<
    DomainOperationValidated,
    WorthQueryPortableDomainOperationDefinition,
    Proof<DomainOperationMeaningValidated, DomainOperationValidationAuthority>,
    FreshnessScopedBasis<CurrentValidity, worth_proof::AssumptionBasis<String>>,
>;
