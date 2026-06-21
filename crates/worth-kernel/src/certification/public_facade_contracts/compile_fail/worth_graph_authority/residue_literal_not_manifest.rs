use worth_kernel::query_graph_authority_gate::WorthGraphAuthorityDeletionLedgerRow;

fn requires_certified_residue_manifest(_: WorthGraphAuthorityDeletionLedgerRow) {}

fn main() {
    requires_certified_residue_manifest("owner=kernel;cap=handoff-only");
}
