use forge_query::facade::ForgeQueryGroupedBaselineMember;

fn main() {
    accepts_grouped_baseline_members(Some(vec![(
        "member-1".to_string(),
        "lane-a".to_string(),
    )]));
}

fn accepts_grouped_baseline_members(_members: Option<Vec<ForgeQueryGroupedBaselineMember>>) {
}
