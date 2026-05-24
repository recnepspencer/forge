use forge_query::facade::ForgeQueryApplicationFacade;

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let support = query.domain_entry_support_snapshot();

    let _ = support.snapshot_digest();
    let _ = support.section_postures();
}
