use forge_query::facade::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectPublication,
};

use super::aspect_kinds::{HadwigerAspectKind, HadwigerAspectPosture};

pub fn query_aspect_contract_for_hadwiger_kind(
    aspect_kind: HadwigerAspectKind,
) -> ForgeQueryDeclarationAspectContract {
    let path = aspect_kind.query_aspect_path();
    match aspect_kind {
        HadwigerAspectKind::AIAdvisory | HadwigerAspectKind::FailureEvidence => {
            ForgeQueryDeclarationAspectContract::from_slices(&[], &[path], &[path], &[], &[])
        }
        HadwigerAspectKind::LowerBoundWitness => ForgeQueryDeclarationAspectContract::from_slices(
            &[path],
            &[
                HadwigerAspectKind::UnitDistanceEmbedding.query_aspect_path(),
                HadwigerAspectKind::NotKColorable.query_aspect_path(),
            ],
            &[],
            &[],
            &[HadwigerAspectKind::AIAdvisory.query_aspect_path()],
        ),
        _ => ForgeQueryDeclarationAspectContract::from_slices(&[path], &[], &[], &[], &[]),
    }
}

pub fn query_aspect_coverage_for_hadwiger_posture(
    aspect_kind: HadwigerAspectKind,
    posture: HadwigerAspectPosture,
) -> ForgeQueryDeclarationAspectCoverage {
    let path = aspect_kind.query_aspect_path();
    match posture {
        HadwigerAspectPosture::Admitted | HadwigerAspectPosture::Advisory => {
            ForgeQueryDeclarationAspectCoverage::from_slices(&[path], &[], &[])
        }
        HadwigerAspectPosture::Conflict | HadwigerAspectPosture::Rejected => {
            ForgeQueryDeclarationAspectCoverage::from_slices(&[], &[], &[path])
        }
        HadwigerAspectPosture::Stale => {
            ForgeQueryDeclarationAspectCoverage::from_slices(&[], &[path], &[])
        }
        HadwigerAspectPosture::Missing
        | HadwigerAspectPosture::Unsupported
        | HadwigerAspectPosture::Deferred => {
            ForgeQueryDeclarationAspectCoverage::from_slices(&[], &[], &[])
        }
    }
}

pub fn query_aspect_publication_for_hadwiger_kind(
    aspect_kind: HadwigerAspectKind,
) -> ForgeQueryDeclarationAspectPublication {
    let path = aspect_kind.query_aspect_path();
    match aspect_kind {
        HadwigerAspectKind::AIAdvisory | HadwigerAspectKind::FailureEvidence => {
            ForgeQueryDeclarationAspectPublication::new(
                [path],
                std::iter::empty::<&str>(),
                std::iter::empty::<&str>(),
                std::iter::empty::<&str>(),
            )
        }
        _ => ForgeQueryDeclarationAspectPublication::new(
            [path],
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
        ),
    }
}
