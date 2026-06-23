use forge_query::facade::ForgeQueryApplicationFacade;

use crate::facade::planar_contract_bundle::{
    PlanarContractBundleValidationQueryDomain, PlanarContractBundleValidationQueryWorld,
};
use crate::facade::planar_diagnostics::{
    PlanarDiagnosticBundleQueryDomain, PlanarDiagnosticBundleQueryWorld,
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
use crate::facade::planar_recovery::{
    PlanarRecoveryPostureQueryDomain, PlanarRecoveryPostureQueryWorld,
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
    ($fn_name:ident, $domain:expr, $world_ctor:expr, $domain_ty:ty, $world_ty:ty) => {
        pub(crate) fn $fn_name(
        ) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty> {
            ForgeQueryApplicationFacade::runtime_backed_default()
                .domain($domain)
                .with_operating_context($world_ctor(super::WORLD))
                .validate()
                .expect("validated local-frame test domain")
                .admit()
                .expect("admitted local-frame test domain")
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
    predicate_consumption_handle,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld::new,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld
);
handle!(
    motion_posture_handle,
    PlanarMotionPostureQueryDomain,
    PlanarMotionPostureQueryWorld::new,
    PlanarMotionPostureQueryDomain,
    PlanarMotionPostureQueryWorld
);
handle!(
    retained_planar_handle,
    RetainedPlanarFactsQueryDomain,
    RetainedPlanarFactsQueryWorld::new,
    RetainedPlanarFactsQueryDomain,
    RetainedPlanarFactsQueryWorld
);
handle!(
    projection_consumption_handle,
    ProjectionConsumedPlanarFactsQueryDomain,
    ProjectionConsumedPlanarFactsQueryWorld::new,
    ProjectionConsumedPlanarFactsQueryDomain,
    ProjectionConsumedPlanarFactsQueryWorld
);
handle!(
    recovery_handle,
    PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld::new,
    PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld
);
handle!(
    diagnostic_handle,
    PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld::new,
    PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld
);
handle!(
    structural_identity_handle,
    PlanarStructuralIdentityQueryDomain,
    PlanarStructuralIdentityQueryWorld::new,
    PlanarStructuralIdentityQueryDomain,
    PlanarStructuralIdentityQueryWorld
);

pub(crate) fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "local-frame-predicate",
        ))
        .validate()
        .expect("validated predicate domain")
        .admit()
        .expect("admitted predicate domain")
}
