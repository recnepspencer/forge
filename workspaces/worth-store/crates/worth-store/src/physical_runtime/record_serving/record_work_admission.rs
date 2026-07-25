use worth_foundational::{
    aspects, AspectContract, AspectMask, AspectValue, ContractValidationInput, InternedString,
    MutationMask, ProjectionMask, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectContractAdmission,
    StoreAspectIdentity, StoreAspectPatchAuthorityInput, StoreAspectPatchBoundaryFact,
    StorePhysicalBoundaryWitness,
};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthorityBoundSecurityScopeReceipt, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use crate::physical_runtime::work::{
    PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole, PhysicalWorkProfileDeclaration,
    PhysicalWorkProfileDenial, PhysicalWorkSemanticBasis, PhysicalWorkSignalFamily,
    PhysicalWorkSignalFamilySet,
};

use super::RecordPublicationStage;

mod read_partition;
#[cfg(test)]
mod tests;

const PUBLICATION_ASPECT_KEY: &str = "store.physical.record.publication-basis";
const RECORD_ASPECT_REVISION: u64 = 1;
pub(in crate::physical_runtime) use read_partition::RecordReadPartition;
use read_partition::RecordReadSemanticBases;

#[derive(Clone, Debug)]
pub(in crate::physical_runtime) struct RecordWorkAdmission {
    security: StoreAuthorityBoundSecurityScopeReceipt,
    scheduler_security: worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
    read_bases: RecordReadSemanticBases,
    publication_bases: RecordPublicationSemanticBases,
}

#[derive(Clone, Debug)]
struct RecordPublicationSemanticBases {
    candidate_data: PhysicalWorkSemanticBasis,
    data_synchronization: PhysicalWorkSemanticBasis,
    payload_manifest: PhysicalWorkSemanticBasis,
    manifest: PhysicalWorkSemanticBasis,
    catalog_candidate: PhysicalWorkSemanticBasis,
    catalog_replacement: PhysicalWorkSemanticBasis,
    namespace_synchronization: PhysicalWorkSemanticBasis,
}

impl RecordWorkAdmission {
    pub(in crate::physical_runtime) fn install(
        profile: PhysicalWorkProfileDeclaration,
    ) -> Result<(Self, PhysicalWorkProfileDeclaration), PhysicalWorkProfileDenial> {
        let witness = physical_witness();
        let read = read_partition::install(witness);
        let (publication_contract, publication_identity, publication_admission) =
            contract(PUBLICATION_ASPECT_KEY, 1_305, witness);
        let publication_bases = RecordPublicationSemanticBases::new(
            &publication_contract,
            publication_identity,
            witness,
            &publication_admission,
        );
        let extensions =
            read.declarations
                .into_iter()
                .chain([PhysicalSignalAspectDeclaration::new(
                    publication_admission,
                    PhysicalSignalAspectRole::DependencyAndOutput,
                )
                .for_families(
                    PhysicalWorkSignalFamilySet::only(PhysicalWorkSignalFamily::ExactWriteback)
                        .with(PhysicalWorkSignalFamily::Publication),
                )]);
        let profile = profile.with_native_extensions(read.security, extensions)?;
        Ok((
            Self {
                security: read.security,
                scheduler_security: read.scheduler_security,
                read_bases: read.bases,
                publication_bases,
            },
            profile,
        ))
    }

    pub(in crate::physical_runtime) const fn security(
        &self,
    ) -> StoreAuthorityBoundSecurityScopeReceipt {
        self.security
    }

    pub(in crate::physical_runtime) const fn scheduler_security(
        &self,
    ) -> &worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission {
        &self.scheduler_security
    }

    pub(in crate::physical_runtime) fn read_basis(
        &self,
        partition: RecordReadPartition,
    ) -> PhysicalWorkSemanticBasis {
        self.read_bases.for_partition(partition)
    }

    pub(in crate::physical_runtime) fn mutation_basis(
        &self,
        stage: RecordPublicationStage,
    ) -> PhysicalWorkSemanticBasis {
        self.publication_bases.for_stage(stage)
    }
}

impl RecordPublicationSemanticBases {
    fn new(
        contract: &AspectContract,
        identity: StoreAspectIdentity,
        witness: StorePhysicalBoundaryWitness,
        admission: &StoreAspectContractAdmission,
    ) -> Self {
        Self {
            candidate_data: publication_basis(
                contract,
                identity.clone(),
                witness,
                admission,
                "candidate-data-work-admitted",
            ),
            data_synchronization: publication_basis(
                contract,
                identity.clone(),
                witness,
                admission,
                "data-synchronization-work-admitted",
            ),
            payload_manifest: publication_basis(
                contract,
                identity.clone(),
                witness,
                admission,
                "payload-manifest-work-admitted",
            ),
            manifest: publication_basis(
                contract,
                identity.clone(),
                witness,
                admission,
                "manifest-work-admitted",
            ),
            catalog_candidate: publication_basis(
                contract,
                identity.clone(),
                witness,
                admission,
                "catalog-candidate-work-admitted",
            ),
            catalog_replacement: publication_basis(
                contract,
                identity.clone(),
                witness,
                admission,
                "catalog-replacement-work-admitted",
            ),
            namespace_synchronization: publication_basis(
                contract,
                identity,
                witness,
                admission,
                "namespace-synchronization-work-admitted",
            ),
        }
    }

    fn for_stage(&self, stage: RecordPublicationStage) -> PhysicalWorkSemanticBasis {
        match stage {
            RecordPublicationStage::CandidateDataWrite => self.candidate_data.clone(),
            RecordPublicationStage::DataSynchronization => self.data_synchronization.clone(),
            RecordPublicationStage::PayloadManifestSynchronization => self.payload_manifest.clone(),
            RecordPublicationStage::ManifestSynchronization => self.manifest.clone(),
            RecordPublicationStage::CatalogCandidateSynchronization => {
                self.catalog_candidate.clone()
            }
            RecordPublicationStage::CatalogReplacement => self.catalog_replacement.clone(),
            RecordPublicationStage::NamespaceSynchronization => {
                self.namespace_synchronization.clone()
            }
        }
    }
}

fn publication_basis(
    contract: &AspectContract,
    identity: StoreAspectIdentity,
    witness: StorePhysicalBoundaryWitness,
    admission: &StoreAspectContractAdmission,
    value: &'static str,
) -> PhysicalWorkSemanticBasis {
    PhysicalWorkSemanticBasis::mutation(
        patch_fact(contract, identity, witness, value),
        admission.clone(),
    )
    .expect("built-in publication stage patch and contract are constructed together")
}

fn contract(
    key: &'static str,
    contract_identity: u64,
    witness: StorePhysicalBoundaryWitness,
) -> (
    AspectContract,
    StoreAspectIdentity,
    StoreAspectContractAdmission,
) {
    let key = aspects()
        .vocabulary()
        .key(key)
        .expect("built-in record aspect key is canonical");
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(contract_identity))
        .at_revision(aspects().vocabulary().revision(RECORD_ASPECT_REVISION))
        .scalar(ScalarAspectType::String);
    let identity = StoreAspectIdentity::from_aspect_key(key);
    let admission = StoreAspectContractAdmission::new(identity.clone(), contract.clone(), witness)
        .expect("built-in record contract identity matches its key")
        .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
        .expect("built-in scalar record contract admits whole projection")
        .admit_mutation_mask(AspectMask::<MutationMask>::whole_aspect())
        .expect("built-in scalar record contract admits whole mutation");
    (contract, identity, admission)
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    let authority =
        worth_store_contracts::StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            worth_store_contracts::ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("record work is inside the aspect-native roadmap gate");
    StorePhysicalBoundaryWitness::from_physical_authority(authority)
        .expect("record work uses aspect-native physical authority")
}

fn boundary_fact(
    contract: &AspectContract,
    identity: StoreAspectIdentity,
    witness: StorePhysicalBoundaryWitness,
    value: &'static str,
) -> StoreAspectBoundaryFact {
    let value = validated_value(contract, value);
    let state = match aspects().authoritative_state().admit([value]) {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("built-in record aspect state must admit: {outcome:?}"),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        identity,
        StoreAspectAuthorityInput::new(state, witness),
    )
    .expect("built-in record state contains exactly its declared identity")
}

fn patch_fact(
    contract: &AspectContract,
    identity: StoreAspectIdentity,
    witness: StorePhysicalBoundaryWitness,
    value: &'static str,
) -> StoreAspectPatchBoundaryFact {
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(validated_value(contract, value))
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("built-in record aspect patch must admit: {outcome:?}"),
    };
    StoreAspectPatchBoundaryFact::from_authoritative_patch(
        identity,
        StoreAspectPatchAuthorityInput::new(patch, witness),
    )
    .expect("built-in record patch targets exactly its declared identity")
}

fn validated_value(
    contract: &AspectContract,
    value: &'static str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(ContractValidationInput::from(AspectValue::String(
            InternedString::from(value),
        ))) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("built-in record aspect value must validate: {outcome:?}"),
    }
}

fn security_scope(
    authority_fact: &StoreAspectBoundaryFact,
) -> (
    StoreAuthorityBoundSecurityScopeReceipt,
    worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
) {
    let current = worth_store_authority::require_current_store_authority(authority_fact.clone());
    let authenticity = StoreAuthenticityRequirement::not_required();
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &current,
        StoreKeyScope::StoreManagedRoot,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::StoreInternal,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
        expectation,
    );
    match worth_store_security::admit_store_security_scope(request) {
        TransitionOutcome::Success(scope) => {
            let scheduler = worth_store_io_scheduler::admit_security_scope_for_scheduler(&scope)
                .expect("built-in record scope is the scheduler's Store-internal scope");
            (scope.authority_bound_receipt(), scheduler)
        }
        outcome => panic!("built-in record security scope must admit: {outcome:?}"),
    }
}
