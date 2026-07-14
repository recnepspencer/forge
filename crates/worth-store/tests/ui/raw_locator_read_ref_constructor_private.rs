use worth_store::{PlacementArtifactFamily, PlacementBoundArtifactRef};

fn main() {
    let _ = PlacementBoundArtifactRef::new(
        PlacementArtifactFamily::AuthoritativeBranchHead,
        "raw-locator://tier-a/main",
        Some(String::from("branch:main")),
    );
}
