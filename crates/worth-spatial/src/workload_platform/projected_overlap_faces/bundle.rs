use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_overlap::{
    CoplanarOverlapContractContracts, CoplanarOverlapContractExtractor,
    CoplanarOverlapContractQueryDomain,
};
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_projection::{
    project_point_to_certified_plane_2d_entry, project_point_to_certified_plane_2d_facts,
    ProjectPointToCertifiedPlane2DCase, ProjectPointToCertifiedPlane2DQueryDomain,
};
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::{
    CertifiedSignedArea2D, CertifiedSignedArea2DContracts, CertifiedSignedArea2DQueryDomain,
};
use crate::bindings::query_native_planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DContracts,
    CertifiedPolygonWinding2DQueryDomain, CertifiedProjectedLoop2D,
};
use crate::planar_contracts::coplanar_overlap_contract::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractReceipt,
};
use crate::planar_contracts::local_frame::PlanarLocalFrameCertificateReceipt;
use crate::planar_contracts::polygon_winding_2d::CertifiedTopologyLoopBasis2D;
use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;
use crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DBasis;
use crate::planar_contracts::signed_area_2d::AreaDegeneracyPolicy;

use super::face_set::{ProjectedOverlapFaceGeometry, ProjectedOverlapFaceSet};
use super::ProjectedOverlapFaceDenial;

pub struct ProjectedOverlapExtractionContracts<'a, OC, SC, PC, PRC, WC, AC>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
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
    pub overlap_contracts: &'a CoplanarOverlapContractContracts<OC, SC, PC>,
    pub precision_receipt: &'a PlanarPrecisionCertificateReceipt,
    pub local_frame_receipt: &'a PlanarLocalFrameCertificateReceipt,
    pub planar_neighborhood_identity: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoplanarOverlapExtractionBundle {
    projection_stage_identity: String,
    receipts: Vec<CoplanarOverlapContractReceipt>,
    candidate_pair_count: usize,
}

impl CoplanarOverlapExtractionBundle {
    pub fn from_projected_faces<OC, SC, PC, PRC, WC, AC>(
        face_set: &ProjectedOverlapFaceSet,
        contracts: ProjectedOverlapExtractionContracts<'_, OC, SC, PC, PRC, WC, AC>,
    ) -> Result<Self, ProjectedOverlapFaceDenial>
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
        PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
        WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
        AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
    {
        let pairs = face_set.candidate_pairs();
        if pairs.is_empty() {
            return Err(ProjectedOverlapFaceDenial::new(
                "projected overlap extraction requires at least one candidate face pair",
            ));
        }
        let mut receipts = Vec::with_capacity(pairs.len());
        for (first, second) in &pairs {
            let first = certify_face(first, &contracts)?;
            let second = certify_face(second, &contracts)?;
            let receipt = CoplanarOverlapContractExtractor::between(first, second)
                .within_planar_neighborhood(contracts.planar_neighborhood_identity)
                .compile(contracts.overlap_contracts)
                .map_err(|error| ProjectedOverlapFaceDenial::new(error.reason().to_string()))?
                .extract()
                .map_err(|error| ProjectedOverlapFaceDenial::new(format!("{error:?}")))?;
            receipts.push(receipt);
        }
        Ok(Self {
            projection_stage_identity: face_set.projection_stage_identity().to_string(),
            receipts,
            candidate_pair_count: pairs.len(),
        })
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn receipts(&self) -> &[CoplanarOverlapContractReceipt] {
        &self.receipts
    }

    pub fn candidate_pair_count(&self) -> usize {
        self.candidate_pair_count
    }
}

fn certify_face<OC, SC, PC, PRC, WC, AC>(
    face: &ProjectedOverlapFaceGeometry,
    contracts: &ProjectedOverlapExtractionContracts<'_, OC, SC, PC, PRC, WC, AC>,
) -> Result<CertifiedCoplanarOverlapFace2D, ProjectedOverlapFaceDenial>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
{
    let outer = certified_loop(face, &face.loop_identity, &face.outer_points, contracts)?;
    let mut winding = CertifiedPolygonWinding2D::certify(outer)
        .within_planar_neighborhood(contracts.planar_neighborhood_identity);
    if let Some(candidate_points) = &face.containment_candidate_points {
        winding = winding.with_containment_candidate(certified_loop(
            face,
            &format!("{}:candidate", face.loop_identity),
            candidate_points,
            contracts,
        )?);
    }
    let winding = winding
        .compile(contracts.winding_contracts)
        .map_err(|error| ProjectedOverlapFaceDenial::new(error.reason().to_string()))?
        .certify()
        .map_err(|error| ProjectedOverlapFaceDenial::new(format!("{error:?}")))?;
    let signed_area = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(contracts.precision_receipt.clone())
        .classifying_degeneracy(AreaDegeneracyPolicy::ClassifyWithoutRepair)
        .compile(contracts.signed_area_contracts)
        .map_err(|error| ProjectedOverlapFaceDenial::new(error.reason().to_string()))?
        .certify()
        .map_err(|error| ProjectedOverlapFaceDenial::new(format!("{error:?}")))?;
    CertifiedCoplanarOverlapFace2D::from_certified_area(
        format!("{}:{}", face.face_identity, face.projected_face_identity),
        signed_area,
    )
    .map_err(|error| ProjectedOverlapFaceDenial::new(error.reason().to_string()))
}

fn certified_loop<OC, SC, PC, PRC, WC, AC>(
    face: &ProjectedOverlapFaceGeometry,
    loop_identity: &str,
    points: &[[f64; 2]],
    contracts: &ProjectedOverlapExtractionContracts<'_, OC, SC, PC, PRC, WC, AC>,
) -> Result<CertifiedProjectedLoop2D, ProjectedOverlapFaceDenial>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
{
    CertifiedProjectedLoop2D::from_projected_vertices(
        format!("{}:{}", loop_identity, face.projected_loop_identity),
        CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
            loop_identity,
            format!("membership:{}", face.projected_loop_identity),
            &face.projected_face_identity,
        ),
        points
            .iter()
            .enumerate()
            .map(|(index, point)| project_point(face, loop_identity, index, *point, contracts))
            .collect::<Result<Vec<_>, _>>()?,
    )
    .map_err(|error| ProjectedOverlapFaceDenial::new(error.reason().to_string()))
}

fn project_point<OC, SC, PC, PRC, WC, AC>(
    face: &ProjectedOverlapFaceGeometry,
    loop_identity: &str,
    index: usize,
    point: [f64; 2],
    contracts: &ProjectedOverlapExtractionContracts<'_, OC, SC, PC, PRC, WC, AC>,
) -> Result<
    crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt,
    ProjectedOverlapFaceDenial,
>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
{
    let origin = contracts.local_frame_receipt.basis().origin();
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(format!("{loop_identity}:point:{index}"))
        .source_point([origin[0] + point[0], origin[1] + point[1], origin[2]])
        .source_point_basis_digest(&face.projected_loop_identity)
        .local_delta_from_frame_origin([point[0], point[1], 0.0])
        .local_frame_receipt(contracts.local_frame_receipt)
        .build()
        .map_err(|error| ProjectedOverlapFaceDenial::new(error.reason().to_string()))?;
    project_point_to_certified_plane_2d_facts(
        &project_point_to_certified_plane_2d_entry(
            ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
        ),
        contracts.projection_handle,
    )
    .map_err(|error| ProjectedOverlapFaceDenial::new(format!("{error:?}")))
}
