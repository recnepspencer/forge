use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_planar_overlap::CoplanarOverlapContractQueryDomain;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_projection::ProjectPointToCertifiedPlane2DQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::{
    CertifiedSignedArea2DContracts, CertifiedSignedArea2DQueryDomain,
};
use crate::bindings::query_native_planar_winding::{
    CertifiedPolygonWinding2DContracts, CertifiedPolygonWinding2DQueryDomain,
};
use crate::planar_contracts::coplanar_overlap_contract::CoplanarOverlapContractReceipt;
use crate::planar_contracts::local_frame::PlanarLocalFrameCertificateReceipt;
use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;
use crate::workload_platform::certification_context::WorkloadCertificationContext;

use super::certified_pair::CertifiedProjectedOverlapCandidatePair;
use super::ProjectedOverlapFaceDenial;

pub(crate) struct ProjectedOverlapExtractionContracts<'a, SC, PC, PRC, WC, AC>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
{
    pub projection_handle: &'a ForgeQueryAdmittedConfiguredDomainHandle<
        ProjectPointToCertifiedPlane2DQueryDomain,
        PRC,
    >,
    pub winding_contracts: &'a CertifiedPolygonWinding2DContracts<WC, SC, PC>,
    pub signed_area_contracts: &'a CertifiedSignedArea2DContracts<AC>,
    pub precision_receipt: &'a PlanarPrecisionCertificateReceipt,
    pub local_frame_receipt: &'a PlanarLocalFrameCertificateReceipt,
    pub planar_neighborhood_identity: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoplanarOverlapExtractionBundle {
    extraction_bundle_digest: String,
    context_identity: String,
    projection_stage_identity: String,
    movement_rotation_posture_identity: String,
    receipts: Vec<CoplanarOverlapContractReceipt>,
    candidate_pair_count: usize,
}

impl CoplanarOverlapExtractionBundle {
    pub fn from_context_candidate_pairs<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        pairs: super::certified_set::CertifiedProjectedOverlapCandidatePairs<'_>,
        context: &WorkloadCertificationContext<'_, OC, SC, PC, PRC, WC, AC, PXC, FC>,
    ) -> Result<Self, ProjectedOverlapFaceDenial>
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
        PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
        WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
        AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
        PXC: ForgeQueryDomainOperatingContext<
            crate::bindings::query_native_planar_precision::PlanarPrecisionCertificationQueryDomain,
        >,
        FC: ForgeQueryDomainOperatingContext<
            crate::bindings::query_native_planar_local_frame::PlanarLocalFrameCertificateQueryDomain,
        >,
    {
        let pairs = pairs.as_slice();
        if pairs.is_empty() {
            return Err(ProjectedOverlapFaceDenial::new(
                "certified projected overlap extraction requires at least one candidate face pair",
            ));
        }
        require_candidate_pairs_match_context(pairs, context)?;
        let mut receipts = Vec::with_capacity(pairs.len());
        for pair in pairs {
            receipts.push(pair.extract_overlap(
                context.overlap_contracts(),
                context.topology_neighborhood_identity(),
            )?);
        }
        let extraction_bundle_digest = extraction_bundle_digest(
            context.context_identity(),
            context.projection_stage_identity(),
            context.movement_rotation_posture_identity(),
            &receipts,
        );
        Ok(Self {
            extraction_bundle_digest,
            context_identity: context.context_identity().to_string(),
            projection_stage_identity: context.projection_stage_identity().to_string(),
            movement_rotation_posture_identity: context
                .movement_rotation_posture_identity()
                .to_string(),
            receipts,
            candidate_pair_count: pairs.len(),
        })
    }

    pub fn context_identity(&self) -> &str {
        &self.context_identity
    }

    pub fn extraction_bundle_digest(&self) -> &str {
        &self.extraction_bundle_digest
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn receipts(&self) -> &[CoplanarOverlapContractReceipt] {
        &self.receipts
    }

    pub fn candidate_pair_count(&self) -> usize {
        self.candidate_pair_count
    }
}

fn require_candidate_pairs_match_context<OC, SC, PC, PRC, WC, AC, PXC, FC>(
    pairs: &[CertifiedProjectedOverlapCandidatePair],
    context: &WorkloadCertificationContext<'_, OC, SC, PC, PRC, WC, AC, PXC, FC>,
) -> Result<(), ProjectedOverlapFaceDenial>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
    PXC: ForgeQueryDomainOperatingContext<
        crate::bindings::query_native_planar_precision::PlanarPrecisionCertificationQueryDomain,
    >,
    FC: ForgeQueryDomainOperatingContext<
        crate::bindings::query_native_planar_local_frame::PlanarLocalFrameCertificateQueryDomain,
    >,
{
    for pair in pairs {
        if pair.projection_stage_identity() != context.projection_stage_identity() {
            return Err(ProjectedOverlapFaceDenial::new(
                "certified projected overlap extraction requires candidate pairs from the same projection stage as the certification context",
            ));
        }
        if pair.first_face().movement_rotation_posture_identity()
            != context.movement_rotation_posture_identity()
            || pair.second_face().movement_rotation_posture_identity()
                != context.movement_rotation_posture_identity()
        {
            return Err(ProjectedOverlapFaceDenial::new(
                "certified projected overlap extraction requires candidate pairs from the same movement and rotation posture as the certification context",
            ));
        }
    }
    Ok(())
}

fn extraction_bundle_digest(
    context_identity: &str,
    projection_stage_identity: &str,
    movement_rotation_posture_identity: &str,
    receipts: &[CoplanarOverlapContractReceipt],
) -> String {
    let mut parts = vec![
        "coplanar-overlap-extraction-bundle".to_string(),
        format!("context:{context_identity}"),
        format!("projection:{projection_stage_identity}"),
        format!("motion:{movement_rotation_posture_identity}"),
        format!("receipts:{}", receipts.len()),
    ];
    parts.extend(
        receipts
            .iter()
            .map(|receipt| format!("overlap:{}", receipt.fact_digest())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn contracts_from_context<'c, 'a, OC, SC, PC, PRC, WC, AC, PXC, FC>(
    context: &'c WorkloadCertificationContext<'a, OC, SC, PC, PRC, WC, AC, PXC, FC>,
) -> ProjectedOverlapExtractionContracts<'c, SC, PC, PRC, WC, AC>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
    PXC: ForgeQueryDomainOperatingContext<
        crate::bindings::query_native_planar_precision::PlanarPrecisionCertificationQueryDomain,
    >,
    FC: ForgeQueryDomainOperatingContext<
        crate::bindings::query_native_planar_local_frame::PlanarLocalFrameCertificateQueryDomain,
    >,
{
    ProjectedOverlapExtractionContracts {
        projection_handle: context.projection_handle(),
        winding_contracts: context.winding_contracts(),
        signed_area_contracts: context.signed_area_contracts(),
        precision_receipt: context.precision_receipt(),
        local_frame_receipt: context.local_frame_receipt(),
        planar_neighborhood_identity: context.topology_neighborhood_identity(),
    }
}
