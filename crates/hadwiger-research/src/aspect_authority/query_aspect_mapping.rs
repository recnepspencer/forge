use worth_query::facade::{
    WORTHQueryDeclarationAspectContract, WORTHQueryDeclarationAspectCoverage,
    WORTHQueryDeclarationAspectPublication,
};

use super::aspect_kinds::{HadwigerAspectKind, HadwigerAspectPosture};

pub fn query_aspect_contract_for_hadwiger_kind(
    aspect_kind: HadwigerAspectKind,
) -> WORTHQueryDeclarationAspectContract {
    let path = aspect_kind.query_aspect_path();
    match aspect_kind {
        HadwigerAspectKind::AIAdvisory | HadwigerAspectKind::FailureEvidence => {
            WORTHQueryDeclarationAspectContract::from_slices(&[], &[path], &[path], &[], &[])
        }
        HadwigerAspectKind::LowerBoundWitness => WORTHQueryDeclarationAspectContract::from_slices(
            &[path],
            &[
                HadwigerAspectKind::UnitDistanceEmbedding.query_aspect_path(),
                HadwigerAspectKind::NotKColorable.query_aspect_path(),
            ],
            &[],
            &[],
            &[HadwigerAspectKind::AIAdvisory.query_aspect_path()],
        ),
        _ => WORTHQueryDeclarationAspectContract::from_slices(&[path], &[], &[], &[], &[]),
    }
}

pub fn query_aspect_coverage_for_hadwiger_posture(
    aspect_kind: HadwigerAspectKind,
    posture: HadwigerAspectPosture,
) -> WORTHQueryDeclarationAspectCoverage {
    let path = aspect_kind.query_aspect_path();
    match posture {
        HadwigerAspectPosture::Admitted | HadwigerAspectPosture::Advisory => {
            WORTHQueryDeclarationAspectCoverage::from_slices(&[path], &[], &[])
        }
        HadwigerAspectPosture::Conflict | HadwigerAspectPosture::Rejected => {
            WORTHQueryDeclarationAspectCoverage::from_slices(&[], &[], &[path])
        }
        HadwigerAspectPosture::Stale => {
            WORTHQueryDeclarationAspectCoverage::from_slices(&[], &[path], &[])
        }
        HadwigerAspectPosture::Missing
        | HadwigerAspectPosture::Unsupported
        | HadwigerAspectPosture::Deferred => {
            WORTHQueryDeclarationAspectCoverage::from_slices(&[], &[], &[])
        }
    }
}

pub fn query_aspect_publication_for_hadwiger_kind(
    aspect_kind: HadwigerAspectKind,
) -> WORTHQueryDeclarationAspectPublication {
    let path = aspect_kind.query_aspect_path();
    match aspect_kind {
        HadwigerAspectKind::AIAdvisory | HadwigerAspectKind::FailureEvidence => {
            WORTHQueryDeclarationAspectPublication::new(
                [path],
                std::iter::empty::<&str>(),
                std::iter::empty::<&str>(),
                std::iter::empty::<&str>(),
            )
        }
        _ => WORTHQueryDeclarationAspectPublication::new(
            [path],
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
        ),
    }
}
