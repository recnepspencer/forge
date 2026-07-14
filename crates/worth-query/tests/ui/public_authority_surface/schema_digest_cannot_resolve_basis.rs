use worth_query::facade::foundation::{resolve_runtime_current_snapshot_basis, SchemaBasisDigest};
use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

fn promote(snapshot: WorthQueryEvidenceIdentity, digest: SchemaBasisDigest) {
    let _ = resolve_runtime_current_snapshot_basis(snapshot, digest);
}

fn main() {}
