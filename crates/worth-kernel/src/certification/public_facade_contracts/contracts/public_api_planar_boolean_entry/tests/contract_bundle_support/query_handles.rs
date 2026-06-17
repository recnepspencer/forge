use forge_query::facade::ForgeQueryApplicationFacade;
use std::sync::OnceLock;
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
    ($fn_name:ident, $cache_name:ident, $domain:expr, $world:expr, $domain_ty:ty, $world_ty:ty) => {
        pub(crate) fn $fn_name(
        ) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty> {
            static $cache_name: OnceLock<
                forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty>,
            > = OnceLock::new();
            $cache_name
                .get_or_init(|| {
                    ForgeQueryApplicationFacade::runtime_backed_default()
                        .domain($domain)
                        .with_operating_context($world(WORLD))
                        .validate()
                        .expect("validated kernel bundle domain")
                        .admit()
                        .expect("admitted kernel bundle domain")
                })
                .clone()
        }
    };
}

handle!(
    bundle_handle,
    BUNDLE_HANDLE_CACHE,
    PlanarContractBundleValidationQueryDomain,
    PlanarContractBundleValidationQueryWorld::new,
    PlanarContractBundleValidationQueryDomain,
    PlanarContractBundleValidationQueryWorld
);
handle!(
    predicate_consumption_handle,
    PREDICATE_CONSUMPTION_HANDLE_CACHE,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld::new,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld
);
handle!(
    precision_handle,
    PRECISION_HANDLE_CACHE,
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld::new,
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld
);
handle!(
    frame_handle,
    FRAME_HANDLE_CACHE,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld::new,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld
);
handle!(
    projection_handle,
    PROJECTION_HANDLE_CACHE,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld::new,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld
);
handle!(
    winding_handle,
    WINDING_HANDLE_CACHE,
    CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld::new,
    CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld
);
handle!(
    signed_area_handle,
    SIGNED_AREA_HANDLE_CACHE,
    CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld::new,
    CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld
);
handle!(
    segment_handle,
    SEGMENT_HANDLE_CACHE,
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld::new,
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld
);
handle!(
    overlap_handle,
    OVERLAP_HANDLE_CACHE,
    CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld::new,
    CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld
);
handle!(
    topology_contract_handle,
    TOPOLOGY_CONTRACT_HANDLE_CACHE,
    PlanarTopologyContractCompletenessQueryDomain,
    PlanarTopologyContractCompletenessQueryWorld::new,
    PlanarTopologyContractCompletenessQueryDomain,
    PlanarTopologyContractCompletenessQueryWorld
);
handle!(
    motion_posture_handle,
    MOTION_POSTURE_HANDLE_CACHE,
    PlanarMotionPostureQueryDomain,
    PlanarMotionPostureQueryWorld::new,
    PlanarMotionPostureQueryDomain,
    PlanarMotionPostureQueryWorld
);
handle!(
    structural_identity_handle,
    STRUCTURAL_IDENTITY_HANDLE_CACHE,
    PlanarStructuralIdentityQueryDomain,
    PlanarStructuralIdentityQueryWorld::new,
    PlanarStructuralIdentityQueryDomain,
    PlanarStructuralIdentityQueryWorld
);
handle!(
    retained_planar_handle,
    RETAINED_PLANAR_HANDLE_CACHE,
    RetainedPlanarFactsQueryDomain,
    RetainedPlanarFactsQueryWorld::new,
    RetainedPlanarFactsQueryDomain,
    RetainedPlanarFactsQueryWorld
);

pub(crate) fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    static PREDICATE_HANDLE_CACHE: OnceLock<
        forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarPredicateAuthorityQueryDomain,
            PlanarPredicateAuthorityQueryWorld,
        >,
    > = OnceLock::new();
    PREDICATE_HANDLE_CACHE
        .get_or_init(|| {
            ForgeQueryApplicationFacade::runtime_backed_default()
                .domain(PlanarPredicateAuthorityQueryDomain)
                .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
                    "kernel-bundle-predicate",
                ))
                .validate()
                .expect("validated predicate")
                .admit()
                .expect("admitted predicate")
        })
        .clone()
}
