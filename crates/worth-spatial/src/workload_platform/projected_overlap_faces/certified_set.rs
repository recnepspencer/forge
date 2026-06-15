use forge_query::facade::ForgeQueryDomainOperatingContext;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_planar_overlap::CoplanarOverlapContractQueryDomain;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_projection::ProjectPointToCertifiedPlane2DQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::CertifiedSignedArea2DQueryDomain;
use crate::bindings::query_native_planar_winding::CertifiedPolygonWinding2DQueryDomain;
use crate::workload_platform::certification_context::WorkloadCertificationContext;

use super::bundle::contracts_from_context;
use super::certified_face::CertifiedProjectedOverlapFace;
use super::certified_pair::CertifiedProjectedOverlapCandidatePair;
use super::face_set::{ProjectedOverlapCandidatePolicy, ProjectedOverlapFaceSet};
use super::ProjectedOverlapFaceDenial;

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedProjectedOverlapFaceSet {
    certified_face_set_digest: String,
    projection_stage_identity: String,
    faces: Vec<CertifiedProjectedOverlapFace>,
    candidate_pairs: Vec<CertifiedProjectedOverlapCandidatePair>,
}

impl CertifiedProjectedOverlapFaceSet {
    pub fn from_projected_faces<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        face_set: ProjectedOverlapFaceSet,
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
        let (projection_stage_identity, geometries, candidate_policy) =
            face_set.into_certification_parts();
        if projection_stage_identity != context.projection_stage_identity() {
            return Err(ProjectedOverlapFaceDenial::new(
                "certified projected overlap faces require the projected workload and certification context to share the same projection stage",
            ));
        }
        let contracts = contracts_from_context(context);
        let faces = geometries
            .into_iter()
            .map(|geometry| {
                CertifiedProjectedOverlapFace::from_projected_geometry(
                    projection_stage_identity.clone(),
                    geometry,
                    &contracts,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let candidate_pairs =
            certified_candidate_pairs(&projection_stage_identity, &faces, candidate_policy)?;
        let certified_face_set_digest =
            certified_face_set_digest(&projection_stage_identity, &faces, &candidate_pairs);
        Ok(Self {
            certified_face_set_digest,
            projection_stage_identity,
            faces,
            candidate_pairs,
        })
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn certified_face_set_digest(&self) -> &str {
        &self.certified_face_set_digest
    }

    pub fn certified_face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn candidate_pair_count(&self) -> usize {
        self.candidate_pairs.len()
    }

    pub fn candidate_pairs(&self) -> CertifiedProjectedOverlapCandidatePairs<'_> {
        CertifiedProjectedOverlapCandidatePairs {
            pairs: &self.candidate_pairs,
        }
    }
}

pub struct CertifiedProjectedOverlapCandidatePairs<'a> {
    pairs: &'a [CertifiedProjectedOverlapCandidatePair],
}

impl<'a> CertifiedProjectedOverlapCandidatePairs<'a> {
    pub fn first_pair(&self) -> Option<&'a CertifiedProjectedOverlapCandidatePair> {
        self.pairs.first()
    }

    pub fn as_slice(&self) -> &'a [CertifiedProjectedOverlapCandidatePair] {
        self.pairs
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

fn certified_face_set_digest(
    projection_stage_identity: &str,
    faces: &[CertifiedProjectedOverlapFace],
    candidate_pairs: &[CertifiedProjectedOverlapCandidatePair],
) -> String {
    let mut parts = vec![
        "certified-projected-overlap-face-set".to_string(),
        format!("projection-stage:{projection_stage_identity}"),
        format!("certified-faces:{}", faces.len()),
        format!("candidate-pairs:{}", candidate_pairs.len()),
    ];
    parts.extend(faces.iter().flat_map(|face| {
        [
            format!("face:{}", face.face_identity()),
            format!("projected-face:{}", face.projected_face_identity()),
            format!("loop:{}", face.loop_identity()),
            format!("projected-loop:{}", face.projected_loop_identity()),
            format!("winding:{}", face.winding_fact_digest()),
            format!("signed-area:{}", face.signed_area_fact_digest()),
            format!("local-frame:{}", face.local_frame_fact_digest()),
            format!("precision:{}", face.precision_fact_digest()),
            format!("motion:{}", face.movement_rotation_posture_identity()),
        ]
    }));
    parts.extend(
        candidate_pairs
            .iter()
            .map(|pair| format!("candidate-pair:{}", pair.pair_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn certified_candidate_pairs(
    projection_stage_identity: &str,
    faces: &[CertifiedProjectedOverlapFace],
    candidate_policy: ProjectedOverlapCandidatePolicy,
) -> Result<Vec<CertifiedProjectedOverlapCandidatePair>, ProjectedOverlapFaceDenial> {
    match candidate_policy {
        ProjectedOverlapCandidatePolicy::AdjacentProjectedFacePairs => {
            if faces.len() % 2 != 0 {
                return Err(ProjectedOverlapFaceDenial::new(
                    "certified projected overlap candidate pairs require an even number of certified faces",
                ));
            }
            Ok(faces
                .chunks_exact(2)
                .map(|pair| {
                    CertifiedProjectedOverlapCandidatePair::new(
                        projection_stage_identity,
                        pair[0].clone(),
                        pair[1].clone(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?)
        }
    }
}
