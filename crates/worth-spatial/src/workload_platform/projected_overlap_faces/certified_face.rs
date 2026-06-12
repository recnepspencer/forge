use forge_query::facade::ForgeQueryDomainOperatingContext;

use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_projection::{
    project_point_to_certified_plane_2d_entry, project_point_to_certified_plane_2d_facts,
    ProjectPointToCertifiedPlane2DCase, ProjectPointToCertifiedPlane2DQueryDomain,
};
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::{
    CertifiedSignedArea2D, CertifiedSignedArea2DQueryDomain,
};
use crate::bindings::query_native_planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DQueryDomain, CertifiedProjectedLoop2D,
};
use crate::planar_contracts::coplanar_overlap_contract::CertifiedCoplanarOverlapFace2D;
use crate::planar_contracts::polygon_winding_2d::CertifiedTopologyLoopBasis2D;
use crate::planar_contracts::projection_2d::{
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DReceipt,
};
use crate::planar_contracts::signed_area_2d::AreaDegeneracyPolicy;
use crate::workload_platform::certification_context::WorkloadCertificationContext;

use super::bundle::{contracts_from_context, ProjectedOverlapExtractionContracts};
use super::face_set::ProjectedOverlapFaceGeometry;
use super::ProjectedOverlapFaceDenial;

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedProjectedOverlapFace {
    projection_stage_identity: String,
    face_identity: String,
    projected_face_identity: String,
    loop_identity: String,
    projected_loop_identity: String,
    source_geometry: ProjectedOverlapFaceGeometry,
    certified_face: CertifiedCoplanarOverlapFace2D,
}

impl CertifiedProjectedOverlapFace {
    pub(crate) fn from_projected_geometry<SC, PC, PRC, WC, AC>(
        projection_stage_identity: impl Into<String>,
        source_geometry: ProjectedOverlapFaceGeometry,
        contracts: &ProjectedOverlapExtractionContracts<'_, SC, PC, PRC, WC, AC>,
    ) -> Result<Self, ProjectedOverlapFaceDenial>
    where
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
        PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
        WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
        AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
    {
        let certified_face = certify_overlap_face(&source_geometry, contracts)?;
        Ok(Self {
            projection_stage_identity: projection_stage_identity.into(),
            face_identity: source_geometry.face_identity.clone(),
            projected_face_identity: source_geometry.projected_face_identity.clone(),
            loop_identity: source_geometry.loop_identity.clone(),
            projected_loop_identity: source_geometry.projected_loop_identity.clone(),
            source_geometry,
            certified_face,
        })
    }

    pub fn recertify_with_context<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        &self,
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
        if self.projection_stage_identity != context.projection_stage_identity() {
            return Err(ProjectedOverlapFaceDenial::new(
                "certified projected overlap face recertification requires the same projection stage as the certification context",
            ));
        }
        let contracts = contracts_from_context(context);
        Self::from_projected_geometry(
            self.projection_stage_identity.clone(),
            self.source_geometry.clone(),
            &contracts,
        )
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn face_identity(&self) -> &str {
        &self.face_identity
    }

    pub fn projected_face_identity(&self) -> &str {
        &self.projected_face_identity
    }

    pub fn loop_identity(&self) -> &str {
        &self.loop_identity
    }

    pub fn projected_loop_identity(&self) -> &str {
        &self.projected_loop_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        self.certified_face
            .signed_area_receipt()
            .basis()
            .precision_receipt()
            .basis()
            .movement_rotation_posture_identity()
    }

    pub fn local_frame_identity(&self) -> &str {
        self.certified_face
            .signed_area_receipt()
            .basis()
            .frame_identity()
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        self.certified_face
            .signed_area_receipt()
            .basis()
            .winding_receipt()
            .basis()
            .local_frame_fact_digest()
    }

    pub fn precision_fact_digest(&self) -> &str {
        self.certified_face
            .signed_area_receipt()
            .basis()
            .precision_receipt()
            .fact_digest()
    }

    pub fn winding_fact_digest(&self) -> &str {
        self.certified_face
            .signed_area_receipt()
            .basis()
            .winding_receipt()
            .fact_digest()
    }

    pub fn signed_area_fact_digest(&self) -> &str {
        self.certified_face.signed_area_receipt().fact_digest()
    }

    pub(crate) fn overlap_face(&self) -> CertifiedCoplanarOverlapFace2D {
        self.certified_face.clone()
    }
}

fn certify_overlap_face<SC, PC, PRC, WC, AC>(
    face: &ProjectedOverlapFaceGeometry,
    contracts: &ProjectedOverlapExtractionContracts<'_, SC, PC, PRC, WC, AC>,
) -> Result<CertifiedCoplanarOverlapFace2D, ProjectedOverlapFaceDenial>
where
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

fn certified_loop<SC, PC, PRC, WC, AC>(
    face: &ProjectedOverlapFaceGeometry,
    loop_identity: &str,
    points: &[[f64; 2]],
    contracts: &ProjectedOverlapExtractionContracts<'_, SC, PC, PRC, WC, AC>,
) -> Result<CertifiedProjectedLoop2D, ProjectedOverlapFaceDenial>
where
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

fn project_point<SC, PC, PRC, WC, AC>(
    face: &ProjectedOverlapFaceGeometry,
    loop_identity: &str,
    index: usize,
    point: [f64; 2],
    contracts: &ProjectedOverlapExtractionContracts<'_, SC, PC, PRC, WC, AC>,
) -> Result<ProjectPointToCertifiedPlane2DReceipt, ProjectedOverlapFaceDenial>
where
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
use crate::bindings::query_native_planar_overlap::CoplanarOverlapContractQueryDomain;
