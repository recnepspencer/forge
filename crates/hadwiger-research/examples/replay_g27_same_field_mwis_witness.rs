use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_same_field_dominant_mwis_witness_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let selected = std::env::args()
        .nth(1)
        .expect("pass a comma-separated one-based W vertex list")
        .split(',')
        .filter(|entry| !entry.is_empty())
        .map(|entry| entry.parse::<usize>().expect("vertex id should parse"))
        .collect::<Vec<_>>();
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_same_field_dominant_mwis_witness_checked(&handle, &selected)
        .expect("dominant MWIS witness should replay");
    println!("{:?}", report.status());
    println!("{:?}", report.summary());
}
