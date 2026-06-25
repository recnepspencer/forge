pub struct WorthValidationAuthorityInventoryCompileFailTarget {
    path: &'static str,
    forbidden_boundary: &'static str,
}

impl WorthValidationAuthorityInventoryCompileFailTarget {
    pub const fn new(path: &'static str, forbidden_boundary: &'static str) -> Self {
        Self {
            path,
            forbidden_boundary,
        }
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }

    pub const fn forbidden_boundary(&self) -> &'static str {
        self.forbidden_boundary
    }
}

const COMPILE_FAIL_TARGETS: &[WorthValidationAuthorityInventoryCompileFailTarget] = &[
    WorthValidationAuthorityInventoryCompileFailTarget::new(
        "tests/ui/validation_authority_inventory/inventory_row_struct_literal.rs",
        "public callers cannot forge inventory rows",
    ),
    WorthValidationAuthorityInventoryCompileFailTarget::new(
        "tests/ui/validation_authority_inventory/from_rows_for_validation_private.rs",
        "public callers cannot validate arbitrary row vectors into inventory proof",
    ),
    WorthValidationAuthorityInventoryCompileFailTarget::new(
        "tests/ui/validation_authority_inventory/cut_line_struct_literal.rs",
        "public callers cannot forge cut-line readiness",
    ),
];

pub const fn validation_authority_inventory_compile_fail_targets(
) -> &'static [WorthValidationAuthorityInventoryCompileFailTarget] {
    COMPILE_FAIL_TARGETS
}

pub const VALIDATION_AUTHORITY_INVENTORY_COMPILE_FAIL_TARGET_COUNT: usize =
    COMPILE_FAIL_TARGETS.len();
