#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarAdmissionFamily {
    ExactPlanarPredicateAuthority,
    PlanarLocalFrameCertificate,
    CertifiedPlaneProjection2d,
    CertifiedSegmentContact2d,
    CertifiedPolygonWinding2d,
    CertifiedSignedArea2d,
    CoplanarOverlapContract,
    PlanarStructuralIdentity,
    MovementRotationPosture,
    RetainedPlanarFact,
    ProjectionConsumedPlanarFact,
    PlanarRecoveryPosture,
    PlanarDiagnostics,
    DirtyPlanarInput,
    UnboundedPlanarDomain,
    PlanarContractBundle,
    PredicateCertificateConsumption,
}

impl PlanarAdmissionFamily {
    pub const fn all() -> [Self; 17] {
        [
            Self::ExactPlanarPredicateAuthority,
            Self::PlanarLocalFrameCertificate,
            Self::CertifiedPlaneProjection2d,
            Self::CertifiedSegmentContact2d,
            Self::CertifiedPolygonWinding2d,
            Self::CertifiedSignedArea2d,
            Self::CoplanarOverlapContract,
            Self::PlanarStructuralIdentity,
            Self::MovementRotationPosture,
            Self::RetainedPlanarFact,
            Self::ProjectionConsumedPlanarFact,
            Self::PlanarRecoveryPosture,
            Self::PlanarDiagnostics,
            Self::DirtyPlanarInput,
            Self::UnboundedPlanarDomain,
            Self::PlanarContractBundle,
            Self::PredicateCertificateConsumption,
        ]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactPlanarPredicateAuthority => "exact-planar-predicate-authority",
            Self::PlanarLocalFrameCertificate => "planar-local-frame-certificate",
            Self::CertifiedPlaneProjection2d => "certified-plane-projection-2d",
            Self::CertifiedSegmentContact2d => "certified-segment-contact-2d",
            Self::CertifiedPolygonWinding2d => "certified-polygon-winding-2d",
            Self::CertifiedSignedArea2d => "certified-signed-area-2d",
            Self::CoplanarOverlapContract => "coplanar-overlap-contract",
            Self::PlanarStructuralIdentity => "planar-structural-identity",
            Self::MovementRotationPosture => "movement-rotation-posture",
            Self::RetainedPlanarFact => "retained-planar-fact",
            Self::ProjectionConsumedPlanarFact => "projection-consumed-planar-fact",
            Self::PlanarRecoveryPosture => "planar-recovery-posture",
            Self::PlanarDiagnostics => "planar-diagnostics",
            Self::DirtyPlanarInput => "dirty-planar-input",
            Self::UnboundedPlanarDomain => "unbounded-planar-domain",
            Self::PlanarContractBundle => "planar-contract-bundle",
            Self::PredicateCertificateConsumption => "predicate-certificate-consumption",
        }
    }
}
