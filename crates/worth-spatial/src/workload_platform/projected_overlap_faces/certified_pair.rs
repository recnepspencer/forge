use forge_query::facade::ForgeQueryDomainOperatingContext;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_planar_overlap::{
    CoplanarOverlapContractContracts, CoplanarOverlapContractExtractor,
    CoplanarOverlapContractPlan, CoplanarOverlapContractQueryDomain,
};
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::planar_contracts::coplanar_overlap_contract::{
    CoplanarOverlapContractReceipt, CoplanarOverlapDenial,
};

use super::certified_face::CertifiedProjectedOverlapFace;
use super::ProjectedOverlapFaceDenial;

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedProjectedOverlapCandidatePair {
    projection_stage_identity: String,
    pair_identity: String,
    first: CertifiedProjectedOverlapFace,
    second: CertifiedProjectedOverlapFace,
}

impl CertifiedProjectedOverlapCandidatePair {
    pub(crate) fn new(
        projection_stage_identity: impl Into<String>,
        first: CertifiedProjectedOverlapFace,
        second: CertifiedProjectedOverlapFace,
    ) -> Result<Self, ProjectedOverlapFaceDenial> {
        let projection_stage_identity = projection_stage_identity.into();
        if first.projection_stage_identity() != projection_stage_identity
            || second.projection_stage_identity() != projection_stage_identity
        {
            return Err(ProjectedOverlapFaceDenial::new(
                "certified projected overlap candidate pairs require both faces to share the candidate projection stage",
            ));
        }
        let pair_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("projection-stage:{projection_stage_identity}"),
                format!("first-face:{}", first.projected_face_identity()),
                format!("first-loop:{}", first.projected_loop_identity()),
                format!("first-winding:{}", first.winding_fact_digest()),
                format!("first-signed-area:{}", first.signed_area_fact_digest()),
                format!("first-frame:{}", first.local_frame_identity()),
                format!("first-frame-fact:{}", first.local_frame_fact_digest()),
                format!("first-precision:{}", first.precision_fact_digest()),
                format!(
                    "first-motion:{}",
                    first.movement_rotation_posture_identity()
                ),
                format!("second-face:{}", second.projected_face_identity()),
                format!("second-loop:{}", second.projected_loop_identity()),
                format!("second-winding:{}", second.winding_fact_digest()),
                format!("second-signed-area:{}", second.signed_area_fact_digest()),
                format!("second-frame:{}", second.local_frame_identity()),
                format!("second-frame-fact:{}", second.local_frame_fact_digest()),
                format!("second-precision:{}", second.precision_fact_digest()),
                format!(
                    "second-motion:{}",
                    second.movement_rotation_posture_identity()
                ),
            ],
        );
        Ok(Self {
            projection_stage_identity,
            pair_identity,
            first,
            second,
        })
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn pair_identity(&self) -> &str {
        &self.pair_identity
    }

    pub fn first_face(&self) -> &CertifiedProjectedOverlapFace {
        &self.first
    }

    pub fn second_face(&self) -> &CertifiedProjectedOverlapFace {
        &self.second
    }

    pub fn extract_overlap<OC, SC, PC>(
        &self,
        overlap_contracts: &CoplanarOverlapContractContracts<OC, SC, PC>,
        planar_neighborhood_identity: &str,
    ) -> Result<CoplanarOverlapContractReceipt, ProjectedOverlapFaceDenial>
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    {
        self.extract_with_second_face(
            self.second_face(),
            overlap_contracts,
            planar_neighborhood_identity,
        )
    }

    pub fn compile_overlap_with_second_face<'a, OC, SC, PC>(
        &self,
        second: &CertifiedProjectedOverlapFace,
        overlap_contracts: &'a CoplanarOverlapContractContracts<OC, SC, PC>,
        planar_neighborhood_identity: &str,
    ) -> Result<CoplanarOverlapContractPlan<'a, OC, SC, PC>, CoplanarOverlapDenial>
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    {
        CoplanarOverlapContractExtractor::between(self.first.overlap_face(), second.overlap_face())
            .within_planar_neighborhood(planar_neighborhood_identity)
            .compile(overlap_contracts)
    }

    fn extract_with_second_face<OC, SC, PC>(
        &self,
        second: &CertifiedProjectedOverlapFace,
        overlap_contracts: &CoplanarOverlapContractContracts<OC, SC, PC>,
        planar_neighborhood_identity: &str,
    ) -> Result<CoplanarOverlapContractReceipt, ProjectedOverlapFaceDenial>
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    {
        let plan = self
            .compile_overlap_with_second_face(
                second,
                overlap_contracts,
                planar_neighborhood_identity,
            )
            .map_err(|error| ProjectedOverlapFaceDenial::new(error.reason().to_string()))?;
        plan.extract()
            .map_err(|error| ProjectedOverlapFaceDenial::new(format!("{error:?}")))
    }
}
