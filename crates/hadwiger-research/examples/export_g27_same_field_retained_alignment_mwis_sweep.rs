use hadwiger_research::facade::{
    admit_hadwiger_research_handle, export_g27_same_field_retained_alignment_mwis_sweep_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let artifact = export_g27_same_field_retained_alignment_mwis_sweep_checked(&handle)
        .expect("retained alignment sweep should replay");
    println!("{}", artifact.line_export());
}
