use super::{
    ConsumedProjectionAuthorityCounters, ConsumedProjectionAuthorityDenial,
    ConsumedProjectionAuthorityDenialKind, ConsumedProjectionAuthorityEvidence,
    ProjectionAuthorityContract, ProjectionAuthorityRequirement,
};
use crate::basis::ResolvedSnapshotBasis;
use crate::projection_consumption::{
    CompletedProjectionFactConsumption, ConsumedProjectionFactSet, MaterializedProjectionContract,
    ProjectionConsumptionReceipt, ProjectionSourceBasisAuthority, ProjectionSourceFamily,
    ProjectionSourceIdentity, ProjectionSourceReferenceIdentity,
};

#[derive(Debug)]
pub struct WorthQueryConsumedProjectionAuthority {
    completed: CompletedProjectionFactConsumption,
    counters: ConsumedProjectionAuthorityCounters,
    consumer_contract: ProjectionAuthorityContract,
}

impl WorthQueryConsumedProjectionAuthority {
    pub fn source_family(&self) -> ProjectionSourceFamily {
        self.completed.source_family()
    }

    pub fn source_identity(&self) -> &ProjectionSourceIdentity {
        self.completed.contract().source_identity_handle()
    }

    pub fn basis_authority(&self) -> &ProjectionSourceBasisAuthority {
        self.completed.contract().basis_authority()
    }

    /// Certifies that `basis` is the exact runtime snapshot basis retained by
    /// this authority. Consumers must not recreate this relationship from a
    /// terminal digest projection.
    pub fn binds_resolved_basis(&self, basis: &ResolvedSnapshotBasis) -> bool {
        self.basis_authority().binds_resolved_basis(basis)
    }

    pub fn source_references(&self) -> &[ProjectionSourceReferenceIdentity] {
        self.completed.contract().source_reference_identities()
    }

    pub fn contract(&self) -> &MaterializedProjectionContract {
        self.completed.contract()
    }

    pub fn facts(&self) -> &ConsumedProjectionFactSet {
        self.completed.facts()
    }

    pub fn receipt(&self) -> &ProjectionConsumptionReceipt {
        self.completed.receipt()
    }

    pub fn counters(&self) -> &ConsumedProjectionAuthorityCounters {
        &self.counters
    }

    pub fn consumer_contract(&self) -> &ProjectionAuthorityContract {
        &self.consumer_contract
    }

    pub fn evidence(&self) -> ConsumedProjectionAuthorityEvidence {
        ConsumedProjectionAuthorityEvidence::new(
            self.source_family(),
            self.source_identity().as_str().to_string(),
            self.receipt().receipt_digest().to_string(),
            self.counters.clone(),
        )
    }

    pub fn structurally_equivalent(&self, other: &Self) -> bool {
        self.completed.declaration() == other.completed.declaration()
            && self.completed.contract() == other.completed.contract()
            && self.completed.facts() == other.completed.facts()
            && self.completed.receipt() == other.completed.receipt()
            && self.consumer_contract == other.consumer_contract
    }

    pub(super) fn seal(
        completed: CompletedProjectionFactConsumption,
        consumer_contract: ProjectionAuthorityContract,
    ) -> Result<Self, ConsumedProjectionAuthorityDenial> {
        let mut relationship_checks = 0;
        let mut requirement_checks = 0;
        let mut source_reference_checks = 0;
        let declaration = completed.declaration();
        let contract = completed.contract();
        let facts = completed.facts();
        let receipt = completed.receipt();

        check(
            &mut relationship_checks,
            declaration.declaration_digest() == contract.declaration_digest(),
            ConsumedProjectionAuthorityDenialKind::DeclarationContractMismatch,
            source_reference_checks,
            requirement_checks,
        )?;
        check(
            &mut relationship_checks,
            declaration.source().source_identity_handle() == contract.source_identity_handle(),
            ConsumedProjectionAuthorityDenialKind::DeclarationSourceMismatch,
            source_reference_checks,
            requirement_checks,
        )?;
        check(
            &mut relationship_checks,
            declaration.source().basis_authority() == contract.basis_authority(),
            ConsumedProjectionAuthorityDenialKind::DeclarationBasisMismatch,
            source_reference_checks,
            requirement_checks,
        )?;
        source_reference_checks = declaration.source().source_reference_identities().len();
        check(
            &mut relationship_checks,
            declaration.source().source_reference_identities()
                == contract.source_reference_identities(),
            ConsumedProjectionAuthorityDenialKind::SourceReferenceMismatch,
            source_reference_checks,
            requirement_checks,
        )?;
        check(
            &mut relationship_checks,
            contract.contract_digest() == facts.contract_digest(),
            ConsumedProjectionAuthorityDenialKind::ContractFactSetMismatch,
            source_reference_checks,
            requirement_checks,
        )?;
        check(
            &mut relationship_checks,
            facts.fact_set_digest() == receipt.fact_set_digest(),
            ConsumedProjectionAuthorityDenialKind::FactSetReceiptMismatch,
            source_reference_checks,
            requirement_checks,
        )?;
        check(
            &mut relationship_checks,
            contract.source_family() == facts.source_family()
                && facts.source_family() == receipt.source_family(),
            ConsumedProjectionAuthorityDenialKind::SourceFamilyMismatch,
            source_reference_checks,
            requirement_checks,
        )?;
        check(
            &mut relationship_checks,
            contract.source_identity_handle() == facts.source_identity_handle()
                && facts.source_identity() == receipt.source_identity(),
            ConsumedProjectionAuthorityDenialKind::SourceIdentityMismatch,
            source_reference_checks,
            requirement_checks,
        )?;
        check(
            &mut relationship_checks,
            contract.support_posture() == facts.support_posture()
                && facts.support_posture() == receipt.support_posture(),
            ConsumedProjectionAuthorityDenialKind::SupportPostureMismatch,
            source_reference_checks,
            requirement_checks,
        )?;

        check(
            &mut relationship_checks,
            declaration
                .requested()
                .requested()
                .eq(consumer_contract.requested_facts()),
            ConsumedProjectionAuthorityDenialKind::ContractRequestMismatch,
            source_reference_checks,
            requirement_checks,
        )?;
        for requirement in consumer_contract.requirements() {
            requirement_checks += 1;
            let satisfied = match requirement {
                ProjectionAuthorityRequirement::SettledConsumption => true,
                ProjectionAuthorityRequirement::SourceAuthority => true,
                ProjectionAuthorityRequirement::BasisGeneration => {
                    contract.basis_authority().has_basis_generation()
                }
                ProjectionAuthorityRequirement::TargetIdentity => {
                    !facts.target_identities().is_empty()
                }
            };
            if !satisfied {
                return Err(ConsumedProjectionAuthorityDenial::new(
                    ConsumedProjectionAuthorityDenialKind::MissingRequirement(requirement),
                    relationship_checks,
                    requirement_checks,
                    source_reference_checks,
                ));
            }
        }

        let counters = ConsumedProjectionAuthorityCounters::checked(
            relationship_checks,
            requirement_checks,
            source_reference_checks,
            facts.counters().extracted_fact_count(),
        );
        Ok(Self {
            completed,
            counters,
            consumer_contract,
        })
    }
}

fn check(
    relationship_checks: &mut usize,
    condition: bool,
    kind: ConsumedProjectionAuthorityDenialKind,
    source_reference_checks: usize,
    requirement_checks: usize,
) -> Result<(), ConsumedProjectionAuthorityDenial> {
    *relationship_checks += 1;
    if condition {
        Ok(())
    } else {
        Err(ConsumedProjectionAuthorityDenial::new(
            kind,
            *relationship_checks,
            requirement_checks,
            source_reference_checks,
        ))
    }
}
