use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_contract_bundle::{
    PlanarContractBundleValidationQueryDomain, PlanarContractBundleValidationQueryWorld,
};
use worth_spatial::facade::planar_local_frame::{
    PlanarLocalFrameCertificateQueryDomain, PlanarLocalFrameCertificateQueryWorld,
};
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionPostureQueryDomain, PlanarMotionPostureQueryWorld,
};
use worth_spatial::facade::planar_overlap::{
    CoplanarOverlapContractQueryDomain, CoplanarOverlapContractQueryWorld,
};
use worth_spatial::facade::planar_precision::{
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
};
use worth_spatial::facade::planar_predicate_consumption::{
    PredicateCertificateConsumptionQueryDomain, PredicateCertificateConsumptionQueryWorld,
};
use worth_spatial::facade::planar_predicates::{
    PlanarPredicateAuthorityQueryDomain, PlanarPredicateAuthorityQueryWorld,
};
use worth_spatial::facade::planar_projection::{
    ProjectPointToCertifiedPlane2DQueryDomain, ProjectPointToCertifiedPlane2DQueryWorld,
};
use worth_spatial::facade::planar_retained_facts::{
    RetainedPlanarFactsQueryDomain, RetainedPlanarFactsQueryWorld,
};
use worth_spatial::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DQueryDomain, CertifiedSegmentSegment2DQueryWorld,
};
use worth_spatial::facade::planar_signed_area::{
    CertifiedSignedArea2DQueryDomain, CertifiedSignedArea2DQueryWorld,
};
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentityQueryDomain, PlanarStructuralIdentityQueryWorld,
};
use worth_spatial::facade::planar_topology_contract::{
    PlanarTopologyContractCompletenessQueryDomain, PlanarTopologyContractCompletenessQueryWorld,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2DQueryDomain, CertifiedPolygonWinding2DQueryWorld,
};

const WORLD: &str = "kernel-planar-contract-bundle";
pub(crate) const MOVEMENT: &str = "movement:kernel-bundle-stable";

macro_rules! handle {
    ($fn_name:ident, $domain:expr, $world:expr, $domain_ty:ty, $world_ty:ty) => {
        pub(crate) fn $fn_name(
        ) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty> {
            ForgeQueryApplicationFacade::runtime_backed_default()
                .domain($domain)
                .with_operating_context($world(WORLD))
                .validate()
                .expect("validated kernel bundle domain")
                .admit()
                .expect("admitted kernel bundle domain")
        }
    };
}

handle!(
    bundle_handle,
    PlanarContractBundleValidationQueryDomain,
    PlanarContractBundleValidationQueryWorld::new,
    PlanarContractBundleValidationQueryDomain,
    PlanarContractBundleValidationQueryWorld
);
handle!(
    predicate_consumption_handle,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld::new,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld
);
handle!(
    precision_handle,
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld::new,
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld
);
handle!(
    frame_handle,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld::new,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld
);
handle!(
    projection_handle,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld::new,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld
);
handle!(
    winding_handle,
    CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld::new,
    CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld
);
handle!(
    signed_area_handle,
    CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld::new,
    CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld
);
handle!(
    segment_handle,
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld::new,
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld
);
handle!(
    overlap_handle,
    CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld::new,
    CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld
);
handle!(
    topology_contract_handle,
    PlanarTopologyContractCompletenessQueryDomain,
    PlanarTopologyContractCompletenessQueryWorld::new,
    PlanarTopologyContractCompletenessQueryDomain,
    PlanarTopologyContractCompletenessQueryWorld
);
handle!(
    motion_posture_handle,
    PlanarMotionPostureQueryDomain,
    PlanarMotionPostureQueryWorld::new,
    PlanarMotionPostureQueryDomain,
    PlanarMotionPostureQueryWorld
);
handle!(
    structural_identity_handle,
    PlanarStructuralIdentityQueryDomain,
    PlanarStructuralIdentityQueryWorld::new,
    PlanarStructuralIdentityQueryDomain,
    PlanarStructuralIdentityQueryWorld
);
handle!(
    retained_planar_handle,
    RetainedPlanarFactsQueryDomain,
    RetainedPlanarFactsQueryWorld::new,
    RetainedPlanarFactsQueryDomain,
    RetainedPlanarFactsQueryWorld
);

pub(crate) fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "kernel-bundle-predicate",
        ))
        .validate()
        .expect("validated predicate")
        .admit()
        .expect("admitted predicate")
}
