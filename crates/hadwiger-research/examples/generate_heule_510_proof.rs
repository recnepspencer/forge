use std::fs;

use hadwiger_research::facade::{
    admit_hadwiger_research_handle, generate_k_colorability_certificate_with_varisat_checked,
    import_frontier_graph_seed_checked, FrontierGraphSeedImport, HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("Heule 510 exact seed imports");
    let certificate =
        generate_k_colorability_certificate_with_varisat_checked(imported.graph_version(), 4)
            .expect("Varisat native UNSAT proof generates and replays");
    fs::write(
        "crates/hadwiger-research/src/frontier_seeds/heule_510.varisat",
        certificate.proof_bytes(),
    )
    .expect("proof file writes");
    println!("{}", certificate.cnf_digest());
}
