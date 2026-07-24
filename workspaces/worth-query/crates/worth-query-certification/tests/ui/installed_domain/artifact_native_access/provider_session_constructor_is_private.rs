use worth_query::facade::domain::{
    WorthQueryArtifactNativeLayoutReference, WorthQueryArtifactProviderAccessSession,
};

fn forge(
    layout: WorthQueryArtifactNativeLayoutReference,
) -> WorthQueryArtifactProviderAccessSession {
    WorthQueryArtifactProviderAccessSession::mint("forged".into(), 1, 1, layout)
}

fn main() {}
