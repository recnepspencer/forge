use worth_query::facade::runtime::{WorthQueryLowerRuntimePublicSurfaceKind, WorthQueryLowerRuntimePublicSurfaceRow, WorthQueryLowerRuntimeSeamKey};

fn main() {
    let _ = WorthQueryLowerRuntimePublicSurfaceRow::new(
        WorthQueryLowerRuntimeSeamKey::ComposeRead,
        "surface",
        "path",
        WorthQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "lane",
    );
}
