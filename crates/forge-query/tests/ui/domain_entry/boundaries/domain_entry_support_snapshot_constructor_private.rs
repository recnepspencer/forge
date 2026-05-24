use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryDomainEntrySupportSnapshot};

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let _ = ForgeQueryDomainEntrySupportSnapshot {
        report: query.support_report(),
        snapshot_digest: String::new(),
    };
}
