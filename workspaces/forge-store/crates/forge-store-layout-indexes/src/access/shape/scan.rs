use super::contract::{S8AccessShapeContract, S8ExpectedCounterClass};
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{
    S8AccessShapeDetail, S8BoundedScanBasis, S8FullDeclaredScanBasis, S8ManifestGraphWalkBasis,
};
use super::lane::S8AccessLaneClassification;
use super::shape::S8AccessShape;
use crate::materialization::S8LayoutCoverageWitness;

#[derive(Debug, PartialEq, Eq)]
enum S8FullDeclaredScanCase {
    Success(S8AccessShapeContract),
    HiddenDenied(S8AccessShapeUnsupportedDenial),
    Denied(S8AccessShapeUnsupportedDenial),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8FullDeclaredScanOutcome {
    case: S8FullDeclaredScanCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8FullDeclaredScanView<'a> {
    Success(&'a S8AccessShapeContract),
    HiddenDenied(&'a S8AccessShapeUnsupportedDenial),
    Denied(&'a S8AccessShapeUnsupportedDenial),
}

impl S8FullDeclaredScanOutcome {
    pub(crate) fn admitted(value: S8AccessShapeContract) -> Self {
        Self::from_owner_payload(S8FullDeclaredScanCase::Success(value))
    }

    pub(crate) fn hidden_denied(value: S8AccessShapeUnsupportedDenial) -> Self {
        Self::from_owner_payload(S8FullDeclaredScanCase::HiddenDenied(value))
    }

    pub(crate) fn denied(value: S8AccessShapeUnsupportedDenial) -> Self {
        Self::from_owner_payload(S8FullDeclaredScanCase::Denied(value))
    }

    fn from_owner_payload(case: S8FullDeclaredScanCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8FullDeclaredScanView<'_> {
        match &self.case {
            S8FullDeclaredScanCase::Success(value) => S8FullDeclaredScanView::Success(value),
            S8FullDeclaredScanCase::HiddenDenied(value) => {
                S8FullDeclaredScanView::HiddenDenied(value)
            }
            S8FullDeclaredScanCase::Denied(value) => S8FullDeclaredScanView::Denied(value),
        }
    }

    fn into_owner_payload(self) -> S8FullDeclaredScanCase {
        self.case
    }
}

impl S8FullDeclaredScanOutcome {
    pub fn into_result(self) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        match self.into_owner_payload() {
            S8FullDeclaredScanCase::Success(value) => Ok(value),
            S8FullDeclaredScanCase::HiddenDenied(denial)
            | S8FullDeclaredScanCase::Denied(denial) => Err(denial),
        }
    }

    pub fn unwrap(self) -> S8AccessShapeContract {
        self.into_result().unwrap()
    }
    pub fn expect(self, message: &str) -> S8AccessShapeContract {
        self.into_result().expect(message)
    }
    pub fn unwrap_err(self) -> S8AccessShapeUnsupportedDenial {
        self.into_result().unwrap_err()
    }
    pub fn expect_err(self, message: &str) -> S8AccessShapeUnsupportedDenial {
        self.into_result().expect_err(message)
    }
}

impl PartialEq<Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial>>
    for S8FullDeclaredScanOutcome
{
    fn eq(&self, other: &Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial>) -> bool {
        match (self.view(), other) {
            (S8FullDeclaredScanView::Success(left), Ok(right)) => left == right,
            (S8FullDeclaredScanView::HiddenDenied(left), Err(right))
            | (S8FullDeclaredScanView::Denied(left), Err(right)) => left == right,
            _ => false,
        }
    }
}

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
) -> S8FullDeclaredScanOutcome {
    match lane {
        S8AccessLaneClassification::Verifier | S8AccessLaneClassification::Terminal => {}
        _ => {
            return S8FullDeclaredScanOutcome::hidden_denied(
                S8AccessShapeUnsupportedDenial::HiddenBroadScan {
                    requested_shape: S8AccessShape::FullDeclaredScan,
                },
            );
        }
    }

    let exact = match coverage.require_exact() {
        Ok(exact) => exact,
        Err(denial) => {
            return S8FullDeclaredScanOutcome::denied(
                S8AccessShapeUnsupportedDenial::MaterializationDenied(denial),
            );
        }
    };
    S8FullDeclaredScanOutcome::admitted(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::FullDeclaredScan(basis),
        lane,
        S8ExpectedCounterClass::FullDeclaredScan,
        exact,
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
