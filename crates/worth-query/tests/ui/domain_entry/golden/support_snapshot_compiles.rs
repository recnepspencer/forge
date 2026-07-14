use worth_query::facade::foundation::WorthQueryApplicationFacade;

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let support = query.domain_entry_support_snapshot();

    let _ = support.snapshot_digest();
    let _ = support.section_postures();
}
