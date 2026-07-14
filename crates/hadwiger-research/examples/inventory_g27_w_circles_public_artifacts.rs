use hadwiger_research::facade::{
    admit_hadwiger_research_handle, inventory_g27_w_circles_public_artifacts_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = inventory_g27_w_circles_public_artifacts_checked(&handle)
        .expect("W_circles_607 public artifact inventory should replay");
    let (source_url, download_name, archive_hash) = report.source_summary();
    let (data_hash, vertices_hash) = report.digest_summary();
    let (files, proof_like, data, model, scripts, generated) = report.inventory_summary();
    println!(
        "source {} download {} archive {} data_hash {} vertices_hash {} files {} proof_like {} data {} model {} scripts {} generated {} target {} status {:?} theorem_authority {}",
        source_url,
        download_name,
        archive_hash,
        data_hash,
        vertices_hash,
        files,
        proof_like,
        data,
        model,
        scripts,
        generated,
        report.target_weight(),
        report.status(),
        report.admits_theorem_authority()
    );
    println!("required_import_schema {}", report.required_import_schema());
    println!("conclusion {}", report.conclusion());
}
