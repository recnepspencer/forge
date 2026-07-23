#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryNativeAccessCounters {
    pub authority_checks: usize,
    pub indexed_accesses: usize,
    pub refinement_checks: usize,
    pub fact_scans: usize,
    pub row_scans: usize,
    pub path_parses: usize,
    pub view_registry_inspections: usize,
    pub domain_registry_inspections: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryNativeAccessBindingCounters {
    pub declared_key_routes: usize,
    pub declared_key_layout_checks: usize,
    pub lane_shape_checks: usize,
    pub fact_scans: usize,
    pub row_scans: usize,
    pub path_parses: usize,
    pub view_registry_inspections: usize,
    pub domain_registry_inspections: usize,
}
