use super::contract::{S8AccessShapeContract, S8ExpectedCounterClass};
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{
    S8AccessShapeDetail, S8BoundedScanBasis, S8FullDeclaredScanBasis, S8ManifestGraphWalkBasis,
};
use super::lane::S8AccessLaneClassification;
use super::shape::S8AccessShape;
use crate::materialization::S8LayoutCoverageWitness;

pub(crate) fn bounded_scan(
    coverage: S8LayoutCoverageWitness,
    lane: S8AccessLaneClassification,
    basis: S8BoundedScanBasis,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::BoundedScan(basis),
        lane,
        S8ExpectedCounterClass::BoundedScan,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}

pub(crate) fn full_declared_scan(
    coverage: S8LayoutCoverageWitness,
    lane: S8AccessLaneClassification,
    basis: S8FullDeclaredScanBasis,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    match lane {
        S8AccessLaneClassification::Verifier | S8AccessLaneClassification::Terminal => {}
        _ => {
            return Err(S8AccessShapeUnsupportedDenial::HiddenBroadScan {
                requested_shape: S8AccessShape::FullDeclaredScan,
            });
        }
    }

    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::FullDeclaredScan(basis),
        lane,
        S8ExpectedCounterClass::FullDeclaredScan,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}

pub(crate) fn manifest_graph_walk(
    coverage: S8LayoutCoverageWitness,
    lane: S8AccessLaneClassification,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    if lane != S8AccessLaneClassification::Terminal {
        return Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: S8AccessShape::ManifestGraphWalk,
            lane,
        });
    }

    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::ManifestGraphWalk(S8ManifestGraphWalkBasis::ManifestAuthorityGraph),
        lane,
        S8ExpectedCounterClass::ManifestGraphWalk,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}
