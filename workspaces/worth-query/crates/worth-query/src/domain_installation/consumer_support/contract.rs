use std::marker::PhantomData;

use crate::basis_lifecycle::{AdmittedBasisCapability, BasisOperationLane};

use super::{
    WorthQueryConsumerSupportCompatibilityDenial, WorthQueryConsumerSupportDimension,
    WorthQueryConsumerSupportPosture, WorthQueryConsumerSupportProfile,
};
use crate::domain_installation::{
    WorthQueryBoundDomainOperation, WorthQueryInstalledAftermathContract,
    WorthQueryOperationCollectionContract, WorthQueryOperationLineageContract,
    WorthQueryOperationNativeProjectionContract, WorthQueryOperationPromotionContract,
    WorthQueryOperationPublicationContract, WorthQueryOperationReplayContract,
    WorthQueryOperationSupportRequirements, WorthQueryOperationTerminalContract,
    WorthQuerySupportRequirement,
};

type ConsumerContractMarker<D, O, F, L> = fn() -> (D, O, F, L);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryConsumerSupportAdmissionCounters {
    pub installation_generation_checks: usize,
    pub mint_guard_checks: usize,
    pub dimensions_evaluated: usize,
    pub reporting_digest_comparisons: usize,
    pub downstream_hook_inspections: usize,
}

pub struct WorthQueryConsumerProjectionContract<D, O, F, L: BasisOperationLane> {
    binding_identity: String,
    capability_identity: u64,
    basis_identity: String,
    basis: AdmittedBasisCapability<L>,
    installation_generation: super::super::WorthQueryDomainInstallationGeneration,
    domain_authority: std::sync::Arc<super::super::WorthQueryInstalledDomainAuthority>,
    operation_identity: worth_query_installation::facade::WorthQueryDomainOperationIdentity,
    canonical_operation_identity: String,
    canonical_projection: crate::canonicalization::CanonicalQueryBundle,
    native_projection: WorthQueryOperationNativeProjectionContract,
    collection: WorthQueryOperationCollectionContract,
    replay: WorthQueryOperationReplayContract,
    aftermath: Option<WorthQueryInstalledAftermathContract>,
    lineage: WorthQueryOperationLineageContract,
    promotion: WorthQueryOperationPromotionContract,
    publication: WorthQueryOperationPublicationContract,
    projection_consumption:
        crate::domain_installation::WorthQueryOperationProjectionConsumptionContract,
    terminal: WorthQueryOperationTerminalContract,
    conditional_nodes:
        Vec<worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration>,
    requirements: WorthQueryOperationSupportRequirements,
    postures: [WorthQueryConsumerSupportPosture; WorthQueryConsumerSupportDimension::COUNT],
    counters: WorthQueryConsumerSupportAdmissionCounters,
    _marker: PhantomData<ConsumerContractMarker<D, O, F, L>>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryConsumerProjectionContract<D, O, F, L> {
    pub(crate) fn shares_execution_projection_with(&self, candidate: &Self) -> bool {
        self.canonical_operation_identity == candidate.canonical_operation_identity
            && self.canonical_projection.query() == candidate.canonical_projection.query()
            && self.canonical_projection.result_shape()
                == candidate.canonical_projection.result_shape()
            && self.native_projection == candidate.native_projection
            && self.collection == candidate.collection
            && self.projection_consumption == candidate.projection_consumption
            && self.requirements == candidate.requirements
            && self.postures == candidate.postures
    }

    pub(crate) fn mint(
        bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
        profile: &WorthQueryConsumerSupportProfile,
        mut counters: WorthQueryConsumerSupportAdmissionCounters,
    ) -> Result<Self, WorthQueryConsumerSupportCompatibilityDenial> {
        let requirements = bound.definition().semantics().support;
        let mut postures = [WorthQueryConsumerSupportPosture::Unsupported;
            WorthQueryConsumerSupportDimension::COUNT];
        for dimension in WorthQueryConsumerSupportDimension::ALL {
            counters.dimensions_evaluated += 1;
            let posture = profile.posture(dimension);
            postures[dimension.index()] = posture;
            if requirement(requirements, dimension) == WorthQuerySupportRequirement::Required
                && posture != WorthQueryConsumerSupportPosture::Supported
            {
                return Err(WorthQueryConsumerSupportCompatibilityDenial::new(
                    dimension, posture, counters,
                ));
            }
        }
        Ok(Self {
            binding_identity: bound.binding_identity().to_string(),
            capability_identity: bound.capability_identity(),
            basis_identity: bound.basis().capability_digest().to_string(),
            basis: bound.basis().clone(),
            installation_generation: bound.operation().installation_generation(),
            domain_authority: std::sync::Arc::clone(bound.operation().domain_authority()),
            operation_identity: bound.definition().identity().clone(),
            canonical_operation_identity: bound.definition().canonical_identity().into(),
            canonical_projection: bound.definition().semantics().canonical_query.clone(),
            native_projection: bound.definition().semantics().native_projection.clone(),
            collection: bound.definition().semantics().collection.clone(),
            replay: bound.definition().semantics().replay.clone(),
            aftermath: bound.definition().semantics().aftermath.clone(),
            lineage: bound.definition().semantics().lineage,
            promotion: bound.definition().semantics().promotion,
            publication: bound.definition().semantics().publication.clone(),
            projection_consumption: bound.definition().semantics().projection_consumption,
            terminal: bound.definition().semantics().terminal.clone(),
            conditional_nodes: bound.definition().semantics().conditional_nodes.clone(),
            requirements,
            postures,
            counters,
            _marker: PhantomData,
        })
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub(crate) fn binds_capability(&self, identity: u64) -> bool {
        self.capability_identity == identity
    }

    pub(crate) fn capability_identity(&self) -> u64 {
        self.capability_identity
    }

    pub(crate) fn runtime_authority(&self) -> u64 {
        self.domain_authority.runtime_authority().as_u64()
    }

    pub(crate) fn domain_authority(
        &self,
    ) -> &std::sync::Arc<super::super::WorthQueryInstalledDomainAuthority> {
        &self.domain_authority
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn basis(&self) -> &AdmittedBasisCapability<L> {
        &self.basis
    }

    pub fn installation_generation(&self) -> super::super::WorthQueryDomainInstallationGeneration {
        self.installation_generation
    }

    pub fn operation_identity(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryDomainOperationIdentity {
        &self.operation_identity
    }

    pub fn native_projection(&self) -> &WorthQueryOperationNativeProjectionContract {
        &self.native_projection
    }

    pub fn canonical_operation_identity(&self) -> &str {
        &self.canonical_operation_identity
    }

    pub fn canonical_projection(&self) -> &crate::canonicalization::CanonicalQueryBundle {
        &self.canonical_projection
    }

    pub fn collection(&self) -> &WorthQueryOperationCollectionContract {
        &self.collection
    }

    pub fn replay(&self) -> &WorthQueryOperationReplayContract {
        &self.replay
    }

    pub fn aftermath(&self) -> Option<&WorthQueryInstalledAftermathContract> {
        self.aftermath.as_ref()
    }

    pub fn lineage(&self) -> WorthQueryOperationLineageContract {
        self.lineage
    }

    pub fn promotion(&self) -> &WorthQueryOperationPromotionContract {
        &self.promotion
    }

    pub fn publication(&self) -> &WorthQueryOperationPublicationContract {
        &self.publication
    }

    pub fn projection_consumption(
        &self,
    ) -> crate::domain_installation::WorthQueryOperationProjectionConsumptionContract {
        self.projection_consumption
    }

    pub fn terminal(&self) -> &WorthQueryOperationTerminalContract {
        &self.terminal
    }

    pub fn conditional_nodes(
        &self,
    ) -> &[worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration] {
        &self.conditional_nodes
    }

    pub fn requirement(
        &self,
        dimension: WorthQueryConsumerSupportDimension,
    ) -> WorthQuerySupportRequirement {
        requirement(self.requirements, dimension)
    }

    pub fn support_posture(
        &self,
        dimension: WorthQueryConsumerSupportDimension,
    ) -> WorthQueryConsumerSupportPosture {
        self.postures[dimension.index()]
    }

    pub fn counters(&self) -> WorthQueryConsumerSupportAdmissionCounters {
        self.counters
    }

    pub fn foundational_support_projection(
        &self,
    ) -> Result<
        worth_foundational::facade::SupportProfiledArtifact<
            super::WorthQueryFoundationalConsumerSupportProjection,
        >,
        super::WorthQueryFoundationalSupportExportDenial,
    > {
        super::materialize_foundational_support_projection(
            &self.binding_identity,
            &self.basis_identity,
            self.installation_generation,
            self.domain_authority.is_current_installation_generation(),
            self.requirements,
            self.postures,
        )
    }
}

pub(super) fn requirement(
    requirements: WorthQueryOperationSupportRequirements,
    dimension: WorthQueryConsumerSupportDimension,
) -> WorthQuerySupportRequirement {
    match dimension {
        WorthQueryConsumerSupportDimension::Basis => WorthQuerySupportRequirement::Required,
        WorthQueryConsumerSupportDimension::Live => requirements.live,
        WorthQueryConsumerSupportDimension::Continuation => requirements.continuation,
        WorthQueryConsumerSupportDimension::AsyncResultState => requirements.async_result_state,
        WorthQueryConsumerSupportDimension::Recovery => requirements.recovery,
        WorthQueryConsumerSupportDimension::Inspection => requirements.inspection,
        WorthQueryConsumerSupportDimension::ProjectionConsumption => {
            requirements.projection_consumption
        }
        WorthQueryConsumerSupportDimension::DependencyImpact => requirements.dependency_impact,
        WorthQueryConsumerSupportDimension::Sharing => requirements.sharing,
        WorthQueryConsumerSupportDimension::Invalidation => requirements.invalidation,
        WorthQueryConsumerSupportDimension::CollectionDelivery => requirements.collection_delivery,
        WorthQueryConsumerSupportDimension::ConditionalEvaluation => {
            requirements.conditional_evaluation
        }
        WorthQueryConsumerSupportDimension::ConditionalComparator => {
            requirements.conditional_comparator
        }
        WorthQueryConsumerSupportDimension::ConditionalTrigger => requirements.conditional_trigger,
        WorthQueryConsumerSupportDimension::ConditionalTemporalOrOnDemand => {
            requirements.conditional_temporal_or_on_demand
        }
    }
}
