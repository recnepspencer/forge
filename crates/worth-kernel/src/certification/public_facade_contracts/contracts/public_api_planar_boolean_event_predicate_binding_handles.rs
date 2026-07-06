use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_local_frame::{
    PlanarLocalFrameCertificateQueryDomain, PlanarLocalFrameCertificateQueryWorld,
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
use worth_spatial::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DQueryDomain, CertifiedSegmentSegment2DQueryWorld,
};

macro_rules! handle {
    ($name:ident, $cache:ident, $domain:expr, $world:expr, $domain_ty:ty, $world_ty:ty) => {
        pub(crate) fn $name(
            world: &'static str,
        ) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty> {
            static $cache: OnceLock<
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
            let mut cache = $cache
                .get_or_init(|| Mutex::new(BTreeMap::new()))
                .lock()
                .expect("predicate-binding handle cache lock");
            cache
                .entry(world)
                .or_insert_with(|| {
                    ForgeQueryApplicationFacade::runtime_backed_default()
                        .domain($domain)
                        .with_operating_context($world(world))
                        .validate()
                        .expect("validated predicate-binding contract domain")
                        .admit()
                        .expect("admitted predicate-binding contract domain")
                })
                .clone()
        }
    };
}

handle!(
    frame_handle,
    FRAME_HANDLE_CACHE,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld::new,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld
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
    projection_handle,
    PROJECTION_HANDLE_CACHE,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld::new,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld
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
    predicate_consumption_handle,
    PREDICATE_CONSUMPTION_HANDLE_CACHE,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld::new,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld
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
                    "event-predicate-binding",
                ))
                .validate()
                .expect("validated predicate domain")
                .admit()
                .expect("admitted predicate domain")
        })
        .clone()
}
