use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStageLinkSet;

fn requires_typed_stage_links(_: WorkloadEvidenceStageLinkSet) {}

fn promote_string_prefix_link(link: String) {
    requires_typed_stage_links(link);
}

fn main() {}
