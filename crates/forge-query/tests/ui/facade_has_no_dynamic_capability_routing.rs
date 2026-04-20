use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily};

fn main() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let _ = facade.capability(ForgeQueryCapabilityFamily::QueryRead);
}
