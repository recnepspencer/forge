use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessPostureCapRow;

fn main() {
    let _ = WorthGraphReadAccessPostureCapRow {
        family: "missing_query_read_family_artifact",
        max_count: 1,
        owner: "worth-kernel",
        expected_denial: "missing_query_read_family_artifact",
        suggested_posture: "declare_query_read_family_artifact",
        blocker: "blocked",
        removal_trigger: "delete row",
        row_digest: String::new(),
    };
}
