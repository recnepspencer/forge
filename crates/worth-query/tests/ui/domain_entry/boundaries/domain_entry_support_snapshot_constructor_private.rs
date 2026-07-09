use worth_query::facade::{WorthQueryApplicationFacade, WorthQueryDomainEntrySupportSnapshot};

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let _ = WorthQueryDomainEntrySupportSnapshot {
        report: query.support_report(),
        snapshot_digest: String::new(),
    };
}
