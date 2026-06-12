use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_signed_area::authoring::{
    certified_signed_area_2d_entry, CertifiedSignedArea2DCase, CertifiedSignedArea2DEntry,
};
use crate::bindings::query_native_planar_signed_area::domain::CertifiedSignedArea2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::facts::{
    certified_signed_area_2d_facts, CertifiedSignedArea2DFactError,
};
use crate::planar_contracts::polygon_winding_2d::CertifiedPolygonWinding2DReceipt;
use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;
use crate::planar_contracts::signed_area_2d::{
    AreaDegeneracyPolicy, CertifiedSignedArea2DBasis, CertifiedSignedArea2DDenial,
    CertifiedSignedArea2DReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSignedArea2D {
    winding_receipt: CertifiedPolygonWinding2DReceipt,
    precision_receipt: Option<PlanarPrecisionCertificateReceipt>,
    degeneracy_policy: AreaDegeneracyPolicy,
}

impl CertifiedSignedArea2D {
    pub fn measure_face(winding_receipt: CertifiedPolygonWinding2DReceipt) -> Self {
        Self {
            winding_receipt,
            precision_receipt: None,
            degeneracy_policy: AreaDegeneracyPolicy::ClassifyWithoutRepair,
        }
    }

    pub fn using_precision_basis(mut self, receipt: PlanarPrecisionCertificateReceipt) -> Self {
        self.precision_receipt = Some(receipt);
        self
    }

    pub fn classifying_degeneracy(mut self, policy: AreaDegeneracyPolicy) -> Self {
        self.degeneracy_policy = policy;
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a CertifiedSignedArea2DContracts<WC>,
    ) -> Result<CertifiedSignedArea2DPlan<'a, WC>, CertifiedSignedArea2DDenial>
    where
        WC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
    {
        let basis = CertifiedSignedArea2DBasis::new(
            self.winding_receipt,
            self.precision_receipt.ok_or_else(|| {
                CertifiedSignedArea2DDenial::new(
                    crate::planar_contracts::signed_area_2d::CertifiedSignedArea2DDenialKind::MissingPrecisionReceipt,
                    "certified signed area requires an explicit precision receipt",
                )
            })?,
            self.degeneracy_policy,
        )?;
        let entry = certified_signed_area_2d_entry(
            CertifiedSignedArea2DCase::from_certified_planar_basis(basis),
        );
        Ok(CertifiedSignedArea2DPlan { entry, contracts })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSignedArea2DContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
{
    signed_area_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<CertifiedSignedArea2DQueryDomain, WC>,
}

impl<WC> CertifiedSignedArea2DContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
{
    pub fn new(
        signed_area_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            CertifiedSignedArea2DQueryDomain,
            WC,
        >,
    ) -> Self {
        Self { signed_area_handle }
    }
}

pub struct CertifiedSignedArea2DPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
{
    entry: CertifiedSignedArea2DEntry,
    contracts: &'a CertifiedSignedArea2DContracts<WC>,
}

impl<WC> CertifiedSignedArea2DPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
{
    pub fn loop_edges_walked(&self) -> usize {
        self.entry
            .case()
            .basis()
            .loops()
            .iter()
            .map(|loop_summary| loop_summary.vertices().len())
            .sum()
    }

    pub fn local_scale_comparisons_required(&self) -> usize {
        3
    }

    pub fn certify(self) -> Result<CertifiedSignedArea2DReceipt, CertifiedSignedArea2DFactError> {
        certified_signed_area_2d_facts(&self.entry, &self.contracts.signed_area_handle)
    }
}
