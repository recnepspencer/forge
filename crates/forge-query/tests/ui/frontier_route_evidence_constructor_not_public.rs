use forge_query::facade::{
    FrontierDisjointnessClass, FrontierRouteEvidence, FrontierSurfaceDigest,
};

fn main() {
    let digest = FrontierSurfaceDigest::from_label("forged");
    let _ = FrontierRouteEvidence::parallel_admission(
        "basis".to_string(),
        digest,
        FrontierDisjointnessClass::CollectionWindowSurface,
    );
}
