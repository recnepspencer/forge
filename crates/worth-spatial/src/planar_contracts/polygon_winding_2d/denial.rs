#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedPolygonWinding2DDenialKind {
    MissingPrimaryLoopIdentity,
    MissingTopologyLoopBasis,
    MissingPlanarNeighborhood,
    MissingWindingPolicy,
    MissingProjectedVertexReceipt,
    TooFewVertices,
    DuplicateVertex,
    FrameBasisMismatch,
    MovementRotationMismatch,
    TolerancePolicyMismatch,
    PredicateBasisMismatch,
    AmbiguousWindingPredicateEvidence,
    DegenerateLoopArea,
    SegmentContactCertificationBasis,
    SelfIntersectionOrAmbiguousTouch,
    ContainmentTouchesBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedPolygonWinding2DDenialBasisLocus {
    LoopIdentity,
    TopologyLoopBasis,
    PlanarNeighborhood,
    WindingPolicy,
    ProjectionReceipt,
    VertexCount,
    VertexUniqueness,
    FrameBasis,
    MovementRotationPosture,
    TolerancePolicy,
    PredicateBasis,
    WindingPredicateEvidence,
    LoopArea,
    SegmentContact,
    ContainmentBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedPolygonWinding2DDenial {
    kind: CertifiedPolygonWinding2DDenialKind,
    basis_locus: CertifiedPolygonWinding2DDenialBasisLocus,
    reason: &'static str,
}

impl CertifiedPolygonWinding2DDenial {
    pub(crate) const fn new(
        kind: CertifiedPolygonWinding2DDenialKind,
        reason: &'static str,
    ) -> Self {
        Self {
            kind,
            basis_locus: denial_basis_locus(kind),
            reason,
        }
    }

    pub fn kind(&self) -> CertifiedPolygonWinding2DDenialKind {
        self.kind
    }

    pub fn basis_locus(&self) -> CertifiedPolygonWinding2DDenialBasisLocus {
        self.basis_locus
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

const fn denial_basis_locus(
    kind: CertifiedPolygonWinding2DDenialKind,
) -> CertifiedPolygonWinding2DDenialBasisLocus {
    match kind {
        CertifiedPolygonWinding2DDenialKind::MissingPrimaryLoopIdentity => {
            CertifiedPolygonWinding2DDenialBasisLocus::LoopIdentity
        }
        CertifiedPolygonWinding2DDenialKind::MissingTopologyLoopBasis => {
            CertifiedPolygonWinding2DDenialBasisLocus::TopologyLoopBasis
        }
        CertifiedPolygonWinding2DDenialKind::MissingPlanarNeighborhood => {
            CertifiedPolygonWinding2DDenialBasisLocus::PlanarNeighborhood
        }
        CertifiedPolygonWinding2DDenialKind::MissingWindingPolicy => {
            CertifiedPolygonWinding2DDenialBasisLocus::WindingPolicy
        }
        CertifiedPolygonWinding2DDenialKind::MissingProjectedVertexReceipt => {
            CertifiedPolygonWinding2DDenialBasisLocus::ProjectionReceipt
        }
        CertifiedPolygonWinding2DDenialKind::TooFewVertices => {
            CertifiedPolygonWinding2DDenialBasisLocus::VertexCount
        }
        CertifiedPolygonWinding2DDenialKind::DuplicateVertex => {
            CertifiedPolygonWinding2DDenialBasisLocus::VertexUniqueness
        }
        CertifiedPolygonWinding2DDenialKind::FrameBasisMismatch => {
            CertifiedPolygonWinding2DDenialBasisLocus::FrameBasis
        }
        CertifiedPolygonWinding2DDenialKind::MovementRotationMismatch => {
            CertifiedPolygonWinding2DDenialBasisLocus::MovementRotationPosture
        }
        CertifiedPolygonWinding2DDenialKind::TolerancePolicyMismatch => {
            CertifiedPolygonWinding2DDenialBasisLocus::TolerancePolicy
        }
        CertifiedPolygonWinding2DDenialKind::PredicateBasisMismatch => {
            CertifiedPolygonWinding2DDenialBasisLocus::PredicateBasis
        }
        CertifiedPolygonWinding2DDenialKind::AmbiguousWindingPredicateEvidence => {
            CertifiedPolygonWinding2DDenialBasisLocus::WindingPredicateEvidence
        }
        CertifiedPolygonWinding2DDenialKind::DegenerateLoopArea => {
            CertifiedPolygonWinding2DDenialBasisLocus::LoopArea
        }
        CertifiedPolygonWinding2DDenialKind::SegmentContactCertificationBasis
        | CertifiedPolygonWinding2DDenialKind::SelfIntersectionOrAmbiguousTouch => {
            CertifiedPolygonWinding2DDenialBasisLocus::SegmentContact
        }
        CertifiedPolygonWinding2DDenialKind::ContainmentTouchesBoundary => {
            CertifiedPolygonWinding2DDenialBasisLocus::ContainmentBoundary
        }
    }
}
