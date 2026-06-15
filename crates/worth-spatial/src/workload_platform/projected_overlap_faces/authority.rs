use forge_query::facade::ForgeQueryDomainOperatingContext;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_planar_overlap::CoplanarOverlapContractQueryDomain;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_projection::ProjectPointToCertifiedPlane2DQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::CertifiedSignedArea2DQueryDomain;
use crate::bindings::query_native_planar_winding::CertifiedPolygonWinding2DQueryDomain;
use crate::workload_platform::certification_context::WorkloadCertificationContext;

use super::{
    CertifiedProjectedOverlapCandidatePairs, CertifiedProjectedOverlapFaceSet,
    CoplanarOverlapExtractionBundle, ProjectedOverlapFaceDenial, ProjectedOverlapFaceSet,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedProjectedOverlapBridgeAuthority {
    authority_digest: String,
    context_identity: String,
    projection_stage_identity: String,
    movement_rotation_posture_identity: String,
    certified_face_set_digest: String,
    certified_faces: CertifiedProjectedOverlapFaceSet,
    extraction_bundle: CoplanarOverlapExtractionBundle,
}

impl CertifiedProjectedOverlapBridgeAuthority {
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
        PXC: ForgeQueryDomainOperatingContext<
            crate::bindings::query_native_planar_precision::PlanarPrecisionCertificationQueryDomain,
        >,
        FC: ForgeQueryDomainOperatingContext<
            crate::bindings::query_native_planar_local_frame::PlanarLocalFrameCertificateQueryDomain,
        >,
    {
        let projected_faces = ProjectedOverlapFaceSet::from_context(context)?;
        let certified_faces =
            CertifiedProjectedOverlapFaceSet::from_projected_faces(projected_faces, context)?;
        let extraction_bundle = CoplanarOverlapExtractionBundle::from_context_candidate_pairs(
            certified_faces.candidate_pairs(),
            context,
        )?;
        let authority_digest = bridge_authority_digest(
            context.context_identity(),
            certified_faces.projection_stage_identity(),
            context.movement_rotation_posture_identity(),
            certified_faces.certified_face_count(),
            certified_faces.candidate_pair_count(),
            certified_faces.certified_face_set_digest(),
            extraction_bundle.receipts().len(),
            extraction_bundle.extraction_bundle_digest(),
        );
        Ok(Self {
            authority_digest,
            context_identity: context.context_identity().to_string(),
            projection_stage_identity: certified_faces.projection_stage_identity().to_string(),
            movement_rotation_posture_identity: context
                .movement_rotation_posture_identity()
                .to_string(),
            certified_face_set_digest: certified_faces.certified_face_set_digest().to_string(),
            certified_faces,
            extraction_bundle,
        })
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub fn context_identity(&self) -> &str {
        &self.context_identity
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn certified_face_set_digest(&self) -> &str {
        &self.certified_face_set_digest
    }

    pub fn certified_face_count(&self) -> usize {
        self.certified_faces.certified_face_count()
    }

    pub fn candidate_pair_count(&self) -> usize {
        self.certified_faces.candidate_pair_count()
    }

    pub fn extraction_receipt_count(&self) -> usize {
        self.extraction_bundle.receipts().len()
    }

    pub fn candidate_pairs(&self) -> CertifiedProjectedOverlapCandidatePairs<'_> {
        self.certified_faces.candidate_pairs()
    }

    pub fn certified_faces(&self) -> &CertifiedProjectedOverlapFaceSet {
        &self.certified_faces
    }

    pub fn extraction_bundle(&self) -> &CoplanarOverlapExtractionBundle {
        &self.extraction_bundle
    }
}

fn bridge_authority_digest(
    context_identity: &str,
    projection_stage_identity: &str,
    movement_rotation_posture_identity: &str,
    certified_face_count: usize,
    candidate_pair_count: usize,
    certified_face_set_digest: &str,
    extraction_receipt_count: usize,
    extraction_bundle_digest: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "certified-projected-overlap-bridge-authority".to_string(),
            format!("context:{context_identity}"),
            format!("projection:{projection_stage_identity}"),
            format!("motion:{movement_rotation_posture_identity}"),
            format!("certified-faces:{certified_face_count}"),
            format!("candidate-pairs:{candidate_pair_count}"),
            format!("certified-face-set:{certified_face_set_digest}"),
            format!("extraction-receipts:{extraction_receipt_count}"),
            format!("extraction-bundle:{extraction_bundle_digest}"),
        ],
    )
}
