use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_overlap::authoring::{
    coplanar_overlap_contract_entry, CoplanarOverlapContractCase, CoplanarOverlapContractEntry,
};
use crate::bindings::query_native_planar_overlap::domain::CoplanarOverlapContractQueryDomain;
use crate::bindings::query_native_planar_overlap::facts::{
    coplanar_overlap_contract_facts, CoplanarOverlapContractFactError,
};
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_segment_segment::{
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DQueryDomain,
};
use crate::planar_contracts::coplanar_overlap_contract::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractBasis, CoplanarOverlapContractReceipt,
    CoplanarOverlapDenial, CoplanarOverlapPolicy,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CoplanarOverlapContractExtractor {
    first_face: CertifiedCoplanarOverlapFace2D,
    second_face: CertifiedCoplanarOverlapFace2D,
    planar_neighborhood_identity: String,
    policy: CoplanarOverlapPolicy,
}

impl CoplanarOverlapContractExtractor {
    pub fn between(
        first_face: CertifiedCoplanarOverlapFace2D,
        second_face: CertifiedCoplanarOverlapFace2D,
    ) -> Self {
        Self {
            first_face,
            second_face,
            planar_neighborhood_identity: String::new(),
            policy: CoplanarOverlapPolicy::ExtractContractsOnly,
        }
    }

    pub fn within_planar_neighborhood(mut self, identity: impl Into<String>) -> Self {
        self.planar_neighborhood_identity = identity.into();
        self
    }

    pub fn with_policy(mut self, policy: CoplanarOverlapPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn compile<'a, OC, SC, PC>(
        self,
        contracts: &'a CoplanarOverlapContractContracts<OC, SC, PC>,
    ) -> Result<CoplanarOverlapContractPlan<'a, OC, SC, PC>, CoplanarOverlapDenial>
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    {
        let basis = CoplanarOverlapContractBasis::new(
            self.first_face,
            self.second_face,
            self.planar_neighborhood_identity,
            self.policy,
        )?;
        let entry = coplanar_overlap_contract_entry(
            CoplanarOverlapContractCase::from_certified_face_pair(basis),
        );
        Ok(CoplanarOverlapContractPlan { entry, contracts })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoplanarOverlapContractContracts<OC, SC, PC>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    overlap_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<CoplanarOverlapContractQueryDomain, OC>,
    segment_contracts: CertifiedSegmentSegment2DContracts<SC, PC>,
}

impl<OC, SC, PC> CoplanarOverlapContractContracts<OC, SC, PC>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    pub fn new(
        overlap_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            CoplanarOverlapContractQueryDomain,
            OC,
        >,
        segment_contracts: CertifiedSegmentSegment2DContracts<SC, PC>,
    ) -> Self {
        Self {
            overlap_handle,
            segment_contracts,
        }
    }
}

pub struct CoplanarOverlapContractPlan<'a, OC, SC, PC>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    entry: CoplanarOverlapContractEntry,
    contracts: &'a CoplanarOverlapContractContracts<OC, SC, PC>,
}

impl<OC, SC, PC> CoplanarOverlapContractPlan<'_, OC, SC, PC>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    pub fn candidate_pair_breadth(&self) -> usize {
        let first = self
            .entry
            .case()
            .basis()
            .first_face()
            .signed_area_receipt()
            .basis()
            .loops()
            .iter()
            .map(|loop_summary| loop_summary.vertices().len())
            .sum::<usize>();
        let second = self
            .entry
            .case()
            .basis()
            .second_face()
            .signed_area_receipt()
            .basis()
            .loops()
            .iter()
            .map(|loop_summary| loop_summary.vertices().len())
            .sum::<usize>();
        first * second
    }

    pub fn topology_mutations(&self) -> usize {
        0
    }

    pub fn boolean_classifications(&self) -> usize {
        0
    }

    pub fn extract(
        self,
    ) -> Result<CoplanarOverlapContractReceipt, CoplanarOverlapContractFactError> {
        coplanar_overlap_contract_facts(
            &self.entry,
            &self.contracts.overlap_handle,
            &self.contracts.segment_contracts,
        )
    }
}
