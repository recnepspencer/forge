use worth_query::facade::{
    FrontierDisjointnessClass, FrontierRouteEvidence, FrontierSurfaceDigest,
};

fn main() {
    let digest = FrontierSurfaceDigest::from_label("Worthd");
    let _ = FrontierRouteEvidence::parallel_admission(
        "basis".to_string(),
        digest,
        FrontierDisjointnessClass::CollectionWindowSurface,
    );
}
