use super::contract::{AccessShapeContract, ExpectedCounterClass};
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::{AccessShapeDetail, FullDeclaredScanBasis};
#[cfg(test)]
use super::detail::{BoundedScanBasis, ManifestGraphWalkBasis};
use super::kind::AccessShape;
use super::lane::AccessLaneClassification;

#[derive(Debug, PartialEq, Eq)]
enum FullDeclaredScanCase {
    Success(AccessShapeContract),
    HiddenDenied(AccessShapeUnsupportedDenial),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FullDeclaredScanOutcome {
    case: FullDeclaredScanCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FullDeclaredScanCaseId {
    Admitted,
    HiddenBroadScanDenied,
}

impl FullDeclaredScanCaseId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "layout.access.full_declared_scan.admitted",
            Self::HiddenBroadScanDenied => "layout.access.full_declared_scan.denied.hidden",
        }
    }
}

pub fn full_declared_scan_cases() -> impl Iterator<Item = FullDeclaredScanCaseId> {
    [
        FullDeclaredScanCaseId::Admitted,
        FullDeclaredScanCaseId::HiddenBroadScanDenied,
    ]
    .into_iter()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullDeclaredScanView<'a> {
    Success(&'a AccessShapeContract),
    HiddenDenied(&'a AccessShapeUnsupportedDenial),
}

impl FullDeclaredScanOutcome {
    fn admitted(value: AccessShapeContract) -> Self {
        Self::from_owner_payload(FullDeclaredScanCase::Success(value))
    }

    fn hidden_denied(value: AccessShapeUnsupportedDenial) -> Self {
        Self::from_owner_payload(FullDeclaredScanCase::HiddenDenied(value))
    }

    fn from_owner_payload(case: FullDeclaredScanCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> FullDeclaredScanView<'_> {
        match &self.case {
            FullDeclaredScanCase::Success(value) => FullDeclaredScanView::Success(value),
            FullDeclaredScanCase::HiddenDenied(value) => FullDeclaredScanView::HiddenDenied(value),
        }
    }

    pub const fn case_id(&self) -> FullDeclaredScanCaseId {
        match &self.case {
            FullDeclaredScanCase::Success(_) => FullDeclaredScanCaseId::Admitted,
            FullDeclaredScanCase::HiddenDenied(_) => FullDeclaredScanCaseId::HiddenBroadScanDenied,
        }
    }

    fn into_owner_payload(self) -> FullDeclaredScanCase {
        self.case
    }
}

impl FullDeclaredScanOutcome {
    pub fn into_result(self) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        match self.into_owner_payload() {
            FullDeclaredScanCase::Success(value) => Ok(value),
            FullDeclaredScanCase::HiddenDenied(denial) => Err(denial),
        }
    }

    pub fn expect(self, message: &str) -> AccessShapeContract {
        self.into_result().expect(message)
    }
}

impl PartialEq<Result<AccessShapeContract, AccessShapeUnsupportedDenial>>
    for FullDeclaredScanOutcome
{
    fn eq(&self, other: &Result<AccessShapeContract, AccessShapeUnsupportedDenial>) -> bool {
        match (self.view(), other) {
            (FullDeclaredScanView::Success(left), Ok(right)) => left == right,
            (FullDeclaredScanView::HiddenDenied(left), Err(right)) => left == right,
            _ => false,
        }
    }
}

#[cfg(test)]
pub(crate) fn bounded_scan(
    lane: AccessLaneClassification,
    basis: BoundedScanBasis,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::BoundedScan(basis),
        lane,
        ExpectedCounterClass::BoundedScan,
    ))
}

pub(super) fn full_declared_scan(
    lane: AccessLaneClassification,
    basis: FullDeclaredScanBasis,
) -> FullDeclaredScanOutcome {
    match lane {
        AccessLaneClassification::Verifier | AccessLaneClassification::Terminal => {}
        _ => {
            return FullDeclaredScanOutcome::hidden_denied(
                AccessShapeUnsupportedDenial::HiddenBroadScan {
                    requested_shape: AccessShape::FullDeclaredScan,
                },
            );
        }
    }

    FullDeclaredScanOutcome::admitted(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::FullDeclaredScan(basis),
        lane,
        ExpectedCounterClass::FullDeclaredScan,
    ))
}

#[cfg(test)]
pub(crate) fn manifest_graph_walk(
    lane: AccessLaneClassification,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    if lane != AccessLaneClassification::Terminal {
        return Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::ManifestGraphWalk,
            lane,
        });
    }

    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::ManifestGraphWalk(ManifestGraphWalkBasis::ManifestAuthorityGraph),
        lane,
        ExpectedCounterClass::ManifestGraphWalk,
    ))
}
