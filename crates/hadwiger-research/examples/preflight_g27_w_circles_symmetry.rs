use hadwiger_research::facade::{
    admit_hadwiger_research_handle, preflight_g27_w_circles_symmetry_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = preflight_g27_w_circles_symmetry_checked(&handle)
        .expect("W_circles_607 symmetry preflight should replay");
    let (vertices, edges, weights) = report.shape_summary();
    let (valid, group, vertex_orbits, largest_orbit, singleton_orbits, edge_orbits) =
        report.symmetry_summary();
    println!(
        "vertices {} edges {} weights {} valid_transforms {} group_size {} vertex_orbits {} largest_vertex_orbit {} singleton_vertex_orbits {} edge_orbits {} status {:?} theorem_authority {}",
        vertices,
        edges,
        weights,
        valid,
        group,
        vertex_orbits,
        largest_orbit,
        singleton_orbits,
        edge_orbits,
        report.status(),
        report.admits_theorem_authority()
    );
    for row in report.transform_rows() {
        println!(
            "transform {} status {:?} fixed_vertices {}",
            row.name(),
            row.status(),
            row.fixed_vertex_count()
        );
    }
    println!("conclusion {}", report.conclusion());
}
