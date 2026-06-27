use worth_ui::facade::WorthUiImpactLookupCounters;

fn main() {
    let _ = WorthUiImpactLookupCounters {
        impact_classifications_consumed: 1,
        dependency_metadata_reads: 1,
        module_impact_lookups: 1,
        subtree_impact_lookups: 1,
        runtime_hook_lookups: 1,
        subtree_digest_lookups: 1,
        full_artifact_scans: 0,
        plan_lowering_attempts: 0,
    };
}
