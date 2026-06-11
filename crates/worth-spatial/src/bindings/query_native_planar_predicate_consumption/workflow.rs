use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_predicate_consumption::authoring::{
    predicate_certificate_consumption_entry, PredicateCertificateConsumptionCase,
    PredicateCertificateConsumptionEntry,
};
use crate::bindings::query_native_planar_predicate_consumption::domain::PredicateCertificateConsumptionQueryDomain;
use crate::bindings::query_native_planar_predicate_consumption::facts::{
    predicate_certificate_consumption_facts, PredicateCertificateConsumptionFactError,
};
use crate::planar_contracts::coplanar_overlap_contract::CoplanarOverlapContractReceipt;
use crate::planar_contracts::polygon_winding_2d::CertifiedPolygonWinding2DReceipt;
use crate::planar_contracts::predicate_authority::PlanarPredicateFactReceipt;
use crate::planar_contracts::predicate_consumption::{
    PredicateCertificateConsumptionBasis, PredicateCertificateConsumptionReceipt,
};
use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DReceipt;
use crate::planar_contracts::signed_area_2d::CertifiedSignedArea2DReceipt;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PredicateCertificateConsumption {
    builder: crate::planar_contracts::predicate_consumption::PredicateCertificateConsumptionBuilder,
}

impl PredicateCertificateConsumption {
    pub fn for_planar_workload() -> Self {
        Self {
            builder: PredicateCertificateConsumptionBasis::builder(),
        }
    }

    pub fn expecting_topology_basis(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.expecting_topology_basis(identity);
        self
    }

    pub fn expecting_movement_rotation_posture(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.expecting_movement_rotation_posture(identity);
        self
    }

    pub fn expecting_local_frame(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.expecting_local_frame(identity);
        self
    }

    pub fn with_predicate_authority<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = PlanarPredicateFactReceipt>,
    {
        self.builder = self.builder.with_predicate_authority(receipts);
        self
    }

    pub fn with_segment_contacts<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = CertifiedSegmentSegment2DReceipt>,
    {
        self.builder = self.builder.with_segment_contacts(receipts);
        self
    }

    pub fn with_polygon_winding(mut self, receipt: CertifiedPolygonWinding2DReceipt) -> Self {
        self.builder = self.builder.with_polygon_winding(receipt);
        self
    }

    pub fn with_signed_area(mut self, receipt: CertifiedSignedArea2DReceipt) -> Self {
        self.builder = self.builder.with_signed_area(receipt);
        self
    }

    pub fn with_coplanar_overlap(mut self, receipt: CoplanarOverlapContractReceipt) -> Self {
        self.builder = self.builder.with_coplanar_overlap(receipt);
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a PredicateCertificateConsumptionContracts<WC>,
    ) -> Result<
        PredicateCertificateConsumptionPlan<'a, WC>,
        crate::planar_contracts::predicate_consumption::PredicateCertificateConsumptionDenial,
    >
    where
        WC: ForgeQueryDomainOperatingContext<PredicateCertificateConsumptionQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry = predicate_certificate_consumption_entry(
            PredicateCertificateConsumptionCase::from_basis(basis),
        );
        Ok(PredicateCertificateConsumptionPlan { entry, contracts })
    }
}

pub struct PredicateCertificateConsumptionContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PredicateCertificateConsumptionQueryDomain>,
{
    predicate_consumption_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PredicateCertificateConsumptionQueryDomain, WC>,
}

impl<WC> PredicateCertificateConsumptionContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PredicateCertificateConsumptionQueryDomain>,
{
    pub fn new(
        predicate_consumption_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PredicateCertificateConsumptionQueryDomain,
            WC,
        >,
    ) -> Self {
        Self {
            predicate_consumption_handle,
        }
    }
}

pub struct PredicateCertificateConsumptionPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PredicateCertificateConsumptionQueryDomain>,
{
    entry: PredicateCertificateConsumptionEntry,
    contracts: &'a PredicateCertificateConsumptionContracts<WC>,
}

impl<WC> PredicateCertificateConsumptionPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PredicateCertificateConsumptionQueryDomain>,
{
    pub fn inspected_predicate_rows(&self) -> usize {
        self.entry.case().basis().consumption_rows().len()
    }

    pub fn certify(
        self,
    ) -> Result<PredicateCertificateConsumptionReceipt, PredicateCertificateConsumptionFactError>
    {
        predicate_certificate_consumption_facts(
            &self.entry,
            &self.contracts.predicate_consumption_handle,
        )
    }
}
