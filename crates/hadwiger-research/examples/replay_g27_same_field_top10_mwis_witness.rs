use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_same_field_top10_mwis_witness_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let atom_mask = std::env::args()
        .nth(1)
        .expect("pass an atom mask")
        .parse::<u32>()
        .expect("atom mask should parse");
    let selected = std::env::args()
        .nth(2)
        .expect("pass a comma-separated one-based W vertex list")
        .split(',')
        .filter(|entry| !entry.is_empty())
        .map(|entry| entry.parse::<usize>().expect("vertex id should parse"))
        .collect::<Vec<_>>();
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_same_field_top10_mwis_witness_checked(&handle, atom_mask, &selected)
        .expect("top-10 witness should replay");
    println!("{:?}", report.status());
    println!("{:?}", report.summary());
}
