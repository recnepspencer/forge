use forge_query::facade::ForgeQueryDomainOperatingContext;
use std::collections::BTreeMap;

use crate::bindings::query_native_planar_local_frame::PlanarLocalFrameCertificateQueryDomain;
use crate::bindings::query_native_planar_overlap::CoplanarOverlapContractQueryDomain;
use crate::bindings::query_native_planar_precision::PlanarPrecisionCertificationQueryDomain;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_projection::ProjectPointToCertifiedPlane2DQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::CertifiedSignedArea2DQueryDomain;
use crate::bindings::query_native_planar_winding::CertifiedPolygonWinding2DQueryDomain;
use crate::workload_platform::certification_context::WorkloadCertificationContext;
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;

use super::ProjectedOverlapFaceDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectedOverlapCandidatePolicy {
    AdjacentProjectedFacePairs,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedOverlapFaceSet {
    projection_stage_identity: String,
    faces: Vec<ProjectedOverlapFaceGeometry>,
    candidate_policy: ProjectedOverlapCandidatePolicy,
}

impl ProjectedOverlapFaceSet {
    pub fn from_context<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        context: &WorkloadCertificationContext<'_, OC, SC, PC, PRC, WC, AC, PXC, FC>,
    ) -> Result<Self, ProjectedOverlapFaceDenial>
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
        PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
        WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
        AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
        PXC: ForgeQueryDomainOperatingContext<PlanarPrecisionCertificationQueryDomain>,
        FC: ForgeQueryDomainOperatingContext<PlanarLocalFrameCertificateQueryDomain>,
    {
        Self::from_projected_workload(context.projected_workload())
    }

    pub fn from_projected_workload(
        projected: &ProjectedPlanarWorkload,
    ) -> Result<Self, ProjectedOverlapFaceDenial> {
        let projection_stage_identity = projected
            .receipts()
            .stage_identity()
            .receipt_identity()
            .to_string();
        let boundary_loops = boundary_loops_by_owner(projected)?;
        let faces = projected
            .projected_faces()
            .iter()
            .map(|face| face_geometry_from_projected_boundary(&boundary_loops, face))
            .collect::<Result<Vec<_>, _>>()?;
        if faces.is_empty() {
            return Err(ProjectedOverlapFaceDenial::new(
                "projected overlap faces require projected loop boundary geometry from the workload",
            ));
        }
        if faces.len() % 2 != 0 {
            return Err(ProjectedOverlapFaceDenial::new(
                "projected overlap candidate policy requires an even number of boundary-backed faces; one projected face has no adjacent overlap partner",
            ));
        }
        Ok(Self {
            projection_stage_identity,
            faces,
            candidate_policy: ProjectedOverlapCandidatePolicy::AdjacentProjectedFacePairs,
        })
    }

    pub fn with_candidate_policy(mut self, policy: ProjectedOverlapCandidatePolicy) -> Self {
        self.candidate_policy = policy;
        self
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub(crate) fn into_certification_parts(
        self,
    ) -> (
        String,
        Vec<ProjectedOverlapFaceGeometry>,
        ProjectedOverlapCandidatePolicy,
    ) {
        (
            self.projection_stage_identity,
            self.faces,
            self.candidate_policy,
        )
    }
}

fn boundary_loops_by_owner<'a>(
    projected: &'a ProjectedPlanarWorkload,
) -> Result<
    BTreeMap<&'a str, &'a crate::workload_platform::projection_workload::ProjectedLoop>,
    ProjectedOverlapFaceDenial,
> {
    let mut loops_by_owner = BTreeMap::new();
    for loop_entity in projected.projected_loops() {
        let Some(boundary) = loop_entity.boundary() else {
            return Err(ProjectedOverlapFaceDenial::new(
                "projected overlap extraction requires every projected loop to carry boundary geometry",
            ));
        };
        if loops_by_owner
            .insert(boundary.owning_face_identity(), loop_entity)
            .is_some()
        {
            return Err(ProjectedOverlapFaceDenial::new(
                "projected overlap extraction requires one boundary loop per projected face",
            ));
        }
    }
    Ok(loops_by_owner)
}

fn face_geometry_from_projected_boundary(
    boundary_loops: &BTreeMap<&str, &crate::workload_platform::projection_workload::ProjectedLoop>,
    face: &crate::workload_platform::projection_workload::ProjectedFace,
) -> Result<ProjectedOverlapFaceGeometry, ProjectedOverlapFaceDenial> {
    let loop_entity = boundary_loops
        .get(face.identity().topology_entity_identity())
        .copied()
        .ok_or_else(|| {
            ProjectedOverlapFaceDenial::new(
                "projected overlap extraction requires each projected face to own a projected loop boundary",
            )
        })?;
    let boundary = loop_entity
        .boundary()
        .expect("projected loop boundary was required before face extraction");
    Ok(ProjectedOverlapFaceGeometry {
        face_identity: face.identity().topology_entity_identity().to_string(),
        projected_face_identity: face.identity().projected_fact_identity().to_string(),
        loop_identity: loop_entity
            .identity()
            .topology_entity_identity()
            .to_string(),
        projected_loop_identity: loop_entity.identity().projected_fact_identity().to_string(),
        outer_points: boundary.outer_points().to_vec(),
        containment_candidate_points: boundary
            .containment_candidate_points()
            .map(<[[f64; 2]]>::to_vec),
    })
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProjectedOverlapFaceGeometry {
    pub(crate) face_identity: String,
    pub(crate) projected_face_identity: String,
    pub(crate) loop_identity: String,
    pub(crate) projected_loop_identity: String,
    pub(crate) outer_points: Vec<[f64; 2]>,
    pub(crate) containment_candidate_points: Option<Vec<[f64; 2]>>,
}
