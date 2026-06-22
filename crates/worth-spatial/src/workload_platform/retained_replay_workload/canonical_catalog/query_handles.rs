use forge_query::facade::ForgeQueryApplicationFacade;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use crate::facade::planar_contract_bundle::{
    PlanarContractBundleValidationQueryDomain, PlanarContractBundleValidationQueryWorld,
};
use crate::facade::planar_local_frame::{
    PlanarLocalFrameCertificateQueryDomain, PlanarLocalFrameCertificateQueryWorld,
};
use crate::facade::planar_motion_posture::{
    PlanarMotionPostureQueryDomain, PlanarMotionPostureQueryWorld,
};
use crate::facade::planar_overlap::{
    CoplanarOverlapContractQueryDomain, CoplanarOverlapContractQueryWorld,
};
use crate::facade::planar_precision::{
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
};
use crate::facade::planar_predicate_consumption::{
    PredicateCertificateConsumptionQueryDomain, PredicateCertificateConsumptionQueryWorld,
};
use crate::facade::planar_predicates::{
    PlanarPredicateAuthorityQueryDomain, PlanarPredicateAuthorityQueryWorld,
};
use crate::facade::planar_projection::{
    ProjectPointToCertifiedPlane2DQueryDomain, ProjectPointToCertifiedPlane2DQueryWorld,
};
use crate::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFactsQueryDomain, ProjectionConsumedPlanarFactsQueryWorld,
};
use crate::facade::planar_retained_facts::{
    RetainedPlanarFactsQueryDomain, RetainedPlanarFactsQueryWorld,
};
use crate::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DQueryDomain, CertifiedSegmentSegment2DQueryWorld,
};
use crate::facade::planar_signed_area::{
    CertifiedSignedArea2DQueryDomain, CertifiedSignedArea2DQueryWorld,
};
use crate::facade::planar_structural_identity::{
    PlanarStructuralIdentityQueryDomain, PlanarStructuralIdentityQueryWorld,
};
use crate::facade::planar_topology_contract::{
    PlanarTopologyContractCompletenessQueryDomain, PlanarTopologyContractCompletenessQueryWorld,
};
use crate::facade::planar_winding::{
    CertifiedPolygonWinding2DQueryDomain, CertifiedPolygonWinding2DQueryWorld,
};
macro_rules! handle {
    ($fn_name:ident, $cache_name:ident, $domain:expr, $world:expr, $domain_ty:ty, $world_ty:ty) => {
        pub(super) fn $fn_name(
            world: &'static str,
        ) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty> {
            static $cache_name: OnceLock<
                Mutex<
                    BTreeMap<
                        &'static str,
                        forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
                            $domain_ty,
                            $world_ty,
                        >,
                    >,
                >,
            > = OnceLock::new();
            let mut cache = $cache_name
                .get_or_init(|| Mutex::new(BTreeMap::new()))
                .lock()
                .expect("canonical workload query handle cache lock should not be poisoned");
            cache
                .entry(world)
                .or_insert_with(|| {
                    ForgeQueryApplicationFacade::runtime_backed_default()
                        .domain($domain)
                        .with_operating_context($world(world))
                        .validate()
                        .expect("validated canonical workload query domain")
                        .admit()
                        .expect("admitted canonical workload query domain")
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
    topology_contract_handle,
    TOPOLOGY_CONTRACT_HANDLE_CACHE,
    PlanarTopologyContractCompletenessQueryDomain,
    PlanarTopologyContractCompletenessQueryWorld::new,
    PlanarTopologyContractCompletenessQueryDomain,
    PlanarTopologyContractCompletenessQueryWorld
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
    projection_handle,
    PROJECTION_HANDLE_CACHE,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld::new,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld
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
    predicate_consumption_handle,
    PREDICATE_CONSUMPTION_HANDLE_CACHE,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld::new,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld
);
handle!(
    retained_planar_handle,
    RETAINED_PLANAR_HANDLE_CACHE,
    RetainedPlanarFactsQueryDomain,
    RetainedPlanarFactsQueryWorld::new,
    RetainedPlanarFactsQueryDomain,
    RetainedPlanarFactsQueryWorld
);
handle!(
    projection_consumption_handle,
    PROJECTION_CONSUMPTION_HANDLE_CACHE,
    ProjectionConsumedPlanarFactsQueryDomain,
    ProjectionConsumedPlanarFactsQueryWorld::new,
    ProjectionConsumedPlanarFactsQueryDomain,
    ProjectionConsumedPlanarFactsQueryWorld
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

pub(super) fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
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
                    "canonical-retained-predicate",
                ))
                .validate()
                .expect("validated canonical predicate domain")
                .admit()
                .expect("admitted canonical predicate domain")
        })
        .clone()
}
