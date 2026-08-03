#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiScalarProjectionWorkCounters {
    native_key_declaration_checks: usize,
    native_key_indexed_slot_lookups: usize,
    native_key_path_matches: usize,
    native_key_scans: usize,
    native_key_path_parses: usize,
    native_indexed_accesses: usize,
    native_fact_scans: usize,
    native_row_scans: usize,
    native_access_path_parses: usize,
    view_registry_inspections: usize,
    domain_registry_inspections: usize,
}

impl UiScalarProjectionWorkCounters {
    pub(crate) fn query_native(
        resolution: worth_query::facade::domain::WorthQueryNativeKeyResolutionCounters,
        access: worth_query::facade::installed::operation::WorthQueryNativeAccessCounters,
    ) -> Self {
        Self {
            native_key_declaration_checks: resolution.declaration_checks,
            native_key_indexed_slot_lookups: resolution.indexed_slot_lookups,
            native_key_path_matches: resolution.path_matches,
            native_key_scans: resolution.key_scans,
            native_key_path_parses: resolution.path_parses,
            native_indexed_accesses: access.indexed_accesses,
            native_fact_scans: access.fact_scans,
            native_row_scans: access.row_scans,
            native_access_path_parses: access.path_parses,
            view_registry_inspections: access.view_registry_inspections,
            domain_registry_inspections: access.domain_registry_inspections,
        }
    }

    pub fn native_key_declaration_checks(&self) -> usize {
        self.native_key_declaration_checks
    }

    pub fn native_key_indexed_slot_lookups(&self) -> usize {
        self.native_key_indexed_slot_lookups
    }

    pub fn native_key_scan_work(&self) -> usize {
        self.native_key_path_matches + self.native_key_scans + self.native_key_path_parses
    }

    pub fn native_indexed_accesses(&self) -> usize {
        self.native_indexed_accesses
    }

    pub fn native_access_scan_work(&self) -> usize {
        self.native_fact_scans
            + self.native_row_scans
            + self.native_access_path_parses
            + self.view_registry_inspections
            + self.domain_registry_inspections
    }
}
