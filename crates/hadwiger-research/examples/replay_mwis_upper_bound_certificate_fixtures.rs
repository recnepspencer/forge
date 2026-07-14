use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_mwis_upper_bound_certificate_fixtures_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_mwis_upper_bound_certificate_fixtures_checked(&handle)
        .expect("MWIS upper-bound certificate fixtures should replay");
    println!(
        "schema {} theorem_authority {}",
        report.schema_name(),
        report.admits_theorem_authority()
    );
    for case in report.cases() {
        let (name, vertices, edges, cliques, objective, target, excess) = case.summary();
        println!(
            "case {} digest {} vertices {} edges {} cliques {} objective {} target {} excess {} status {:?}",
            name,
            case.graph_digest(),
            vertices,
            edges,
            cliques,
            objective,
            target,
            excess,
            case.status()
        );
    }
    println!("conclusion {}", report.conclusion());
}
