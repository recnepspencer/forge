#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IndexPageIntegrityCounters {
    derived_scope_checks: u32,
    index_page_header_checks: u32,
    authority_basis_checks: u32,
    generation_link_checks: u32,
    rebuildable_classifications: u32,
    indeterminate_classifications: u32,
    authority_damage_denials: u32,
    skipped_semantic_index_lookups: u32,
}

impl IndexPageIntegrityCounters {
    pub const fn start() -> Self {
        Self {
            derived_scope_checks: 1,
            index_page_header_checks: 0,
            authority_basis_checks: 0,
            generation_link_checks: 0,
            rebuildable_classifications: 0,
            indeterminate_classifications: 0,
            authority_damage_denials: 0,
            skipped_semantic_index_lookups: 0,
        }
    }

    pub const fn with_index_page_header_check(mut self) -> Self {
        self.index_page_header_checks += 1;
        self
    }

    pub const fn with_authority_basis_check(mut self) -> Self {
        self.authority_basis_checks += 1;
        self
    }

    pub const fn with_generation_link_check(mut self) -> Self {
        self.generation_link_checks += 1;
        self
    }

    pub const fn with_rebuildable_classification(mut self) -> Self {
        self.rebuildable_classifications += 1;
        self
    }

    pub const fn with_indeterminate_classification(mut self) -> Self {
        self.indeterminate_classifications += 1;
        self
    }

    pub const fn with_authority_damage_denial(mut self) -> Self {
        self.authority_damage_denials += 1;
        self
    }

    pub const fn with_skipped_semantic_index_lookup(mut self) -> Self {
        self.skipped_semantic_index_lookups += 1;
        self
    }

    pub const fn derived_scope_checks(self) -> u32 {
        self.derived_scope_checks
    }

    pub const fn index_page_header_checks(self) -> u32 {
        self.index_page_header_checks
    }

    pub const fn authority_basis_checks(self) -> u32 {
        self.authority_basis_checks
    }

    pub const fn generation_link_checks(self) -> u32 {
        self.generation_link_checks
    }

    pub const fn rebuildable_classifications(self) -> u32 {
        self.rebuildable_classifications
    }

    pub const fn indeterminate_classifications(self) -> u32 {
        self.indeterminate_classifications
    }

    pub const fn authority_damage_denials(self) -> u32 {
        self.authority_damage_denials
    }

    pub const fn skipped_semantic_index_lookups(self) -> u32 {
        self.skipped_semantic_index_lookups
    }
}
