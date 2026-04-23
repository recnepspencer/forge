use forge_query::facade::{PreviewResidueWidth, PreviewSubscriptionResidueReport};

fn main() {
    let _ = PreviewSubscriptionResidueReport {
        authoritative_routing_width: PreviewResidueWidth::measured(0),
        authoritative_checkpoint_width: PreviewResidueWidth::measured(0),
        authoritative_replay_width: PreviewResidueWidth::measured(0),
        authoritative_diagnostics_width: PreviewResidueWidth::measured(0),
        authoritative_writeback_width: PreviewResidueWidth::measured(0),
        temporary_execution_width: PreviewResidueWidth::measured(1),
        temporary_diagnostics_width: PreviewResidueWidth::measured(1),
        report_digest: "residue".to_string(),
    };
}
