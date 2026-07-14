use std::path::PathBuf;

use hadwiger_research::facade::{
    admit_hadwiger_research_handle,
    replay_g27_same_field_retained_alignment_mwis_witnesses_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let path = PathBuf::from(std::env::args().nth(1).expect("pass solution file path"));
    let witnesses = parse_solution_file(&std::fs::read_to_string(path).expect("solution file"));
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let reports =
        replay_g27_same_field_retained_alignment_mwis_witnesses_checked(&handle, &witnesses)
            .expect("alignment witnesses should replay");
    for report in reports {
        println!("{:?} {:?}", report.status(), report.summary());
    }
}

fn parse_solution_file(text: &str) -> Vec<(usize, usize, u32, Vec<usize>)> {
    let mut witnesses = Vec::new();
    let mut key = None;
    for line in text.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.first() == Some(&"channel") {
            key = Some((
                parts[1].parse::<usize>().expect("g27 anchor"),
                parts[2].parse::<usize>().expect("w anchor"),
                parts[4].parse::<u32>().expect("atom mask"),
            ));
        } else if parts.first() == Some(&"selected") {
            let selected = parts
                .get(1)
                .copied()
                .unwrap_or("")
                .split(',')
                .filter(|entry| !entry.is_empty())
                .map(|entry| entry.parse::<usize>().expect("vertex id"))
                .collect::<Vec<_>>();
            let (g27_anchor, w_anchor, atom_mask) = key.expect("selected after channel");
            witnesses.push((g27_anchor, w_anchor, atom_mask, selected));
        }
    }
    witnesses
}
