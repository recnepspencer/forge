use worth_query::facade::foundation::{WorthQueryApplicationFacade, WorthQueryCapabilityFamily};

fn main() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let _ = facade.capability(WorthQueryCapabilityFamily::QueryRead);
}
