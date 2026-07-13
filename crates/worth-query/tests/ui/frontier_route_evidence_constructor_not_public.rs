use worth_query::facade::foundation::{FrontierDisjointnessClass, FrontierSurfaceDigest};
use worth_query::facade::FrontierRouteEvidence;

fn main() {
    let digest = FrontierSurfaceDigest::from_label("Worthd");
    let _ = FrontierRouteEvidence::parallel_admission(
        "basis".to_string(),
        digest,
        FrontierDisjointnessClass::CollectionWindowSurface,
    );
}
