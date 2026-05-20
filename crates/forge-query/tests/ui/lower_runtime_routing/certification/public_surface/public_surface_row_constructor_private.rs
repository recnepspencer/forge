use forge_query::facade::{
    ForgeQueryLowerRuntimePublicSurfaceKind, ForgeQueryLowerRuntimePublicSurfaceRow,
    ForgeQueryLowerRuntimeSeamKey,
};

fn main() {
    let _ = ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::ComposeRead,
        "surface",
        "path",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "lane",
    );
}
