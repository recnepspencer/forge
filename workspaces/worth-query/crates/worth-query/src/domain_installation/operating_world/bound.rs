use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::basis_lifecycle::{AdmittedBasisCapability, BasisOperationLane};
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, WorthQueryBoundOperationPhase, WorthQueryOperationAuthorityBasis,
    WorthQueryOperationPhaseProof,
};

use super::super::{
    WorthQueryConsumerProjectionContract, WorthQueryConsumerProjectionContractDenial,
    WorthQueryConsumerSupportProfile, WorthQueryExecutableDomainOperation,
    WorthQueryInstalledDomainOperation, WorthQueryInstalledDomainOperationExecutor,
    WorthQueryInstalledGraphParticipationRecord, WorthQueryPublishingOperation,
};

type BoundOperationMarker<D, O, F> = fn() -> (D, O, F);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBoundCommitPosture {
    ReadOnly,
    Atomic,
    Compensated,
}

pub(crate) struct WorthQueryBoundGraphParticipation {
    pub(crate) role: String,
    pub(crate) record: Arc<WorthQueryInstalledGraphParticipationRecord>,
}

pub(crate) struct WorthQueryBoundRequiredDomain {
    pub(crate) role: String,
    pub(crate) authority: Arc<super::super::WorthQueryInstalledDomainAuthority>,
}

pub(crate) struct WorthQueryBoundAuthoritySet {
    pub(crate) graph_participations: Vec<WorthQueryBoundGraphParticipation>,
    pub(crate) required_domains: Vec<WorthQueryBoundRequiredDomain>,
    pub(crate) commit_posture: WorthQueryBoundCommitPosture,
    pub(super) shape_proofs: super::authority_shape::WorthQueryBoundAuthorityShapeProofs,
}

pub(crate) struct WorthQueryBoundRuntimeProviders {
    pub(crate) executor: Option<Arc<WorthQueryInstalledDomainOperationExecutor>>,
    pub(crate) workflow_executor:
        Option<Arc<super::super::WorthQueryInstalledWorkflowStageExecutor>>,
    pub(crate) workflow_parallel_admission_provider:
        Option<Arc<super::super::WorthQueryInstalledWorkflowParallelAdmissionProvider>>,
    pub(crate) conditional_nodes: Vec<Arc<super::super::WorthQueryInstalledConditionalNode>>,
}

static NEXT_BOUND_CAPABILITY_IDENTITY: AtomicU64 = AtomicU64::new(1);

pub struct WorthQueryBoundDomainOperation<D, O, F, L: BasisOperationLane> {
    operation: WorthQueryInstalledDomainOperation<D, O, F>,
    basis: AdmittedBasisCapability<L>,
    graph_participations: Vec<WorthQueryBoundGraphParticipation>,
    required_domains: Vec<WorthQueryBoundRequiredDomain>,
    commit_posture: WorthQueryBoundCommitPosture,
    binding_identity: String,
    capability_identity: u64,
    authority_proof: WorthQueryOperationPhaseProof<WorthQueryBoundOperationPhase>,
    _authority_shape_proofs: super::authority_shape::WorthQueryBoundAuthorityShapeProofs,
    consumer_support_profile: WorthQueryConsumerSupportProfile,
    consumer_contract_minted: Cell<bool>,
    executor: Option<Arc<WorthQueryInstalledDomainOperationExecutor>>,
    workflow_executor: Option<Arc<super::super::WorthQueryInstalledWorkflowStageExecutor>>,
    workflow_parallel_admission_provider:
        Option<Arc<super::super::WorthQueryInstalledWorkflowParallelAdmissionProvider>>,
    conditional_nodes: Vec<Arc<super::super::WorthQueryInstalledConditionalNode>>,
    _marker: PhantomData<BoundOperationMarker<D, O, F>>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryBoundDomainOperation<D, O, F, L> {
    pub(super) fn mint(
        operation: WorthQueryInstalledDomainOperation<D, O, F>,
        basis: AdmittedBasisCapability<L>,
        authorities: WorthQueryBoundAuthoritySet,
        consumer_support_profile: WorthQueryConsumerSupportProfile,
        providers: WorthQueryBoundRuntimeProviders,
    ) -> Self {
        let required_domain_authority_identities = authorities
            .required_domains
            .iter()
            .map(|binding| {
                format!(
                    "{}:{}",
                    binding.role,
                    binding.authority.authority_identity().as_str()
                )
            })
            .collect::<Vec<_>>();
        let graph_authority_identities = authorities
            .graph_participations
            .iter()
            .map(|participation| {
                format!(
                    "{}:{}",
                    participation.role, participation.record.authority_identity
                )
            })
            .collect::<Vec<_>>();
        let capability_identity = NEXT_BOUND_CAPABILITY_IDENTITY.fetch_add(1, Ordering::Relaxed);
        let conditional_lowering_identities = providers
            .conditional_nodes
            .iter()
            .map(|node| node.lowering.identity().as_str())
            .collect::<Vec<_>>();
        let binding_identity = crate::identity::hash_parts(&[
            "worth_query_bound_operation_v1".into(),
            format!("operation:{}", operation.definition().canonical_identity()),
            format!("basis:{}", basis.capability_digest()),
            format!(
                "domain_authority:{}",
                operation.domain_authority().authority_identity().as_str()
            ),
            format!("graph_authorities:{}", graph_authority_identities.join(",")),
            format!(
                "required_domains:{}",
                required_domain_authority_identities.join(",")
            ),
            format!(
                "conditional_lowerings:{}",
                conditional_lowering_identities.join(",")
            ),
        ]);
        let authority_proof = mint_operation_phase_proof(
            binding_identity.clone(),
            None,
            WorthQueryOperationAuthorityBasis {
                runtime_authority: operation.domain_authority().runtime_authority().as_u64(),
                installation_generation: operation.installation_generation().ordinal(),
                domain_authority_identity: operation
                    .domain_authority()
                    .authority_identity()
                    .as_str()
                    .into(),
                operation_identity: operation.definition().canonical_identity().into(),
                binding_identity: binding_identity.clone(),
                capability_identity,
                basis_identity: basis.capability_digest().into(),
                graph_authority_identities,
                required_domain_authority_identities,
            },
        );
        Self {
            operation,
            basis,
            graph_participations: authorities.graph_participations,
            required_domains: authorities.required_domains,
            commit_posture: authorities.commit_posture,
            binding_identity,
            capability_identity,
            authority_proof,
            _authority_shape_proofs: authorities.shape_proofs,
            consumer_support_profile,
            consumer_contract_minted: Cell::new(false),
            executor: providers.executor,
            workflow_executor: providers.workflow_executor,
            workflow_parallel_admission_provider: providers.workflow_parallel_admission_provider,
            conditional_nodes: providers.conditional_nodes,
            _marker: PhantomData,
        }
    }

    pub fn definition(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryPortableDomainOperationDefinition {
        self.operation.definition()
    }

    pub fn basis(&self) -> &AdmittedBasisCapability<L> {
        &self.basis
    }

    pub fn commit_posture(&self) -> WorthQueryBoundCommitPosture {
        self.commit_posture
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn graph_roles(&self) -> impl ExactSizeIterator<Item = &str> {
        self.graph_participations
            .iter()
            .map(|participation| participation.role.as_str())
    }

    pub fn required_domain_roles(&self) -> impl ExactSizeIterator<Item = &str> {
        self.required_domains
            .iter()
            .map(|binding| binding.role.as_str())
    }

    pub(crate) fn operation(&self) -> &WorthQueryInstalledDomainOperation<D, O, F> {
        &self.operation
    }

    pub(crate) fn installation_is_current(&self) -> bool {
        self.operation
            .domain_authority()
            .is_current_installation_generation()
    }

    pub(crate) fn capability_identity(&self) -> u64 {
        self.capability_identity
    }

    pub(crate) fn authority_proof(
        &self,
    ) -> &WorthQueryOperationPhaseProof<WorthQueryBoundOperationPhase> {
        &self.authority_proof
    }

    pub(crate) fn graph_participations(&self) -> &[WorthQueryBoundGraphParticipation] {
        &self.graph_participations
    }

    pub(crate) fn executor(&self) -> Option<&Arc<WorthQueryInstalledDomainOperationExecutor>> {
        self.executor.as_ref()
    }

    pub(crate) fn workflow_executor(
        &self,
    ) -> Option<&Arc<super::super::WorthQueryInstalledWorkflowStageExecutor>> {
        self.workflow_executor.as_ref()
    }

    pub(crate) fn workflow_parallel_admission_provider(
        &self,
    ) -> Option<&Arc<super::super::WorthQueryInstalledWorkflowParallelAdmissionProvider>> {
        self.workflow_parallel_admission_provider.as_ref()
    }

    pub(crate) fn conditional_nodes(
        &self,
    ) -> &[Arc<super::super::WorthQueryInstalledConditionalNode>] {
        &self.conditional_nodes
    }

    pub(crate) fn conditional_graph_authorities(
        &self,
    ) -> Vec<(
        &str,
        &Arc<worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority>,
    )> {
        self.graph_participations
            .iter()
            .map(|participation| {
                (
                    participation.role.as_str(),
                    &participation.record.installation_authority,
                )
            })
            .collect()
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryBoundDomainOperation<D, O, F, L>
where
    O: WorthQueryExecutableDomainOperation<D, F, Publication = WorthQueryPublishingOperation>,
{
    pub fn consumer_projection_contract(
        &self,
    ) -> Result<
        WorthQueryConsumerProjectionContract<D, O, F, L>,
        WorthQueryConsumerProjectionContractDenial,
    > {
        if !self.installation_is_current() {
            return Err(WorthQueryConsumerProjectionContractDenial::StaleInstallationGeneration);
        }
        if self.consumer_contract_minted.replace(true) {
            return Err(WorthQueryConsumerProjectionContractDenial::AlreadyMinted);
        }
        WorthQueryConsumerProjectionContract::mint(self, &self.consumer_support_profile)
            .map_err(Into::into)
    }
}
