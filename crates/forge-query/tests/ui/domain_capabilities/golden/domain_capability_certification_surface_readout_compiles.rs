use forge_query::facade::runtime::{
    forge_query_domain_capability_certification_surface,
    forge_query_domain_capability_compile_fail_boundaries,
    forge_query_domain_capability_golden_transcripts,
    forge_query_domain_capability_public_surface_inventory,
};

fn certification_surface_readout() {
    let surface = forge_query_domain_capability_certification_surface();
    let inventory = forge_query_domain_capability_public_surface_inventory();
    let goldens = forge_query_domain_capability_golden_transcripts();
    let compile_fail = forge_query_domain_capability_compile_fail_boundaries();

    let _ = surface.public_surface_digest();
    let _ = surface.target_dx_digest();
    let _ = surface.golden_transcript_digest();
    let _ = surface.compile_fail_boundary_digest();
    let _ = surface.certification_surface_digest();
    let _ = surface.category_count();
    let _ = surface.golden_transcript_count();
    let _ = surface.compile_fail_boundary_count();
    let _ = inventory.public_surface_digest();
    let _ = goldens[0].path();
    let _ = compile_fail[0].path();
}

fn main() {
    certification_surface_readout();
}
