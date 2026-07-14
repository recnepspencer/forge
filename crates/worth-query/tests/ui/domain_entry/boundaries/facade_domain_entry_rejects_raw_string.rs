use worth_query::facade::foundation::WorthQueryApplicationFacade;

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let _ = query.domain("worth.spatial");
}
