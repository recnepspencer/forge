use worth_query::facade::foundation::{
    AspectFieldKey, WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectPublication,
};

use super::aspect_kinds::{HadwigerAspectKind, HadwigerAspectPosture};

fn query_aspect_field(path: &str) -> AspectFieldKey {
    let (aspect, field) = path
        .rsplit_once('.')
        .expect("Hadwiger query aspect paths include an aspect and field");
    AspectFieldKey::from_authoring_parts(aspect, field)
        .expect("Hadwiger query aspect paths are valid declaration keys")
}

fn query_aspect_fields(paths: &[&str]) -> Vec<AspectFieldKey> {
    paths.iter().map(|path| query_aspect_field(path)).collect()
}

pub fn query_aspect_contract_for_hadwiger_kind(
    aspect_kind: HadwigerAspectKind,
) -> WorthQueryDeclarationAspectContract {
    let path = aspect_kind.query_aspect_path();
    match aspect_kind {
        HadwigerAspectKind::AIAdvisory | HadwigerAspectKind::FailureEvidence => {
            WorthQueryDeclarationAspectContract::new(
                [],
                query_aspect_fields(&[path]),
                query_aspect_fields(&[path]),
                [],
                [],
            )
        }
        HadwigerAspectKind::LowerBoundWitness => WorthQueryDeclarationAspectContract::new(
            query_aspect_fields(&[path]),
            query_aspect_fields(&[
                HadwigerAspectKind::UnitDistanceEmbedding.query_aspect_path(),
                HadwigerAspectKind::NotKColorable.query_aspect_path(),
            ]),
            [],
            [],
            query_aspect_fields(&[HadwigerAspectKind::AIAdvisory.query_aspect_path()]),
        ),
        _ => WorthQueryDeclarationAspectContract::new(query_aspect_fields(&[path]), [], [], [], []),
    }
}

pub fn query_aspect_coverage_for_hadwiger_posture(
    aspect_kind: HadwigerAspectKind,
    posture: HadwigerAspectPosture,
) -> WorthQueryDeclarationAspectCoverage {
    let path = aspect_kind.query_aspect_path();
    match posture {
        HadwigerAspectPosture::Admitted | HadwigerAspectPosture::Advisory => {
            WorthQueryDeclarationAspectCoverage::from_present(query_aspect_fields(&[path]))
        }
        HadwigerAspectPosture::Conflict | HadwigerAspectPosture::Rejected => {
            WorthQueryDeclarationAspectCoverage::new([], [], query_aspect_fields(&[path]))
        }
        HadwigerAspectPosture::Stale => {
            WorthQueryDeclarationAspectCoverage::new([], query_aspect_fields(&[path]), [])
        }
        HadwigerAspectPosture::Missing
        | HadwigerAspectPosture::Unsupported
        | HadwigerAspectPosture::Deferred => WorthQueryDeclarationAspectCoverage::empty(),
    }
}

pub fn query_aspect_publication_for_hadwiger_kind(
    aspect_kind: HadwigerAspectKind,
) -> WorthQueryDeclarationAspectPublication {
    let path = aspect_kind.query_aspect_path();
    match aspect_kind {
        HadwigerAspectKind::AIAdvisory | HadwigerAspectKind::FailureEvidence => {
            WorthQueryDeclarationAspectPublication::new(query_aspect_fields(&[path]), [], [], [])
        }
        _ => WorthQueryDeclarationAspectPublication::new(query_aspect_fields(&[path]), [], [], []),
    }
}
