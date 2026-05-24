use forge_query::facade::ForgeQueryApplicationFacade;

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let _ = query.domain("worth.spatial");
}
